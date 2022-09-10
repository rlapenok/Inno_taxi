use actix_web::{HttpServer,App};



//Внутренние модули
mod models;
mod dbconn;
mod api;
mod schema;


use crate::api::api_for_user_service::*;



#[actix_web::main]
async fn main()->std::io::Result<()> {
    HttpServer::new(||{
        App::new()
        .service(singin)
        .service(singup)
        .service(all)
    }).bind(("127.0.0.1",8080)).unwrap().run().await
}
