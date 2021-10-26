use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CreateSubject {
    pub name: String,
    pub slug: String,
    pub summary: String,
}
#[derive(Deserialize)]
pub struct UpdateSubject {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub summary: String,
}
#[derive(Deserialize)]
pub struct CreateTag {
    pub name: String,
}
#[derive(Deserialize)]
pub struct UpdateTag {
    pub id: i32,
    pub name: String,
}
