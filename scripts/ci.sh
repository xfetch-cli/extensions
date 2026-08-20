#!/usr/bin/env bash
# Local CI: run before committing (Linux / macOS).
# Enforces the extension standard: every extension must wrap its work in
# with_timeout (see CONTRIBUTING.md).
set -euo pipefail
cd "$(dirname "$0")"

echo "==> cargo fmt --check"
cargo fmt --all --check

echo "==> cargo clippy --all-targets -- -D warnings"
cargo clippy --all-targets -- -D warnings

echo "==> cargo test"
cargo test

echo "==> with_timeout standard"
for f in ../extensions/*/src/main.rs; do
  grep -q "with_timeout" "$f" || {
    echo "::error::$f must use xfetch_extension_api::with_timeout" >&2
    exit 1
  }
done
echo "All extensions use with_timeout."

echo "==> CI OK"
