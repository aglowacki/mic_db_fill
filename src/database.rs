use postgres::Client;
//use chrono::{DateTime, Utc, NaiveDateTime};
use crate::activity;


//--------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct UserAccessControl
{
    id: i32,
    pub level: String,
    pub description: String,
}

impl UserAccessControl 
{
    pub fn new(my_id: i32, user_access_control: &str, descr: &str) -> Self 
    {
        UserAccessControl { id: my_id, level: String::from(user_access_control), description: String::from(descr) }
    }
}
#[derive(Debug, Clone)]
pub struct User 
{
    pub badge: i32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub institution: String,
    pub email: String,
    pub user_access_control: UserAccessControl,
}

impl User 
{
    pub fn from_db(row: &postgres::Row) -> Self 
    {
        User { badge: row.get(0), username: row.get(1), first_name: row.get(2), last_name: row.get(3), institution: row.get(4), email: row.get(5), user_access_control: UserAccessControl::new(row.get(6), row.get(7), row.get(8)) }
    }
    pub fn from_experimenter(experimenter: &activity::Experimenter, uac: &UserAccessControl) -> Self 
    {
        User { badge: experimenter.badge.parse::<i32>().unwrap(), username: experimenter.email.clone().unwrap(), first_name: experimenter.firstName.clone(), last_name: experimenter.lastName.clone(), institution: experimenter.institution.clone(), email: experimenter.email.clone().unwrap(), user_access_control: uac.clone() }
    }
}
#[derive(Debug, Clone)]
pub struct Proposal
{
    pub id: i32,
    pub title: String,
    pub proprietaryFlag: String,
    pub mailInFlag: String,
    pub status: String,
}

impl Proposal 
{
    pub fn from_proposal(proposal: &activity::Proposal) -> Self 
    {
        Proposal { id: proposal.gupId.unwrap(), title: proposal.proposalTitle.clone().unwrap(), proprietaryFlag: proposal.proprietaryFlag.clone().unwrap(), mailInFlag: proposal.mailInFlag.clone().unwrap(), status: String::from("Done") }
    }
}
#[derive(Debug, Clone)]
pub struct DataStore
{
    id: i32,
    root: String,
    path: String,
}

#[derive(Debug, Clone)]
pub struct Beamline
{
    id: i32,
    name: String,
    acronym: String, 
    division: String,
    link: String,
}

impl Beamline
{
    pub fn get_id(&self) -> i32
    {
        return self.id;
    }
}

#[derive(Debug, Clone)]
pub struct ScanType
{
    id: i32,
    name: String,
    description: String,
}

#[derive(Debug, Clone)]
pub struct SyncRun
{
    id: i32,
    pub name: String,
    start_timestamp: std::time::SystemTime,
    end_timestamp: std::time::SystemTime,
}

impl SyncRun
{
    pub fn get_id(&self) -> i32
    {
        return self.id;
    }
}

#[derive(Debug, Clone)]
pub struct Dataset
{
    id: i32,
    beamline_id: i32,
    syncotron_run_id: i32,
    scan_type_id: i32,
    path: String,
    acquisition_timestamp: std::time::SystemTime,
}

impl Dataset
{
    pub fn new(beamline_id: i32, syncotron_run_id: i32, scan_type_id: i32, ppath: &str, acquisition_timestamp: std::time::SystemTime) -> Self 
    {
        Dataset { id: 0, beamline_id: beamline_id, syncotron_run_id: syncotron_run_id, scan_type_id: scan_type_id, path: ppath.to_owned(), acquisition_timestamp: acquisition_timestamp }
    }

    pub fn get_id(&self) -> i32
    {
        return self.id;
    }
}

#[derive(Debug, Clone)]
pub struct ExperimenterRole
{
    id: i32,
    role: String,
}

