#!/usr/bin/env python3
"""Prepare pinned Ravel archives and independent prefixes; never invoke their programs."""

from __future__ import annotations

import argparse
import hashlib
import io
import os
import shutil
import subprocess
import sys
import tarfile
import tempfile
from collections.abc import Iterator
from contextlib import contextmanager
from pathlib import Path

# the published 1.0.0 contract, independent of Grove and of an adjacent checkout.
# https://github.com/Linkuistics/Ravel/releases/tag/v1.0.0
VERSION = "1.0.0"
PINS = {
    "aarch64-apple-darwin": "d1f253db86cde3c625950cda2d8fa929ca4d1e4529ca6c3f223e138f9be9a922",
    "aarch64-unknown-linux-gnu": "288158c1259667dbd11d9f71aa5d643b4da4e08ffc4402524f4852afb314f732",
    "x86_64-unknown-linux-gnu": "1264ea0f3dd113f5dbb0914992ee9a7734285c0ddcc161e9acaffd83c037f9a9",
}
MANIFEST = (
    "LICENSE",
    "README.md",
    "bin/ravel",
    "libexec/ravel/examples/sample.ts",
    "libexec/ravel/examples/static.d.ts",
    "libexec/ravel/examples/static.ts",
    "libexec/ravel/notices/NOTICES.md",
    "libexec/ravel/notices/bun-LICENSE.md",
    "libexec/ravel/ravel-policy",
    "libexec/ravel/sdk/index.d.ts",
    "libexec/ravel/sdk/index.ts",
)
EXECUTABLES = {"bin/ravel", "libexec/ravel/ravel-policy"}
ROOT = Path(__file__).resolve().parent.parent


class InputError(ValueError):
    """An archive cannot be admitted."""


def archive_name(target: str) -> str:
    return f"ravel-v{VERSION}-{target}.tar.xz"


@contextmanager
def validate_archive(
    data: bytes, target: str, expected_sha: str
) -> Iterator[tarfile.TarFile]:
    """Check the entire private snapshot before allowing extraction of any member."""
    actual_sha = hashlib.sha256(data).hexdigest()
    if actual_sha != expected_sha:
        raise InputError(f"{target}: SHA-256 {actual_sha}; expected {expected_sha}")
    if target not in PINS:
        raise InputError(f"unsupported target: {target}")
    top = archive_name(target).removesuffix(".tar.xz")
    files = {f"{top}/{name}" for name in MANIFEST}
    directories = {top}
    for name in files:
        directories.update(
            str(parent) for parent in Path(name).parents if str(parent) != "."
        )
    with tarfile.open(fileobj=io.BytesIO(data), mode="r:xz") as archive:
        members = archive.getmembers()
        names = [member.name for member in members]
        if len(names) != len(set(names)):
            raise InputError(f"{target}: duplicate archive member")
        if set(names) != files | directories:
            raise InputError(
                f"{target}: manifest mismatch; missing={sorted((files | directories) - set(names))}; "
                f"unexpected={sorted(set(names) - (files | directories))}"
            )
        for member in members:
            if member.name in directories:
                if not member.isdir():
                    raise InputError(
                        f"{member.name}: manifest directory is not a directory"
                    )
            elif not member.isfile():
                raise InputError(f"{member.name}: manifest file is not regular")
            if member.mode & 0o7000:
                raise InputError(f"{member.name}: special permission bits")
            if member.name in {f"{top}/{name}" for name in EXECUTABLES}:
                if member.mode & 0o111 != 0o111:
                    raise InputError(
                        f"{member.name}: not executable by all installed users"
                    )
                stream = archive.extractfile(member)
                if stream is None:
                    raise InputError(f"{member.name}: no executable contents")
                with stream:
                    header = stream.read(20)
                if target == "aarch64-apple-darwin":
                    matches = header[:8] == bytes.fromhex("cffaedfe0c000001")
                else:
                    machine = (
                        b"\xb7\x00" if target.startswith("aarch64-") else b"\x3e\x00"
                    )
                    matches = (
                        header[:7] == b"\x7fELF\x02\x01\x01"
                        and header[18:20] == machine
                    )
                if not matches:
                    raise InputError(
                        f"{member.name}: executable architecture does not match {target}"
                    )
        yield archive


