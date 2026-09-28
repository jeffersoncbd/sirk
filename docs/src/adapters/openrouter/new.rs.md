## Summary
Creates an OpenRouter adapter with an executable and optional base URL and API key.

## Behavior
Converts the executable and base URL to `String`, applies `nonempty` to the URL, and discards the API key if it is empty or contains only whitespace.

## Imports
- `super::OpenRouterAdapter`: Adapter type being created.
- `super::nonempty::nonempty`: Handles the base URL.
