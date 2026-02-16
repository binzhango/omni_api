# omni_api

`omni_api` is a payload adaptation layer for platform teams that need to enforce backend API contracts while supporting heterogeneous upstream producers.

Upstream systems rarely emit a single stable shape. Backend services, however, depend on strict contracts for correctness, validation, and long-term maintainability. Without a shared adaptation layer, transformation logic fragments across services and drifts over time. `omni_api` centralizes that responsibility so backend contracts remain stable and producer variation stays isolated.

## Opening and Positioning

`omni_api` is designed for platform and backend teams that own shared API boundaries.

It provides:
- Centralized transformation logic instead of per-service adapter code
- Deterministic and auditable mapping behavior
- Isolation between producer payloads and backend contracts
- Faster integration onboarding across providers and internal clients

## Core Architecture Model

`omni_api` follows a deterministic two-step model:
1. Compile a Transform Plan from source payload plus target contract.
2. Execute that plan with a pure transformation step.

This separates planning from execution and avoids opaque "guess and mutate" behavior at runtime. The result is easier debugging, consistent replayability, and straightforward test coverage of transformation decisions.

Core components:
- Router: selects the target backend profile and route
- Schema Loader: loads the canonical backend contract
- Planner: generates a Transform Plan (heuristic-first, optional assisted planning for unresolved mappings)
- Executor: applies `mappings`, `defaults`, and `drops`
- Validator: enforces required fields and contract validity
- Diagnostics: emits a structured report of what changed

Each request returns both transformed payload output and diagnostics metadata.

## Contracts and Canonical Artifacts

Backend contracts are the source of truth and should be maintained as JSON Schema files under `schemas/<provider>/<endpoint>.json`.

The Transform Plan is a versioned DSL that includes:
- `mappings`: source-path to target-field copies or transformations
- `defaults`: fallback values for absent optional fields
- `drops`: explicit removals or exclusion rules
- `required`: fields that must exist before backend dispatch

Execution guarantees:
- Output keys are constrained to the target JSON Schema unless policy explicitly allows passthrough
- Invalid or missing source paths are captured in diagnostics
- Required fields are validated before backend calls

Diagnostics contract:
- Mapped fields
- Dropped fields
- Missing required fields
- Confidence/warning signals for uncertain mappings

Policy controls:
- strict mode behavior
- unknown-field handling
- fallback strategy selection

## Runtime Behavior and Operational Expectations

The request lifecycle is:
ingress payload -> route selection -> plan generation -> deterministic execution -> validation -> diagnostics emission.

Planner strategy:
- Heuristic mapping first (exact key, alias, and type-aware matches)
- Assisted planning only for unresolved cases
- Confidence-based handling for acceptance versus manual review

Failure and safety expectations:
- validation failure is returned as a structured response, not a silent mutation
- strict mode prevents silent field loss
- warnings surface low-confidence mappings for operator visibility

Operational observability:
- Log plan version and selected route profile
- Emit mapped/dropped/missing counters per request
- Track validation and policy failures as first-class metrics

## Repository Map

Primary references in this repository:
- `openspec/changes/phase-1/design.md`: phase architecture baseline
- `openspec/changes/phase-1/specs/chat-canonical-transform/spec.md`: canonical transform behavior
- `openspec/changes/phase-1/specs/provider-availability-routing/spec.md`: routing expectations
- `openspec/changes/phase-1/specs/adapter-diagnostics-contract/spec.md`: diagnostics schema and obligations
- `openspec/changes/phase-1/specs/chat-openai-vertical-slice/spec.md`: first vertical-slice constraints

## First-Read Onboarding Path

For new platform engineers, follow this order:
1. Read this `README.md` for architecture and boundaries.
2. Read `openspec/changes/phase-1/design.md` for phase-level design intent.
3. Read spec files under `openspec/changes/phase-1/specs/` relevant to your route or provider.
4. Align implementation tasks with the diagnostics and canonical transform contracts before adding new adapters.

## Python MVP Usage

```python
from omni_api import transform

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

result = transform(source_payload, target_schema)
print(result.payload)
# {'name': 'John Doe', 'age': 30, 'email': 'john@example.com'}
```

## Scope and Non-Goals

Current scope:
- Normalize producer payloads into provider/backend-specific contracts
- Provide deterministic transform execution and diagnostics
- Maintain explicit operational policy boundaries

This section states explicit non-goals to prevent accidental scope expansion.

Non-goals:
- Replacing backend business logic validation rules
- Defining provider SDK behavior outside payload contract transformation
- Allowing implicit schema drift through silent passthrough defaults
