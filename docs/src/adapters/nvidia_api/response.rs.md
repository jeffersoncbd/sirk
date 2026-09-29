## Summary
Parses NVIDIA API JSON output into a harness response containing generated text and optional token usage.

## Behavior
Deserializes `stdout`, returning `InvalidResponse` if parsing fails or there are no choices. It uses the first choice’s content and includes token usage only when both token counts are present.

## Imports
- `serde::Deserialize`: Enables deserialization of the JSON response.
- `super::NvidiaApiAdapter`: Provides the adapter identity for errors.
- `crate::harness`: Supplies response, usage, adapter, and error types.
