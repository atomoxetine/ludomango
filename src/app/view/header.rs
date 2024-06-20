use askama::Template;

#[derive(Template)]
#[template(path = "header.html")]
struct HeaderTmpl;

pub fn render() -> String {
    HeaderTmpl {}.render().unwrap()
}
