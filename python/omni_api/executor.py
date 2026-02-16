from __future__ import annotations

from typing import Any

from .plan_types import TransformPlan, TransformReport


def _get_by_path(payload: dict[str, Any], path: str) -> Any:
    current: Any = payload
    for part in path.split("."):
        if not isinstance(current, dict) or part not in current:
            raise KeyError(path)
        current = current[part]
    return current


def apply_plan(
    source_payload: dict[str, Any],
    target_schema: dict[str, Any],
    plan: TransformPlan,
) -> tuple[dict[str, Any], TransformReport]:
    properties = target_schema.get("properties", {})
    allowed_keys = set(properties.keys())

    payload: dict[str, Any] = {}
    mapped: list[str] = []
    warnings = list(plan.warnings)

    for mapping in plan.mappings:
        if mapping.to_key not in allowed_keys:
            continue
        try:
            value = _get_by_path(source_payload, mapping.from_path)
        except KeyError:
            warnings.append(
                f"Missing source path '{mapping.from_path}' for target '{mapping.to_key}'"
            )
            continue
        payload[mapping.to_key] = value
        mapped.append(mapping.to_key)

    dropped = list(plan.drops)

    report = TransformReport(
        mapped=sorted(set(mapped)),
        dropped=sorted(dropped),
        missing_required=[],
        warnings=warnings,
    )
    return payload, report
