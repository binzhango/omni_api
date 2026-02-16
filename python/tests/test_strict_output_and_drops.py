from omni_api import transform


def test_extra_source_fields_are_dropped_and_reported() -> None:
    result = transform(
        {"name": "John", "age": 30, "extra": "x"},
        {
            "type": "object",
            "properties": {"name": {"type": "string"}, "age": {"type": "number"}},
            "required": ["name"],
        },
    )
    assert "extra" not in result.payload
    assert any("extra" in dropped for dropped in result.report.dropped)
