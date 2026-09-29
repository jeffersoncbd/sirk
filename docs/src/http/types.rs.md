## Summary
Defines HTTP content-type constants and request and response data types.

## Behavior
`AgentRequest` carries a directory, agent name, and input; `DirectoryRequest` carries a directory. Both reject unknown fields when deserialized. `HttpResponse` stores a status code, body, and content type.

## Imports
- `serde`: Deserializes request types.
- `std::path::PathBuf`: Represents request directories.
