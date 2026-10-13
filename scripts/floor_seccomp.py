"""Derive the test-only native arm64 profile from a pinned Moby allowlist."""

from __future__ import annotations

import hashlib
import json
import sys
from pathlib import Path

SOURCE = Path(__file__).with_name("floor-seccomp") / "default.json"
SHA256 = "6416b47770785a41ac59073cdc77d9fe98517df2799dc83ef207e622de3053f6"


def profile(source: Path) -> dict[str, object]:
    data = source.read_bytes()
    if hashlib.sha256(data).hexdigest() != SHA256:
        raise ValueError(f"{source} differs from pinned Moby seccomp source")
    result = json.loads(data)
    # Docker supports client-supplied allowlists, retaining the default deny:
    # https://docs.docker.com/engine/security/seccomp/
    # bwrap 0.11.0 raw_clone flags and subsequent user namespace reset:
    # https://github.com/containers/bubblewrap/blob/v0.11.0/bubblewrap.c
    # SIGCHLD | NEWNS | NEWUSER | NEWPID | NEWIPC | NEWUTS, ARM64 arg 0.
    result["syscalls"].extend(
        [
            {
                "names": ["clone"],
                "action": "SCMP_ACT_ALLOW",
                "args": [{"index": 0, "value": 0x3C020011, "op": "SCMP_CMP_EQ"}],
            },
            {
                "names": ["unshare"],
                "action": "SCMP_ACT_ALLOW",
                "args": [{"index": 0, "value": 0x10000000, "op": "SCMP_CMP_EQ"}],
            },
            {"names": ["mount", "pivot_root", "umount2"], "action": "SCMP_ACT_ALLOW"},
        ]
    )
    return result


def main() -> None:
    if len(sys.argv) > 2:
        raise ValueError("usage: floor_seccomp.py [PINNED_SOURCE]")
    source = Path(sys.argv[1]) if len(sys.argv) == 2 else SOURCE
    print(json.dumps(profile(source), indent=2))


if __name__ == "__main__":
    try:
        main()
    except (OSError, ValueError) as error:
        print(f"floor-seccomp: {error}", file=sys.stderr)
        sys.exit(1)
