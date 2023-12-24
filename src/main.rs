use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
mod routes;
mod structs;

use routes::premium::tea_plus;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(tea_plus)
            .default_service(web::route().to(|| async { HttpResponse::NotFound().finish() }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/{tail:.*}")]
async fn hello() -> impl Responder {
    HttpResponse::Found()
        .append_header(("Location", "https://teaclient.net/docs/api/intro/"))
        .finish()
}

