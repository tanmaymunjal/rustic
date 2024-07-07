use crate::connection::{handle_connection, listen_at_port};
use crate::http11_response::{write_connection, Response};
use crate::parse_headers::{parse_headers, RequestType};
use crate::parse_path::parse_path;
use crate::parse_url::parse_url_param;
use std::collections::HashMap;

/// Represents an HTTP request.
pub struct Request {
    pub headers: HashMap<String, String>,
    pub body: HashMap<String, String>,
    pub url_params: HashMap<String, String>,
}

/// Represents an endpoint in the application.
pub struct Endpoint<'a> {
    pub path: &'a str,
    pub request: RequestType,
    pub mapper: fn(Request) -> Option<Response<'a>>,
}

/// Represents the application with multiple endpoints.
pub struct App<'a> {
    pub endpoints: Vec<Endpoint<'a>>,
}

impl<'a> App<'a> {
    /// Creates a new instance of the application.
    pub fn new() -> Self {
        App { endpoints: vec![] }
    }

    /// Adds a new endpoint to the application.
    ///
    /// # Arguments
    ///
    /// * `path` - The path for the endpoint.
    /// * `request` - The type of HTTP request (GET, POST, etc.).
    /// * `mapper` - The function that maps a request to a response.
    pub fn add_endpoint(
        &mut self,
        path: &'a str,
        request: RequestType,
        mapper: fn(Request) -> Option<Response<'a>>,
    ) {
        let endpoint = Endpoint {
            path,
            request,
            mapper,
        };
        self.endpoints.push(endpoint);
    }

    /// Matches an endpoint based on the path and request type.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to match.
    /// * `request_type` - The type of HTTP request (GET, POST, etc.).
    ///
    /// # Returns
    ///
    /// * `Result<&Endpoint<'a>, &'static str>` - The matching endpoint or an error message.
    pub fn match_endpoint(
        &self,
        path: &str,
        request_type: RequestType,
    ) -> Result<&Endpoint<'a>, &'static str> {
        for endpoint in &self.endpoints {
            if endpoint.path == path && endpoint.request == request_type {
                return Ok(endpoint);
            }
        }
        Err("No matching endpoint found")
    }
}

/// Runs the application, listening for incoming connections and handling requests.
///
/// # Arguments
///
/// * `app` - The application instance.
/// * `port` - The port to listen on.
/// * `verbose` - Whether to print verbose output.
pub fn run(app: App, port: u16, verbose: bool) {
    let listener = listen_at_port(port);
    if verbose {
        println!("Listening at port {:?}", port);
    }
    for stream in listener.incoming() {
        let mut stream = stream.unwrap(); // Make stream mutable to use it later for writing
        let headers = handle_connection(&mut stream); // Pass stream as mutable reference
        let (request_type, _, headers_map, url) = parse_headers(headers).unwrap();

        if let Some(url) = url {
            let url_str = url.as_str();
            let url_params = parse_url_param(url_str);
            let path = parse_path(url_str).unwrap();

            match app.match_endpoint(path, request_type) {
                Ok(endpoint) => {
                    let request = Request {
                        headers: headers_map,
                        body: HashMap::new(),
                        url_params,
                    };

                    if let Some(response) = (endpoint.mapper)(request) {
                        write_connection(&mut stream, response);
                    }
                }
                Err(err) => {
                    if verbose {
                        eprintln!("Error matching endpoint: {}", err);
                    }
                }
            }
        }
    }
}
