# Local CI: run before committing (Windows PowerShell).
# Enforces the extension standard: every extension must wrap its work in
# with_timeout (see CONTRIBUTING.md).
$ErrorActionPreference = "Stop"
Set-Location $PSScriptRoot

Write-Host "==> cargo fmt --check"
cargo fmt --all --check
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "==> cargo clippy --all-targets -- -D warnings"
cargo clippy --all-targets -- -D warnings
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "==> cargo test"
cargo test
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "==> with_timeout standard"
foreach ($f in Get-ChildItem "..\extensions\*\src\main.rs") {
    if (-not (Select-String -Path $f.FullName -Pattern "with_timeout" -Quiet)) {
        Write-Error "$($f.FullName) must use xfetch_extension_api::with_timeout"
        exit 1
    }
}
Write-Host "All extensions use with_timeout."

Write-Host "==> CI OK"
