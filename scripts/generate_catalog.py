#!/usr/bin/env python3
"""Generate the machine-readable addon catalog for the extensions repository.

Scans ``extensions/*/`` and writes ``index.json`` at the repository root. Native
Rust extensions take their version and description from ``Cargo.toml``; wasm
guests take theirs from the adjacent ``xfetch-extension.json`` manifest.

Usage:
    python3 scripts/generate_catalog.py
    python3 scripts/generate_catalog.py --check

Only the Python standard library is used.
"""

from __future__ import annotations

import argparse
import difflib
import json
import sys
import tomllib
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parent.parent
EXTENSIONS_DIR = ROOT / "extensions"
OUTPUT = ROOT / "index.json"

SCHEMA_VERSION = 1
CATALOG_VERSION = "1.0.2"
WASM_RUNTIMES = {"core", "component"}


def fail(message: str) -> None:
    raise SystemExit(f"generate_catalog: {message}")


def read_toml(path: Path) -> dict[str, Any]:
    try:
        with path.open("rb") as handle:
            return tomllib.load(handle)
    except OSError as exc:
        fail(f"cannot read {path.relative_to(ROOT)}: {exc}")
    except tomllib.TOMLDecodeError as exc:
        fail(f"invalid TOML in {path.relative_to(ROOT)}: {exc}")


def read_json(path: Path) -> dict[str, Any]:
    try:
        with path.open(encoding="utf-8") as handle:
            data = json.load(handle)
    except OSError as exc:
        fail(f"cannot read {path.relative_to(ROOT)}: {exc}")
    except json.JSONDecodeError as exc:
        fail(f"invalid JSON in {path.relative_to(ROOT)}: {exc}")
    if not isinstance(data, dict):
        fail(f"{path.relative_to(ROOT)} must contain a JSON object")
    return data


def require_string(data: dict[str, Any], key: str, source: Path) -> str:
    value = data.get(key)
    if not isinstance(value, str) or not value.strip():
        fail(f"{source.relative_to(ROOT)} is missing a non-empty {key!r}")
    return value


def native_entry(directory: Path) -> dict[str, Any]:
    cargo_path = directory / "Cargo.toml"
    package = read_toml(cargo_path).get("package")
    if not isinstance(package, dict):
        fail(f"{cargo_path.relative_to(ROOT)} has no [package] table")

    name = directory.name
    return {
        "name": name,
        "description": require_string(package, "description", cargo_path),
        "version": require_string(package, "version", cargo_path),
        "kind": "config_provider",
        "runtime": "native",
        "path": f"extensions/{name}",
        "install": name,
        "capabilities": {},
    }


def wasm_entry(directory: Path, manifest_path: Path) -> dict[str, Any]:
    manifest = read_json(manifest_path)
    name = directory.name
    manifest_name = require_string(manifest, "name", manifest_path)
    if manifest_name != name:
        fail(
            f"{manifest_path.relative_to(ROOT)} declares name {manifest_name!r}, "
            f"but its directory is {name!r}"
        )

    guest_runtime = require_string(manifest, "runtime", manifest_path)
    if guest_runtime not in WASM_RUNTIMES:
        fail(
            f"{manifest_path.relative_to(ROOT)} has runtime {guest_runtime!r}; "
            f"expected one of {sorted(WASM_RUNTIMES)}"
        )

    version = require_string(manifest, "version", manifest_path)
    cargo_path = directory / "Cargo.toml"
    if cargo_path.is_file():
        package = read_toml(cargo_path).get("package")
        if not isinstance(package, dict):
            fail(f"{cargo_path.relative_to(ROOT)} has no [package] table")
        cargo_version = require_string(package, "version", cargo_path)
        if cargo_version != version:
            fail(
                f"{cargo_path.relative_to(ROOT)} version {cargo_version!r} does not "
                f"match {manifest_path.relative_to(ROOT)} version {version!r}"
            )

    capabilities = manifest.get("capabilities", {})
    if not isinstance(capabilities, dict):
        fail(f"{manifest_path.relative_to(ROOT)} capabilities must be an object")

    kind = manifest.get("kind", "config_provider")
    if not isinstance(kind, str) or not kind:
        fail(f"{manifest_path.relative_to(ROOT)} has an invalid kind")

    return {
        "name": name,
        "description": require_string(manifest, "description", manifest_path),
        "version": version,
        "kind": kind,
        "runtime": "wasm",
        "guest_runtime": guest_runtime,
        "path": f"extensions/{name}",
        "install": name,
        "capabilities": capabilities,
    }


def build_catalog() -> dict[str, Any]:
    if not EXTENSIONS_DIR.is_dir():
        fail(f"extensions directory not found at {EXTENSIONS_DIR}")

    entries: list[dict[str, Any]] = []
    for directory in sorted(EXTENSIONS_DIR.iterdir(), key=lambda path: path.name):
        if not directory.is_dir() or directory.name.startswith("."):
            continue

        manifest_path = directory / "xfetch-extension.json"
        cargo_path = directory / "Cargo.toml"
        if manifest_path.is_file():
            entries.append(wasm_entry(directory, manifest_path))
        elif cargo_path.is_file():
            entries.append(native_entry(directory))
        else:
            fail(
                f"extensions/{directory.name} has neither "
                "xfetch-extension.json nor Cargo.toml"
            )

    entries.sort(key=lambda entry: entry["name"])
    names = [entry["name"] for entry in entries]
    if len(names) != len(set(names)):
        fail(f"duplicate extension names in catalog: {names}")

    return {
        "schema_version": SCHEMA_VERSION,
        "catalog_version": CATALOG_VERSION,
        "entries": entries,
    }


def render(catalog: dict[str, Any]) -> str:
    return json.dumps(catalog, indent=2, ensure_ascii=False) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Generate or verify the extensions repository catalog."
    )
    parser.add_argument(
        "--check",
        action="store_true",
        help="fail if index.json is missing or differs from the generated catalog",
    )
    args = parser.parse_args()

    expected = render(build_catalog())

    if args.check:
        if not OUTPUT.is_file():
            print(f"generate_catalog: {OUTPUT.name} is missing", file=sys.stderr)
            return 1
        actual = OUTPUT.read_text(encoding="utf-8")
        if actual != expected:
            diff = difflib.unified_diff(
                actual.splitlines(keepends=True),
                expected.splitlines(keepends=True),
                fromfile="index.json (current)",
                tofile="index.json (generated)",
            )
            sys.stderr.writelines(diff)
            print(
                "generate_catalog: index.json is stale; "
                "run python3 scripts/generate_catalog.py",
                file=sys.stderr,
            )
            return 1
        print("index.json is up to date")
        return 0

    OUTPUT.write_text(expected, encoding="utf-8")
    print(f"wrote {OUTPUT.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
