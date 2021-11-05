use crate::{arg, db::pagination::Pagination, model};
use askama::Template;

#[derive(Template)]
#[template(path = "backend/topic/add.html")]
pub struct AddTemplate {
    pub subjects: Vec<model::SubjectList>,
}
#[derive(Template)]
#[template(path = "backend/topic/index.html")]
pub struct IndexTemplate {
    pub list: Pagination<Vec<model::TopicSubjectListView>>,
    pub arg: arg::BackendQueryArg,
}
#[derive(Template)]
#[template(path = "backend/topic/edit.html")]
pub struct EditTemplate {
    pub subjects: Vec<model::SubjectList>,
    pub topic: model::TopicWithMdAndTagsForEdit,
}
