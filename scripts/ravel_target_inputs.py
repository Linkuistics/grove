#!/usr/bin/env python3
"""Prepare recorded Ravel archives of one runtime lane as independent prefixes; never invoke their programs."""

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
from dataclasses import dataclass
from pathlib import Path

TARGETS: tuple[str, ...] = (
    "aarch64-apple-darwin",
    "aarch64-unknown-linux-gnu",
    "x86_64-unknown-linux-gnu",
)


@dataclass(frozen=True)
class Lane:
    """The recorded archives of one Ravel runtime. Nothing outside the record is tested."""

    runtime: str
    # The version in the archive name. A build of the 2.0 feature line keeps
    # 1.1.0 until the cut, so the name does not tell the two lanes apart.
    stem: str
    pins: dict[str, str]
    # Each supported target without a recorded archive, and the reason.
    unrecorded: dict[str, str]
    manifest: tuple[str, ...]
    executables: frozenset[str]
    # The release download directory, or None for archives that only a local
    # build supplies; those are never fetched.
    release: str | None
    default_archives: str
    source_commit: str | None = None
    # Ravel's build.rs embeds this source digest in bin/ravel as ASCII text.
    build_identity: str | None = None


LANES = {
    # the published 1.1.0 contract, independent of Grove and of an adjacent checkout.
    # https://github.com/Linkuistics/Ravel/releases/tag/v1.1.0
    "1.1": Lane(
        runtime="1.1",
        stem="1.1.0",
        pins={
            "aarch64-apple-darwin": "92ec94083495f536136a17310ce1624004d145569f6fbf92da978aee65e7c357",
            "aarch64-unknown-linux-gnu": "4334bd0adfa97391925b44029e96ce70744ecf3d141a2b5bb21174fd7cfa9c60",
            "x86_64-unknown-linux-gnu": "2a0f91c793069fa2dd8d73fcbe0fe383d383d029f60adc4a5b7f3726919b60fb",
        },
        unrecorded={},
        manifest=(
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
        ),
        executables=frozenset({"bin/ravel", "libexec/ravel/ravel-policy"}),
        release="https://github.com/Linkuistics/Ravel/releases/download/v1.1.0",
        default_archives="target/ravel-archives",
    ),
    # The unpublished directory build of Ravel 2.0, from `task release:archives`
    # in the workspace Ravel.skill-directory-delivery at the build commit below.
    # docs/ravel-target-inputs.md states how rollout replaces this record with
    # the published 2.0.0 archives before Grove's release.
    "2.0": Lane(
        runtime="2.0",
        stem="1.1.0",
        pins={
            "aarch64-apple-darwin": "289b4ac57ffd148255960507f02809034e8fb35fe97132813230116add804921",
            "aarch64-unknown-linux-gnu": "2160a92e280f2d6881fd4d72c8196a787abc1b8e0fe20607e5028fa6041e75f3",
            "x86_64-unknown-linux-gnu": "ab49d80db83bf4815342d562d8c265ac21bd97d13f4b9df0a7e18afc79dbdb6c",
        },
        unrecorded={},
        manifest=(
            "LICENSE",
            "README.md",
            "bin/ravel",
            "share/ravel/examples/llm.rhai",
            "share/ravel/examples/sample.rhai",
            "share/ravel/examples/static.rhai",
            "share/ravel/notices/THIRD-PARTY-NOTICES.md",
        ),
        executables=frozenset({"bin/ravel"}),
        release=None,
        default_archives="target/ravel-archives-2.0",
        source_commit="ff14cba826b02ac492fcde0996d662c5378ef923",
        build_identity="58d479ed058f1094b8754c72eecf9f0cec5a60b3d22b76baecdb827364abd217",
    ),
}
# The 1.1 lane's names, kept for callers that predate the lanes.
VERSION = LANES["1.1"].stem
PINS = LANES["1.1"].pins
MANIFEST = LANES["1.1"].manifest
EXECUTABLES = LANES["1.1"].executables
ROOT = Path(__file__).resolve().parent.parent


class InputError(ValueError):
    """An archive cannot be admitted."""


def archive_name(target: str, lane: Lane = LANES["1.1"]) -> str:
    return f"ravel-v{lane.stem}-{target}.tar.xz"


def expected_digest(target: str, lane: Lane) -> str:
    """The recorded SHA-256 of TARGET's archive, or the refusal that names why none exists."""
    if target in lane.pins:
        return lane.pins[target]
    if target in lane.unrecorded:
        raise InputError(
            f"lane {lane.runtime} records no archive for {target}: {lane.unrecorded[target]}"
        )
    raise InputError(f"unsupported target: {target}")


