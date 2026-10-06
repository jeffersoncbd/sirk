## Summary
`Agent::parse` parses YAML-front-matter source and builds an `Agent`.

## Behavior
Requires opening and closing `---` delimiters, valid metadata, a nonempty adapter, and nonempty Markdown instructions. It rejects `json: true` and delete-without-confirm unless delete permission is enabled; failures return descriptive strings.

## Imports
- `serde_yaml`: Deserializes the YAML metadata.
- `model`: Deserializes the optional model.
- `call_prefix`: Deserializes call prefixes.
- `tree_permission`: Deserializes tree-tool permission.
- `read_permission`: Deserializes read-tool permission.
- `ask_permission`: Deserializes ask-tool permission.
- `edit_permission`: Deserializes edit-tool permission.
- `delete_permission`: Deserializes delete permissions.
