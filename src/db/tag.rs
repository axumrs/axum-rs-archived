use deadpool_postgres::Client;
use tokio_postgres::types::ToSql;

use crate::{
    error::AppError,
    form::{CreateTag, UpdateTag},
    model::{Tag, TagID},
    Result,
};

use super::{execute, pagination::Pagination, query_one, select_stmt::SelectStmt, PAGE_SIZE};

pub async fn select(
    client: &Client,
    condition: Option<&str>,
    args: &[&(dyn ToSql + Sync)],
    page: u32,
) -> Result<Pagination<Vec<Tag>>> {
    let sql = SelectStmt::builder()
        .table("tag")
        .fields("id,name,is_del")
        .condition(condition)
        .order(Some("id DESC"))
        .limit(Some(PAGE_SIZE))
        .offset(Some(page * PAGE_SIZE as u32))
        .build();
    let count_sql = SelectStmt::builder()
        .table("tag")
        .fields("COUNT(*)")
        .condition(condition)
        .build();
    super::select::<Tag>(client, &sql, &count_sql, args, page).await
}
pub async fn find(
    client: &Client,
    condition: Option<&str>,
    args: &[&(dyn ToSql + Sync)],
) -> Result<Tag> {
    let sql = SelectStmt::builder()
        .table("tag")
        .fields("id,name,is_del")
        .condition(condition)
        .limit(Some(1))
        .build();
    query_one::<Tag>(client, &sql, args, Some("没有找到符合条件的标签")).await
}
pub async fn count(
    client: &Client,
    condition: Option<&str>,
    args: &[&(dyn ToSql + Sync)],
) -> Result<i64> {
    let sql = SelectStmt::builder()
        .table("tag")
        .fields("COUNT(*)")
        .condition(condition)
        .build();
    super::count(client, &sql, args).await
}
pub async fn is_exists(
    client: &Client,
    condtion: &str,
    args: &[&(dyn ToSql + Sync)],
) -> Result<bool> {
    let c = count(client, Some(condtion), args).await?;
    Ok(c > 0)
}
pub async fn name_is_exists(client: &Client, name: &str) -> Result<bool> {
    is_exists(client, &format!("name=$1"), &[&name]).await
}
pub async fn del(client: &Client, id: i32) -> Result<u64> {
    super::del(client, "tag", &id).await
}
pub async fn restore(client: &Client, id: i32) -> Result<u64> {
    super::restore(client, "tag", &id).await
}
pub async fn create(client: &Client, ct: &CreateTag) -> Result<TagID> {
    if name_is_exists(client, &ct.name).await? {
        return Err(AppError::is_exists("同名的标签已存在"));
    }
    let sql = "INSERT INTO tag (name, is_del) VALUES ($1, false) RETURNING id";
    query_one::<TagID>(client, sql, &[&ct.name], Some("创建标签失败")).await
}
pub async fn update(client: &Client, ut: &UpdateTag) -> Result<u64> {
    if is_exists(client, "name=$1 AND id<>$2", &[&ut.name, &ut.id]).await? {
        return Err(AppError::is_exists("同名的标签已存在"));
    }
    let sql = "UPDATE tag SET name =$1 WHERE id=$2";
    execute(client, sql, &[&ut.name, &ut.id]).await
}
pub async fn all(client: &Client) -> Result<Vec<Tag>> {
    let sql = SelectStmt::builder()
        .table("tag")
        .fields("id,name,is_del")
        .condition(Some("is_del=false"))
        .limit(Some(u8::MAX))
        .order(Some("id ASC"))
        .build();
    super::query(client, &sql, &[]).await
}
