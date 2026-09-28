## Summary
Connects to S.I.R.K. and runs the documentation flow.

## Behavior
Establishes the connection and propagates any failure, then runs `documentation::run` with the connection and returns its result.

## Imports
- `sirk_sdk::Sirk`: Creates the S.I.R.K. connection.
- `documentation`: Provides the documentation flow.
