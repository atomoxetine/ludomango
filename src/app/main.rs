use std::sync::Mutex;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use game::GameState;

pub mod view;
pub mod config;
pub mod utils;
pub mod game;

#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body(view::index::render(&data.game))
}

struct AppState {
    game: Mutex<GameState>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let app_state = web::Data::new(AppState {
        game: Mutex::new(GameState::new())
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(index)
            .service(actix_files::Files::new("/assets", "./src/assets"))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
