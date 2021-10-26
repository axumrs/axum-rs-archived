//! 主题数据库操作

use crate::error::AppError;
use crate::form::{CreateSubject, UpdateSubject};
use crate::model::{Subject, SubjectID, SubjectList};
use crate::Result;
use deadpool_postgres::Client;
use tokio_postgres::types::ToSql;

use super::pagination::Pagination;
use super::select_stmt::SelectStmt;
use super::{execute, query, PAGE_SIZE};

/// 表名
const TABLE_NAME: &str = "subject";

/// 根据条件统计。返回符合条件的记录数，或包含[`AppError`]的错误信息
///
/// # 参数
///
/// * `client` - 数据库连接对象
/// * `condition` - 条件
/// * `args` - 条件对应的参数
async fn count(
    client: &Client,
    condition: Option<&str>,
    args: &[&(dyn ToSql + Sync)],
) -> Result<i64> {
    let sql_count = SelectStmt::builder()
        .table(TABLE_NAME)
        .fields("COUNT(*)")
        .condition(condition)
        .build();
    super::count(client, &sql_count, args).await
}

/// 根据条件判断主题是否存在。返回主题是否存在，或包含[`AppError`]的错误信息
///
/// # 参数
///
/// * `client` - 数据库连接对象
/// * `condition` - 条件
/// * `args` - 条件对应的参数
async fn is_exists(
    client: &Client,
    condition: Option<&str>,
    args: &[&(dyn ToSql + Sync)],
) -> Result<bool> {
    let c = count(client, condition, args).await?;
    Ok(c > 0)
}

/// 获取主题列表。返回满足条件的主题列表及分页([`Pagination`])信息，或包含[`AppError`]的错误信息
///
/// # 参数
///
/// * `client` - 数据库连接对象
/// * `condition` - 条件
/// * `args` - 条件对应的参数
/// * `page` - 当前分页的页码
pub async fn select(
    client: &Client,
    condition: &str,
    args: &[&(dyn ToSql + Sync)],
    page: u32,
) -> Result<Pagination<Vec<SubjectList>>> {
    let sql = SelectStmt::builder()
        .table(TABLE_NAME)
        .fields("id, name, slug, is_del")
        .condition(Some(condition))
        .order(Some("id DESC"))
        .limit(Some(PAGE_SIZE))
        .offset(Some(page * PAGE_SIZE as u32))
        .build();
    tracing::debug!("{}", sql);
    let result = query::<SubjectList>(client, &sql, args).await?;
    let total_records = count(client, Some(condition), args).await?;
    Ok(Pagination::new(page, PAGE_SIZE, total_records, result))
}

/// 根据条件获取主题。返回主题，或包含[`AppError`]的错误信息
///
/// # 参数
///
/// * `client` - 数据库连接对象
/// * `condition` - 条件
/// * `args` - 条件对应的参数
pub async fn find(
    client: &Client,
    condition: Option<&str>,
    args: &[&(dyn ToSql + Sync)],
) -> Result<Subject> {
    let sql = SelectStmt::builder()
        .table(TABLE_NAME)
        .fields("id, name, slug,summary,is_del")
        .condition(condition)
        .limit(Some(1))
        .build();
    let result = query::<Subject>(client, &sql, args).await?.pop();
    match result {
        Some(subject) => Ok(subject),
        None => Err(AppError::not_found("没有找到符合条件的主题")),
    }
}
/// 根据固定链接获取主题。返回主题，或包含[`AppError`]的错误信息
///
/// # 参数
///
/// * `client` - 数据库连接对象
/// * `slug` - 固定链接
pub async fn find_by_slug(client: &Client, slug: &str) -> Result<Subject> {
    find(client, Some("slug=$1"), &[&slug]).await
}

/// 判断主题的固定链接是否存在。返回固定链接是否存在，或包含[`AppError`]的错误信息
///
/// # 参数
///
/// * `client` - 数据库连接对象
/// * `slug` - 固定链接
pub async fn slug_is_exists(client: &Client, slug: &str) -> Result<bool> {
    is_exists(client, Some("slug=$1"), &[&slug]).await
}

/// 创建主题。返回新创建的主题的ID，或包含[`AppError`]的错误信息
///
/// # 参数
///
/// * `client` - 数据库连接对象
/// * `cs` - 输入的主题信息
pub async fn create(client: &Client, cs: &CreateSubject) -> Result<SubjectID> {
    if slug_is_exists(client, &cs.slug).await? {
        return Err(AppError::is_exists(&format!(
            "主题的固定链接 '{}' 已存在",
            &cs.slug
        )));
    };
    let sql = "INSERT INTO subject (name, slug, summary) VALUES ($1, $2, $3) RETURNING id";
    query::<SubjectID>(client, sql, &[&cs.name, &cs.slug, &cs.summary])
        .await?
        .pop()
        .ok_or(AppError::db_error_from_str("插入主题失败"))
}

/// 更新主题。返回更新结果，或包含[`AppError`]的错误信息
///
/// # 参数
///
/// * `client` - 数据库连接对象
/// * `us` - 输入的主题信息
pub async fn update(client: &Client, us: &UpdateSubject) -> Result<bool> {
    if is_exists(client, Some("slug=$1 AND id<>$2"), &[&us.slug, &us.id]).await? {
        return Err(AppError::is_exists(&format!(
            "主题的固定链接 '{}' 已存在",
            &us.slug
        )));
    }
    let result = execute(
        client,
        "UPDATE subject SET name=$1, slug=$2, summary=$3 WHERE id=$4",
        &[&us.name, &us.slug, &us.summary, &us.id],
    )
    .await?;
    match result {
        ref updated if *updated == 1 => Ok(true),
        _ => Ok(false),
    }
}

/// 删除或恢复主题。返回操作结果，或包含[`AppError`]的错误信息
///
/// # 参数
///
/// * `client` - 数据库连接对象
/// * `id` - 要操作的主题ID
/// * `is_del_opt` - 是否为删除操作
async fn del_or_restore(client: &Client, id: i32, is_del_opt: bool) -> Result<bool> {
    let result = execute(
        client,
        "UPDATE subject SET is_del=$1 WHERE id=$2",
        &[&is_del_opt, &id],
    )
    .await?;
    match result {
        ref updated if *updated == 1 => Ok(true),
        _ => Ok(false),
    }
}

/// 删除主题。返回操作结果，或包含[`AppError`]的错误信息
///
/// # 参数
///
/// * `client` - 数据库连接对象
/// * `id` - 要操作的主题ID
pub async fn delete(client: &Client, id: i32) -> Result<bool> {
    del_or_restore(client, id, true).await
}

/// 恢复主题。返回操作结果，或包含[`AppError`]的错误信息
///
/// # 参数
///
/// * `client` - 数据库连接对象
/// * `id` - 要操作的主题ID
pub async fn restore(client: &Client, id: i32) -> Result<bool> {
    del_or_restore(client, id, false).await
}
