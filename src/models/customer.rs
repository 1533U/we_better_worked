use serde::Serialize;
use chrono::NaiveDateTime;
use sqlx::FromRow;


// customer struct
#[derive(Debug,Serialize,FromRow)]
pub struct Customer{
    pub customer_id: i64,
    pub name: String,
    pub email: String,
    pub active: bool,

    pub creation_user_id: i64,
    pub last_modify_user_id: i64,
    pub last_modify_date: NaiveDateTime,
    pub creation_date: NaiveDateTime,
}

#[derive(Debug,Serialize,FromRow)]
pub struct NewCustomer {
    pub name: String,
    pub email: String,
    pub active: bool,

    pub creation_user_id: i64,
    pub last_modify_user_id: i64,
}

#[derive(Debug,Serialize,FromRow)]
pub struct UpdateCustomer {
    pub name: String,
    pub email: String,
    pub active: bool,

    pub last_modify_user_id: i64,
    pub last_modify_date: NaiveDateTime,
}