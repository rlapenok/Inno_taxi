//Внешние
use actix_web::{post, get,Responder, HttpResponse,web::Json};
use rdkafka::error::KafkaError;
use tonic::transport::{Channel, channel};
pub mod response {
    tonic::include_proto!("response");} 
 use response::response_from_grc_client::ResponseFromGrcClient;
 use response::Mark; 
 use response::ResponseFromServer;
 use tokio::sync::oneshot;
 use tonic::Request;
use std::sync::Arc;

//Внутреннee
use crate::dbconn::conn_db_users;
use crate::models::traits::{InOn,MsgToKafka};
use crate::models::user::User;
use crate::models::order::Msg;
use crate::models::delivery_kafka_msg::DeliveryKafkaMsg;


#[post("/singin")]
pub async fn singin   (data:Json<User>)->impl Responder
{
    let mut conn=conn_db_users();
    HttpResponse::Ok().json(data.into_inner().singin(&mut conn))
}

#[post("/singup")]
pub async fn singup(data:Json<User>)->impl Responder{
    let mut conn=conn_db_users();
    HttpResponse::Ok().json(data.into_inner().singup(&mut conn))
}

/*#[get("/all")]
pub async fn all()->impl Responder{
    let mut conn=conn_db_users();
    HttpResponse::Ok().json(User::all(&mut conn))
}*/
#[derive(Debug)]
enum resp{
    Error(KafkaError),
    Ok(ResponseFromServer),
}
type abc=resp;
enum asd{
    Ok(Msg),
    Err(KafkaError)
}
#[post("/order")]
pub async fn order(msg: Json<Msg>)->impl Responder{
    let data=Arc::new(msg.into_inner());

    //Создаю передатчик(tx) и приемник(rx)
    let (tx,rx)=oneshot::channel();
    //async поток для отправки сообщения в kafka (async поток используется, т.к. внутри используется FutureProducer
        let thread_resp_from_kafka=tokio::spawn(async move {
                
            let resp=   match tx.send(User::send_msg_to_kafka(data.clone()).await) {
                Ok(_)=>{ return },
                Err(_)=>{return}
                    
                };
        }).await.unwrap();
        let resp_from_order_service=tokio::spawn(async move{
            match rx.await.unwrap() {
                DeliveryKafkaMsg::Ok=>{
                                  
                    
                    let channel=Channel::from_static("http://127.0.0.1:5050").connect().await.unwrap();
                    let mut client=ResponseFromGrcClient::new(channel);

                    let resp:ResponseFromServer=client.send(Request::new(Mark{ phone:data.phone})).await.unwrap().into_inner();
                    abc::Ok(resp)},
                DeliveryKafkaMsg::KafkaError(e)=>abc::Error(e)
            } 
        }).await.unwrap();
         HttpResponse::Ok().body("asdasa")
}

#[get("/find_message")]
async fn find()->impl Responder{

    HttpResponse::Ok()
}