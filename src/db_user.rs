use postgres::Client;

use crate::activity::{Experimenter, Proposal};

//--------------------------------------------------------------

pub struct DbUserAccessControl
{
    id: i32,
    level: String,
    description: String,
}

impl DbUserAccessControl 
{
    pub fn new(my_id: i32, user_access_control: &str, descr: &str) -> Self 
    {
        DbUserAccessControl { id: my_id, level: String::from(user_access_control), description: String::from(descr) }
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
    user_access_control: DbUserAccessControl,
}

impl DbUser 
{
    pub fn from_db(row: &postgres::Row) -> Self 
    {
        DbUser { badge: row.get(0), username: row.get(1), first_name: row.get(2), last_name: row.get(3), institution: row.get(4), email: row.get(5), user_access_control: DbUserAccessControl::new(row.get(6), row.get(7), row.get(8)) }
    }
    pub fn from_experimenter(experimenter: &Experimenter) -> Self 
    {
        DbUser { badge: experimenter.badge.parse::<i32>().unwrap(), username: experimenter.email.clone().unwrap(), first_name: experimenter.firstName.clone(), last_name: experimenter.lastName.clone(), institution: experimenter.institution.clone(), email: experimenter.email.clone().unwrap(), user_access_control: DbUserAccessControl::new(-1 , "Visitor", " ") }
    }
}

pub struct DbProposal
{
    id: i32,
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
    
    for row in client.query("SELECT u.badge, u.username, u.first_name, u.last_name, u.institution, u.email, uac.id, uac.level, uac.description FROM users u INNER JOIN user_access_control uac ON u.user_access_control_id = uac.id;", &[])? 
    {
        
        let user = DbUser::from_db(&row);
        
        println!("Badge: {}, Username: {}, Access Level {}", user.badge, user.username, user.user_access_control.level);
    }

    Ok(())
}

pub fn get_user_by_badge(client: &mut Client, badge: u32) -> Result<Option<DbUser>, postgres::Error> 
{
    for row in client.query("SELECT u.badge, u.username, u.first_name, u.last_name, u.institution, u.email, uac.id, uac.level, uac.description FROM users u INNER JOIN user_access_control uac ON u.user_access_control_id = uac.id WHERE u.badge == {};", &[&badge])? 
    {
        let user_access_control = DbUserAccessControl {
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
            user_access_control: user_access_control,
        };
        println!("Badge: {}, Username: {}, Access Level {}", user.badge, user.username, user.user_access_control.level);
        return Ok(Some(user));
    }

    Ok(None)
}


pub fn insert_user(client: &mut Client, user: &DbUser) -> Result<u64, postgres::Error> 
{
    let query = "INSERT INTO users (badge, username, first_name, last_name, institution, email, ) VALUES ($1, $2, $3, $4, $5, $6, $7)";
    let params: &[&(dyn postgres::types::ToSql + Sync)] = &[&user.badge, &user.username, &user.first_name, &user.last_name, &user.institution, &user.email, &user.user_access_control.level];
    return client.execute(query, params)
}   
