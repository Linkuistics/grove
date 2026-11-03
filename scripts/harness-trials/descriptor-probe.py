#!/usr/bin/env python3
"""Show how a sandboxed reader can read descriptor 3, as a pipe and as a socket.

    descriptor-probe.py PROFILE_COPY CONFINEMENT_RS WORK_DIR

PROFILE_COPY is `scripts/harness-trials/profile-copy.py`. CONFINEMENT_RS is
`crates/keyed-launch/src/confinement.rs` of a Ravel checkout. WORK_DIR is an
existing scratch directory that becomes the one writable grant.

The probe writes dummy text, never a credential, into a pipe and into one end
of a socket pair. It passes the other end to `/bin/sh` as descriptor 3, and
the shell reads it in two ways: `cat /dev/fd/3` opens the descriptor by its
path, and `cat <&3` reads the inherited descriptor directly. Each pair runs
unconfined and under a copy of Ravel's macOS Seatbelt profile.

ravel-token-handoff-k132 used it to choose a socket for Ravel's token handoff,
because Claude Code 2.1.317 opens a pipe by its path and reads a socket
directly. It makes no model call and gives no credit to a trial.
"""

from __future__ import annotations

import os
import socket
import subprocess
import sys

DUMMY = b"dummy-not-a-credential\n"


def run(profile_copy: str, confinement: str, work: str, kind: str, inner: str, confined: bool) -> None:
    hold = os.open("/dev/null", os.O_RDONLY)
    if hold != 3:
        raise SystemExit(f"descriptor 3 is in use (got {hold}); run the probe from a plain shell")
    keep = None
    if kind == "pipe":
        read_end, write_end = os.pipe()
        os.write(write_end, DUMMY)
        os.close(write_end)
        child = read_end
    else:
        keep, other = socket.socketpair()
        keep.sendall(DUMMY)
        keep.shutdown(socket.SHUT_WR)
        child = os.dup(other.fileno())
        other.close()
    os.dup2(child, 3)
    os.close(child)
    os.set_inheritable(3, True)
    command = ["/bin/sh", "-c", inner]
    if confined:
        command = [sys.executable, "-I", profile_copy, confinement, "--writable", work, "--", *command]
    result = subprocess.run(command, pass_fds=(3,), capture_output=True, cwd=work, check=False)
    os.close(3)
    if keep is not None:
        keep.close()
    place = "confined" if confined else "open"
    print(
        f"{kind:6} {place:8} {inner!r:16} exit={result.returncode} "
        f"out={result.stdout!r} err={result.stderr.strip()[:120]!r}"
    )


def main(argv: list[str]) -> int:
    if len(argv) != 3:
        print(__doc__.strip().splitlines()[2].strip(), file=sys.stderr)
        return 2
    profile_copy, confinement, work = (os.path.realpath(path) for path in argv)
    for confined in (False, True):
        for kind in ("pipe", "socket"):
            for inner in ("cat /dev/fd/3", "cat <&3"):
                run(profile_copy, confinement, work, kind, inner, confined)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
