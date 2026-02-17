from __future__ import annotations

import os
from typing import Any

import pytest

from omni_api import transform


@pytest.mark.integration
def test_ollama_llm_alignment_respects_target_schema() -> None:
    if os.getenv("RUN_OLLAMA_INTEGRATION") != "1":
        pytest.skip("Set RUN_OLLAMA_INTEGRATION=1 to run Ollama integration test")

    source_payload = {
        "user": {
            "full": "John Doe",
            "years_old": 30,
            "mail": "john@example.com",
        },
        "metadata": {"region": "us", "tier": "pro"},
        "extra": "ignore-me",
    }

    target_schema: dict[str, Any] = {
        "type": "object",
        "properties": {
            "name": {"type": "string"},
            "age": {"type": "number"},
            "email": {"type": "string"},
        },
        "required": ["name", "age", "email"],
    }

    result = transform(
        source_payload,
        target_schema,
        llm_provider="ollama",
        llm_model=os.getenv("OLLAMA_MODEL", "llama3.1:latest"),
        llm_base_url=os.getenv("OLLAMA_BASE_URL", "http://127.0.0.1:11434"),
    )

    payload = result.payload
    print("LLM-aligned payload:", payload, flush=True)

    # Required fields present.
    for key in target_schema["required"]:
        assert key in payload, f"missing required field {key}"

    # Types are reasonable.
    assert isinstance(payload["name"], str) and payload["name"].strip()
    assert isinstance(payload["email"], str) and "@" in payload["email"]
    assert isinstance(payload["age"], (int, float))

    # No extra keys outside target schema.
    allowed = set(target_schema["properties"].keys())
    assert set(payload.keys()).issubset(allowed)
