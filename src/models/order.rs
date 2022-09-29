use serde::{Serialize,Deserialize};
use crate::models::driver::{TaxiType};

#[derive(Serialize,Deserialize,Debug,Clone)]
 pub struct Msg{
    pub phone:i32,
    pub  name:String,
    pub taxi_type:TaxiType,
}
