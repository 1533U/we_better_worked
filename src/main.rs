mod queries;
mod models;
mod db; 

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("Hello, world!");
    let pool = db::connection::create_pool().await;
    println!("{:?}", pool);

}
