from omni_api import to_ollama_payload


def test_adapter_symbol_is_exported() -> None:
    assert callable(to_ollama_payload)


def test_chat_payload_passthrough_messages() -> None:
    result = to_ollama_payload(
        {
            "model": "llama3",
            "messages": [{"role": "user", "content": "hello"}],
            "stream": False,
            "extra": "drop-me",
        }
    )
    assert result["model"] == "llama3"
    assert result["messages"] == [{"role": "user", "content": "hello"}]
    assert result["stream"] is False
    assert "extra" not in result
