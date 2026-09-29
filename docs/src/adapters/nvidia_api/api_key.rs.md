## Summary
`api_key` resolves the NVIDIA API key from configured, environment, or dotenv values.

## Behavior
It checks the adapter’s stored key first, then a nonempty `NVIDIA_API_KEY` environment value, then the directory’s dotenv value. Dotenv errors propagate; if no value is found, it returns an `InvalidConfiguration` error naming the adapter.

## Imports
- `std::path::Path`: Provides the directory path for dotenv lookup.
- `NvidiaApiAdapter`: Supplies configured key, dotenv lookup, and adapter ID.
- `nonempty`: Filters out empty environment variable values.
- `HarnessAdapter`, `HarnessError`: Provide adapter ID access and error type.
