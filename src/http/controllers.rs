use crate::models::*;
use actix_web::{get, web, Responder};

#[get("/{id}")]
async fn index(path : web::Path<(u32)>) -> impl Responder {
    let (id) = path.into_inner();
    format!("Hello {}! ", id)
}

