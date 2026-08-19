# Changelog

## 2026-08-19

### Timeout Standard

- Both extensions wrap their work in `with_timeout` with a 2 s budget; on timeout they exit with an error instead of hanging the config load.
- An extension without a runtime limit is rejected — enforced by CI (`ci/unix.sh`, `ci/windows.ps1`, running on Linux, macOS and Windows). PRs must pass CI.
- Requires `xfetch-extension-api` with `with_timeout` (see the `api` repo).

### Extensions (as of 2026-08-19)

- `config-roulette` — picks a config from a routes list (random or daily); `~` expansion fixed on Windows (`USERPROFILE` fallback)
- `layout-override` — overrides layout and/or modules at load time

Each extension has its own CHANGELOG with its specific changes.
