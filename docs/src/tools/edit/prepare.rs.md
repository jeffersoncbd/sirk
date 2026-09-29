## Summary
Prepares an edit by resolving its target and validating the operation against the current contents.

## Behavior
Allows missing targets for `Append`, `Prepend`, and `Write`; other operations require an existing file. Reads the optional contents, applies the requested operation to validate it, then stores whether the file was missing and its previous contents. Returns an error if target resolution, reading, or application fails.

## Imports
- `Operation`: Identifies edit types that allow missing files.
- `Pending`: Provides the request and stores preparation state.
- `read_optional`: Reads file contents when the target exists.
- `target`: Resolves the target path and checks missing-file rules.
- `Path`: Supplies the working directory.
