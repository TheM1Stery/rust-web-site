use sqlx::sqlite::{Sqlite, SqlitePool};
use sqlx::{Pool, Result};

pub async fn get_pooled_connection(connection_str: &str) -> Result<Pool<Sqlite>> {
    SqlitePool::connect(connection_str).await
}
