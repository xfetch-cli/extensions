<h1>Contributing Extensions</h1>

<p>
  Thanks for contributing to the <strong>xfetch</strong> extension ecosystem.
  This repository contains the official config-provider extensions.
</p>

<h2>Workflow</h2>

<ol>
  <li>Fork the repository and create a feature branch.</li>
  <li>Create or update an extension directory at <code>extensions/&lt;name&gt;</code>.</li>
  <li>Run <code>cargo test --workspace</code>.</li>
  <li>
    Run the full CI locally before opening the PR:
    <code>bash ci/unix.sh</code> (Linux/macOS) or <code>./ci/windows.ps1</code>
    (Windows). The CI checks tests <strong>and</strong> the extension standard.
  </li>
  <li>Document the extension in its own <code>README.md</code> and in the repository <code>README.md</code>.</li>
  <li>
    Update the <a href="./docs/compatibility.md">platform compatibility
    table</a> with the new extension and its Linux/macOS/Windows support —
    required for every new or modified extension.
  </li>
  <li>
    Open a pull request with usage details and any required external
    dependencies. PRs that fail CI are rejected.
  </li>
</ol>

<h2>Extension Rules</h2>

<ul>
  <li>Use the binary naming convention <code>xfetch-extension-&lt;name&gt;</code>.</li>
  <li>Keep extensions focused on a single responsibility.</li>
  <li>Write errors to stderr and exit with a non-zero status on failure.</li>
  <li>Prefer stable, actively maintained dependencies and keep them minimal.</li>
  <li>
    <strong>Every extension MUST have a runtime limit.</strong> Wrap all work
    in <code>with_timeout</code> (from <code>xfetch_extension_api</code>)
    with a <code>const BUDGET</code> that fits the extension and exit with an
    error when the budget elapses. An extension without a timeout is
    rejected: it could hang the config load forever. This is enforced by CI.
  </li>
</ul>

<h2>Protocol Guide</h2>

<p>
  The full stdin/stdout JSON protocol is defined in the
  <a href="https://github.com/xfetch-cli/api">xfetch-cli/api</a> repository
  (<code>crates/extension-api</code>).
</p>
