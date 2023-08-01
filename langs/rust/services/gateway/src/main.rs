mod panic_handler;
mod server;
mod router;

use crate::server::run_server;

#[tokio::main]
async fn main() {
    println!("RUST => Gateway running!");
    
    run_server().await;
}
