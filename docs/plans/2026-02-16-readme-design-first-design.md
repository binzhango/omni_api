# README Design-First Structure (Platform Teams)

**Date:** 2026-02-16  
**Source:** Brainstorming session on `draft.md`  
**Audience:** Platform/backend teams standardizing internal API contracts  
**Success Criterion:** New platform engineers understand architecture and data flow in under 10 minutes

## 1. Opening and Positioning

### Purpose
Position `omni_api` as a payload adaptation layer that enforces backend contracts while absorbing upstream payload variation.

### Content
- One-paragraph problem statement:
  - Producers send heterogeneous payloads.
  - Backends require strict schemas.
  - Per-service ad hoc transforms cause drift and maintenance cost.
- Value proposition bullets:
  - Centralized transformation logic
  - Deterministic and auditable behavior
  - Provider/backend contract isolation
  - Faster onboarding for new integrations
- Explicit audience line:
  - This README is for platform/backend teams responsible for shared API boundaries.

## 2. Core Architecture Model

### Principle
`omni_api` uses a deterministic two-step model:
1. Compile a Transform Plan.
2. Execute the plan with a pure transformer.

### Why
- Repeatable outputs
- Easier debugging
- Testable transformation logic

### Component Breakdown
- Router: selects target backend profile/schema.
- Schema Loader: resolves canonical contract.
- Planner: heuristic-first, optional LLM assistance for unresolved mappings.
- Executor: applies mappings/defaults/drops.
- Validator: enforces schema and required fields.
- Diagnostics: reports what changed and why.

### Data Flow Statement
Each request yields:
- Final transformed payload
- Structured diagnostics metadata

## 3. Contracts and Canonical Artifacts

### Backend Contract Source of Truth
- Store canonical JSON Schemas under `schemas/<provider>/<endpoint>.json`.

### Transform Plan DSL
- Versioned, machine-checkable structure with:
  - `mappings`
  - `defaults`
  - `drops`
  - `required`

### Execution Guarantees
- Output keys must exist in target schema unless policy allows passthrough.
- Source paths must exist or generate diagnostics.
- Required fields are enforced before backend dispatch.

### Diagnostics Contract
- Standard report includes:
  - mapped fields
  - dropped fields
  - missing required fields
  - confidence and warnings

### Policy Knobs
- strict mode
- unknown-field handling
- fallback behavior

## 4. Runtime Behavior and Operational Expectations

### Request Lifecycle
Inbound payload -> route selection -> plan compilation -> deterministic execution -> validation -> diagnostics output.

### Planner Strategy
- Heuristic mapping first (exact/alias/type-aware).
- LLM planner only for unresolved mappings.
- Confidence-aware acceptance with review path for low-confidence cases.

### Failure Handling
- Validation failures return structured error payloads.
- Low-confidence mappings surface warnings.
- Strict mode disallows silent drops.

### Observability
- Request-level transformation report.
- Plan version in logs.
- Metrics for mapped/dropped/missing fields.

## 5. README Readability Structure

### Final Section Order
1. What `omni_api` is (problem + positioning)
2. Architecture model (plan then execute)
3. Contracts (schemas, DSL, diagnostics)
4. Runtime behavior (failure modes, observability)
5. Repo map (responsibility by directory)
6. First-read onboarding path
7. Near-term scope and explicit non-goals

### First-Read Path
- `README.md`
- `openspec/changes/phase-1/design.md`
- Relevant phase-1 spec files under `openspec/changes/phase-1/specs/`

## Notes
- This design intentionally prioritizes architecture comprehension over tutorial-style setup.
- Follow-on work should convert this design into a rewritten `README.md` and then produce an implementation plan before editing content.
