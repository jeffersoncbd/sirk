use super::types::{AgentParams, Request, Response, RpcError};
use serde_json::Value;
use std::path::Path;

pub(super) fn handle(request: Request, directory: &Path) -> Response {
    let id = request.id.unwrap_or(Value::Null);
    if request.jsonrpc != "2.0" || (!id.is_number() && !id.is_string()) {
        return Response {
            jsonrpc: "2.0",
            id,
            result: None,
            error: Some(RpcError {
                code: -32600,
                message: "Invalid Request".to_owned(),
            }),
        };
    }
    if request.method != "agent.run" {
        return Response {
            jsonrpc: "2.0",
            id,
            result: None,
            error: Some(RpcError {
                code: -32601,
                message: "Method not found".to_owned(),
            }),
        };
    }
    let params = match serde_json::from_value::<AgentParams>(request.params) {
        Ok(params) => params,
        Err(_) => {
            return Response {
                jsonrpc: "2.0",
                id,
                result: None,
                error: Some(RpcError {
                    code: -32602,
                    message: "Invalid params".to_owned(),
                }),
            };
        }
    };
    match crate::agent_service::run(directory, params.agent, params.input) {
        Ok(result) => Response {
            jsonrpc: "2.0",
            id,
            result: Some(result),
            error: None,
        },
        Err(message) => Response {
            jsonrpc: "2.0",
            id,
            result: None,
            error: Some(RpcError {
                code: -32000,
                message,
            }),
        },
    }
}
