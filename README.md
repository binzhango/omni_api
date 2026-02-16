# omni_api

`omni_api` is a payload adaptation layer for platform teams that need to enforce backend API contracts while supporting heterogeneous upstream producers.

Upstream systems emit varied payload shapes, while backend services require strict and stable contracts. `omni_api` centralizes payload normalization so contract enforcement does not sprawl across services.

## Opening and Positioning

`omni_api` is intended for platform/backend teams responsible for shared API boundaries.

It provides:
- Centralized transformation logic
- Deterministic and auditable behavior
- Isolation between producer payloads and backend contracts
- Faster onboarding for new integrations

## Core Architecture Model

`omni_api` follows a deterministic two-step model:
1. Compile a Transform Plan from source payload and target contract.
2. Execute the plan with a pure deterministic transformer.

This separation improves replayability, testability, and root-cause debugging.

Core components:
- Router: selects the target backend route/profile
- Schema Loader: resolves canonical contract
- Planner: creates Transform Plan output (heuristic-first)
- Executor: applies plan instructions
- Validator: checks contract conformance and required fields
- Diagnostics: emits structured transformation evidence

## Contracts and Canonical Artifacts

Backend contracts are the source of truth and should be maintained as JSON Schema files under `schemas/<provider>/<endpoint>.json`.

The Transform Plan DSL is versioned and includes:
- `mappings`
- `defaults`
- `drops`
- `required`

Execution guarantees:
- Output keys stay within target JSON Schema unless policy allows passthrough
- Missing source paths are captured in diagnostics
- Required fields are enforced before backend dispatch

Diagnostics contract includes:
- mapped fields
- dropped fields
- missing required fields
- warnings and confidence

Policy controls:
- strict mode
- unknown-field handling
- fallback behavior

## Runtime Behavior and Operational Expectations

- Runtime flow: ingress -> routing -> planning -> execution -> validation -> diagnostics.
- Failure handling and observability expectations are defined with the specs.

## Repository Map

- `openspec/changes/phase-1/design.md`
- `openspec/changes/phase-1/specs/`

## First-Read Onboarding Path

1. Read `README.md`.
2. Read `openspec/changes/phase-1/design.md`.
3. Read relevant specs under `openspec/changes/phase-1/specs/`.

## Scope and Non-Goals

- Scope: deterministic payload transformation into backend contracts.
- Non-goals: replacing backend business validation and policy.
