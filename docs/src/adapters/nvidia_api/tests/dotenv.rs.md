## Summary
This file contains no main function to document.

## Behavior
The file only defines a test that checks whether `NvidiaApiAdapter::invocation` reads an API key from a temporary `.env` file.

## Imports
- `std`: Creates, writes, and removes the temporary test directory.
- `NvidiaApiAdapter`: Builds the invocation checked by the test.
- `HarnessAdapter`, `RunRequest`: Provide the request and invocation interface.
