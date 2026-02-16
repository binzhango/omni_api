# omni_api

`omni_api` is a payload adaptation layer for backend contract normalization.

## Opening and Positioning

- Problem: upstream payloads vary while backend contracts are strict.
- Value: centralize transforms and reduce per-service adapter drift.
- Audience: platform/backend teams managing shared API boundaries.

## Core Architecture Model

- Transform in two phases: plan generation then deterministic execution.
- Components: router, schema loader, planner, executor, validator, diagnostics.

## Contracts and Canonical Artifacts

- Canonical backend contracts are defined in JSON Schema files.
- Transform Plan DSL defines mappings, defaults, drops, and required fields.

## Runtime Behavior and Operational Expectations

- Runtime flow: ingress -> routing -> planning -> execution -> validation -> diagnostics.
- Failure handling and operational visibility are explicit.

## Repository Map

- `openspec/changes/phase-1/design.md`
- `openspec/changes/phase-1/specs/`

## First-Read Onboarding Path

1. Read `README.md`.
2. Read phase design.
3. Read relevant provider specs.

## Scope and Non-Goals

- Scope: deterministic payload transformation to target contracts.
- Non-goals: replacing backend business validation.
