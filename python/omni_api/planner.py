from __future__ import annotations

from typing import Any

from .plan_types import Mapping, TransformPlan


def _normalize(value: str) -> str:
    return value.replace("_", "").lower()


def _flatten_paths(payload: dict[str, Any], prefix: str = "") -> list[tuple[str, Any]]:
    paths: list[tuple[str, Any]] = []
    for key in sorted(payload.keys()):
        path = f"{prefix}.{key}" if prefix else key
        value = payload[key]
        if isinstance(value, dict):
            paths.extend(_flatten_paths(value, path))
        else:
            paths.append((path, value))
    return paths


def _best_candidates(target_key: str, source_paths: list[str]) -> tuple[list[str], list[str]]:
    exact = [path for path in source_paths if path == target_key]
    if exact:
        return exact, []

    target_norm = _normalize(target_key)
    normalized_direct = [
        path for path in source_paths if "." not in path and _normalize(path) == target_norm
    ]
    if normalized_direct:
        return normalized_direct, []

    nested_leaf = [
        path
        for path in source_paths
        if "." in path and _normalize(path.rsplit(".", 1)[1]) == target_norm
    ]
    if nested_leaf:
        return nested_leaf, nested_leaf

    return [], []


def _tie_break(candidates: list[str]) -> str:
    return sorted(candidates, key=lambda p: (len(p), p))[0]


def build_plan(source_payload: dict[str, Any], target_schema: dict[str, Any]) -> TransformPlan:
    flattened = _flatten_paths(source_payload)
    source_paths = [path for path, _ in flattened]

    mappings: list[Mapping] = []
    warnings: list[str] = []
    mapped_from: set[str] = set()

    properties = target_schema.get("properties", {})
    for target_key in properties.keys():
        candidates, ambiguous_group = _best_candidates(target_key, source_paths)
        if not candidates:
            continue

        chosen = _tie_break(candidates)
        mappings.append(Mapping(from_path=chosen, to_key=target_key))
        mapped_from.add(chosen)

        if len(ambiguous_group) > 1:
            warnings.append(
                f"Ambiguous mapping for '{target_key}': candidates={sorted(ambiguous_group)}; chose '{chosen}'"
            )

    drops = sorted(path for path in source_paths if path not in mapped_from)

    required_raw = target_schema.get("required", [])
    required = [key for key in required_raw if isinstance(key, str)]

    return TransformPlan(
        mappings=mappings,
        defaults=[],
        drops=drops,
        required=required,
        warnings=warnings,
    )
