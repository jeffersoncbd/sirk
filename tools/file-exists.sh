#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 || -z "$1" ]]; then
  printf '%s\n' 'Usage: file-exists.sh <path>' >&2
  exit 1
fi

if [[ -f "$1" ]]; then
  printf '%s' true
else
  printf '%s' false
fi
