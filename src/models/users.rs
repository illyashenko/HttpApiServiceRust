use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::additional_service::{id_default};

#[derive(Deserialize, Serialize)]
pub struct User {
    pub uuid: Uuid,
    pub  id: i32,
    pub login: String,
    pub pass: String,
    pub email: String,
    pub phone: String,
    pub name: String,
    pub code: String,
    pub description: String,
    pub notvalid: bool,
}
impl User {
    pub fn new_user(login_: String, pass_: String, email_: String, phone_:String, name_: String)-> User{
        User {
            uuid: Uuid::new_v4(),
            id: id_default(),
            login: login_,
            pass: pass_,
            email: email_,
            phone: phone_,
            name: name_,
            code: "".to_string(),
            description: "".to_string(),
            notvalid: false,
        }
    }

    pub fn error()->i32{
        404
    }
}