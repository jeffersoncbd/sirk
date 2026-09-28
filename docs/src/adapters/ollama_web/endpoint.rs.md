## Summary
Builds the Ollama Web generation endpoint from the configured URL.

## Behavior
Prioritizes the adapter URL, then the environment variable, and finally the dotenv value or default. Removes trailing slashes, returns an error if the URL is empty, and appends the required path unless it already ends in `/api/generate`.

## Imports
- `super`: Types, default URL, and nonempty-value validation.
- `crate::harness`: Adapter trait and error type.
- `std::path::Path`: Path used to query the dotenv file.
