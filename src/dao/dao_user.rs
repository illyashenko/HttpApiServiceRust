use crate::models::{User};
use crate::data::{ContextBuilder, Context};
use crate::sql_mapper::{user_mapper};
use tokio_postgres::{Row, Error};

pub struct DaoUser{
    context: Context
}

impl DaoUser{
    pub async fn new()->DaoUser{
        DaoUser{context: ContextBuilder::new().await.unwrap()}
    }
}
impl DaoUser{
    pub async fn get_user(&mut self, email: String)->Result<User, String>{
        let selection  = self.context.client
                                 .query("SELECT * FROM guids_users WHERE email = $1",
                                     &[&email.as_str()]).await.unwrap();
        if selection.len() > 1 || selection.len() == 0 {
            Result::Err(String::from("404"))
        }
        else {
        Result::Ok(user_mapper(&selection[0]))
        }
    }
}


