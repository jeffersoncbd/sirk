## Summary
Saves history to disk by serializing its metadata and blocks.

## Behavior
Serializes the snapshot as YAML and builds the file contents, escaping reserved lines in blocks. Writes a temporary file, synchronizes the data, renames it to the destination, and synchronizes the directory. Serialization and file-operation errors are converted to `String` and returned.

## Imports
- `super`: Accesses history types and constants.
- `reserved`: Identifies lines requiring escaping.
- `std::fs`: Creates, renames, and synchronizes files.
- `std::io::Write`: Writes data to the temporary file.
- `serde_yaml`: Serializes the snapshot as YAML.
