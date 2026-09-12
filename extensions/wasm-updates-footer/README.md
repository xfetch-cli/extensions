<div align="center">
  <h1>Wasm Updates Footer Extension</h1>
  <p>Appends the pending package-update count to the footer.</p>
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
      <td><code>wasm-updates-footer.wasm</code> (core module)</td>
    </tr>
    <tr>
      <td><strong>Runtime</strong></td>
      <td><code>wasm32-wasip1</code> via <code>GOOS=wasip1</code></td>
    </tr>
    <tr>
      <td><strong>Capabilities</strong></td>
      <td><code>exec: checkupdates, pacman</code></td>
    </tr>
    <tr>
      <td><strong>Toolchain</strong></td>
      <td>Go &gt;= 1.24</td>
    </tr>
  </table>
</div>

<br>

<h2>Build</h2>

<pre><code>GOOS=wasip1 GOARCH=wasm go build -o dist/wasm-updates-footer.wasm .</code></pre>

<h2>Install</h2>

<pre><code>xfetch extension install ./extensions/wasm-updates-footer</code></pre>

<h2>Configuration</h2>

<pre><code class="language-jsonc">{
  "config_providers": [
    {
      "extension": "wasm-updates-footer",
      "args": {
        "program": "pacman",
        "args": ["-Qu"],
        "prefix": " · ",
        "up_to_date": "up to date"
      },
      "timeout_secs": 15
    }
  ]
}</code></pre>

<h3>Args</h3>

<table>
  <thead>
    <tr><th>Field</th><th>Type</th><th>Default</th><th>Description</th></tr>
  </thead>
  <tbody>
    <tr><td><code>program</code></td><td>string</td><td><code>checkupdates</code></td><td>Allowlisted program; falls back to <code>pacman -Qu</code> when missing.</td></tr>
    <tr><td><code>args</code></td><td>string[]</td><td><code>[]</code></td><td>Arguments passed verbatim to the program.</td></tr>
    <tr><td><code>prefix</code></td><td>string</td><td><code> · </code></td><td>Separator appended before the count.</td></tr>
    <tr><td><code>up_to_date</code></td><td>string</td><td><code>up to date</code></td><td>Label used when there are no pending updates.</td></tr>
  </tbody>
</table>

<h2>Output</h2>

<pre><code>footer_text: "wasm all · 12 updates"
footer_text: "wasm all · up to date"</code></pre>

<p>
  Failures stay silent so a missing tool can never break the fetch. The
  provider is idempotent: it does not append the count twice.
</p>
