<div align="center">
  <h1>Wasm Night Mode Extension</h1>
  <p>Dims the fetch outside the daytime window.</p>
</div>

<br>

<div align="center">
  <table>
    <tr>
      <td><strong>Kind</strong></td>
      <td><code>config_provider</code></td>
    </tr>
    <tr>
      <td><strong>Artifact</strong></td>
      <td><code>xfetch-extension-wasm-night-mode.wasm</code> (core module)</td>
    </tr>
    <tr>
      <td><strong>Runtime</strong></td>
      <td><code>wasm32-wasip1</code></td>
    </tr>
    <tr>
      <td><strong>Capabilities</strong></td>
      <td>none (reads the WASI clock)</td>
    </tr>
  </table>
</div>

<br>

<h2>Build</h2>

<pre><code>rustup target add wasm32-wasip1
cargo build --release --target wasm32-wasip1 -p xfetch-extension-wasm-night-mode</code></pre>

<h2>Install</h2>

<pre><code>xfetch extension install ./extensions/wasm-night-mode</code></pre>

<h2>Configuration</h2>

<pre><code class="language-jsonc">{
  "config_providers": [
    {
      "extension": "wasm-night-mode",
      "args": {
        "utc_offset": 2,
        "night_start": 22,
        "night_end": 7,
        "dim_colors": true,
        "night_palette": "dots"
      }
    }
  ]
}</code></pre>

<h3>Args</h3>

<table>
  <thead>
    <tr><th>Field</th><th>Type</th><th>Default</th><th>Description</th></tr>
  </thead>
  <tbody>
    <tr><td><code>night_start</code></td><td>number</td><td><code>22</code></td><td>First hour (inclusive) of the night window.</td></tr>
    <tr><td><code>night_end</code></td><td>number</td><td><code>7</code></td><td>First hour of the day window; windows may wrap midnight.</td></tr>
    <tr><td><code>utc_offset</code></td><td>number</td><td><code>0</code></td><td>Hours from UTC, used instead of a timezone database.</td></tr>
    <tr><td><code>dim_colors</code></td><td>bool</td><td><code>true</code></td><td>Sets <code>show_colors: false</code> at night.</td></tr>
    <tr><td><code>night_palette</code></td><td>string</td><td>unchanged</td><td>Optional <code>palette_style</code> applied at night.</td></tr>
  </tbody>
</table>

<p>
  During the day the config is returned untouched.
</p>
