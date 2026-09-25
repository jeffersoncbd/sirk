#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 2 || -z "$2" ]]; then
  printf '%s\n' 'Usage: line-exists-by-prefix.sh <content> <prefix>' >&2
  exit 1
fi

content=$1
prefix=$2
matches=0

while IFS= read -r line || [[ -n "$line" ]]; do
  if [[ "$line" == "$prefix"* ]]; then
    ((matches += 1))
  fi
done < <(printf '%s' "$content")

case "$matches" in
  0) printf '%s' false ;;
  1) printf '%s' true ;;
  *)
    printf 'Expected at most one line starting with "%s", found %d.\n' \
      "$prefix" "$matches" >&2
    exit 1
    ;;
esac
