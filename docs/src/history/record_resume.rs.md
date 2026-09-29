## Summary
Records resume call counts and token usage by adapter in the resume file.

## Behavior
Returns immediately when there are no resume calls. Otherwise, it reads existing data (treating a missing file as empty), updates or adds each adapter’s totals, and inserts the current agent and model. It then truncates and rewrites the file, syncing the file and its parent directory; I/O errors are returned as strings.

## Imports
- `super::History`: Provides the history data used to update the resume record.
- `std::fs::OpenOptions`: Opens or creates the resume file for rewriting.
- `std::io::Write`: Writes the updated record and syncs it.
