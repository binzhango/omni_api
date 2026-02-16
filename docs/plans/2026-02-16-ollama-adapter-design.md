# Ollama Adapter v1 Design

**Date:** 2026-02-16  
**Audience:** Local developers validating provider-specific payload conversion  
**Goal:** Add a dedicated Ollama adapter utility with direct tests for chat and generate payload shapes.

## Approved Decisions

- Implement a dedicated adapter module, not inline logic in `transform(...)`.
- Support both chat-style and generate-style payloads.
- Preserve `messages` payload as chat-style when present; do not collapse messages into prompt.
- Prefer explicit adapter utility for local verification.

## 1. API and Module Layout

Add:
- `python/omni_api/adapters/ollama.py`
  - `to_ollama_payload(source_payload: dict[str, Any]) -> dict[str, Any]`
- `python/omni_api/adapters/__init__.py`
  - export `to_ollama_payload`
- `python/omni_api/__init__.py`
  - re-export `to_ollama_payload`

Existing `transform(...)` remains unchanged in this iteration.

## 2. Adapter Behavior Rules

Input contract:
- Requires `model` (string-like presence check in v1).
- Supports either:
  - `messages` (chat-style), or
  - `prompt` (generate-style).

Selection behavior:
- If `messages` is present, output chat payload shape (`model`, `messages`, optional passthrough keys).
- Else if `prompt` is present, output generate payload shape (`model`, `prompt`, optional passthrough keys).
- If both exist, choose chat behavior (`messages` path).

Validation failures:
- Missing `model` -> raise `TransformValidationError`.
- Missing both `messages` and `prompt` -> raise `TransformValidationError`.

Optional passthrough keys (if present):
- `stream`
- `format`
- `options`

## 3. Test Coverage

Add `python/tests/test_ollama_adapter.py`:

1. `test_chat_payload_passthrough_messages`
- verifies chat input yields chat output.

2. `test_generate_payload_passthrough_prompt`
- verifies generate input yields generate output.

3. `test_chat_takes_precedence_when_both_present`
- verifies `messages` path is selected when both fields exist.

4. `test_missing_model_raises_validation_error`
- verifies missing model raises `TransformValidationError`.

5. `test_missing_messages_and_prompt_raises_validation_error`
- verifies missing content input raises `TransformValidationError`.

## Notes

- This is adapter-only behavior for local functionality checks.
- It does not yet introduce a provider registry or routing integration.
