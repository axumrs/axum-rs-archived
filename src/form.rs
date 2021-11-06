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

#[derive(Deserialize)]
pub struct CreateTopic {
    pub subject_id: i32,
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub src: String,
    pub author: String,
    pub md: String,
    pub tags: String,
}

#[derive(Deserialize)]
pub struct UpdateTopic {
    pub id: i64,
    pub title: String,
    pub subject_id: i32,
    pub slug: String,
    pub summary: String,
    pub src: String,
    pub author: String,
    pub md: String,
    pub tags: String,
}
#[derive(Deserialize)]
pub struct AdminLogin {
    pub username: String,
    pub password: String,
}
