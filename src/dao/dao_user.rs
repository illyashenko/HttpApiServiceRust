use crate::models::{User};
use crate::data::{ContextBuilder, Context};
use crate::sql_mapper::{user_mapper};
use postgres::{Row, Error};

pub struct DaoUser{
    context: Context
}

impl DaoUser{
    pub fn new()->DaoUser{
        DaoUser{context: ContextBuilder::new()}
    }
}
impl DaoUser{

    pub fn get_user(&mut self, email: String)->Result<User, Error>{

        let selection  = self.context.client
                                 .query("SELECT * FROM guids_users WHERE email = $1",
                                     &[&email.as_str()]).unwrap();

        if selection.len() > 1 || selection.len() == 0 {
          // Err(());
        }

        Result::Ok(user_mapper(&selection[0]))
    }
}


