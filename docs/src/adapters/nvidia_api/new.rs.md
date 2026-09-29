## Summary
Constructs a `NvidiaApiAdapter` from an executable name and optional API key.

## Behavior
Converts the executable into a `String` and stores it. Keeps the API key only if `nonempty` accepts it; otherwise, stores `None`.

## Imports
- `NvidiaApiAdapter`: Adapter type being constructed.
- `nonempty`: Filters out empty API keys.
