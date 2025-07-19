use crate::models::user::User;
use sqlx::SqlitePool;

pub async fn get_all_users(pool: &SqlitePool) -> Result<Vec<User>, sqlx::Error> {
    let users: Vec<User> = sqlx::query_as!(
        User,
        r#"
        SELECT user_id, display_name, email, active, password_hash, creation_date
        FROM user_tbl
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}
