## Summary
Reads user responses and confirmations from the terminal.

## Behavior
`ask` displays the question and repeats until it receives a nonempty answer; `/cancel`, closed input, or I/O failures return an error. `await_confirmation` reads directly from the interactive terminal (`/dev/tty`), accepts any line as confirmation, and returns an error on cancellation, closed input, or failure.

## Imports
- `std::fs::OpenOptions`: Opens the interactive terminal for confirmation.
- `std::io`: Reads and writes terminal data and handles I/O operations.
- `crate::interfaces::UserInput`: Defines the implemented interface.
