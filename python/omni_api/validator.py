from __future__ import annotations

from typing import Any

from .errors import TransformValidationError


def validate_payload(
    payload: dict[str, Any],
    target_schema: dict[str, Any],
    required: list[str],
) -> list[str]:
    allowed = set(target_schema.get("properties", {}).keys())
    unknown = sorted(key for key in payload.keys() if key not in allowed)
    if unknown:
        raise TransformValidationError(f"Output contains unknown keys: {unknown}")

    missing_required = sorted(key for key in required if key not in payload)
    if missing_required:
        raise TransformValidationError(f"Missing required fields: {missing_required}")

    return missing_required
