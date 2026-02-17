from __future__ import annotations

from typing import Any

import omni_api.api as api_module
from omni_api import transform


def test_transform_uses_ollama_llm_when_provider_selected(monkeypatch) -> None:
    monkeypatch.setattr(api_module, "_ollama_generate", lambda *args, **kwargs: {
        "name": "Jane Doe",
        "age": 28,
        "email": "jane@example.com",
    })

    result = transform(
        {"random": "input", "nested": {"x": 1}},
        {
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "number"},
                "email": {"type": "string"},
            },
            "required": ["name", "age", "email"],
        },
        llm_provider="ollama",
        llm_model="llama3.1:latest",
    )

    assert result.payload == {
        "name": "Jane Doe",
        "age": 28,
        "email": "jane@example.com",
    }
