#!/usr/bin/env bash
# Checks that deny.toml passes the workspace and rejects the fixture that depends on `ctor`.
set -euo pipefail

cd "$(dirname "$0")/.."

cargo deny --config deny.toml check bans

if cargo deny --manifest-path fixtures/deny/Cargo.toml --config deny.toml check bans 2>/dev/null; then
  echo "deny.toml accepted fixtures/deny, which depends on the banned crate ctor." >&2
  exit 1
fi
echo "deny.toml rejects fixtures/deny as expected."
