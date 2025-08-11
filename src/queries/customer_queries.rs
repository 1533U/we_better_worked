use crate::models::customer;
use sqlx::SqlitePool;

//get all customer
pub async fn get_all_customers(pool: &SqlitePool) -> Result<Vec<customer::Customer>, sqlx::Error> {
    let customers: Vec<customer::Customer> = sqlx::query_as!(
        customer::Customer,
        r#"
        SELECT customer_id, name, email, active, creation_user_id, last_modify_user_id, last_modify_date, creation_date
        FROM customer
        "#
    ).fetch_all(pool)
    .await?;

    Ok(customers)
}

//insert new customer
pub async fn insert_customer(pool: &SqlitePool,customer: customer::NewCustomer) -> Result<i64, sqlx::Error> {
    sqlx::query!(
        "INSERT INTO customer (name, email, active, creation_user_id, last_modify_user_id, last_modify_date, creation_date)
        VALUES (?, ?, ?, ?, ?, datetime('now'), datetime('now'))",
        customer.name,
        customer.email,
        customer.active,
        customer.creation_user_id,
        customer.last_modify_user_id
    )
    .execute(pool)
    .await?;

    Ok(sqlx::query_scalar!("SELECT last_insert_rowid()")
        .fetch_one(pool)
        .await?)
}

//update customer
pub async fn update_customer(pool: &SqlitePool, customer: customer::UpdateCustomer) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE customer SET name = ?, email = ?, active = ?, last_modify_user_id = ?, last_modify_date = datetime('now') WHERE customer_id = ?",
        customer.name,
        customer.email,
        customer.active,
        customer.last_modify_user_id,
        customer.customer_id
    )
    .execute(pool)
    .await?;

    Ok(())  
}
