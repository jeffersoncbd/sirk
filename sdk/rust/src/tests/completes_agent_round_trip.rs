use crate::Sirk;
use std::{
    io::{BufRead, BufReader, Read, Write},
    net::TcpListener,
    thread,
};

#[test]
fn completes_agent_round_trip() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let endpoint = format!("http://{}", listener.local_addr().unwrap());
    let server = thread::spawn(move || {
        for (expected_path, response_body) in [
            ("/health", r#"{"status":"ok"}"#),
            ("/v1/agent/run", r#"{"result":"documented"}"#),
        ] {
            let (mut stream, _) = listener.accept().unwrap();
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut first_line = String::new();
            reader.read_line(&mut first_line).unwrap();
            assert!(first_line.contains(expected_path));
            let mut content_length = 0;
            loop {
                let mut line = String::new();
                reader.read_line(&mut line).unwrap();
                if line == "\r\n" {
                    break;
                }
                if let Some(value) = line.to_ascii_lowercase().strip_prefix("content-length:") {
                    content_length = value.trim().parse().unwrap();
                }
            }
            let mut request_body = vec![0; content_length];
            reader.read_exact(&mut request_body).unwrap();
            if expected_path == "/v1/agent/run" {
                let request: serde_json::Value = serde_json::from_slice(&request_body).unwrap();
                assert_eq!(request["directory"], "/workspace/project");
                assert_eq!(request["agent"], "code-explainer");
                assert_eq!(request["input"], "Explain this file");
            }
            write!(
                stream,
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                response_body.len(),
                response_body
            )
            .unwrap();
        }
    });
    let sirk = Sirk::connect_to(endpoint, "/workspace/project").unwrap();
    assert_eq!(
        sirk.agent("code-explainer", "Explain this file").unwrap(),
        "documented"
    );
    server.join().unwrap();
}
