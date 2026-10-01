"""Offline byte check of this bounded runner diagnostic packet; no campaigns."""
from pathlib import Path
import hashlib
import json
import subprocess

HERE = Path(__file__).resolve().parent
ROOT = Path(subprocess.check_output(["git", "rev-parse", "--show-toplevel"], cwd=HERE).decode().strip())


def check(path, expected):
    raw = Path(path).read_bytes()
    assert len(raw) == expected["bytes"], str(path)
    assert hashlib.sha256(raw).hexdigest() == expected["sha256"], str(path)


def main():
    manifest = json.loads((HERE / "manifest.json").read_text(encoding="utf-8"))
    for item in manifest["payloads"]:
        check(HERE / item["path"], item)
    for item in manifest["source_files"]:
        # Repository Python text is normalized to LF for portability between
        # Git's Windows autocrlf checkout and the committed source blob.
        raw = (ROOT / item["path"]).read_bytes().replace(b"\r\n", b"\n")
        assert len(raw) == item["bytes"]
        assert hashlib.sha256(raw).hexdigest() == item["sha256"]
    retained = json.loads((HERE / "failure-evidence.json").read_text(encoding="utf-8"))
    assert retained["native_reexecuted"] is False
    assert retained["old_metadata_unchanged_during_review"] is True
    for item in retained["old_metadata_pins"]:
        check(item["path"], item)
    for report in retained["wer"]:
        for field in ("original", "retained_external_copy"):
            check(report[field]["path"], report[field])
    print(json.dumps({"passed": True, "payloads": len(manifest["payloads"]),
                      "source_files": len(manifest["source_files"]),
                      "retained_metadata": len(retained["old_metadata_pins"]),
                      "wer_files": 4, "native_reexecuted": False,
                      "qualification": False, "s25_complete": False}))


if __name__ == "__main__":
    main()
