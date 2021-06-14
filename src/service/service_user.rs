use crate::dao::*;
use crate::models::{User, UserBuilder, ReqUser};
use tokio_postgres::Error;
use actix_web::web::{Form};

pub struct ServiceUser{
    dao: DaoUser
}
impl ServiceUser{
    pub async fn new()->ServiceUser{
        ServiceUser{
            dao: DaoUser::new().await
        }
    }
}
impl ServiceUser{
    pub async fn get_user(&mut self, email: String) -> Result<User, ()> {
        self.dao.get_user(email).await
    }
    pub async fn create_user(&mut self, req_user: ReqUser) -> Result<(), ()>{
        self.dao.create_user(UserBuilder::new(req_user)).await
    }
    pub async fn delete_user(&mut self, email: &String) -> Result<(), ()>{
        self.dao.delete_user(email).await
    }
}