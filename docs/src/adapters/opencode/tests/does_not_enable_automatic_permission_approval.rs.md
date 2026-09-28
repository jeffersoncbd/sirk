## Summary
Verifies that an OpenCode adapter invocation does not enable automatic permission approval.

## Behavior
Creates a run request, obtains the adapter invocation, and checks that its arguments do not include `--auto`; invocation creation errors are propagated with `unwrap`.

## Imports
- `super::*`: Accesses the parent module's types and adapter.
