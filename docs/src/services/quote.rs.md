## Summary
Produces a shell-safe version of `value`.

## Behavior
If `value` starts with `$` followed by a valid name present in `environment`, it keeps the variable in double quotes. Otherwise, it wraps the value in single quotes and escapes apostrophes. It always returns a `String`.

## Imports
- `std::collections::BTreeMap`: Looks up variables available in the environment.
