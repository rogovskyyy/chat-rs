use std::net::{TcpStream, TcpListener, SocketAddr, IpAddr, Ipv4Addr};
use std::io::{Read, Write};
use std::time;

pub struct Client;

impl Client { 
    pub async fn sender() { 
        let mut client = TcpStream::connect("127.0.0.1:25000").unwrap();
        let mut buffer : [u8; 16] = [0x0; 16];
        loop {
            client.write(&buffer).unwrap();
            client.read(&mut buffer).unwrap();
            println!("{:?}", buffer);
            let time = time::Duration::from_millis(1000);
            std::thread::sleep(time);
        }
    }
}