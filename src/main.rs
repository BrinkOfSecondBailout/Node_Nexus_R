#![allow(unused)]

mod server;
mod commands;
use std::net::Ipv4Addr;
use crate::server::server_init;

const MAX_USERNAME_LEN:usize = 32;

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
    server_init();
}
