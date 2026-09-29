## Summary
`DirectoryRequest` represents a request containing a server-visible directory path.

## Behavior
The request deserializes a `directory` field into a `PathBuf`, rejects unknown fields, and exposes the path as a string in the OpenAPI schema.

## Imports
- `serde`: Provides request deserialization.
- `std::path::PathBuf`: Stores the directory path.
- `utoipa`: Provides OpenAPI schema metadata.
