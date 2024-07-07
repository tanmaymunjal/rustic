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

pub fn parse_headers(
    headers: Vec<String>,
) -> Result<(RequestType, HttpType, HashMap<String, String>), String> {
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

    for header in headers.iter().skip(1) {
        let parts: Vec<&str> = header.splitn(2, ": ").collect();
        if parts.len() == 2 {
            header_map.insert(parts[0].to_string(), parts[1].to_string());
        }
    }

    Ok((request_type, http_type, header_map))
}

#[cfg(test)]
mod parse_headers_test {
    use super::*;
    #[test]
    pub fn test_parse_headers() {
        let headers = vec![
            "GET / HTTP/1.1".to_string(),
            "Host: example.com".to_string(),
            "User-Agent: Mozilla/5.0".to_string(),
        ];

        let (request_type, http_type, headers_map) = parse_headers(headers).unwrap();

        assert_eq!(request_type, RequestType::GET);
        assert_eq!(http_type, HttpType::OnePointOne);
        assert_eq!(headers_map.get("Host"), Some(&"example.com".to_string()));
        assert_eq!(
            headers_map.get("User-Agent"),
            Some(&"Mozilla/5.0".to_string())
        );
    }
}
