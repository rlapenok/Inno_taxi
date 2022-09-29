use actix_web::{HttpServer,App};



//Внутренние модули
mod models;
mod dbconn;
mod api;
mod schema;


use crate::api::api_for_driver_service::*;



#[actix_web::main]
async fn main()->std::io::Result<()> {
    HttpServer::new(||{
        App::new()
        .service(singin)
        .service(singup)
        //.service(all)
        .service(order)
    }).bind(("127.0.0.1",6060)).unwrap().run().await
}