from omni_api import to_ollama_payload


def test_adapter_symbol_is_exported() -> None:
    assert callable(to_ollama_payload)
