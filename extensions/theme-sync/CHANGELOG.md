# Changelog

## 0.1.0

- Initial `theme-sync` config provider.
- Reads the active equisdots palette from `settings.json`, loads the matching
  `themes/<slug>.jsonc` and merges colors, plugin fallbacks and `logo_color`
  into the xfetch config.
- Skips configs that already have non-empty colors unless `force` is set, so
  it restores the palette after `config-roulette`.
- Sandboxed WebAssembly core module with read-only access to `~/.config`.
