## Summary
Dispatches arguments to the corresponding command or displays help.

## Behavior
With no arguments or with `help`, `--help`, or `-h`, displays help and returns success. Recognizes `--newAgent` and `--new-agent` for agent creation and `http` with an optional address. Any other combination returns an error containing the usage message.

## Imports
- `create_agent`: Creates an agent.
- `http`: Runs the HTTP command.
- `print_usage`: Displays help.
- `usage`: Provides the usage message.
