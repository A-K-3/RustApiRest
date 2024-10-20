mod routes;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use crate::routes::{echo, hello, manual_hello};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(manual_hello)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}