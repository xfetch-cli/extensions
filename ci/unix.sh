#!/usr/bin/env bash
# CI for Linux/macOS: build, test and enforce the extension standard.
set -euo pipefail
cd "$(dirname "$0")/.."

cargo test --workspace

# Standard: every extension must wrap its work in with_timeout (CONTRIBUTING.md).
for f in extensions/*/src/main.rs; do
  grep -q "with_timeout" "$f" || {
    echo "::error::$f must use xfetch_extension_api::with_timeout" >&2
    exit 1
  }
done
echo "All extensions use with_timeout."
