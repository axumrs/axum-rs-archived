use crate::model;
use askama::Template;

#[derive(Template)]
#[template(path = "backend/topic/add.html")]
pub struct AddTemplate {
    pub subjects: Vec<model::SubjectList>,
}
