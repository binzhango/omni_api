import pytest

from omni_api import TransformSchemaError, transform


def test_rejects_non_object_root_schema() -> None:
    with pytest.raises(TransformSchemaError):
        transform({"name": "x"}, {"type": "array"})
