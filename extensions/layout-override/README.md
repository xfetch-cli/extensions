# Layout Override Extension

Forces a specific layout and/or module set at config load time, regardless of
what the config or theme file specifies. Runs after theme merge but before rendering.

## Install

```bash
cp target/release/xfetch-extension-layout-override ~/.config/xfetch/extensions/
```

## Usage

Add a `config_providers` entry to your xfetch config:

```jsonc
{
  "layout": "pacman",
  "modules": ["os", "kernel", "uptime"],
  "config_providers": [
    {
      "extension": "layout-override",
      "args": {
        "layout": "tree",
        "modules": ["os", "kernel", "uptime", "packages", "shell", "cpu", "memory", "palette"]
      }
    }
  ]
}
```

In this example, the config says `"layout": "pacman"`, but the extension
overrides it to `"tree"` before rendering.

### Args

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `layout` | `string` | — | Layout to force (`default`, `tree`, `pacman`, `compact`, `box`, ...) |
| `modules` | `string[]` | — | Module list to replace |

If a field is omitted, the original value from the config is preserved.

## Use Cases

- Force a specific layout when using a theme that doesn't specify one
- Temporarily override modules without editing the config file
- Chain with other extensions — let one extension randomize, another normalize the layout

## Protocol

See [extensions/README.md](../../README.md#authoring-extensions) for the protocol spec.
