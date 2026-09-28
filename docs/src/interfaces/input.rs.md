## Summary
Defines the shared interface for user questions and confirmations.

## Behavior
`ask` receives a question and returns the answer or an error. `await_confirmation` sends the prompt through `ask`, discards the answer, and returns success or the received error.

## Imports
- None: The file imports no dependencies.
