use askama::Template;

#[derive(Template)]
#[template(path = "frontend/index/index.html")]
pub struct IndexTemplate {}
