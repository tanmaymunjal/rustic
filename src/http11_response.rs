use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct Response<'a> {
    pub status_code: u16,
    pub reason: &'a str,
    pub response_body: Option<&'a str>,
    pub headers: HashMap<String, String>,
}

fn get_current_utc_date() -> String {
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

pub fn write_status_header(status_code: u16, reason: &str) -> String {
    format!("HTTP/1.1 {} {} \r\n", status_code, reason)
}

pub fn write_header(headers: &mut HashMap<String, String>) -> String {
    headers.insert("Date".to_string(), get_current_utc_date());

    let mut header_string = String::new();
    for (key, value) in headers {
        header_string.push_str(&format!("{}: {}\r\n", key, value));
    }
    header_string.push_str("\r\n");
    header_string
}

pub fn write_connection(stream: &mut TcpStream, mut response: Response) {
    let status_line = write_status_header(response.status_code, response.reason);
    let headers_string = write_header(&mut response.headers);
    let mut full_response = status_line;
    full_response.push_str(&headers_string);

    if let Some(response_body) = response.response_body {
        full_response.push_str(response_body);
    }
    stream.write_all(full_response.as_bytes()).unwrap();
}

pub fn hashmap_to_json<K: std::fmt::Display, V: std::fmt::Display>(map: &HashMap<K, V>) -> String {
    let mut json_string = String::from("{");

    for (i, (key, value)) in map.iter().enumerate() {
        json_string.push_str(&format!("\"{}\": \"{}\"", key, value));
        if i < map.len() - 1 {
            json_string.push_str(", ");
        }
    }

    json_string.push_str("}");

    json_string
}

#[cfg(test)]
mod test_http_response_functions {
    use super::*;

    #[test]
    fn test_write_status_header() {
        let status_line = write_status_header(200, "OK");
        assert_eq!(status_line, "HTTP/1.1 200 OK \r\n");
    }

    #[test]
    fn test_write_header() {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/plain".to_string());
        headers.insert("Content-Length".to_string(), "123".to_string());

        let header_string = write_header(&mut headers);
        assert!(header_string.contains("Content-Type: text/plain\r\n"));
        assert!(header_string.contains("Content-Length: 123\r\n"));
    }

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
