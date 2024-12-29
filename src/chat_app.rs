use std::net::{TcpListener};




mod server {
    // server side logic will go here.
}

mod client {
    // client side logic will reside here.
}

pub fn start() {
    let port = "127.0.0.1:8080";
    let listener = TcpListener::bind(port).unwrap();
    println!("Server listening on {}", port)
}
