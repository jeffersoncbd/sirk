## Summary
`DirectoryRequest` carries a server-visible directory path supplied in a request.

## Behavior
Deserialization reads the `directory` field into a `PathBuf` and rejects unknown fields. Its schema presents the path as a string with an example.

## Imports
- `serde`: Provides request deserialization.
- `std::path::PathBuf`: Stores the directory path.
- `utoipa`: Provides OpenAPI schema metadata.
