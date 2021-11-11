use std::sync::Arc;

use crate::error::AppError;
use crate::model::AppState;
use crate::Result;
use askama::Template;
use axum::response::Html;
use deadpool_postgres::Client;
use rand::Rng;
use regex::Regex;
use uuid::Uuid;

pub async fn get_client(state: Arc<AppState>, handler_name: &str) -> Result<Client> {
    state.pool.get().await.map_err(|err| {
        tracing::error!("无法获取数据库连接：{:?},  {}", err, handler_name);
        AppError::from(err)
    })
}

pub fn log_error(handler_name: String) -> Box<dyn Fn(AppError) -> AppError> {
    Box::new(move |err| {
        tracing::error!("操作失败：{:?},  {}", err, handler_name);
        err
    })
}

pub fn render<T: Template>(tmpl: T, handler_name: &str) -> Result<Html<String>> {
    let out = tmpl.render().map_err(|err| {
        tracing::error!("模板渲染出错：{:?}, {}", err, handler_name);
        AppError::from(err)
    })?;
    Ok(Html(out))
}

pub struct ProtectedContent {
    pub uuid: String,
    pub tag: String,
    pub content: String,
}
fn in_idx(idx: &[usize], i: usize) -> bool {
    for ii in idx.iter() {
        if *ii == i {
            return true;
        }
    }
    false
}
pub fn protected_content(html: &str, protect_num: usize) -> String {
    let mut hcs = vec![];
    let re = Regex::new(r"(?sm)<(p|pre)>(.+?)</(p|pre)>").unwrap();
    for cap in re.captures_iter(html) {
        hcs.push(ProtectedContent {
            uuid: Uuid::new_v4().to_simple().to_string(),
            tag: cap[1].to_string(),
            content: cap[2].to_string(),
        });
    }
    let mut hcs: Vec<&ProtectedContent> = hcs.iter().rev().into_iter().collect();
    let mut protect_idx: Vec<usize> = vec![0; protect_num];
    let protect_num = if protect_num > hcs.len() {
        hcs.len() - 1
    } else {
        protect_num
    };
    for i in 0..protect_num {
        loop {
            let tmp: usize = rand::thread_rng().gen_range(0..100);
            let tmp = tmp % hcs.len();
            if !in_idx(&protect_idx[..], tmp) {
                protect_idx[i] = tmp;
                break;
            }
        }
    }
    // 替换
    let hide_tag = "---待显示---";
    let mut out = String::new();
    let html = re.replace_all(html, hide_tag);
    let mut line_idx = 0usize;
    for line in html.lines() {
        if line == hide_tag {
            if let Some(c) = hcs.pop() {
                let line = if !in_idx(&protect_idx[..], line_idx) {
                    format!("<{}>{}</{}>", c.tag, c.content, c.tag)
                } else {
                    format!("<div>{}</div>", c.uuid)
                };
                out.push_str(&line);
                out.push('\n');
                line_idx += 1;
                continue;
            } else {
                line_idx += 1;
            }
        }
        out.push_str(line);
        out.push('\n');
    }
    out
}
