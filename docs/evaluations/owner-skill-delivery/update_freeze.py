"""Record and check the owner's update freeze for Claude Code.

owner-marketplace-freeze-k253 froze automatic plugin updates and held Claude
Code's version. docs/owner-cutover/owner-checklist.md records the setting.
This script only reads. It has three subcommands:

- baseline prints a JSON record of the frozen state on standard output.
- check compares the current state with a baseline. It prints one line for
  each difference and exits 1 when a difference breaks the freeze.
- controls builds a fixture HOME and shows that check fails for each class of
  change and passes for an unchanged HOME.

The record holds:

- The freeze settings: env.DISABLE_AUTOUPDATER and the autoUpdate value of
  each extraKnownMarketplaces entry in ~/.claude/settings.json.
- The launcher target of ~/.local/bin/claude, the output of claude --version
  and the entries of ~/.local/share/claude/versions.
- Each version directory under ~/.claude/plugins/cache/<marketplace>/<plugin>
  and whether it holds .orphaned_at.
- The version and installPath of each entry of installed_plugins.json, and
  lastUpdated of each entry of known_marketplaces.json.

A new version directory, a new orphan marker, a changed installation or a
refreshed marketplace breaks the freeze. Claude Code's cleanup deletes a
directory 14 days after its orphan marker, so the removal of a directory that
already had a marker in the baseline is reported as cleanup and passes. Paths
use ~ for HOME.
"""

from __future__ import annotations

import argparse
import json
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

MARKETPLACES = ("claude-plugins-official", "linkuistics", "quint-llm-kit")


def tilde(path: Path, home: Path) -> str:
    text = str(path)
    return "~" + text[len(str(home)):] if text.startswith(str(home)) else text


def read_json(path: Path) -> dict:
    try:
        return json.loads(path.read_text())
    except FileNotFoundError:
        return {}


def claude_version(home: Path, run: bool) -> str | None:
    launcher = home / ".local/bin/claude"
    if not run or not launcher.exists():
        return None
    out = subprocess.run([str(launcher), "--version"], capture_output=True,
                         text=True, timeout=60, check=False)
    return out.stdout.strip() or None


def record(home: Path, run: bool) -> dict:
    claude = home / ".claude"
    settings = read_json(claude / "settings.json")
    launcher = home / ".local/bin/claude"
    versions = home / ".local/share/claude/versions"
    cache = claude / "plugins/cache"
    dirs = {}
    if cache.is_dir():
        for vdir in sorted(cache.glob("*/*/*")):
            if vdir.is_dir():
                dirs[tilde(vdir, home)] = (vdir / ".orphaned_at").exists()
    installed = {}
    for pid, entries in read_json(claude / "plugins/installed_plugins.json").get("plugins", {}).items():
        installed[pid] = sorted(
            f"{e.get('scope')}|{e.get('version')}|{tilde(Path(e.get('installPath', '')), home)}"
            for e in entries)
    known = {name: entry.get("lastUpdated")
             for name, entry in read_json(claude / "plugins/known_marketplaces.json").items()}
    return {
        "freeze": {
            "DISABLE_AUTOUPDATER": settings.get("env", {}).get("DISABLE_AUTOUPDATER"),
            "autoUpdate": {name: entry.get("autoUpdate") for name, entry
                           in sorted(settings.get("extraKnownMarketplaces", {}).items())},
        },
        "claude": {
            "launcher": tilde(Path(os.readlink(launcher)), home) if launcher.is_symlink() else None,
            "version": claude_version(home, run),
            "versions": sorted(p.name for p in versions.iterdir()) if versions.is_dir() else [],
        },
        "cache": dirs,
        "installed": installed,
        "known_marketplaces": known,
    }


def compare(before: dict, after: dict) -> tuple[list[str], list[str]]:
    """Return (breaks, notes)."""
    breaks: list[str] = []
    notes: list[str] = []
    freeze = after["freeze"]
    if freeze["DISABLE_AUTOUPDATER"] != "1":
        breaks.append(f"setting\tenv.DISABLE_AUTOUPDATER is {freeze['DISABLE_AUTOUPDATER']!r}")
    for name in MARKETPLACES:
        if freeze["autoUpdate"].get(name) is not False:
            breaks.append(f"setting\textraKnownMarketplaces.{name}.autoUpdate is "
                          f"{freeze['autoUpdate'].get(name)!r}")
    for key in ("launcher", "version"):
        if before["claude"][key] != after["claude"][key]:
            breaks.append(f"claude\t{key} {before['claude'][key]!r} -> {after['claude'][key]!r}")
    for name in sorted(set(after["claude"]["versions"]) - set(before["claude"]["versions"])):
        breaks.append(f"claude\tnew version directory {name}")
    for name in sorted(set(before["claude"]["versions"]) - set(after["claude"]["versions"])):
        notes.append(f"claude\tremoved version directory {name}")
    b, a = before["cache"], after["cache"]
    for path in sorted(set(a) - set(b)):
        breaks.append(f"cache\tnew version directory {path}")
    for path in sorted(set(a) & set(b)):
        if a[path] and not b[path]:
            breaks.append(f"cache\tnew orphan marker {path}/.orphaned_at")
        elif b[path] and not a[path]:
            breaks.append(f"cache\torphan marker removed {path}/.orphaned_at")
    for path in sorted(set(b) - set(a)):
        if b[path]:
            notes.append(f"cache\tcleanup removed orphaned {path}")
        else:
            breaks.append(f"cache\tremoved live version directory {path}")
    for pid in sorted(set(before["installed"]) | set(after["installed"])):
        if before["installed"].get(pid) != after["installed"].get(pid):
            breaks.append(f"installed\t{pid} {before['installed'].get(pid)} -> "
                          f"{after['installed'].get(pid)}")
    for name in sorted(set(before["known_marketplaces"]) | set(after["known_marketplaces"])):
        old, new = before["known_marketplaces"].get(name), after["known_marketplaces"].get(name)
        if old != new:
            breaks.append(f"marketplace\t{name} lastUpdated {old} -> {new}")
    return breaks, notes


