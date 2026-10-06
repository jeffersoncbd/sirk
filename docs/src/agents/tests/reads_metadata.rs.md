## Summary
`Agent::parse` creates an agent from an ID and CRLF-formatted metadata and Markdown.

## Behavior
It returns a `Result` containing the supplied ID, model, and normalized instructions; unspecified tool options are disabled and the call prefix is empty.

## Imports
- `super::super::Agent`: Provides the agent type and its `parse` method.
