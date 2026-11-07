#!/usr/bin/env python3
"""Rehearse the owner switch program on a fixture plan in disposable layouts.

docs/specs/owner-skill-delivery.md, "The rehearsed plan" and "Test seams",
owns the controls. Each control builds a new disposable HOME under OUT, runs
scripts/owner_switch.py under a fixture harness process, and checks the
layout, the run record and the program's output. A hook acts at the
program's named points: it crashes the program, writes as another program,
holds a descriptor or launches a process. The real HOME is never read or
changed.

Usage:
    python3 -I owner_switch_rehearsal.py rehearse --out OUT [--program PATH] [--only REGEX]
                                                  [--settings-writes]

The record OUT/record.json names the digests of the plan and the program and
the result of each control. For each run of a control, it counts the Claude
Code commands that the run ran. It is a passing rehearsal record of the
program only when every control passed. OUT/plan.json is the fixture plan.

Each run finds stub `claude` and `brew` commands first on PATH. The stub
`claude` keeps its plugin state in the disposable HOME, and it writes
settings.json as Claude Code 2.1.319 does: one read, then a new file renamed
onto the path. The point "stub:<command>:read" lies between that read and
the rename.

--settings-writes also reruns
docs/evaluations/hooks-installation-writes/settings-writes.sh with the real
`claude` on PATH, in OUT/settings-writes. The record then names that Claude
Code version. The rerun passes when `marketplace add` and `plugin install`
each rename one file onto the path, and when each command's measured
interval meets the evidence's range of 30 to 38 ms. The interval of a
command lies between the largest lead at which a race lost the outside write
and the smallest lead at which the live file kept it.

The fixture plan names the program's own gates of steps 4 to 7. Each run
therefore finds a fixture Homebrew prefix in the case: five kegs with the ten
sources, the links of `brew link`, `bin/ravel` for the promoted Ravel and
Grove's `bin/grove`. Stub `mise` and `brew info` read that prefix, and the
control file can change their answers. The stub `ravel` digests each source's
files, so a one-byte change changes its resourceDigest. It refuses the
account's HOME, the switch's HOME and every RAVEL_* name, and each run sets
one such name, so each resolution shows the gate's environment rule.

The controls cover the switch, the inverse, detection, interruptions,
entries, step 14, the processes, the rehearsal condition, the preludes of
step 3, its baseline rows and the gates with their source controls. The
owner's inverse and the merge entry add their controls in a later slice.
"""
from __future__ import annotations

import argparse
import json
import os
import platform
import re
import shutil
import signal
import subprocess
import sys
import threading
import time

HERE = os.path.dirname(os.path.realpath(__file__))
SELF = os.path.realpath(__file__)
PY = sys.executable
HARNESS_MARK = "owner-switch-fixture-harness"
WRITER_MARK = "owner-switch-fixture-writer"
JOB_MARK = "owner-switch-fixture-job"
LOOP_CONTROL = (
    "GROVE_SIGNAL_FILE", "GROVE_RUN_SIGNAL_FILE", "GROVE_LAUNCH_DIR",
    "HARNESS_DISPATCH_EXIT_FILE", "HARNESS_DISPATCH_RUN_ID",
    "HARNESS_DISPATCH_STATE_DIR", "RAVEL_EXIT_FILE", "GROVE_PROVIDER_EVIDENCE",
    "GROVE_HARNESS_PID", "GROVE_CLAUDE_PID",
)
STATE = ".local/state/owner-switch"
MANUAL = "notes/manual.md"
OFF = {"schemaVersion": 1, "sources": []}
SWITCHED = {"schemaVersion": 1, "mandatory": ["superpowers/skills/using-superpowers/SKILL.md"]}
SUPERSEDED_PLUGIN = ["enabledPlugins", "grove@linkuistics"]
HOOKS_ID = "technical-communication-hooks@writing-hooks"
HOOKS_PLUGIN = ["enabledPlugins", HOOKS_ID]
PLUGIN_FILES = {
    ".claude-plugin/plugin.json": '{"name": "technical-communication-hooks"}\n',
    "hooks/hooks.json": '{"hooks": {}}\n',
}
PLUGIN_SOURCE = "skills/writing/craft/technical-communication"
PLUGIN_CACHE = ".claude/plugins/cache/writing-hooks/technical-communication-hooks/unknown"
# The fixture kegs of the ten sources: formula -> (version, source IDs).
FORMULA_OF_GROVE = "linkuistics/taps/grove"
KEGS = {
    "grove": ("1", ["design", "engineering", "grove-core", "requirements", "research", "writing"]),
    "personal": ("2", ["personal"]),
    "testanyware": ("3", ["testanyware"]),
    "superpowers": ("4", ["superpowers"]),
    "quint-llm-kit": ("5", ["quint"]),
}
# The plugins that the fixture enables, with their registered version directories.
READERS = {
    "grove@linkuistics": ".claude/plugins/cache/linkuistics/grove/0a1b2c3d",
    "context7@claude-plugins-official": ".claude/plugins/cache/claude-plugins-official/context7/4e5f6a7b",
}


# --- the fixture layout --------------------------------------------------------

def write(path: str, text: str) -> None:
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w") as f:
        f.write(text)


def write_script(path: str, text: str) -> None:
    write(path, text)
    os.chmod(path, 0o755)


def build_prefix(prefix: str) -> str:
    """A Homebrew prefix with the five skill formulae's kegs, linked as `brew link` links them.

    Grove's keg holds the writing-hooks marketplace, `bin/grove` and six
    sources. `bin/ravel` stands in for the promoted Ravel, and
    `var/homebrew/linked` records each linked keg, as Homebrew does.
    """
    keg = os.path.join(prefix, "Cellar/grove/1")
    write(os.path.join(keg, ".claude-plugin/marketplace.json"), json.dumps({
        "name": "writing-hooks", "owner": {"name": "Linkuistics"},
        "plugins": [{"name": "technical-communication-hooks", "source": "./" + PLUGIN_SOURCE}],
    }) + "\n")
    for rel, text in PLUGIN_FILES.items():
        write(os.path.join(keg, PLUGIN_SOURCE, rel), text)
    write_script(os.path.join(keg, "bin/grove"), "#!/bin/sh\necho 'grove 1'\n")
    for folder in ("opt", "share/ravel/skills", "var/homebrew/linked"):
        os.makedirs(os.path.join(prefix, folder), exist_ok=True)
    for name, (version, ids) in KEGS.items():
        for sid in ids:
            rel = f"Cellar/{name}/{version}/share/ravel/skills/{sid}"
            write(os.path.join(prefix, rel, f"skills/{sid}-skill/SKILL.md"),
                  f"---\nname: {sid}-skill\n---\nThe fixture source {sid}.\n")
            os.symlink(f"../../../{rel}", os.path.join(prefix, "share/ravel/skills", sid))
        os.symlink(f"../Cellar/{name}/{version}", os.path.join(prefix, "opt", name))
        os.symlink(f"../../../Cellar/{name}/{version}", os.path.join(prefix, "var/homebrew/linked", name))
    write_script(os.path.join(prefix, "bin/ravel"), f"#!/bin/sh\nexec '{PY}' -I '{SELF}' stub-ravel \"$0\" \"$@\"\n")
    return os.path.join(prefix, "opt/grove")


def source_digest(path: str) -> str:
    """The fixture resourceDigest of a source: each relative path, link target and file content."""
    import hashlib
    h = hashlib.sha256()
    root = os.path.realpath(path)
    for folder, dirs, files in os.walk(root):
        dirs.sort()
        for name in sorted(dirs + files):
            p = os.path.join(folder, name)
            h.update(os.path.relpath(p, root).encode() + b"\0")
            if os.path.islink(p):
                h.update(b"link:" + os.readlink(p).encode() + b"\0")
            elif os.path.isfile(p):
                with open(p, "rb") as f:
                    h.update(f.read() + b"\0")
    return h.hexdigest()


def build_home(home: str, marketplace: str, ravel: str = "off", hooks: str = "disabled") -> None:
    """The fixture HOME. ravel is off, absent-key or absent-file; hooks is
    disabled, enabled or absent. The defaults are the layout after the preludes."""
    settings = {"enabledPlugins": {"grove@linkuistics": True, "context7@claude-plugins-official": True},
                "theme": "dark"}
    if hooks != "absent":
        settings["enabledPlugins"][HOOKS_ID] = hooks == "enabled"
        settings["extraKnownMarketplaces"] = {"writing-hooks": {"source": {"source": "directory",
                                                                             "path": marketplace}}}
        save_json(os.path.join(home, ".claude/plugins/known_marketplaces.json"), {"writing-hooks": {
            "source": {"source": "directory", "path": marketplace}, "installLocation": marketplace}})
    plugins = {pid: [{"scope": "user", "version": os.path.basename(rel), "installPath": os.path.join(home, rel)}]
               for pid, rel in READERS.items()}
    for rel in READERS.values():
        write(os.path.join(home, rel, ".claude-plugin/plugin.json"), "{}\n")
    if hooks != "absent":
        plugins[HOOKS_ID] = [{"scope": "user", "version": "unknown", "installPath": os.path.join(home, PLUGIN_CACHE)}]
        for rel, text in PLUGIN_FILES.items():
            write(os.path.join(home, PLUGIN_CACHE, rel), text)
    save_json(os.path.join(home, ".claude/plugins/installed_plugins.json"), {"version": 2, "plugins": plugins})
    write(os.path.join(home, ".claude/settings.json"), json.dumps(settings, indent=2) + "\n")
    write(os.path.join(home, ".config/ravel/config.rhai"), "// fixture\n")
    if ravel != "absent-file":
        body = {"skills": OFF, "policy": "config.rhai"} if ravel == "off" else {"policy": "config.rhai"}
        write(os.path.join(home, ".config/ravel/settings.json"), json.dumps(body, indent=2) + "\n")
    write(os.path.join(home, ".claude/skills/grove/SKILL.md"), "---\nname: grove\n---\nThe earlier copy.\n")
    os.makedirs(os.path.join(home, ".codex/skills"), exist_ok=True)
    os.symlink("../../.grove-snapshot/skills/grove", os.path.join(home, ".codex/skills/grove"))
    write(os.path.join(home, ".grove-snapshot/skills/grove/SKILL.md"), "---\nname: grove\n---\nSnapshot.\n")
    write(os.path.join(home, ".codex/skills/codebase-memory/SKILL.md"), "---\nname: codebase-memory\n---\n")
    write(os.path.join(home, MANUAL), "# AI Dev Tooling\n")


def fixture_command(name: str) -> list:
    return [PY, "-I", SELF, "fixture-command", name]


def gate_command(step: str, name: str) -> dict:
    return {"name": name, "argv": ["$python", "-I", "$program", "gate", step]}


