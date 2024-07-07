use crate::connection::{handle_connection, listen_at_port, write_connection};
use crate::parse_headers::parse_headers;
use crate::parse_headers::RequestType;
use crate::parse_path::parse_path;
use crate::parse_url::parse_url_param;
use std::collections::HashMap;

pub struct Request {
    headers: HashMap<String, String>,
    body: HashMap<String, String>,
    url_params: HashMap<String, String>,
}

pub struct Endpoint<'a> {
    path: &'a str,
    request: RequestType,
    mapper: fn(Request) -> Option<String>,
}

pub struct App<'a> {
    endpoints: Vec<Endpoint<'a>>,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        App { endpoints: vec![] }
    }

    pub fn add_endpoint(
        &mut self,
        path: &'a str,
        request: RequestType,
        mapper: fn(Request) -> Option<String>,
    ) {
        let endpoint = Endpoint {
            path,
            request,
            mapper,
        };
        self.endpoints.push(endpoint);
    }

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

pub fn run(app: App, port: u16, verbose: bool) {
    let listener = listen_at_port(port);
    if verbose {
        println!("Listening at port {:?}", port);
    };
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
                        url_params: url_params,
                    };

                    match (endpoint.mapper)(request) {
                        Some(response) => {
                            write_connection(&mut stream, response.as_bytes());
                        }
                        None => {}
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
