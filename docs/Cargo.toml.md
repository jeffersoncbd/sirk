## Summary
Configures the `sirk` package, its libraries, dependencies, and executable.

## Behavior
Defines package metadata, points the library to `src/lib.rs` and the executable to `src/main.rs`, and declares project dependencies.

## Imports
- `serde`: Serialization and deserialization.
- `serde_yaml`: YAML support.
- `serde_json`: JSON support.
- `dotenvy`: Environment-variable loading.
- `sha2`: SHA-2 hashing functions.
- `sirk-sdk`: The project's local SDK.
- `similar`: Text comparison.
- `tiny_http`: Lightweight HTTP server.
