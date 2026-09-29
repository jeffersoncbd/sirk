## Summary
The main function cannot be documented because the provided file contains only a test.

## Behavior
The test checks that `response` extracts the first choice’s content and token usage, and returns an error when choices are missing.

## Imports
- `crate::adapters::NvidiaApiAdapter`: Used to construct the adapter in the test.
- `crate::harness`: Supplies response and token usage types for the test.
