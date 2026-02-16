from dataclasses import dataclass, field
from typing import Any


@dataclass(frozen=True)
class Mapping:
    from_path: str
    to_key: str
    op: str = "copy"


@dataclass(frozen=True)
class TransformPlan:
    version: str = "1"
    mappings: list[Mapping] = field(default_factory=list)
    defaults: list[dict[str, Any]] = field(default_factory=list)
    drops: list[str] = field(default_factory=list)
    required: list[str] = field(default_factory=list)
    warnings: list[str] = field(default_factory=list)


@dataclass(frozen=True)
class TransformReport:
    mapped: list[str] = field(default_factory=list)
    dropped: list[str] = field(default_factory=list)
    missing_required: list[str] = field(default_factory=list)
    warnings: list[str] = field(default_factory=list)


@dataclass(frozen=True)
class TransformResult:
    payload: dict[str, Any]
    plan: TransformPlan
    report: TransformReport
