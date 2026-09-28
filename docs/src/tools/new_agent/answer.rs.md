## Summary
Requests an answer until a nonempty value is received.

## Behavior
Repeats the question and returns the original answer when it contains more than whitespace. If `input.ask` fails, propagates the error.

## Imports
- `crate::input::UserInput`: Interface used to request answers.
