
use std::{collections::HashMap, sync::Arc, sync::Mutex};

use tokio::time::Instant;

use crate::response::Value;


pub type Database = Arc<Mutex<HashMap<String, RedisItem>>>;

pub struct RedisItem {
    value: String,
    created_at: Instant,
    expiration: Option<i64>,
}

pub struct CommandHandler {
    db:Database
}


impl CommandHandler {
    pub fn new(db: Database) -> Self {
        CommandHandler {
           db
        }
    }
    pub fn handle_set(&self, key: String, value: String, expiry: Option<i64>) {
        let mut db = self.db.lock().unwrap();
        let redis_item = if let Some(_exp_time) = expiry {
            RedisItem {
                value,
                created_at: Instant::now(),
                expiration: Some(_exp_time),
            }
        } else {
            RedisItem {
                value,
                created_at: Instant::now(),
                expiration: None,
            }
        };
        db.insert(key, redis_item);

    }
   pub fn handle_get(&self, key: String) -> Value {
        let db = self.db.lock().unwrap();
        match db.get(&key) {
            Some(value) => {
                let response = if let Some(expiration) = value.expiration {
                    let now = Instant::now();
                    if now.duration_since(value.created_at).as_millis()
                        > expiration as u128
                    {
                        Value::NullBulkString
                    } else {
                        Value::BulkString(value.value.clone())
                    }
                } else {
                    Value::BulkString(value.value.clone())
                };
                response
            },
            None => Value::NullBulkString,
        }
    }
    
}