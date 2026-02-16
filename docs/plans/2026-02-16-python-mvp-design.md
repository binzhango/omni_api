# Python MVP Design: Heuristic Plan-First Transformer

**Date:** 2026-02-16  
**Source:** Brainstorming session  
**Audience:** Platform/backend engineers  
**Goal:** Deliver a deterministic Python library MVP for schema-constrained payload transformation with strict-with-warnings default behavior.

## Scope Decisions

- Heuristic-only planner (no LLM-assisted planning in MVP)
- Library API only (no CLI in MVP)
- Target schema support: JSON Schema object subset (`type`, `properties`, `required`)
- Default policy: `strict-with-warnings`

## 1. MVP Architecture

Python package layout under `python/`:

- `python/omni_api/plan_types.py`
  - Define typed structures for `TransformPlan`, mapping entries, report, warnings, and result.
- `python/omni_api/planner.py`
  - Build deterministic transform plans from source payload and target schema.
- `python/omni_api/executor.py`
  - Apply plan deterministically to produce output payload and execution warnings.
- `python/omni_api/validator.py`
  - Validate required fields and schema-key constraints.
- `python/omni_api/api.py`
  - Public entrypoint `transform(source_payload, target_schema, policy=STRICT_WITH_WARNINGS)`.
- `python/omni_api/__init__.py`
  - Export stable public API and core types.

Rationale:
- Separates planning, execution, and validation for testability and extensibility.
- Aligns with architecture principle: plan first, execute second.

## 2. Data Model and Algorithm Flow

Flow for `transform(...)`:

1. Planner stage:
   - Flatten source payload into deterministic dotted paths.
   - For each target schema property, find best source candidate by priority:
     1. exact key match
     2. normalized key match (case/underscore-insensitive)
     3. nested leaf-name match
   - Record ambiguous candidates as warnings.
   - Emit `TransformPlan` with `mappings`, `defaults` (empty in MVP), `drops`, and `required`.

2. Executor stage:
   - Apply `mappings` and populate output payload.
   - Enforce strict output key set (schema-defined keys only).
   - Track dropped/ignored source fields.
   - Emit warnings for unresolved optional keys and ambiguous mapping resolution.

3. Validator stage:
   - Verify required target keys exist.
   - Verify output keys are schema-allowed.
   - Return structured validation errors.

4. Result contract:
   - Return `TransformResult` containing:
     - `payload`
     - `plan`
     - `report` with `mapped`, `dropped`, `missing_required`, `warnings`

Supported schema subset for MVP:
- Root `type: "object"`
- `properties: {name: {type: ...}}`
- `required: [...]` optional list

## 3. Error Model, Policy, and Determinism

Default policy: `STRICT_WITH_WARNINGS`
- Output includes only schema-defined keys.
- Missing required fields are hard failures.
- Non-fatal warnings include:
  - ambiguous source matches
  - unresolved optional target keys
  - dropped unknown input fields

Determinism rules:
- Stable source candidate ordering (sorted paths).
- Stable tie-break rule:
  - shortest path first
  - lexical path order second
- No stochastic/model-assisted behavior.

Error classes:
- `TransformValidationError`: missing required fields or invalid output.
- `TransformSchemaError`: unsupported schema shape/subset violations.

## 4. Testing Strategy and Acceptance Criteria

Planned tests under `python/tests/`:

1. `test_exact_mapping.py`
   - Exact source-to-target mapping.
2. `test_nested_leaf_mapping.py`
   - Nested leaf extraction (`contact.email` -> `email`).
3. `test_strict_output_and_drops.py`
   - Extra source fields are dropped and reported.
4. `test_required_field_failure.py`
   - Missing required fields fail with structured diagnostics.
5. `test_ambiguous_mapping_warning.py`
   - Ambiguous candidates emit warning and deterministic tie-break applies.
6. `test_schema_subset_validation.py`
   - Unsupported schema shapes raise `TransformSchemaError`.

Acceptance criteria:
- Public `transform(...)` works end-to-end.
- All tests pass using `uv` workflow.
- Output and reports are deterministic across runs.
- README-style example behavior is covered by tests.

## Notes

- MVP intentionally excludes LLM planning, provider-specific adapters, and runtime HTTP behavior.
- Design preserves extension seams for future planner enhancements.
