pub mod index;
pub mod subject;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct PaginationArgs {
    pub page: u32,
}
