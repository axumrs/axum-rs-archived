use askama::Template;

#[derive(Template)]
#[template(path = "frontend/subject/index.html")]
pub struct IndexTemplate {}
