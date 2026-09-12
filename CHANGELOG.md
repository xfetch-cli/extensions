# Changelog

## 2026-09-12 — WebAssembly extensions

- Added `wasm-night-mode` (Rust core module): disables colors outside the daytime window and can switch the palette style.
- Added `wasm-updates-footer` (Go core module): appends the pending package-update count to the footer via allowlisted `checkupdates`/`pacman`.
- Added `wasm-lang-labels` (Python component): localizes module labels from `LANG`/`LC_ALL` without overriding custom labels.
- Added `scripts/ci-wasm.sh` for local builds of all three examples.


## 2026-08-19

### Timeout Standard

- Both extensions wrap their work in `with_timeout` with a 2 s budget; on timeout they exit with an error instead of hanging the config load.
- An extension without a runtime limit is rejected — enforced by CI (`scripts/ci.sh`, `scripts/ci.ps1`, running on Linux, macOS and Windows). PRs must pass CI.
- Requires `xfetch-extension-api` with `with_timeout` (see the `api` repo).

### Extensions (as of 2026-08-19)

- `config-roulette` — picks a config from a routes list (random or daily); `~` expansion fixed on Windows (`USERPROFILE` fallback)
- `layout-override` — overrides layout and/or modules at load time

Each extension has its own CHANGELOG with its specific changes.
