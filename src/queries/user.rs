use crate::models::user;
use sqlx::SqlitePool;

pub async fn get_all_users(pool: &SqlitePool) -> Result<Vec<user::User>, sqlx::Error> {
    let users: Vec<user::User> = sqlx::query_as!(
        user::User,
        r#"
        SELECT user_id, display_name, email, active, password_hash, creation_date
        FROM user_tbl
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}


pub async fn insert_user(pool: &SqlitePool,user: user::NewUser) -> Result<i64,sqlx::Error>{

    sqlx::query!(
    "INSERT INTO user_tbl (display_name, email, active, password_hash, creation_date)
         VALUES (?, ?, ?, ?, datetime('now'))",
        user.display_name,
        user.email,
        true,//allways insert as active
        user.password_hash
    )
    .execute(pool)
    .await?;

    Ok(sqlx::query_scalar!("SELECT last_insert_rowid()").fetch_one(pool).await?)
}

pub async fn update_user(pool: &SqlitePool, user: user::UpdateUser) -> Result<(), sqlx::Error>{
    sqlx::query!(
        "UPDATE user_tbl SET display_name = ?, email = ?, active = ?, password_hash = ? WHERE user_id = ?",
        user.display_name,
        user.email,
        user.active,
        user.password_hash,
        user.user_id
    )   
    .execute(pool)
    .await?;

    Ok(())

}

//user should never be deleted in code