## Summary
Records a model call in the history, tracking its adapter and any token usage.

## Behavior
Finds or creates an entry for the adapter, increments its call count, and adds input and output tokens when usage is provided, marking usage as available.

## Imports
- `History`, `ResumeCall`: Access and initialize recorded call data.
- `TokenUsage`: Provides input and output token counts.
