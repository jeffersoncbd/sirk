## Summary
`Agent::parse` validates and converts a textual definition into an agent.

## Behavior
Requires YAML metadata between `---` delimiters, validates fields such as adapter and permissions, and rejects empty instructions. On error it returns a message; otherwise, it returns an `Agent` with normalized metadata and Markdown instructions.

## Imports
- `serde`: Metadata serialization and deserialization.
- `serde_yaml`: YAML metadata conversion.
- `model`: Model deserialization.
- `call_prefix`: Call-prefix deserialization.
- `tree_default`: Default tree permission.
- `tree_permission`: Tree-permission deserialization.
- `edit_permission`: Edit-permission deserialization.
- `delete_permission`: Delete-permission deserialization.
