# Changelog

## 2026-09-12
- Initial release: appends the pending update count to the footer using
  allowlisted `checkupdates` (with a `pacman -Qu` fallback), implemented in
  Go with the wasm host bridge.
