## Summary
Verifies that file deletion requires explicit agent permission.

## Behavior
Creates an agent that requests deletion of `note.txt` and runs the request with confirmation disallowed and allowed. It checks both responses; the file should be removed only when both permissions are enabled.

## Imports
- `handle`: Processes the agent HTTP request.
- `std::fs`: Creates files and checks their removal.
- `PermissionsExt`: Makes the adapter executable.
- `SystemTime` and `UNIX_EPOCH`: Generate a unique temporary name.
