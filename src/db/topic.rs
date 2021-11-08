use crate::{
    error::AppError,
    form::{CreateTopic, UpdateTopic},
    model::{
        SubjectTopicWithTagsAndTopicSummary, TagID, TopicID, TopicSubjectListView,
        TopicWithMdAndTagsForEdit,
    },
    time::now,
    Result,
};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::types::ToSql;

use super::{pagination::Pagination, query_one, select_stmt::SelectStmt, PAGE_SIZE};

/// 创建新的文章
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

    let stmt = tx.prepare("INSERT INTO topic (title, subject_id, slug, summary, author,  dateline, src) VALUES ($1, $2, $3, $4, $5, $6 ) RETURNING id").await;
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
                &ct.src,
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

/// 分页显示文章
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
pub async fn select_with_summary(
    client: &Client,
    condition: Option<&str>,
    args: &[&(dyn ToSql + Sync)],
    page: u32,
) -> Result<Pagination<Vec<SubjectTopicWithTagsAndTopicSummary>>> {
    let sql = SelectStmt::builder()
        .table("v_subject_topics")
        .fields("id,title,slug,subject_slug,tag_names,summary")
        .condition(condition)
        .order(Some("id ASC"))
        .limit(Some(PAGE_SIZE))
        .offset(Some(page * PAGE_SIZE as u32))
        .build();
    let count_sql = SelectStmt::builder()
        .table("v_subject_topics")
        .fields("COUNT(*)")
        .condition(condition)
        .build();
    super::select(client, &sql, &count_sql, args, page).await
}

/// 删除或还原文章
pub async fn del_or_restore(client: &mut Client, id: i64, is_del: bool) -> Result<(u64, u64)> {
    let tx = client.transaction().await.map_err(AppError::from)?;
    let stmt = match tx.prepare("UPDATE topic SET is_del=$1 WHERE id=$2").await {
        Ok(s) => s,
        Err(err) => {
            tx.rollback().await.map_err(AppError::from)?;
            return Err(AppError::from(err));
        }
    };
    let topic_rows = match tx.execute(&stmt, &[&is_del, &id]).await {
        Ok(s) => s,
        Err(err) => {
            tx.rollback().await.map_err(AppError::from)?;
            return Err(AppError::from(err));
        }
    };
    let stmt = match tx
        .prepare("UPDATE topic_tag SET is_del=$1 WHERE topic_id=$2")
        .await
    {
        Ok(s) => s,
        Err(err) => {
            tx.rollback().await.map_err(AppError::from)?;
            return Err(AppError::from(err));
        }
    };
    let topic_tag_rows = match tx.execute(&stmt, &[&is_del, &id]).await {
        Ok(s) => s,
        Err(err) => {
            tx.rollback().await.map_err(AppError::from)?;
            return Err(AppError::from(err));
        }
    };

    tx.commit().await.map_err(AppError::from)?;
    Ok((topic_rows, topic_tag_rows))
}

/// 获取用于修改的文章
pub async fn find_to_edit(client: &Client, id: i64) -> Result<TopicWithMdAndTagsForEdit> {
    let sql = SelectStmt::builder()
        .table("v_topic_with_md_and_tags_for_edit")
        .fields("id,title,subject_id,slug,summary,author,md,tag_names,src")
        .condition(Some("id=$1"))
        .limit(Some(1))
        .build();
    query_one(client, &sql, &[&id], Some("没有找到符合条件的文章")).await
}
/// 修改文章
pub async fn update(client: &mut Client, ut: &UpdateTopic, html: &str) -> Result<bool> {
    let tx = client.transaction().await.map_err(AppError::from)?;
    // 是否存在
    let stmt = match tx
        .prepare("SELECT COUNT(*) FROM topic WHERE subject_id=$1 AND slug=$2 AND id<>$3")
        .await
    {
        Ok(s) => s,
        Err(err) => {
            tx.rollback().await.map_err(AppError::from)?;
            return Err(AppError::from(err));
        }
    };
    let count_row = match tx
        .query_one(&stmt, &[&ut.subject_id, &ut.slug, &ut.id])
        .await
    {
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

    let stmt = tx.prepare("UPDATE topic SET title=$1, subject_id=$2, slug=$3, summary=$4, author=$5, src=$6 WHERE id=$7").await;
    let stmt = match stmt {
        Ok(s) => s,
        Err(err) => {
            tx.rollback().await.map_err(AppError::from)?;
            return Err(AppError::from(err));
        }
    };
    match tx
        .execute(
            &stmt,
            &[
                &ut.title,
                &ut.subject_id,
                &ut.slug,
                &ut.summary,
                &ut.author,
                &ut.src,
                &ut.id,
            ],
        )
        .await
    {
        Err(err) => {
            tx.rollback().await.map_err(AppError::from)?;
            return Err(AppError::from(err));
        }
        _ => {}
    };
    // 内容
    let stmt = match tx
        .prepare("UPDATE topic_content SET  md=$1, html=$2 WHERE topic_id=$3")
        .await
    {
        Ok(s) => s,
        Err(err) => {
            tx.rollback().await.map_err(AppError::from)?;
            return Err(AppError::from(err));
        }
    };
    match tx.execute(&stmt, &[&ut.md, &html, &ut.id]).await {
        Err(err) => {
            tx.rollback().await.map_err(AppError::from)?;
            return Err(AppError::from(err));
        }
        _ => {}
    };

    // tag
    // 删除所有关联的tag
    let stmt = match tx.prepare("DELETE FROM topic_tag WHERE topic_id=$1").await {
        Ok(s) => s,
        Err(err) => {
            tx.rollback().await.map_err(AppError::from)?;
            return Err(AppError::from(err));
        }
    };
    match tx.execute(&stmt, &[&ut.id]).await {
        Err(err) => {
            tx.rollback().await.map_err(AppError::from)?;
            return Err(AppError::from(err));
        }
        _ => {}
    };

    // 添加并关联标签
    if !ut.tags.is_empty() {
        let tags = ut.tags.split(',').collect::<Vec<&str>>();
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
            match tx.execute(&stmt, &[&ut.id, &tag_id.id]).await {
                Err(err) => {
                    tx.rollback().await.map_err(AppError::from)?;
                    return Err(AppError::from(err));
                }
                _ => {}
            };
        }
    }
    tx.commit().await.map_err(AppError::from)?;
    Ok(true)
}
