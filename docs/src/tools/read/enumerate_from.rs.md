## Summary
`enumerate_from` prefixes each line of content with a line number starting at the given offset.

## Behavior
It splits the input at newline boundaries, appends each line with its number and a `" | "` separator after a header, and returns the formatted text. Empty input returns only the header.

## Imports
- No imports used.
