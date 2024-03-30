mod commands;

use commands::Value;
use tokio::net::{TcpListener, TcpStream};
use anyhow::Result;

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    
    loop {
        let stream = listener.accept().await;
        match stream {
            Ok((_stream,_)) => {
                println!("accepted new connection");
                tokio::spawn(async move {
                handle_incoming_connection(_stream).await;
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

async fn handle_incoming_connection(stream: TcpStream) {

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
                c => panic!("Cannot handle command {}", c),
            }
        } else {
            break;
        };
        println!("Sending value {:?}", response);
        handler.write_value(response).await.unwrap();
    }
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