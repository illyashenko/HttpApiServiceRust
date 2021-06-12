use appHttpMicroService::http::*;
use appHttpMicroService::models::*;
use appHttpMicroService::service::*;
use actix_web::{App, HttpServer};
use actix_web::dev::Server;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new().service(index)
        }).bind("localhost:5001")?.run().await
}

