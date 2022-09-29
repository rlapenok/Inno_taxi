

use diesel::{Insertable,Queryable};
use serde::{Serialize,Deserialize};
use diesel::prelude::*;
use rdkafka::util::Timeout;
use rdkafka::{ClientConfig,producer::{FutureProducer,FutureRecord}};
use async_trait::async_trait;
use std::sync::Arc;

//Внутреннее
use crate::{schema::list_of_drivers};
use crate::models::traits::{InOn,MsgToKafka};
use crate::models::delivery_kafka_msg::typeDeliveryKafkaMsg;
use crate::models::order::Msg;
#[derive(Serialize,Deserialize,Debug,Clone,Copy)]
pub enum TaxiType{
    Economy,
    Comfort,
    Business,
}

#[derive(Insertable,Queryable,Serialize,Deserialize)]
#[diesel(table_name=list_of_drivers)]
pub struct Driver{
     phone:i32,
     name:Option<String>,
     email:Option<String>,
     password:String,
     taxi_type:String,
}

impl InOn for Driver {
    fn singup(&self,conn:& mut PgConnection)->String {
        use crate::schema::list_of_drivers::dsl::*;    
        match diesel::insert_into(list_of_drivers).values(self).execute(conn){
            Ok(_)=>String::from("Ok"),
            Err(e)=>format!("{}",e.to_string()),
        }
}
    fn singin(&self,conn:& mut PgConnection)->String {
        use crate::schema::list_of_drivers::dsl::*;
        
        let result= match list_of_drivers.filter(phone.eq(self.phone)).filter(password.eq(self.password.as_str())).get_result::<Driver>(conn){
        Ok(_data)=>String::from("Ok"),
        Err(e)=>format!("{}",e.to_string())
      };
      result
    }

}
#[async_trait]
impl MsgToKafka<Driver> for Driver {
    async fn send_msg_to_kafka(msg:Arc<Msg>)->typeDeliveryKafkaMsg  {
        let topic=match &msg.taxi_type {
            TaxiType::Economy=>"UserEconomy",
            TaxiType::Comfort=>"UserComfort",
            TaxiType::Business=>"UserBusiness",
        };    
        let data=serde_json::to_string_pretty(&msg.as_ref()).unwrap();
        
        //Если все Ок, то ответ kafka - номер партиции и оффсета
        //Если нет, то kafka Producer возвращает KafkaError b OwnedMessage
        let prod:FutureProducer=ClientConfig::new().set("bootstrap.servers", "localhost:9092").create().unwrap();
        match prod.send(FutureRecord::to(topic).payload(&data).key("2"), Timeout::Never).await {
        Ok((partition,offset))=> {
            //в текущих обстоятельствах просто их дропаю
            drop(partition);
            drop(offset);
            return typeDeliveryKafkaMsg::Ok(msg)
        },
        Err((e,msg))=>{
            // в текущих обстоятельствах просто дропаю
            drop(msg);
            return typeDeliveryKafkaMsg::KafkaError(e)
        } 
    };

}
}


/*impl Driver {
    pub fn all(conn:&mut PgConnection)->Vec<Driver>{
        use crate::schema::list_of_drivers::dsl::*;
        list_of_drivers.load::<Driver>(conn).unwrap()
    }
    
}*/
