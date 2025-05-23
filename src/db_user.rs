use postgres::Client;
use std::env;

use crate::activity::{Experimenter, Proposal};

/*
use r2d2_postgres::postgres::{NoTls, Client};
use r2d2_postgres::PostgresConnectionManager;
//---------------------------------------------------------------

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref POOL: r2d2::Pool<PostgresConnectionManager<NoTls>> = {
        let manager = PostgresConnectionManager::new(
            // TODO: PLEASE MAKE SURE NOT TO USE HARD CODED CREDENTIALS!!!
            "host=localhost user=postgres password=password".parse().unwrap(),
            NoTls,
        );
        r2d2::Pool::new(manager).unwrap()
    };
}



pub fn get_player(id: i32) {
    // Use global client connection object:
    let mut client = POOL.get().unwrap();
    for row in client.query("SELECT * FROM public.\"User\" WHERE \"accountID\"=$1;",&[&id]).unwrap(){
        let id: i32 = row.get(0);
        let name: &str = row.get(1);

        println!("found player: {} {}", id, name);
    }
}
    */
//--------------------------------------------------------------


pub struct DbUserType 
{
    id: i64,
    level: String,
    description: String,
}

impl DbUserType 
{
    pub fn new(my_id: i64, user_type: &str, descr: &str) -> Self 
    {
        DbUserType { id: my_id, level: String::from(user_type), description: String::from(descr) }
    }
}

pub struct DbUser 
{
    badge: i32,
    username: String,
    first_name: String,
    last_name: String,
    institution: String,
    email: String,
    user_type: DbUserType,
}

impl DbUser 
{
    pub fn from_db(row: &postgres::Row) -> Self 
    {
        DbUser { badge: row.get(0), username: row.get(1), first_name: row.get(2), last_name: row.get(3), institution: row.get(4), email: row.get(5), user_type: DbUserType::new(row.get(6), row.get(7), row.get(8)) }
    }
    pub fn from_experimenter(experimenter: &Experimenter) -> Self 
    {
        DbUser { badge: experimenter.badge.parse::<i32>().unwrap(), username: experimenter.email.clone().unwrap(), first_name: experimenter.firstName.clone(), last_name: experimenter.lastName.clone(), institution: experimenter.institution.clone(), email: experimenter.email.clone().unwrap(), user_type: DbUserType::new(-1 , "Visitor", " ") }
    }
}

pub struct DbProposal
{
    id: i64,
    title: String,
    proprietaryFlag: String,
    mailInFlag: String,
    status: String,
}

impl DbProposal 
{
    pub fn from_proposal(proposal: &Proposal) -> Self 
    {
        DbProposal { id: proposal.gupId.unwrap(), title: proposal.proposalTitle.clone().unwrap(), proprietaryFlag: proposal.proprietaryFlag.clone().unwrap(), mailInFlag: proposal.mailInFlag.clone().unwrap(), status: String::from("Done") }
    }
}

pub struct DbDataStore
{
    id: i64,
    root: String,
    path: String,
}

pub struct DbBeamline
{
    id: i64,
    name: String,
    acronym: String, 
    division: String,
    link: String,
}

pub struct DbScanType
{
    id: i64,
    name: String,
    description: String,
}

pub struct DbSyncRun
{
    id: i64,
    name: String,
    start_timestamp: String, // timestamp
    end_timestamp: String, // timestamp
}

pub struct DbDataset
{
    id: i64,
    beamline_id: DbBeamline,
    syncotron_run_id: DbSyncRun,
    scan_type_id: DbScanType,
    data_store_id: DbDataStore,
    acquisition_timestamp: String,
}


pub fn print_all_user(client: &mut Client) -> Result<(), postgres::Error> 
{
    
    for row in client.query("SELECT u.badge, u.username, u.first_name, u.last_name, u.institution, u.email, ut.id, ut.level, ut.description FROM users u INNER JOIN user_types ut ON u.user_type_id = ut.id;", &[])? 
    {
        
        let user = DbUser::from_db(&row);
        
        println!("Badge: {}, Username: {}, Access Level {}", user.badge, user.username, user.user_type.level);
    }

    Ok(())
}

pub fn get_user_by_badge(client: &mut Client, badge: u32) -> Result<Option<DbUser>, postgres::Error> 
{
    for row in client.query("SELECT u.badge, u.username, u.first_name, u.last_name, u.institution, u.email, ut.id, ut.level, ut.description FROM users u INNER JOIN user_types ut ON u.user_type_id = ut.id WHERE u.badge == {};", &[&badge])? 
    {
        let user_type = DbUserType {
            id: row.get(6),
            level: row.get(7),
            description: row.get(8),
        };
        let user = DbUser {
            badge: row.get(0),
            username: row.get(1),
            first_name: row.get(2),
            last_name: row.get(3),
            institution: row.get(4),
            email: row.get(5),
            user_type: user_type,
        };
        println!("Badge: {}, Username: {}, Access Level {}", user.badge, user.username, user.user_type.level);
        return Ok(Some(user));
    }

    Ok(None)
}


pub fn insert_user(client: &mut Client, user: &DbUser) -> Result<u64, postgres::Error> 
{
    let query = "INSERT INTO users (badge, username, first_name, last_name, institution, email, ) VALUES ($1, $2, $3, $4, $5, $6, $7)";
    let params: &[&(dyn postgres::types::ToSql + Sync)] = &[&user.badge, &user.username, &user.first_name, &user.last_name, &user.institution, &user.email, &user.user_type.level];
    return client.execute(query, params)
}   


