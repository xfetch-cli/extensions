<h1 align="center">
<img src="https://raw.githubusercontent.com/xfetch-cli/assets/main/logo/banner/xfetch.svg" width="30%" alt="XFetch banner" />Extensions</h1>

<p>
  Official extensions for <strong>xfetch</strong> live in this repository. Extensions are
  standalone binaries that hook into the xfetch lifecycle — they can modify the
  configuration before rendering, change layouts, randomize themes, and more.
</p>

<p>
  Unlike plugins (which provide info lines or animate logos), extensions operate at
  the config level: they receive the full resolved configuration via stdin and return
  a modified version via stdout. This allows them to alter layouts, modules, colors,
  icons, logos, or even load an entirely different config file.
</p>

<h2>Install an Extension</h2>

<p>Copy the binary to the extensions directory:</p>

<pre><code>cp xfetch-extension-&lt;name&gt; ~/.config/xfetch/extensions/</code></pre>

<p>Then add it to your xfetch config:</p>

```jsonc
{
  "config_providers": [
    {
      "extension": "&lt;name&gt;",
      "args": { ... }
    }
  ]
}
```

<p>Extensions run in declaration order, after the theme merge.</p>

<h2>Available Extensions</h2>

<table>
  <thead>
    <tr>
      <th>Extension</th>
      <th>Description</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><code>layout-override</code></td>
      <td>Overrides the layout and/or modules at load time.</td>
    </tr>
    <tr>
      <td><code>config-roulette</code></td>
      <td>Picks a random (or daily) config from a list of paths — shows a different look every time.</td>
    </tr>
  </tbody>
</table>

<p>
  See <a href="./docs/compatibility.md">Platform Compatibility</a> for which
  extensions work on Linux, macOS and Windows.
</p>

<h2>Developing Locally</h2>

<p>
  This repository is a Cargo workspace. Build every extension together:
</p>

<pre><code>cargo build --release</code></pre>

<p>
  Extension implementations are grouped under <code>extensions/&lt;name&gt;</code>,
  keeping the root clean as the ecosystem grows.
</p>

<p>
  The shared wire protocol used by the core and extensions is maintained in
  <a href="https://github.com/xfetch-cli/api">xfetch-cli/api</a>.
</p>

<h2>Installed Binary Directory</h2>

<ul>
  <li><strong>Linux/macOS:</strong> <code>~/.config/xfetch/extensions/</code></li>
  <li><strong>Windows:</strong> <code>%APPDATA%/xfetch/extensions/</code></li>
</ul>

<h2>Authoring Extensions</h2>

<p>
  Each extension is a binary named <code>xfetch-extension-&lt;name&gt;</code>
  (or <code>xfetch-extension-&lt;name&gt;.exe</code> on Windows).
</p>

<p>
  The protocol uses stdin/stdout JSON:
</p>

<h3>Request</h3>

```json
{
  "version": 1,
  "kind": "config_provider",
  "config": { ... },
  "args": { ... }
}
```

<h3>Response</h3>

```json
{
  "config": { ... }
}
```

<p>
  The extension receives the fully resolved xfetch configuration (after applying
  defaults, config file, and theme), modifies the fields it cares about, and
  returns the entire config object. Unchanged fields are preserved.
</p>

<p>
  Errors should be printed to stderr and the process should exit with a non-zero status.
</p>
