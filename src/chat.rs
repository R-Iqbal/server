use std::error::Error;
use std::io::prelude::*;
use std::net::{Shutdown, TcpListener, TcpStream};

pub enum Event {
    Connected { username: String },
    SetUsername { username: String },
    Disconnected { username: String },
    CreateRoom { roomId: String },
    JoinRoom { roomId: String },
}

pub enum Commands {
    SetUsername,
}

impl Commands {
    pub fn value(&self) -> String {
        match self {
            Commands::SetUsername => String::from("!username"),
        }
    }
}

pub struct EventHandler {}

impl EventHandler {
    pub fn handle_event(event: Event) {
        println!("Tiggering the event handler!");
        match event {
            Event::Connected { username } => {}
            Event::Disconnected { username } => todo!(),
            Event::CreateRoom { roomId } => todo!(),
            Event::JoinRoom { roomId } => todo!(),
            Event::SetUsername { username } => {
                println!("The user is trying to change their name to: {}", username);
            }
        }
    }
}

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

        std::thread::spawn(move || {
            while match stream.read(&mut data) {
                Ok(size) => {
                    let data = data[..size].to_vec();
                    let message = String::from_utf8(data).unwrap();

                    if (message.len() > 0) {
                        println!("Recieved a message: {:?}", message);

                        // Begin event handle

                        if message.starts_with(&Commands::SetUsername.value()) {
                            let username =
                                message.replace("!username", "").trim_start().to_string();
                            let event = Event::SetUsername { username };
                            EventHandler::handle_event(event);
                        }
                    }

                    true
                }
                Err(_) => {
                    stream.shutdown(Shutdown::Both).unwrap();
                    true
                }
            } {}
        });
    }

    pub fn start_listening(&mut self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.connected_clients += 1;
                    println!(
                        "A new client has connected! There are now {} connected clinets",
                        self.connected_clients
                    );
                    Self::connection_handler(stream);
                }
                Err(e) => {
                    panic!("Uh oh!")
                }
            }
        }
    }
}
