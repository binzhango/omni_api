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
    except error.HTTPError as exc:
        body = exc.read().decode("utf-8", errors="replace")
        raise AssertionError(
            f"Ollama HTTP error {exc.code} calling {url}. Response body: {body}"
        ) from exc
    except error.URLError as exc:
        raise AssertionError(f"Failed to call Ollama at {url}: {exc}") from exc


@pytest.mark.integration
def test_ollama_llama31_generate_from_transformed_source() -> None:
    if os.getenv("RUN_OLLAMA_INTEGRATION") != "1":
        pytest.skip("Set RUN_OLLAMA_INTEGRATION=1 to run Ollama integration test")

    # Exact README quickstart example input.
    source_payload = {
        "full_name": "John Doe",
        "age": 30,
        "contact": {"email": "john@example.com"},
        "extra_data": "ignored",
    }

    # Exact README quickstart target schema.
    target_schema = {
        "type": "object",
        "properties": {
            "name": {"type": "string"},
            "age": {"type": "number"},
            "email": {"type": "string"},
        },
        "required": ["name", "age", "email"],
    }

    transformed = transform(source_payload, target_schema)
    assert transformed.payload == {
        "name": "John Doe",
        "age": 30,
        "email": "john@example.com",
    }

    ollama_source_payload = {
        "model": os.getenv("OLLAMA_MODEL", "llama3.1:latest"),
        "prompt": (
            "Given this user profile in markdown-style bullet format:\\n"
            f"- name: {transformed.payload['name']}\\n"
            f"- age: {transformed.payload['age']}\\n"
            f"- email: {transformed.payload['email']}\\n"
            "Reply with exact token: OK_OMNI_API"
        ),
        "stream": False,
        "extra_data": "drop-me",
    }
    ollama_payload = to_ollama_payload(ollama_source_payload)

    print("Transformed payload:", transformed.payload, flush=True)
    print("Ollama request payload:", ollama_payload, flush=True)

    assert ollama_payload["model"] == ollama_source_payload["model"]
    assert isinstance(ollama_payload.get("prompt"), str)
    assert ollama_payload["prompt"].strip()
    assert "extra_data" not in ollama_payload

    base_url = os.getenv("OLLAMA_BASE_URL", "http://127.0.0.1:11434")
    response = _post_json(f"{base_url}/api/generate", ollama_payload)
    print("Ollama response:", response, flush=True)

    assert "error" not in response, f"Ollama returned error payload: {response}"

    assert isinstance(response.get("response"), str)
    assert response["response"].strip(), f"Empty response payload: {response}"
