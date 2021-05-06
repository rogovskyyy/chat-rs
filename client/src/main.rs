mod client;

use client::Client;
use tokio::*;

#[tokio::main]
async fn main() {
    tokio::spawn(async move {
        Client::sender().await;
    }).await;
}