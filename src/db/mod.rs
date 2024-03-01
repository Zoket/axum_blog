use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::{
    types::{FromSqlOwned, ToSql},
    GenericClient, Statement,
};

pub mod paginate;
pub mod category;
pub mod topic;
pub mod admin;

use crate::{error::AppError, Result};

use self::paginate::Paginate;

const DEFAULT_PAGE_SIZE: u8 = 20;

async fn get_stmt(client: &impl GenericClient, sql: &str) -> Result<Statement> {
    client.prepare(sql).await.map_err(AppError::from)
}

async fn query<T>(
    client: &impl GenericClient,
    sql: &str,
    params: &[&(dyn ToSql + Sync)],
) -> Result<Vec<T>>
where
    T: FromTokioPostgresRow,
{
    let stmt = get_stmt(client, sql).await?;
    let result = client
        .query(&stmt, params)
        .await
        .map_err(AppError::from)?
        .iter()
        .map(|row: &tokio_postgres::Row| <T>::from_row_ref(row).unwrap())
        .collect::<Vec<T>>();
    Ok(result)
}

async fn query_row_opt<T>(
    client: &impl GenericClient,
    sql: &str,
    params: &[&(dyn ToSql + Sync)],
    msg: Option<String>,
) -> Result<T>
where
    T: FromTokioPostgresRow,
{
    query(client, sql, params)
        .await?
        .pop()
        .ok_or(AppError::notfound_opt(msg))
}

async fn query_row_msg<T>(
    client: &impl GenericClient,
    sql: &str,
    params: &[&(dyn ToSql + Sync)],
    msg: &str,
) -> Result<T>
where
    T: FromTokioPostgresRow,
{
    query_row_opt(client, sql, params, Some(msg.to_string())).await
}

async fn query_row<T>(
    client: &impl GenericClient,
    sql: &str,
    params: &[&(dyn ToSql + Sync)],
) -> Result<T>
where
    T: FromTokioPostgresRow,
{
    query_row_opt(client, sql, params, None).await
}

async fn insert<T>(
    client: &impl GenericClient,
    sql: &str,
    params: &[&(dyn ToSql + Sync)],
    msg: &str,
) -> Result<T>
where
    T: FromTokioPostgresRow,
{
    query_row_msg(client, sql, params, msg).await
}

async fn query_col<T>(
    client: &impl GenericClient,
    sql: &str,
    params: &[&(dyn ToSql + Sync)],
) -> Result<T>
where
    T: FromSqlOwned,
{
    let stmt = get_stmt(client, sql).await?;
    Ok(client
        .query_one(&stmt, params)
        .await
        .map_err(AppError::from)?
        .get(0))
}

async fn count(
    client: &impl GenericClient,
    sql: &str,
    params: &[&(dyn ToSql + Sync)],
) -> Result<i64> {
    query_col(client, sql, params).await
}

async fn execute(
    client: &impl GenericClient,
    sql: &str,
    args: &[&(dyn ToSql + Sync)],
) -> Result<u64> {
    let stmt = get_stmt(client, sql).await?;
    client.execute(&stmt, args).await.map_err(AppError::from)
}

async fn pagination<T>(
    client: &impl GenericClient,
    sql: &str,
    count_sql: &str,
    params: &[&(dyn ToSql + Sync)],
    page: u32,
) -> Result<Paginate<Vec<T>>>
where
    T: FromTokioPostgresRow,
{
    let data = query(client, sql, params).await?;
    let total_records = count(client, count_sql, params).await?;
    Ok(Paginate::new(page, DEFAULT_PAGE_SIZE, total_records, data))
}

async fn del_or_restore(
    client: &impl GenericClient,
    table: &str,
    id: &(dyn ToSql + Sync),
    is_del: bool,
) -> Result<u64> {
    let sql = format!("UPDATE {} SET is_del = $1 WHERE id = $2", table);
    execute(client, &sql, &[&is_del, id]).await
}
