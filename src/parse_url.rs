use std::collections::HashMap;

/// Parses URL parameters from a given URL string and returns them as a `HashMap<String, String>`.
///
/// # Arguments
///
/// * `url` - A string slice representing the URL containing parameters.
///
/// # Returns
///
/// A `HashMap` where each key-value pair corresponds to a parsed parameter from the URL.
///
/// # Examples
///
/// ```
/// use rustic::parse_url::parse_url_param;
/// use std::collections::HashMap;
/// let url = "http://example.com/?key=value";
/// let result = parse_url_param(url);
/// let mut expected = HashMap::new();
/// expected.insert("key".to_string(), "value".to_string());
/// assert_eq!(result, expected);
/// ```
pub fn parse_url_param(url: &str) -> HashMap<String, String> {
    // Split the URL at the '?' character to isolate parameters part.
    url.split_once('?')
        .map(|(_, params)| params) // Take the parameters part or an empty string if no '?'
        .unwrap_or("")
        .split('&') // Split parameters into key-value pairs
        .filter_map(|s| s.split_once('=')) // Filter out invalid pairs and split into (key, value)
        .map(|(k, v)| (k.to_string(), v.to_string())) // Convert (key, value) pairs into (String, String)
        .collect() // Collect into a HashMap<String, String>
}

#[cfg(test)]
mod test_parse_url_param {
    use super::*;

    #[test]
    fn test_single_param() {
        let url = "http://example.com/?key=value";
        let result1 = parse_url_param(url);
        let mut expected = HashMap::new();
        expected.insert("key".to_string(), "value".to_string());
        assert_eq!(result1, expected);
    }

    #[test]
    fn test_multiple_param() {
        let url = "http://example.com/?key1=value1&key2=value2&key3=value3";
        let result = parse_url_param(url);
        let mut expected = HashMap::new();
        expected.insert("key1".to_string(), "value1".to_string());
        expected.insert("key2".to_string(), "value2".to_string());
        expected.insert("key3".to_string(), "value3".to_string());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_no_param() {
        let url = "http://example.com/";
        let result = parse_url_param(url);
        let expected = HashMap::new();
        assert_eq!(result, expected);
    }
}
