"""Derive the test-only seccomp profile of the QEMU floor route from a pinned Moby allowlist."""

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
    # The helper of scripts/release-smoke-qemu.sh mounts a private binfmt_misc
    # instance and binds its inputs, then scripts/floor-qemu-enter.c pivots
    # root. The helper creates its namespaces with unshare(NEWUSER | NEWNS)
    # before it drops every capability.
    result["syscalls"].extend(
        [
            {"names": ["mount", "pivot_root", "umount2"], "action": "SCMP_ACT_ALLOW"},
            {
                "names": ["unshare"],
                "action": "SCMP_ACT_ALLOW",
                "args": [{"index": 0, "value": 0x10020000, "op": "SCMP_CMP_EQ"}],
            },
        ]
    )
    return result


def main() -> None:
    arguments = sys.argv[1:]
    if len(arguments) > 1:
        raise ValueError("usage: floor_seccomp.py [PINNED_SOURCE]")
    source = Path(arguments[0]) if arguments else SOURCE
    print(json.dumps(profile(source), indent=2))


if __name__ == "__main__":
    try:
        main()
    except (OSError, ValueError) as error:
        print(f"floor-seccomp: {error}", file=sys.stderr)
        sys.exit(1)
