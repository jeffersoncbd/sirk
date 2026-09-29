## Summary
Validates that a READ path is nonempty, stays inside the execution directory, and is not ignored.

## Behavior
Returns an error for blank paths, directories that cannot be resolved, paths containing non-normal components, or ignored paths. Otherwise, returns `Ok(())`.

## Imports
- `std::path`: Builds and checks the requested path.
- `super::ignored`: Checks whether the path is ignored.
