<div align="center">
  <h1>Wasm Lang Labels Extension</h1>
  <p>Localizes module labels based on the system language.</p>
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
      <td><code>wasm-lang-labels.wasm</code> (component)</td>
    </tr>
    <tr>
      <td><strong>Runtime</strong></td>
      <td>Component model, world <code>extension</code></td>
    </tr>
    <tr>
      <td><strong>Capabilities</strong></td>
      <td><code>env: LANG, LC_ALL, LC_MESSAGES</code></td>
    </tr>
    <tr>
      <td><strong>Toolchain</strong></td>
      <td><code>componentize-py</code> &gt;= 0.25</td>
    </tr>
  </table>
</div>

<br>

<h2>Build</h2>

<pre><code>python3 -m venv .venv
. .venv/bin/activate
pip install -r requirements.txt
componentize-py -d ../../../api/wit -w extension componentize app -p . -o dist/wasm-lang-labels.wasm</code></pre>

<h2>Install</h2>

<pre><code>xfetch extension install ./extensions/wasm-lang-labels</code></pre>

<h2>Configuration</h2>

<pre><code class="language-jsonc">{
  "config_providers": [
    {
      "extension": "wasm-lang-labels",
      "args": { "language": "es" }
    }
  ]
}</code></pre>

<h3>Args</h3>

<table>
  <thead>
    <tr><th>Field</th><th>Type</th><th>Default</th><th>Description</th></tr>
  </thead>
  <tbody>
    <tr>
      <td><code>language</code></td>
      <td>string</td>
      <td>from <code>LANG</code></td>
      <td>Overrides environment detection; <code>es</code>, <code>en</code> and <code>de</code> are included.</td>
    </tr>
  </tbody>
</table>

<p>
  Existing labels are never overwritten, including intentionally hidden ones
  (empty strings), so per-module customization always wins.
</p>
