use crate::dao::*;
use crate::models::{User, UserBuilder, ReqUser};
use crate::additional_service::*;
use tokio_postgres::Error;
use actix_web::web::{Form};
use waiter_di::*;

#[module]
pub struct ServiceUser{
    dao: Box<dyn IDaoUser> // DI
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
    pub async fn new() -> Self {
        let mut container =  get::<profiles::Default>();
        Provider::<ServiceUser>::create(&mut container)
    }
}