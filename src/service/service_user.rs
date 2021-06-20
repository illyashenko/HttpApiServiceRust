use crate::dao::*;
use crate::models::{User, UserBuilder, ReqUser};
use tokio_postgres::Error;
use actix_web::web::{Form};
use async_trait::async_trait;

#[async_trait]
pub trait IServiceUser{
    type Service;

    async fn get_user(&mut self, email: String) -> Result<User, ()>;
    async fn create_user(&mut self, req_user: ReqUser) -> Result<(), ()>;
    async fn delete_user(&mut self, email: &String) -> Result<(), ()>;

    async fn new(dao_: Box<DaoUser>)->Self::Service;
}

pub struct ServiceUser{
    dao: Box<DaoUser>
}

#[async_trait]
impl IServiceUser for ServiceUser{

    type Service = ServiceUser;

    async fn get_user(&mut self, email: String) -> Result<User, ()> {
        self.dao.get_user(email).await
    }
    async fn create_user(&mut self, req_user: ReqUser) -> Result<(), ()>{
        self.dao.create_user(UserBuilder::new(req_user)).await
    }
    async fn delete_user(&mut self, email: &String) -> Result<(), ()>{
        self.dao.delete_user(email).await
    }

    async fn new(dao_: Box<DaoUser>) -> Self::Service {
        ServiceUser{
            dao: dao_
        }
    }
}