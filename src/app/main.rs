use actix_web::{get, App, HttpResponse, HttpServer, Responder};

pub mod view;
pub mod config;
pub mod utils;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(view::index::render())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(actix_files::Files::new("/assets", "./src/assets"))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