def validate_and_extract(
    data: bytes, target: str, expected_sha: str, output: Path
) -> None:
    """Write only admitted regular files/dirs, without tar's link/path extraction semantics."""
    with validate_archive(data, target, expected_sha) as archive:
        for member in archive.getmembers():
            path = output / member.name
            if member.isdir():
                path.mkdir(parents=True, exist_ok=True)
            else:
                path.parent.mkdir(parents=True, exist_ok=True)
                stream = archive.extractfile(member)
                if stream is None:
                    raise InputError(f"{member.name}: no file contents")
                with stream, path.open("xb") as destination:
                    shutil.copyfileobj(stream, destination)
                path.chmod(member.mode & 0o777)


def obtain_archive(target: str, directory: Path, fetch: bool) -> bytes:
    path = directory / archive_name(target)
    if path.exists():
        data = path.read_bytes()
    elif not fetch:
        raise InputError(
            f"missing archive: {path}; explicit inputs never download replacements"
        )
    else:
        directory.mkdir(parents=True, exist_ok=True)
        url = f"https://github.com/Linkuistics/Ravel/releases/download/v{VERSION}/{path.name}"
        with tempfile.TemporaryDirectory(
            prefix=".ravel-download-", dir=directory
        ) as scratch:
            download = Path(scratch) / path.name
            subprocess.run(
                [
                    "curl",
                    "--fail",
                    "--silent",
                    "--show-error",
                    "--location",
                    "--proto",
                    "=https",
                    "--proto-redir",
                    "=https",
                    "--max-time",
                    "300",
                    "--output",
                    str(download),
                    url,
                ],
                check=True,
            )
            data = download.read_bytes()
            with validate_archive(data, target, PINS[target]):
                pass
            os.replace(download, path)
    # Cache corruption is a refusal, not an implicit fetch/repair.
    with validate_archive(data, target, PINS[target]):
        pass
    return data


def prepare(targets: list[str], archives: Path, output: Path, fetch: bool) -> None:
    if output.exists() or output.is_symlink():
        raise InputError(
            f"output already exists: {output}; choose a fresh per-run directory"
        )
    output.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(
        prefix=".ravel-inputs-", dir=output.parent
    ) as scratch:
        staged = Path(scratch) / "inputs"
        staged.mkdir()
        (staged / "archives").mkdir()
        # Validate ALL requested archives before extracting any.
        for target in targets:
            data = obtain_archive(target, archives, fetch)
            (staged / "archives" / archive_name(target)).write_bytes(data)
        for target in targets:
            data = (staged / "archives" / archive_name(target)).read_bytes()
            validate_and_extract(data, target, PINS[target], staged / target)
        os.rename(staged, output)
    for target in targets:
        print(
            f"{target}\t{output / target / archive_name(target).removesuffix('.tar.xz') / 'bin/ravel'}"
        )


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--archives",
        type=Path,
        help="offline input directory; missing/corrupt inputs fail without downloading",
    )
    parser.add_argument(
        "--output",
        required=True,
        type=Path,
        help="fresh output directory (must not exist)",
    )
    parser.add_argument("targets", nargs="*", choices=list(PINS))
    args = parser.parse_args()
    targets = args.targets or list(PINS)
    if len(targets) != len(set(targets)):
        parser.error("duplicate targets")
    try:
        prepare(
            targets,
            args.archives or ROOT / "target/ravel-archives",
            args.output.absolute(),
            fetch=args.archives is None,
        )
    except (
        InputError,
        OSError,
        tarfile.TarError,
        subprocess.CalledProcessError,
    ) as error:
        print(f"ravel-inputs: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
