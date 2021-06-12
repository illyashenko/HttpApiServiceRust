use crate::models::*;
use crate::test::*;
use crate::service::*;
use actix_web::{get, web, Responder, HttpResponse};

#[get("/{email}")]
pub async fn index(path : web::Path<(String)>) -> impl Responder {
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

