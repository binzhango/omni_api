from omni_api import transform


def test_import_transform_symbol() -> None:
    assert callable(transform)
