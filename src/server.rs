use std::net::TcpListener;


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

pub fn server_init() {
    let listener = TcpListener::bind(LOCAL_HOST).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
    }
}
