use askama::Template;

use crate::{
    db::pagination::Pagination,
    model::{SubjectTopicWithTagsAndTopicSummary, TopicDetail},
};

#[derive(Template)]
#[template(path = "frontend/topic/index.html")]
pub struct IndexTemplate {
    pub list: Pagination<Vec<SubjectTopicWithTagsAndTopicSummary>>,
    pub page: u32,
}
#[derive(Template)]
#[template(path = "frontend/topic/detail.html")]
pub struct DetailTemplate {
    pub topic: TopicDetail,
    pub uuids: Vec<String>,
    pub hc: bool,
}
