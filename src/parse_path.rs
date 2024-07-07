pub fn parse_path(url: &str) -> Option<&str> {
    url.find("://")
        .map_or_else(
            || url.split_once('/').map(|(_, path)| path),
            |i| url[i + 3..].split_once('/').map(|(_, path)| path),
        )
        .map(|path| path.trim_matches('/'))
        .filter(|&path| !path.is_empty())
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
        assert_eq!(
            parse_path("example.com/path/to/resource"),
            Some("path/to/resource")
        );
    }
}