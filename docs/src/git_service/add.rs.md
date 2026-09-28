## Summary
Adds all project changes in the supplied directory to Git.

## Behavior
Locates the project from `directory` and returns an error if that fails. It then runs `git add --all -- .` in the project directory and returns `Ok(())` when the command succeeds.

## Imports
- `super::project::project`: Locates the project associated with the directory.
- `super::run::run`: Runs the Git command and propagates errors.
- `std::path::Path`: Represents the supplied directory.
