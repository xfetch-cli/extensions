# Config Roulette Extension

Picks a random (or daily) configuration from a list of paths and loads it.
Every invocation of `xfetch` can show a completely different look — different
layout, logo, theme, colors, modules, and icons.

## Install

```bash
cp target/release/xfetch-extension-config-roulette ~/.config/xfetch/extensions/
```

## Usage

### 1. Create a routes file

```json
[
  { "_name": "default-full",   "path": "~/.config/xfetch/fetchs/001-layout-default/config.jsonc" },
  { "_name": "tree-view",      "path": "~/.config/xfetch/fetchs/004-layout-tree/config.jsonc" },
  { "_name": "pacman-classic",  "path": "~/.config/xfetch/fetchs/008-layout-pacman/config.jsonc" }
]
```

### 2. Add to your xfetch config

```jsonc
{
  "config_providers": [
    {
      "extension": "config-roulette",
      "args": {
        "routes": "~/.config/xfetch/routes.json",
        "strategy": "random"
      }
    }
  ]
}
```

Now every `xfetch` invocation loads a different config from the list.

### Args

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `routes` | `string` | `~/.config/xfetch/routes.json` | Path to the JSON routes file |
| `strategy` | `"random"` or `"daily"` | `"daily"` | Selection strategy |

### Strategies

- **`random`** — picks a different config on every execution (uses sub-second timer as seed)
- **`daily`** — picks the same config all day, changes the next day (uses date as seed)

## Routes File Format

The routes file is a JSON array of objects:

```json
[
  { "_name": "route-name", "path": "~/.config/xfetch/path/to/config.jsonc" },
  { "_name": "another-one", "path": "~/.config/xfetch/path/to/another.jsonc" }
]
```

Each route points to a complete xfetch config file. The `_name` field is optional
and only used for readability. You can have as many routes as you want.

## Use Cases

- See a different visual style every time you open a terminal
- Test all your configs automatically without running them manually
- Daily themes — same look all day, new look tomorrow
- Rotate through 300+ configs from the test suite

## Protocol

See [extensions/README.md](../../README.md#authoring-extensions) for the protocol spec.
