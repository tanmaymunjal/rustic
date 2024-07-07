/// Extracts the path from a given URL string, removing any leading and trailing slashes.
///
/// This function handles URLs with and without schemes (e.g., `https://`, `http://`, etc.).
/// If the URL does not contain a path, the function returns `None`.
///
/// # Arguments
///
/// * `url` - A string slice that holds the URL from which to extract the path.
///
/// # Returns
///
/// * `Option<&str>` - Returns `Some(&str)` containing the path if it exists and is non-empty, otherwise returns `None`.
///
/// # Examples
///
/// ```
/// use rustic::parse_path::parse_path;
/// assert_eq!(
///     parse_path("https://example.com/path/to/resource/"),
///     Some("path/to/resource")
/// );
/// assert_eq!(parse_path("https://example.com"), None);
/// assert_eq!(
///     parse_path("example.com/path/to/resource"),
///     Some("path/to/resource")
/// );
/// ```
pub fn parse_path(url: &str) -> Option<&str> {
    url.find("://")
        // Check if the URL contains a scheme (e.g., "https://")
        .map_or_else(
            // If no scheme is found, split the URL at the first '/' to get the path
            || url.split_once('/').map(|(_, path)| path),
            // If a scheme is found, skip the scheme and "://" part, then split at the first '/'
            |i| url[i + 3..].split_once('/').map(|(_, path)| path),
        )
        // Remove leading and trailing slashes from the path
        .map(|path| path.trim_matches('/'))
        // Return None if the path is empty after trimming
        .filter(|&path| !path.is_empty())
}

#[cfg(test)]
mod test_parse_path {
    use super::*;

    /// Tests the `parse_path` function with a full URL including scheme and path.
    #[test]
    fn test_parse_path_simple() {
        assert_eq!(
            parse_path("https://example.com/path/to/resource/"),
            Some("path/to/resource")
        );
    }

    /// Tests the `parse_path` function with a URL that has no path, only the scheme and host.
    #[test]
    fn test_parse_path_no_path() {
        assert_eq!(parse_path("https://example.com"), None);
    }

    /// Tests the `parse_path` function with a URL that does not include a scheme.
    #[test]
    fn test_parse_path_no_scheme() {
        assert_eq!(
            parse_path("example.com/path/to/resource"),
            Some("path/to/resource")
        );
    }
}
