use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
/// Represents an HTTP response sent by the server.
pub struct Response<'a> {
    pub status_code: u16,
    pub reason: &'a str,
    pub response_body: Option<&'a str>,
    pub headers: HashMap<String, String>,
}

/// Retrieves the current date and time in UTC format as a string.
///
/// This function uses the system's current time and formats it
/// according to the HTTP-date specification.
///
/// # Returns
///
/// * `String` - The current date and time in UTC format.
///
/// # Examples
///
/// ```
/// use rustic::http11_response::get_current_utc_date;
/// let date = get_current_utc_date();
/// println!("{}", date); // Example: "Sun, 07 Jul 2024 12:00:00 GMT"
/// ```
pub fn get_current_utc_date() -> String {
    let now = SystemTime::now();
    let seconds_since_epoch = now.duration_since(UNIX_EPOCH).unwrap().as_secs();
    let formatted_date = chrono::DateTime::<chrono::Utc>::from_utc(
        chrono::NaiveDateTime::from_timestamp(seconds_since_epoch as i64, 0),
        chrono::Utc,
    );
    formatted_date
        .format("%a, %d %b %Y %H:%M:%S GMT")
        .to_string()
}

/// Constructs the status line for an HTTP response.
///
/// This function formats the HTTP status line based on the provided status code and reason phrase.
///
/// # Arguments
///
/// * `status_code` - The HTTP status code.
/// * `reason` - The reason phrase associated with the status code.
///
/// # Returns
///
/// * `String` - The formatted HTTP status line.
///
/// # Examples
///
/// ```
/// use rustic::http11_response::write_status_header;
/// let status_line = write_status_header(200, "OK");
/// assert_eq!(status_line, "HTTP/1.1 200 OK \r\n");
/// ```
pub fn write_status_header(status_code: u16, reason: &str) -> String {
    format!("HTTP/1.1 {} {} \r\n", status_code, reason)
}

/// Constructs the HTTP headers from a given `HashMap` and includes an optional body and Content-Length.
///
/// This function formats the HTTP headers, adds the current date if not already present,
/// and includes the Content-Length header based on the length of the provided body if present.
/// If `body` is `None`, it adds Content-Length as 0.
///
/// # Arguments
///
/// * `headers` - A mutable reference to a `HashMap` containing the headers.
/// * `body` - An optional body content as a string slice (`Option<&str>`).
///
/// # Returns
///
/// * `String` - The formatted HTTP headers.
///
/// # Examples
///
/// ```
/// use rustic::http11_response::write_header;
/// use std::collections::HashMap;
/// let mut headers = HashMap::new();
/// headers.insert("Content-Type".to_string(), "text/plain".to_string());
/// let body = Some("Hello, world!");
/// let headers_string = write_header(&mut headers, body);
/// println!("{}", headers_string);
/// assert!(headers_string.contains("Content-Type: text/plain\r\n"));
/// ```
pub fn write_header(headers: &mut HashMap<String, String>, body: Option<&str>) -> String {
    headers.insert("Date".to_string(), get_current_utc_date());
    if let Some(body) = body {
        headers.insert("Content-Length".to_string(), body.len().to_string());
    } else {
        headers.insert("Content-Length".to_string(), "0".to_string());
    }

    let mut header_string = String::new();
    for (key, value) in headers {
        header_string.push_str(&format!("{}: {}\r\n", key, value));
    }
    header_string.push_str("\r\n");
    header_string
}

/// Writes an HTTP response to the given TCP stream.
///
/// This function writes the status line, headers, and optionally the response body to the TCP stream.
///
/// # Arguments
///
/// * `stream` - A mutable reference to the `TcpStream`.
/// * `response` - The HTTP response to be written.
///
/// # Examples
///
/// ```no_run
/// use rustic::http11_response::{write_connection,Response};
/// use std::net::TcpStream;
/// use std::collections::HashMap;
/// let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
/// let response = Response {
///     status_code: 200,
///     reason: "OK",
///     response_body: Some("Hello, world!"),
///     headers: HashMap::new(),
/// };
/// write_connection(&mut stream, response);
/// ```
pub fn write_connection(stream: &mut TcpStream, mut response: Response) {
    let status_line = write_status_header(response.status_code, response.reason);
    let headers_string = write_header(&mut response.headers, response.response_body);
    let mut full_response = status_line;
    full_response.push_str(&headers_string);

    if let Some(response_body) = response.response_body {
        full_response.push_str(response_body);
    }
    stream.write_all(full_response.as_bytes()).unwrap();
}

/// Converts a `HashMap` to a JSON string.
///
/// This function formats a `HashMap` as a JSON string.
///
/// # Arguments
///
/// * `map` - A reference to the `HashMap` to be converted.
///
/// # Returns
///
/// * `String` - The formatted JSON string.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use rustic::connection::listen_at_port;
/// use rustic::http11_response::hashmap_to_json;
/// let mut map = HashMap::new();
/// map.insert("key1", "value1");
/// let json_data = hashmap_to_json(&map);
/// assert_eq!(json_data, "{\"key1\": \"value1\"}");
/// ```
pub fn hashmap_to_json<K: std::fmt::Display, V: std::fmt::Display>(map: &HashMap<K, V>) -> String {
    let mut json_string = String::from("{");

    for (i, (key, value)) in map.iter().enumerate() {
        json_string.push_str(&format!("\"{}\": \"{}\"", key, value));
        if i < map.len() - 1 {
            json_string.push_str(", ");
        }
    }

    json_string.push('}');

    json_string
}

#[cfg(test)]
mod test_http_response_functions {
    use super::*;

    /// Tests the `write_status_header` function.
    #[test]
    fn test_write_status_header() {
        let status_line = write_status_header(200, "OK");
        assert_eq!(status_line, "HTTP/1.1 200 OK \r\n");
    }

    /// Tests the `write_header` function.
    #[test]
    fn test_write_header() {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/plain".to_string());

        let header_string = write_header(&mut headers, None);
        assert!(header_string.contains("Content-Type: text/plain\r\n"));
        assert!(header_string.contains("Content-Length: 0\r\n"));
    }

    /// Tests the `hashmap_to_json` function.
    #[test]
    fn test_hashmap_to_json() {
        let mut map = HashMap::new();
        map.insert("key1", "value1");
        map.insert("key2", "value2");

        let json_data = hashmap_to_json(&map);

        assert!(
            json_data == "{\"key1\": \"value1\", \"key2\": \"value2\"}"
                || json_data == "{\"key2\": \"value2\", \"key1\": \"value1\"}"
        );
    }
}
