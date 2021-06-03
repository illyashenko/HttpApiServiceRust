use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{id}")]
async fn index(path : web::Path<(u32)>) -> impl Responder {
    let (i+2d) = path.into_inner();
    format!("Hello {}! ", id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("localhost:5001")?
        .run()
        .await
}
