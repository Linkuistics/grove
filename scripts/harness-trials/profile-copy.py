#!/usr/bin/env python3
"""Run one command under a copy of Ravel's macOS Seatbelt profile.

    profile-copy.py CONFINEMENT_RS [--writable DIR]... [--read FILE]...
                    [--allow RULE]... -- PROGRAM ARG...

CONFINEMENT_RS is `crates/keyed-launch/src/confinement.rs` of a Ravel
checkout. The script takes the base profile and the system reads from that
file, adds each writable directory and each read-only file as Ravel does, and
runs the command with `/usr/bin/sandbox-exec`. Each `--allow RULE` appends one
Seatbelt rule, such as `(allow mach-lookup (global-name "com.apple.SecurityServer"))`,
to test what a profile change would admit.

This is a diagnostic. It makes no model call by itself, and it gives no credit
to a trial: a trial runs through `grove run` and Ravel's own profile. The
record of `confined-harness-trials-k128` in
docs/common-loading-acceptance-record.md used it to find each blocker. Ravel
adds the denies that protect a read-only file beneath a writable directory, and
this copy omits them, because no diagnostic grants such a file.
"""

from __future__ import annotations

import argparse
import os
import re
import sys
from pathlib import Path


def profile_parts(source: str) -> tuple[str, list[str]]:
    """The base profile of the macOS backend and its system reads."""
    mac = source[source.index('#[cfg(target_os = "macos")]\nfn platform_command'):]
    base = mac[mac.index('r#"') + 3: mac.index('"#')]
    start = source.index("pub fn confinement_system_reads")
    end = source.index('#[cfg(target_os = "linux")]\npub fn confinement_system_reads')
    return base, re.findall(r'"(/[^"]*)",', source[start:end])


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("confinement", type=Path)
    parser.add_argument("--writable", action="append", default=[])
    parser.add_argument("--read", action="append", default=[])
    parser.add_argument("--allow", action="append", default=[])
    split = argv.index("--") if "--" in argv else len(argv)
    args = parser.parse_args(argv[:split])
    command = argv[split + 1:]
    if not command or not args.writable:
        parser.error("give at least one --writable directory and a command after --")
    base, system_reads = profile_parts(args.confinement.read_text())
    profile = base + "".join(f'(allow file-read* file-map-executable (subpath "{path}"))\n' for path in system_reads)
    sandbox = ["/usr/bin/sandbox-exec"]
    for index, path in enumerate(args.writable):
        profile += f'(allow file-read* file-write* file-map-executable (subpath (param "W{index}")))\n'
        sandbox += ["-D", f"W{index}={os.path.realpath(path)}"]
    for index, path in enumerate(args.read):
        profile += f'(allow file-read* file-map-executable (literal (param "R{index}")))\n'
        sandbox += ["-D", f"R{index}={os.path.realpath(path)}"]
    profile += "".join(rule + "\n" for rule in args.allow)
    os.chdir(os.path.realpath(args.writable[0]))
    os.execv(sandbox[0], [*sandbox, "-p", profile, "--", *command])
    return 1


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
