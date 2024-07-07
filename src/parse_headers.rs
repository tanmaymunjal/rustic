use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum RequestType {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    UPDATE,
    Other(String),
}

#[derive(Debug, PartialEq)]
pub enum HttpType {
    OnePointOne,
    NotSupported(String),
}

type ParsedHeaders = Result<
    (
        RequestType,
        HttpType,
        HashMap<String, String>,
        Option<String>,
    ),
    String,
>;

pub fn parse_headers(headers: Vec<String>) -> ParsedHeaders {
    if headers.is_empty() {
        return Err("No headers to parse.".to_string());
    }

    let split_request: Vec<&str> = headers[0].split_whitespace().collect();

    let request_type = match split_request.first() {
        Some(&"GET") => RequestType::GET,
        Some(&"POST") => RequestType::POST,
        Some(&"PUT") => RequestType::PUT,
        Some(&"DELETE") => RequestType::DELETE,
        Some(&"PATCH") => RequestType::PATCH,
        Some(&"UPDATE") => RequestType::UPDATE,
        Some(other) => RequestType::Other(other.to_string()),
        None => return Err("Invalid request line.".to_string()),
    };

    let http_type = match split_request.get(2) {
        Some(&"HTTP/1.1") => HttpType::OnePointOne,
        Some(other) => HttpType::NotSupported(other.to_string()),
        None => return Err("Invalid HTTP version.".to_string()),
    };

    let mut header_map = HashMap::new();
    let mut host_name = None;

    // Parse headers
    for header in headers.iter().skip(1) {
        let parts: Vec<&str> = header.splitn(2, ": ").collect();
        if parts.len() == 2 {
            let key = parts[0].trim();
            let value = parts[1].trim().to_string();
            header_map.insert(key.to_string(), value.clone());

            // Check if the header is "Host" and extract its value
            if key.eq_ignore_ascii_case("Host") {
                host_name = Some(value);
            }
        }
    }

    // Construct the URL
    let url =
        host_name.and_then(|host| split_request.get(1).map(|path| format!("{}{}", host, path)));

    Ok((request_type, http_type, header_map, url))
}

#[cfg(test)]
mod parse_headers_test {
    use super::*;
    #[test]
    pub fn test_parse_headers() {
        let headers = vec![
            "GET /test HTTP/1.1".to_string(),
            "Host: localhost:8002".to_string(),
            "User-Agent: curl/8.2.1".to_string(),
            "Accept: */*".to_string(),
        ];

        let (request_type, http_type, headers_map, url) = parse_headers(headers).unwrap();

        assert_eq!(request_type, RequestType::GET);
        assert_eq!(http_type, HttpType::OnePointOne);
        assert_eq!(headers_map.get("Host"), Some(&"localhost:8002".to_string()));
        assert_eq!(
            headers_map.get("User-Agent"),
            Some(&"curl/8.2.1".to_string())
        );
        assert_eq!(url, Some("localhost:8002/test".to_string()));
    }
}
