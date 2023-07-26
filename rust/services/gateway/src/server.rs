use axum::{Router, routing::get};
use hyper::client::HttpConnector;

pub async fn server() {

    let client = hyper::Client::builder().build(HttpConnector::new());

    let app = Router::new().route("/", get(handler)).with_state(client);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("Rust Gateway listening on {}", listener.local_addr().unwrap());
    
    axum::serve(listener, app).await.unwrap();
}