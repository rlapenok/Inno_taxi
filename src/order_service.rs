pub mod response {
    tonic::include_proto!("response");}        

use response::{{Mark,ResponseFromServer},response_from_grc_server::ResponseFromGrc};
use response::response_from_grc_server::ResponseFromGrcServer;


use rdkafka::{{config::ClientConfig},consumer::{ Consumer, BaseConsumer}, Message};
use tonic::{Status,Request,Response,transport::{Server}};
use serde::{Serialize,Deserialize};


#[derive(Serialize,Deserialize,Debug)]
pub enum TaxiType{
    Economy,
    Comfort,
    Business,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Msg<'a,'b>{
    pub phone:i32,
    pub name: & 'a str,
    pub taxi_type:&'b str,
}

#[derive(Default)]
struct ResponseToUser{}

#[tonic::async_trait]
impl ResponseFromGrc for ResponseToUser{

    async fn send(&self,req:Request<Mark>)->Result<Response<ResponseFromServer>,Status>{

        let cons:BaseConsumer=ClientConfig::new().set("bootstrap.servers", "localhost:9092").set("group.id", "test-consumer-group").create().unwrap();

        cons.subscribe(&["UserEconomy"]).unwrap();    
            let number=req.into_inner().clone().phone;
        loop {
            match cons.poll(None){
                Some(info)=>match info {
                        Ok(msg)=> {
                            
                        if number==serde_json::from_slice::<Msg>(msg.payload().unwrap()).unwrap().phone
                    {
                        println!("{}",serde_json::from_slice::<Msg>(msg.payload().unwrap()).unwrap().phone);
                        break Ok(Response::new(ResponseFromServer {type_response :0}))
                    }
                    else {
                        {
                            break Ok(Response::new(ResponseFromServer { type_response: 1 }))
                        }
                    }
                }
                Err(_e)=> break Ok(Response::new(ResponseFromServer {type_response :0})),
                    
                }
                None=>break Err(Status::not_found("message".to_string())),
            }
        }

    }
}

#[tokio::main]
async fn main()->Result<(),Box<dyn std::error::Error>>{

    let adr="127.0.0.1:5050".parse().unwrap();

        Server::builder()
        .add_service(ResponseFromGrcServer::new(ResponseToUser::default()))
        .serve(adr).await?;
        Ok(())
}
