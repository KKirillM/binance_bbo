use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

// Глобальная переменная для управления идентификаторами
static MESSAGE_ID_COUNTER: AtomicU64 = AtomicU64::new(1); // или unsafe

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestMessage {
    pub method: String,
    pub params: Vec<String>,
    pub id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseMessage {
    pub result: Option<serde_json::Value>,
    pub id: u64,
}

impl RequestMessage {
    fn new(method: String, params: Vec<String>) -> Self {
        RequestMessage {
            method,
            params,
            id: MESSAGE_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
        }
    }

    pub fn new_subscribe(params: Vec<String>) -> Self {
        RequestMessage::new("SUBSCRIBE".to_string(), params)
    }
}

impl ResponseMessage {
    // pub fn new_empty(id: u64) -> Self {
    //     ResponseMessage {
    //         result: None,
    //         id,
    //     }
    // }
}
