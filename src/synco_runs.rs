//use tokio_postgres::{NoTls, Error};
use std::env;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use postgres::{Client, NoTls, Error};

#[derive(Deserialize)]
struct SyncotronRun
{
    runId: i32,
    runName: String,
    startTime: String,
    endTime: String,
    version: i32
}

impl SyncotronRun
{
    fn parse_times(&self) -> Result<(DateTime<Utc>, DateTime<Utc>), chrono::ParseError> {
        let start_time: DateTime<Utc> = self.startTime.parse().expect("Failed to parse datetime");
        let end_time:  DateTime<Utc> = self.endTime.parse().expect("Failed to parse datetime");
        
        Ok((start_time, end_time))
    }

}

pub fn fill_syncotron_runs(json_data: &str) -> Result<(), Error> 
{
    let psql_conn_str = env::var("SVC_PSQL_CONN_STR").unwrap();
    let mut client = Client::connect(&psql_conn_str, NoTls)?; 

    let runs: Vec<SyncotronRun> = serde_json::from_str(json_data).expect("Failed to parse JSON");

    for run in runs 
    {
        let (start_time, end_time) = run.parse_times().expect("Failed to parse times");
        match client.execute("INSERT INTO syncotron_runs ( name, start_timestamp, end_timestamp) VALUES ($1, $2, $3 )", &[ &run.runName, &start_time, &end_time ], )
        {
            Ok(_) => { println!("inserted {}", run.runName) }
            Err(e) => { println!("{:?}", e); }
        }

    }
    Ok(())
}
