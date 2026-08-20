# Security Policy

## Reporting Security Vulnerabilities

If you discover a security vulnerability in the **extension ecosystem** (e.g. an extension
that modifies the config unexpectedly, reads local paths it should not, or hangs the fetch),
please report it responsibly by contacting:

**Email:** `x@xscriptor.com`

### What to Include

When reporting a security issue, please provide:

1. **Description** — A clear explanation of the vulnerability
2. **Type** — What kind of security issue is it? (e.g., config tampering, path traversal, denial of service, supply-chain)
3. **Steps to Reproduce** — Detailed steps to trigger the vulnerability
4. **Impact** — How severe is the issue? What could an attacker do?
5. **Affected Versions** — Which extension and xfetch versions are affected?
6. **Proposed Fix** (optional) — If you have a suggestion for how to fix it

### Guidelines

- **Do not** open public GitHub issues for security vulnerabilities
- **Do not** disclose the vulnerability publicly until a fix is released
- **Do** give the maintainers reasonable time to address the issue before public disclosure
- Typically, we aim to respond within **7 days** and release a fix within **30 days** for critical issues

## Scope

Extensions are user-installed binaries (`xfetch extension install`) that act as config
providers: xfetch feeds them the config over stdin and applies the JSON they return at load
time. Anything that lets an extension step outside that contract is in scope:

- **Config tampering**: an extension must return a modified config, nothing else. Writing files,
  reading arbitrary user data or setting keys outside the config contract is out of scope for a
  provider and is a vulnerability.
- **Path handling**: extensions resolve local files (e.g. `config-roulette` routes, `~`
  expansion) — path traversal or resolution against attacker-controlled locations is in scope.
- **Availability**: every extension must wrap its work in `with_timeout`; one that can hang the
  fetch indefinitely is a denial-of-service vector.
- The `xfetch-extension-api` wire protocol and the build/install path (supply chain).
