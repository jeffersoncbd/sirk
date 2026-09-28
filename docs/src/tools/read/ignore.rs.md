## Summary
Checks whether a path matches the supplied ignore pattern.

## Behavior
Removes leading slashes from the pattern and splits the pattern and path into components. For a single-component pattern, checks whether any path component matches; for patterns with multiple components, delegates comparison and indicates whether the pattern ends with `/`.

## Imports
- `super::component`: Compares a pattern with a path component.
- `super::components`: Compares component sequences.
