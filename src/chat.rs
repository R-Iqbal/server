use std::error::Error;
use std::fmt;
use std::io::prelude::*;

use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use crossbeam_channel::{bounded, Receiver, Sender};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct SocketMessage {
    r#type: String,
    data: DataKind,
}
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum DataKind {
    StringVector(Vec<String>),
}

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
    pub fn handle_event(event: Event, connected_clients: &Arc<Mutex<Vec<String>>>) {
        println!("Tiggering the event handler!");
        match event {
            Event::Connected { username } => {
                // Acquire a lock on the mutex guard
                let mut connected_clients = connected_clients.lock().unwrap();
                connected_clients.push(username);
            }
            Event::Disconnected { username } => todo!(),
            Event::CreateRoom { roomId } => todo!(),
            Event::JoinRoom { roomId } => todo!(),
            Event::SetUsername { username } => {
                println!("The user is trying to change their name to: {}", username);
            }
        }
    }
}

struct Message {
    sender: String,
    contents: String,
}

pub struct Server {
    pub host: String,
    address: String,
    port: String,
    listener: TcpListener,
    pub connected_clients: u64,
    pub clients: Arc<Mutex<Vec<String>>>,
}

impl Server {
    pub fn new(address: String, port: String) -> Result<Server, Box<dyn Error>> {
        let mut host = address.clone();

        host.push(':');
        host.push_str(&port);

        let listener = TcpListener::bind(&host).unwrap();

        let clients = Arc::new(Mutex::new(Vec::<String>::new()));
        Ok(Server {
            host,
            address,
            port,
            listener,
            clients,
            connected_clients: 0,
        })
    }

    pub fn connection_handler(
        mut connected_clients: Arc<Mutex<Vec<String>>>,
        mut stream: TcpStream,
        rx: Receiver<String>,
        tx: Sender<String>,
    ) {
        let mut data = [0 as u8; 50]; // using 50 byte buffer

        while match stream.read(&mut data) {
            Ok(size) => {
                let data = data[..size].to_vec();
                let message = String::from_utf8(data).unwrap();

                rx.try_recv().map(|thread_message| {
                    println!("Recieved a message! {}", thread_message);
                });

                if message.len() > 0 {
                    println!("Recieved a message: {:?}", message);

                    // Begin event handle

                    if message.starts_with(&Commands::SetUsername.value()) {
                        let username = message.replace("!username", "").trim_start().to_string();

                        // Acquire a lock on the mutex guard
                        let mut connected_clients = connected_clients.lock().unwrap();

                        // Push the new clients onto the list of connected clients
                        connected_clients.push(username);

                        let socket_message = SocketMessage {
                            r#type: String::from("array"),
                            data: DataKind::StringVector(connected_clients.clone()),
                        };

                        let json = serde_json::to_string(&socket_message).unwrap();

                        let y = serde_json::to_vec(&socket_message).unwrap();

                        stream.write(&y).unwrap();
                    }
                }

                true
            }
            Err(_) => {
                stream.shutdown(Shutdown::Both).unwrap();
                true
            }
        } {}
    }

    pub fn start_listening(&mut self) {
        let (s1, r1) = bounded::<String>(0);

        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.connected_clients += 1;
                    println!(
                        "A new client has connected! There are now {} connected clinets",
                        self.connected_clients
                    );

                    let clients_arc = Arc::clone(&self.clients);

                    let (s2, r2) = (s1.clone(), r1.clone());
                    let joinHandle = thread::spawn(move || {
                        Self::connection_handler(clients_arc, stream, r2, s2);
                    });

                    s1.send(String::from("Hello!")).unwrap();
                }
                Err(e) => {
                    panic!("Uh oh!")
                }
            }
        }
    }
}
