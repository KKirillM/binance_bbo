use std::error::Error;
use std::sync::mpsc::{self, RecvTimeoutError};
use std::thread;
use std::time::Duration;

use log::{error, info};
use tungstenite::Message;

use crate::config::Config;
use crate::connector::ConnectionManager;
use crate::messages::RequestMessage;

enum Command {
    SendMessage(String),
    Terminate,
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let mut client = ConnectionManager::connect(config.addr(), config.port())?;
    info!("Connected to {}:{}", config.addr(), config.port());

    let (to_ws_tx, to_ws_rx) = mpsc::channel();
    let (from_ws_tx, from_ws_rx) = mpsc::channel();

    let websocket_thread = thread::spawn(move || {
        loop {
            match to_ws_rx.recv_timeout(Duration::from_millis(100)) {
                Ok(Command::SendMessage(msg)) => {
                    if let Err(e) = client.send_message(&msg) {
                        error!("Error sending message: {}", e);
                    }
                }
                Ok(Command::Terminate) => break,
                Err(RecvTimeoutError::Timeout) => {}
                Err(RecvTimeoutError::Disconnected) => break,
            }

            match client.receive_message() {
                Ok(msg) => {
                    if from_ws_tx.send(msg).is_err() {
                        break;
                    }
                }
                Err(e) => {
                    error!("Error receiving message: {}", e);
                    break;
                }
            }
        }
    });

    let params: Vec<String> = config
        .currencies()
        .iter()
        .map(|s| format!("{}@bookTicker", s))
        .collect();

    let subscribe_message = RequestMessage::new_subscribe(params);
    let subscribe_message_str = serde_json::to_string(&subscribe_message)?;
    to_ws_tx.send(Command::SendMessage(subscribe_message_str))?;

    for msg in from_ws_rx {
        process_message(msg);
    }

    let _ = to_ws_tx.send(Command::Terminate);
    let _ = websocket_thread.join();

    Ok(())
}

fn process_message(msg: Message) {
    match msg {
        Message::Text(msg) => {
            info!("Received text message: {}", msg);
        }
        Message::Binary(data) => {
            info!("Received binary data: {:?}", data);
        }
        Message::Ping(ping) => {
            info!("Received ping: {:?}", ping);
        }
        Message::Pong(pong) => {
            info!("Received pong: {:?}", pong);
        }
        Message::Close(data) => {
            info!("Received close message: {:?}", data);
        }
        Message::Frame(_) => {}
    }
}
