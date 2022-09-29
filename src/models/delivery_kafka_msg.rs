use rdkafka::error::KafkaError;
use crate::models::order::Msg;

#[derive(Debug)]
pub enum DeliveryKafkaMsg{
    Ok,
    KafkaError(KafkaError),

}

pub type typeDeliveryKafkaMsg=DeliveryKafkaMsg;