use crate::db::connection::SqlitePool;
use crate::models::user::{User, NewUser, UpdateUser};
use crate::queries::user_queries; 
use bcrypt::{hash, DEFAULT_COST};
use regex::Regex;


#[derive(thiserror::Error, Debug)]
pub enum UserCreationError {
    #[error("Email is invalid")]
    InvalidEmail,
    #[error("Email already exists")]
    EmailExists,
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Hashing error: {0}")]
    HashError(#[from] bcrypt::BcryptError),
}


pub async fn list_users(pool: &SqlitePool) -> Result<Vec<User>, sqlx::Error> {
    user_queries::get_all_users(pool).await
}

pub async fn create_user(pool: &SqlitePool, user: NewUser) -> Result<i64, sqlx::Error> {
  
    // 1. Validate email format (basic)
    let email_regex = Regex::new(r"^[\w\.-]+@[\w\.-]+\.\w+$").unwrap();
    if !email_regex.is_match(&new_user.email) {
        return Err(UserCreationError::InvalidEmail);
    }

    // 2. check if email 
    let existing_user = user_queries::get_user_by_email(pool, &user.email).await;
    if existing_user.is_ok() {
        return Err(UserCreationError::EmailExists);
    }

    // 3. Hash password
    let password_hash = bcrypt::hash(&user.password_hash, bcrypt::DEFAULT_COST)?;

    // 4. Create user
    let user = NewUser {
        display_name: user.display_name,
        email: user.email,
        password_hash: password_hash,
    };

    // 5. Insert user
    let user_id = user_queries::insert_user(pool, user).await?;

    Ok(user_id)
}

pub async fn update_user(pool: &SqlitePool, user: UpdateUser) -> Result<(), sqlx::Error> {
    user_queries::update_user(pool, user).await
}