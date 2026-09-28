## Summary
The adapter configures request execution through the `codex exec` command.

## Behavior
`invocation` builds arguments with read-only access and approvals disabled; it includes JSON output and a model when requested, and appends the prompt after `--`. It returns an invocation with the executable and working directory and no additional environment variables. `id` identifies the adapter as `codex`.

## Imports
- `default`, `new`: Codex adapter modules.
- `crate::harness`: Harness interface and invocation types.
