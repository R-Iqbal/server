use std::collections::HashMap;
use std::error::Error;
use std::io::prelude::*;

use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread::{self};

use crossbeam_channel::{bounded, Receiver, Sender};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct SocketMessage {
    payload: SocketPayloadKind,
}

#[derive(Serialize, Deserialize)]
pub enum SocketPayloadKind {
    Connected { username: String },
    SetUsername { user_id: String, username: String },
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
    pub clients: Arc<Mutex<HashMap<String, String>>>,
}

impl Server {
    pub fn new(address: String, port: String) -> Result<Server, Box<dyn Error>> {
        let mut host = address.clone();

        host.push(':');
        host.push_str(&port);

        let listener = TcpListener::bind(&host).unwrap();

        let clients = Arc::new(Mutex::new(HashMap::<String, String>::new()));
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
        mut connected_clients: Arc<Mutex<HashMap<String, String>>>,
        mut stream: TcpStream,
        rx: Receiver<String>,
        tx: Sender<String>,
    ) {
        // Create a 200 byte buffer to read in the data from the client
        let mut data = [0 as u8; 200];

        // While there is new data to be read from the stream
        while if let Ok(size) = stream.read(&mut data) {
            // Check if we have recieved any messages from the main thread
            // which need to be passed onto this client
            rx.try_recv().map(|thread_message| {
                println!("Recieved a message! {}", thread_message);
            });

            // Read in the data from the socket
            let data = data[..size].to_vec();
            let message = String::from_utf8(data).unwrap();

            println!("Recieved a message: {}", message);

            // Parse the message into a SocketMessage so we can determine
            // how to handle the execution
            let message: SocketMessage = serde_json::from_str(&message).unwrap();

            // Examine the type of SocketPayload to determine how we should handle
            // the request
            match message.payload {
                SocketPayloadKind::Connected { username } => todo!(),
                SocketPayloadKind::SetUsername { user_id, username } => {
                    // Attempt to acquire the lock on the mutex
                    let mut connected_clients = connected_clients.lock().unwrap();
                    connected_clients.insert(user_id, username);

                    println!("The list of connected users are: {:?}", connected_clients);
                }
                SocketPayloadKind::Disconnected { username } => todo!(),
                SocketPayloadKind::CreateRoom { roomId } => todo!(),
                SocketPayloadKind::JoinRoom { roomId } => todo!(),
            }

            true
        } else {
            stream.shutdown(Shutdown::Both).unwrap();
            true
        } {}
    }

    pub fn start_listening(&mut self) {
        let (s1, r1) = bounded::<String>(0);

        // For each of the incoming connections create a connection handler
        // to deal with any requests
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();

            self.connected_clients += 1;
            println!(
                "A new client has connected! There are now {} connected clinets",
                self.connected_clients
            );

            let clients_arc = Arc::clone(&self.clients);

            let (s2, r2) = (s1.clone(), r1.clone());
            thread::spawn(move || {
                Self::connection_handler(clients_arc, stream, r2, s2);
            });
        }
    }
}
