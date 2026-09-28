## Summary
Loads and validates an agent from a Markdown file.

## Behavior
Rejects invalid IDs, reads `{id}.md` from the supplied directory, and converts read or parse failures into error messages. Returns the parsed agent or a `String` describing the failure.

## Imports
- `super::Agent`: Loaded agent type.
- `super::valid_id`: Validates the agent identifier.
- `std::fs`: Reads file contents.
- `std::path::Path`: Represents the source directory.
