use uuid::Uuid;
use serde::{Serialize, Deserialize};
use actix_web::web::{Form};

use crate::additional_service::{id_default};

#[derive(Deserialize, Serialize)]
pub struct User {
    pub uuid: Uuid,
    pub id: i32,
    pub login: String,
    pub pass: String,
    pub email: String,
    pub phone: String,
    pub name: String,
    pub code: String,
    pub description: String,
    pub notvalid: bool,
}

#[derive(Deserialize)]
pub struct ReqUser {
    pub login: String,
    pub pass: String,
    pub email: String,
    pub phone: String,
    pub name: String
}

pub struct UserBuilder{}

impl UserBuilder{
    pub fn new(req_user: ReqUser) -> User{
        User {
            uuid: Uuid::new_v4(),
            id: id_default(),
            login: req_user.login,
            pass: req_user.pass,
            email: req_user.email,
            phone: req_user.phone,
            name: req_user.name,
            code: "".to_string(),
            description: "".to_string(),
            notvalid: false
        }
    }
    pub fn new_from_form(form: Form<ReqUser>)->User{
        User {
            uuid: Uuid::new_v4(),
            id: id_default(),
            login: form.login.to_string(),
            pass: form.pass.to_string(),
            email: form.email.to_string(),
            phone: form.phone.to_string(),
            name: form.name.to_string(),
            code: "".to_string(),
            description: "".to_string(),
            notvalid: false
        }
    }
}