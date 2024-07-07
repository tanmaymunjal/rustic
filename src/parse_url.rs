use std::collections::HashMap;

pub fn parse_url_param(url: &str) -> HashMap<String, String> {
    url.split_once('?')
        .map(|(_, params)| params)
        .unwrap_or("")
        .split('&')
        .filter_map(|s| s.split_once('='))
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
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
