use std::sync::Mutex;

use askama::Template;

use crate::game::GameState;

use super::MinifyTemplate;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTmpl {
    header: String,
    game: String,
}

pub fn render(game: &Mutex<GameState>) -> String {
    let header = super::header::render();

    let mut game_lock = game.lock().unwrap();
    (*game_lock).move_piece(0, 1);

    let game = format!("{game_lock}");

    IndexTmpl { header, game }.render_minify().unwrap()
}
