use crate::models::customer;
use sqlx::SqlitePool;


//get all customer
pub async fn get_all_customers(pool: &SqlitePool)-> Result<Vec<customer::Customer>, sqlx::Error>{
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


//update customer