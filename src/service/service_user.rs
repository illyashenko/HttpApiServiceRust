use crate::dao::*;
use crate::models::User;
use tokio_postgres::Error;

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
    pub async fn get_user(&mut self, email: String) -> Result<User, String> {
        self.dao.get_user(email).await
    }
}