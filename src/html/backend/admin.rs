use askama::Template;

use crate::{arg, db::pagination::Pagination, model::Admin};

#[derive(Template)]
#[template(path = "backend/admin/add.html")]
pub struct AddTemplate {}
#[derive(Template)]
#[template(path = "backend/admin/edit.html")]
pub struct EditTemplate {
    pub admin: Admin,
}

#[derive(Template)]
#[template(path = "backend/admin/index.html")]
pub struct IndexTemplate {
    pub list: Pagination<Vec<Admin>>,
    pub arg: arg::BackendQueryArg,
}
