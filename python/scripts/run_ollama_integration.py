from __future__ import annotations

import json
import os
from typing import Any
from urllib import error, request

from omni_api import to_ollama_payload, transform


def post_json(url: str, payload: dict[str, Any], timeout: float = 60.0) -> dict[str, Any]:
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
        response_body = exc.read().decode("utf-8", errors="replace")
        raise RuntimeError(
            f"Ollama HTTP error {exc.code} at {url}. Response: {response_body}"
        ) from exc
    except error.URLError as exc:
        raise RuntimeError(f"Failed to call Ollama at {url}: {exc}") from exc


def main() -> int:
    source_payload = {
        "full_name": "John Doe",
        "age": 30,
        "contact": {"email": "john@example.com"},
        "extra_data": "ignored",
    }

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
    ollama_source_payload = {
        "model": os.getenv("OLLAMA_MODEL", "llama3.1:latest"),
        "prompt": (
            "Given this user profile in markdown-style bullet format:\n"
            f"- name: {transformed.payload['name']}\n"
            f"- age: {transformed.payload['age']}\n"
            f"- email: {transformed.payload['email']}\n"
            "Reply with exact token: OK_OMNI_API"
        ),
        "stream": False,
    }
    ollama_payload = to_ollama_payload(ollama_source_payload)

    print("Transformed payload:")
    print(json.dumps(transformed.payload, indent=2), flush=True)
    print("Ollama request payload:")
    print(json.dumps(ollama_payload, indent=2), flush=True)

    base_url = os.getenv("OLLAMA_BASE_URL", "http://127.0.0.1:11434")
    response = post_json(f"{base_url}/api/generate", ollama_payload)

    print("Ollama response:")
    print(json.dumps(response, indent=2), flush=True)

    if "error" in response:
        raise RuntimeError(f"Ollama returned error payload: {response}")
    if not isinstance(response.get("response"), str) or not response["response"].strip():
        raise RuntimeError(f"Ollama returned empty response: {response}")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
