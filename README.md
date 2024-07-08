# Rustic: A Lightweight Multi-threaded HTTP Micro Framework in Rust

Rustic is a minimalist HTTP micro framework built from the ground up in Rust. It implements a multi-threaded http web server from the socket level, providing a simple, Flask-like interface for adding endpoints with minimal boilerplate.

## Features

- **Socket-Level Implementation**: Built from scratch for maximum control and efficiency.
- **Flask-like API**: Intuitive and easy-to-use interface for defining routes and handlers.
- **Lightweight**: Minimal dependencies for a small footprint and fast compilation.
- **Flexible**: Support for various HTTP methods and custom request handling.
- **Performance-Focused**: Leverages Rust's speed and safety for optimal server performance.

**Note**: This is a random side project. Please do not use for production systems.

## Quick Start

Add Rustic to your `Cargo.toml`:

```toml
[dependencies]
rustic = { git = "https://github.com/tanmaymunjal/rustic" }
```

```rust
use rustic::app::{run, App, Request};
use rustic::http11_response::Response;
use rustic::parse_headers::RequestType;
use std::collections::HashMap;

fn main() {
    let mut application = App::new();

    fn hello_world(_: Request) -> Option<Response<'static>> {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/plain".to_string());
        let response = Response {
            status_code: 200,
            reason: "Ok",
            response_body: Some("Hi!"),
            headers,
        };
        Some(response)
    }

    application.add_endpoint("test", RequestType::POST, hello_world);
    run(application, 8002, true);
}
```

## Basic API Overview

i) **App::new()**: Create a new application instance.

ii) **add_endpoint(path, method, handler)**: Add a new endpoint to your application.

iii) **run(app, port, debug)**: Start the server with the specified configuration.

For more details, please take a look at our docs: https://tanmaymunjal.github.io/rustic/rustic/
