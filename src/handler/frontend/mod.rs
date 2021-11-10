pub mod about;
pub mod index;
pub mod subject;
pub mod tag;
pub mod topic;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct PaginationArgs {
    pub page: u32,
}
