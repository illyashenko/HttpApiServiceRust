use appHttpMicroService::http::*;
use actix_web::{App, HttpServer, HttpResponse};
use actix_web::web::get;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .service(get_user_by_email)
            .service(create_new_user)
        }).bind("localhost:5001")?.run().await
}

//.service(actix_web::web::resource("/test/about").route(get().to(||HttpResponse::Ok())))