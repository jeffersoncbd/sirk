## Summary
Compares pattern components with path components, supporting `**` and prefix matching.

## Behavior
Returns `true` when the pattern and path end together; if the pattern ends first, uses `prefix` as the result. `**` can match zero or more components. Otherwise, compares each component pair recursively; incompatible combinations return `false`.

## Imports
- `super::component`: Compares one pattern component with a path component.
