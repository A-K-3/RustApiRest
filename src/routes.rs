use actix_web::{get, post, HttpResponse, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}


#[post("/hey")]
async fn manual_hello(req_body: String) -> impl Responder {
    HttpResponse::Ok().body("Hey thereeeee!")
}
