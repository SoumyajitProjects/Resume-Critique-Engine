use sqlx::{PgPool, postgres::PgPoolOptions};
use anyhow::Result;

pub async fn create_pool(database_url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await?;
    
    // Run migrations if needed
    sqlx::migrate!("./migrations").run(&pool).await.ok();
    
    Ok(pool)
}
