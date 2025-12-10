#!/usr/bin/env bash
set -euo pipefail

README="README.md"
COVERAGE="COVERAGE.md"
TMP="README.tmp"

if [[ ! -f "$COVERAGE" ]]; then
  echo "ERROR: $COVERAGE not found!"
  exit 1
fi

if [[ ! -f "$README" ]]; then
  echo "ERROR: $README not found!"
  exit 1
fi

before=$(sed -n '1,/<!-- COVERAGE_START -->/p' "$README")
after=$(sed -n '/<!-- COVERAGE_END -->/,$p' "$README")

{
  echo "$before"
  echo
  cat "$COVERAGE"
  echo
  echo "$after"
} > "$TMP"

mv "$TMP" "$README"

echo "README.md updated with coverage from $COVERAGE"
