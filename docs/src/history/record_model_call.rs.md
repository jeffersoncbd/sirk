## Summary
Records a model call and accumulates its token usage by adapter.

## Behavior
Finds or creates the adapter’s usage entry, increments its call count, and, when usage is provided, adds input and output tokens and marks usage as available.

## Imports
- `History`, `UsageCall`: Store and initialize per-adapter call data.
- `TokenUsage`: Supplies input and output token counts.
