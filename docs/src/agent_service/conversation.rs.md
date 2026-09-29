## Summary
Runs an agent conversation until it returns a final response or encounters an error.

## Behavior
Adds the input to history, then repeatedly records prompts and adapter responses. It executes permitted edit, write, delete, read, and tree requests; unavailable user input, unauthorized tools, empty responses, and adapter, tool, or history failures return errors.

## Imports
- `adapters`: Resolves the agent adapter and parses responses.
- `RunRequest`: Supplies parameters for each adapter invocation.
- `Block`, `History`: Store conversation history.
- `Invocation`: Represents an agent invocation.
- `tools`: Parses and executes read and tree requests.
