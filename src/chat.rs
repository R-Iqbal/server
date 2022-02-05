use std::error::Error;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

pub struct Server {
    pub host: String,
    address: String,
    port: String,
    listener: TcpListener,
    pub connected_clients: u64,
}

impl Server {
    pub fn new(address: String, port: String) -> Result<Server, std::io::Error> {
        let mut host = address.clone();
        host.push(':');
        host.push_str(&port);

        let listener = TcpListener::bind(&host)?;
        Ok(Server {
            host,
            address,
            port,
            listener,
            connected_clients: 0,
        })
    }

    pub fn connection_handler(mut stream: TcpStream) {
        let mut data = [0 as u8; 50]; // using 50 byte buffer

        std::thread::spawn(move || loop {
            match stream.read(&mut data) {
                Ok(size) => {
                    let data = data[..size].to_vec();
                }
                Err(_) => todo!(),
            }
        });
    }

    pub fn start_listening(&mut self) {
        
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.connected_clients += 1;
                    Self::connection_handler(stream);
                }
                Err(e) => {
z
                }
            }
        }
    }
}
