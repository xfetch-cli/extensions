# CI for Windows: build, test and enforce the extension standard.
$ErrorActionPreference = "Stop"
Set-Location (Join-Path $PSScriptRoot "..")

cargo test --workspace

# Standard: every extension must wrap its work in with_timeout (CONTRIBUTING.md).
foreach ($f in Get-ChildItem "extensions\*\src\main.rs") {
    if (-not (Select-String -Path $f.FullName -Pattern "with_timeout" -Quiet)) {
        Write-Error "$($f.FullName) must use xfetch_extension_api::with_timeout"
        exit 1
    }
}
Write-Host "All extensions use with_timeout."
