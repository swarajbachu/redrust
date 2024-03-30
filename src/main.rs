mod commands;

use std::{collections::HashMap, sync::Arc, sync::Mutex};

use commands::Value;
use tokio::net::{TcpListener, TcpStream};
use anyhow::Result;

type Database = Arc<Mutex<HashMap<String, String>>>;

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    let db: Database = std::sync::Arc::new(std::sync::Mutex::new(HashMap::new()));

    
    loop {
        let stream = listener.accept().await;
        let db = db.clone();

        match stream {
            Ok((_stream,_)) => {
                println!("accepted new connection");
                tokio::spawn(async move {
                handle_incoming_connection(_stream,db).await;
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}


async fn handle_incoming_connection(stream: TcpStream,db: Database) {

    let mut handler = commands::CommandHandler::new(stream);
    println!("Starting read loop");
    loop {
        let value = handler.read_value().await.unwrap();
        println!("Got value {:?}", value);
        
        let response = if let Some(v) = value {
            let (command, args) = extract_command(v).unwrap();
            match command.as_str() {
                "ping" => Value::SimpleString("PONG".to_string()),
                "echo" => args.first().unwrap().clone(),
                "set" => {
                    let key = unpack_bulk_str(args.get(0).unwrap().clone()).unwrap();
                    let value = unpack_bulk_str(args.get(1).unwrap().clone()).unwrap();
                    handle_set(db.clone(), key, value);
                    Value::SimpleString("OK".to_string())
                },
                "get" => {
                    let key = unpack_bulk_str(args.get(0).unwrap().clone()).unwrap();
                    match handle_get(db.clone(), key) {
                        Some(value) => Value::BulkString(value),
                        None => Value::NullBulkString,
                    }
                },
                c => panic!("Cannot handle command {}", c),
            }
        } else {
            break;
        };
        println!("Sending value {:?}", response);
        handler.write_value(response).await.unwrap();
    }
}

fn handle_set(db:Database, key: String, value: String) {
    let mut db = db.lock().unwrap();
    db.insert(key, value);
}
fn handle_get(db:Database, key: String) -> Option<String> {
    let db = db.lock().unwrap();
    db.get(&key).cloned()
}


fn extract_command(value: Value) -> Result<(String, Vec<Value>)> {
    match value {
        Value::Array(a) => {
            Ok((
                unpack_bulk_str(a.first().unwrap().clone())?,
                a.into_iter().skip(1).collect(),
            ))
        },
        _ => Err(anyhow::anyhow!("Unexpected command format")),
    }
}

fn unpack_bulk_str(value: Value) -> Result<String> {
    match value {
        Value::BulkString(s) => Ok(s),
        _ => Err(anyhow::anyhow!("Expected command to be a bulk string"))
    }
}