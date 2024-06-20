use askama::Template;

use super::MinifyTemplate;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTmpl {
    header: String,
}

pub fn render() -> String {
    let header = super::header::render();

    IndexTmpl { header }.render_minify().unwrap()
}
