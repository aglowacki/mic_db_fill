use postgres::{Client, NoTls, Error};
//use std::collections::HashMap;
use std::env;

//mod dataset;
//use dataset::XRF_Dataset;

struct DbUser 
{
    badge: i32,
    username: String,
    level: String
}

pub fn db_print_users() -> Result<(), Error> 
{
    let psql_conn_str = env::var("SVC_PSQL_CONN_STR").unwrap();
    let mut client = Client::connect(&psql_conn_str, NoTls)?;
    
    for row in client.query("SELECT u.badge,u.username,ut.level FROM \"Users\" u INNER JOIN \"UserTypes\" ut ON u.user_type = ut.id;", &[])? 
    {
        let user = DbUser {
            badge: row.get(0),
            username: row.get(1),
            level: row.get(2),
        };
        println!("Badge: {}, Username: {}, Access Level {}", user.badge, user.username, user.level);
    }

    Ok(())

}
