use std::env;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;
use std::io::{self, Read};
use reqwest;
use clap::Parser;
use postgres::{Client, NoTls};

//use tokio;

mod database;
mod data_walker;
mod activity;
mod beamtime;
mod synco_runs;

use activity::{Activity, Experimenter};

static STR_URL_ACTIVITY_HEADER: &'static str = "https://beam-api.aps.anl.gov/beamline-scheduling//sched-api/activity/findByRunNameAndBeamlineId/";
//static STR_URL_BEAMTIME_HEADER: &'static str = "https://beam-api.aps.anl.gov/beamline-scheduling/sched-api/beamtimeRequests/findBeamtimeRequestsByRunAndBeamline/";
static STR_IMG_DAT: &'static str = "img.dat";
static STR_MDA: &'static str = "mda";
static STR_PI: &'static str = "Principal Investigator";
static STR_CI: &'static str = "Co-Investigator";
static NUM_DETECTORS: u32 = 8;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// How deep to search for datasets
    #[arg(short, long, default_value_t=2)]
    num_recursive: u32,

    /// Export counts as png 
    #[arg(short, long, action)]
    export_counts_png: bool,

    /// Number of times to greet
    #[arg(short, long, action)]
    test: bool,

    /// Verbose output
    #[arg(short, long, action)]
    verbose: bool,

    /// Load beamtime data from file
    #[arg(short, long)]
    filename: Option<String>,

    /// beamline name
    #[arg(short, long)]
    beamline: Option<String>,

    /// beamtime run
    #[arg(short, long)]
    run: Option<String>,

    /// Experimenter last name
    #[arg(short, long)]
    search_dir: Option<String>,

    /// Query db users
    #[arg(short, long, action)]
    query_db_users: bool,

}

struct Config
{
    activities: Vec<activity::Activity>,
    db_staff: Vec<database::User>,
    db_access_control: HashMap<String, database::UserAccessControl>,
    db_sync_runs: std::collections::HashMap<String, database::SyncRun>,
    db_beamlines: std::collections::HashMap<String, database::Beamline>,
    db_experimenter_roles: std::collections::HashMap<String, database::ExperimenterRole>,
    db_scan_types: std::collections::HashMap<String, database::ScanType>,
    run_id: i32,
    beamline_id: i32,
    pub verbose: bool,
}

impl Config
{
    fn new(beam_schedule: &str,  verbose: bool) -> Self
    {
        Config 
        { 
            activities: serde_json::from_str(&beam_schedule).unwrap(),
            db_staff: Vec::new(),
            db_access_control: HashMap::new(),
            db_sync_runs: HashMap::new(),
            db_beamlines: HashMap::new(),
            db_experimenter_roles: HashMap::new(),
            db_scan_types: HashMap::new(),
            run_id: -1,
            beamline_id: -1,
            verbose: verbose 
        }
    }
    fn search_for_pi_activity(&self, experimenter_lastname: &str) -> (Option<&Activity>, Option<&Experimenter>)
    {
        let mut found_act = None;
        let mut found_exp = None;
        self.activities.iter().for_each(|activity| 
        {
            activity.beamtime.proposal.experimenters.iter().for_each(|experimenter: &Experimenter| 
            {
                if experimenter.piFlag.is_some() && experimenter.lastName == experimenter_lastname
                {
                    if experimenter.piFlag.is_some() && experimenter.piFlag.as_ref().unwrap() == "Y"
                    {
                        //println!("found pi: {} {}", experimenter.firstName, experimenter.lastName);
                        found_act = Some(activity);
                        found_exp = Some(experimenter);
                    }
                }
            });
        });
        (found_act, found_exp)
    }

    fn get_bealine_id(&self) -> u32
    {
        return self.beamline_id as u32;
    }

    fn get_experimenter_role_id(&self, is_pi: &str) -> i32
    {
        if is_pi == "Y"
        {
            let role = self.db_experimenter_roles.get(STR_PI).unwrap();
            return role.get_id();
        }
        else 
        {
            let role =  self.db_experimenter_roles.get(STR_CI).unwrap();
            return role.get_id();
        }
    }

    fn init_run_info(&mut self, run_name: &str, beamline_name: &str)
    {
        if self.verbose
        {
            println!("searching run : {} len {}", run_name, run_name.len());
            for key in self.db_sync_runs.keys().into_iter()
            {
                println!("{} : {}", key, key.len());
            }
        }
        if self.db_sync_runs.contains_key(run_name)
        {
            let sync_run = self.db_sync_runs.get(run_name).unwrap();
            self.run_id = sync_run.get_id();
        }
        else 
        {
            println!("Error: could not find run {}", run_name);
        }
        if self.verbose
        {
            println!("searching beamline : {} ", beamline_name);
            for key in self.db_beamlines.keys().into_iter()
            {
                println!("{}", key);
            }
        }
        if self.db_beamlines.contains_key(beamline_name)
        {
            let beamline = self.db_beamlines.get(beamline_name).unwrap();
            self.beamline_id = beamline.get_id();
        }
        else 
        {
            println!("Error: could not find beamline {}", beamline_name);
        }
    }
}

