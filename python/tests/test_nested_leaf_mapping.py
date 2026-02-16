from omni_api.planner import build_plan


def test_nested_leaf_mapping_selected() -> None:
    plan = build_plan(
        {"contact": {"email": "a@b.com"}},
        {"type": "object", "properties": {"email": {"type": "string"}}},
    )
    assert any(m.from_path == "contact.email" and m.to_key == "email" for m in plan.mappings)
