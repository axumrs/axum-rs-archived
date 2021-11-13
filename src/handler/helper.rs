use std::sync::Arc;

use crate::model::AppState;
use crate::Result;
use crate::{error::AppError, rdb};
use askama::Template;
use axum::http::HeaderMap;
use axum::response::Html;
use deadpool_postgres::Client;
use rand::Rng;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;
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

#[derive(Deserialize, Serialize)]
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
pub async fn protected_content(
    html: &str,
    client: redis::Client,
    site_key: &str,
) -> (String, Vec<String>) {
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
    let hcs_num = hcs.len();
    let protect_num: usize = match hcs_num {
        0..=1 => 0,
        2..=4 => 1,
        5..=8 => 2,
        _ => 3,
    };
    tracing::debug!("protect_num: {:?}, hcs_num: {:?}", protect_num, hcs_num);
    if protect_num < 1 {
        return (html.to_string(), vec![]);
    }
    let mut protect_idx: Vec<usize> = Vec::with_capacity(protect_num);
    for _ in 0..protect_num {
        loop {
            let tmp: usize = rand::thread_rng().gen_range(0..100);
            let tmp = tmp % hcs_num;
            if !in_idx(&protect_idx[..], tmp) {
                protect_idx.push(tmp);
                break;
            }
        }
    }
    // 替换
    let hide_tag = "---待验证---";
    let mut out = String::new();
    let html = re.replace_all(html, hide_tag);
    let mut line_idx = 0usize;
    let mut out_uuids = vec![];
    for line in html.lines() {
        if line == hide_tag {
            if let Some(c) = hcs.pop() {
                let line = if !in_idx(&protect_idx[..], line_idx) {
                    format!("<{}>{}</{}>", c.tag, c.content, c.tag)
                } else {
                    out_uuids.push(c.uuid.clone());
                    let key = format!("protected_content:{}", c.uuid);
                    rdb::set(client.clone(), &key, json!(c).to_string().as_str(), 60 * 20)
                        .await
                        .unwrap();
                    format!(
                        "<div id=\"hcaptcha-{uuid}\" class=\"callout callout-info\"><div>你需要进行人机验证才能查看隐藏的内容(大约{count}字节)</div><div class=\"h-captcha\" data-sitekey=\"{site_key}\"  data-callback=\"get_procted_content_{uuid}\"></div></div>",
                        site_key=site_key, uuid=c.uuid,count=word_count(&c.content)
                    )
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
    (out, out_uuids)
}

fn word_count(s: &str) -> usize {
    s.len()
}

pub fn get_cookie(headers: &HeaderMap, name: &str) -> Option<String> {
    let cookie = headers
        .get(axum::http::header::COOKIE)
        .and_then(|value| value.to_str().ok())
        .map(|value| value.to_string());
    match cookie {
        Some(cookie) => {
            let cookie = cookie.as_str();
            let cs: Vec<&str> = cookie.split(';').collect();
            for item in cs {
                let item: Vec<&str> = item.split('=').collect();
                if item.len() != 2 {
                    continue;
                }
                let key = item[0];
                let val = item[1];
                let key = key.trim();
                let val = val.trim();
                if key == name {
                    return Some(val.to_string());
                }
            }
            None
        }
        None => None,
    }
}
