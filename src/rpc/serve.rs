use super::{
    handle::handle,
    types::{Request, Response, RpcError},
};
use serde_json::Value;
use std::io::{self, BufRead, Write};

pub fn serve() -> Result<(), String> {
    let directory = std::env::current_dir().map_err(|error| error.to_string())?;
    let stdin = io::stdin();
    let mut stdout = io::stdout().lock();
    for line in stdin.lock().lines() {
        let line = line.map_err(|error| error.to_string())?;
        let response = match serde_json::from_str::<Value>(&line) {
            Ok(value) => match serde_json::from_value::<Request>(value) {
                Ok(request) => handle(request, &directory),
                Err(_) => Response {
                    jsonrpc: "2.0",
                    id: Value::Null,
                    result: None,
                    error: Some(RpcError {
                        code: -32600,
                        message: "Invalid Request".to_owned(),
                    }),
                },
            },
            Err(_) => Response {
                jsonrpc: "2.0",
                id: Value::Null,
                result: None,
                error: Some(RpcError {
                    code: -32700,
                    message: "Parse error".to_owned(),
                }),
            },
        };
        serde_json::to_writer(&mut stdout, &response).map_err(|error| error.to_string())?;
        stdout
            .write_all(b"\n")
            .and_then(|_| stdout.flush())
            .map_err(|error| error.to_string())?;
    }
    Ok(())
}
