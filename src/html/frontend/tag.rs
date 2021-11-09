use askama::Template;

use crate::{
    db::pagination::Pagination,
    model::{SubjectTopicWithTagsAndTopicSummary, Tag},
};

#[derive(Template)]
#[template(path = "frontend/tag/index.html")]
pub struct IndexTemplate {
    pub tags: Vec<Tag>,
}
#[derive(Template)]
#[template(path = "frontend/tag/topics.html")]
pub struct TopicsTemplate {
    pub page: u32,
    pub list: Pagination<Vec<SubjectTopicWithTagsAndTopicSummary>>,
    pub name: String,
    pub tag: Tag,
}
