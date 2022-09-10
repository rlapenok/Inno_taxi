use actix_web::{post, Responder, HttpResponse,get};
use actix_web::web::Json;
use crate::dbconn::conn;
use crate::models::traits::InOn;
use crate::models::driver::Driver;

#[post("/singin")]
async fn singin   (data:Json<Driver>)->impl Responder
{
    let mut conn=conn();
    HttpResponse::Ok().body(data.into_inner().singin(&mut conn))
}

#[post("/singup")]
async fn singup(data:Json<Driver>)->impl Responder{
    let mut conn=conn();
    HttpResponse::Ok().body(data.into_inner().singup(& mut conn))
}

#[get("/all")]
async fn all()->impl Responder{
    let mut conn=conn();
    HttpResponse::Ok().json(Driver::all(&mut conn))
}