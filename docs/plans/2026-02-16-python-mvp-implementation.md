# Python MVP Transformer Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement a deterministic Python library API for schema-constrained payload transformation using a heuristic planner and strict-with-warnings default policy.

**Architecture:** Implement a plan-first pipeline (`planner -> executor -> validator`) behind a single public `transform(...)` function. Keep MVP scope to JSON Schema object subset and deterministic matching/tie-break behavior.

**Tech Stack:** Python 3, `uv`, `pytest`, standard library (`dataclasses`, `typing`)

---

### Task 1: Initialize Python package and test harness

**Files:**
- Create: `python/pyproject.toml`
- Create: `python/omni_api/__init__.py`
- Create: `python/tests/test_smoke_import.py`

**Step 1: Write the failing test**

```python
from omni_api import transform


def test_import_transform_symbol() -> None:
    assert callable(transform)
```

**Step 2: Run test to verify it fails**

Run: `cd python && uv run pytest python/tests/test_smoke_import.py -q`  
Expected: FAIL due to missing package and/or symbol.

**Step 3: Write minimal implementation**

- Add `pyproject.toml` with package metadata and `pytest` test dependency.
- Add `python/omni_api/__init__.py` that exports placeholder `transform`.

**Step 4: Run test to verify it passes**

Run: `cd python && uv run pytest python/tests/test_smoke_import.py -q`  
Expected: PASS.

**Step 5: Commit**

```bash
git add python/pyproject.toml python/omni_api/__init__.py python/tests/test_smoke_import.py
git commit -m "build: bootstrap python package and test harness"
```

### Task 2: Add core types and schema validation errors

**Files:**
- Create: `python/omni_api/plan_types.py`
- Create: `python/omni_api/errors.py`
- Modify: `python/omni_api/__init__.py`
- Create: `python/tests/test_schema_subset_validation.py`

**Step 1: Write the failing test**

```python
import pytest

from omni_api import transform, TransformSchemaError


def test_rejects_non_object_root_schema() -> None:
    with pytest.raises(TransformSchemaError):
        transform({"name": "x"}, {"type": "array"})
```

**Step 2: Run test to verify it fails**

Run: `cd python && uv run pytest python/tests/test_schema_subset_validation.py -q`  
Expected: FAIL (error class/functionality missing).

**Step 3: Write minimal implementation**

- Add dataclasses/types for plan/result/report in `plan_types.py`.
- Add `TransformSchemaError` and `TransformValidationError` in `errors.py`.
- Add schema subset check in `transform` path for root `type == "object"` and `properties` dict.

**Step 4: Run test to verify it passes**

Run: `cd python && uv run pytest python/tests/test_schema_subset_validation.py -q`  
Expected: PASS.

**Step 5: Commit**

```bash
git add python/omni_api/plan_types.py python/omni_api/errors.py python/omni_api/__init__.py python/tests/test_schema_subset_validation.py
git commit -m "feat: add core transform types and schema subset validation"
```

### Task 3: Implement heuristic planner with deterministic matching

**Files:**
- Create: `python/omni_api/planner.py`
- Modify: `python/omni_api/plan_types.py`
- Create: `python/tests/test_exact_mapping.py`
- Create: `python/tests/test_nested_leaf_mapping.py`
- Create: `python/tests/test_ambiguous_mapping_warning.py`

**Step 1: Write the failing tests**

```python
from omni_api.planner import build_plan


def test_exact_mapping_selected() -> None:
    plan = build_plan({"name": "John"}, {"type": "object", "properties": {"name": {"type": "string"}}})
    assert any(m.to_key == "name" for m in plan.mappings)
```

```python
from omni_api.planner import build_plan


def test_nested_leaf_mapping_selected() -> None:
    plan = build_plan(
        {"contact": {"email": "a@b.com"}},
        {"type": "object", "properties": {"email": {"type": "string"}}},
    )
    assert any(m.from_path == "contact.email" and m.to_key == "email" for m in plan.mappings)
```

```python
from omni_api.planner import build_plan


def test_ambiguous_mapping_emits_warning() -> None:
    plan = build_plan(
        {"a": {"email": "x"}, "b": {"email": "y"}},
        {"type": "object", "properties": {"email": {"type": "string"}}},
    )
    assert any("ambiguous" in w.lower() for w in plan.warnings)
```

**Step 2: Run tests to verify they fail**

Run: `cd python && uv run pytest python/tests/test_exact_mapping.py python/tests/test_nested_leaf_mapping.py python/tests/test_ambiguous_mapping_warning.py -q`  
Expected: FAIL.

**Step 3: Write minimal implementation**

- Implement source-path flattening and normalized-key matching.
- Implement priority: exact -> normalized -> nested leaf.
- Implement deterministic tie-break: shortest path, then lexical order.
- Record ambiguity warnings in plan.

**Step 4: Run tests to verify they pass**

Run: `cd python && uv run pytest python/tests/test_exact_mapping.py python/tests/test_nested_leaf_mapping.py python/tests/test_ambiguous_mapping_warning.py -q`  
Expected: PASS.

**Step 5: Commit**

