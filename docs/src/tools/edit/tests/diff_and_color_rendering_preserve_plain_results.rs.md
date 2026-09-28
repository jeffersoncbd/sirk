## Summary
Contains a test that verifies generation and rendering of text diffs.

## Behavior
The test checks that the diff shows removed and added lines, uncolored rendering preserves the result, and colored rendering highlights those lines. It also verifies escape-sequence removal and the marker for a missing final newline.

## Imports
- `super::*`: Imports parent-module items used by the test.
- `crate::tools::edit::render::render`: Renders diffs with or without colors.
