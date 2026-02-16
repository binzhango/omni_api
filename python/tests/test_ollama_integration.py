from __future__ import annotations

import json
import os
from typing import Any
from urllib import error, request

import pytest

from omni_api import to_ollama_payload, transform


def _post_json(url: str, payload: dict[str, Any], timeout: float = 60.0) -> dict[str, Any]:
    body = json.dumps(payload).encode("utf-8")
    req = request.Request(
        url,
        data=body,
        headers={"Content-Type": "application/json"},
        method="POST",
    )
    try:
        with request.urlopen(req, timeout=timeout) as resp:
            return json.loads(resp.read().decode("utf-8"))
    except error.URLError as exc:
        raise AssertionError(f"Failed to call Ollama at {url}: {exc}") from exc


@pytest.mark.integration
def test_ollama_llama31_generate_from_transformed_source() -> None:
    if os.getenv("RUN_OLLAMA_INTEGRATION") != "1":
        pytest.skip("Set RUN_OLLAMA_INTEGRATION=1 to run Ollama integration test")

    source_payload = {
        "model_name": os.getenv("OLLAMA_MODEL", "llama3.1:latest"),
        "user_prompt": "Reply with exact token: OK_OMNI_API",
        "stream": False,
        "extra_data": "drop-me",
    }

    target_schema = {
        "type": "object",
        "properties": {
            "model": {"type": "string"},
            "prompt": {"type": "string"},
            "stream": {"type": "boolean"},
        },
        "required": ["model", "prompt"],
    }

    transformed = transform(source_payload, target_schema)
    ollama_payload = to_ollama_payload(transformed.payload)

    assert ollama_payload["model"] == source_payload["model_name"]
    assert "prompt" in ollama_payload
    assert "extra_data" not in ollama_payload

    base_url = os.getenv("OLLAMA_BASE_URL", "http://127.0.0.1:11434")
    response = _post_json(f"{base_url}/api/generate", ollama_payload)

    assert isinstance(response.get("response"), str)
    assert response["response"].strip()
    assert "OK_OMNI_API" in response["response"]
