use std::env;
use std::fs::File;
use std::io::{self, Read};
use reqwest::{blocking};

mod database;
mod data_walker;
mod beamtime;
mod synco_runs;

fn read_json_from_file(file_path: &str) -> Result<String, io::Error> 
{
    // Open the file
    let mut file = File::open(file_path)?;

    // Read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn read_json_from_url(url_path: &str) -> Result<String, reqwest::Error> 
{
    let resp = reqwest::blocking::get(url_path)?;
    let body = resp.text()?;
    Ok(body)
}

fn help()
{
    eprintln!("Usage: [--test-users | --insert-runs | --get-beamtime | --from-file --from-url] ");
    //eprintln!("mic_db_fill <options> directory");
    //eprintln!("\t -r : recursive scan");
    //eprintln!("\t -e : export counts png");
}

fn main() 
{
    let args = env::args().collect::<Vec<String>>();
    if args.len() > 1
    {
        let mut json_data = String::new();
        let mut expr_lastname = String::new();
        let mut opt_insert_runs = false;
        let mut opt_get_beam_time = false;
        let mut config = data_walker::Config::new();
        for i in 1..args.len()
        {
            if args[i] == "-r"
            {
                println!("recursive");
                config.recursive = true;
            }
            else if args[i] == "-e"
            {
                println!("export counts png");
                config.export_counts_png = true;
            }
            else if args[i] == "--test-users"
            {
                println!("testing database conn");
                database::db_print_users().unwrap();
            }
            else if args[i] == "--from-file"
            {
                let file_path = &args[i+1];
                println!("reading from file {}", file_path);
                json_data = match read_json_from_file(file_path) 
                {
                    Ok(data) => data,
                    Err(e) =>  String::new(),
                };
            }
            else if args[i] == "--from-url"
            {
                let url_path = &args[i+1];
                println!("reading from url {}", url_path);
                json_data = match read_json_from_url(url_path) 
                {
                    Ok(data) => data,
                    Err(e) =>  String::new(),
                };
            }
            else if args[i] == "--insert-runs"
            {
                opt_insert_runs = true;
            }
            else if args[i] == "--get-beamtime"
            {
                opt_get_beam_time = true;
                expr_lastname = args[i+1].clone();
            }
        }
        
        if opt_insert_runs && json_data.len() > 0
        {
            synco_runs::fill_syncotron_runs(&json_data).unwrap();
        }
        if opt_get_beam_time && json_data.len() > 0 && expr_lastname.len() > 0
        {
            beamtime::parse_activity(&json_data, &expr_lastname);
        }
        //let _ = saerch_hdf5(config);
    }
    else 
    {
        help();
    }
}
