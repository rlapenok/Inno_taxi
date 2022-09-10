use diesel::pg::PgConnection;
use std::env;
use diesel::prelude::*;
use dotenvy::dotenv;

pub fn conn()->PgConnection{
    dotenv().ok();
    let user_db_url=env::var("user_db_url").expect("Not found DB url for Users");
    PgConnection::establish(&user_db_url).unwrap()
}