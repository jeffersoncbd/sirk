#!/usr/bin/env bash

set -euo pipefail

if [[ $# -ne 0 ]]; then
  printf '%s\n' 'Usage: sort-tree.sh' >&2
  exit 1
fi

tree_path='docs/TREE.md'

if [[ ! -f "$tree_path" ]]; then
  printf 'TREE document is not a regular file: %s\n' "$tree_path" >&2
  exit 1
fi

LC_ALL=C sed '/^[[:space:]]*$/d' "$tree_path" | LC_ALL=C sort
