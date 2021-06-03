use appHttpMicroService::http::*;
use appHttpMicroService::models::*;
use appHttpMicroService::service::*;
use serde_json::*;
//use actix_web::{App, HttpServer};
//use actix_web::dev::Server;
use tiny_http::*;
use actix_web::dev::ResourcePath;


//#[actix_web::main]
fn main() {
    let server = Server::http("localhost:5001").unwrap();

    for request in server.incoming_requests() {
        println!("{:?}", request.url());
        let user = ServiceUser::new().get_user(String::from("potapov@inbox.ru")).unwrap();
        let j = serde_json::to_string(&user).unwrap();
        let resp = Response::from_string(j);
        request.respond(resp);
    }

   /* let listener = HttpServer::new(|| App::new()
        .service(index_get)
        .service(index_get2))
        .bind("localhost:5001").unwrap();

    listener.run();*/
}

