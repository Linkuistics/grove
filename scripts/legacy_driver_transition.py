"""Preserve an original 22.1.0 keg during the live driver's Ravel cutover.

No verb invokes Homebrew, modifies the keg, or addresses a session channel.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import shlex
import shutil
import stat
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import TypedDict

VERSION = "22.1.0"
LINKS = ("harness-dispatch", "grove-llm")
REQUIRED = (
    "LICENSE",
    "bin/grove",
    "bin/grove-llm",
    "bin/harness-dispatch",
    "libexec/harness-dispatch/harness-dispatch-policy",
    "libexec/harness-dispatch/sdk/index.ts",
    "libexec/harness-dispatch/sdk/index.d.ts",
    "libexec/harness-dispatch/notices/NOTICES.md",
    "libexec/harness-dispatch/notices/bun-LICENSE.md",
    "libexec/harness-dispatch/notices/sqlite.md",
)
CONTROLS = (
    "GROVE_SIGNAL_FILE",
    "GROVE_RUN_SIGNAL_FILE",
    "GROVE_LAUNCH_DIR",
    "GROVE_HARNESS_PID",
    "GROVE_CLAUDE_PID",
    "GROVE_PROVIDER_EVIDENCE",
)


class TransitionError(ValueError):
    """A preservation precondition is not satisfied; no upgrade is safe yet."""


class Entry(TypedDict):
    kind: str
    mode: int
    value: str


class Manifest(TypedDict):
    version: str
    keg: str
    stable_bin: str
    inherited_path: str
    inventory: dict[str, Entry]


def clean_environment() -> dict[str, str]:
    """Drop old and new inherited authority before any test/helper subprocess."""
    return {
        key: value
        for key, value in os.environ.items()
        if not key.startswith(("GROVE_", "HARNESS_DISPATCH_", "RAVEL_"))
    }


def inventory(root: Path) -> dict[str, Entry]:
    """Digest all entries, including extra keg assets and modes, without following links."""
    if not root.is_dir():
        raise TransitionError(f"inventory keg directory missing: {root}")
    entries: dict[str, Entry] = {}
    for path in [root, *sorted(root.rglob("*"))]:
        info = path.lstat()
        mode = stat.S_IMODE(info.st_mode)
        name = str(path.relative_to(root))
        if stat.S_ISLNK(info.st_mode):
            target = os.readlink(path)
            if Path(target).is_absolute() or not path.resolve().is_relative_to(root.resolve()):
                raise TransitionError(f"external keg symlink refuses complete preservation: {path}")
            entry: Entry = {"kind": "link", "mode": mode, "value": target}
        elif stat.S_ISDIR(info.st_mode):
            entry = {"kind": "directory", "mode": mode, "value": ""}
        elif stat.S_ISREG(info.st_mode):
            with path.open("rb") as stream:
                digest = hashlib.file_digest(stream, "sha256").hexdigest()
            entry = {"kind": "file", "mode": mode, "value": digest}
        else:
            raise TransitionError(f"unsupported keg inventory entry: {path}")
        entries[name] = entry
    return entries


def check_path(stable: Path) -> None:
    if not stable.is_dir():
        raise TransitionError(f"stable bin must already be a directory: {stable}")
    if not any(
        Path(entry).is_absolute() and Path(entry).resolve() == stable
        for entry in os.environ.get("PATH", "").split(os.pathsep)
        if entry
    ):
        raise TransitionError(f"stable bin is not on inherited PATH: {stable}")


def check_cleanup() -> None:
    # Homebrew's two separate protections: https://docs.brew.sh/Manpage
    protected = os.environ.get("HOMEBREW_NO_CLEANUP_FORMULAE", "").split(",")
    if not os.environ.get("HOMEBREW_NO_INSTALL_CLEANUP") or "grove" not in protected:
        raise TransitionError(
            "cleanup protection requires HOMEBREW_NO_INSTALL_CLEANUP=1 "
            "and HOMEBREW_NO_CLEANUP_FORMULAE containing grove"
        )


def check_versions(keg: Path) -> None:
    for name in ("grove", *LINKS):
        result = subprocess.run(
            [str(keg / "bin" / name), "--version"],
            env=clean_environment(),
            capture_output=True,
            text=True,
            timeout=15,
            check=False,
        )
        if result.returncode or result.stdout.strip() != f"{name} {VERSION}":
            raise TransitionError(f"required original {name} {VERSION}: {keg / 'bin' / name}")


def prepare(keg: Path, stable: Path, output: Path) -> None:
    keg, stable, output = keg.resolve(), stable.resolve(), output.absolute()
    if output.exists() or output.is_symlink():
        raise TransitionError(f"output exists; choose a fresh directory: {output}")
    if output.resolve().is_relative_to(keg):
        raise TransitionError("preparation output must be outside the original keg")
    check_path(stable)
    before = inventory(keg)
    for name in REQUIRED:
        if name not in before or before[name]["kind"] != "file":
            raise TransitionError(f"required original keg member missing: {name}")
    for name in ("bin/grove", "bin/grove-llm", "bin/harness-dispatch", REQUIRED[4]):
        if not os.access(keg / name, os.X_OK):
            raise TransitionError(f"required executable member: {name}")
    check_versions(keg)
    output.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix=".legacy-preserve-", dir=output.parent) as temporary:
        staged = Path(temporary) / "plan"
        staged.mkdir()
        shutil.copytree(keg, staged / "keg", symlinks=True)
        if inventory(staged / "keg") != before or inventory(keg) != before:
            raise TransitionError("inventory changed while preparing complete keg")
        manifest: Manifest = {
            "version": VERSION,
            "keg": str(keg),
            "stable_bin": str(stable),
            "inherited_path": os.environ["PATH"],
            "inventory": before,
        }
        (staged / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
        # Preserve any pre-existing formula exclusions in the shell doing the release.
        (staged / "release-env.sh").write_text(
            "# Source in EVERY release/recovery shell while the original driver is live.\n"
            "export HOMEBREW_NO_INSTALL_CLEANUP=1\n"
            'export HOMEBREW_NO_CLEANUP_FORMULAE="${HOMEBREW_NO_CLEANUP_FORMULAE:+'
            '$HOMEBREW_NO_CLEANUP_FORMULAE,}grove"\n'
            f"export GROVE_ORIGINAL_LLM={shlex.quote(str(keg / 'bin/grove-llm'))}\n"
            f"export GROVE_ORIGINAL_DISPATCH={shlex.quote(str(keg / 'bin/harness-dispatch'))}\n"
        )
        os.rename(staged, output)


def load(plan: Path) -> Manifest:
    manifest: Manifest = json.loads((plan / "manifest.json").read_text())
    if manifest["version"] != VERSION:
        raise TransitionError("plan version is not the original 22.1.0 contract")
    return manifest


def check_inventory(plan: Path, manifest: Manifest) -> None:
    for root in (Path(manifest["keg"]), plan / "keg"):
        if inventory(root) != manifest["inventory"]:
            raise TransitionError(f"inventory differs from prepared original: {root}")


def owned_links(manifest: Manifest) -> list[tuple[Path, Path]]:
    return [
        (Path(manifest["stable_bin"]) / name, Path(manifest["keg"]) / "bin" / name)
        for name in LINKS
    ]


def link_identity(link: Path) -> list[int]:
    info = link.lstat()
    return [info.st_dev, info.st_ino, info.st_ctime_ns, info.st_mtime_ns]


def check_links(plan: Path, manifest: Manifest) -> None:
    receipt: dict[str, list[int]] = json.loads((plan / "links.json").read_text())
    for link, target in owned_links(manifest):
        if (
            not link.is_symlink()
            or os.readlink(link) != str(target)
            or receipt.get(link.name) != link_identity(link)
        ):
            raise TransitionError(f"link is not owned by this plan: {link}")


def verify(plan: Path) -> None:
    manifest = load(plan)
    check_cleanup()
    check_path(Path(manifest["stable_bin"]))
    check_inventory(plan, manifest)
    check_links(plan, manifest)
    for link, _ in owned_links(manifest):
        found = shutil.which(link.name)
        if found is None or Path(found).parent.resolve() / link.name != link:
            raise TransitionError(f"inherited PATH shadows preserved command: {link.name}: {found}")


def apply(plan: Path) -> None:
    manifest = load(plan)
    check_cleanup()
    check_path(Path(manifest["stable_bin"]))
    check_inventory(plan, manifest)
    receipt = plan / "links.json"
    if receipt.exists() or receipt.is_symlink():
        raise TransitionError(
            f"existing application receipt; inspect original application: {receipt}"
        )
    for link, _ in owned_links(manifest):
        if link.exists() or link.is_symlink():
            raise TransitionError(f"refusing existing PATH entry (never overwrite): {link}")
    created: list[tuple[Path, Path]] = []
    created_receipt = False
    try:
        for link, target in owned_links(manifest):
            link.symlink_to(target)
            created.append((link, target))
        with receipt.open("x") as stream:
            created_receipt = True
            json.dump({link.name: link_identity(link) for link, _ in created}, stream, indent=2)
            stream.write("\n")
        verify(plan)
    except (OSError, TransitionError):
        for link, target in reversed(created):
            if link.is_symlink() and os.readlink(link) == str(target):
                link.unlink()
        if created_receipt:
            receipt.unlink()
        raise


def cleanup(plan: Path, driver_ended: bool) -> None:
    if not driver_ended:
        raise TransitionError("cleanup requires --driver-ended after the original driver is reaped")
    manifest = load(plan)
    check_links(plan, manifest)  # Check both before touching either; retain owner's replacements.
    for link, _ in owned_links(manifest):
        link.unlink()


def main() -> int:
    parser = argparse.ArgumentParser(
        description=__doc__,
        epilog="Exit 0 success, 1 refusal/I/O failure, 2 usage. Prepare/apply refuse repeats. "
        "Cleanup removes only matching links, never the keg. See docs/legacy-driver-transition.md.",
    )
    verbs = parser.add_subparsers(dest="verb", required=True)
    prep = verbs.add_parser("prepare", help="read/copy complete original keg to a fresh plan")
    prep.add_argument("--keg", type=Path, required=True)
    prep.add_argument("--stable-bin", type=Path, required=True)
    prep.add_argument("--output", type=Path, required=True)
    for verb in ("apply", "verify", "cleanup"):
        command = verbs.add_parser(verb)
        command.add_argument("--plan", type=Path, required=True)
        if verb == "cleanup":
            command.add_argument("--driver-ended", action="store_true")
    args = parser.parse_args()
    try:
        if args.verb == "prepare":
            prepare(args.keg, args.stable_bin, args.output)
        elif args.verb == "apply":
            apply(args.plan)
        elif args.verb == "verify":
            verify(args.plan)
        else:
            cleanup(args.plan, args.driver_ended)
    except (OSError, ValueError, KeyError, subprocess.TimeoutExpired) as error:
        print(f"legacy-transition: {error}", file=sys.stderr)
        return 1
    print(json.dumps({"action": args.verb, "result": "ok"}))
    return 0


if __name__ == "__main__":
    sys.exit(main())
