use crate::{
    error::AppError,
    form::{CreateTopic, UpdateTopic},
    model::{
        SubjectTopicWithTagsAndTopicSummary, TagID, TopicDetail, TopicID, TopicSubjectListView,
        TopicWithMdAndTagsForEdit,
    },
    time::now,
    Result,
};
use tokio_postgres::types::ToSql;
use tokio_postgres::Client;

use super::{pagination::Pagination, query_one, select_stmt::SelectStmt, PAGE_SIZE};

/// 创建新的文章
pub async fn create(client: &mut Client, ct: &CreateTopic, html: &str) -> Result<TopicID> {
    let tx = client.transaction().await.map_err(AppError::from)?;
    // 是否存在
    match super::count(
        &tx,
        "SELECT COUNT(*) FROM topic WHERE subject_id=$1 AND slug=$2",
        &[&ct.subject_id, &ct.slug],
    )
    .await
    {
        Ok(row) if row > 0 => {
            tx.rollback().await.map_err(AppError::from)?;
            return Err(AppError::is_exists("相同专题、相同固定链接的文章已存在"));
        }
        Err(err) => {
            tx.rollback().await.map_err(AppError::from)?;
            return Err(err);
        }
        _ => {}
    };

    let now = now();
    let topic_id: TopicID = match super::query_one(&tx, "INSERT INTO topic (title, subject_id, slug, summary, author,  dateline, src) VALUES ($1, $2, $3, $4, $5, $6,$7 ) RETURNING id",&[
        &ct.title,
        &ct.subject_id,
        &ct.slug,
        &ct.summary,
        &ct.author,
        &now,
        &ct.src,
    ] , Some("插入文章失败")).await
    {
        Ok(s) => s,
            Err(err) => {tx.rollback().await.map_err(AppError::from)?;
            return Err(err);
            }
    };

    // 内容
    match super::execute(&tx, "INSERT INTO topic_content (topic_id, md, html) VALUES ($1, $2, $3) ON CONFLICT(topic_id) DO UPDATE SET md=EXCLUDED.md,html=EXCLUDED.html",& [&topic_id.id, &ct.md, &html]).await {
        Ok(_) => {},
        Err(err) => {
            tx.rollback().await.map_err(AppError::from)?;
            return Err(err);
            }
    };

    // tag
    if !ct.tags.is_empty() {
        let tags = ct.tags.split(',').collect::<Vec<&str>>();
        let mut tags_id_list: Vec<TagID> = Vec::with_capacity(tags.len());
        for &tag_name in tags.iter() {
            let tags_id:TagID = match super::query_one(&tx, "INSERT INTO tag(name) VALUES($1) ON CONFLICT(name) DO UPDATE SET name=EXCLUDED.name RETURNING id", &[&tag_name], Some("插入标签失败")).await {
                Ok(s) => s,
                Err(err) => {
                    tx.rollback().await.map_err(AppError::from)?;
                    return Err(err);
                }
            };
            tags_id_list.push(tags_id);
        }

        // topic_tag
        for tag_id in tags_id_list.iter() {
            if let Err(err) = super::execute(&tx, "INSERT INTO topic_tag (topic_id, tag_id) VALUES($1,$2) ON CONFLICT(topic_id,tag_id) DO NOTHING", &[&topic_id.id, &tag_id.id]).await {
                tx.rollback().await.map_err(AppError::from)?;
                return Err(err);
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
    order: Option<&str>,
    page: u32,
) -> Result<Pagination<Vec<SubjectTopicWithTagsAndTopicSummary>>> {
    let sql = SelectStmt::builder()
        .table("v_subject_topics")
        .fields("id,title,slug,subject_slug,tag_names,summary,subject_name")
        .condition(condition)
        .order(order)
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
    let topic_rows = match super::execute(
        &tx,
        "UPDATE topic SET is_del=$1 WHERE id=$2",
        &[&is_del, &id],
    )
    .await
    {
        Ok(s) => s,
        Err(err) => {
            tx.rollback().await.map_err(AppError::from)?;
            return Err(AppError::from(err));
        }
    };
    let topic_tag_rows = match super::execute(
        &tx,
        "UPDATE topic_tag SET is_del=$1 WHERE topic_id=$2",
        &[&is_del, &id],
    )
    .await
    {
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
    match super::count(
        &tx,
        "SELECT COUNT(*) FROM topic WHERE subject_id=$1 AND slug=$2 AND id<>$3",
        &[&ut.subject_id, &ut.slug, &ut.id],
    )
    .await
    {
        Ok(row) if row > 0 => {
            tx.rollback().await.map_err(AppError::from)?;
            return Err(AppError::is_exists("已存在"));
        }
        Err(err) => {
            tx.rollback().await.map_err(AppError::from)?;
            return Err(err);
        }
        _ => {}
    };

    if let Err(err) = super::execute(&tx, "UPDATE topic SET title=$1, subject_id=$2, slug=$3, summary=$4, author=$5, src=$6 WHERE id=$7", &[
        &ut.title,
        &ut.subject_id,
        &ut.slug,
        &ut.summary,
        &ut.author,
        &ut.src,
        &ut.id,
    ]).await {
        tx.rollback().await.map_err(AppError::from)?;
        return Err(err);
    };
    // 内容
    if let Err(err) = super::execute(
        &tx,
        "UPDATE topic_content SET  md=$1, html=$2 WHERE topic_id=$3",
        &[&ut.md, &html, &ut.id],
    )
    .await
    {
        tx.rollback().await.map_err(AppError::from)?;
        return Err(err);
    };

    // tag
    // 删除所有关联的tag
    if let Err(err) =
        super::execute(&tx, "DELETE FROM topic_tag WHERE topic_id=$1", &[&ut.id]).await
    {
        tx.rollback().await.map_err(AppError::from)?;
        return Err(err);
    };

    // 添加并关联标签
    if !ut.tags.is_empty() {
        let tags = ut.tags.split(',').collect::<Vec<&str>>();
        let mut tags_id_list: Vec<TagID> = Vec::with_capacity(tags.len());
        for &tag_name in tags.iter() {
            let tags_id :TagID = match super::query_one(&tx, "INSERT INTO tag(name) VALUES($1) ON CONFLICT(name) DO UPDATE SET name=EXCLUDED.name RETURNING id",&[&tag_name] , Some("插入标签失败")).await {
                Ok(s) => s,
                Err(err) => {
                    tx.rollback().await.map_err(AppError::from)?;
                    return Err(err);
                }
            };
            tags_id_list.push(tags_id);
        }

        // topic_tag
        for tag_id in tags_id_list.iter() {
            if let Err(err) = super::execute(&tx, "INSERT INTO topic_tag (topic_id, tag_id) VALUES($1,$2) ON CONFLICT(topic_id,tag_id) DO NOTHING", &[&ut.id, &tag_id.id]).await {
                tx.rollback().await.map_err(AppError::from)?;
                return Err(AppError::from(err));

            };
        }
    }
    tx.commit().await.map_err(AppError::from)?;
    Ok(true)
}

pub async fn detail(client: &mut Client, subject_slug: &str, slug: &str) -> Result<TopicDetail> {
    let tx = client.transaction().await.map_err(AppError::from)?;
    let result:TopicDetail = match super::query_one(&tx, "SELECT id,title,subject_id,slug,author,src,html,tag_names,subject_slug,dateline,hit,subject_name FROM v_topic_detail WHERE subject_slug=$1 AND slug=$2", &[&subject_slug, &slug], Some("没有符合条件的文章")).await {
        Ok(s) => s,
        Err(err) => {
            tx.rollback().await.map_err(AppError::from)?;
            return Err(err);
        }
    };
    if let Err(err) =
        super::execute(&tx, "UPDATE topic SET hit=hit+1 WHERE id=$1", &[&result.id]).await
    {
        tx.rollback().await.map_err(AppError::from)?;
        return Err(err);
    };
    tx.commit().await.map_err(AppError::from)?;
    Ok(result)
}
