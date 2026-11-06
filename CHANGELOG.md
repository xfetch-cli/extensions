# Changelog

## 2026-11-06 — SDK 0.3.1 migration

- Updated the workspace to `xfetch-extension-api` 0.3.1.
- The JSON protocol remains version 1 and the WIT remains
  `xfetch:runtime@0.1.0`.
- Extension code, manifests and catalog versions are unchanged.

## 2026-11-06 — CI and wasm validation

- Added a `Tests` workflow: fmt, clippy, workspace tests and catalog
  verification on pull requests and on main after merge. It does not build or
  publish release artifacts.
- Validated `wasm-night-mode`, `wasm-lang-labels` and `wasm-updates-footer`
  with their native wasm toolchains and runtime checks against xfetch 1.0.3.

## 2026-11-06 — SDK 0.3.0 migration

- Updated the workspace to `xfetch-extension-api` 0.3.0.
- The JSON protocol remains version 1 and the WIT remains
  `xfetch:runtime@0.1.0`.
- Extension code, manifests and catalog versions are unchanged; this is a
  dependency alignment with the core 0.3.0 SDK.

## 2026-11-03 — v1.0.2

Release train v1.0.2 (catalog schema 1, catalog_version 1.0.2).

### Dependency updates

- Updated the workspace lockfile with semver-compatible patches: `serde_json` 1.0.154, `syn` 3.0.9 and `unicode-ident` 1.0.27.
- Updated the security-audit workflow to `actions/checkout` 7.0.1 and the pinned `dtolnay/rust-toolchain` commit.
- No extension code or manifests changed; crate versions remain 0.2.0. The catalog index is unchanged.

### Addon catalog index

- Added root `index.json`, a machine-readable catalog of the five installable extensions (`config-roulette`, `layout-override`, `wasm-night-mode`, `wasm-lang-labels`, `wasm-updates-footer`), consumed by `xfetch extension search`.
- Native Rust extensions publish their Cargo version and description; wasm guests publish the metadata from their adjacent `xfetch-extension.json`, with `guest_runtime` set to `core` or `component`.
- Added `scripts/generate_catalog.py`, a standard-library-only deterministic generator with `--check` support for CI and local verification.
- No extension code or manifests changed. Crate versions remain `0.2.0`.

## 2026-10-24 — Dependency security automation

- Added `.github/workflows/security-audit.yml`: runs `cargo audit` 0.22.2 with `--deny warnings` on push, pull request and a weekly schedule, with actions pinned by commit SHA and no persisted checkout credentials.
- Added weekly Dependabot updates for Cargo dependencies and GitHub Actions (`open-pull-requests-limit: 5`, minor/patch grouped).
- No extension code or assets changed. The current lockfile audits clean (15 crates, 0 vulnerabilities, 0 warnings).

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
