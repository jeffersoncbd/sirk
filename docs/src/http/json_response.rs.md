## Summary
Wraps a serializable value in an HTTP response with a JSON content type.

## Behavior
`into_response` pairs the configured JSON content type with Axum’s `Json` wrapper around the stored value, then converts the pair into a response.

## Imports
- `super::content_type::JSON`: Supplies the JSON content-type value.
- `axum`: Provides JSON wrapping, response types, and conversion.
- `serde::Serialize`: Bounds values that can be serialized.
