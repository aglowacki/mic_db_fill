use std::env;
use std::fs::File;
use std::path::Path;
use std::io::{self, Read};
use reqwest;
use clap::Parser;
use postgres::{Client, NoTls};

use tokio;

mod db_user;
mod data_walker;
mod activity;
mod beamtime;
mod synco_runs;

static STR_URL_ACTIVITY_HEADER: &'static str = "https://beam-api.aps.anl.gov/beamline-scheduling//sched-api/activity/findByRunNameAndBeamlineId/";
static STR_URL_BEAMTIME_HEADER: &'static str = "https://beam-api.aps.anl.gov/beamline-scheduling/sched-api/beamtimeRequests/findBeamtimeRequestsByRunAndBeamline/";
static STR_IMG_DAT: &'static str = "img.dat";
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

fn search_for_datasets(direcotry: &str, search_ext: &Vec<String>, cur_depth: u32, activities: &Vec<activity::Activity>, db_client: &mut Client, verbose: bool) -> Result<(), std::io::Error>
{
    let dirs = data_walker::get_dirs(direcotry).unwrap();
    for dir in dirs
    {
        if let Some(dir_name) = dir
        {
            if verbose
            {
                println!("dir: {}", dir_name);
            }
            if dir_name.ends_with(STR_IMG_DAT)
            {
                let hdf5_files = data_walker::saerch_hdf5(&dir_name, search_ext).unwrap();
                println!("found {} hdf5 files in {}", hdf5_files.len(), dir_name);

                if hdf5_files.len() > 0
                {
                    let path = Path::new(&direcotry);
                    if let Some(pi_name) = path.file_stem()
                    {
                        //println!("{}", last_folder.to_str().unwrap());
                        let (found_activity, found_experiementer) = activity::search_for_pi_activity(activities, pi_name.to_str().unwrap());
                        if found_activity.is_some()
                        {
                            let activity = found_activity.unwrap();
                            println!("{:?} {:?} {:?}", activity.activityId, activity.experimentId, pi_name);
                            println!{"{:?} {:?} {:?}", activity.beamtime.proposal.gupId, activity.beamtime.proposal.proposalTitle, activity.beamtime.proposalStatus};
                        }
                        if found_experiementer.is_some()
                        {
                            let experimenter = found_experiementer.unwrap();
                            let pi_user = db_user::get_user_by_badge(db_client, experimenter.badge.parse::<u32>().unwrap()).unwrap();
                            if pi_user.is_none()
                            {
                                let pi_user = db_user::DbUser::from_experimenter(experimenter);
                                db_user::insert_user(db_client, &pi_user).unwrap();
                            }
                           
                        }
                    }
                    
                    for hdf5_file in hdf5_files
                    {
                        println!("found hdf5 file {}", hdf5_file);
                        //let mut xrf_dataset = data_walker::XrfDataset::new();
                        //xrf_dataset.load_from_hdf5(&hdf5_file).unwrap();
                    }
                    
                }
            }
            else if cur_depth > 0
            {
                let new_depth = cur_depth - 1;
                let _ = search_for_datasets(&dir_name, search_ext, new_depth, activities, db_client, verbose);
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
        db_user::print_all_user(&mut db_client).unwrap();
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

            let activities: Vec<activity::Activity> = serde_json::from_str(&beam_schedule).unwrap();

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

            search_for_datasets(args.search_dir.as_ref().unwrap(), &analyzed_search_ext, args.num_recursive, &activities,  &mut db_client, args.verbose).unwrap();
        }
    }

}
