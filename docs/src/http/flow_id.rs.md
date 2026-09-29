## Summary
Extracts and validates the `X-Sirk-Flow-Id` header, returning its value or an error response.

## Behavior
Reads the header as text and checks it with `valid_flow_id`; missing, non-text, or invalid values produce a boxed JSON error response.

## Imports
- `JsonResponse`, `ErrorResponse`: Build the JSON error response.
- `HeaderMap`: Provides request headers.
- `IntoResponse`, `Response`: Convert and return the error response.

