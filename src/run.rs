use std::error::Error;
use std::thread;
use std::sync::{Arc, Mutex};
use websocket::OwnedMessage;

use crate::config::Config;
use crate::messages::{RequestMessage};
use crate::connector::ConnectionManager;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let mut client = ConnectionManager::new(config.get_addr(), config.get_port());

    println!("Connecting to {}:{}", config.get_addr(), config.get_port());
    if let Err(e) = client.connect() {
        eprintln!("Failed to connect: {}", e);
        return Err(e);
    }
    println!("Connected");

    // Оборачиваем client в Arc<Mutex<>> для безопасного доступа из нескольких потоков
    let client = Arc::new(Mutex::new(client));

    // Клонируем клиент для использования в потоке получения сообщений
    let client_receive = Arc::clone(&client);

    // Запускаем поток для получения сообщений
    let receive_thread = thread::spawn(move || {
        let mut client = client_receive.lock().unwrap();
        loop {
            match client.receive_message() {
                Ok(msg) => process_message(msg),
                Err(e) => {
                    println!("Error receiving message: {}", e);
                    break;
                },
            }            
        }
    });

    let mut params: Vec<String> = config.get_currencies_collection();
    params.iter_mut().for_each(|s| s.push_str("@bookTicker"));

    let subscribe_message = RequestMessage::new_subscribe(params);

    let subscribe_message_str = serde_json::to_string(&subscribe_message)?;
    
    println!("Sending message: {}", subscribe_message_str);
    {
        let mut client = client.lock().unwrap();
        if let Err(e) = client.send_message(&subscribe_message_str) {
            eprintln!("Failed to send message: {}", e);
        }
    }

    // Ожидаем завершения потока получения сообщений
    if let Err(e) = receive_thread.join() {
        eprintln!("Receive thread panicked: {:?}", e);
    }

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