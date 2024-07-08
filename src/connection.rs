use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

/// Binds a TCP listener to the specified port on the localhost.
///
/// This function creates a `TcpListener` that listens for incoming TCP connections on the
/// given port at the address `127.0.0.1`.
///
/// # Arguments
///
/// * `port` - The port number to bind the TCP listener to.
///
/// # Returns
///
/// * `TcpListener` - The bound TCP listener.
///
/// # Panics
///
/// This function will panic if it fails to bind to the specified port.
///
/// # Examples
///
/// ```no_run
/// use rustic::connection::listen_at_port;
/// let listener = listen_at_port(8080);
/// ```
pub fn listen_at_port(port: u16) -> TcpListener {
    let address = format!("127.0.0.1:{}", port);
    TcpListener::bind(address).expect("Failed to bind to port")
}

/// Handles an incoming TCP connection, reading the HTTP request headers and body.
///
/// This function reads from the given TCP stream using a buffered reader, collecting
/// the headers and body of the HTTP request separately. It first reads the headers
/// line by line until an empty line is encountered, which signifies the end of the
/// HTTP headers. Then, it reads the bytes set by content-length header as body.
///
/// # Arguments
///
/// * `stream` - A mutable reference to the `TcpStream` from which to read the HTTP request.
///
/// # Returns
///
/// * `(Vec<String>, String)` - A tuple containing:
///   - A vector of strings, each representing a line of the HTTP headers.
///   - A string containing the body of the HTTP request.
///
/// # Examples
///
/// ```no_run
/// use rustic::connection::{listen_at_port, handle_connection};
/// let listener = listen_at_port(8080);
/// let mut stream = listener.accept().unwrap().0;
/// let (headers, body) = handle_connection(&mut stream);
/// ```
pub fn handle_connection(stream: &mut TcpStream) -> (Vec<String>, String) {
    let mut buf_reader = BufReader::new(stream);

    let mut headers: Vec<String> = Vec::new();
    let mut content_length = 0;

    // Read headers and find Content-Length
    for line in buf_reader.by_ref().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        if line.to_lowercase().starts_with("content-length:") {
            content_length = line
                .split(':')
                .nth(1)
                .and_then(|len| len.trim().parse::<usize>().ok())
                .unwrap_or(0);
        }
        headers.push(line);
    }

    // Read body
    let mut body = String::with_capacity(content_length);
    buf_reader
        .take(content_length as u64)
        .read_to_string(&mut body)
        .unwrap_or(0);
    (headers, body)
}
