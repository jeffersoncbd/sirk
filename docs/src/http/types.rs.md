## Summary
Defines types for agent and directory requests and HTTP responses.

## Behavior
`AgentRequest` holds a directory, agent, and input; `DirectoryRequest` contains the directory. Both reject unknown fields during deserialization. `HttpResponse` stores the status code and body.

## Imports
- `serde`: Deserializes requests and rejects unknown fields.
- `std::path::PathBuf`: Represents directories in requests.
