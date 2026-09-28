## Summary
Reads a nonempty key value from a directory's `.env` file.

## Behavior
Returns `Ok(None)` if the file does not exist or the key is missing. Read or parse errors are converted to `HarnessError::InvalidConfiguration`; when the key is found, returns its value if nonempty.

## Imports
- `OllamaWebAdapter`: Provides the adapter identifier.
- `nonempty`: Converts empty values to `None`.
- `HarnessAdapter`: Allows retrieving the adapter identifier.
- `HarnessError`: Represents configuration errors.
- `io`: Identifies missing-file errors.
- `Path`: Represents the queried directory.
- `dotenvy`: Reads and parses the `.env` file.
