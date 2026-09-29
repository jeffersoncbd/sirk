## Summary
This file contains tests, but no main function to document.

## Behavior
The tests check that the NVIDIA API adapter rejects requests without a model and requests with event streams enabled.

## Imports
- `std::path::PathBuf`: Builds the test working directory path.
- `crate::adapters::NvidiaApiAdapter`: Adapter exercised by the tests.
- `crate::harness`: Request and error types used by the tests.
