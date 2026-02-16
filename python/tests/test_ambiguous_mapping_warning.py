from omni_api.planner import build_plan


def test_ambiguous_mapping_emits_warning() -> None:
    plan = build_plan(
        {"a": {"email": "x"}, "b": {"email": "y"}},
        {"type": "object", "properties": {"email": {"type": "string"}}},
    )
    assert any("ambiguous" in w.lower() for w in plan.warnings)
