## Summary
Appends token usage details for an adapter to the history file.

## Behavior
Opens the history file in append mode, writes the adapter and input/output token counts, then syncs the file and its parent directory. Returns an error string if opening, writing, syncing, or finding the parent directory fails.

## Imports
- `super::History`: Provides the history path.
- `crate::harness::TokenUsage`: Supplies input and output token counts.
- `std::fs::OpenOptions`: Opens the history file for appending.
- `std::io::Write`: Writes usage data to the file.