impl ExperimenterRole
{
    pub fn get_id(&self) -> i32
    {
        return self.id;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Experimenter
{
    datasest_id: i32,// integer REFERENCES datasets (id),
    user_badge: i32, //integer REFERENCES users (badge),
    proposal_id: i32, //integer REFERENCES proposals (id),
    experiment_role_id: i32 //integer REFERENCES experiment_roles (id)
}

impl Experimenter
{
    pub fn new(dataset_id: i32, user_badge: i32, proposal_id: i32, experiment_role_id: i32) -> Self 
    {
        Experimenter { datasest_id: dataset_id, user_badge: user_badge, proposal_id: proposal_id, experiment_role_id: experiment_role_id }
    }
}

pub fn print_all_user(client: &mut Client) -> Result<(), postgres::Error> 
{
    
    for row in client.query("SELECT u.badge, u.username, u.first_name, u.last_name, u.institution, u.email, uac.id, uac.level, uac.description FROM users u INNER JOIN user_access_control uac ON u.user_access_control_id = uac.id;", &[])? 
    {
        
        let user = User::from_db(&row);
        
        println!("Badge: {}, Username: {}, Access Level {}", user.badge, user.username, user.user_access_control.level);
    }

    Ok(())
}

// -------------- Get Functions -----------------------------

pub fn get_user_by_badge(db_client: &mut Client, badge: u32) -> Result<Option<User>, postgres::Error> 
{
    for row in db_client.query("SELECT u.badge, u.username, u.first_name, u.last_name, u.institution, u.email, uac.id, uac.level, uac.description FROM users u INNER JOIN user_access_control uac ON u.user_access_control_id = uac.id WHERE u.badge == {};", &[&badge])? 
    {
        let user_access_control = UserAccessControl 
        {
            id: row.get(6),
            level: row.get(7),
            description: row.get(8),
        };
        let user = User 
        {
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

pub fn get_all_staff_users(db_client: &mut Client, staff: &mut Vec<User>) -> Result<(), postgres::Error> 
{
    for row in db_client.query("SELECT u.badge, u.username, u.first_name, u.last_name, u.institution, u.email, uac.id, uac.level, uac.description FROM users u INNER JOIN user_access_control uac ON u.user_access_control_id = uac.id WHERE uac.level = 'Staff';", &[])? 
    {
        let user_access_control = UserAccessControl 
        {
            id: row.get(6),
            level: row.get(7),
            description: row.get(8),
        };
        staff.push(User 
        {
            badge: row.get(0),
            username: row.get(1),
            first_name: row.get(2),
            last_name: row.get(3),
            institution: row.get(4),
            email: row.get(5),
            user_access_control: user_access_control,
        });
    }
    Ok(())
}

pub fn get_access_control(db_client: &mut Client, uac: &mut std::collections::HashMap<String, UserAccessControl>) -> Result<(), postgres::Error> 
{
    for row in db_client.query("SELECT uac.id, uac.level, uac.description FROM user_access_control uac", &[])? 
    {
        uac.insert(row.get(1), UserAccessControl 
        {
            id: row.get(0),
            level: row.get(1),
            description: row.get(2),
        });
    }
    Ok(())
}

pub fn get_experimenter_roles(db_client: &mut Client, roles: &mut std::collections::HashMap<String, ExperimenterRole>) -> Result<(), postgres::Error> 
{
    for row in db_client.query("SELECT id, role FROM experiment_roles", &[])? 
    {
        roles.insert(row.get(1), ExperimenterRole 
        {
            id: row.get(0),
            role: row.get(1),
        });
    }
    Ok(())
}

pub fn get_sync_runs(db_client: &mut Client, runs: &mut std::collections::HashMap<String, SyncRun>) -> Result<(), postgres::Error> 
{
    for row in db_client.query("SELECT id, name, start_timestamp, end_timestamp FROM syncotron_runs", &[])? 
    {
        runs.insert(row.get(1), SyncRun 
        {
            id: row.get(0),
            name: row.get(1),
            start_timestamp: row.get(2),
            end_timestamp: row.get(3),
        });
    }
    Ok(())
}

pub fn get_scan_types(db_client: &mut Client, scan_types: &mut std::collections::HashMap<String, ScanType>) -> Result<(), postgres::Error> 
{
    for row in db_client.query("SELECT id, name, description FROM scan_type", &[])? 
    {
        scan_types.insert(row.get(1), ScanType 
        {
            id: row.get(0),
            name: row.get(1),
            description: row.get(2),
        });
    }
    Ok(())
}

pub fn get_beamlines(db_client: &mut Client, beamlines: &mut std::collections::HashMap<String, Beamline>) -> Result<(), postgres::Error> 
{
    for row in db_client.query("SELECT id, name, acronym, division, link FROM beamlines", &[])? 
    {
        beamlines.insert(row.get(2), Beamline 
        {
            id: row.get(0),
            name: row.get(1),
            acronym: row.get(2),
            division: row.get(2),
            link: row.get(2),
        });
    }
    Ok(())
}


// ----------- Insert Functions -----------------------------

pub fn insert_user(db_client: &mut Client, user: &User) -> Result<u64, postgres::Error> 
{
    let query = "INSERT INTO users (badge, username, first_name, last_name, institution, email, user_access_control_id) VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT DO NOTHING";
    let params: &[&(dyn postgres::types::ToSql + Sync)] = &[&user.badge, &user.username, &user.first_name, &user.last_name, &user.institution, &user.email, &user.user_access_control.id];
    return db_client.execute(query, params)
}

pub fn insert_experimenter(db_client: &mut Client, user: &Experimenter) -> Result<u64, postgres::Error> 
{
    let query = "INSERT INTO experimenters (datasest_id, user_badge, proposal_id, experiment_role_id) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING";
    let params: &[&(dyn postgres::types::ToSql + Sync)] = &[&user.datasest_id, &user.user_badge, &user.proposal_id, &user.experiment_role_id];
    return db_client.execute(query, params)
}

pub fn insert_proposal(db_client: &mut Client, proposal: &Proposal) -> Result<i32, postgres::Error> 
{
    let query = "INSERT INTO proposals (id, title, proprietaryFlag, mailInFlag, status) VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING RETURNING id";
    let params: &[&(dyn postgres::types::ToSql + Sync)] = &[&proposal.id, &proposal.title, &proposal.proprietaryFlag, &proposal.mailInFlag, &proposal.status];
    for row in  db_client.query(query, params)?
    {
        let id:i32 = row.get(0);
        return Ok(id)
    }
    Ok(-1)
}

pub fn insert_dataset(db_client: &mut Client, dataset: &Dataset) -> Result<i32, postgres::Error> 
{
    let query = "INSERT INTO datasets (path, acquisition_timestamp, beamline_id, syncotron_run_id, scan_type_id) VALUES ($1, $2, $3, $4, $5) RETURNING id";
    let params: &[&(dyn postgres::types::ToSql + Sync)] = &[&dataset.path, &dataset.acquisition_timestamp, &dataset.beamline_id, &dataset.syncotron_run_id, &dataset.scan_type_id];
    for row in  db_client.query(query, params)?
    {
        let id:i32 = row.get(0);
        return Ok(id)
    }
    Ok(-1)
}
