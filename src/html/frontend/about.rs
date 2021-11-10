use askama::Template;

#[derive(Template)]
#[template(path = "frontend/about/index.html")]
pub struct IndexTemplate {}
