use tokio_pg_mapper_derive::PostgresMapper;

pub struct AppState {
    pub pool: deadpool_postgres::Pool,
}

#[derive(PostgresMapper)]
#[pg_mapper(table = "subject")]
pub struct Subject {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub summary: String,
    pub is_del: bool,
}
#[derive(PostgresMapper)]
#[pg_mapper(table = "subject")]
pub struct SubjectList {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub is_del: bool,
}

#[derive(PostgresMapper)]
#[pg_mapper(table = "subject")]
pub struct SubjectID {
    pub id: i32,
}

#[derive(PostgresMapper)]
#[pg_mapper(table = "topic")]
pub struct Topic {
    pub id: i64,
    pub title: String,
    pub subject_id: i32,
    pub slug: String,
    pub summary: String,
    pub src: String,
    pub author: String,
    pub hit: i32,
    pub dateline: i32,
    pub is_del: bool,
}
#[derive(PostgresMapper)]
#[pg_mapper(table = "topic")]
pub struct TopicID {
    pub id: i64,
}

#[derive(PostgresMapper)]
#[pg_mapper(table = "topic_content")]
pub struct TopicContent {
    pub topic_id: i64,
    pub md: String,
    pub html: String,
}

#[derive(PostgresMapper)]
#[pg_mapper(table = "tag")]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub is_del: bool,
}
#[derive(PostgresMapper)]
#[pg_mapper(table = "tag")]
pub struct TagID {
    pub id: i32,
}

#[derive(PostgresMapper)]
#[pg_mapper(table = "topic_tag")]
pub struct TopicTag {
    pub topic_id: i64,
    pub tag_id: i32,
    pub is_del: bool,
}
