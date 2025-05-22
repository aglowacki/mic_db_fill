use postgres::{Client, NoTls, Error};
//use std::collections::HashMap;
use std::env;

//mod dataset;
//use dataset::XRF_Dataset;

struct DbUser 
{
    badge: i32,
    username: String,
    first_name: String,
    last_name: String,
    institution: String,
    email: String,
    user_type: String,
}

/*
*/
pub fn db_print_users() -> Result<(), postgres::Error> 
{
    let psql_conn_str = env::var("SVC_PSQL_CONN_STR").unwrap_or(String::from("postgresql://localhost/mydata"));
    let mut client = Client::connect(&psql_conn_str, NoTls)?;
    
    for row in client.query("SELECT u.badge,u.username,ut.level FROM users u INNER JOIN user_types ut ON u.user_type_id = ut.id;", &[])? 
    {
        let user = DbUser {
            badge: row.get(0),
            username: row.get(1),
            first_name: row.get(2),
            last_name: row.get(3),
            institution: row.get(4),
            email: row.get(5),
            user_type: row.get(6),
        };
        println!("Badge: {}, Username: {}, Access Level {}", user.badge, user.username, user.user_type);
    }

    Ok(())

}

