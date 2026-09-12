#!/usr/bin/env bash
# Local CI for the WebAssembly extension examples.
#
# Toolchains that are missing are skipped with a notice; the script fails only
# when an available toolchain cannot build its example.
set -euo pipefail
cd "$(dirname "$0")/.."

if rustup target list --installed 2>/dev/null | grep -q '^wasm32-wasip1$'; then
  echo "==> cargo build --release --target wasm32-wasip1 (wasm-night-mode)"
  cargo build --release --target wasm32-wasip1 -p xfetch-extension-wasm-night-mode
else
  echo "==> skip Rust wasm (wasm-night-mode): wasm32-wasip1 target not installed"
fi

if command -v componentize-py >/dev/null 2>&1; then
  echo "==> componentize-py (wasm-lang-labels)"
  (
    cd extensions/wasm-lang-labels
    mkdir -p dist
    componentize-py --quiet -d ../../../api/wit -w extension componentize app -p . -o dist/wasm-lang-labels.wasm
  )
else
  echo "==> skip Python component: componentize-py not in PATH"
fi

if command -v go >/dev/null 2>&1; then
  echo "==> go build (wasm-updates-footer)"
  (
    cd extensions/wasm-updates-footer
    mkdir -p dist
    GOOS=wasip1 GOARCH=wasm go build -o dist/wasm-updates-footer.wasm .
  )
else
  echo "==> skip Go example: go not in PATH"
fi

echo "==> wasm extensions CI OK"
