use crate::models::*;
use crate::test::*;
use crate::service::*;
use serde::{Serialize, Deserialize};
use actix_web::{get, post, put, delete, web, Responder, HttpResponse, Scope};

pub fn user_scope() -> Scope{
   web::scope("/user")
       .service(get_user_by_email)
       .service(create_new_user)
       .service(delete_user)
       .service(update_user)
}

#[get("/{email}")]
async fn get_user_by_email(path : web::Path<(String)>) -> impl Responder {
    let user = ServiceUser::new().await.get_user(path.into_inner()).await;
    match user {
        Ok(answer) => {
            let json_answer = serde_json::to_string(&answer).unwrap();
            HttpResponse::Ok().content_type("application/json").body(json_answer)
        }
        Err(()) => {
            HttpResponse::NotFound().body("")
        }
    }
}

#[post("/")]
async fn create_new_user(req: String) -> impl Responder {
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

#[delete("/{email}")]
async fn delete_user(path: web::Path<String>)-> impl Responder{
    let email = path.into_inner();
    let result = ServiceUser::new().await.delete_user(&email).await;
    match result {
        Ok(()) => {
            HttpResponse::Ok().content_type("application/json").body(format!("delete {:?}", email))
        }
        Err(()) => {
            HttpResponse::NotFound().body("error")
        }
    }
}

#[put("/{email}")]
async fn update_user(path: web::Path<String>)-> impl Responder{
    HttpResponse::BadRequest().content_type("application/json").body("error")
}


