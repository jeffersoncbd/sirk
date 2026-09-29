## Summary
Checks whether a string is a valid flow ID.

## Behavior
Requires the value to start with `flow-`, followed by at least one ASCII hexadecimal digit or hyphen; returns `true` if every remaining byte meets that rule.

## Imports
- None: uses Rust standard library methods.
