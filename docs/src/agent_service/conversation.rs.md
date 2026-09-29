## Summary
Runs an agent conversation until it returns a final response or encounters an error.

## Behavior
Records the input and each response in history, then invokes the configured adapter in a loop. It executes permitted edit, write, delete, read, and tree requests; unavailable user input, unauthorized tools, empty responses, and adapter, tool, or history failures return errors.

## Imports
- `adapters`: Resolves the agent adapter and parses responses.
- `RunRequest`: Supplies parameters for each adapter invocation.
- `Block`, `History`: Store conversation history.
- `Invocation`: Represents an agent invocation.
- `tools`: Parses and executes read and tree requests.
