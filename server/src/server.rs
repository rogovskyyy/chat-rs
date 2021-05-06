#![allow(unused_imports)]
use tokio::net::{TcpListener, TcpStream};
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use std::sync::{Arc, Mutex};

pub struct Server;

unsafe impl Send for Server {}

impl Server {
    pub fn create() -> Self {
        Self { }
    }

    pub async fn listener() { 
        println!("*** Starting server at 127.0.0.1:25000 ***");
        let server = TcpListener::bind("127.0.0.1:25000").await.unwrap();
        println!("*** Ready to go... ***");
        
        // Listen for all incoming connection
        loop {
            // Foreach new connection that never existed before
            for packet in server.accept().await {
                match packet {
                    // If stream and socket exists - handle that
                    (mut stream, socket) => {
                        // Spawn thread to "hold" Tcp connection between server and client
                        tokio::spawn(async move {
                            loop {
                                println!("user => {}", socket);
                                Server::handler(&mut stream).await;
                            }
                        });
                    }
                }
            }
        }
    }

    async fn handler(stream: &mut TcpStream) { 
        let mut buffer: [u8; 16] = [0u8; 16];
        stream.read(&mut buffer).await.unwrap();
        println!("data => {:?} \n", buffer);
        stream.write(&buffer).await.unwrap();
    }
}