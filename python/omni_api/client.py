import json
import os
import subprocess
from pathlib import Path
from typing import Any, Dict, Optional


class OmniApiClient:
    def __init__(self, binary_path: Optional[str] = None) -> None:
        default_bin = Path(__file__).resolve().parents[2] / "target" / "debug" / "omni_transform"
        self.binary_path = binary_path or os.getenv("OMNI_TRANSFORM_BIN") or str(default_bin)

    def transform_chat(self, payload: Dict[str, Any]) -> Dict[str, Any]:
        proc = subprocess.run(
            [self.binary_path],
            input=json.dumps(payload),
            text=True,
            capture_output=True,
            check=False,
        )
        if proc.returncode != 0:
            raise RuntimeError(f"Rust transform process failed: {proc.stderr.strip()}")

        try:
            return json.loads(proc.stdout)
        except json.JSONDecodeError as exc:
            raise RuntimeError(f"Invalid JSON response from Rust transform process: {exc}") from exc


def transform_chat(payload: Dict[str, Any], binary_path: Optional[str] = None) -> Dict[str, Any]:
    return OmniApiClient(binary_path=binary_path).transform_chat(payload)
