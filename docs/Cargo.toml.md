## Summary
Configures the `sirk` package, library, executable, and Rust dependencies.

## Behavior
Sets package metadata and maps the library and binary to their source files. Declares runtime dependencies for serialization, environment loading, hashing, text comparison, the local SDK, and the Tokio-based Axum server, plus Tower utilities for development.

## Imports
- `serde`: Serialization and deserialization.
- `serde_yaml`: YAML support.
- `serde_json`: JSON support.
- `dotenvy`: Environment-variable loading.
- `sha2`: SHA-2 hashing.
- `sirk-sdk`: Local SDK.
- `similar`: Text comparison.
- `axum`: HTTP server framework.
- `tokio`: Async runtime and networking.
- `tower`: Development utilities.