def build_plan(program: str, out: str) -> str:
    probe = os.path.join(out, "plan-home")
    prefix = os.path.join(out, "plan-prefix")
    for folder in (probe, prefix):
        if os.path.exists(folder):
            shutil.rmtree(folder)
    build_home(probe, build_prefix(prefix))
    sources = {sid: {"formula": f"linkuistics/taps/{name}",
                     "resourceDigest": source_digest(os.path.join(prefix, "share/ravel/skills", sid))}
               for name, (_, ids) in KEGS.items() for sid in ids}

    def ident(rel):
        return subprocess.run([PY, "-I", program, "identity", os.path.join(probe, rel)],
                              capture_output=True, text=True, check=True).stdout.strip()

    plan = {
        "schema": "owner-switch-plan/1",
        "state": STATE,
        "manual": MANUAL,
        "parts": [
            {"kind": "entry", "path": ".claude/skills/grove", "identity": ident(".claude/skills/grove")},
            {"kind": "entry", "path": ".codex/skills/grove", "identity": ident(".codex/skills/grove")},
            {"kind": "file", "path": ".claude/settings.json", "cells": [
                {"key": SUPERSEDED_PLUGIN, "earlier": True, "switched": False},
                {"key": HOOKS_PLUGIN, "earlier": False, "switched": True},
            ]},
            {"kind": "file", "path": ".config/ravel/settings.json", "cells": [
                {"key": ["skills"], "earlier": OFF, "switched": SWITCHED},
            ]},
        ],
        "outside": [
            {"root": ".codex/skills", "entries": {"codebase-memory": ident(".codex/skills/codebase-memory")}},
            {"root": ".claude/skills", "entries": {}},
            {"path": ".claude/plugins/cache/writing-hooks", "identity": ident(".claude/plugins/cache/writing-hooks")},
        ],
        "session": {"harness": [HARNESS_MARK], "mcp": []},
        "watch": [WRITER_MARK],
        "jobs": {"argv": fixture_command("jobs"), "patterns": [JOB_MARK]},
        "baseline": {"name": "baseline", "argv": fixture_command("baseline")},
        "gates": {"4": [gate_command("4", "grove-release")], "5": [gate_command("5", "skill-formulae")],
                  "6": [gate_command("6", "resolved-sources")], "7": [gate_command("7", "reader-evidence")]},
        "accepted": {
            "sources": sources,
            "versions": {f"linkuistics/taps/{name}": version for name, (version, _) in KEGS.items() if name != "grove"},
            "trials": {sid: [{"name": f"trial-{sid}", "argv": fixture_command("trial")}] for sid in KEGS["grove"][1]},
        },
        "probes": [{"name": "probe-switched", "argv": fixture_command("probe-switched")}],
        "native_probe": [{"name": "probe-native", "argv": fixture_command("probe-native")}],
        "mac_check": {"name": "mac-check", "argv": fixture_command("mac-check")},
    }
    shutil.rmtree(probe)
    shutil.rmtree(prefix)
    path = os.path.join(out, "plan.json")
    with open(path, "w") as f:
        json.dump(plan, f, indent=2)
        f.write("\n")
    return path


# --- fixture commands, the hook, the holder and the harness ----------------------

def case_root() -> str:
    return os.path.dirname(os.environ["OWNER_SWITCH_HOME"])


def load(path: str, default):
    try:
        with open(path) as f:
            return json.load(f)
    except FileNotFoundError:
        return default


def save(path: str, value) -> None:
    with open(path, "w") as f:
        json.dump(value, f)


def save_json(path: str, value) -> None:
    write(path, json.dumps(value, indent=2) + "\n")


def take_failure(root: str, name: str) -> bool:
    control = load(os.path.join(root, "control.json"), {})
    used = load(os.path.join(root, "failed.json"), {})
    if used.get(name, 0) < control.get("fail", {}).get(name, 0):
        used[name] = used.get(name, 0) + 1
        save(os.path.join(root, "failed.json"), used)
        return True
    return False


def cmd_fixture(name: str) -> int:
    root = case_root()
    home = os.environ["OWNER_SWITCH_HOME"]
    if name == "jobs":
        for label in load(os.path.join(root, "control.json"), {}).get("jobs", []):
            print(f"-\t0\t{label}")
        print("-\t0\tcom.apple.unrelated")
        return 0
    if take_failure(root, name):
        print(f"{name}: failed by the control")
        return 1
    if name == "baseline":
        # The rows of inventory.py: harness, root, name, ..., disposition.
        print("claude\t~/.claude/skills\tgrove\tskill\tgrove\t0123456789abcdef\t1\tstale\t-\t-\tnative-move")
        print("claude\tbuilt-in\tdebug\tbuilt-in skill\tdebug\tunverified\t0\tplatform\t-\t-\tnative-keep")
        for row in load(os.path.join(root, "control.json"), {}).get("baseline_rows", []):
            print(row)
        return 0
    if name == "trial":
        with open(os.path.join(root, "trials.log"), "a") as f:
            f.write(f"{os.environ['OWNER_SWITCH_SOURCE']} {os.environ['OWNER_SWITCH_DIGEST']}\n")
        return 0
    ravel = load(os.path.join(home, ".config/ravel/settings.json"), {}).get("skills")
    link = os.path.lexists(os.path.join(home, ".codex/skills/grove"))
    if name == "probe-switched":
        ok = ravel == SWITCHED and not link
    elif name == "probe-native":
        ok = ravel == OFF and link
    else:
        ok = True
    print(f"{name}: {'passed' if ok else 'failed'}")
    return 0 if ok else 1


def set_key(obj: dict, key: list, value) -> None:
    for k in key[:-1]:
        obj = obj.setdefault(k, {})
    if value is None:
        obj.pop(key[-1], None)
    else:
        obj[key[-1]] = value


def write_in_place(path: str, key: list, value) -> None:
    """Change one key while keeping the inode, as an editor that writes in place."""
    with open(path, "r+") as f:
        obj = json.load(f)
        set_key(obj, key, value)
        f.seek(0)
        f.truncate()
        f.write(json.dumps(obj, indent=2) + "\n")


def record_pid(root: str, pid: int) -> None:
    with open(os.path.join(root, "pids"), "a") as f:
        f.write(f"{pid}\n")


def wait_for(path: str, seconds: float = 10.0) -> None:
    end = time.time() + seconds
    while not os.path.exists(path):
        if time.time() > end:
            raise SystemExit(f"timed out waiting for {path}")
        time.sleep(0.02)