@contextmanager
def validate_archive(
    data: bytes, target: str, expected_sha: str, lane: Lane = LANES["1.1"]
) -> Iterator[tarfile.TarFile]:
    """Check the entire private snapshot before allowing extraction of any member."""
    actual_sha = hashlib.sha256(data).hexdigest()
    if actual_sha != expected_sha:
        raise InputError(f"{target}: SHA-256 {actual_sha}; expected {expected_sha}")
    if target not in TARGETS:
        raise InputError(f"unsupported target: {target}")
    top = archive_name(target, lane).removesuffix(".tar.xz")
    files = {f"{top}/{name}" for name in lane.manifest}
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
            if member.name in {f"{top}/{name}" for name in lane.executables}:
                if member.mode & 0o111 != 0o111:
                    raise InputError(
                        f"{member.name}: not executable by all installed users"
                    )
                stream = archive.extractfile(member)
                if stream is None:
                    raise InputError(f"{member.name}: no executable contents")
                with stream:
                    contents = stream.read()
                header = contents[:20]
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
                if (
                    lane.build_identity is not None
                    and member.name == f"{top}/bin/ravel"
                    and lane.build_identity.encode() not in contents
                ):
                    raise InputError(
                        f"{member.name}: build identity {lane.build_identity} is not in the executable; "
                        f"the archive is not the recorded build of {lane.source_commit}"
                    )
        yield archive


def validate_and_extract(
    data: bytes,
    target: str,
    expected_sha: str,
    output: Path,
    lane: Lane = LANES["1.1"],
) -> None:
    """Write only admitted regular files/dirs, without tar's link/path extraction semantics."""
    with validate_archive(data, target, expected_sha, lane) as archive:
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


def obtain_archive(target: str, directory: Path, fetch: bool, lane: Lane) -> bytes:
    expected = expected_digest(target, lane)
    path = directory / archive_name(target, lane)
    if path.exists():
        data = path.read_bytes()
    elif not fetch:
        raise InputError(
            f"missing archive: {path}; explicit inputs never download replacements"
        )
    elif lane.release is None:
        raise InputError(
            f"missing archive: {path}; lane {lane.runtime} is a local build and is never "
            f"downloaded. Build it with `task release:archives` in Ravel at "
            f"{lane.source_commit}, then copy it into {directory}"
        )
    else:
        directory.mkdir(parents=True, exist_ok=True)
        url = f"{lane.release}/{path.name}"
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
            with validate_archive(data, target, expected, lane):
                pass
            os.replace(download, path)
    # Cache corruption is a refusal, not an implicit fetch/repair.
    with validate_archive(data, target, expected, lane):
        pass
    return data


def prepare(
    targets: list[str],
    archives: Path,
    output: Path,
    fetch: bool,
    lane: Lane = LANES["1.1"],
) -> None:
    if output.exists() or output.is_symlink():
        raise InputError(
            f"output already exists: {output}; choose a fresh per-run directory"
        )
    # A target without a record refuses before any archive is read.
    for target in targets:
        expected_digest(target, lane)
    output.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(
        prefix=".ravel-inputs-", dir=output.parent
    ) as scratch:
        staged = Path(scratch) / "inputs"
        staged.mkdir()
        (staged / "archives").mkdir()
        # Validate ALL requested archives before extracting any.
        for target in targets:
            data = obtain_archive(target, archives, fetch, lane)
            (staged / "archives" / archive_name(target, lane)).write_bytes(data)
        for target in targets:
            data = (staged / "archives" / archive_name(target, lane)).read_bytes()
            validate_and_extract(data, target, lane.pins[target], staged / target, lane)
        os.rename(staged, output)
    for target in targets:
        print(
            f"{target}\t{output / target / archive_name(target, lane).removesuffix('.tar.xz') / 'bin/ravel'}"
        )


def status(targets: list[str], lane: Lane) -> None:
    """One line per target: its archive name, or why the lane does not run there."""
    for target in targets:
        if target in lane.pins:
            print(f"{target}\trecorded\t{archive_name(target, lane)}")
        else:
            print(f"{target}\tnot recorded\t{lane.unrecorded[target]}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--lane",
        choices=list(LANES),
        default="1.1",
        help="the Ravel runtime whose recorded archives to admit (default 1.1)",
    )
    parser.add_argument(
        "--archives",
        type=Path,
        help="offline input directory; missing/corrupt inputs fail without downloading",
    )
    parser.add_argument(
        "--status",
        action="store_true",
        help="print which targets the lane records, and why the others do not run",
    )
    parser.add_argument(
        "--output",
        type=Path,
        help="fresh output directory (must not exist)",
    )
    parser.add_argument("targets", nargs="*", choices=list(TARGETS))
    args = parser.parse_args()
    lane = LANES[args.lane]
    targets = args.targets or list(TARGETS)
    if len(targets) != len(set(targets)):
        parser.error("duplicate targets")
    if args.status:
        status(targets, lane)
        return 0
    if args.output is None:
        parser.error("--output is required")
    try:
        prepare(
            targets,
            args.archives or ROOT / lane.default_archives,
            args.output.absolute(),
            fetch=args.archives is None,
            lane=lane,
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
