## Summary
This file contains tests only and has no main function to document.

## Behavior
The test checks that a non-streaming NVIDIA chat request includes the expected endpoint and payload, while keeping the API key out of arguments and placing it in the environment.

## Imports
- `std::path::PathBuf`: Builds the test request’s working directory.
- `NvidiaApiAdapter`: Creates the adapter invocation under test.
- `HarnessAdapter`, `RunRequest`: Provide the invocation method and request type.
