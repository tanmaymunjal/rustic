use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum RequestType {
    GET,
    HEAD,
    POST,
    PUT,
    PATCH,
    UPDATE,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
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

/// Parses a vector of HTTP header strings into a structured format.
///
/// This function processes a list of HTTP headers and extracts the request type, HTTP version,
/// headers, and constructs the URL if the "Host" header is present. It returns a tuple containing
/// the request type, HTTP version, headers as a `HashMap`, and an optional parsed URL.
///
/// # Arguments
///
/// * `headers` - A vector of strings where each string represents a single HTTP header.
///
/// # Returns
///
/// * `ParsedHeaders` - A result containing a tuple with the request type, HTTP version, headers map, and optional URL,
///   or an error message if parsing fails.
///
/// # Errors
///
/// This function returns an error if the headers are empty, if the request line is invalid, or if the HTTP version is invalid.
///
/// # Examples
///
/// ```
/// use rustic::parse_headers::{parse_headers,RequestType,HttpType};
/// let headers = vec![
///     "GET /test HTTP/1.1".to_string(),
///     "Host: localhost:8002".to_string(),
///     "User-Agent: curl/8.2.1".to_string(),
///     "Accept: */*".to_string(),
/// ];
///
/// let result = parse_headers(headers).unwrap();
/// assert_eq!(result.0, RequestType::GET);
/// assert_eq!(result.1, HttpType::OnePointOne);
/// assert_eq!(result.2.get("Host"), Some(&"localhost:8002".to_string()));
/// assert_eq!(result.3, Some("/test".to_string()));
/// ```
pub fn parse_headers(headers: Vec<String>) -> ParsedHeaders {
    if headers.is_empty() {
        return Err("No headers to parse.".to_string());
    }

    let split_request: Vec<&str> = headers[0].split_whitespace().collect();

    let request_type = match split_request.first() {
        Some(&"GET") => RequestType::GET,
        Some(&"HEAD") => RequestType::HEAD,
        Some(&"POST") => RequestType::POST,
        Some(&"PUT") => RequestType::PUT,
        Some(&"DELETE") => RequestType::DELETE,
        Some(&"PATCH") => RequestType::PATCH,
        Some(&"UPDATE") => RequestType::UPDATE,
        Some(&"CONNECT") => RequestType::CONNECT,
        Some(&"OPTIONS") => RequestType::OPTIONS,
        Some(&"TRACE") => RequestType::TRACE,
        Some(option) => return Err(format!("Invalid request type: {option}")),
        None => return Err("Invalid request line.".to_string()),
    };

    let http_type = match split_request.get(2) {
        Some(&"HTTP/1.1") => HttpType::OnePointOne,
        Some(other) => HttpType::NotSupported(other.to_string()),
        None => return Err("Invalid HTTP version.".to_string()),
    };

    let mut header_map = HashMap::new();

    // Parse headers
    for header in headers.iter().skip(1) {
        let parts: Vec<&str> = header.splitn(2, ": ").collect();
        if parts.len() == 2 {
            let key = parts[0].trim();
            let value = parts[1].trim().to_string();
            header_map.insert(key.to_string(), value.clone());
        }
    }

    // Construct the URL
    let url = split_request.get(1).map(|&path| path.to_string());

    Ok((request_type, http_type, header_map, url))
}

#[cfg(test)]
mod parse_headers_test {
    use super::*;

    /// Tests the `parse_headers` function with a valid set of HTTP headers.
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
        assert_eq!(url, Some("/test".to_string()));
    }
}
