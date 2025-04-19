use sqlx::{PgPool, FromRow};
use anyhow::Result;

#[derive(Debug, FromRow)]
pub struct Order {
    pub id: i64,
    pub user_id: i32,
    pub products_ids: Vec<i32>,
    pub is_paid: bool,
}

pub async fn create_order(pool: &PgPool, user_id: i32, products_ids: Vec<i32>) -> Result<()> {
    sqlx::query_file!("queries/create.sql", user_id, &products_ids)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_by_id(pool: &PgPool, id: i64) -> Result<()> {
    sqlx::query_file!("queries/delete_by_id.sql", id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn init_table(pool: &PgPool) -> Result<()> {
    sqlx::query_file!("queries/init_table.sql")
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn pay_by_id(pool: &PgPool, id: i64) -> Result<()> {
    sqlx::query_file!("queries/pay_by_id.sql", id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn products_by_user_id(pool: &PgPool, user_id: i32) -> Result<Vec<Option<i32>>> {
    let rows = sqlx::query_file_scalar!("queries/products_by_user_id.sql", user_id)
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

pub async fn read_by_id(pool: &PgPool, id: i64) -> Result<Option<Order>> {
    let order = sqlx::query_file_as!(Order, "queries/read_by_id.sql", id)
        .fetch_optional(pool)
        .await?;
    Ok(order)
}

pub async fn read_by_user_id(pool: &PgPool, user_id: i32) -> Result<Vec<Order>> {
    let orders = sqlx::query_file_as!(Order, "queries/read_by_user_id.sql", user_id)
        .fetch_all(pool)
        .await?;
    Ok(orders)
}

