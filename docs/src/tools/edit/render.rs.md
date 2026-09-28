## Summary
Renders a diff, highlighting changes and escaping control characters.

## Behavior
After the first `@@` header line, colors added and removed lines when `color` is enabled. Escapes control characters from the contents, preserves line breaks, and returns the rendered text.

## Imports
- None: Uses only standard-library types.
