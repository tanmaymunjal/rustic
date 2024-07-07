use crate::parse_headers::RequestType;
use std::collections::HashMap;
use std::net::TcpListener;

pub struct Request {
    headers: HashMap<String, String>,
    body: HashMap<String, String>,
    url_params: HashMap<String, String>,
}

pub struct Endpoint<'a> {
    path: &'a str,
    request: RequestType,
    map: fn(Request),
}

pub struct App<'a> {
    listener: TcpListener,
    endpoints: Vec<Endpoint<'a>>,
}

impl<'a> App<'a> {
    pub fn new(listener: TcpListener) -> Self {
        App {
            listener,
            endpoints: vec![],
        }
    }

    pub fn add_endpoint(&mut self, path: &'a str, request: RequestType, map: fn(Request)) {
        let endpoint = Endpoint { path, request, map };
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

