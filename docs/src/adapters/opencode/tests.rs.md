## Summary
Declares test submodules for the OpenCode adapter.

## Behavior
There is no main function; the file imports parent-module symbols and support types, and declares two test modules.

## Imports
- `super::*`: Parent-module symbols.
- `crate::harness`: Types used by adapter tests.
- `std::path::PathBuf`: Type for representing paths.
