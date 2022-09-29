use async_trait::async_trait;
use diesel::prelude::*;
use std::sync::Arc;


use crate::models::{order::Msg};
use crate::models::delivery_kafka_msg::typeDeliveryKafkaMsg;

pub trait InOn{
    fn singin(&self,conn:& mut PgConnection)->String;
    fn singup(&self,conn:& mut PgConnection)->String;
}


#[async_trait]
pub trait MsgToKafka<T:InOn>{
    async fn send_msg_to_kafka(msg:Arc<Msg>)->typeDeliveryKafkaMsg;
}
