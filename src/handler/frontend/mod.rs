pub mod index;
pub mod subject;
pub mod tag;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct PaginationArgs {
    pub page: u32,
}
