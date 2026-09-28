## Summary
Releases the `History` lock when the value is dropped.

## Behavior
On drop, attempts to unlock `_lock` and ignores any error.

## Imports
- `super::History`: Type whose drop implementation releases the lock.
