#!/usr/bin/env bash
# Formats fixtures/rustfmt/input.rs with the repository's rustfmt.toml on nightly rustfmt and
# compares the result with fixtures/rustfmt/expected.rs.
set -euo pipefail

cd "$(dirname "$0")/.."
work_dir="$(mktemp -d)"
trap 'rm -rf "$work_dir"' EXIT

cp fixtures/rustfmt/input.rs "$work_dir/input.rs"
rustfmt +nightly --edition 2024 --config-path rustfmt.toml "$work_dir/input.rs"
diff -u fixtures/rustfmt/expected.rs "$work_dir/input.rs"
echo "rustfmt output matches fixtures/rustfmt/expected.rs."
