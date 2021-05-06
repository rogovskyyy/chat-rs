#![allow(unused_imports)]
use tokio::net::{TcpListener, TcpStream};
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use tokio::io::{AsyncWriteExt, AsyncReadExt, AsyncRead};
use std::io::Read;
use std::fs;

pub struct Server;

impl Server {
    pub async fn listener() { 
        println!("*** Starting server at 0.0.0.0:14070 ***");
        let server = TcpListener::bind("0.0.0.0:14070").await.unwrap();
        println!("*** Ready to go... ***");
        
        loop {
            for packet in server.accept().await {
                match packet {
                    (mut stream, socket) => {
                        tokio::spawn(async move {
                            println!("user => {}", socket);
                            Server::handler(&mut stream).await;
                            drop(stream);
                        });
                    }
                }
            }
        }
    }

    async fn handler(stream: &mut TcpStream) { 
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).await.unwrap();
    
        let contents = fs::read_to_string("index.html").unwrap();
    
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
    
        stream.write(response.as_bytes()).await.unwrap();
        stream.flush().await.unwrap();
    }
}