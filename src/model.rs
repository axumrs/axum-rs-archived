use chrono::{Local, TimeZone};
use redis::Client;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

use crate::config::{HCaptchaConfig, ReCaptchaConfig, SessionConfig};

pub struct AppState {
    pub pool: deadpool_postgres::Pool,
    pub rdc: Client,
    pub sess_cfg: SessionConfig,
    pub hcap_cfg: HCaptchaConfig,
    pub recap_cfg: ReCaptchaConfig,
}

#[derive(PostgresMapper, Deserialize, Serialize)]
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

#[derive(PostgresMapper, Deserialize, Serialize)]
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

#[derive(PostgresMapper)]
#[pg_mapper(table = "v_topic_subject_list")]
pub struct TopicSubjectListView {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub subject_name: String,
    pub subject_slug: String,
    pub subject_id: i32,
    pub is_del: bool,
    pub subject_is_del: bool,
}
#[derive(PostgresMapper)]
#[pg_mapper(table = "v_topic_with_md_and_tags_for_edit")]
pub struct TopicWithMdAndTagsForEdit {
    pub id: i64,
    pub title: String,
    pub subject_id: i32,
    pub slug: String,
    pub summary: String,
    pub src: String,
    pub author: String,
    pub md: String,
    pub tag_names: Vec<String>,
}
impl TopicWithMdAndTagsForEdit {
    pub fn tags(&self) -> String {
        self.tag_names.join(",").to_string()
    }
}
#[derive(Deserialize, Serialize, Debug)]
pub struct AdminSession {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub is_sys: bool,
    pub dateline: i32,
}

#[derive(PostgresMapper)]
#[pg_mapper(table = "v_subject_topic")]
pub struct SubjectTopicWithTagsAndTopicSummary {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub subject_slug: String,
    pub tag_names: Vec<String>,
    pub summary: String,
    pub subject_name: String,
}

#[derive(PostgresMapper)]
#[pg_mapper(table = "v_topic_detail")]
pub struct TopicDetail {
    pub id: i64,
    pub subject_id: i32,
    pub title: String,
    pub slug: String,
    pub author: String,
    pub src: String,
    pub html: String,
    pub tag_names: Vec<String>,
    pub subject_slug: String,
    pub dateline: i32,
    pub hit: i32,
    pub subject_name: String,
}
impl TopicDetail {
    pub fn dateline(&self) -> String {
        let dt = Local.timestamp(self.dateline as i64, 0);
        dt.format("%Y/%m/%d %H:%M:%S").to_string()
    }
}

#[derive(PostgresMapper)]
#[pg_mapper(table = "admin")]
pub struct Admin {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub is_sys: bool,
    pub is_del: bool,
}
#[derive(PostgresMapper)]
#[pg_mapper(table = "admin")]
pub struct AdminID {
    pub id: i32,
}
