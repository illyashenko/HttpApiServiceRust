use crate::dao::*;
use crate::models::User;
use postgres::Error;

pub struct ServiceUser{
    dao: DaoUser
}
impl ServiceUser{
    pub fn new()->ServiceUser{
        ServiceUser{
            dao: DaoUser::new()
        }
    }
}
impl ServiceUser{
    pub fn get_user(&mut self, email: String) -> Result<User, Error> {
        self.dao.get_user(email)
    }
}