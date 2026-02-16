import pytest

from omni_api import TransformValidationError, to_ollama_payload


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


def test_generate_payload_passthrough_prompt() -> None:
    result = to_ollama_payload({"model": "llama3", "prompt": "hello", "extra": "drop-me"})
    assert result == {"model": "llama3", "prompt": "hello"}


def test_chat_takes_precedence_when_both_present() -> None:
    result = to_ollama_payload(
        {
            "model": "llama3",
            "messages": [{"role": "user", "content": "chat"}],
            "prompt": "ignored prompt",
        }
    )
    assert "messages" in result
    assert "prompt" not in result


def test_missing_model_raises_validation_error() -> None:
    with pytest.raises(TransformValidationError):
        to_ollama_payload({"messages": [{"role": "user", "content": "x"}]})


def test_missing_messages_and_prompt_raises_validation_error() -> None:
    with pytest.raises(TransformValidationError):
        to_ollama_payload({"model": "llama3"})
