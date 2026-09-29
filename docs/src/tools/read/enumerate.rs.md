## Summary
`enumerate` adds stable, one-based line numbers to file content.

## Behavior
Delegates to `super::enumerate_from` with a starting number of 1, returning the numbered content as a `String`.

## Imports
- `super::enumerate_from`: Formats content with line numbers.
