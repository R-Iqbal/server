use std::error::Error;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

use crate::chat;

pub fn main() -> Result<(), Box<dyn Error>> {
    let address = String::from("127.0.0.1");
    let port = String::from("3040");

    let mut server = chat::Server::new(address, port)?;

    server.start_listening();

    Ok(())
}
