use askama::Template;

use crate::{
    db::pagination::Pagination,
    model::{Subject, SubjectTopicWithTagsAndTopicSummary},
};

#[derive(Template)]
#[template(path = "frontend/subject/index.html")]
pub struct IndexTemplate {
    pub page: u32,
    pub list: Pagination<Vec<Subject>>,
}
#[derive(Template)]
#[template(path = "frontend/subject/topics.html")]
pub struct TopicsTemplate {
    pub page: u32,
    pub list: Pagination<Vec<SubjectTopicWithTagsAndTopicSummary>>,
    pub subject: Subject,
    pub slug: String,
}
