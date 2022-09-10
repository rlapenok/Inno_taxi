use diesel::pg::PgConnection;
use std::env;
use diesel::prelude::*;
use dotenvy::dotenv;



pub fn conn_db_users()->PgConnection{
    dotenv().ok();
    let user_db_url=env::var("user_db_url").expect("Not found DB url for Users");
    PgConnection::establish(&user_db_url).unwrap()
}

pub fn conn_db_drivers()->PgConnection{
    dotenv().ok();
    let user_db_url=env::var("driver_db_url").expect("Not found DB url for Drivers");
    PgConnection::establish(&user_db_url).unwrap()
}