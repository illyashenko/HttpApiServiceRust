use appHttpMicroService::http::*;
use actix_web::{App, HttpServer, HttpResponse};
use actix_web::web::get;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(user_scope())
            .service(actix_web::web::resource("/test/about").route(get().to(||HttpResponse::Ok())))
        }).bind("localhost:5001")?.run().await
}