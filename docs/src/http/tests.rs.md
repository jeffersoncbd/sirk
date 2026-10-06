## Summary
Declares HTTP test modules and associates each with its test file.

## Behavior
Each `#[path]` attribute points a module declaration to a file under `tests/`; compiling this file includes those test modules.

## Imports
- Test modules: HTTP test cases loaded from files under `tests/`.
