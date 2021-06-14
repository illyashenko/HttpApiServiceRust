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
    pub async fn get_user(&mut self, email: String)->Result<User, ()>{
        let selection  = self.context.client
                                 .query("SELECT * FROM guids_users WHERE email = $1",
                                     &[&email.as_str()]).await.unwrap();
        if selection.len() > 1 || selection.len() == 0 {
            Result::Err(())
        }
        else {
        Result::Ok(user_mapper(&selection[0]))
        }
    }
    pub async fn create_user(&mut self, user: User) -> Result<(), ()> {

        let create = self.context.client
                                  .query("INSERT INTO guids_users (UUID, login, pass, email, phone, name, notvalid)\
                                                   VALUES($1, $2, $3, $4, $5, $6, $7) RETURNING true",
                                         &[&user.uuid, &user.login.as_str(),
                                                  &user.pass.as_str(),&user.email.as_str(),
                                                  &user.phone.as_str(), &user.name.as_str(), &user.notvalid]).await.unwrap();
        if create[0].get(0) {
            Result::Ok(())
        }
        else {
            Result::Err(())
        }
    }
    pub async fn delete_user(&mut self, email: &String)->Result<(),()>{
        let result = self.context.client
                                 .query("DELETE FROM guids_users WHERE email = $1 RETURNING true",
                                        &[email]).await.unwrap();
        if result[0].get(0) {
            Result::Ok(())
        }
        else {
            Result::Err(())
        }
    }
}


