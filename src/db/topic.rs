use crate::{
    error::AppError,
    form::CreateTopic,
    model::{TagID, TopicID, TopicSubjectListView},
    time::now,
    Result,
};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::types::ToSql;

use super::{pagination::Pagination, select_stmt::SelectStmt, PAGE_SIZE};

pub async fn create(client: &mut Client, ct: &CreateTopic, html: &str) -> Result<TopicID> {
    let tx = client.transaction().await.map_err(AppError::from)?;
    // 是否存在
    let stmt = match tx
        .prepare("SELECT COUNT(*) FROM topic WHERE subject_id=$1 AND slug=$2")
        .await
    {
        Ok(s) => s,
        Err(err) => {
            tx.rollback().await.map_err(AppError::from)?;
            return Err(AppError::from(err));
        }
    };
    let count_row = match tx.query_one(&stmt, &[&ct.subject_id, &ct.slug]).await {
        Ok(row) => row,
        Err(err) => {
            tx.rollback().await.map_err(AppError::from)?;
            return Err(AppError::from(err));
        }
    };
    let count: i64 = count_row.get(0);
    if count > 0 {
        tx.rollback().await.map_err(AppError::from)?;
        return Err(AppError::is_exists("相同专题、相同固定链接的文章已存在"));
    }

    let stmt = tx.prepare("INSERT INTO topic (title, subject_id, slug, summary, author,  dateline) VALUES ($1, $2, $3, $4, $5, $6 ) RETURNING id").await;
    let stmt = match stmt {
        Ok(s) => s,
        Err(err) => {
            tx.rollback().await.map_err(AppError::from)?;
            return Err(AppError::from(err));
        }
    };
    let now = now();
    let result = match tx
        .query(
            &stmt,
            &[
                &ct.title,
                &ct.subject_id,
                &ct.slug,
                &ct.summary,
                &ct.author,
                &now,
            ],
        )
        .await
    {
        Ok(s) => s,
        Err(err) => {
            tx.rollback().await.map_err(AppError::from)?;
            return Err(AppError::from(err));
        }
    };
    let topic_id = result
        .iter()
        .map(|row| TopicID::from_row_ref(row).unwrap())
        .collect::<Vec<TopicID>>()
        .pop()
        .ok_or(AppError::not_found("插入失败"))?;
    // 内容
    let stmt = match tx
        .prepare("INSERT INTO topic_content (topic_id, md, html) VALUES ($1, $2, $3) ON CONFLICT(topic_id) DO UPDATE SET md=EXCLUDED.md,html=EXCLUDED.html")
        .await
    {
        Ok(s) => s,
        Err(err) => {
            tx.rollback().await.map_err(AppError::from)?;
            return Err(AppError::from(err));
        }
    };
    match tx.execute(&stmt, &[&topic_id.id, &ct.md, &html]).await {
        Err(err) => {
            tx.rollback().await.map_err(AppError::from)?;
            return Err(AppError::from(err));
        }
        _ => {}
    };

    // tag
    if !ct.tags.is_empty() {
        let tags = ct.tags.split(',').collect::<Vec<&str>>();
        let mut tags_id_list: Vec<TagID> = Vec::with_capacity(tags.len());
        for &tag_name in tags.iter() {
            let stmt = match tx
            .prepare("INSERT INTO tag(name) VALUES($1) ON CONFLICT(name) DO UPDATE SET name=EXCLUDED.name RETURNING id")
            .await
        {
            Ok(s) => s,
            Err(err) => {
                tx.rollback().await.map_err(AppError::from)?;
                return Err(AppError::from(err));
            }
        };
            let result = match tx.query(&stmt, &[&tag_name]).await {
                Ok(s) => s,
                Err(err) => {
                    tx.rollback().await.map_err(AppError::from)?;
                    return Err(AppError::from(err));
                }
            };
            let tags_id = result
                .iter()
                .map(|row| TagID::from_row_ref(row).unwrap())
                .collect::<Vec<TagID>>()
                .pop()
                .ok_or(AppError::not_found("插入标签失败"))?;
            tags_id_list.push(tags_id);
        }

        // topic_tag
        for tag_id in tags_id_list.iter() {
            let stmt = match tx.prepare("INSERT INTO topic_tag (topic_id, tag_id) VALUES($1,$2) ON CONFLICT(topic_id,tag_id) DO NOTHING").await {
                Ok(s) => s,
                Err(err) => {
                    tx.rollback().await.map_err(AppError::from)?;
                    return Err(AppError::from(err));
                }
            };
            match tx.execute(&stmt, &[&topic_id.id, &tag_id.id]).await {
                Err(err) => {
                    tx.rollback().await.map_err(AppError::from)?;
                    return Err(AppError::from(err));
                }
                _ => {}
            };
        }
    }
    tx.commit().await.map_err(AppError::from)?;
    Ok(topic_id)
}

pub async fn select(
    client: &Client,
    condition: Option<&str>,
    args: &[&(dyn ToSql + Sync)],
    page: u32,
) -> Result<Pagination<Vec<TopicSubjectListView>>> {
    let sql = SelectStmt::builder()
        .table("v_topic_subject_list")
        .fields("id,title,slug,subject_name,subject_slug,subject_id,is_del,subject_is_del")
        .condition(condition)
        .order(Some("id DESC"))
        .limit(Some(PAGE_SIZE))
        .offset(Some(page * PAGE_SIZE as u32))
        .build();
    let count_sql = SelectStmt::builder()
        .table("v_topic_subject_list")
        .fields("COUNT(*)")
        .condition(condition)
        .build();
    super::select(client, &sql, &count_sql, args, page).await
}