def cmd_hook(point: str) -> int:
    root = case_root()
    home = os.environ["OWNER_SWITCH_HOME"]
    control = load(os.path.join(root, "control.json"), {})
    fired = load(os.path.join(root, "fired.json"), [])
    with open(os.path.join(root, "points"), "a") as f:
        f.write(point + "\n")
    crash = False
    for i, action in enumerate(control.get("actions", [])):
        if action["point"] != point or i in fired:
            continue
        fired.append(i)
        save(os.path.join(root, "fired.json"), fired)
        do = action["do"]
        path = os.path.join(home, action.get("path", ""))
        if do == "crash":
            crash = True
        elif do == "write-key":
            write_in_place(path, action["key"], action["value"])
        elif do == "erase-key":
            write_in_place(path, action["key"], None)
        elif do == "chmod":
            os.chmod(path, int(action["mode"], 8))
        elif do == "xattr":
            subprocess.run(["/usr/bin/xattr", "-w", "user.fault", "1", path], check=True)
        elif do == "acl":
            subprocess.run(["/bin/chmod", "+a", "everyone allow read", path], check=True)
        elif do == "create":
            os.makedirs(path, exist_ok=True)
        elif do == "modify-entry":
            with open(os.path.join(path, "SKILL.md"), "a") as f:
                f.write("changed\n")
        elif do == "retarget":
            os.remove(path)
            os.symlink("/elsewhere", path)
        elif do == "spawn-writer":
            p = subprocess.Popen(["/bin/bash", "-c", f"exec -a {WRITER_MARK} /bin/sleep 120"],
                                 start_new_session=True, stdin=subprocess.DEVNULL,
                                 stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
            record_pid(root, p.pid)
        elif do == "spawn-in-session":
            request = os.path.join(root, "spawn", f"{i}.req")
            os.makedirs(os.path.dirname(request), exist_ok=True)
            save(request + ".tmp", action["argv"])
            os.rename(request + ".tmp", request)
            wait_for(request + ".ack")
        elif do == "open-fd":
            slot = os.path.join(root, f"slot-{action['slot']}")
            p = subprocess.Popen([PY, "-I", SELF, "holder", path, slot], start_new_session=True,
                                 stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL,
                                 stderr=subprocess.DEVNULL)
            record_pid(root, p.pid)
            wait_for(slot + ".ready")
        elif do == "fd-write":
            fd_write(os.path.join(root, f"slot-{action['slot']}"), action["key"], action.get("value"))
        else:
            raise SystemExit(f"unknown action {do}")
    return 99 if crash else 0


def fd_write(slot: str, key: list, value) -> None:
    save(slot + ".go.tmp", {"key": key, "value": value})
    os.rename(slot + ".go.tmp", slot + ".go")
    wait_for(slot + ".done")


def cmd_holder(path: str, slot: str) -> int:
    """Hold a descriptor opened now, and write through it when asked."""
    fd = os.open(path, os.O_RDWR)
    open(slot + ".ready", "w").close()
    wait_for(slot + ".go", 600)
    request = load(slot + ".go", {})
    size = os.fstat(fd).st_size
    obj = json.loads(os.pread(fd, size, 0).decode())
    set_key(obj, request["key"], request["value"])
    data = (json.dumps(obj, indent=2) + "\n").encode()
    os.ftruncate(fd, 0)
    os.pwrite(fd, data, 0)
    os.close(fd)
    open(slot + ".done", "w").close()
    return 0


def cmd_stub_brew(argv: list) -> int:
    prefix = os.path.join(case_root(), "prefix")
    if argv == ["--prefix"]:
        print(prefix)
        return 0
    if argv == ["--cellar"]:
        print(os.path.join(prefix, "Cellar"))
        return 0
    if argv == ["info", "--json=v2", FORMULA_OF_GROVE]:
        # The fields of `brew info --json=v2` that Homebrew 7.0.12 gave on 2026-11-07.
        control = load(os.path.join(case_root(), "control.json"), {})
        linked = os.path.join(prefix, "var/homebrew/linked/grove")
        cellar = os.path.join(prefix, "Cellar/grove")
        print(json.dumps({"formulae": [{
            "name": "grove", "full_name": FORMULA_OF_GROVE, "tap": "linkuistics/taps",
            "versions": {"stable": control.get("tap_grove", "1"), "head": None, "bottle": False},
            "revision": 0,
            "linked_keg": os.path.basename(os.readlink(linked)) if os.path.islink(linked) else None,
            "installed": [{"version": v} for v in sorted(os.listdir(cellar))],
        }], "casks": []}))
        return 0
    print(f"stub brew: unsupported {argv}", file=sys.stderr)
    return 2


def cmd_stub_mise(argv: list) -> int:
    """`mise -C HOME bootstrap packages status --json`, in the shape of mise 2026.11.2."""
    home = os.environ.get("OWNER_SWITCH_HOME", "")
    if argv != ["-C", home, "bootstrap", "packages", "status", "--json"] or os.environ.get("HOME") != home:
        print(f"stub mise: unsupported {argv} with HOME {os.environ.get('HOME')}", file=sys.stderr)
        return 2
    prefix = os.path.join(case_root(), "prefix")
    override = load(os.path.join(case_root(), "control.json"), {}).get("mise", {})
    rows = [{"package": "jq", "requested_version": "latest", "desired_state": "present", "state": "installed",
             "installed_version": "1.8.1"}]
    for name in KEGS:
        formula = f"linkuistics/taps/{name}"
        linked = os.path.join(prefix, "var/homebrew/linked", name)
        row = {"package": formula, "requested_version": "latest", "desired_state": "present",
               "state": "installed", "installed_version": os.path.basename(os.readlink(linked))}
        if formula in override:
            if override[formula] is None:
                continue
            row.update(override[formula])
        rows.append(row)
    print(json.dumps({"brew": {"available": True, "packages": rows},
                      "brew-cask": {"available": True, "packages": []}}, indent=2))
    return 0


def cmd_stub_ravel(argv: list) -> int:
    """The promoted `ravel skills resolve --json`, reading the default skill directory of its prefix."""
    prefix = os.path.dirname(os.path.dirname(os.path.abspath(argv[0])))
    args = argv[1:]
    if args == ["--version"]:
        print("ravel 2.0.0 (fixture)")
        return 0
    if args != ["skills", "resolve", "--json"]:
        print(f"stub ravel: unsupported {args}", file=sys.stderr)
        return 2
    home = os.environ.get("HOME", "")
    leaked = sorted(k for k in os.environ if k.startswith("RAVEL_"))
    if leaked or not home or os.path.realpath(home) in (os.path.realpath(os.path.expanduser("~" + os.environ.get("USER", ""))),
                                                        os.path.realpath(os.environ.get("OWNER_SWITCH_HOME", ""))):
        print(f"stub ravel: refused HOME {home} with {leaked}", file=sys.stderr)
        return 2
    skills = load(os.path.join(home, ".config/ravel/settings.json"), {}).get("skills") or {}
    if "sources" in skills:
        sources = [(s["id"], s["path"]) for s in skills["sources"]]
    else:
        folder = os.path.join(prefix, "share/ravel/skills")
        sources = [(n, os.path.join(folder, n)) for n in sorted(os.listdir(folder)) if not n.startswith(".")]
    if not sources and not skills.get("mandatory"):
        print(json.dumps({"schemaVersion": 1, "status": "disabled"}))
        return 0
    records = []
    for sid, path in sources:
        digest = source_digest(path)
        records.append({"id": sid, "type": "local", "resourceDigest": digest, "originPath": os.path.realpath(path),
                        "snapshotRoot": os.path.join(home, ".cache/ravel/skills/v1", digest)})
    print(json.dumps({"schemaVersion": 1, "status": "resolved", "sources": records}, indent=2))
    return 0


def stub_rewrite(home: str, root: str, label: str, mutate) -> int:
    """Write settings.json as Claude Code does: one read, then a rename onto the path."""
    path = os.path.join(home, ".claude/settings.json")
    with open(path, "rb") as f:
        data = f.read()
    mode = os.stat(path).st_mode & 0o7777
    if cmd_hook(f"stub:{label}:read") == 99:
        return 99
    obj = json.loads(data.decode())
    mutate(obj)
    text = json.dumps(obj, indent=2) + "\n"
    if label in load(os.path.join(root, "control.json"), {}).get("stub_in_place", []):
        with open(path, "r+") as f:
            f.truncate()
            f.write(text)
        return 0
    tmp = f"{path}.tmp.{os.getpid()}.{os.urandom(4).hex()}"
    with open(tmp, "w") as f:
        f.write(text)
    os.chmod(tmp, mode)
    os.rename(tmp, path)
    return 0


def cmd_stub_claude(argv: list) -> int:
    """Stand in for the `claude plugin` commands that the hooks installation runs."""
    home = os.environ["HOME"]
    if os.path.realpath(home) != os.path.realpath(os.environ.get("OWNER_SWITCH_HOME", "")):
        print("stub claude: HOME is not the switch's HOME", file=sys.stderr)
        return 2
    if any(v in os.environ for v in ("CLAUDE_CONFIG_DIR", "CLAUDE_CODE_PLUGIN_DIRS")):
        print("stub claude: a Claude Code location variable is set", file=sys.stderr)
        return 2
    root = case_root()
    with open(os.path.join(root, "claude-calls"), "a") as f:
        f.write(" ".join(argv) + "\n")
    known_path = os.path.join(home, ".claude/plugins/known_marketplaces.json")
    installed_path = os.path.join(home, ".claude/plugins/installed_plugins.json")
    known = load(known_path, {})
    installed = load(installed_path, {"version": 2, "plugins": {}})
    settings = load(os.path.join(home, ".claude/settings.json"), {})
    if argv == ["plugin", "list", "--json"]:
        rows = [{"id": pid, "version": e["version"], "scope": e["scope"],
                 "enabled": settings.get("enabledPlugins", {}).get(pid) is True, "installPath": e["installPath"]}
                for pid, entries in sorted(installed["plugins"].items()) for e in entries]
        print(json.dumps(rows, indent=2))
        return 0
    if argv == ["plugin", "marketplace", "list", "--json"]:
        rows = [{"name": n, "source": m["source"]["source"], "path": m["source"]["path"],
                 "installLocation": m["installLocation"]} for n, m in sorted(known.items())]
        print(json.dumps(rows, indent=2))
        return 0
    if argv[:3] == ["plugin", "marketplace", "add"] and len(argv) == 4:
        if take_failure(root, "claude-marketplace-add"):
            return 1
        path = argv[3]
        name = load(os.path.join(path, ".claude-plugin/marketplace.json"), {})["name"]

        def add(obj):
            obj.setdefault("extraKnownMarketplaces", {})[name] = {"source": {"source": "directory", "path": path}}
        code = stub_rewrite(home, root, "marketplace-add", add)
        if code == 0:
            known[name] = {"source": {"source": "directory", "path": path}, "installLocation": path}
            save_json(known_path, known)
            print(f"Successfully added marketplace: {name}")
        return code
    if argv[:2] == ["plugin", "install"] and len(argv) == 3:
        if take_failure(root, "claude-install"):
            return 1
        pid = argv[2]
        name, market = pid.split("@", 1)
        if market not in known:
            print(f"marketplace {market} is not registered", file=sys.stderr)
            return 1
        mpath = known[market]["source"]["path"]
        entry = next(p for p in load(os.path.join(mpath, ".claude-plugin/marketplace.json"), {})["plugins"]
                     if p["name"] == name)
        cache = os.path.join(home, f".claude/plugins/cache/{market}/{name}/unknown")
        if os.path.exists(cache):
            shutil.rmtree(cache)
        shutil.copytree(os.path.join(mpath, entry["source"]), cache)
        installed["plugins"][pid] = [{"scope": "user", "version": "unknown", "installPath": cache}]
        save_json(installed_path, installed)
        code = stub_rewrite(home, root, "install", lambda obj: obj.setdefault("enabledPlugins", {}).__setitem__(pid, True))
        if code == 0:
            print(f"Successfully installed plugin: {pid}")
        return code
    print(f"stub claude: unsupported {argv}", file=sys.stderr)
    return 2


def cmd_harness(root: str, argv: list) -> int:
    """Stand in for the harness process of the session that runs the switch."""
    children = []
    stop = threading.Event()

    def spawn(args):
        p = subprocess.Popen(args, stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL,
                             stderr=subprocess.DEVNULL)
        children.append(p)
        record_pid(root, p.pid)

    def serve():
        folder = os.path.join(root, "spawn")
        while not stop.is_set():
            if os.path.isdir(folder):
                for name in sorted(os.listdir(folder)):
                    if name.endswith(".req"):
                        req = os.path.join(folder, name)
                        spawn(load(req, []))
                        os.rename(req, req + ".done")
                        open(req + ".ack", "w").close()
            time.sleep(0.02)

    for args in load(os.path.join(root, "control.json"), {}).get("session", []):
        spawn(args)
    thread = threading.Thread(target=serve, daemon=True)
    thread.start()
    code = subprocess.run(argv).returncode
    stop.set()
    thread.join()
    return code


# --- the rehearsal -------------------------------------------------------------

def clean_env() -> dict:
    return {k: v for k, v in os.environ.items() if k not in LOOP_CONTROL}


class Case:
    def __init__(self, rehearsal, name: str, control=None, rehearsed=True, ravel="off", hooks="disabled"):
        self.r = rehearsal
        self.name = name
        self.root = os.path.join(rehearsal.out, "cases", name)
        if os.path.exists(self.root):
            shutil.rmtree(self.root)
        self.home = os.path.join(self.root, "home")
        self.marketplace = build_prefix(os.path.join(self.root, "prefix"))
        build_home(self.home, self.marketplace, ravel, hooks)
        self.set_control(control or {})
        if rehearsed:
            self.rehearse_record(rehearsal.plan_digest, rehearsal.program_digest)
        self.outputs = []
        self.runs = []

    def claude_calls(self) -> list:
        try:
            with open(os.path.join(self.root, "claude-calls")) as f:
                return f.read().splitlines()
        except FileNotFoundError:
            return []

    def commands(self, run: int = -1) -> dict:
        """The Claude Code commands of one run, counted by kind."""
        return self.runs[run]["claude"]

    def rehearse_record(self, plan_digest: str, program_digest: str) -> None:
        folder = os.path.join(self.home, STATE, "rehearsals")
        os.makedirs(folder, exist_ok=True)
        save(os.path.join(folder, f"fixture-{plan_digest[:8]}-{program_digest[:8]}.json"), {
            "schema": "owner-switch-rehearsal/1", "passed": True,
            "plan_digest": plan_digest, "program_digest": program_digest,
        })

    def set_control(self, control: dict) -> None:
        save(os.path.join(self.root, "control.json"), control)
        for name in ("fired.json", "failed.json"):
            p = os.path.join(self.root, name)
            if os.path.exists(p):
                os.remove(p)

    def run(self, program=None, plan=None, dry=False, env=None) -> tuple:
        args = [PY, "-I", program or self.r.program, "run", "--plan", plan or self.r.plan, "--home", self.home]
        if dry:
            args.append("--dry-run")
        # A Claude Code location variable in the environment must not reach a
        # command: the stub refuses it, and so does a staging launch.
        env = dict(clean_env(), OWNER_SWITCH_REHEARSAL_HOOK=self.r.hook,
                   PATH=self.r.stubs + os.pathsep + os.environ.get("PATH", ""),
                   CLAUDE_CONFIG_DIR=os.path.join(self.root, "elsewhere"),
                   # The stub ravel refuses every RAVEL_* name, so no resolution may inherit it.
                   RAVEL_SKILL_DIRECTORIES=os.path.join(self.root, "elsewhere"), **(env or {}))
        before = len(self.claude_calls())
        result = subprocess.run([PY, "-I", SELF, "harness", HARNESS_MARK, self.root, "--"] + args,
                                env=env, capture_output=True, text=True, timeout=300)
        out = result.stdout + result.stderr
        self.outputs.append(out)
        counts = {}
        for call in self.claude_calls()[before:]:
            words = call.split()
            kind = " ".join(words[:3] if words[1:2] == ["marketplace"] else words[:2])
            counts[kind] = counts.get(kind, 0) + 1
        self.runs.append({"exit": result.returncode, "claude": counts})
        with open(os.path.join(self.root, "runs.log"), "a") as f:
            f.write(f"$ run (exit {result.returncode})\n{out}\n")
        return result.returncode, out

    def state(self) -> dict:
        result = subprocess.run([PY, "-I", self.r.program, "state", "--json", "--plan", self.r.plan,
                                 "--home", self.home], capture_output=True, text=True, env=clean_env())
        if result.returncode != 0:
            raise AssertionError(f"state failed: {result.stdout}{result.stderr}")
        return json.loads(result.stdout)

    def json(self, rel: str) -> dict:
        return load(os.path.join(self.home, rel), None)

    def receipts(self) -> int:
        with open(os.path.join(self.home, MANUAL)) as f:
            return f.read().count("<!-- owner-switch-receipt ")

    def snapshot(self) -> dict:
        out = {}
        for folder, dirs, files in os.walk(self.home):
            for name in sorted(dirs + files):
                p = os.path.join(folder, name)
                rel = os.path.relpath(p, self.home)
                if os.path.islink(p):
                    out[rel] = "link:" + os.readlink(p)
                elif os.path.isfile(p):
                    with open(p, "rb") as f:
                        out[rel] = f.read().hex() + oct(os.lstat(p).st_mode)
                else:
                    out[rel] = "dir"
        return out

    def points(self) -> list:
        try:
            with open(os.path.join(self.root, "points")) as f:
                return f.read().split()
        except FileNotFoundError:
            return []

    def kill(self) -> None:
        try:
            with open(os.path.join(self.root, "pids")) as f:
                pids = [int(x) for x in f.read().split()]
        except FileNotFoundError:
            return
        for pid in pids:
            try:
                os.kill(pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
        os.remove(os.path.join(self.root, "pids"))


def expect(condition: bool, message: str) -> None:
    if not condition:
        raise AssertionError(message)


def expect_exit(got: tuple, code: int, text: str = "") -> str:
    out = got[1]
    expect(got[0] == code, f"expected exit {code}, got {got[0]}: {out[-1500:]}")
    if text:
        expect(re.search(text, out) is not None, f"expected {text!r} in output: {out[-1500:]}")
    return out


def run_to_complete(case: Case) -> None:
    expect_exit(case.run(), 0, "result: complete")
    st = case.state()
    expect(st["state"] == "complete", f"expected complete, got {st['state']}")
    expect(case.receipts() == 1, f"expected one receipt in the manual, got {case.receipts()}")
    expect(not st["flagged"], f"unexpected flagged copies {st['flagged']}")


class Rehearsal:
    def __init__(self, out: str, program: str):
        self.out = os.path.realpath(out)
        os.makedirs(self.out, exist_ok=True)
        self.program = os.path.realpath(program)
        self.plan = build_plan(self.program, self.out)
        self.plan_digest = digest_file(self.plan)
        self.program_digest = digest_file(self.program)
        self.hook = os.path.join(self.out, "hook.sh")
        with open(self.hook, "w") as f:
            f.write(f"#!/bin/sh\nexec '{PY}' -I '{SELF}' hook \"$@\"\n")
        os.chmod(self.hook, 0o755)
        self.stubs = os.path.join(self.out, "stubs")
        os.makedirs(self.stubs, exist_ok=True)
        for name in ("claude", "brew", "mise"):
            stub = os.path.join(self.stubs, name)
            with open(stub, "w") as f:
                f.write(f"#!/bin/sh\nexec '{PY}' -I '{SELF}' stub-{name} \"$@\"\n")
            os.chmod(stub, 0o755)
        self.results = []

    def control(self, name: str, fn, **case_args) -> None:
        case = Case(self, name, **case_args)
        started = time.time()
        try:
            detail = fn(case) or "passed"
            ok = True
        except AssertionError as e:
            detail, ok = str(e), False
        except subprocess.TimeoutExpired as e:
            detail, ok = f"timeout: {e}", False
        finally:
            case.kill()
        self.results.append({"control": name, "passed": ok, "detail": detail,
                             "seconds": round(time.time() - started, 2), "runs": case.runs})
        print(f"{'PASS' if ok else 'FAIL'} {name}" + ("" if ok else f": {detail}"), flush=True)


def digest_file(path: str) -> str:
    import hashlib
    with open(path, "rb") as f:
        return hashlib.sha256(f.read()).hexdigest()


# --- the controls ---------------------------------------------------------------

SETTINGS = ".claude/settings.json"
RAVEL = ".config/ravel/settings.json"


def c_dry_run(case):
    before = case.snapshot()
    out = expect_exit(case.run(dry=True), 0, "switch, in order")
    expect("inverse, in order" in out and "rehearsed: yes" in out, "the dry run omits the inverse")
    expect(case.snapshot() == before, "the dry run changed the layout")
    expect(case.points() == [], "the dry run reached a hook point")


def c_switch(case):
    path = os.path.join(case.home, SETTINGS)
    os.chmod(path, 0o640)
    subprocess.run(["/usr/bin/xattr", "-w", "user.owner", "kept", path], check=True)
    subprocess.run(["/bin/chmod", "+a", "everyone allow read", path], check=True)
    run_to_complete(case)
    acl = subprocess.run(["/bin/ls", "-le", path], capture_output=True, text=True).stdout
    expect("everyone allow read" in acl, "the published file lost its ACL")
    expect(oct(os.stat(path).st_mode & 0o777) == "0o640", "the published file lost its mode")
    xattr = subprocess.run(["/usr/bin/xattr", "-p", "user.owner", path], capture_output=True, text=True)
    expect(xattr.stdout.strip() == "kept", "the published file lost its extended attribute")
    expect(case.json(SETTINGS)["theme"] == "dark", "the switch lost another key")
    before = case.snapshot()
    expect_exit(case.run(), 0, "state: complete")
    expect(case.snapshot() == before and case.receipts() == 1, "a complete run changed the layout")


def c_no_rehearsal(case):
    expect_exit(case.run(), 3, "no passing rehearsal record")
    st = case.state()
    expect(st["state"] == "native" and st["backup"] is None, "step 8 wrote a backup without a rehearsal")


def c_changed_program(case):
    other = os.path.join(case.root, "program-changed.py")
    with open(case.r.program) as f, open(other, "w") as g:
        g.write(f.read() + "\n# changed\n")
    expect_exit(case.run(program=other), 3, "no passing rehearsal record")
    expect(case.state()["backup"] is None, "a changed program wrote a backup")


def c_changed_plan(case):
    plan = load(case.r.plan, {})
    plan["manual"] = "notes/other.md"
    other = os.path.join(case.root, "plan-changed.json")
    save(other, plan)
    expect_exit(case.run(plan=other), 3, "no passing rehearsal record")


def c_continuation_copy(case):
    case.set_control({"actions": [{"point": "switch-after-part:1", "do": "crash"}]})
    expect_exit(case.run(), 99)
    other = os.path.join(case.root, "program-changed.py")
    with open(case.r.program) as f, open(other, "w") as g:
        g.write(f.read() + "\n# changed\n")
    out = expect_exit(case.run(program=other), 0, "continuation: running the program copy")
    expect(f"program: {case.r.program_digest}" in out, "the continuation did not run the backup's copy")


def crash_then_complete(point: str):
    def fn(case):
        case.set_control({"actions": [{"point": point, "do": "crash"}]})
        expect_exit(case.run(), 99)
        middle = case.state()
        if point == "before-backup-rename":
            expect(middle["incomplete_backups"], "no incomplete backup after a crash before its rename")
            expect(middle["backup"] is None, "an incomplete backup counted as complete")
        run_to_complete(case)
        return f"after the crash: {middle['state']}"
    return fn


def inverse_crash_then_complete(point: str):
    def fn(case):
        case.set_control({"fail": {"probe-switched": 1}, "actions": [{"point": point, "do": "crash"}]})
        expect_exit(case.run(), 99)
        middle = case.state()
        run_to_complete(case)
        return f"after the crash: {middle['state']}"
    return fn


def c_probe_fails(case):
    case.set_control({"fail": {"probe-switched": 1}})
    expect_exit(case.run(), 3, "the native probe passed")
    st = case.state()
    expect(st["state"] == "native" and st["records"][-1] == "inverse-ended", f"unexpected {st['state']}")
    expect(case.receipts() == 0, "a failed probe copied a receipt")
    run_to_complete(case)


def erasable(rel: str) -> str:
    """A key outside the plan's cells that the fixture file holds."""
    return "theme" if rel == SETTINGS else "policy"


def write_before_exchange(rel: str, index: int, do: str, phase: str = "switch"):
    def fn(case):
        action = {"point": f"{phase}-before-exchange:{index}", "do": do, "path": rel,
                  "key": [erasable(rel)] if do == "erase-key" else ["fault"], "value": 1}
        control = {"actions": [action]}
        if phase == "inverse":
            control["fail"] = {"probe-switched": 1}
        case.set_control(control)
        expect_exit(case.run(), 3)
        st = case.state()
        expect(st["flagged"], "the outside write left no flagged kept copy")
        live = case.json(rel)
        if do == "erase-key":
            expect(erasable(rel) in live, "the outside erasure before the exchange reached the live file")
        else:
            expect("fault" not in live, "the outside write before the exchange is in the live file")
        out = expect_exit(case.run(), 3, "flagged")
        expect("a kept copy is flagged" in out, "step 8 did not stop while a copy is flagged")
        return f"state {case.state()['state']}"
    return fn


def write_after_exchange(rel: str, index: int, phase: str = "switch"):
    def fn(case):
        control = {"actions": [{"point": f"{phase}-after-part:{index}", "do": "write-key", "path": rel,
                                "key": ["fault"], "value": 1}]}
        if phase == "inverse":
            control["fail"] = {"probe-switched": 1}
        case.set_control(control)
        got = case.run()
        expect(case.json(rel).get("fault") == 1, "the write after the exchange is not in the live file")
        expect(not case.state()["flagged"], "a write after the exchange flagged a copy")
        if phase == "switch":
            expect(got[0] == 0, f"the switch did not complete: {got[1][-800:]}")
    return fn


def descriptor_write(rel: str, index: int, when: str):
    def fn(case):
        actions = [{"point": f"switch-read:{index}", "do": "open-fd", "path": rel, "slot": "a"}]
        if when == "inside":
            actions.append({"point": f"switch-after-part:{index}", "do": "fd-write", "slot": "a",
                            "key": ["fault"], "value": 1})
        case.set_control({"actions": actions})
        got = case.run()
        if when == "inside":
            expect(got[0] == 3 and "conflict" in got[1], "a descriptor write did not stop the switch")
        else:
            expect(got[0] == 0, f"the switch did not complete: {got[1][-800:]}")
            fd_write(os.path.join(case.root, "slot-a"), ["fault"], 1)
        st = case.state()
        expect(st["flagged"], "the descriptor write left no flagged copy")
        expect("fault" not in case.json(rel), "the descriptor write reached the live file")
        out = case.run()[1]
        expect("flagged: kept copy" in out, "a later run did not report the flagged copy")
    return fn


def metadata_change(do: str):
    def fn(case):
        case.set_control({"actions": [{"point": "switch-read:2", "do": do, "path": SETTINGS, "mode": "600"}]})
        expect_exit(case.run(), 3, "conflict")
        expect(case.state()["flagged"], "a metadata change left no flagged copy")
    return fn


def c_acl_denies_exchange(case):
    path = os.path.join(case.home, SETTINGS)
    subprocess.run(["/bin/chmod", "+a", "everyone deny delete", path], check=True)
    with open(path, "rb") as f:
        before = f.read()
    expect_exit(case.run(), 3, "exchange failed")
    st = case.state()
    expect(st["state"] == "native" and st["records"][-1] == "inverse-ended", f"unexpected {st['state']}")
    with open(path, "rb") as f:
        expect(f.read() == before, "a refused exchange changed the file")


def c_third_before(case):
    write_in_place(os.path.join(case.home, SETTINGS), SUPERSEDED_PLUGIN, "other")
    expect_exit(case.run(), 3, "classification")
    expect(case.state()["state"] == "changed", "a third value did not read as changed")


def c_third_after_receipt(case):
    run_to_complete(case)
    write_in_place(os.path.join(case.home, RAVEL), ["skills"], {"other": True})
    before = case.snapshot()
    expect_exit(case.run(), 3, "drifted")
    expect(case.state()["state"] == "drifted" and case.snapshot() == before, "a drifted run changed the layout")


def c_third_before_receipt(case):
    case.set_control({"actions": [{"point": "before-record:receipt", "do": "crash"}]})
    expect_exit(case.run(), 99)
    write_in_place(os.path.join(case.home, RAVEL), ["skills"], {"other": True})
    expect_exit(case.run(), 3, "drifted")


def c_switched_while_open(case):
    case.set_control({"actions": [{"point": "switch-after-part:1", "do": "crash"}]})
    expect_exit(case.run(), 99)
    path = os.path.join(case.home, SETTINGS)
    write_in_place(path, SUPERSEDED_PLUGIN, False)
    write_in_place(path, HOOKS_PLUGIN, True)
    out = expect_exit(case.run(), 0, "result: complete")
    first = out.split("record: inverse-ended")[0]
    expect("replaced: part 2 .claude/settings.json enabledPlugins.grove@linkuistics: false -> true" in first,
           "the resumed inverse did not list the switched value that it reset")


def c_switched_after_inverse(case):
    case.set_control({"fail": {"probe-switched": 1}})
    expect_exit(case.run(), 3)
    write_in_place(os.path.join(case.home, RAVEL), ["skills"], SWITCHED)
    expect_exit(case.run(), 3, "classification")


def entry_change(do: str, rel: str, when: str):
    def fn(case):
        if when == "before":
            case.set_control({"actions": [{"point": "start", "do": do, "path": rel}]})
            expect_exit(case.run(), 3, "classification")
            expect(case.state()["state"] == "changed", "a changed entry did not read as changed")
            return
        run_to_complete(case)
        backup = case.state()["backup"]
        target = os.path.join(backup, "retained", rel) if do != "create" else os.path.join(case.home, rel)
        if do == "modify-entry":
            with open(os.path.join(target, "SKILL.md"), "a") as f:
                f.write("changed\n")
        elif do == "retarget":
            os.remove(target)
            os.symlink("/elsewhere", target)
        else:
            os.makedirs(target)
        expect_exit(case.run(), 3, "drifted")
    return fn


def c_occupied_after_read(case):
    case.set_control({"fail": {"probe-switched": 1},
                      "actions": [{"point": "inverse-read:1", "do": "create", "path": ".codex/skills/grove"}]})
    expect_exit(case.run(), 3, "move back failed")
    st = case.state()
    expect(os.path.isdir(os.path.join(case.home, ".codex/skills/grove")), "the occupying entry moved")
    expect(os.path.lexists(os.path.join(st["backup"], "retained/.codex/skills/grove")),
           "the retained entry moved")
    expect(st["state"] == "drifted", f"expected drifted, got {st['state']}")


def c_step14_resume(case):
    case.set_control({"actions": [{"point": "step-14", "do": "crash"}]})
    expect_exit(case.run(), 99)
    expect(case.state()["state"] == "complete" and case.receipts() == 0, "the crash point is not before step 14")
    run_to_complete(case)


def c_outside_inside_run(case):
    case.set_control({"actions": [{"point": "after-record:receipt", "do": "create",
                                   "path": ".codex/skills/new-entry"}]})
    expect_exit(case.run(), 3, "changed outside the switch set")
    st = case.state()
    expect(case.receipts() == 0 and st["records"][-1] == "inverse-ended", "a changed outside entry was accepted")


def c_outside_after_stop(case):
    case.set_control({"actions": [{"point": "step-14", "do": "crash"}]})
    expect_exit(case.run(), 99)
    os.makedirs(os.path.join(case.home, ".claude/skills/new-entry"))
    expect_exit(case.run(), 3, "changed outside the switch set")
    expect(case.receipts() == 0, "the complete continuation copied a receipt with a changed outside entry")


def c_cell_after_receipt_no_copy(case):
    case.set_control({"actions": [{"point": "step-14", "do": "crash"}]})
    expect_exit(case.run(), 99)
    write_in_place(os.path.join(case.home, SETTINGS), SUPERSEDED_PLUGIN, True)
    expect_exit(case.run(), 3, "drifted")
    expect(case.receipts() == 0, "a run copied a receipt for a layout that is not switched")


def c_writer_before(case):
    p = subprocess.Popen(["/bin/bash", "-c", f"exec -a {WRITER_MARK} /bin/sleep 120"], start_new_session=True)
    record_pid(case.root, p.pid)
    time.sleep(0.2)
    before = case.snapshot()
    expect_exit(case.run(), 3, "other processes or jobs remain before step 1")
    expect(case.snapshot() == before, "a stopped step 2 changed the layout")


def interval_closed(do: str, argv=None):
    def fn(case):
        action = {"point": "gate:step 9", "do": do}
        if argv:
            action["argv"] = argv
        case.set_control({"actions": [action]})
        expect_exit(case.run(), 3, "before step 9")
        st = case.state()
        expect(all(p["status"] == "earlier" for p in st["parts"]), "a part changed inside a closed interval")
        case.kill()
        run_to_complete(case)
        return f"after the stop: {st['state']}"
    return fn


def session_process(argv):
    def fn(case):
        case.set_control({"session": [argv]})
        time.sleep(0.1)
        expect_exit(case.run(), 3, "of the session")
        expect(case.state()["backup"] is None, "a session process did not stop step 2")
    return fn


def c_job(case):
    case.set_control({"jobs": [f"com.example.{JOB_MARK}"]})
    expect_exit(case.run(), 3, "job: ")


def c_prepared_differs(case):
    case.set_control({"actions": [{"point": "switch-after-part:1", "do": "crash"}]})
    expect_exit(case.run(), 99)
    plan = load(case.r.plan, {})
    # The prepared plan lacks the Claude Code settings part. It keeps the Ravel
    # settings part, because steps 4 and 6 draft the real settings from it.
    plan["parts"] = [plan["parts"][0], plan["parts"][1], plan["parts"][3]]
    other = os.path.join(case.root, "plan-changed.json")
    save(other, plan)
    out = expect_exit(case.run(plan=other), 3, "the prepared plan differs from the open run's plan")
    expect("record: inverse-ended" in out, "the open run's plan did not apply to its inverse")
    expect("no passing rehearsal record" in out, "the unrehearsed prepared plan passed step 8")


# --- the preludes of step 3 -------------------------------------------------------

def settings_folder(case) -> str:
    return os.path.join(case.home, STATE, "settings")


def settings_records(case) -> list:
    folder = settings_folder(case)
    if not os.path.isdir(folder):
        return []
    out = []
    for name in sorted(os.listdir(folder)):
        body = load(os.path.join(folder, name, "record.json"), None) if not name.startswith(".") else None
        if body is not None:
            body["_copy"] = os.path.join(folder, name, "copy")
            out.append(body)
    return out


def skills_cell(case):
    body = case.json(RAVEL)
    if body is None:
        return "absent-file"
    return body.get("skills", "absent-key")


def expect_installs(case, run: int, add: int, install: int) -> None:
    got = case.commands(run)
    expect(got.get("plugin marketplace add", 0) == add and got.get("plugin install", 0) == install,
           f"run {run} ran {got}, not {add} registration and {install} installation")


def expect_flag_holds(case, text: str = "flagged: ") -> None:
    """The copy stays flagged: the state reports it, and the next run reports it and stops at step 8."""
    st = case.state()
    expect(st["flagged"], "no copy is flagged")
    expect(st["backup"] is None, "a backup exists before the flagged copy was checked")
    out = expect_exit(case.run(), 3, "a kept copy is flagged")
    expect(text in out, f"the next run did not report the flagged copy: {out[-800:]}")
    expect(case.state()["backup"] is None, "step 8 wrote a backup while a copy is flagged")


def c_native_full(case):
    before = skills_cell(case)
    out = expect_exit(case.run(), 0, "result: complete")
    expect("replaced: .config/ravel/settings.json skills: (absent" in out, "the native settings listed no replaced value")
    records = [r for r in settings_records(case) if r["label"] == "native"]
    if before == "absent-file":
        expect(not records, "the creation of an absent file wrote a settings record")
    else:
        expect(len(records) == 1, f"expected one settings record, got {len(records)}")
        expect(case.json(RAVEL)["policy"] == "config.rhai", "the native settings lost another key")
    st = case.state()
    expect(st["state"] == "complete" and not st["flagged"], f"unexpected {st['state']} {st['flagged']}")
    return f"from {before}"


def prelude_crash(point: str):
    def fn(case):
        case.set_control({"actions": [{"point": point, "do": "crash"}]})
        expect_exit(case.run(), 99)
        middle = skills_cell(case)
        expect(middle in ("absent-file", "absent-key", OFF), f"the crash left skills {middle}")
        out = expect_exit(case.run(), 0, "result: complete")
        expect("state: native" in out, "the next run did not reach the native state")
        expect(not case.state()["flagged"], "a crash flagged a copy")
        return f"after the crash: skills {middle}, hooks {case.json(SETTINGS)['enabledPlugins'].get(HOOKS_ID, 'absent')}"
    return fn


def c_native_write_before_exchange(case):
    case.set_control({"actions": [{"point": "native-read", "do": "write-key", "path": RAVEL,
                                   "key": ["fault"], "value": 1}]})
    expect_exit(case.run(), 3, "it is flagged")
    expect("fault" not in case.json(RAVEL), "the outside write before the exchange is in the live file")
    expect_flag_holds(case, "flagged: kept copy")


def c_native_write_then_crash(case):
    case.set_control({"actions": [{"point": "native-read", "do": "write-key", "path": RAVEL,
                                   "key": ["fault"], "value": 1},
                                  {"point": "native-exchanged", "do": "crash"}]})
    expect_exit(case.run(), 99)
    expect_flag_holds(case, "flagged: kept copy")


def c_native_descriptor(case):
    case.set_control({"actions": [{"point": "native-read", "do": "open-fd", "path": RAVEL, "slot": "a"},
                                  {"point": "native-compared", "do": "fd-write", "slot": "a",
                                   "key": ["fault"], "value": 1}]})
    expect_exit(case.run(), 3, "a kept copy is flagged")
    expect("fault" not in case.json(RAVEL), "the descriptor write reached the live file")
    expect_flag_holds(case, "flagged: kept copy")


def c_native_third(case):
    write_in_place(os.path.join(case.home, RAVEL), ["skills"], {"other": True})
    before = case.snapshot()
    out = expect_exit(case.run(), 3, "classification")
    expect("state: changed" in out and case.state()["state"] == "changed", "a third skills value did not read as changed")
    expect(case.snapshot() == before, "the native settings changed a third value")
    write_in_place(os.path.join(case.home, RAVEL), ["skills"], OFF)
    out = expect_exit(case.run(), 0, "result: complete")
    expect("state: native" in out, "the restored off value did not read as native")


def c_native_erased_after_inverse(case):
    case.set_control({"fail": {"probe-switched": 1}})
    expect_exit(case.run(), 3, "the native probe passed")
    write_in_place(os.path.join(case.home, RAVEL), ["skills"], None)
    out = expect_exit(case.run(), 0, "result: complete")
    expect("replaced: .config/ravel/settings.json skills: (absent) -> " in out, "the replaced value was not listed")
    expect("state: native" in out, "the next run did not find the native state")


def prelude_writer(point: str, interval: str):
    def fn(case):
        case.set_control({"actions": [{"point": point, "do": "spawn-writer"}]})
        before = case.snapshot()
        expect_exit(case.run(), 3, f"before {interval}")
        expect(case.snapshot() == before, f"a write ran inside the closed interval of {interval}")
        case.kill()
        run_to_complete(case)
    return fn


def c_hooks_full(case):
    run_to_complete(case)
    expect_installs(case, 0, 1, 1)
    held = [r for r in settings_records(case) if r["kind"] == "held"]
    expect(sorted(r["label"] for r in held) == ["install", "marketplace-add"], f"held records {held}")
    expect(all(os.path.exists(r["_copy"]) for r in held), "a held copy of a command is gone")
    expect(len([r for r in settings_records(case) if r["label"] == "hooks-disable"]) == 1,
           "the disabling wrote no settings record")
    return f"commands {case.commands(0)}"


def c_hooks_after_registration(case):
    case.set_control({"actions": [{"point": "hooks-before-held:install", "do": "crash"}]})
    expect_exit(case.run(), 99)
    expect_installs(case, 0, 1, 0)
    out = expect_exit(case.run(), 0, "result: complete")
    expect("state: native" in out, "the next run did not reach the native state")
    expect_installs(case, 1, 0, 1)


def c_hooks_held_crash(label: str):
    def fn(case):
        case.set_control({"actions": [{"point": f"hooks-held:{label}", "do": "crash"}]})
        expect_exit(case.run(), 99)
        expect_installs(case, 0, 1 if label == "install" else 0, 0)
        out = expect_exit(case.run(), 0, "result: complete")
        expect("erased: held link" in out, "the next run did not erase the held link")
        expect("state: native" in out, "the next run did not reach the native state")
        expect_installs(case, 1, 0 if label == "install" else 1, 1)
        expect(not case.state()["flagged"], "an erased held link left a flagged copy")
    return fn


def c_hooks_write_in_interval(case):
    case.set_control({"actions": [{"point": "stub:install:read", "do": "write-key", "path": SETTINGS,
                                   "key": ["fault"], "value": 1}]})
    expect_exit(case.run(), 3, "differs from its record; it is flagged")
    expect("fault" not in case.json(SETTINGS), "the write inside the interval is in the live file")
    expect_flag_holds(case, "flagged: held copy")


def c_hooks_descriptor(case):
    case.set_control({"actions": [{"point": "hooks-held:install", "do": "open-fd", "path": SETTINGS, "slot": "a"},
                                  {"point": "hooks-compared:install", "do": "fd-write", "slot": "a",
                                   "key": ["fault"], "value": 1}]})
    expect_exit(case.run(), 3, "a kept copy is flagged")
    expect("fault" not in case.json(SETTINGS), "the descriptor write reached the live file")
    expect_flag_holds(case, "flagged: held copy")


def c_hooks_third_before_held(case):
    case.set_control({"actions": [{"point": "hooks-before-held:marketplace-add", "do": "write-key",
                                   "path": SETTINGS, "key": HOOKS_PLUGIN, "value": "other"}]})
    expect_exit(case.run(), 3, "classification")
    expect_installs(case, 0, 0, 0)


def hooks_metadata(do: str, when: str):
    def fn(case):
        path = os.path.join(case.home, SETTINGS)
        if when == "initial":
            if do == "xattr":
                subprocess.run(["/usr/bin/xattr", "-w", "user.fault", "1", path], check=True)
            else:
                subprocess.run(["/bin/chmod", "+a", "everyone allow read", path], check=True)
        else:
            point = "hooks-before-held:marketplace-add" if when == "before-held" else "hooks-held:install"
            case.set_control({"actions": [{"point": point, "do": do, "path": SETTINGS}]})
        if when == "after-reading":
            expect_exit(case.run(), 3, "differs from its record; it is flagged")
            expect_flag_holds(case, "flagged: held copy")
            return
        expect_exit(case.run(), 3, "would drop")
        expect_installs(case, 0, 0, 0)
    return fn


def c_hooks_in_place(case):
    case.set_control({"stub_in_place": ["install"]})
    expect_exit(case.run(), 3, "replaced no inode")
    case.set_control({})
    out = expect_exit(case.run(), 0, "result: complete")
    expect("erased: held link" in out, "the next run did not erase the held link")
    expect_installs(case, 1, 0, 0)


def c_hooks_command_fails(case):
    case.set_control({"fail": {"claude-install": 1}})
    expect_exit(case.run(), 3, "failed")
    expect(case.json(SETTINGS)["enabledPlugins"].get(HOOKS_ID) is None, "a failed command changed the cell")
    run_to_complete(case)
    expect_installs(case, 1, 0, 1)


def hooks_disable_fault(do: str, when: str):
    def fn(case):
        if do == "descriptor":
            actions = [{"point": "hooks-disable-read", "do": "open-fd", "path": SETTINGS, "slot": "a"},
                       {"point": "hooks-disable-compared", "do": "fd-write", "slot": "a",
                        "key": ["fault"], "value": 1}]
        else:
            point = "hooks-disable-read" if when == "before" else "hooks-disable-compared"
            actions = [{"point": point, "do": do, "path": SETTINGS,
                        "key": ["theme"] if do == "erase-key" else ["fault"], "value": 1}]
        case.set_control({"actions": actions})
        got = case.run()
        if when == "after":
            expect(got[0] == 0, f"the switch did not complete: {got[1][-800:]}")
            expect(case.json(SETTINGS).get("fault") == 1, "the write after the exchange is not in the live file")
            expect(not case.state()["flagged"], "a write after the exchange flagged a copy")
            return
        expect(got[0] == 3, f"expected a stop: {got[1][-800:]}")
        live = case.json(SETTINGS)
        expect("fault" not in live and "theme" in live, "the outside write reached the live file")
        expect_flag_holds(case, "flagged: kept copy")
    return fn


def open_run_no_prelude(point: str, state: str):
    def fn(case):
        case.set_control({"actions": [{"point": point, "do": "crash"}]})
        expect_exit(case.run(), 99)
        expect(case.state()["state"] == state, f"the crash did not leave the {state} state")
        mark = len(case.points())
        out = expect_exit(case.run(), 0, "result: complete")
        expect("no prelude runs" in out, "the next run did not say that no prelude runs")
        expect(not [p for p in case.points()[mark:] if p.startswith("prelude:")], "a prelude ran while a run was open")
        expect(case.commands(1) == {}, f"the next run ran Claude Code commands {case.commands(1)}")
    return fn


def c_prelude_unrehearsed(case):
    before = case.snapshot()
    expect_exit(case.run(), 3, "so no prelude runs")
    expect(case.snapshot() == before, "an unrehearsed program changed the layout")
    expect(case.commands(0) == {}, f"an unrehearsed program ran {case.commands(0)}")


def c_prelude_other_plan(case):
    case.rehearse_record("0" * 64, case.r.program_digest)
    out = expect_exit(case.run(), 3, "no passing rehearsal record names the prepared plan")
    expect(skills_cell(case) == OFF, "a record that names the program did not admit the preludes")
    expect("replaced: .config/ravel/settings.json" in out, "the native settings did not run")


# --- the gates of steps 4 to 7, and the baseline of step 3 ----------------------------

def keg_source(case, sid: str) -> str:
    return os.path.realpath(os.path.join(case.root, "prefix/share/ravel/skills", sid))


def one_byte(case, sid: str) -> None:
    """Change one byte of a keg source, so that its digest differs from the accepted one."""
    path = os.path.join(keg_source(case, sid), f"skills/{sid}-skill/SKILL.md")
    with open(path, "rb") as f:
        data = bytearray(f.read())
    data[-2] ^= 0x01
    with open(path, "wb") as f:
        f.write(bytes(data))


def eleventh_source(case) -> None:
    prefix = os.path.join(case.root, "prefix")
    rel = "Cellar/extra/9/share/ravel/skills/eleventh"
    write(os.path.join(prefix, rel, "skills/eleventh/SKILL.md"), "---\nname: eleventh\n---\n")
    os.symlink(f"../../../{rel}", os.path.join(prefix, "share/ravel/skills/eleventh"))


def trials(case) -> list:
    try:
        with open(os.path.join(case.root, "trials.log")) as f:
            return f.read().split()[::2]
    except FileNotFoundError:
        return []


def backup_evidence(case, step: str) -> list:
    backup = case.state()["backup"]
    return load(os.path.join(backup, "evidence", f"step-{step}.json"), [])


def expect_gate_stop(case, text: str, **run) -> str:
    out = expect_exit(case.run(**run), 3, text)
    st = case.state()
    expect(st["state"] == "native" and st["backup"] is None, f"a stopped gate left {st['state']}, {st['backup']}")
    expect("before-backup-rename" not in case.points(), "a stopped gate reached step 8")
    return out


def c_gates_evidence(case):
    out = expect_exit(case.run(), 0, "result: complete")
    expect("compare with finish's own skill listing: claude | built-in | debug" in out,
           "step 3 did not print the built-in row for finish's listing")
    four = backup_evidence(case, "4")[0]["result"]
    expect(four["admitted"] == {"grove_version": "1", "digests": {}} and four["reran"] == [],
           f"step 4 admitted {four.get('admitted')}")
    six = backup_evidence(case, "6")[0]["result"]
    expect(sorted(six["sources"]) == sorted(load(case.r.plan, {})["accepted"]["sources"]), "step 6 kept other sources")
    readers = {r["plugin"]: r for r in backup_evidence(case, "7")[0]["result"]["readers"]}
    expect(sorted(readers) == sorted(READERS) and not any(r["orphaned"] for r in readers.values()),
           f"step 7 recorded {sorted(readers)}")
    expect(readers["grove@linkuistics"]["path"] == os.path.join(case.home, READERS["grove@linkuistics"]),
           "step 7 did not name the registered version directory")
    receipt = [n for n in os.listdir(os.path.join(case.state()["backup"], "run")) if n.endswith("-receipt.json")]
    body = load(os.path.join(case.state()["backup"], "run", receipt[0]), {})
    expect(sorted(body.get("gates", {})) == ["4", "6", "7"], f"the receipt lacks the gates: {sorted(body.get('gates', {}))}")
    expect(trials(case) == [], f"step 4 ran trials {trials(case)} with each digest accepted")


def c_eleventh_source(case):
    eleventh_source(case)
    expect_gate_stop(case, "step 6: the resolved sources are not the accepted ten: extra eleventh")


def c_one_byte_other(case):
    one_byte(case, "superpowers")
    expect_gate_stop(case, "step 6: superpowers gives [0-9a-f]{64}, not its accepted")
    expect(trials(case) == [], "a source outside Grove's keg made step 4 rerun trials")


def c_one_byte_grove(case):
    one_byte(case, "design")
    digest = source_digest(keg_source(case, "design"))
    out = expect_exit(case.run(), 0, "the staging trials of design must run again")
    expect(trials(case) == ["design"], f"step 4 ran the trials {trials(case)}, not those of design")
    four = backup_evidence(case, "4")[0]["result"]
    expect(four["reran"] == ["design"] and four["admitted"]["digests"] == {"design": digest},
           f"step 4 admitted {four['admitted']}")
    expect("result: complete" in out and case.receipts() == 1, "the switch did not complete with the rerun digest")
    expect_exit(case.run(), 0, "state: complete")
    expect(trials(case) == ["design"], "a complete run ran the trials again")


def c_one_byte_grove_resumed(case):
    one_byte(case, "design")
    case.set_control({"actions": [{"point": "switch-after-part:3", "do": "crash"}]})
    expect_exit(case.run(), 99)
    expect(case.state()["state"] == "active", "the crash did not leave the active state")
    out = expect_exit(case.run(), 0, "result: complete")
    expect("command resolved-sources (step 6): exit 0" in out, "the active continuation did not rerun step 6")
    expect(trials(case) == ["design"], f"the continuation ran trials again: {trials(case)}")


def c_trial_fails(case):
    one_byte(case, "design")
    case.set_control({"fail": {"trial": 1}})
    expect_gate_stop(case, "step 4: the staging trial trial-design of design failed")


def c_no_trial(case):
    one_byte(case, "writing")
    plan = load(case.r.plan, {})
    del plan["accepted"]["trials"]["writing"]
    other = os.path.join(case.root, "plan-no-trial.json")
    save(other, plan)
    expect_gate_stop(case, "step 4: the plan names no staging trial for writing", plan=other)


def c_tap_newer(case):
    case.set_control({"tap_grove": "2"})
    expect_gate_stop(case, "the tap's current version 2 is not installed")


def c_linked_older(case):
    """The tap's current version 2 is installed, and Homebrew still links version 1."""
    prefix = os.path.join(case.root, "prefix")
    shutil.copytree(os.path.join(prefix, "Cellar/grove/1"), os.path.join(prefix, "Cellar/grove/2"), symlinks=True)
    case.set_control({"tap_grove": "2"})
    expect_gate_stop(case, "the linked keg is 1, not the tap's current version 2")


def c_opt_elsewhere(case):
    prefix = os.path.join(case.root, "prefix")
    shutil.copytree(os.path.join(prefix, "Cellar/grove/1"), os.path.join(prefix, "Cellar/grove/0"), symlinks=True)
    os.remove(os.path.join(prefix, "opt/grove"))
    os.symlink("../Cellar/grove/0", os.path.join(prefix, "opt/grove"))
    expect_gate_stop(case, "opt/grove reaches .*Cellar/grove/0, not the keg")


def c_grove_version(case):
    write_script(os.path.join(case.root, "prefix/Cellar/grove/1/bin/grove"), "#!/bin/sh\necho 'grove 0'\n")
    expect_gate_stop(case, "printed 'grove 0', not 'grove 1'")


def c_formula(override: dict, text: str):
    def fn(case):
        case.set_control({"mise": override})
        expect_gate_stop(case, text)
    return fn


def c_reader_orphaned(case):
    write(os.path.join(case.home, READERS["grove@linkuistics"], ".orphaned_at"), "1792928060164")
    expect_gate_stop(case, "step 7: the version directory .* of grove@linkuistics holds .orphaned_at")


def c_reader_missing(case):
    shutil.rmtree(os.path.join(case.home, READERS["context7@claude-plugins-official"]))
    expect_gate_stop(case, "step 7: the version directory .* of context7@claude-plugins-official is missing")


def c_reader_unregistered(case):
    old = os.path.join(case.home, ".claude/plugins/cache/linkuistics/grove/99999999")
    write(os.path.join(old, "skills/grove-finish/SKILL.md"), "---\nname: grove-finish\n---\n")
    expect_gate_stop(case, "which finish read, has no registration", env={"OWNER_SWITCH_READERS": old})


def c_reader_named(case):
    read = os.path.join(case.home, READERS["grove@linkuistics"], ".claude-plugin")
    expect_exit(case.run(env={"OWNER_SWITCH_READERS": read}), 0, "result: complete")
    named = [r for r in backup_evidence(case, "7")[0]["result"]["readers"] if r["reason"] == "read"]
    expect(len(named) == 1 and named[0]["read"] == read and named[0]["plugin"] == "grove@linkuistics",
           f"step 7 did not record the directory that finish read: {named}")


def c_step6_active(case):
    case.set_control({"actions": [{"point": "switch-after-part:3", "do": "crash"}]})
    expect_exit(case.run(), 99)
    eleventh_source(case)
    expect_exit(case.run(), 3, "failed probe \\(step 6 failed: resolved-sources\\)")
    st = case.state()
    expect(st["state"] == "native" and st["records"][-1] == "inverse-ended", f"step 6 left {st['state']}")
    expect(case.receipts() == 0, "a failed step 6 copied a receipt")


BASELINE_ROW = "control\t~/scratch\tunknown\tskill\tunknown\t0123456789abcdef\t1\tunknown\t-\t-\tUNCLASSIFIED"


def c_baseline_native(case):
    case.set_control({"baseline_rows": [BASELINE_ROW]})
    out = expect_gate_stop(case, "the switch set differs from the prepared plan; stop for classification")
    expect("changed outside the switch set: baseline row: control | ~/scratch" in out,
           "step 3 did not report the baseline row")


def c_baseline_active(case):
    case.set_control({"actions": [{"point": "switch-after-part:3", "do": "crash"}]})
    expect_exit(case.run(), 99)
    case.set_control({"baseline_rows": [BASELINE_ROW.replace("UNCLASSIFIED", "NEEDS-REVIEW")]})
    mark = len(case.points())
    expect_exit(case.run(), 3, "failed probe \\(changed outside the switch set: baseline row")
    st = case.state()
    expect(st["records"][-1] == "inverse-ended" and case.receipts() == 0, "an anomaly row did not fail the probe")
    expect("command:probe-switched" not in case.points()[mark:], "the probes ran on a layout with an anomaly row")


def c_baseline_fails(case):
    case.set_control({"fail": {"baseline": 1}})
    expect_gate_stop(case, "the baseline instrument baseline failed")


def gate_controls() -> list:
    return [
        ("gates-evidence", c_gates_evidence, {}),
        ("source-eleventh-stops-step-6", c_eleventh_source, {}),
        ("source-one-byte-stops-step-6", c_one_byte_other, {}),
        ("source-one-byte-grove-reruns-trials", c_one_byte_grove, {}),
        ("source-one-byte-grove-resumed", c_one_byte_grove_resumed, {}),
        ("source-grove-trial-fails", c_trial_fails, {}),
        ("source-grove-no-trial", c_no_trial, {}),
        ("grove-tap-newer", c_tap_newer, {}),
        ("grove-linked-older", c_linked_older, {}),
        ("grove-opt-elsewhere", c_opt_elsewhere, {}),
        ("grove-version-differs", c_grove_version, {}),
        ("formula-missing", c_formula({"linkuistics/taps/personal": {"state": "missing"}},
                                      "step 5: linkuistics/taps/personal is missing, not installed"), {}),
        ("formula-undeclared", c_formula({"linkuistics/taps/testanyware": None},
                                         "step 5: linkuistics/taps/testanyware is not declared"), {}),
        ("formula-version", c_formula({"linkuistics/taps/superpowers": {"installed_version": "9"}},
                                      "step 5: linkuistics/taps/superpowers is at 9, not its accepted version 4"), {}),
        ("reader-orphaned", c_reader_orphaned, {}),
        ("reader-missing", c_reader_missing, {}),
        ("reader-named-unregistered", c_reader_unregistered, {}),
        ("reader-named", c_reader_named, {}),
        ("step-6-active-eleventh", c_step6_active, {}),
        ("baseline-anomaly-native", c_baseline_native, {}),
        ("baseline-anomaly-active", c_baseline_active, {}),
        ("baseline-fails", c_baseline_fails, {}),
    ]


def all_controls(r: Rehearsal) -> list:
    parts = load(r.plan, {})["parts"]
    files = [i for i, p in enumerate(parts) if p["kind"] == "file"]
    controls = [
        ("dry-run", c_dry_run, {}),
        ("switch", c_switch, {}),
        ("rehearsal-missing", c_no_rehearsal, {"rehearsed": False}),
        ("rehearsal-changed-program", c_changed_program, {}),
        ("rehearsal-changed-plan", c_changed_plan, {}),
        ("continuation-runs-backup-copy", c_continuation_copy, {}),
        ("probe-fails", c_probe_fails, {}),
    ]
    points = ["before-backup-rename", "after-backup-rename"]
    for i, p in enumerate(parts):
        points += [f"switch-before-part:{i}", f"switch-read:{i}"]
        if p["kind"] == "file":
            points.append(f"switch-before-exchange:{i}")
        points += [f"switch-exchanged:{i}", f"switch-after-part:{i}"]
    points += ["command:probe-switched", "command:mac-check", "before-record:receipt",
               "after-record:receipt", "step-14"]
    controls += [(f"crash-{pt}", crash_then_complete(pt), {}) for pt in points]
    inverse_points = ["before-record:inverse-begun", "after-record:inverse-begun"]
    for i, p in reversed(list(enumerate(parts))):
        inverse_points.append(f"inverse-before-part:{i}")
        if p["kind"] == "file":
            inverse_points += [f"inverse-read:{i}", f"inverse-before-exchange:{i}", f"inverse-exchanged:{i}"]
        else:
            inverse_points.append(f"inverse-read:{i}")
        inverse_points.append(f"inverse-after-part:{i}")
    inverse_points += ["before-record:inverse-ended", "command:probe-native"]
    controls += [(f"crash-{pt}", inverse_crash_then_complete(pt), {}) for pt in inverse_points]
    for i in files:
        rel = parts[i]["path"]
        controls += [
            (f"write-before-exchange:{i}", write_before_exchange(rel, i, "write-key"), {}),
            (f"erase-before-exchange:{i}", write_before_exchange(rel, i, "erase-key"), {}),
            (f"write-after-exchange:{i}", write_after_exchange(rel, i), {}),
            (f"inverse-write-before-exchange:{i}", write_before_exchange(rel, i, "write-key", "inverse"), {}),
            (f"inverse-write-after-exchange:{i}", write_after_exchange(rel, i, "inverse"), {}),
            (f"descriptor-inside:{i}", descriptor_write(rel, i, "inside"), {}),
            (f"descriptor-after-run:{i}", descriptor_write(rel, i, "after"), {}),
        ]
    controls += [
        ("metadata-mode", metadata_change("chmod"), {}),
        ("metadata-xattr", metadata_change("xattr"), {}),
        ("metadata-acl", metadata_change("acl"), {}),
        ("acl-denies-exchange", c_acl_denies_exchange, {}),
        ("third-before-switch", c_third_before, {}),
        ("third-after-receipt", c_third_after_receipt, {}),
        ("third-before-receipt", c_third_before_receipt, {}),
        ("switched-while-open", c_switched_while_open, {}),
        ("switched-after-inverse", c_switched_after_inverse, {}),
        ("entry-content-before", entry_change("modify-entry", ".claude/skills/grove", "before"), {}),
        ("entry-content-after", entry_change("modify-entry", ".claude/skills/grove", "after"), {}),
        ("entry-retarget-before", entry_change("retarget", ".codex/skills/grove", "before"), {}),
        ("entry-retarget-after", entry_change("retarget", ".codex/skills/grove", "after"), {}),
        ("entry-occupied-after", entry_change("create", ".codex/skills/grove", "after"), {}),
        ("entry-occupied-after-inverse-read", c_occupied_after_read, {}),
        ("step-14-resume", c_step14_resume, {}),
        ("outside-entry-inside-run", c_outside_inside_run, {}),
        ("outside-entry-after-stop", c_outside_after_stop, {}),
        ("cell-after-receipt-no-copy", c_cell_after_receipt_no_copy, {}),
        ("writer-before-run", c_writer_before, {}),
        ("writer-later", interval_closed("spawn-writer"), {}),
        ("delegate-before-run", session_process(["/bin/bash", "-c", "exec -a fixture-delegate /bin/sleep 120"]), {}),
        ("background-before-run", session_process(["/bin/sleep", "120"]), {}),
        ("delegate-later", interval_closed("spawn-in-session",
                                           ["/bin/bash", "-c", "exec -a fixture-delegate /bin/sleep 120"]), {}),
        ("background-later", interval_closed("spawn-in-session", ["/bin/sleep", "120"]), {}),
        ("job-before-run", c_job, {}),
        ("prepared-plan-differs", c_prepared_differs, {}),
    ]
    controls += prelude_controls()
    controls += gate_controls()
    return controls


def prelude_controls() -> list:
    no_file, no_key, no_hooks = {"ravel": "absent-file"}, {"ravel": "absent-key"}, {"hooks": "absent"}
    controls = [
        ("native-absent-file", c_native_full, no_file),
        ("native-absent-key", c_native_full, no_key),
        ("native-and-hooks", c_native_full, dict(no_key, **no_hooks)),
    ]
    for pt in ("prelude:native-settings", "gate:the native settings", "native-before-create", "native-created"):
        controls.append((f"native-absent-file-crash-{pt}", prelude_crash(pt), no_file))
    for pt in ("prelude:native-settings", "gate:the native settings", "native-read", "native-recorded",
               "native-exchanged", "native-compared"):
        controls.append((f"native-absent-key-crash-{pt}", prelude_crash(pt), no_key))
    controls += [
        ("native-write-before-exchange", c_native_write_before_exchange, no_key),
        ("native-write-then-crash-before-comparison", c_native_write_then_crash, no_key),
        ("native-descriptor-after-comparison", c_native_descriptor, no_key),
        ("native-third-value", c_native_third, {}),
        ("native-erased-after-inverse", c_native_erased_after_inverse, {}),
        ("native-writer-later", prelude_writer("prelude:native-settings", "the native settings"), no_key),
        ("hooks-absent", c_hooks_full, no_hooks),
        ("hooks-crash-between-preludes", prelude_crash("prelude:hooks-installation"), dict(no_key, **no_hooks)),
    ]
    for label in ("marketplace-add", "install"):
        for pt in ("command", "renamed", "compared"):
            controls.append((f"hooks-crash-hooks-{pt}:{label}", prelude_crash(f"hooks-{pt}:{label}"), no_hooks))
        controls.append((f"hooks-crash-held-link:{label}", c_hooks_held_crash(label), no_hooks))
    for pt in ("gate:the hooks installation", "hooks-before-held:marketplace-add", "hooks-before-disable",
               "hooks-disable-read", "hooks-disable-recorded", "hooks-disable-exchanged", "hooks-disable-compared"):
        controls.append((f"hooks-crash-{pt}", prelude_crash(pt), no_hooks))
    controls += [
        ("hooks-crash-after-registration", c_hooks_after_registration, no_hooks),
        ("hooks-write-inside-interval", c_hooks_write_in_interval, no_hooks),
        ("hooks-descriptor-after-comparison", c_hooks_descriptor, no_hooks),
        ("hooks-third-value-before-held-link", c_hooks_third_before_held, no_hooks),
        ("hooks-command-in-place", c_hooks_in_place, no_hooks),
        ("hooks-command-fails", c_hooks_command_fails, no_hooks),
        ("hooks-writer-later", prelude_writer("prelude:hooks-installation", "the hooks installation"), no_hooks),
    ]
    for do in ("xattr", "acl"):
        for when in ("initial", "before-held", "after-reading"):
            controls.append((f"hooks-{do}-{when}", hooks_metadata(do, when), no_hooks))
    enabled = {"hooks": "enabled"}
    controls += [
        ("hooks-enabled", run_to_complete, enabled),
        ("hooks-disable-write-before-exchange", hooks_disable_fault("write-key", "before"), enabled),
        ("hooks-disable-erase-before-exchange", hooks_disable_fault("erase-key", "before"), enabled),
        ("hooks-disable-write-after-exchange", hooks_disable_fault("write-key", "after"), enabled),
        ("hooks-disable-descriptor", hooks_disable_fault("descriptor", "before"), enabled),
        ("open-run-no-prelude-active", open_run_no_prelude("switch-after-part:3", "active"), {}),
        ("open-run-no-prelude-complete", open_run_no_prelude("step-14", "complete"), {}),
        ("prelude-rehearsal-missing", c_prelude_unrehearsed, dict(no_key, rehearsed=False, **no_hooks)),
        ("prelude-rehearsal-other-plan", c_prelude_other_plan, dict(no_key, rehearsed=False)),
    ]
    return controls


def cmd_rehearse(args) -> int:
    r = Rehearsal(args.out, args.program)
    for name, fn, kw in all_controls(r):
        if args.only and not re.search(args.only, name):
            continue
        r.control(name, fn, **kw)
    rerun = settings_writes(r.out) if args.settings_writes else None
    controls_passed = all(x["passed"] for x in r.results)
    rerun_passed = rerun is None or rerun["passed"]
    passed = controls_passed and rerun_passed and not args.only
    record = {
        "schema": "owner-switch-rehearsal/1",
        "passed": passed,
        "plan": r.plan, "plan_digest": r.plan_digest,
        "program": r.program, "program_digest": r.program_digest,
        "python": platform.python_version(), "macos": platform.mac_ver()[0],
        "controls": len(r.results), "failed": [x["control"] for x in r.results if not x["passed"]],
        "results": r.results,
        "settings_writes": rerun,
        "partial": bool(args.only),
    }
    save(os.path.join(r.out, "record.json"), record)
    print(f"{len(r.results) - len(record['failed'])}/{len(r.results)} controls passed; record {r.out}/record.json")
    if rerun is not None:
        print(f"settings-writes.sh rerun with claude {rerun['claude_version']}: "
              f"{'passed' if rerun['passed'] else 'FAILED: ' + rerun['detail']}")
    return 0 if controls_passed and rerun_passed else 1


SETTINGS_WRITES = os.path.join(HERE, "..", "docs/evaluations/hooks-installation-writes/settings-writes.sh")
# docs/homebrew-skill-delivery-evidence.md, "The interval between the read and
# the rename": each command made its write from a read 30 to 38 ms before
# its rename.
EVIDENCE_RANGE_MS = (30.0, 38.0)


def settings_writes(out: str) -> dict:
    """Rerun settings-writes.sh with the real claude, and check its write method."""
    work = os.path.join(out, "settings-writes")
    if os.path.exists(work):
        shutil.rmtree(work)
    scratch = os.path.join(work, "scratch")
    os.makedirs(scratch)
    script = os.path.realpath(SETTINGS_WRITES)
    # The script's shebang finds bash on PATH. The system's bash 3.2 expands
    # braces inside a quoted command substitution, which breaks its jq filters.
    result = subprocess.run([script, scratch], capture_output=True, text=True, env=clean_env(), timeout=7200)
    text = result.stdout + result.stderr
    with open(os.path.join(work, "output.md"), "w") as f:
        f.write(text)
    version = re.search(r"^claude (.+?) sha256 (\w+)$", text, re.M)
    renames = {m.group(1): int(m.group(2)) for m in
               re.finditer(r"^### (marketplace add|plugin install|plugin disable):.*?\n.*?"
                           r"^renames onto the path: (\d+)$", text, re.M | re.S)}
    races = {}
    for m in re.finditer(r"^(add|install|disable)\.(inplace|rename)\s+runs \d+ .*? missing from live at leads "
                         r"([\d.]+)\.\.([\d.]+) ms, in live at leads from ([\d.]+) ms$", text, re.M):
        races.setdefault(m.group(1), []).append((float(m.group(4)), float(m.group(5))))
    intervals = {cmd: {"from_ms": max(lost for lost, _ in rows), "to_ms": min(kept for _, kept in rows)}
                 for cmd, rows in races.items()}
    # The criterion reads the measured sections, not the exit status. The
    # script's last section starts codebase-memory-mcp in the scratch HOME,
    # and that server refuses to start while an account daemon serves another
    # cache directory. A section that did not run leaves its result absent.
    problems = []
    if not version:
        problems.append("the output names no Claude Code version")
    for label in ("marketplace add", "plugin install"):
        if renames.get(label) != 1:
            problems.append(f"{label} renamed {renames.get(label)} files onto the path, not 1")
    low, high = EVIDENCE_RANGE_MS
    for cmd in ("add", "install"):
        iv = intervals.get(cmd)
        if iv is None:
            problems.append(f"no race result for {cmd}")
        elif iv["from_ms"] > high or iv["to_ms"] < low:
            problems.append(f"the interval of {cmd}, {iv['from_ms']} to {iv['to_ms']} ms, misses {low} to {high} ms")
    return {
        "passed": not problems, "detail": "; ".join(problems) or "passed",
        "claude_version": version.group(1) if version else None,
        "claude_sha256_prefix": version.group(2) if version else None,
        "script_digest": digest_file(script), "script_exit": result.returncode,
        "renames": renames, "intervals_ms": intervals,
        "output": os.path.join(work, "output.md"),
    }


def main() -> int:
    if len(sys.argv) > 1 and sys.argv[1] == "fixture-command":
        return cmd_fixture(sys.argv[2])
    if len(sys.argv) > 1 and sys.argv[1] == "hook":
        return cmd_hook(sys.argv[2])
    if len(sys.argv) > 1 and sys.argv[1] == "holder":
        return cmd_holder(sys.argv[2], sys.argv[3])
    if len(sys.argv) > 1 and sys.argv[1] == "stub-claude":
        return cmd_stub_claude(sys.argv[2:])
    if len(sys.argv) > 1 and sys.argv[1] == "stub-brew":
        return cmd_stub_brew(sys.argv[2:])
    if len(sys.argv) > 1 and sys.argv[1] == "stub-mise":
        return cmd_stub_mise(sys.argv[2:])
    if len(sys.argv) > 1 and sys.argv[1] == "stub-ravel":
        return cmd_stub_ravel(sys.argv[2:])
    if len(sys.argv) > 1 and sys.argv[1] == "harness":
        sep = sys.argv.index("--")
        return cmd_harness(sys.argv[3], sys.argv[sep + 1:])
    parser = argparse.ArgumentParser(description=__doc__.split("\n", 1)[0])
    sub = parser.add_subparsers(dest="command", required=True)
    p = sub.add_parser("rehearse")
    p.add_argument("--out", required=True)
    p.add_argument("--program", default=os.path.join(HERE, "owner_switch.py"))
    p.add_argument("--only", help="run only the controls whose name matches; the record is then partial")
    p.add_argument("--settings-writes", action="store_true",
                   help="also rerun settings-writes.sh with the real claude, and name its version in the record")
    args = parser.parse_args()
    return cmd_rehearse(args)


if __name__ == "__main__":
    sys.exit(main())
