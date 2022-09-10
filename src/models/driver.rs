

use diesel::{Insertable,Queryable};
use serde::{Serialize,Deserialize};
use diesel::prelude::*;

//Внутреннее
use crate::{schema::list_of_drivers};
use crate::models::traits::InOn;

/*pub enum TaxiType{
    Economy,
    Comfort,
    Business,
}*/

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
    fn singup(&self,conn:& mut PgConnection) {
        use crate::schema::list_of_drivers::dsl::*;    
        diesel::insert_into(list_of_drivers).values(self).execute(conn).unwrap();
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

impl Driver {
    pub fn all(conn:&mut PgConnection)->Vec<Driver>{
        use crate::schema::list_of_drivers::dsl::*;
        list_of_drivers.load::<Driver>(conn).unwrap()
    }
    
}