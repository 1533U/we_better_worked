mod db;
mod queries;
mod models;

use queries::user as user_queries;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("Hello, world!");
    let pool = db::connection::create_pool().await;
    println!("{:?}", pool);


    //insert user
    // let user = models::user::NewUser { display_name: "foo".to_string(), email: "foo@foo".to_string(), password_hash: "foo".to_string()};

    // match user_queries::insert_user(&pool, user).await {
    //     Ok(user_id) => println!("Inserted user with id {}", user_id),
    //     Err(e) => println!("Error inserting user: {}", e),
    // }

    //select users
    let users = user_queries::get_all_users(&pool).await.unwrap();
    for user in users {
        println!("{:?}", user);
    }

    //update user
    let user = models::user::UpdateUser { display_name: "foo".to_string(), email: "foo5@foo".to_string(), active: true, password_hash: "foo".to_string(), user_id:1};
    match user_queries::update_user(&pool, user).await {
        Ok(()) => println!("Updated user"),
        Err(e) => println!("Error updating user: {}", e),
    }

    //select users
    let users = user_queries::get_all_users(&pool).await.unwrap();
    for user in users {
        println!("{:?}", user);
    }

}
