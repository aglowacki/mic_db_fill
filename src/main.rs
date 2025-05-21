use std::env;
use std::fs::File;
use std::io::{self, Read};
use reqwest;
use clap::Parser;

use tokio;

mod db_user;
mod data_walker;
mod activity;
mod beamtime;
mod synco_runs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Scan recursively
    #[arg(short, long, action)]
    scan_recursive: bool,

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
    pi_last_name: Option<String>,
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

fn help()
{
    eprintln!("Usage: [--test-users | --insert-runs | --get-beamtime | --from-file --from-url] ");
    //eprintln!("mic_db_fill <options> directory");
    //eprintln!("\t -r : recursive scan");
    //eprintln!("\t -e : export counts png");
}

#[tokio::main] 
async fn main() 
{
    let args = Args::parse();
    /*    
    if args.test
    {
        println!("testing database conn");
        db_user::db_print_users().unwrap();
    }
    */
    if args.pi_last_name.is_some()
    {
        let pi_name = args.pi_last_name.clone().unwrap();
        if args.filename.is_some()
        {
            let filename = args.filename.clone().unwrap();
            println!("reading from file {}", filename);
            let json_data = match read_json_from_file(&filename) 
            {
                Ok(data) => data,
                Err(e) =>  String::new(),
            };
            if args.verbose
            {
                println!("json data: {}", json_data);
            }
            let _ = beamtime::parse_beamtime(&json_data, &pi_name);
        }
        else if args.run.is_some() && args.beamline.is_some()
        {
            let run = args.run.clone().unwrap();
            let beamline = args.beamline.clone().unwrap();
            //let url_path = "https://beam-api.aps.anl.gov/beamline-scheduling/sched-api/beamtimeRequests/findBeamtimeRequestsByRunAndBeamline/2025-1/2-ID-D";
            //let mut url_path = "https://beam-api.aps.anl.gov/beamline-scheduling/sched-api/beamtimeRequests/findBeamtimeRequestsByRunAndBeamline/".to_owned();
            let mut url_path = "https://beam-api.aps.anl.gov/beamline-scheduling//sched-api/activity/findByRunNameAndBeamlineId/".to_owned();
            url_path.push_str(&run);
            url_path.push_str("/");
            url_path.push_str(&beamline);
            println!("reading from url {}", url_path);
            let json_data = match futures::executor::block_on(read_json_from_url(&url_path))
            {
                Ok(data) => data,
                Err(e) => panic!("{}",e.to_string()), //println!("{}",e.to_string()),
            };
            if args.verbose
            {
                println!("json data: {}", json_data);
            }
            //let t  = beamtime::parse_beamtime(&json_data, &pi_name);
            let t  = activity::parse_activity(&json_data, &pi_name);
            t.unwrap();
        }
    }
    //let _ = saerch_hdf5(config);
}
