use actix_web::{post, Responder, HttpResponse,get};
use actix_web::web::Json;
use crate::dbconn::conn;
use crate::models::traits::InOn;
use crate::models::user::User;


#[post("/singin")]
pub async fn singin   (data:Json<User>)->impl Responder
{
    let mut conn=conn();
    HttpResponse::Ok().body(data.into_inner().singin(&mut conn))
}

#[post("/singup")]
pub async fn singup(data:Json<User>)->impl Responder{
    let mut conn=conn();
    HttpResponse::Ok().body(data.into_inner().singup(&mut conn))
}

#[get("/all")]
pub async fn all()->impl Responder{
    let mut conn=conn();
    HttpResponse::Ok().json(User::all(&mut conn))
}