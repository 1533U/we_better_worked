mod db;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("Hello, world!");
    let pool = db::connection::create_pool().await;
    println!("{:?}", pool);

    //select users
   let ok = db::connection::get_users(&pool).await;
}
