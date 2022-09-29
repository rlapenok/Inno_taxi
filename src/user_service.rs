use actix_web::{HttpServer,App};         


//Внутренние модули
mod models;
mod dbconn;
mod api;
mod schema;


use crate::api::api_for_user_service::*;



#[actix_web::main]
async fn main()->std::io::Result<()> {
    HttpServer::new(move ||{
        App::new()
        .service(singin)
        .service(singup)
        //.service(all)
        .service(order)
        .service(find)
    }).bind(("127.0.0.1",7070)).unwrap().run().await
}
