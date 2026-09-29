## Summary
Configures the `sirk` package, library, executable, and dependencies.

## Behavior
Sets package metadata and source paths, declares runtime dependencies for serialization, configuration, hashing, comparison, the SDK, HTTP, async execution, and OpenAPI generation, and adds Tower utilities for development.

## Imports
- `serde`: Serialization and deserialization.
- `serde_yaml`: YAML support.
- `serde_json`: JSON support.
- `dotenvy`: Environment-variable loading.
- `sha2`: SHA-2 hashing.
- `sirk-sdk`: Local SDK dependency.
- `similar`: Text comparison.
- `axum`: HTTP server framework.
- `tokio`: Async runtime and networking.
- `utoipa`: OpenAPI generation.
- `utoipa-axum`: Axum OpenAPI integration.
- `tower`: Development utilities.
