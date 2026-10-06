## Summary
`Agent::parse` reads agent metadata and Markdown instructions from a CRLF-formatted string.

## Behavior
The test shows parsing returns an agent with the supplied ID, model, and normalized instructions; unspecified tool options default to disabled, and the call prefix is empty.

## Imports
- `super::super::Agent`: Agent type constructed by `parse`.
