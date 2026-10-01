## Summary
Records usage totals and the current agent under each adapter in the usage summary file.

## Behavior
Returns immediately when there are no usage calls. Otherwise it reads existing data, treating a missing file as empty, updates or adds adapter totals, and adds the agent and model under each adapter. It truncates and rewrites the file, then syncs the file and parent directory; I/O errors are returned as strings.

## Imports
- `super::History`: Provides the history data used to build the summary.
- `std::fs::OpenOptions`: Opens or creates the summary file for rewriting.
- `std::io::Write`: Writes and syncs the summary file.
