use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                handle_incoming_connection(_stream)
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_incoming_connection(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    for _ in 0..1024 {
        let bytes_read = stream.read(&mut buf).expect("Failed to read stream");
        if bytes_read == 0 {
            break;
        }
        print!["{}", String::from_utf8(buf.to_vec()).unwrap()];
        if true | (String::from_utf8(buf.to_vec()).expect("invalid bytes") == String::from("ping"))
        {
            stream
                .write_all("+PONG\r\n".as_bytes())
                .expect("Failed to write");

        }
    }
}
