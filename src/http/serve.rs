use super::{handle::handle, types::HttpResponse};
use tiny_http::{Header, Response, Server, StatusCode};

pub fn serve(address: &str) -> Result<(), String> {
    let server = Server::http(address).map_err(|error| error.to_string())?;
    eprintln!("S.I.R.K. HTTP server listening on http://{address}");
    for mut request in server.incoming_requests() {
        std::thread::spawn(move || {
            let method = request.method().to_string();
            let path = request.url().to_owned();
            let mut body = String::new();
            let response = match request.as_reader().read_to_string(&mut body) {
                Ok(_) => handle(&method, &path, &body),
                Err(_) => HttpResponse {
                    status: 400,
                    body: serde_json::json!({ "error": "Invalid request body" }).to_string(),
                    content_type: super::types::JSON_CONTENT_TYPE,
                },
            };
            let content_type = Header::from_bytes(b"Content-Type", response.content_type)
                .expect("static HTTP header must be valid");
            let response = Response::from_string(response.body)
                .with_status_code(StatusCode(response.status))
                .with_header(content_type);
            if let Err(error) = request.respond(response) {
                eprintln!("Failed to send HTTP response: {error}");
            }
        });
    }
    Ok(())
}
