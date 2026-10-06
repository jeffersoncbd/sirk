## Summary
Runs one of the available flows selected by a single command-line argument.

## Behavior
Requires exactly one flow name, returning an input error if it is missing or extra arguments are provided. It connects to Sirk and runs the matching documentation or profile interviewer flow; an unknown name returns a not-found error, and connection or flow errors propagate.

## Imports
- `sirk_sdk::Sirk`: Connects to the Sirk service.
- `documentation`: Provides the documentation flow.
- `profile_interviewer`: Provides the profile interviewer flow.
