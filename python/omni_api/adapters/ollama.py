from __future__ import annotations

from typing import Any

PASSTHROUGH_KEYS = ("stream", "format", "options")


def to_ollama_payload(source_payload: dict[str, Any]) -> dict[str, Any]:
    if "messages" in source_payload:
        payload: dict[str, Any] = {
            "model": source_payload.get("model"),
            "messages": source_payload["messages"],
        }
        for key in PASSTHROUGH_KEYS:
            if key in source_payload:
                payload[key] = source_payload[key]
        return payload

    return dict(source_payload)
