from omni_api.planner import build_plan


def test_exact_mapping_selected() -> None:
    plan = build_plan({"name": "John"}, {"type": "object", "properties": {"name": {"type": "string"}}})
    assert any(m.to_key == "name" for m in plan.mappings)
