## Summary
Prompts for and returns an available adapter selected by the user.

## Behavior
Displays the available options and requests a response. Trims whitespace and lowercases the input; if it matches an available adapter, returns `Ok` with its name. Otherwise, prompts again with an error message. Propagates `input.ask` failures as `Err`.

## Imports
- `crate::adapters`: Provides the list of available adapters.
- `crate::input::UserInput`: Allows requesting the user's choice.
