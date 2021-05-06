mod server;

use server::Server;
use tokio::*;

#[tokio::main]
async fn main() {
    tokio::spawn(async move {
        Server::listener().await;
    }).await;
}