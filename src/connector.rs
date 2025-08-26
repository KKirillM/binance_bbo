use std::error::Error;
use tungstenite::{connect, Message, WebSocket};
use tungstenite::stream::MaybeTlsStream;
use url::Url;

pub struct ConnectionManager {
    socket: WebSocket<MaybeTlsStream<std::net::TcpStream>>,
}

impl ConnectionManager {
    pub fn connect(addr: &str, port: u16) -> Result<Self, Box<dyn Error>> {
        let url = Url::parse(&format!("wss://{}:{}/ws", addr, port))?;
        let (socket, _resp) = connect(url)?;
        Ok(Self { socket })
    }

    pub fn send_message(&mut self, msg: &str) -> Result<(), Box<dyn Error>> {
        self.socket.send(Message::Text(msg.to_string()))?;
        Ok(())
    }

    pub fn receive_message(&mut self) -> Result<Message, Box<dyn Error>> {
        Ok(self.socket.read()?)
    }
}
