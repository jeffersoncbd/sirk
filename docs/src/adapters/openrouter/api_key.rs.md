## Summary
Retrieves the OpenRouter API key or returns an error if it is not configured.

## Behavior
Prioritizes the key stored on the adapter; if absent, checks the `OPENROUTER_API_KEY` environment variable and finally the dotenv file in the supplied directory. Returns the first key found or `HarnessError::InvalidConfiguration` if none is available.

## Imports
- `OpenRouterAdapter`: Provides configuration and access to the dotenv file.
- `nonempty`: Discards empty environment-variable values.
- `HarnessAdapter`: Provides the adapter identifier.
- `HarnessError`: Represents configuration failures.
- `Path`: Represents the directory used to find the dotenv file.