```bash
git add python/omni_api/planner.py python/omni_api/plan_types.py python/tests/test_exact_mapping.py python/tests/test_nested_leaf_mapping.py python/tests/test_ambiguous_mapping_warning.py
git commit -m "feat: implement deterministic heuristic planner"
```

### Task 4: Implement executor and strict-with-warnings behavior

**Files:**
- Create: `python/omni_api/executor.py`
- Create: `python/tests/test_strict_output_and_drops.py`

**Step 1: Write the failing test**

```python
from omni_api import transform


def test_extra_source_fields_are_dropped_and_reported() -> None:
    result = transform(
        {"name": "John", "age": 30, "extra": "x"},
        {"type": "object", "properties": {"name": {"type": "string"}, "age": {"type": "number"}}, "required": ["name"]},
    )
    assert "extra" not in result.payload
    assert any("extra" in dropped for dropped in result.report.dropped)
```

**Step 2: Run test to verify it fails**

Run: `cd python && uv run pytest python/tests/test_strict_output_and_drops.py -q`  
Expected: FAIL.

**Step 3: Write minimal implementation**

- Apply plan mappings to output payload.
- Enforce strict output schema keys.
- Capture dropped source paths in report.
- Merge planner/executor warnings.

**Step 4: Run test to verify it passes**

Run: `cd python && uv run pytest python/tests/test_strict_output_and_drops.py -q`  
Expected: PASS.

**Step 5: Commit**

```bash
git add python/omni_api/executor.py python/tests/test_strict_output_and_drops.py
git commit -m "feat: add strict executor with dropped-field reporting"
```

### Task 5: Implement validator and required-field failures

**Files:**
- Create: `python/omni_api/validator.py`
- Create: `python/tests/test_required_field_failure.py`
- Modify: `python/omni_api/api.py`
- Modify: `python/omni_api/__init__.py`

**Step 1: Write the failing test**

```python
import pytest

from omni_api import transform, TransformValidationError


def test_missing_required_field_raises_validation_error() -> None:
    with pytest.raises(TransformValidationError):
        transform(
            {"full_name": "John"},
            {"type": "object", "properties": {"name": {"type": "string"}, "email": {"type": "string"}}, "required": ["name", "email"]},
        )
```

**Step 2: Run test to verify it fails**

Run: `cd python && uv run pytest python/tests/test_required_field_failure.py -q`  
Expected: FAIL.

**Step 3: Write minimal implementation**

- Add validator required-key checks and output key guard.
- Raise `TransformValidationError` with missing required field details.
- Wire API orchestration through planner -> executor -> validator.

**Step 4: Run test to verify it passes**

Run: `cd python && uv run pytest python/tests/test_required_field_failure.py -q`  
Expected: PASS.

**Step 5: Commit**

```bash
git add python/omni_api/validator.py python/omni_api/api.py python/omni_api/__init__.py python/tests/test_required_field_failure.py
git commit -m "feat: add validator and required-field enforcement"
```

### Task 6: End-to-end API coverage and full verification

**Files:**
- Create: `python/tests/test_transform_end_to_end.py`
- Modify: `python/omni_api/api.py`

**Step 1: Write the failing test**

```python
from omni_api import transform


def test_end_to_end_returns_payload_plan_and_report() -> None:
    result = transform(
        {"full_name": "John Doe", "contact": {"email": "john@example.com"}, "age": 30, "extra_data": "ignore"},
        {"type": "object", "properties": {"name": {"type": "string"}, "age": {"type": "number"}, "email": {"type": "string"}}, "required": ["name", "age", "email"]},
    )
    assert result.payload == {"name": "John Doe", "age": 30, "email": "john@example.com"}
    assert result.report.missing_required == []
```

**Step 2: Run test to verify it fails**

Run: `cd python && uv run pytest python/tests/test_transform_end_to_end.py -q`  
Expected: FAIL.

**Step 3: Write minimal implementation**

- Finalize API return contract (`payload`, `plan`, `report`).
- Ensure deterministic report lists (`mapped`, `dropped`, `warnings`).
- Keep policy default as strict-with-warnings.

**Step 4: Run test to verify it passes**

Run: `cd python && uv run pytest python/tests/test_transform_end_to_end.py -q`  
Expected: PASS.

**Step 5: Commit**

```bash
git add python/omni_api/api.py python/tests/test_transform_end_to_end.py
git commit -m "feat: finalize transform API end-to-end behavior"
```

### Task 7: Run full test suite and document usage

**Files:**
- Modify: `README.md`

**Step 1: Write the failing test**

```bash
cd python && uv run pytest -q
```

Expected: at least one failure before all fixes are complete.

**Step 2: Run test to verify it fails**

Run: `cd python && uv run pytest -q`  
Expected: FAIL (before final adjustments).

**Step 3: Write minimal implementation**

- Resolve remaining test failures.
- Add short Python usage snippet to `README.md` with `transform(...)` example.

**Step 4: Run test to verify it passes**

Run: `cd python && uv run pytest -q`  
Expected: all tests PASS.

**Step 5: Commit**

```bash
git add python README.md
git commit -m "test: pass full python suite and document API usage"
```
