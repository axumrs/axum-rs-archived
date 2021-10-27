use crate::{arg, db::pagination::Pagination, model::Tag};
use askama::Template;

#[derive(Template)]
#[template(path = "backend/tag/index.html")]
pub struct IndexTemplate {
    pub arg: arg::TagBackendQueryArg,
    pub list: Pagination<Vec<Tag>>,
}
