## Summary
Creates an `OllamaWebAdapter` with the supplied values and normalizes optional fields.

## Behavior
Converts `executable` and `base_url` to `String`, applies `nonempty` to the URL, and discards the API key if it is empty or contains only whitespace. Returns the configured instance.

## Imports
- `super::OllamaWebAdapter`: Adapter type being instantiated.
- `nonempty::nonempty`: Converts the URL to an optional value.
