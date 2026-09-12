"""wasm-lang-labels: localizes module labels based on the system language.

A WebAssembly config provider (component model). It reads `LANG`/`LC_ALL`
from the sandboxed environment and fills the config `labels` map with
translations, without overwriting labels the user already customized.
"""

import json
import os

from wit_world.imports import host

TRANSLATIONS = {
    "es": {
        "os": "sistema",
        "kernel": "nucleo",
        "hostname": "equipo",
        "packages": "paquetes",
        "shell": "shell",
        "wm": "wm",
        "terminal": "terminal",
        "memory": "memoria",
        "disk": "disco",
        "battery": "bateria",
        "uptime": "uptime",
        "user": "usuario",
        "datetime": "fecha",
        "local_ip": "ip local",
        "cpu": "cpu",
        "gpu": "gpu",
    },
    "en": {
        "os": "system",
        "kernel": "kernel",
        "hostname": "host",
        "packages": "packages",
        "memory": "memory",
        "disk": "disk",
        "battery": "battery",
        "user": "user",
        "datetime": "date",
        "local_ip": "local ip",
    },
    "de": {
        "os": "system",
        "kernel": "kernel",
        "hostname": "rechner",
        "packages": "pakete",
        "memory": "speicher",
        "disk": "festplatte",
        "battery": "akku",
        "user": "benutzer",
        "datetime": "datum",
        "local_ip": "lokale ip",
    },
}


class WitWorld:
    """Config provider: one JSON request in, one JSON response out."""

    def run(self, request: str) -> str:
        payload = json.loads(request)
        config = payload.get("config") or {}
        args = payload.get("args") or {}

        language = args.get("language") or detect_language()
        table = TRANSLATIONS.get(language[:2].lower())
        if not table:
            return json.dumps({"config": config})

        host.log("info", f"wasm-lang-labels: applying '{language[:2]}' labels")
        labels = config.get("labels")
        if not isinstance(labels, dict):
            labels = {}
            config["labels"] = labels

        # Never override explicit user labels (including intentionally
        # hidden ones, represented as empty strings).
        for module, label in table.items():
            labels.setdefault(module, label)

        return json.dumps({"config": config})


def detect_language() -> str:
    for name in ("LC_ALL", "LC_MESSAGES", "LANG"):
        value = os.environ.get(name)
        if value:
            return value
    return "en"
