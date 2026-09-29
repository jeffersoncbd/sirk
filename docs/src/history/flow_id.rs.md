## Summary
Generates a process-scoped flow ID from the current time and an incrementing counter.

## Behavior
Reads nanoseconds since the Unix epoch, returning the clock error as a string if system time precedes the epoch. It then increments a global atomic counter and formats the timestamp, process ID, and counter as a hexadecimal string prefixed with `flow-`.

## Imports
- `AtomicU64`, `Ordering`: Maintain and increment the global counter.
- `SystemTime`, `UNIX_EPOCH`: Get the current time relative to the epoch.
