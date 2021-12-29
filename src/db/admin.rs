use tokio_postgres::types::ToSql;
use tokio_postgres::Client;

use crate::{
    error::AppError,
    form::{CreateAdmin, UpdateAdmin},
    model::{Admin, AdminID},
    Result,
};

use super::{pagination::Pagination, select_stmt::SelectStmt, PAGE_SIZE};

pub async fn create(client: &Client, ca: CreateAdmin) -> Result<AdminID> {
    let sql = "SELECT COUNT(*) FROM admin WHERE username=$1";
    let c = super::count(client, sql, &[&ca.username]).await?;
    if c > 0 {
        return Err(AppError::is_exists("同名的管理员已存在"));
    }
    let sql = "INSERT INTO admin (username, password) VALUES ($1, $2) RETURNING id";
    Ok(super::query_one(
        client,
        sql,
        &[&ca.username, &ca.password],
        Some("添加管理员失败"),
    )
    .await?)
}
pub async fn del_or_restore(client: &Client, id: i32, is_del_opt: bool) -> Result<()> {
    let sql = "UPDATE admin SET is_del = $1 WHERE id=$2 AND is_sys=false";
    super::execute(client, sql, &[&is_del_opt, &id]).await?;
    Ok(())
}

pub async fn find_by_condition(
    client: &Client,
    condition: &str,
    args: &[&(dyn ToSql + Sync)],
) -> Result<Admin> {
    let sql = SelectStmt::builder()
        .table("admin")
        .fields("id, username, password, is_sys, is_del")
        .condition(Some(condition))
        .limit(Some(1))
        .build();
    Ok(super::query_one(client, &sql, args, Some("不存在的管理员")).await?)
}
pub async fn find(client: &Client, username: &str) -> Result<Admin> {
    find_by_condition(client, "username=$1 AND is_del=false", &[&username]).await
}
pub async fn find_by_id(client: &Client, id: i32) -> Result<Admin> {
    find_by_condition(client, "id=$1", &[&id]).await
}

pub async fn select(
    client: &Client,
    condition: Option<&str>,
    args: &[&(dyn ToSql + Sync)],
    page: u32,
) -> Result<Pagination<Vec<Admin>>> {
    let sql = SelectStmt::builder()
        .table("admin")
        .fields("id,username,password,is_sys,is_del")
        .condition(condition)
        .order(Some("id DESC"))
        .limit(Some(PAGE_SIZE))
        .offset(Some(page * PAGE_SIZE as u32))
        .build();
    let count_sql = SelectStmt::builder()
        .table("admin")
        .fields("COUNT(*)")
        .condition(condition)
        .build();
    Ok(super::select(client, &sql, &count_sql, args, page).await?)
}
pub async fn update(client: &Client, ua: UpdateAdmin) -> Result<u64> {
    let sql = "UPDATE admin SET password = $1 WHERE id=$2";
    super::execute(client, sql, &[&ua.new_password, &ua.id]).await
}
