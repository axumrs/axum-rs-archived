use askama::Template;

#[derive(Template)]
#[template(path = "backend/subject/index.html")]
pub struct IndexTemplate {
    pub msg: Option<String>,
}
#[derive(Template)]
#[template(path = "backend/subject/add.html")]
pub struct AddTemplate {}
