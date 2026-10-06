## Summary
This test verifies that agent definitions with malformed content or unsafe IDs are rejected.

## Behavior
It passes invalid definition strings to `Agent::parse` and invalid IDs to `Agent::load`, asserting each call returns an error.

## Imports
- `Agent`: Provides definition parsing and loading.
- `std::path::Path`: Constructs the agent directory path.