fn read_json_from_file(file_path: &str) -> Result<String, io::Error> 
{
    // Open the file
    let mut file = File::open(file_path)?;

    // Read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

async fn read_json_from_url(url_path: &str) -> Result<String, reqwest::Error> 
{
    let auth_str = env::var("SVC_AUTH_STR").unwrap_or_else(|_| "Bearer ".to_string());
    let client = reqwest::Client::new();
    let resp = client
    .get(url_path)
    .header("accept", "*/*")
    .header("Authorization", auth_str)
    .send().await?;
    //let resp = reqwest::blocking::get(url_path)?;
    if resp.status() != reqwest::StatusCode::OK
    {
        panic!("Error: {}", resp.status());
    }
    let body = resp.text().await?;
    Ok(body,)
}

fn insert_experimenters_as_users_to_db(experimenters: &Vec<Experimenter>, config: &Config, db_client: &mut Client)
{
    //add experimenter as a user
    for experimenter in experimenters.iter()
    {
        //println!("Experimenter: {} {} ({})", experimenter.firstName, experimenter.lastName, experimenter.piFlag.as_ref().unwrap_or(&"N".to_string()));
        let pi_user: database::User = database::User::from_experimenter(experimenter, config.db_access_control.get("Visitor").unwrap());
        let result = database::insert_user(db_client, &pi_user);
        if result.is_err()
        {
            println!("Error inserting user {} {}: {:?}", pi_user.first_name, pi_user.last_name, result.err().unwrap());
        }
        else 
        {
            println!("Inserted user {} {}", pi_user.first_name, pi_user.last_name);
        }
    }
}

fn link_experimenters_to_dataset(experimenters: &Vec<Experimenter>, dataset_id: i32, proposal_id: i32, config: &Config, db_client: &mut Client)
{
    for experimenter in experimenters.iter()
    {
        let mut pi_flag = "N";
        if experimenter.piFlag.is_some() && experimenter.piFlag.as_ref().unwrap() == "Y"
        {
            pi_flag = "Y";
        }
        let experimenter_role_id = config.get_experimenter_role_id(pi_flag);
        let user_badge:i32 = experimenter.badge.parse().expect("Failed to parse string to integer");
        let db_expr = database::Experimenter::new(dataset_id, user_badge, proposal_id, experimenter_role_id);
        let result =  database::insert_experimenter(db_client, &db_expr);
        if result.is_err()
        {
            println!("Error inserting experimenter {}: {:?}", user_badge, result.err().unwrap());
        }
        else 
        {
            
        }
    }
}

fn process_found_activity(activity: &Activity, raw_files: &Vec<data_walker::MyFile>, config: &Config, db_client: &mut Client)
{
    println!("{:?} {:?}", activity.activityId, activity.experimentId);
    println!{"{:?} {:?} {:?}", activity.beamtime.proposal.gupId, activity.beamtime.proposal.proposalTitle, activity.beamtime.proposalStatus};

    insert_experimenters_as_users_to_db(&activity.beamtime.proposal.experimenters, config, db_client);
    
    let result2 = database::insert_proposal(db_client, &database::Proposal::from_proposal(&activity.beamtime.proposal));
    if result2.is_err()
    {
        println!("Error inserting proposal {:?}: {:?}", activity.activityId, result2.err().unwrap());
    }
    else 
    {
        println!("Inserted proposal {:?}", activity.activityId);
        let proposal_id:i32 = result2.unwrap();
        for raw_file in raw_files
        {
            println!("found raw dataset file {}", raw_file.name);
            
            //let mut xrf_dataset = data_walker::XrfDataset::new();
            //xrf_dataset.load_from_hdf5(&hdf5_file).unwrap();
            let scan_type_id = 1; //hard code to step scan. TODO: check if we have netcdf files to tell if fly scan
            
            let dataset = database::Dataset::new(config.beamline_id, config.run_id, scan_type_id, &raw_file.name, raw_file.ctime);
            let result = database::insert_dataset(db_client, &dataset);
            if result.is_err()
            {
                println!("Error inserting dataset {}: {:?}", raw_file.name, result.err().unwrap());
            }
            else 
            {
                println!("Inserted dataset {}", raw_file.name);
                // link experimenter to this dataset
                let dataset_id = result.unwrap();
                if dataset_id > -1
                {
                    link_experimenters_to_dataset(&activity.beamtime.proposal.experimenters, dataset_id, proposal_id, config, db_client);
                }
                else 
                {
                    println!("Failed to insert dataset. ID = -1");    
                }
            }
        }
    }
}

fn search_for_datasets(direcotry: &str, search_raw_ext: &Vec<String>, search_analyzed_ext: &Vec<String>, cur_depth: u32, config: &mut Config, db_client: &mut Client) -> Result<(), std::io::Error>
{
    let dirs = data_walker::get_dirs(direcotry).unwrap();
    for dir in dirs
    {
        if let Some(dir_name) = dir
        {
            if config.verbose
            {
                println!("dir: {}", dir_name);
            }
            if dir_name.ends_with(STR_MDA)
            {
                let mut raw_files = Vec::new();
                data_walker::saerch_for_ext(&dir_name, search_raw_ext, &mut raw_files);
                println!("found {} files in {}", raw_files.len(), dir_name);

                if raw_files.len() > 0
                {
                    let path = Path::new(&direcotry);
                    if let Some(pi_name) = path.file_stem()
                    {
                        //println!("{}", last_folder.to_str().unwrap());
                        let (found_activity, found_experiementer) = config.search_for_pi_activity(pi_name.to_str().unwrap());
                        if found_activity.is_some() && found_experiementer.is_some()
                        {
                            let activity = found_activity.unwrap();
                            process_found_activity(activity, &raw_files, config, db_client);
                        }
                        else 
                        {
                            println!("Error: could not find pi activity for {}", pi_name.to_str().unwrap());
                        }
                    }
                    else 
                    {
                        println!("Error: could not get last folder name from path {}", direcotry);
                    }    
                }
            }
            else if cur_depth > 0
            {
                let new_depth = cur_depth - 1;
                let _ = search_for_datasets(&dir_name, search_raw_ext, search_analyzed_ext, new_depth, config, db_client);
            }               
        }
    }
    Ok(())
}

//#[tokio::main] 
//async fn main() 
fn main()
{
    let args = Args::parse();

    let psql_conn_str = env::var("SVC_PSQL_CONN_STR").unwrap_or(String::from("postgresql://localhost/mydata"));
    let mut db_client = Client::connect(&psql_conn_str, NoTls).unwrap();
    
    if args.query_db_users
    {
        database::print_all_user(&mut db_client).unwrap();
        return;
    }
    if args.search_dir.is_some()
    {
        if args.run.is_some() && args.beamline.is_some()
        {
            let mut beam_schedule: String = String::new();
            if args.filename.is_some()
            {
                let filename = args.filename.clone().unwrap();
                println!("reading from file {}", filename);
                beam_schedule = read_json_from_file(&filename).unwrap();
                if beam_schedule.is_empty()
                {
                    println!("Error: file {} is empty", filename);
                    return;
                }
                /*
                if args.verbose
                {
                    println!("file contents: {}", beam_schedule);
                }
                */
            }
            else 
            {
                let run = args.run.clone().unwrap();
                let beamline = args.beamline.clone().unwrap();
                
                let mut url_path = STR_URL_ACTIVITY_HEADER.to_owned();
                url_path.push_str(&run);
                url_path.push_str("/");
                url_path.push_str(&beamline);
                println!("reading from url {}", url_path);
                beam_schedule = futures::executor::block_on(read_json_from_url(&url_path)).unwrap();
            }

            //let activities: Vec<activity::Activity> = serde_json::from_str(&beam_schedule).unwrap();

            let mut analyzed_search_ext: Vec<String> = Vec::new();
            analyzed_search_ext.push(".h5".to_owned());
            for i in 0..NUM_DETECTORS
            {
                let mut h5_ext = ".h5".to_owned();
                h5_ext.push_str(&i.to_string());
                analyzed_search_ext.push(h5_ext);
            }

            let mut raw_search_ext: Vec<String> = Vec::new();
            raw_search_ext.push(".mda".to_owned());

            let mut config = Config::new(&beam_schedule,  args.verbose);
            database::get_all_staff_users(&mut db_client, &mut config.db_staff).unwrap();
            database::get_access_control(&mut db_client, &mut config.db_access_control).unwrap();
            database::get_sync_runs(&mut db_client, &mut config.db_sync_runs).unwrap();
            database::get_experimenter_roles(&mut db_client, &mut config.db_experimenter_roles).unwrap();
            database::get_scan_types(&mut db_client, &mut config.db_scan_types).unwrap();
            database::get_beamlines(&mut db_client, &mut config.db_beamlines).unwrap();

            config.init_run_info(&args.run.unwrap(), &args.beamline.unwrap());

            search_for_datasets(args.search_dir.as_ref().unwrap(), &raw_search_ext, &analyzed_search_ext, args.num_recursive, &mut config, &mut db_client).unwrap();
        }
        else
        {
            println!("Error: --run and --beamline must be specified when using --search-dir");  
        }
    }
    else 
    {
        println!("Error: --search-dir must be specified");
    }

}
