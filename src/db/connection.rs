
use sqlx::sqlite::SqlitePool;
use std::env;

pub(crate) async fn create_pool() -> SqlitePool {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Connecting to {}", db_url);
    SqlitePool::connect(&db_url).await.unwrap()
}

