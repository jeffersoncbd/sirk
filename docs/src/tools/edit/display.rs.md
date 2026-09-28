## Summary
Displays a rendered diff, using colors when the terminal supports them.

## Behavior
Checks whether standard output is a terminal and `NO_COLOR` is absent, uses that condition to render the diff, and prints the result to standard output.

## Imports
- `std::io::IsTerminal`: Checks whether standard output is a terminal.
- `super::render::render`: Renders the diff with or without colors.
