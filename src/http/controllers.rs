use crate::models::*;
use crate::test::*;
use crate::service::*;
use serde::{Serialize, Deserialize};
use actix_web::{get, post, web, Responder, HttpResponse};
use actix_web::web::Form;

#[get("/user/{email}")]
pub async fn get_user_by_email(path : web::Path<(String)>) -> impl Responder {
    let user = ServiceUser::new().await.get_user(path.into_inner()).await;
    match user {
        Ok(answer) => {
            let json_answer = serde_json::to_string(&answer).unwrap();
            HttpResponse::Ok().content_type("application/json").body(json_answer)
        }
        Err(err) => {
            HttpResponse::NotFound().body(err)
        }
    }
}

#[post("/user/create/")]
pub async fn create_new_user(req: String) -> impl Responder {
    let user : ReqUser = serde_json::from_str(req.as_str()).unwrap();
    let answer = ServiceUser::new().await.create_user(user).await;
    match answer {
        Ok(()) => {
            HttpResponse::Ok().content_type("application/json").body("user create")
        }
        Err(()) => {
            HttpResponse::BadRequest().content_type("application/json").body("error")
        }
    }
}

