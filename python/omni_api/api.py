import json
from urllib import error, request
from typing import Any

from .errors import TransformSchemaError, TransformValidationError
from .executor import apply_plan
from .plan_types import TransformPlan, TransformReport, TransformResult
from .planner import build_plan
from .validator import validate_payload


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


def _extract_json_object(text: str) -> dict[str, Any]:
    start = text.find("{")
    end = text.rfind("}")
    if start == -1 or end == -1 or start >= end:
        raise TransformValidationError(f"LLM response is not JSON object: {text}")

    try:
        parsed = json.loads(text[start : end + 1])
    except json.JSONDecodeError as exc:
        raise TransformValidationError(f"Failed to parse LLM JSON payload: {text}") from exc

    if not isinstance(parsed, dict):
        raise TransformValidationError(f"LLM payload must be an object: {parsed}")
    return parsed


def _build_alignment_prompt(source_payload: dict[str, Any], target_schema: dict[str, Any]) -> str:
    return (
        "You are a payload alignment engine.\n"
        "Given source_payload and target_schema, return only a JSON object aligned to target_schema.\n"
        "Rules:\n"
        "- Output must be a JSON object.\n"
        "- Include only keys defined in target_schema.properties.\n"
        "- Respect required fields from target_schema.\n"
        "- No markdown, no explanations, no code fences.\n\n"
        f"source_payload:\n{json.dumps(source_payload, ensure_ascii=True)}\n\n"
        f"target_schema:\n{json.dumps(target_schema, ensure_ascii=True)}\n"
    )


def _ollama_generate(
    source_payload: dict[str, Any],
    target_schema: dict[str, Any],
    model: str,
    base_url: str,
    timeout: float = 60.0,
) -> dict[str, Any]:
    prompt = _build_alignment_prompt(source_payload, target_schema)
    body = json.dumps({"model": model, "prompt": prompt, "stream": False}).encode("utf-8")
    req = request.Request(
        f"{base_url.rstrip('/')}/api/generate",
        data=body,
        headers={"Content-Type": "application/json"},
        method="POST",
    )

    try:
        with request.urlopen(req, timeout=timeout) as resp:
            payload = json.loads(resp.read().decode("utf-8"))
    except error.HTTPError as exc:
        body = exc.read().decode("utf-8", errors="replace")
        raise TransformValidationError(
            f"Ollama HTTP error {exc.code}: {body}"
        ) from exc
    except error.URLError as exc:
        raise TransformValidationError(f"Ollama connection failed: {exc}") from exc

    if not isinstance(payload, dict) or "response" not in payload:
        raise TransformValidationError(f"Unexpected Ollama response shape: {payload}")

    return _extract_json_object(str(payload["response"]))


def transform(
    source_payload: dict[str, Any],
    target_schema: dict[str, Any],
    *,
    llm_provider: str | None = None,
    llm_model: str | None = None,
    llm_base_url: str = "http://127.0.0.1:11434",
) -> TransformResult:
    _validate_schema_subset(target_schema)

    if llm_provider == "ollama":
        model = llm_model or "llama3.1:latest"
        payload = _ollama_generate(source_payload, target_schema, model=model, base_url=llm_base_url)
        missing_required = validate_payload(
            payload,
            target_schema,
            [key for key in target_schema.get("required", []) if isinstance(key, str)],
        )
        report = TransformReport(
            mapped=sorted(payload.keys()),
            dropped=[],
            missing_required=missing_required,
            warnings=["aligned via ollama llm"],
        )
        return TransformResult(payload=payload, plan=TransformPlan(), report=report)

    plan = build_plan(source_payload, target_schema)
    payload, report = apply_plan(source_payload, target_schema, plan)
    missing_required = validate_payload(payload, target_schema, plan.required)

    final_report = TransformReport(
        mapped=report.mapped,
        dropped=report.dropped,
        missing_required=missing_required,
        warnings=report.warnings,
    )
    return TransformResult(payload=payload, plan=plan, report=final_report)
