## Summary
Generates an agent in the specified directory and returns the saved path.

## Behavior
Delegates generation to `create_with`, executing the invocation through `BashService`. Converts execution failures into errors and rejects output with an unsuccessful status; on success, returns stdout.

## Imports
- `UserInput`: Provides input used during generation.
- `BashService`: Executes the invocation and captures output.
- `Path`, `PathBuf`: Represent the directory and returned path.
