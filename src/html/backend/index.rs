use askama::Template;

#[derive(Template)]
#[template(path = "backend/index/index.html")]
pub struct IndexTemplate {}
