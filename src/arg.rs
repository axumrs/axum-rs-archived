use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SubjectBackendQueryArg {
    pub page: Option<u32>,
    pub keyword: Option<String>,
    pub msg: Option<String>,
}
