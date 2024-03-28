use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};

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
                handle_incoming_connection(_stream)
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_incoming_connection(mut stream: TcpStream) {

    tokio::spawn(async move {
        let mut buf = [0; 512];
        // read 1024 bytes at a time
        loop {
            let read_count = stream.read(&mut buf).await.unwrap();
            if read_count == 0 {
                break;
            }
            println!("received {} bytes", read_count);
            stream.write(b"+PONG\r\n").await.unwrap();
        }
    });
    

}
