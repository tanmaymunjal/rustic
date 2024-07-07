use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

pub fn listen_at_port(port: u16) -> TcpListener {
    let address = format!("127.0.0.1:{}", port);
    TcpListener::bind(address).expect("Failed to bind to port")
}

pub fn read_listener(listener: TcpListener, should_continue: bool) {
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
        if !should_continue {
            break;
        }
    }
}

pub fn handle_connection(mut stream: TcpStream) -> Vec<String> {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    http_request
}
