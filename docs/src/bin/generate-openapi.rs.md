## Summary
Generates the OpenAPI YAML document and writes it to `openapi.yaml`.

## Behavior
Builds the document, converts it to YAML, then writes it under the Cargo manifest directory; conversion and write errors are returned as strings.

## Imports
- `sirk::http`: Provides the OpenAPI document.
- `std::fs`: Writes the YAML file.
- `std::path::Path`: Builds the output path.
