## Summary
Defines `History`, which stores conversation state, flow paths, resume calls, and its lock.

## Behavior
`History` keeps the history and resume paths, a snapshot, conversation blocks, resume-call data, and a private lock file.

## Imports
- `crate::interfaces`: Provides the `Block` and `Snapshot` types.
- `resume_call::ResumeCall`: Stores resume-call data.
- `std::fs::File`: Holds the lock file.
- `std::path::PathBuf`: Holds the history paths.
