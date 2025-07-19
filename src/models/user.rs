use serde::Serialize;
use chrono::NaiveDateTime;
use sqlx::FromRow;


// user_tbl struct
#[derive(Debug,Serialize,FromRow)]
pub struct User{
    pub user_id: i64,
    pub display_name: String,
    pub email: String,
    pub active: bool,
    pub password_hash: String,
    pub creation_date: NaiveDateTime,
}
