use askama::Template;

use crate::{arg, db::pagination::Pagination, model};

#[derive(Template)]
#[template(path = "backend/subject/index.html")]
pub struct IndexTemplate {
    pub subject_list: Pagination<Vec<model::SubjectList>>,
    pub arg: arg::SubjectBackendQueryArg,
}
#[derive(Template)]
#[template(path = "backend/subject/add.html")]
pub struct AddTemplate {}

#[derive(Template)]
#[template(path = "backend/subject/edit.html")]
pub struct EditTemplate {
    pub subject: model::Subject,
}
