use serde::{Deserialize, Serialize};

use actix_web::{post, get, web, HttpResponse, Responder, Result};

#[get("/")]
async fn index() -> Result<HttpResponse> {
   Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("static/index.html")))
}

#[derive(Serialize, Deserialize)]
pub struct Params {
    name: String,
}

#[post("/")]
async fn hello_name(params: web::Form<Params>) -> impl Responder {
    HttpResponse::Ok().body(format!("hello {} ", params.name))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};
    HttpServer::new(|| {
        App::new()
        .service(index)
        .service(hello_name)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
