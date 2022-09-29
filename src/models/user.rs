use diesel::{Insertable, Queryable,PgConnection};
use serde::{Serialize,Deserialize};
use diesel::prelude::*;
use rdkafka::{ClientConfig,producer::{FutureProducer,FutureRecord},util::Timeout};
use async_trait::async_trait;
use std::sync::Arc;

//Внутреннее
use crate::models::{traits::{InOn,MsgToKafka},driver::TaxiType,delivery_kafka_msg::{typeDeliveryKafkaMsg},order::Msg};
use crate::schema::list_of_users;

#[derive(Insertable,Queryable,Serialize,Deserialize)]
#[diesel(table_name=list_of_users)]
pub struct User{
    phone:i32,
    name:Option<String>,
    email:Option<String>,
    password:String,
}

impl InOn for User {
    fn singup(&self,conn:& mut PgConnection) ->String{
        use crate::schema::list_of_users::dsl::*;    
         match diesel::insert_into(list_of_users).values(self).execute(conn){
            Ok(_)=>String::from("Ok"),
            Err(e)=>format!("{}",e.to_string())
         }
}

    fn singin(&self,conn:& mut PgConnection)->String {
        use crate::schema::list_of_users::dsl::*;
      let result= match  list_of_users.filter(phone.eq(self.phone)).filter(password.eq(self.password.as_str())).get_result::<User>(conn){
        Ok(_data)=>String::from("Ok"),
        Err(e)=>format!("{}",e.to_string())
      };
      result
    }
}
#[async_trait]
impl MsgToKafka<User> for User {
    async fn send_msg_to_kafka(msg:Arc<Msg>)->typeDeliveryKafkaMsg  {
        let topic=match &msg.taxi_type {
            TaxiType::Economy=>"UserEconomy",
            TaxiType::Comfort=>"UserComfort",
            TaxiType::Business=>"UserBusiness",
        };    
        let data=serde_json::to_string_pretty(&msg).unwrap();

        let prod:FutureProducer=ClientConfig::new().set("bootstrap.servers", "localhost:9092").create().unwrap();
        println!("tut");

         /*Создаю FutureProducer, который отправляет сообщение в kafka и 
           Если все Ок, то ответ kafka - номер партиции и оффсета
           Если нет, то kafka Producer возвращает KafkaError и OwnedMessage*/
        let response=match prod.send(FutureRecord::to(topic).payload(&data).key("1"), Timeout::Never).await {
        Ok((partition,offset))=> {
            //в текущих обстоятельствах просто их дропаю
            drop(partition);
            drop(offset);
            return typeDeliveryKafkaMsg::Ok
        },
        Err((e,msg))=>{
            // в текущих обстоятельствах просто дропаю
            drop(msg);
            return typeDeliveryKafkaMsg::KafkaError(e)
        } 
    };
}
}
/*impl User {
    pub fn all(conn:&mut PgConnection)->Vec<User>{
        use crate::schema::list_of_users::dsl::*;
        list_of_users.load::<User>(conn).unwrap()
    }
    
}*/