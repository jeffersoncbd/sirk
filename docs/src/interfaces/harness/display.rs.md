## Summary
Formats `HarnessError` values as readable messages.

## Behavior
Selects a message for the error variant, includes the adapter and available details, and writes it to the formatter.

## Imports
- `super::HarnessError`: Error type being formatted.
- `std::fmt`: Provides the formatting implementation.
