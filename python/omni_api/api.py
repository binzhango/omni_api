from typing import Any

from .errors import TransformSchemaError
from .plan_types import TransformPlan, TransformReport, TransformResult


REQUIRED_SCHEMA_KEYS = {"type", "properties", "required"}


def _validate_schema_subset(target_schema: dict[str, Any]) -> None:
    if target_schema.get("type") != "object":
        raise TransformSchemaError("Only object root schema is supported in MVP")

    properties = target_schema.get("properties")
    if not isinstance(properties, dict):
        raise TransformSchemaError("Schema must define object properties as a dict")

    unknown = set(target_schema.keys()) - REQUIRED_SCHEMA_KEYS
    if unknown:
        raise TransformSchemaError(f"Unsupported top-level schema keys: {sorted(unknown)}")


def transform(source_payload: dict[str, Any], target_schema: dict[str, Any]):
    _validate_schema_subset(target_schema)
    return TransformResult(payload=source_payload, plan=TransformPlan(), report=TransformReport())
