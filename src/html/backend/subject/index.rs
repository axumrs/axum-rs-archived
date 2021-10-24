use askama::Template;

#[derive(Template)]
#[template(path = "backend/subject/index.html")]
pub struct IndexTemplate {
    pub name: String,
}
