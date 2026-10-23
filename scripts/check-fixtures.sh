#!/usr/bin/env bash
# Asserts that the violations fixture triggers exactly the lints its markers name.
# `//~ lint` expects the lint on the marker's own line, `//~^ lint` on the line above.
# A marked lint that does not fire means the config stopped enforcing a rule; an unmarked
# lint that fires means the fixture or the config drifted. Both fail the check.
set -euo pipefail

cd "$(dirname "$0")/.."
fixture_dir="fixtures/violations/src"

expected="$(
  grep -rnE '//~\^? ' "$fixture_dir" \
    | sed -E 's|^([^:]+):([0-9]+):.*//~(\^?) (.*)$|\1 \2 \3 \4|' \
    | while read -r file line caret lints; do
        if [[ "$caret" == "^" ]]; then
          line=$((line - 1))
        else
          lints="$caret $lints"
        fi
        for lint in $lints; do
          echo "$file:$line $lint"
        done
      done \
    | sort -u
)"

actual="$(
  # Deny-level lints make clippy exit non-zero; the JSON stream is still complete.
  { cargo clippy --quiet --package violations --message-format=json 2>/dev/null || true; } \
    | jq -r '
        select(.reason == "compiler-message")
        | .message as $message
        | select($message.code != null)
        | $message.spans[]
        | select(.is_primary)
        | "\(.file_name):\(.line_start) \($message.code.code)"
      ' \
    | sort -u
)"

missing="$(comm -23 <(echo "$expected") <(echo "$actual"))"
unexpected="$(comm -13 <(echo "$expected") <(echo "$actual"))"

if [[ -n "$missing" || -n "$unexpected" ]]; then
  [[ -n "$missing" ]] && printf 'Marked lints that did not fire:\n%s\n\n' "$missing"
  [[ -n "$unexpected" ]] && printf 'Lints that fired without a marker:\n%s\n' "$unexpected"
  exit 1
fi

echo "All $(echo "$expected" | wc -l | tr -d ' ') expected lints fired, and nothing else."
