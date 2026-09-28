## Summary
Applies an edit operation to contents and returns the updated text or an error.

## Behavior
Validates the request and checks that the supplied version matches the current contents. Calculates edit boundaries for the operation and rejects insertions or ranges beyond the end of the file. Finally, combines the previous contents, input, and preserved portion.

## Imports
- `super::{Operation, Request, version}`: Edit types and version calculation.
