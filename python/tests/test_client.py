import json
import subprocess
import unittest
from pathlib import Path
from unittest.mock import patch

from omni_api.client import OmniApiClient


def sample_payload() -> dict:
    return {
        "capability": "chat",
        "provider": {
            "preferred": ["openai", "gemini"],
            "availability": {
                "openai": {"available": True, "compatible": True},
                "gemini": {"available": True, "compatible": True},
            },
        },
        "request": {
            "model": "gpt-4o-mini",
            "messages": [
                {
                    "role": "user",
                    "content": [{"type": "text", "text": "hello"}],
                }
            ],
            "tools": [],
            "generation": {"temperature": 0.7, "top_p": 1.0, "max_tokens": 64, "stop": []},
            "stream": False,
        },
        "metadata": {},
    }


class OmniApiClientTests(unittest.TestCase):
    def test_wrapper_marshals_without_business_logic(self) -> None:
        payload = sample_payload()
        expected = {"ok": True, "selected_provider": "openai"}

        with patch("subprocess.run") as mock_run:
            mock_run.return_value = subprocess.CompletedProcess(
                args=["omni_transform"], returncode=0, stdout=json.dumps(expected), stderr=""
            )

            client = OmniApiClient(binary_path="omni_transform")
            result = client.transform_chat(payload)

        self.assertEqual(result, expected)
        self.assertEqual(mock_run.call_count, 1)
        sent_input = mock_run.call_args.kwargs["input"]
        self.assertEqual(json.loads(sent_input), payload)

    def test_contract_parity_with_rust_binary(self) -> None:
        project_root = Path(__file__).resolve().parents[2]
        binary = project_root / "target" / "debug" / "omni_transform"
        if not binary.exists():
            self.skipTest("Rust binary not built; run `cargo build --bin omni_transform` first")

        payload = sample_payload()
        client = OmniApiClient(binary_path=str(binary))
        wrapper_result = client.transform_chat(payload)

        direct_proc = subprocess.run(
            [str(binary)],
            input=json.dumps(payload),
            text=True,
            capture_output=True,
            check=False,
        )
        self.assertEqual(direct_proc.returncode, 0)
        direct_result = json.loads(direct_proc.stdout)

        self.assertEqual(wrapper_result, direct_result)


if __name__ == "__main__":
    unittest.main()
