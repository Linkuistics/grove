"""Controls for archive admission before any file is extracted or invoked."""

from __future__ import annotations

import hashlib
import io
import subprocess
import sys
import tarfile
import tempfile
from collections.abc import Iterator
from contextlib import contextmanager
from pathlib import Path

import ravel_target_inputs as inputs

TARGET = "aarch64-unknown-linux-gnu"
TOP = f"ravel-v1.1.0-{TARGET}"
# Independent published manifest: omitting validation of any member must fail.
FILES = [
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
]
EXECUTABLES = {"bin/ravel", "libexec/ravel/ravel-policy"}


def archive_bytes(change: str = "", subject: str = "") -> bytes:
    """Hand-built tar, including valid aarch64 ELF headers for both programs."""
    members: dict[str, tuple[bytes, bytes, int, str]] = {}
    directories = {TOP}
    for name in FILES:
        path = Path(f"{TOP}/{name}")
        directories.update(str(parent) for parent in path.parents if str(parent) != ".")
        data = b"fixture"
        if name in EXECUTABLES:
            data = b"\x7fELF\x02\x01\x01" + bytes(9) + b"\x02\x00\xb7\x00"
        members[str(path)] = (
            data,
            tarfile.REGTYPE,
            0o755 if name in EXECUTABLES else 0o644,
            "",
        )
    for directory in directories:
        members[directory] = (b"", tarfile.DIRTYPE, 0o755, "")
    path = f"{TOP}/{subject or 'bin/ravel'}"
    if change == "missing":
        del members[path]
    elif change in {"extra", "traversal", "absolute", "appledouble", "wrong-top"}:
        path = {
            "extra": f"{TOP}/extra",
            "traversal": f"{TOP}/../escaped",
            "absolute": "/escaped",
            "appledouble": f"{TOP}/._LICENSE",
            "wrong-top": "ravel-v1.1.0-x86_64-unknown-linux-gnu/",
        }[change]
        members[path] = (b"bad", tarfile.REGTYPE, 0o644, "")
    elif change in {"symlink", "hardlink", "fifo", "directory", "mode", "architecture"}:
        data, kind, mode, link = members[path]
        if change == "mode":
            mode = 0o644
        elif change == "architecture":
            data = data[:18] + b"\x3e\x00"
        else:
            kind = {
                "symlink": tarfile.SYMTYPE,
                "hardlink": tarfile.LNKTYPE,
                "fifo": tarfile.FIFOTYPE,
                "directory": tarfile.DIRTYPE,
            }[change]
            data, link = b"", "/escaped"
        members[path] = (data, kind, mode, link)
    buffer = io.BytesIO()
    with tarfile.open(fileobj=buffer, mode="w:xz") as archive:
        for name, (data, kind, mode, link) in sorted(members.items()):
            member = tarfile.TarInfo(name)
            member.type, member.mode, member.linkname, member.size = (
                kind,
                mode,
                link,
                len(data),
            )
            archive.addfile(member, io.BytesIO(data))
            if change == "duplicate" and name == path:
                archive.addfile(member, io.BytesIO(data))
    return buffer.getvalue()


def test_complete_layout_is_admitted_and_extracted(tmp_path: Path) -> None:
    data = archive_bytes()
    inputs.validate_and_extract(
        data, TARGET, hashlib.sha256(data).hexdigest(), tmp_path
    )
    assert (tmp_path / TOP / "bin/ravel").read_bytes()[18:20] == b"\xb7\x00"
    assert sorted(
        str(path.relative_to(tmp_path / TOP))
        for path in (tmp_path / TOP).rglob("*")
        if path.is_file()
    ) == sorted(FILES)


def test_each_missing_member_is_refused_before_extraction(
    tmp_path: Path, subject: str
) -> None:
    data = archive_bytes("missing", subject)
    with refused("manifest"):
        inputs.validate_and_extract(
            data, TARGET, hashlib.sha256(data).hexdigest(), tmp_path
        )
    assert list(tmp_path.iterdir()) == []


def test_invalid_members_are_refused_before_extraction(
    tmp_path: Path, change: str
) -> None:
    data = archive_bytes(change)
    with refused():
        inputs.validate_and_extract(
            data, TARGET, hashlib.sha256(data).hexdigest(), tmp_path
        )
    assert list(tmp_path.iterdir()) == []


def test_digest_is_checked_before_parsing_or_extracting(tmp_path: Path) -> None:
    with refused("SHA-256"):
        inputs.validate_and_extract(b"not even a tar", TARGET, "0" * 64, tmp_path)
    assert list(tmp_path.iterdir()) == []


def test_cli_missing_archive_does_not_download_or_create_layout(tmp_path: Path) -> None:
    archive_dir = tmp_path / "archives"
    archive_dir.mkdir()
    output = tmp_path / "output"
    result = subprocess.run(
        [
            sys.executable,
            str(Path(inputs.__file__)),
            "--archives",
            str(archive_dir),
            "--output",
            str(output),
            TARGET,
        ],
        capture_output=True,
        text=True,
        check=False,
    )
    assert result.returncode != 0
    assert "missing archive" in result.stderr
    assert not output.exists()


def test_cli_unpinned_archive_cannot_publish_partial_layout(tmp_path: Path) -> None:
    archive_dir = tmp_path / "archives"
    archive_dir.mkdir()
    (archive_dir / f"{TOP}.tar.xz").write_bytes(archive_bytes())
    output = tmp_path / "output"
    result = subprocess.run(
        [
            sys.executable,
            str(Path(inputs.__file__)),
            "--archives",
            str(archive_dir),
            "--output",
            str(output),
            TARGET,
        ],
        capture_output=True,
        text=True,
        check=False,
    )
    assert result.returncode != 0
    assert "SHA-256" in result.stderr
    assert not output.exists()


@contextmanager
def refused(message: str = "") -> Iterator[None]:
    try:
        yield
    except inputs.InputError as error:
        assert message in str(error), (message, str(error))
    else:
        raise AssertionError("invalid archive was admitted")


def main() -> None:
    cases = [
        (test_complete_layout_is_admitted_and_extracted, ()),
        (test_digest_is_checked_before_parsing_or_extracting, ()),
        (test_cli_missing_archive_does_not_download_or_create_layout, ()),
        (test_cli_unpinned_archive_cannot_publish_partial_layout, ()),
    ]
    cases += [
        (test_each_missing_member_is_refused_before_extraction, (name,))
        for name in FILES
    ]
    cases += [
        (test_invalid_members_are_refused_before_extraction, (change,))
        for change in [
            "extra",
            "duplicate",
            "traversal",
            "absolute",
            "appledouble",
            "wrong-top",
            "symlink",
            "hardlink",
            "fifo",
            "directory",
            "mode",
            "architecture",
        ]
    ]
    for test, arguments in cases:
        with tempfile.TemporaryDirectory() as directory:
            test(Path(directory), *arguments)
        print(f"PASS {test.__name__}{arguments}")
    print(f"ravel archive controls: {len(cases)} passed")


if __name__ == "__main__":
    main()
