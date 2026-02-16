# omni_api

`omni_api` is a payload adaptation layer for platform teams that need to enforce backend API contracts while supporting heterogeneous upstream producers.

Upstream systems rarely emit a single stable shape. Backend services require strict contracts for correctness and long-term maintainability. Without a shared adaptation layer, transform logic fragments across services and drifts over time.

## Opening and Positioning

`omni_api` is designed for platform/backend teams that own shared API boundaries.

It provides:
- Centralized transformation logic instead of per-service adapters
- Deterministic and auditable mapping behavior
- Isolation between producer payloads and backend contracts
- Faster integration onboarding

## Core Architecture Model

`omni_api` follows a deterministic two-step model:
1. Compile a Transform Plan from source payload and target contract.
2. Execute the plan with a pure transformer.

Core components:
- Router: selects target backend profile and route
- Schema Loader: resolves canonical contract
- Planner: generates Transform Plan output
- Executor: applies `mappings`, `defaults`, and `drops`
- Validator: enforces required fields and contract validity
- Diagnostics: reports what changed and why

## Contracts and Canonical Artifacts

Backend contracts are the source of truth and should be maintained as JSON Schema files under `schemas/<provider>/<endpoint>.json`.

Transform Plan DSL fields:
- `mappings`
- `defaults`
- `drops`
- `required`

Execution guarantees:
- Output keys stay within target JSON Schema unless policy allows passthrough
- Missing source paths emit diagnostics
- Required fields are validated before backend dispatch

Policy controls:
- strict mode
- unknown-field handling
- fallback behavior

## Runtime Behavior and Operational Expectations

The request lifecycle is:
ingress payload -> route selection -> plan generation -> deterministic execution -> validation -> diagnostics emission.

Failure handling:
- validation failure is returned as a structured response
- strict mode prevents silent field loss
- warnings surface uncertain mappings

Operational observability:
- log plan version and selected profile
- emit mapped/dropped/missing counters per request
- track validation and policy failures as metrics

## Repository Map

Primary references:
- `openspec/changes/phase-1/design.md`
- `openspec/changes/phase-1/specs/chat-canonical-transform/spec.md`
- `openspec/changes/phase-1/specs/provider-availability-routing/spec.md`
- `openspec/changes/phase-1/specs/adapter-diagnostics-contract/spec.md`
- `openspec/changes/phase-1/specs/chat-openai-vertical-slice/spec.md`

## First-Read Onboarding Path

1. Read `README.md` for system boundaries.
2. Read `openspec/changes/phase-1/design.md` for phase intent.
3. Read relevant files in `openspec/changes/phase-1/specs/`.
4. Align new adapter work with canonical transform and diagnostics contracts.

## Scope and Non-Goals

Current scope:
- deterministic payload transformation to target contracts
- explicit diagnostics and policy boundaries

This section states explicit non-goals to prevent scope drift.

Non-goals:
- replacing backend business validation rules
- defining provider SDK behavior outside contract transformation
- allowing implicit schema drift through silent passthrough defaults
