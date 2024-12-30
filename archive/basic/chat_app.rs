mod server {
    use std::{net::{TcpListener, TcpStream}, thread, io::{Read, Write}};
    // server side logic will go here.
    pub fn start() {
        let port = "127.0.0.1:8080";
        let listener = TcpListener::bind(port).unwrap();
        println!("Server listening on {}", port);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(move || handle_client(stream));
                }
                Err(e) => {
                    println!("Error accepting connection: {}", e);
                }
                
            }
        }
    }

    fn handle_client(mut stream: TcpStream) {
        let mut buffer = [0; 1024];

        loop {
            match stream.read(&mut buffer) {
                Ok(size) => {
                    if size == 0 {
                        println!("Client Disconnected");
                        break;
                    }

                    stream.write(&buffer[..size]).unwrap();
                }
                Err(e) => {
                    println!("Error reading from client: {}", e);
                    break;
                }
            }
        }

    }
}

mod client {
    // client side logic will reside here.
}

pub fn start() {
    server::start();
    
}