def check(baseline: Path, home: Path, run: bool) -> int:
    before = json.loads(baseline.read_text())
    breaks, notes = compare(before, record(home, run))
    for line in breaks:
        print("BREAK\t" + line)
    for line in notes:
        print("NOTE\t" + line)
    print(f"{len(breaks)} breaks, {len(notes)} notes")
    return 1 if breaks else 0


def fixture(home: Path) -> None:
    (home / ".claude/plugins/cache/claude-plugins-official/context7/aaa").mkdir(parents=True)
    old = home / ".claude/plugins/cache/linkuistics/grove/old"
    old.mkdir(parents=True)
    (old / ".orphaned_at").write_text("1")
    (home / ".claude/plugins/cache/linkuistics/grove/new").mkdir(parents=True)
    (home / ".local/share/claude/versions").mkdir(parents=True)
    (home / ".local/share/claude/versions/2.1.322").write_text("")
    (home / ".local/bin").mkdir(parents=True)
    (home / ".local/bin/claude").symlink_to(home / ".local/share/claude/versions/2.1.322")
    settings = {"env": {"DISABLE_AUTOUPDATER": "1"}, "extraKnownMarketplaces": {
        name: {"source": {"source": "github", "repo": f"x/{name}"}, "autoUpdate": False}
        for name in MARKETPLACES}}
    (home / ".claude/settings.json").write_text(json.dumps(settings))
    (home / ".claude/plugins/installed_plugins.json").write_text(json.dumps({"plugins": {
        "grove@linkuistics": [{"scope": "user", "version": "new",
                               "installPath": str(home / ".claude/plugins/cache/linkuistics/grove/new")}]}}))
    (home / ".claude/plugins/known_marketplaces.json").write_text(json.dumps(
        {name: {"lastUpdated": "2026-11-11T00:00:00Z"} for name in MARKETPLACES}))


def controls() -> int:
    def mutate_settings(home: Path, edit) -> None:
        path = home / ".claude/settings.json"
        data = json.loads(path.read_text())
        edit(data)
        path.write_text(json.dumps(data))

    def mutate_json(home: Path, rel: str, edit) -> None:
        path = home / rel
        data = json.loads(path.read_text())
        edit(data)
        path.write_text(json.dumps(data))

    def new_claude(home: Path) -> None:
        (home / ".local/share/claude/versions/2.1.323").write_text("")
        (home / ".local/bin/claude").unlink()
        (home / ".local/bin/claude").symlink_to(home / ".local/share/claude/versions/2.1.323")

    cases = {
        "unchanged": (lambda h: None, 0),
        "cleanup of an orphaned directory": (
            lambda h: shutil.rmtree(h / ".claude/plugins/cache/linkuistics/grove/old"), 0),
        "env removed": (lambda h: mutate_settings(h, lambda d: d["env"].pop("DISABLE_AUTOUPDATER")), 1),
        "official autoUpdate on": (lambda h: mutate_settings(
            h, lambda d: d["extraKnownMarketplaces"]["claude-plugins-official"].update(autoUpdate=True)), 1),
        "new Claude Code version": (new_claude, 1),
        "new plugin version directory": (lambda h: (
            h / ".claude/plugins/cache/claude-plugins-official/context7/bbb").mkdir(), 1),
        "new orphan marker": (lambda h: (
            h / ".claude/plugins/cache/linkuistics/grove/new/.orphaned_at").write_text("1"), 1),
        "live directory removed": (lambda h: shutil.rmtree(
            h / ".claude/plugins/cache/claude-plugins-official/context7/aaa"), 1),
        "installation changed": (lambda h: mutate_json(
            h, ".claude/plugins/installed_plugins.json",
            lambda d: d["plugins"]["grove@linkuistics"][0].update(version="newer")), 1),
        "marketplace refreshed": (lambda h: mutate_json(
            h, ".claude/plugins/known_marketplaces.json",
            lambda d: d["linkuistics"].update(lastUpdated="2026-11-12T00:00:00Z")), 1),
    }
    failed = 0
    for name, (change, expected) in cases.items():
        with tempfile.TemporaryDirectory() as tmp:
            home = Path(tmp).resolve()
            fixture(home)
            baseline = Path(tmp) / "baseline.json"
            baseline.write_text(json.dumps(record(home, run=False)))
            change(home)
            breaks, _ = compare(json.loads(baseline.read_text()), record(home, run=False))
            got = 1 if breaks else 0
            status = "ok" if got == expected else "FAIL"
            failed += got != expected
            print(f"{status}\t{name}\texpected exit {expected}, got {got}")
    print(f"{len(cases) - failed}/{len(cases)} controls behaved as expected")
    return 1 if failed else 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    sub = parser.add_subparsers(dest="command", required=True)
    base = sub.add_parser("baseline", help="print the frozen state as JSON")
    base.add_argument("--home", type=Path, default=Path.home())
    chk = sub.add_parser("check", help="compare the current state with a baseline")
    chk.add_argument("baseline", type=Path)
    chk.add_argument("--home", type=Path, default=Path.home())
    sub.add_parser("controls", help="show in a fixture HOME that check sees each change")
    args = parser.parse_args()
    if args.command == "baseline":
        json.dump(record(args.home, run=True), sys.stdout, indent=2, sort_keys=True)
        print()
        return 0
    if args.command == "check":
        return check(args.baseline, args.home, run=True)
    return controls()


if __name__ == "__main__":
    sys.exit(main())
