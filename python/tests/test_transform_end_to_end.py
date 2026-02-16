from omni_api import transform


def test_end_to_end_returns_payload_plan_and_report() -> None:
    result = transform(
        {
            "full_name": "John Doe",
            "contact": {"email": "john@example.com"},
            "age": 30,
            "extra_data": "ignore",
        },
        {
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "number"},
                "email": {"type": "string"},
            },
            "required": ["name", "age", "email"],
        },
    )
    assert result.payload == {"name": "John Doe", "age": 30, "email": "john@example.com"}
    assert result.report.missing_required == []
