## Summary
`Agent::parse` validates a YAML-front-matter definition and converts it into an `Agent`.

## Behavior
Requires opening and closing `---` delimiters, valid metadata, a nonempty adapter, and nonempty Markdown instructions. It rejects unsupported JSON mode and delete-without-confirm unless delete permission is enabled; errors are returned as messages.

## Imports
- `serde`: Defines metadata deserialization.
- `serde_yaml`: Parses YAML metadata.
- `model`: Deserializes the optional model.
- `call_prefix`: Deserializes call prefixes.
- `tree_permission`: Deserializes tree-tool permission.
- `ask_permission`: Deserializes ask-tool permission.
- `edit_permission`: Deserializes edit-tool permission.
- `delete_permission`: Deserializes delete permissions.
- `tree_default`: Supplies the default tree-tool setting.
