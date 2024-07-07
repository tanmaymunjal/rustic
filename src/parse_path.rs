pub fn parse_path(url: &str) -> Option<&str> {
    url.split_once("://")
        .and_then(|(_, after_scheme)| after_scheme.split_once('/'))
        .map(|(_, path)| path.trim_matches('/'))
}

#[cfg(test)]
mod test_parse_path {
    use super::*;

    #[test]
    fn test_parse_path_simple() {
        assert_eq!(
            parse_path("https://example.com/path/to/resource/"),
            Some("path/to/resource")
        );
    }

    #[test]
    fn test_parse_path_no_path() {
        assert_eq!(parse_path("https://example.com"), None);
    }

    #[test]
    fn test_parse_path_no_scheme() {
        assert_eq!(parse_path("example.com/path/to/resource"), None);
    }
}
