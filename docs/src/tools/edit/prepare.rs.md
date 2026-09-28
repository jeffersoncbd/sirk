## Summary
Prepares an edit by checking the file and applying the operation to its current contents.

## Behavior
Allows a nonexistent file only for `Append` or `Prepend`; otherwise, returns an error. Reads the contents, applies the requested operation to validate it, and records the previous contents and whether the file was missing.

## Imports
- `Operation`: Identifies the edit type.
- `Pending`: Type that owns `prepare`.
- `read_optional`: Reads contents if the file exists.
- `target`: Resolves the target file path.
- `Path`: Represents the working directory.
