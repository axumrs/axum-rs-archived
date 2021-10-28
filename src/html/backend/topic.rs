use askama::Template;
use crate::model;

#[derive(Template)]
#[template(path = "backend/topic/add.html")]
pub struct AddTemplate {
    pub subjects: Vec<model::Subject>,
}
