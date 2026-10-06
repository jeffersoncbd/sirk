## Summary
No function is defined in the provided file; it declares conversation data types.

## Behavior
The structs and enums model conversation metadata, status, user and assistant messages, and tool messages. Serde attributes define their serialized field names and enum representations, and reject unknown fields in `ConversationData`.

## Imports
- `serde`: Serialization and deserialization derives and attributes.
- `std::path::PathBuf`: Stores the conversation file path.
