use tokio_postgres::{types::ToSql, Statement};

use crate::{error::AppError, Result};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use self::pagination::Pagination;

pub mod pagination;
pub mod select_stmt;
pub mod subject;
pub mod tag;

/// 默认分页大小
const PAGE_SIZE: u8 = 30;

/// 获取[`Statement`]对象。
///
/// # 参数
///
/// * `client` - 数据库连接对象
/// * `sql` - SQL语句
async fn get_stmt(client: &Client, sql: &str) -> Result<Statement> {
    client.prepare(sql).await.map_err(AppError::from)
}

/// 查询数据库
///
/// # 参数
///
/// * `client` - 数据库连接对象
/// * `sql` - SQL语句
/// * `args` - 查询参数
async fn query<T: FromTokioPostgresRow>(
    client: &Client,
    sql: &str,
    args: &[&(dyn ToSql + Sync)],
) -> Result<Vec<T>> {
    let stmt = get_stmt(client, sql).await?;
    let result = client
        .query(&stmt, args)
        .await
        .map_err(AppError::from)?
        .iter()
        .map(|row| <T>::from_row_ref(row).unwrap())
        .collect::<Vec<T>>();
    Ok(result)
}

async fn query_one<T: FromTokioPostgresRow>(
    client: &Client,
    sql: &str,
    args: &[&(dyn ToSql + Sync)],
    msg: Option<&str>,
) -> Result<T> {
    let msg = match msg {
        Some(msg) => msg,
        None => "没有找到符合条件的记录",
    };
    query::<T>(client, &sql, args)
        .await?
        .pop()
        .ok_or(AppError::not_found(msg))
}

async fn del_or_restore(
    client: &Client,
    table: &str,
    id: &(dyn ToSql + Sync),
    is_del_opt: bool,
) -> Result<u64> {
    let is_del = if is_del_opt { true } else { false };
    let sql = format!("UPDATE {} SET is_del=$1 WHERE id=$2", table);
    execute(client, &sql, &[&is_del, id]).await
}
async fn del(client: &Client, table: &str, id: &(dyn ToSql + Sync)) -> Result<u64> {
    del_or_restore(client, table, id, true).await
}
async fn restore(client: &Client, table: &str, id: &(dyn ToSql + Sync)) -> Result<u64> {
    del_or_restore(client, table, id, false).await
}

/// 执行数据库语句
///
/// # 参数
///
/// * `client` - 数据库连接对象
/// * `sql` - SQL语句
/// * `args` - 查询参数
async fn execute(client: &Client, sql: &str, args: &[&(dyn ToSql + Sync)]) -> Result<u64> {
    let stmt = get_stmt(client, sql).await?;
    client.execute(&stmt, args).await.map_err(AppError::from)
}

/// 统计记录数
///
/// # 参数
///
/// * `client` - 数据库连接对象
/// * `sql` - SQL语句
/// * `args` - 查询参数
async fn count(client: &Client, sql: &str, args: &[&(dyn ToSql + Sync)]) -> Result<u32> {
    let stmt = get_stmt(client, sql).await?;
    let result = client
        .query_one(&stmt, args)
        .await
        .map_err(AppError::from)?
        .get(0);
    Ok(result)
}

async fn select<T: FromTokioPostgresRow>(
    client: &Client,
    sql: &str,
    count_sql: &str,
    args: &[&(dyn ToSql + Sync)],
    page: u32,
) -> Result<Pagination<Vec<T>>> {
    let data = query::<T>(client, sql, args).await?;
    let total_records = count(client, count_sql, args).await?;
    Ok(Pagination::new(page, PAGE_SIZE, total_records, data))
}
