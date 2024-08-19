use websocket::{ClientBuilder, Message, OwnedMessage, sync::Client};
use std::net::TcpStream;
use std::error::Error;
use websocket::stream::sync::TlsStream;

pub struct ConnectionManager {
    url: String,
    client: Option<Client<TlsStream<TcpStream>>>, // Используем TlsStream для безопасного подключения
}

impl ConnectionManager {
    pub fn new(addr: &str, port: u16) -> Self {
        let url = format!("wss://{}:{}/ws", addr, port); // Используем безопасное соединение wss
        Self { url, client: None }
    }

    // Метод для подключения к WebSocket
    pub fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        let client = ClientBuilder::new(&self.url)?
            .connect_secure(None)?;

        self.client = Some(client); // Client<> не реализует типаж Copy поэтому здесь move
        Ok(())
    }

    pub fn send_message(&mut self, msg: &str) -> Result<(), Box<dyn Error>> {
        if let Some(client) = self.client.as_mut() {
            let result = client.send_message(&Message::text(msg));
            match result {
                Ok(()) => Ok(()),
                Err(e) => Err(e.into()),
            }
        } else {
            return Err("Client is not connected".into());
        }        
    }

    pub fn receive_message(&mut self) -> Result<OwnedMessage, Box<dyn Error>> {
        if self.client.is_none() {
            return Err("Client is not connected".into()); 
        }
        
        let client = self.client.as_mut().unwrap();
        match client.recv_message() {
            Ok(message) => Ok(message),
            Err(e) => Err(e.into()),
        }
    }
}
