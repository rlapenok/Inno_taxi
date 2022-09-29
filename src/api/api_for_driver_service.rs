use actix_web::{post, Responder, HttpResponse};
use actix_web::web::Json;
use std::sync::Arc;

//Внутреннee
use crate::dbconn::conn_db_drivers;
use crate::models::{traits::{InOn,MsgToKafka},driver::Driver,order::Msg};

#[post("/singin")]
pub async fn singin(data:Json<Driver>)->impl Responder
{
    let mut conn=conn_db_drivers();
    HttpResponse::Ok().json(data.into_inner().singin(&mut conn))
}

#[post("/singup")]
pub async fn singup(data:Json<Driver>)->impl Responder{
    let mut conn=conn_db_drivers();
    HttpResponse::Ok().json(data.into_inner().singup(& mut conn))
}

/*#[get("/all")]
pub async fn all()->impl Responder{
    let mut conn=conn_db_drivers();
    HttpResponse::Ok().json(Driver::all(&mut conn))
}*/

#[post("/order")]
pub async fn order(msg:Json<Msg>)-> impl Responder{
    let data=Arc::new(msg.into_inner());
    let response=Driver::send_msg_to_kafka(data.clone()).await;
        HttpResponse::Ok().body("asds")
}
