mod db;
mod queries;
mod models;

use queries::user as user_queries;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("Hello, world!");
    let pool = db::connection::create_pool().await;
    println!("{:?}", pool);

    //select users
    let users = user_queries::get_all_users(&pool).await.unwrap();
    for user in users {
        println!("{:?}", user);
    }

}
