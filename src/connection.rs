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
/// ```
/// let listener = listen_at_port(8080);
/// ```
pub fn listen_at_port(port: u16) -> TcpListener {
    let address = format!("127.0.0.1:{}", port);
    TcpListener::bind(address).expect("Failed to bind to port")
}

/// Handles an incoming TCP connection, reading the HTTP request line by line.
///
/// This function reads from the given TCP stream using a buffered reader, collecting
/// each line of the HTTP request into a vector of strings. The reading stops when an
/// empty line is encountered, which signifies the end of the HTTP request headers.
///
/// # Arguments
///
/// * `stream` - A mutable reference to the `TcpStream` from which to read the HTTP request.
///
/// # Returns
///
/// * `Vec<String>` - A vector containing each line of the HTTP request.
///
/// # Examples
///
/// ```
/// let mut stream = listener.accept().unwrap().0;
/// let request_lines = handle_connection(&mut stream);
/// ```
pub fn handle_connection(stream: &mut TcpStream) -> Vec<String> {
    // Create a buffered reader from the TCP stream
    let buf_reader = BufReader::new(stream);
    // Read lines from the buffered reader, collecting them into a vector until an empty line is encountered
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        // Continue reading lines until an empty line is found
        .take_while(|line| !line.is_empty())
        // Collect the lines into a vector
        .collect();
    http_request
}
