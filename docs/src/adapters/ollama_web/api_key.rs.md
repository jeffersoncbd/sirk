## Summary
Retrieves the Ollama API key from the available sources.

## Behavior
Prioritizes a key already configured on the adapter, then checks `OLLAMA_API_KEY` in the environment, and finally checks the directory's dotenv file. Empty values are treated as absent. Dotenv read errors are propagated.

## Imports
- `super`: Adapter and nonempty-value validator.
- `crate::harness::HarnessError`: Returned error type.
- `std::path::Path`: Represents the queried directory.
