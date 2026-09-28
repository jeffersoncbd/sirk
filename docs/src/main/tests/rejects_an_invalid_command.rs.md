## Summary
Verifies that an invalid command is rejected.

## Behavior
Runs `run` with the command `"invalid"`, expects an error, and checks that the message contains `"invalid command"`.

## Imports
- `super::*`: Imports parent-module items used by the test.
