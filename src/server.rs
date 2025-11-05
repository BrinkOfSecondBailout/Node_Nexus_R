use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;


const MAX_CONNECTIONS: i32 = 10;
const METHOD_LENGTH: i32 = 8;
const URL_LENGTH: i32 = 128;
const LOCAL_HOST: &str = "127.0.0.1:7878";

struct HttpRequest {
    method: String,
    url: String,
}

impl HttpRequest {
    pub fn new(method: String, url: String) -> Self {
        HttpRequest {
            method,
            url,
        }
    }
    
}

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!(
        "Request: {}",
        String::from_utf8_lossy(&buffer[..])
    )
}

pub fn server_init() {
    let listener = TcpListener::bind(LOCAL_HOST).unwrap();
    println!("Server running and listening!");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}
