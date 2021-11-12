use askama::Template;

#[derive(Template)]
#[template(path = "err.html")]
pub struct ErrTemplate {
    pub err: String,
}
