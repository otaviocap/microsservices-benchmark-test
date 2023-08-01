mod components;
mod utils;
mod server;

#[tokio::main]
async fn main() {

    println!("RUST => Db Consumer running!");

    dotenv::dotenv().ok();

    let app = server::build().await;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("Rust Db Consumer is listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}