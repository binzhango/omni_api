import pytest

from omni_api import TransformValidationError, transform


def test_missing_required_field_raises_validation_error() -> None:
    with pytest.raises(TransformValidationError):
        transform(
            {"full_name": "John"},
            {
                "type": "object",
                "properties": {"name": {"type": "string"}, "email": {"type": "string"}},
                "required": ["name", "email"],
            },
        )
