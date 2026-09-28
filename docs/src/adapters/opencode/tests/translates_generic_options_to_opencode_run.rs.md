## Summary
Verifies that generic options are converted into an OpenCode invocation.

## Behavior
Creates a request with a prompt, directory, model, and event streaming enabled; compares the resulting invocation with the expected command and requires conversion to succeed.

## Imports
- `super::*`: Provides the parent module's types and adapter.
