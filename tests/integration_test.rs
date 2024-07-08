#[cfg(test)]
mod integration_tests {
    use reqwest::blocking::Client;
    use rustic::app::{run, App, Request};
    use rustic::connection::{handle_connection, listen_at_port};
    use rustic::http11_response::Response;
    use rustic::parse_headers::RequestType;
    use std::collections::HashMap;
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_port_bind() {
        listen_at_port(8000);
    }

    #[test]
    fn test_request() {
        let listener = listen_at_port(8001);
        let (tx, rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            if let Ok((mut stream, _)) = listener.accept() {
                let request = handle_connection(&mut stream);
                tx.send(request).unwrap();
            }
        });

        // Send a request
        let client = Client::new();
        let url = "http://localhost:8001/";
        let _ = client.get(url).send();

        // Wait to receive the processed request
        let (received_headers, _) = rx
            .recv_timeout(Duration::from_secs(2))
            .expect("Didn't receive a request within the timeout period");

        // Assert that we received the expected request
        assert!(
            !received_headers.is_empty(),
            "The received request shouldn't be empty"
        );
        assert_eq!(
            received_headers[0], "GET / HTTP/1.1",
            "The first line should be the GET request"
        );
        assert_eq!(
            received_headers[1], "accept: */*",
            "The second line should be accept header"
        );
        assert_eq!(
            received_headers[2], "host: localhost:8001",
            "The third line should be host header"
        );

        // Wait for the listener thread to finish
        handle.join().unwrap();
    }

    #[test]
    fn test_create_app() {
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

        let (tx, rx) = mpsc::channel();

        // Start the server in a separate thread
        let server_handle = thread::spawn(move || {
            tx.send(()).unwrap();
            run(application, 8002, true);
        });

        // Wait for the signal that the server has started
        rx.recv_timeout(Duration::from_secs(2))
            .expect("Server did not start in time");

        // Create a client and send a POST request
        let client = Client::new();
        let url = "http://localhost:8002/test";
        let response = client.post(url).send().expect("Failed to send request");

        // Assert that we received the expected response
        assert_eq!(response.status().as_u16(), 200, "Status code should be 200");
        assert_eq!(
            response.text().unwrap(),
            "Hi!",
            "Response body should be 'Hi!'"
        );
    }
}
