use std::error::Error;
use std::thread;
use std::sync::mpsc::{self, TryRecvError};
use websocket::OwnedMessage;

use crate::config::Config;
use crate::messages::RequestMessage;
use crate::connector::ConnectionManager;

enum Command {
    SendMessage(String),
    Terminate,
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let mut client = ConnectionManager::new(config.get_addr(), config.get_port());

    println!("Connecting to {}:{}", config.get_addr(), config.get_port());
    if let Err(e) = client.connect() {
        return Err(format!("Failed to connect: {}", e).into());
    }
    println!("Connected");

    let (to_ws_tx, to_ws_rx) = mpsc::channel();
    let (from_ws_tx, from_ws_rx) = mpsc::channel();

    // Запускаем поток для работы с сообщениями Websocket
    let websocket_thread = thread::spawn(move || {
        loop {
            match to_ws_rx.try_recv() {
                Ok(Command::SendMessage(msg)) => {
                    println!("Sending message: {}", msg);
                    if let Err(e) = client.send_message(&msg) {
                        println!("Error sending message: {}", e);
                        continue;
                    }
                },
                
                Ok(Command::Terminate) => {
                    println!("Terminating websocket thread");
                    break;
                }

                Err(TryRecvError::Empty) => {},

                Err(TryRecvError::Disconnected) => {
                    println!("to_ws_rx channel has been disconnected");
                    break;
                }
            }

            match client.receive_message() {
                Ok(msg) => from_ws_tx.send(msg).unwrap(),
                Err(e) => {
                    eprintln!("Error receiving message: {}", e);
                    break;
                }
            }
        }

        drop(from_ws_tx);
    });

    // Отправляем сообщение о подписке
    let mut params: Vec<String> = config.get_currencies_collection();
    params.iter_mut().for_each(|s| s.push_str("@bookTicker"));

    let subscribe_message = RequestMessage::new_subscribe(params);
    let subscribe_message_str = serde_json::to_string(&subscribe_message)?;
    to_ws_tx.send(Command::SendMessage(subscribe_message_str))?;

    // обрабатываем входящие сообщения
    for msg in from_ws_rx {
        process_message(msg);
    }

    println!("im here");
    // Отправляем команду на завершение потока получения
    to_ws_tx.send(Command::Terminate)?;
    // Ожидаем завершения потока
    websocket_thread.join().unwrap();

    Ok(())
}

fn process_message(msg: OwnedMessage) {
    match msg {
        OwnedMessage::Text(msg) => {
            println!("Received text message: {}", msg);
            // обработать сообщение
            // ...
        },

        OwnedMessage::Binary(data) => {
            println!("Received binary data: {:?}", data);
        },

        OwnedMessage::Ping(ping) => {
            println!("Received ping: {:?}", ping);
            // отправить Pong в ответ
            // ...
        },

        OwnedMessage::Pong(pong) => {
            println!("Received pong: {:?}", pong);
        },

        OwnedMessage::Close(data) => {
            println!("Received close message: {:?}", data);
            // закрыть соединение
            // ...
        },
    }
}