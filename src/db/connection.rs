
use sqlx::sqlite::SqlitePool;
use std::env;

pub(crate) async fn create_pool() -> SqlitePool {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Connecting to {}", db_url);
    SqlitePool::connect(&db_url).await.unwrap()
}

pub(crate) async fn get_users(pool: &SqlitePool) -> Result<(), sqlx::Error> {

    let users = sqlx::query!(
        r#"SELECT display_name FROM user_tbl"#
    ).fetch_all(pool).await?;

    for user in users {
        println!("{:?}", user);
    }
    Ok(())
}