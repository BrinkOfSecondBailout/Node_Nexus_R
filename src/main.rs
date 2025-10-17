use std::net::Ipv4Addr;

mod server;
mod commands;

const MAX_USERNAME_LEN = 32;

#[derive(Debug)]
struct Client {
    socket: i32,
    ip: Ipv4Addr,
    port: u16,
    logged_in: bool,
    username: String,
}

impl Client {
    fn new(socket: i32, ip: Ipv4Addr, port: u16) -> Self {
        Client {
            socket,
            ip,
            port,
            logged_in: false,
            username: String::new(),
        }
    }
    fn username(&self) -> &str {
        &self.username
    }
    fn set_username(&mut self, name: &str) {
        self.username = name.chars().take(MAX_USERNAME_LEN).collect();
    }
}

fn main() {
    println!("Hello, world!");
}
