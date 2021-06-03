mod http;
mod models;
mod libs;

use models::User;
use http::*;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("localhost:5001")?
        .run()
        .await
}

