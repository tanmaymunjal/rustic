#[cfg(test)]
mod integration_tests {
    use reqwest::blocking::Client;
    use rustic::connection::{handle_connection, listen_at_port};
    use std::sync::mpsc;
    use std::thread;

    #[test]
    fn test_port_bind() {
        listen_at_port(8000);
    }

    #[test]
    fn test_request() {
        let listener = listen_at_port(8000);
        let (tx, rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            if let Ok((stream, _)) = listener.accept() {
                let request = handle_connection(stream);
                tx.send(request).unwrap();
            }
        });

        // Send a request
        let client = Client::new();
        let url = "http://localhost:8000/";
        let _ = client.get(url).send();

        // Wait to receive the processed request
        let received_request = rx
            .recv_timeout(std::time::Duration::from_secs(2))
            .expect("Didn't receive a request within the timeout period");

        // Assert that we received the expected request
        assert!(
            !received_request.is_empty(),
            "The received request shouldn't be empty"
        );
        assert_eq!(
            received_request[0], "GET / HTTP/1.1",
            "The first line should be the GET request"
        );
        assert_eq!(
            received_request[1], "accept: */*",
            "The second line should be accept header"
        );
        assert_eq!(
            received_request[2], "host: localhost:8000",
            "The third line should be host header"
        );

        // Wait for the listener thread to finish
        handle.join().unwrap();
    }
}
