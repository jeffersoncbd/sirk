## Summary
Checks whether a line uses a reserved history marker.

## Behavior
Returns `true` for fixed markers, the separator, lines beginning with `Step `, or a backslash; otherwise, returns `false`. It has no side effects.

## Imports
- `super::SEPARATOR`: Reserved history separator.
