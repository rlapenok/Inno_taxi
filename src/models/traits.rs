use diesel::prelude::*;

pub trait InOn{
    fn singin(&self,conn:& mut PgConnection)->String;
    fn singup(&self,conn:& mut PgConnection);
}
