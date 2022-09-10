use actix_web::{post, Responder, HttpResponse,get};
use actix_web::web::Json;

//Внутреннee
use crate::dbconn::conn_db_drivers;
use crate::models::traits::InOn;
use crate::models::driver::Driver;

#[post("/singin")]
async fn singin   (data:Json<Driver>)->impl Responder
{
    let mut conn=conn_db_drivers();
    HttpResponse::Ok().body(data.into_inner().singin(&mut conn))
}

#[post("/singup")]
async fn singup(data:Json<Driver>)->impl Responder{
    let mut conn=conn_db_drivers();
    HttpResponse::Ok().body(data.into_inner().singup(& mut conn))
}

#[get("/all")]
async fn all()->impl Responder{
    let mut conn=conn_db_drivers();
    HttpResponse::Ok().json(Driver::all(&mut conn))
}