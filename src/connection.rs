use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

pub fn listen_at_port(port: u16) -> TcpListener {
    let address = format!("127.0.0.1:{}", port);
    TcpListener::bind(address).expect("Failed to bind to port")
}

pub fn handle_connection(stream: &mut TcpStream) -> Vec<String> {
    let buf_reader = BufReader::new(stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    http_request
}
