#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 2 || -z "$2" ]]; then
  printf '%s\n' 'Usage: find-line-by-prefix.sh <content> <prefix>' >&2
  exit 1
fi

content=$1
prefix=$2
current_line=0
matched_line=0
matches=0

while IFS= read -r line || [[ -n "$line" ]]; do
  ((current_line += 1))
  if [[ "$line" == "$prefix"* ]]; then
    matched_line=$current_line
    ((matches += 1))
  fi
done < <(printf '%s' "$content")

if [[ $matches -ne 1 ]]; then
  printf 'Expected exactly one line starting with "%s", found %d.\n' \
    "$prefix" "$matches" >&2
  exit 1
fi

printf '%s' "$matched_line"
