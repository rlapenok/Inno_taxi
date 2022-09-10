use diesel::{Insertable, Queryable,PgConnection};
use serde::{Serialize,Deserialize};
use diesel::prelude::*;

//Внутреннее
use crate::models::traits::InOn;
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
    fn singup(&self,conn:& mut PgConnection) {
        use crate::schema::list_of_users::dsl::*;    
        diesel::insert_into(list_of_users).values(self).execute(conn).unwrap();
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

impl User {
    pub fn all(conn:&mut PgConnection)->Vec<User>{
        use crate::schema::list_of_users::dsl::*;
        list_of_users.load::<User>(conn).unwrap()
    }
    
}