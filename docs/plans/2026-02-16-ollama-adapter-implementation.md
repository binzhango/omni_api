# Ollama Adapter v1 Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement a local Ollama adapter utility that supports chat and generate payload modes, and verify behavior with focused tests.

**Architecture:** Add a dedicated adapter module (`adapters/ollama.py`) with explicit validation rules and deterministic mode selection. Keep core `transform(...)` pipeline unchanged, and expose adapter at package boundary for local use.

**Tech Stack:** Python 3, `uv`, `pytest`

---

### Task 1: Add adapter package skeleton and export wiring

**Files:**
- Create: `python/omni_api/adapters/__init__.py`
- Modify: `python/omni_api/__init__.py`

**Step 1: Write the failing test**

```python
from omni_api import to_ollama_payload


def test_adapter_symbol_is_exported() -> None:
    assert callable(to_ollama_payload)
```

**Step 2: Run test to verify it fails**

Run: `cd python && uv run pytest tests/test_ollama_adapter.py::test_adapter_symbol_is_exported -q`  
Expected: FAIL due to missing export/function.

**Step 3: Write minimal implementation**

- Create adapter package init with placeholder export.
- Update `omni_api/__init__.py` to export `to_ollama_payload`.

**Step 4: Run test to verify it passes**

Run: `cd python && uv run pytest tests/test_ollama_adapter.py::test_adapter_symbol_is_exported -q`  
Expected: PASS.

**Step 5: Commit**

```bash
git add python/omni_api/adapters/__init__.py python/omni_api/__init__.py python/tests/test_ollama_adapter.py
git commit -m "feat: add ollama adapter export wiring"
```

### Task 2: Implement chat-mode adapter behavior

**Files:**
- Create: `python/omni_api/adapters/ollama.py`
- Modify: `python/omni_api/adapters/__init__.py`
- Modify: `python/tests/test_ollama_adapter.py`

**Step 1: Write the failing test**

```python
from omni_api import to_ollama_payload


def test_chat_payload_passthrough_messages() -> None:
    result = to_ollama_payload(
        {
            "model": "llama3",
            "messages": [{"role": "user", "content": "hello"}],
            "stream": False,
        }
    )
    assert result["model"] == "llama3"
    assert result["messages"] == [{"role": "user", "content": "hello"}]
    assert result["stream"] is False
```

**Step 2: Run test to verify it fails**

Run: `cd python && uv run pytest tests/test_ollama_adapter.py::test_chat_payload_passthrough_messages -q`  
Expected: FAIL.

**Step 3: Write minimal implementation**

- Implement `to_ollama_payload` chat path:
  - require `model`
  - if `messages` present, return chat payload with optional passthrough (`stream`, `format`, `options`)

**Step 4: Run test to verify it passes**

Run: `cd python && uv run pytest tests/test_ollama_adapter.py::test_chat_payload_passthrough_messages -q`  
Expected: PASS.

**Step 5: Commit**

```bash
git add python/omni_api/adapters/ollama.py python/omni_api/adapters/__init__.py python/tests/test_ollama_adapter.py
git commit -m "feat: implement ollama chat payload adapter"
```

### Task 3: Implement generate-mode and precedence behavior

**Files:**
- Modify: `python/omni_api/adapters/ollama.py`
- Modify: `python/tests/test_ollama_adapter.py`

**Step 1: Write the failing tests**

```python
from omni_api import to_ollama_payload


def test_generate_payload_passthrough_prompt() -> None:
    result = to_ollama_payload({"model": "llama3", "prompt": "hello"})
    assert result == {"model": "llama3", "prompt": "hello"}


def test_chat_takes_precedence_when_both_present() -> None:
    result = to_ollama_payload(
        {
            "model": "llama3",
            "messages": [{"role": "user", "content": "chat"}],
            "prompt": "ignored prompt",
        }
    )
    assert "messages" in result
    assert "prompt" not in result
```

**Step 2: Run tests to verify they fail**

Run: `cd python && uv run pytest tests/test_ollama_adapter.py::test_generate_payload_passthrough_prompt tests/test_ollama_adapter.py::test_chat_takes_precedence_when_both_present -q`  
Expected: FAIL.

**Step 3: Write minimal implementation**

- Add generate path (`prompt`) when `messages` absent.
- Ensure chat path precedence when both keys are present.

**Step 4: Run tests to verify they pass**

Run: `cd python && uv run pytest tests/test_ollama_adapter.py::test_generate_payload_passthrough_prompt tests/test_ollama_adapter.py::test_chat_takes_precedence_when_both_present -q`  
Expected: PASS.

**Step 5: Commit**

```bash
git add python/omni_api/adapters/ollama.py python/tests/test_ollama_adapter.py
git commit -m "feat: add ollama generate mode and chat precedence"
```

### Task 4: Add validation-error tests and enforce failures

**Files:**
- Modify: `python/omni_api/adapters/ollama.py`
- Modify: `python/tests/test_ollama_adapter.py`

**Step 1: Write the failing tests**

```python
import pytest

from omni_api import TransformValidationError, to_ollama_payload


def test_missing_model_raises_validation_error() -> None:
    with pytest.raises(TransformValidationError):
        to_ollama_payload({"messages": [{"role": "user", "content": "x"}]})


def test_missing_messages_and_prompt_raises_validation_error() -> None:
    with pytest.raises(TransformValidationError):
        to_ollama_payload({"model": "llama3"})
```

**Step 2: Run tests to verify they fail**

Run: `cd python && uv run pytest tests/test_ollama_adapter.py::test_missing_model_raises_validation_error tests/test_ollama_adapter.py::test_missing_messages_and_prompt_raises_validation_error -q`  
Expected: FAIL.

**Step 3: Write minimal implementation**

- Raise `TransformValidationError` for missing `model`.
- Raise `TransformValidationError` when neither `messages` nor `prompt` is present.

**Step 4: Run tests to verify they pass**

Run: `cd python && uv run pytest tests/test_ollama_adapter.py::test_missing_model_raises_validation_error tests/test_ollama_adapter.py::test_missing_messages_and_prompt_raises_validation_error -q`  
Expected: PASS.

**Step 5: Commit**

```bash
git add python/omni_api/adapters/ollama.py python/tests/test_ollama_adapter.py
git commit -m "feat: enforce ollama adapter input validation"
```

### Task 5: Full adapter test run and suite verification

**Files:**
- Verify: `python/tests/test_ollama_adapter.py`
- Verify: `python/`

**Step 1: Write the failing test**

```bash
cd python && uv run pytest tests/test_ollama_adapter.py -q
```

Expected: may fail before all prior tasks complete.

**Step 2: Run test to verify it fails**

Run: `cd python && uv run pytest tests/test_ollama_adapter.py -q`  
Expected: FAIL before completion.

**Step 3: Write minimal implementation**

- Resolve any remaining adapter/test issues.

**Step 4: Run test to verify it passes**

Run: `cd python && uv run pytest tests/test_ollama_adapter.py -q && uv run pytest -q`  
Expected: Ollama tests pass and full suite remains green.

**Step 5: Commit**

```bash
git add python/omni_api python/tests/test_ollama_adapter.py python/uv.lock
git commit -m "test: add ollama adapter coverage and verify full suite"
```
