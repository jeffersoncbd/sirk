## Summary
Executes a Bash invocation and captures its binary output.

## Behavior
Starts the process in the invocation's directory and environment, with no standard input and errors inherited by the parent. It reads output in chunks, copies each chunk to `output`, and accumulates it. If reading or writing fails, it terminates and waits for the process before returning the error. On completion, it returns the status and captured bytes.

## Imports
- `super::{BashService, BinaryProcessOutput}`: Service and result type.
- `crate::services::Invocation`: Execution parameters.
- `std::io::{self, Read, Write}`: Reading, writing, and I/O errors.
- `std::process::{Command, Stdio}`: Process creation and configuration.
