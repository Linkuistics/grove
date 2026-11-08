#!/usr/bin/env python3
"""Rehearse the owner switch program on a fixture plan in disposable layouts.

docs/specs/owner-skill-delivery.md, "The rehearsed plan" and "Test seams",
owns the controls. Each control builds a new disposable HOME under OUT, runs
scripts/owner_switch.py under a fixture harness process, and checks the
layout, the run record and the program's output. A hook acts at the
program's named points: it crashes the program, writes as another program,
holds a descriptor or launches a process. The real HOME is never changed,
and the rehearsal reads only its `mac-check` command and that command's
folder.

Usage:
    python3 -I owner_switch_rehearsal.py rehearse --out OUT [--program PATH] [--only REGEX]
                                                  [--skip-settings-writes]

The record OUT/record.json names the digests of the plan and the program and
the result of each control. For each run of a control, it counts the Claude
Code commands that the run ran. It is a passing rehearsal record of the
program only when every control passed and the rerun of settings-writes.sh
passed. OUT/plan.json is the fixture plan.

Each run finds stub `claude` and `brew` commands first on PATH. The stub
`claude` keeps its plugin state in the disposable HOME, and it writes
settings.json as Claude Code 2.1.319 does: one read, then a new file renamed
onto the path. The point "stub:<command>:read" lies between that read and
the rename.

Each rehearsal also reruns
docs/evaluations/hooks-installation-writes/settings-writes.sh with the real
`claude` on PATH, in OUT/settings-writes. The record names that Claude Code
version, and the program's hooks installation runs a Claude Code command only
with a version that a passing record names. --skip-settings-writes leaves the
rerun out, and the record is then partial. The rerun passes when `marketplace add` and `plugin install`
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
one such name, so each resolution shows the gate's environment rule. Each run
also sets CLAUDE_CONFIG_DIR and CODEX_HOME. Each fixture command fails when
its environment selects another subject than its HOME. A staging trial fails
unless HOME is the plan's staging HOME and CODEX_HOME is its .codex tree.

The controls cover the switch, the inverse, detection, interruptions,
entries, step 14, the processes, the rehearsal condition, the preludes of
step 3, its baseline rows and the gates with their source controls. They
also cover the owner's inverse, the merge entry, the purge check, the
pending-activation record and the switch's area of the owner's `mac-check`.
The native-off controls run `native-off` without a plan or a switch backup.
They cover an absent file, an absent key with its metadata, an existing off
value and each refused form. They cover another Ravel process, a process
titled ravel, a Ravel ancestor, other tools' processes, which do not stop
it, a stop of the switch before its backup and --print-command. They
also cover a failed creation, a failed rename and a crash at each named
point. A reader polls the file during repeated publications.

The process controls cover the narrow process inspection. One control passes
each measured and constructed launch form through the program's recognise().
The others run real processes: APFS clones of this interpreter in
OUT/fixture-bin under the names and paths of the four tools, of `node`,
`python3`, `caffeinate` and `helper`, with Node entry scripts, an npm bin link
and a wrapper script. They cover native and versioned executables, Node
launchers, reparented Codex daemons, unrelated arguments, review forms, a
deleted executable, exact ancestors, a descendant harness, a caffeinate
child, an MCP server and its tool child, and probe-owned records with a
changed creation time or executable. Each control checks that the fixture
processes still run, because the program signals none.
OWNER_SWITCH_REHEARSAL_SCOPE limits recognition to that folder in every run,
because the machine's own tool processes share the process table. The
persistence control fails unless OUT is on the volume of
the account's HOME. The control of the accepted 2.0 build needs its darwin
archive in target/ravel-archives-2.0, and the digest of
scripts/harness-trials/builds.json.
The serial controls run `attempt`, `switch` and `restore` on a HOME before
the hooks installation. Each case computes the plan's "switch" section for
its own HOME, because installed_plugins.json names absolute paths. They
cover the full switch with a later restore through the note's literal
command and its recovery backup, and a stop before the backup's
publication. They also cover an attempt mismatch, a later restore that
drift refuses, and refusals before any write. A caught failure or a crash
at each named point completes them. The hook action "fail" makes the hook
exit 1, so the program meets a caught failure at that point.
Further serial controls cover ACLs and attributes through backup and
restore, failed reads and changed copies before publication, and changed
backup records. They cover interrupted and failed restores, the off-value
exception, an interrupted later restore and the owner's restore after
finish. They also cover Claude Code command failures, an unwritable
manual, a live probe-owned service and changes during the probes. The
hook action "append-glob" changes each file that a pattern names. The
fixture flag "probe_owned" makes the switched probe leave a service
running. The off-value controls make their backup through the program's
own `create_backup`, because the switch refuses without the off value.
Each `mac-check` control runs the real command with HOME set to the case's
HOME, which links the checker's folder and holds copies of the plan and the
program in <state>/prepared. That command file is the only file of the
account's HOME that the rehearsal reads.
"""
from __future__ import annotations

import argparse
import contextlib
import glob
import hashlib
import importlib.util
import io
import json
import os
import platform
import pwd
import re
import shlex
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
# The fixture executables in OUT/fixture-bin: clones of this interpreter under
# the names and paths of the four tools and of other programs. Each sleeps.
FIXTURE_NAMES = ("ravel", "helper", "claude", "codex", "grove", "grove-llm", "node", "python3", "caffeinate",
                 "share/claude/versions/2.1.319")
SLEEP = "import time; time.sleep(120)"
# The fixture scripts of the Node and wrapper forms, relative to OUT/fixture-bin.
FIXTURE_SCRIPTS = ("lib/node_modules/@openai/codex/bin/codex.js",
                   "lib/node_modules/@anthropic-ai/claude-code/cli.js",
                   "lib/other-server.js", "lib/grove.js", "wrappers/codex")
# The version that the stub claude prints, and that a fixture record measured.
FIXTURE_CLAUDE = "2.1.319 (Claude Code)"
# The owner's `mac-check`. The machine contract owns it, and the rehearsal runs
# it against each disposable HOME, never against the account's HOME.
MAC_CHECK = os.path.join(os.path.realpath(pwd.getpwuid(os.getuid()).pw_dir), ".local/bin/mac-check")
# No command of a plan may inherit these: they select another configuration than HOME's.
CLAUDE_LOCATION_VARS = (
    "CLAUDE_CONFIG_DIR", "CLAUDE_SECURESTORAGE_CONFIG_DIR", "CLAUDE_CODE_PLUGIN_CACHE_DIR",
    "CLAUDE_CODE_PLUGIN_DIRS", "CLAUDE_CODE_PLUGIN_SEED_DIR",
)
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


KNOWN_MARKETPLACES = ".claude/plugins/known_marketplaces.json"
INSTALLED_PLUGINS = ".claude/plugins/installed_plugins.json"
# The serial switch's write set and its restore order: the Ravel settings first.
SERIAL_WRITES = ((".claude/skills/grove", "entry"), (".codex/skills/grove", "entry"),
                 (".claude/settings.json", "file"), (KNOWN_MARKETPLACES, "file"), (INSTALLED_PLUGINS, "file"),
                 (".config/ravel/settings.json", "file"))
SERIAL_RESTORE = (".config/ravel/settings.json", KNOWN_MARKETPLACES, INSTALLED_PLUGINS, ".claude/settings.json",
                  ".claude/skills/grove", ".codex/skills/grove")


def switch_fields(program: str, home: str) -> dict:
    """The plan's switch section for HOME: the whole write set with its originals, the expected
    inventories, the command inputs by content, the delivery assertions and the restore order.

    The originals and the inventories depend on HOME, because installed_plugins.json names
    absolute paths, so each case of the serial switch computes its own section.
    """
    def program_line(command: str, rel: str) -> str:
        return subprocess.run([PY, "-I", program, command, os.path.join(home, rel)],
                              capture_output=True, text=True, check=True).stdout.strip()

    def original(rel: str) -> dict:
        found = program_line("restore-identity", rel)
        return {"absent": True} if found == "absent" else {"identity": found}

    def root(rel: str, drop=()) -> dict:
        names = sorted(n for n in os.listdir(os.path.join(home, rel)) if n not in drop)
        return {"root": rel, "entries": {n: program_line("identity", f"{rel}/{n}") for n in names}}

    return {
        "writes": [{"path": rel, "type": kind, "original": original(rel)} for rel, kind in SERIAL_WRITES],
        "restore": list(SERIAL_RESTORE),
        "inventory": {"before": [root(".claude/skills"), root(".codex/skills")],
                      "after": [root(".claude/skills", ["grove"]), root(".codex/skills", ["grove"])]},
        "inputs": [{"role": "program", "path": "$program", "digest": digest_file(program)},
                   {"role": "fixture commands", "path": SELF, "digest": digest_file(SELF)}],
        "assertions": [{"path": ".config/ravel/settings.json", "key": ["skills"], "value": SWITCHED},
                       {"path": ".claude/settings.json", "key": SUPERSEDED_PLUGIN, "value": False},
                       {"path": ".claude/settings.json", "key": HOOKS_PLUGIN, "value": True},
                       {"path": ".claude/skills/grove", "absent": True},
                       {"path": ".codex/skills/grove", "absent": True}],
        "tools": {"claude": FIXTURE_CLAUDE},
        "retained": [{"item": "the original driver's Ravel keg", "path": "/opt/homebrew/Cellar/ravel/1.1.0",
                      "condition": ["its last driver ends"]}],
    }


def build_plan(program: str, out: str) -> str:
    # The retained staging HOME, with its own .codex tree, and the candidate keg.
    os.makedirs(os.path.join(out, "staging-home", ".codex"), exist_ok=True)
    os.makedirs(os.path.join(out, "candidate-keg"), exist_ok=True)
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
        "baseline": {"name": "baseline", "argv": fixture_command("baseline")},
        "gates": {"4": [gate_command("4", "grove-release")], "5": [gate_command("5", "skill-formulae")],
                  "6": [gate_command("6", "resolved-sources")], "7": [gate_command("7", "reader-evidence")]},
        "accepted": {
            "sources": sources,
            "versions": {f"linkuistics/taps/{name}": version for name, (version, _) in KEGS.items() if name != "grove"},
            "trials": {sid: [{"name": f"trial-{sid}", "argv": fixture_command("trial")}] for sid in KEGS["grove"][1]},
            "staging": {"home": os.path.join(out, "staging-home"), "candidate_keg": os.path.join(out, "candidate-keg")},
        },
        "probes": [{"name": "probe-switched", "argv": fixture_command("probe-switched")}],
        "native_probe": [{"name": "probe-native", "argv": fixture_command("probe-native")}],
        "mac_check": {"name": "mac-check", "argv": fixture_command("mac-check")},
    }
    # The serial switch starts before the hooks installation, so its fixture HOME lacks the plugin.
    serial = os.path.join(out, "plan-home-serial")
    if os.path.exists(serial):
        shutil.rmtree(serial)
    build_home(serial, os.path.join(prefix, "opt/grove"), hooks="absent")
    plan["switch"] = switch_fields(program, serial)
    shutil.rmtree(serial)
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


def environment_problems(name: str) -> list:
    """Each variable that would point a fixture command at another subject than its own."""
    problems = [k for k in os.environ if k in CLAUDE_LOCATION_VARS or k.startswith("RAVEL_")]
    if name != "trial":
        if "CODEX_HOME" in os.environ:
            problems.append("CODEX_HOME")
        if os.environ.get("HOME") != os.environ["OWNER_SWITCH_HOME"]:
            problems.append("HOME " + os.environ.get("HOME", ""))
        return problems
    # A staging trial runs in the retained staging HOME, with its own .codex tree.
    staging = load(os.environ["OWNER_SWITCH_PLAN"], {})["accepted"]["staging"]["home"]
    if os.environ.get("HOME") != staging:
        problems.append("HOME " + os.environ.get("HOME", ""))
    if os.environ.get("CODEX_HOME") != os.path.join(staging, ".codex"):
        problems.append("CODEX_HOME " + os.environ.get("CODEX_HOME", ""))
    if os.path.realpath(os.getcwd()) != os.path.realpath(staging):
        problems.append("the working directory " + os.getcwd())
    return problems


def cmd_fixture(name: str) -> int:
    root = case_root()
    home = os.environ["OWNER_SWITCH_HOME"]
    leaked = environment_problems(name)
    if leaked:
        print(f"{name}: the environment selects another subject: {', '.join(sorted(leaked))}")
        return 1
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
    if name == "probe-switched" and load(os.path.join(root, "control.json"), {}).get("probe_owned"):
        # A service that the probe proves it created and leaves running: "Probe process lifetime".
        p = subprocess.Popen([PY, "-I", "-c", SLEEP], start_new_session=True, stdin=subprocess.DEVNULL,
                             stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        record_pid(root, p.pid)
        start = program_module(os.path.join(HERE, "owner_switch.py")).process_start(p.pid)
        print("owner-switch-result: " + json.dumps({"owned": [
            {"pid": p.pid, "start": start, "executable": os.path.realpath(PY), "home": home}]}))
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
    crash = failing = False
    for i, action in enumerate(control.get("actions", [])):
        if action["point"] != point or i in fired:
            continue
        fired.append(i)
        save(os.path.join(root, "fired.json"), fired)
        do = action["do"]
        path = os.path.join(home, action.get("path", ""))
        if do == "crash":
            crash = True
        elif do == "fail":
            # A caught failure: the program stops at this point as after a failed read or write.
            failing = True
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
        elif do == "touch":
            write(path, action.get("text", ""))
        elif do == "append-glob":
            # Change each existing file that the pattern names, such as a copy inside an unpublished backup.
            for p in glob.glob(path):
                with open(p, "a") as f:
                    f.write("changed\n")
        elif do == "modify-entry":
            with open(os.path.join(path, "SKILL.md"), "a") as f:
                f.write("changed\n")
        elif do == "retarget":
            os.remove(path)
            os.symlink("/elsewhere", path)
        elif do == "spawn-tool":
            # A tool process outside the session, which the hook leaves behind.
            p = subprocess.Popen(action["argv"], executable=action.get("executable"),
                                 env={"PYTHONHOME": sys.base_prefix}, start_new_session=True,
                                 stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
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
    return 99 if crash else (1 if failing else 0)


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
    if any(v in os.environ for v in CLAUDE_LOCATION_VARS + ("CODEX_HOME",)):
        print("stub claude: a Claude Code location variable or CODEX_HOME is set", file=sys.stderr)
        return 2
    root = case_root()
    if argv == ["--version"]:
        print(load(os.path.join(root, "control.json"), {}).get("claude_version", FIXTURE_CLAUDE))
        return 0
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
        spec = args if isinstance(args, dict) else {"argv": args}
        p = subprocess.Popen(spec["argv"], executable=spec.get("executable"), stdin=subprocess.DEVNULL,
                             stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
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

    def rehearse_record(self, plan_digest: str, program_digest: str, settings_writes: bool = True) -> None:
        folder = os.path.join(self.home, STATE, "rehearsals")
        os.makedirs(folder, exist_ok=True)
        record = {"schema": "owner-switch-rehearsal/1", "passed": True,
                  "plan_digest": plan_digest, "program_digest": program_digest}
        if settings_writes:
            record["settings_writes"] = {"passed": True, "claude_version": FIXTURE_CLAUDE}
        save(os.path.join(folder, f"fixture-{plan_digest[:8]}-{program_digest[:8]}.json"), record)

    def set_control(self, control: dict) -> None:
        save(os.path.join(self.root, "control.json"), control)
        for name in ("fired.json", "failed.json"):
            p = os.path.join(self.root, name)
            if os.path.exists(p):
                os.remove(p)

    def run(self, program=None, plan=None, dry=False, env=None, via=None) -> tuple:
        """Run the switch under the fixture harness process. VIA names launchers that become its ancestors."""
        args = [PY, "-I", program or self.r.program, "run", "--plan", plan or self.r.plan, "--home", self.home]
        if dry:
            args.append("--dry-run")
        # A Claude Code location variable or CODEX_HOME in the environment
        # must not reach a command: each fixture command and the stub refuse
        # it, and so does a staging launch.
        env = dict(clean_env(), OWNER_SWITCH_REHEARSAL_HOOK=self.r.hook,
                   PATH=self.r.stubs + os.pathsep + os.environ.get("PATH", ""),
                   CLAUDE_CONFIG_DIR=os.path.join(self.root, "elsewhere"),
                   CODEX_HOME=os.path.join(self.root, "elsewhere", ".codex"),
                   # The stub ravel refuses every RAVEL_* name, so no resolution may inherit it.
                   RAVEL_SKILL_DIRECTORIES=os.path.join(self.root, "elsewhere"),
                   # The inspection recognises only the fixture executables, which need PYTHONHOME.
                   OWNER_SWITCH_REHEARSAL_SCOPE=self.r.fixture_bin, PYTHONHOME=sys.base_prefix, **(env or {}))
        before = len(self.claude_calls())
        result = subprocess.run([PY, "-I", SELF, "harness", HARNESS_MARK, self.root, "--"] + (via or []) + args,
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

    def serial_plan(self, mutate=None) -> str:
        """The fixture plan with the switch section of this case's HOME, as `self.serial`."""
        plan = load(self.r.plan, {})
        plan["switch"] = switch_fields(self.r.program, self.home)
        if mutate:
            mutate(plan)
        self.serial = os.path.join(self.root, "serial-plan.json")
        save_json(self.serial, plan)
        return self.serial

    def invoke(self, argv: list, via=None) -> tuple:
        """Run ARGV under the fixture harness process, with the hook, the stubs and the fixture scope."""
        env = dict(clean_env(), OWNER_SWITCH_REHEARSAL_HOOK=self.r.hook,
                   PATH=self.r.stubs + os.pathsep + os.environ.get("PATH", ""),
                   CLAUDE_CONFIG_DIR=os.path.join(self.root, "elsewhere"),
                   CODEX_HOME=os.path.join(self.root, "elsewhere", ".codex"),
                   RAVEL_SKILL_DIRECTORIES=os.path.join(self.root, "elsewhere"),
                   OWNER_SWITCH_REHEARSAL_SCOPE=self.r.fixture_bin, PYTHONHOME=sys.base_prefix)
        before = len(self.claude_calls())
        result = subprocess.run([PY, "-I", SELF, "harness", HARNESS_MARK, self.root, "--"] + (via or []) + argv,
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
            f.write(f"$ {' '.join(argv[3:6])} (exit {result.returncode})\n{out}\n")
        return result.returncode, out

    def attempt(self) -> dict:
        """Write an attempt note for `self.serial`, and return it with its path."""
        out = expect_exit(self.invoke([PY, "-I", self.r.program, "attempt", "--plan", self.serial,
                                       "--home", self.home]), 0, "attempt note: ")
        path = re.search(r"^attempt note: (.+)$", out, re.M).group(1)
        return dict(load(path, {}), _path=path)

    def switch(self, note: dict, via=None) -> tuple:
        return self.invoke([PY, "-I", self.r.program, "switch", "--plan", self.serial, "--home", self.home,
                            "--attempt", note["_path"]], via=via)

    def restore(self, backup: str, note: dict, home=None, extra=(), via=None) -> tuple:
        return self.invoke([PY, "-I", self.r.program, "restore", "--backup", backup, "--attempt", note["_path"],
                            "--home", home or self.home, *extra], via=via)

    def literal(self, command: str) -> tuple:
        """Run a literal command of an attempt note, as the owner pastes it."""
        return self.invoke(shlex.split(command))

    def native_off(self, *extra: str, argv=None, via=None) -> tuple:
        """Run native-off under the fixture harness process, with the hook and the fixture scope.

        VIA names a launcher that runs the command as its child, so the
        launcher is an ancestor of the program.
        """
        args = argv or [PY, "-I", self.r.program, "native-off", "--home", self.home, *extra]
        env = dict(clean_env(), OWNER_SWITCH_REHEARSAL_HOOK=self.r.hook,
                   OWNER_SWITCH_REHEARSAL_SCOPE=self.r.fixture_bin, PYTHONHOME=sys.base_prefix)
        result = subprocess.run([PY, "-I", SELF, "harness", HARNESS_MARK, self.root, "--"] + (via or []) + args,
                                env=env, capture_output=True, text=True, timeout=120)
        out = result.stdout + result.stderr
        self.outputs.append(out)
        self.runs.append({"exit": result.returncode, "claude": {}})
        with open(os.path.join(self.root, "runs.log"), "a") as f:
            f.write(f"$ native-off {' '.join(extra)} (exit {result.returncode})\n{out}\n")
        return result.returncode, out

    def processes(self, *extra: str, via=None) -> tuple:
        """Run the `processes` entry under the fixture harness process, with the fixture scope."""
        args = [PY, "-I", self.r.program, "processes", "--home", self.home, *extra]
        env = dict(clean_env(), OWNER_SWITCH_REHEARSAL_SCOPE=self.r.fixture_bin, PYTHONHOME=sys.base_prefix)
        result = subprocess.run([PY, "-I", SELF, "harness", HARNESS_MARK, self.root, "--"] + (via or []) + args,
                                env=env, capture_output=True, text=True, timeout=120)
        out = result.stdout + result.stderr
        self.outputs.append(out)
        self.runs.append({"exit": result.returncode, "claude": {}})
        with open(os.path.join(self.root, "runs.log"), "a") as f:
            f.write(f"$ processes {' '.join(extra)} (exit {result.returncode})\n{out}\n")
        return result.returncode, out

    def entry(self, command: str, *extra: str, program=None) -> tuple:
        """Run one of the owner's entries under the fixture harness process."""
        args = [PY, "-I", program or self.r.program, command, "--plan", self.r.plan, "--home", self.home,
                *extra]
        env = dict(clean_env(), PATH=self.r.stubs + os.pathsep + os.environ.get("PATH", ""),
                   CLAUDE_CONFIG_DIR=os.path.join(self.root, "elsewhere"),
                   CODEX_HOME=os.path.join(self.root, "elsewhere", ".codex"),
                   RAVEL_SKILL_DIRECTORIES=os.path.join(self.root, "elsewhere"),
                   OWNER_SWITCH_REHEARSAL_SCOPE=self.r.fixture_bin, PYTHONHOME=sys.base_prefix)
        result = subprocess.run([PY, "-I", SELF, "harness", HARNESS_MARK, self.root, "--"] + args,
                                env=env, capture_output=True, text=True, timeout=300)
        out = result.stdout + result.stderr
        self.outputs.append(out)
        with open(os.path.join(self.root, "runs.log"), "a") as f:
            f.write(f"$ {command} {' '.join(extra)} (exit {result.returncode})\n{out}\n")
        return result.returncode, out

    def mac_check(self, prepared: bool = True) -> tuple:
        """The real `mac-check`, for the switch's area only, with HOME set to this case's HOME.

        The case's HOME links the checker's folder, and `prepared/` holds
        copies of the plan and the program, as in the owner's layout.
        """
        lib = os.path.join(self.home, ".local/lib/machine-config")
        if not os.path.lexists(lib):
            os.makedirs(os.path.dirname(lib), exist_ok=True)
            os.symlink(self.r.machine_config, lib)
        folder = os.path.join(self.home, STATE, "prepared")
        if prepared and not os.path.isdir(folder):
            os.makedirs(folder)
            shutil.copyfile(self.r.plan, os.path.join(folder, "plan.json"))
            shutil.copyfile(self.r.program, os.path.join(folder, "owner_switch.py"))
        env = dict(clean_env(), HOME=self.home)
        result = subprocess.run([self.r.mac_check, "--only", "Owner skill switch"], env=env,
                                capture_output=True, text=True, timeout=120)
        with open(os.path.join(self.root, "runs.log"), "a") as f:
            f.write(f"$ mac-check (exit {result.returncode})\n{result.stdout}{result.stderr}\n")
        row = re.search(r"^\| Owner skill switch \| ([^|]+) \| ([^|]+) \|$", result.stdout, re.M)
        expect(row is not None, f"mac-check printed no row for the switch: {result.stdout}{result.stderr}")
        return result.returncode, row.group(1).strip(), row.group(2).strip()

    def manual(self) -> str:
        with open(os.path.join(self.home, MANUAL)) as f:
            return f.read()

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
        """Every entry of the HOME, except the run logs and the owner's prepared copies.

        A stopped run writes its run log, and `mac-check` reads the prepared
        copies. Neither is part of the layout that a stop leaves unchanged.
        """
        out = {}
        skip = (os.path.join(STATE, "runs"), os.path.join(STATE, "prepared"), ".local/lib")
        for folder, dirs, files in os.walk(self.home):
            for name in sorted(dirs + files):
                p = os.path.join(folder, name)
                rel = os.path.relpath(p, self.home)
                if any(rel == s or rel.startswith(s + "/") for s in skip):
                    continue
                if os.path.isdir(p) and not os.path.islink(p) and any(s.startswith(rel + "/") for s in skip):
                    # A folder above a skipped path, which a run log can create.
                    continue
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
    def __init__(self, out: str, program: str, mac_check: str = ""):
        self.mac_check = os.path.realpath(mac_check or MAC_CHECK)
        self.machine_config = os.path.join(os.path.dirname(os.path.dirname(self.mac_check)), "lib/machine-config")
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
        # Clones of this interpreter under the names and paths of FIXTURE_NAMES.
        # The process controls run them as other processes, and the program
        # recognises only executables inside this folder (SCOPE_VAR).
        self.fixture_bin = os.path.join(self.out, "fixture-bin")
        os.makedirs(self.fixture_bin, exist_ok=True)
        for name in FIXTURE_NAMES:
            copy = os.path.join(self.fixture_bin, name)
            if not os.path.exists(copy):
                os.makedirs(os.path.dirname(copy), exist_ok=True)
                # An APFS clone shares the interpreter's blocks; a full copy is the fallback.
                if subprocess.run(["/bin/cp", "-c", os.path.realpath(PY), copy]).returncode != 0:
                    shutil.copy2(os.path.realpath(PY), copy)
        for rel in FIXTURE_SCRIPTS:
            write(os.path.join(self.fixture_bin, rel), SLEEP + "\n")
        # The bin link of an npm installation, which Node runs as its script.
        link = os.path.join(self.fixture_bin, "npm-bin", "codex")
        if not os.path.lexists(link):
            os.makedirs(os.path.dirname(link), exist_ok=True)
            os.symlink("../lib/node_modules/@openai/codex/bin/codex.js", link)
        self.results = []
        self._module = None

    def module(self):
        """The program under rehearsal as a module, loaded once."""
        if self._module is None:
            self._module = program_module(self.program)
        return self._module

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


def program_module(path: str):
    """The program at PATH as a module, for a control that calls one of its functions directly."""
    spec = importlib.util.spec_from_file_location("owner_switch_under_test", path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


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


def c_outside_platform_subtree(case):
    """A probe's change inside each platform subtree passes. outside-entry-inside-run fails a sibling."""
    case.set_control({"actions": [
        {"point": "after-record:receipt", "do": "create", "path": ".claude/skills/synced/account-1/docs"},
        {"point": "after-record:receipt", "do": "create", "path": ".codex/skills/.system/imagegen"}]})
    run_to_complete(case)


def c_outside_platform_link(case):
    """A link in place of a platform subtree is compared like any other child."""
    case.set_control({"actions": [{"point": "step-14", "do": "crash"}]})
    expect_exit(case.run(), 99)
    os.makedirs(os.path.join(case.home, ".elsewhere/account-1/grove"))
    os.symlink("../../.elsewhere", os.path.join(case.home, ".claude/skills/synced"))
    expect_exit(case.run(), 3, "changed outside the switch set: .claude/skills/synced")
    expect(case.receipts() == 0, "the complete continuation copied a receipt with a link in place of synced")


def c_cell_after_receipt_no_copy(case):
    case.set_control({"actions": [{"point": "step-14", "do": "crash"}]})
    expect_exit(case.run(), 99)
    write_in_place(os.path.join(case.home, SETTINGS), SUPERSEDED_PLUGIN, True)
    expect_exit(case.run(), 3, "drifted")
    expect(case.receipts() == 0, "a run copied a receipt for a layout that is not switched")


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


# --- the narrow process inspection ---------------------------------------------
# docs/specs/owner-skill-delivery.md, "The narrow process inspection" and "Test
# seams". The fixture executables are clones of this interpreter in OUT/fixture-bin.

# A launcher that runs its arguments as its child, so it becomes an ancestor.
RELAY = "import subprocess, sys; sys.exit(subprocess.run(sys.argv[1:]).returncode)"


def fixture(case, rel: str) -> str:
    return os.path.join(case.r.fixture_bin, rel)


def alive(pid: int) -> bool:
    """Whether PID runs and is no zombie."""
    stat = subprocess.run(["/bin/ps", "-o", "stat=", "-p", str(pid)], capture_output=True, text=True).stdout.strip()
    return bool(stat) and "Z" not in stat


def start_fixture(case, argv: list, executable=None, reparent: bool = False) -> int:
    """Start ARGV as another process outside the session and return its PID.

    EXECUTABLE runs in place of argv[0], so argv[0] can be any title. With
    REPARENT, an intermediate process starts it and exits, so its parent
    becomes launchd, as for a daemon that outlives its launcher.
    """
    # A probe-owned service runs with the probe's HOME, which its record names.
    env = {"PYTHONHOME": sys.base_prefix, "HOME": case.home}
    if reparent:
        code = ("import json, subprocess, sys; s = json.loads(sys.argv[1]); "
                "p = subprocess.Popen(s['argv'], executable=s['executable'], env=s['env'], start_new_session=True, "
                "stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL); print(p.pid)")
        spec = json.dumps({"argv": argv, "executable": executable, "env": env})
        pid = int(subprocess.run([PY, "-I", "-c", code, spec], capture_output=True, text=True, check=True).stdout)
    else:
        pid = subprocess.Popen(argv, executable=executable, env=env, start_new_session=True, stdin=subprocess.DEVNULL,
                               stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL).pid
    record_pid(case.root, pid)
    time.sleep(0.3)
    expect(alive(pid), f"the fixture {argv[:3]} ended at once")
    return pid


def sleeper(case, rel: str, *args: str, title=None, reparent=False) -> int:
    """Start the fixture executable REL, sleeping, with ARGS after its code."""
    path = fixture(case, rel)
    return start_fixture(case, [title or path, "-c", SLEEP, *args], executable=path, reparent=reparent)


def node(case, script: str, *args: str) -> int:
    """Start the fixture `node` with SCRIPT as its entry script."""
    return start_fixture(case, [fixture(case, "node"), script, *args])


def line_of(out: str, pid: int) -> str:
    found = re.search(rf"^\s*(admitted \S+|remaining) process {pid} .*$", out, re.M)
    expect(found is not None, f"the inspection did not report process {pid}: {out[-1500:]}")
    return found.group(0)


def expect_alive(pids: list) -> None:
    expect(all(alive(p) for p in pids), "the program signalled a fixture process")


def expect_reported(out: str, pid: int, *texts: str) -> str:
    line = line_of(out, pid)
    for text in texts:
        expect(text in line, f"expected {text!r} for process {pid}: {line}")
    return line


def recognition_cases() -> list:
    """Each form: (label, executable, launch path, argv, expected tool or "review" or None, text)."""
    u = "/Users/owner"
    claude = f"{u}/.local/share/claude/versions/2.1.319"
    daemon = f"{u}/.codex/packages/app-server-daemon/releases/0.167.0-aarch64-apple-darwin/bin/codex"
    node_exe = "/opt/homebrew/Cellar/node/26.13.0_2/bin/node"
    pkg = "/opt/homebrew/lib/node_modules"
    return [
        ("claude-versioned", claude, f"{u}/.local/bin/claude", ["claude", "--resume"], "Claude Code", "versioned"),
        ("claude-as-ugrep", claude, claude, ["ugrep", "-G", "--hidden"], "Claude Code", "as ugrep"),
        ("codex-cask", "/opt/homebrew/Caskroom/codex/0.167.0/bin/codex", "/opt/homebrew/bin/codex",
         ["codex", "exec", "--json", "hello"], "Codex", "native"),
        ("codex-daemon", daemon, daemon, [daemon, "app-server", "daemon", "pid-update-loop"], "Codex",
         "app-server daemon"),
        ("codex-managed-app-server", daemon, daemon,
         [daemon, "app-server", "--listen", "unix://", "--managed-daemon"], "Codex", "managed app-server"),
        ("codex-npm-vendor", f"{pkg}/@openai/codex/vendor/aarch64-apple-darwin/codex/codex", None, ["codex"],
         "Codex", "native"),
        ("grove-cellar", "/opt/homebrew/Cellar/grove/22.8.0/bin/grove", "/opt/homebrew/bin/grove", ["grove"],
         "Grove", "native"),
        ("grove-llm", "/opt/homebrew/Cellar/grove/22.8.0/bin/grove-llm", "/opt/homebrew/bin/grove-llm",
         ["grove-llm", "pick"], "Grove", "native"),
        ("grove-debug-build", "/private/tmp/x/target/debug/grove", "/private/tmp/x/target/debug/grove",
         ["/private/tmp/x/target/debug/grove"], "Grove", "native"),
        ("ravel-cellar", "/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel", "/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel",
         ["/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel", "start-session", "--kind=impl"], "Ravel", "native"),
        ("node-npm-bin-link", node_exe, "/opt/homebrew/bin/node", ["node", "/opt/homebrew/bin/codex", "exec"],
         "Codex", f"Node launcher of {pkg}/@openai/codex/bin/codex.js"),
        ("node-claude-package", node_exe, "/opt/homebrew/bin/node",
         ["node", "--no-warnings", f"{pkg}/@anthropic-ai/claude-code/cli.js"], "Claude Code", "Node launcher"),
        ("node-claude-title", node_exe, "/opt/homebrew/bin/node", ["claude"], "Claude Code", "titled claude"),
        ("node-option-value-skipped", node_exe, "/opt/homebrew/bin/node",
         ["node", "--require", "/srv/claude.js", "/srv/other.js"], None, ""),
        ("node-playwright-title", node_exe, "/opt/homebrew/bin/node", ["npm exec @playwright/mcp@latest", "", ""],
         None, ""),
        ("node-quint-server", node_exe, "/opt/homebrew/bin/node",
         ["/opt/homebrew/bin/node", f"{u}/.quint-llm-kit/repo/mcp-servers/kb/dist/server.js"], None, ""),
        ("node-pi", node_exe, "/opt/homebrew/bin/node",
         ["node", f"{pkg}/@mariozechner/pi-coding-agent/dist/cli.js"], None, ""),
        ("node-grove-script", node_exe, "/opt/homebrew/bin/node", ["node", "/srv/grove.js"], "review", "grove"),
        ("argument-task", "/opt/homebrew/bin/task", "/opt/homebrew/bin/task",
         ["task", "serve", f"DIR={u}/Development/AgenticOrchestration/grove"], None, ""),
        ("argument-http-server", "/opt/homebrew/bin/python3.15", "/opt/homebrew/bin/python3",
         ["python3", "-m", "http.server", "--directory", f"{u}/Ravel.skill-directory-delivery/docs"], None, ""),
        ("argument-rg", "/opt/homebrew/bin/rg", "/opt/homebrew/bin/rg", ["rg", "claude", f"{u}/.codex"], None, ""),
        ("argument-shell-command", "/bin/bash", "/bin/sh", ["/bin/sh", "-c", "codex exec hello"], None, ""),
        ("desktop-claude", "/Applications/Claude.app/Contents/MacOS/Claude",
         "/Applications/Claude.app/Contents/MacOS/Claude", ["/Applications/Claude.app/Contents/MacOS/Claude"],
         None, ""),
        ("desktop-chatgpt", "/Applications/ChatGPT.app/Contents/MacOS/ChatGPT",
         "/Applications/ChatGPT.app/Contents/MacOS/ChatGPT", ["/Applications/ChatGPT.app/Contents/MacOS/ChatGPT"],
         None, ""),
        ("caffeinate", "/usr/bin/caffeinate", "caffeinate", ["caffeinate", "-i", "-t", "300"], None, ""),
        ("versions-without-claude", "/srv/versions/2.1.319", "/srv/versions/2.1.319", ["x"], None, ""),
        ("claude-prerelease-as-ugrep", f"{u}/.local/share/claude/versions/2.2.0-rc.1", None, ["ugrep"],
         "Claude Code", "versioned"),
        ("codex-exec-server", "/opt/homebrew/Caskroom/codex/0.167.0/bin/codex-exec-server", None,
         ["codex-exec-server"], "Codex", "helper executable"),
        ("codex-code-mode-host", daemon + "-code-mode-host", None, [daemon + "-code-mode-host"], "Codex",
         "helper executable"),
        ("node-title-ending-in-grove", node_exe, "/opt/homebrew/bin/node",
         [f"npm exec @modelcontextprotocol/server-filesystem {u}/Development/grove"], None, ""),
        ("node-title-ending-in-codex", node_exe, "/opt/homebrew/bin/node", [f"npm exec x {u}/src/codex"], None, ""),
        ("node-env-file-if-exists", node_exe, "/opt/homebrew/bin/node",
         ["node", "--env-file-if-exists", ".env", f"{pkg}/@anthropic-ai/claude-code/cli.js"], "Claude Code",
         "Node launcher"),
        ("node-relative-script", node_exe, "/opt/homebrew/bin/node", ["node", "cli.js"], "Claude Code",
         "@anthropic-ai/claude-code/cli.js"),
        ("bun-claude-package", "/opt/homebrew/bin/bun", "/opt/homebrew/bin/bun",
         ["bun", f"{pkg}/@anthropic-ai/claude-code/cli.js"], "Claude Code", "Node launcher"),
        ("shell-rcfile-value", "/bin/bash", "/bin/bash", ["bash", "--rcfile", f"{u}/grove.sh", "x"], None, ""),
        ("python-option-value", "/opt/homebrew/bin/python3.15", "/opt/homebrew/bin/python3",
         ["python3", "-X", "dev", f"{u}/bin/ravel.py"], "review", "ravel"),
        ("title-without-tool", "/bin/sleep", "/bin/sleep", ["claude", "120"], "review", "claude"),
        ("shell-wrapper", "/bin/bash", "/bin/sh", ["/bin/sh", f"{u}/bin/codex", "exec"], "review", "codex"),
        ("python-wrapper", "/opt/homebrew/bin/python3.15", "/opt/homebrew/bin/python3",
         ["python3", "-u", f"{u}/bin/ravel.py"], "review", "ravel"),
        ("unreadable", None, None, None, "review", "unreadable"),
        ("deleted-codex-daemon", None, daemon, [daemon, "app-server", "daemon"], "Codex",
         "executable unreadable, by its launch path"),
        ("deleted-watchman", None, "/opt/homebrew/bin/watchman", ["watchman", "--foreground"], "review",
         "the executable is unreadable"),
    ]


def c_recognition_forms(case):
    """Each measured and constructed launch form, through the program's own recognise()."""
    import importlib.util
    spec = importlib.util.spec_from_file_location("owner_switch_under_test", case.r.program)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    links = {"/opt/homebrew/bin/codex": "/opt/homebrew/lib/node_modules/@openai/codex/bin/codex.js",
             # A relative entry script resolves against the process's own folder.
             "cli.js": "/opt/homebrew/lib/node_modules/@anthropic-ai/claude-code/cli.js"}
    failed = []
    for label, exe, launch, argv, want, text in recognition_cases():
        got = module.recognise(exe, launch, argv, resolve=lambda p: links.get(p, p))
        if want is None:
            ok = got is None
        elif want == "review":
            ok = got is not None and "review" in got and text in got["review"]
        else:
            ok = got is not None and got.get("tool") == want and text in f"{got['form']}, {got['role']}"
        if not ok:
            failed.append(f"{label}: {got}")
    expect(not failed, "; ".join(failed))
    return f"{len(recognition_cases())} forms"


def c_processes_native(case):
    pids = {"Claude Code": sleeper(case, "share/claude/versions/2.1.319", title="claude"),
            "Codex": sleeper(case, "codex"), "Grove": sleeper(case, "grove"),
            "Ravel": sleeper(case, "ravel")}
    llm = sleeper(case, "grove-llm")
    out = expect_exit(case.processes(), 3, "result: refused")
    for tool, pid in pids.items():
        expect_reported(out, pid, "remaining", tool)
    expect_reported(out, pids["Claude Code"], "versioned executable")
    expect_reported(out, llm, "remaining", "Grove")
    expect_alive(list(pids.values()) + [llm])
    return "Claude Code by its versioned path, Codex, Grove, grove-llm and Ravel by basename"


def c_processes_node(case):
    pids = [node(case, fixture(case, "lib/node_modules/@openai/codex/bin/codex.js"), "exec"),
            node(case, fixture(case, "lib/node_modules/@anthropic-ai/claude-code/cli.js")),
            node(case, fixture(case, "npm-bin/codex")),
            sleeper(case, "node", title="claude")]
    out = expect_exit(case.processes(), 3, "result: refused")
    expect_reported(out, pids[0], "Codex", "Node launcher of")
    expect_reported(out, pids[1], "Claude Code", "@anthropic-ai/claude-code/cli.js")
    expect_reported(out, pids[2], "Codex", "@openai/codex/bin/codex.js")
    expect_reported(out, pids[3], "Claude Code", "Node launcher titled claude")
    expect_alive(pids)
    return "package entry scripts, an npm bin link and a Node title"


def c_processes_codex_daemon(case):
    loop = sleeper(case, "codex", "app-server", "daemon", "pid-update-loop", reparent=True)
    server = sleeper(case, "codex", "app-server", "--listen", "unix://", "--managed-daemon", reparent=True)
    out = expect_exit(case.processes(), 3, "result: refused")
    expect_reported(out, loop, "remaining", "(parent 1)", "app-server daemon")
    expect_reported(out, server, "remaining", "managed app-server")
    expect_alive([loop, server])
    return "a reparented daemon loop and a managed app-server refuse without a signal"


def c_processes_unrelated(case):
    pids = [sleeper(case, "helper", "claude", "codex", "grove", "ravel", fixture(case, "codex")),
            node(case, fixture(case, "lib/other-server.js"), "--name", "codex"),
            sleeper(case, "caffeinate", "-i", "-t", "300"),
            start_fixture(case, [fixture(case, "python3"), fixture(case, "lib/other-server.js"), "claude"])]
    out = expect_exit(case.processes(), 0, "result: clear")
    for pid in pids:
        expect(not re.search(rf"process {pid} ", out), f"process {pid} was recognised: {out[-1500:]}")
    return "tool names in other arguments, a Node server, caffeinate and a Python script"


def c_processes_review(case):
    pids = [sleeper(case, "helper", title="claude"),
            start_fixture(case, [fixture(case, "python3"), fixture(case, "wrappers/codex")]),
            node(case, fixture(case, "lib/grove.js"))]
    out = expect_exit(case.processes(), 3, "result: refused")
    for pid, name in zip(pids, ("claude", "codex", "grove")):
        expect_reported(out, pid, "remaining", "review:", f"names {name}")
    expect_alive(pids)
    return "a tool title, a wrapper script and a Node script named grove need review"


def c_processes_deleted(case):
    folder = fixture(case, f"deleted-{case.name}")
    os.makedirs(folder, exist_ok=True)
    copy = os.path.join(folder, "codex")
    subprocess.run(["/bin/cp", "-c", fixture(case, "codex"), copy], check=True)
    pid = start_fixture(case, [copy, "-c", SLEEP, "app-server", "daemon"])
    os.remove(copy)
    out = expect_exit(case.processes(), 3, "result: refused")
    expect_reported(out, pid, "Codex", "executable unreadable, by its launch path")
    return "a deleted Codex executable is recognised by its launch path"


def c_processes_deleted_other(case):
    folder = fixture(case, f"deleted-{case.name}")
    os.makedirs(folder, exist_ok=True)
    copy = os.path.join(folder, "watchman")
    subprocess.run(["/bin/cp", "-c", fixture(case, "helper"), copy], check=True)
    pid = start_fixture(case, [copy, "-c", SLEEP])
    os.remove(copy)
    out = expect_exit(case.processes(), 3, "result: refused")
    expect_reported(out, pid, "review:", "the executable is unreadable")
    return "a deleted executable that names no tool needs review"


def c_processes_relative_script(case):
    """A relative entry script resolves against the process's own folder."""
    package = fixture(case, "lib/node_modules/@anthropic-ai/claude-code")
    pid = subprocess.Popen([fixture(case, "node"), "cli.js"], cwd=package, env={"PYTHONHOME": sys.base_prefix},
                           start_new_session=True, stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL,
                           stderr=subprocess.DEVNULL).pid
    record_pid(case.root, pid)
    time.sleep(0.3)
    out = expect_exit(case.processes(), 3, "result: refused")
    expect_reported(out, pid, "Claude Code", "@anthropic-ai/claude-code/cli.js")
    return "node cli.js in the package folder is Claude Code"


def relay(case, rel: str, prelude: str = "") -> list:
    """A launcher of the fixture REL that runs its arguments as its child, after PRELUDE."""
    return [fixture(case, rel), "-c", prelude + RELAY]


def child(case, rel: str, *args: str) -> str:
    """Python code that starts the fixture REL, sleeping, as a child, and records its PID."""
    argv = [fixture(case, rel), "-c", SLEEP, *args]
    # The child must not hold the captured output open after its launcher ends.
    return (f"import subprocess; _c = subprocess.Popen({argv!r}, stdin=subprocess.DEVNULL, "
            "stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL); "
            f"open({os.path.join(case.root, 'pids')!r}, 'a').write(f'{{_c.pid}}\\n'); ")


def c_processes_ancestors(case):
    via = relay(case, "ravel") + relay(case, "share/claude/versions/2.1.319")
    out = expect_exit(case.processes(via=via), 0, "result: clear")
    expect(re.search(r"^admitted ancestor process \d+ .*Ravel", out, re.M) is not None, f"no Ravel ancestor: {out}")
    expect(re.search(r"^admitted ancestor process \d+ .*Claude Code \(versioned", out, re.M) is not None,
           f"no Claude Code ancestor: {out}")
    return "a Ravel driver and a Claude Code harness ancestor are admitted"


def c_processes_descendant(case):
    via = relay(case, "ravel") + relay(case, "share/claude/versions/2.1.319",
                                       child(case, "codex") + child(case, "caffeinate", "-i", "-t", "300"))
    out = expect_exit(case.processes(via=via), 3, "result: refused")
    expect(re.search(r"^remaining process \d+ \(parent \d+\) Codex", out, re.M) is not None,
           f"the descendant harness was not refused: {out}")
    expect("caffeinate" not in out, f"the caffeinate child was reported: {out}")
    case.kill()
    time.sleep(0.3)
    expect_exit(case.processes(via=relay(case, "share/claude/versions/2.1.319",
                                               child(case, "caffeinate", "-i", "-t", "300"))), 0, "result: clear")
    return "a descendant Codex of the harness refuses, and a caffeinate child needs no exemption"


def c_processes_outer_harness(case):
    """A harness ancestor above the operator's harness is another session."""
    via = relay(case, "codex") + relay(case, "ravel") + relay(case, "share/claude/versions/2.1.319")
    out = expect_exit(case.processes(via=via), 3, "result: refused")
    expect(re.search(r"^remaining process \d+ \(parent \d+\) outer harness ancestor, Codex", out, re.M)
           is not None, f"the outer Codex was not refused: {out}")
    expect(re.search(r"^admitted ancestor process \d+ .*Claude Code", out, re.M) is not None,
           f"the operator's harness was not admitted: {out}")
    expect(re.search(r"^admitted ancestor process \d+ .*Ravel", out, re.M) is not None,
           f"the driver was not admitted: {out}")
    return "an outer Codex refuses; the inner harness and its driver are admitted"


def c_processes_probe_owned(case):
    owned = sleeper(case, "codex", "app-server", "daemon", "pid-update-loop", reparent=True)
    other = sleeper(case, "codex", "app-server", "daemon", "pid-update-loop", reparent=True)
    code, out = case.processes("--json")
    report = json.loads(out)
    item = next(i for i in report["other"] if i["pid"] == owned)
    record = {"pid": owned, "start": item["start"], "executable": item["executable"], "home": item["home"]}
    expect(record["home"] == case.home, f"the record does not name the fixture's HOME: {item}")
    expect(re.fullmatch(r"\d+\.\d{6}", record["start"] or "") is not None, f"no creation time: {item}")

    def run_with(records) -> tuple:
        path = os.path.join(case.root, "owned.json")
        save_json(path, records)
        return case.processes("--owned", path)

    out = expect_exit(run_with([record]), 3, "result: refused")
    expect_reported(out, owned, "admitted probe-owned")
    expect_reported(out, other, "remaining")
    os.kill(other, signal.SIGKILL)
    time.sleep(0.3)
    expect_exit(run_with([record]), 0, "result: clear")
    sec, usec = record["start"].split(".")
    reused = dict(record, start=f"{sec}.{(int(usec) + 1) % 1000000:06d}")
    expect_reported(expect_exit(run_with([reused]), 3, "result: refused"), owned, "remaining")
    moved = dict(record, executable=fixture(case, "grove"))
    expect_reported(expect_exit(run_with([moved]), 3, "result: refused"), owned, "remaining")
    elsewhere = dict(record, home=case.root)
    expect_reported(expect_exit(run_with([elsewhere]), 3, "result: refused"), owned, "remaining")
    expect_exit(run_with([{k: v for k, v in record.items() if k != "home"}]), 3,
                "is not a list of pid, start, executable and home")
    expect_alive([owned])
    return "only an exact PID, creation time, executable and HOME admit a probe-owned process"


def c_tool_before_run(case):
    pid = sleeper(case, "codex", "app-server", "daemon", "pid-update-loop", reparent=True)
    before = case.snapshot()
    out = expect_exit(case.run(), 3, "remains before step 1")
    expect_reported(out, pid, "remaining", "Codex", "app-server daemon")
    expect(case.snapshot() == before, "a refused step 1 changed the layout")
    expect_alive([pid])


def session_tool(spec: dict, text: str):
    """A process of the fixture harness's session stops step 1 when it is one of the tools."""
    def fn(case):
        case.set_control({"session": [spec]})
        time.sleep(0.3)
        expect_exit(case.run(), 3, text)
        expect(case.state()["backup"] is None, "a session tool did not stop step 1")
    return fn


def session_allowed(argv):
    """A process of the session that is none of the tools needs no exemption."""
    def fn(case):
        case.set_control({"session": [[fixture(case, a) if a in FIXTURE_NAMES else a for a in argv]]})
        time.sleep(0.3)
        run_to_complete(case)
    return fn


def c_mcp_server_tool_child(case):
    """No descendant exemption: a tool under an MCP server of the session stops step 1."""
    server = fixture(case, "lib/server-with-child.js")
    ready = os.path.join(case.root, "server-ready")
    write(server, child(case, "codex") + f"open({ready!r}, 'w').close(); " + SLEEP + "\n")
    case.set_control({"session": [[fixture(case, "node"), server]]})
    # The harness starts the server beside the program, so the program waits for its child.
    wait = (f"import os, subprocess, sys, time\nfor _ in range(200):\n    if os.path.exists({ready!r}): break\n"
            "    time.sleep(0.05)\nsys.exit(subprocess.run(sys.argv[1:]).returncode)")
    out = expect_exit(case.run(via=[PY, "-I", "-c", wait]), 3, "remains before step 1")
    expect(re.search(r"remaining process \d+ \(parent \d+\) Codex", out) is not None, f"no Codex child: {out}")


def c_finish_ancestors(case):
    """The switch admits its exact Ravel driver and Claude Code harness ancestors."""
    via = relay(case, "ravel") + relay(case, "share/claude/versions/2.1.319")
    expect_exit(case.run(via=via), 0, "result: complete")
    out = case.outputs[-1]
    expect(re.search(r"admitted ancestor process \d+ .*Ravel", out) is not None, "no admitted Ravel ancestor")


def c_plan_process_patterns(case):
    """The plan holds no process, MCP or job pattern."""
    old = {"session": {"harness": ["claude"], "mcp": []}, "watch": ["codex"],
           "jobs": {"argv": ["/bin/launchctl", "list"], "patterns": ["mise"]}}
    plans = {name: plan_copy(case, f"plan-{name}.json", lambda p, n=name: p.__setitem__(n, old[n]))
             for name in old}
    before = case.snapshot()
    for name, other in plans.items():
        expect_exit(case.run(plan=other), 3, f"the plan holds {name}: the narrow process inspection is built in")
    expect(case.snapshot() == before, "a plan with patterns changed the layout")
    return ", ".join(plans)


def c_native_off_other_tool(case):
    """native-off refuses only another Ravel driver."""
    sleeper(case, "codex")
    sleeper(case, "share/claude/versions/2.1.319", title="claude")
    before = settings_bytes(case)
    expect_exit(case.native_off(), 0, "result: off value in place")
    expect_off(case, before)
    return "Codex and Claude Code processes do not stop native-off"


def c_native_off_ravel_title(case):
    """A process titled ravel without a Ravel executable needs review, so native-off stops."""
    sleeper(case, "helper", title="ravel")
    before = case.snapshot()
    expect_exit(case.native_off(), 3, "another Ravel driver runs")
    expect(case.snapshot() == before, "native-off wrote beside a process titled ravel")


def process_controls(r) -> list:
    codex_child = {"argv": [os.path.join(r.fixture_bin, "codex"), "-c", SLEEP], "executable": None}
    claude_child = {"argv": ["claude", "-c", SLEEP],
                    "executable": os.path.join(r.fixture_bin, "share/claude/versions/2.1.319")}
    return [
        ("recognition-forms", c_recognition_forms, {}),
        ("processes-native", c_processes_native, {}),
        ("processes-node", c_processes_node, {}),
        ("processes-codex-daemon", c_processes_codex_daemon, {}),
        ("processes-unrelated", c_processes_unrelated, {}),
        ("processes-review", c_processes_review, {}),
        ("processes-deleted-executable", c_processes_deleted, {}),
        ("processes-deleted-other", c_processes_deleted_other, {}),
        ("processes-relative-script", c_processes_relative_script, {}),
        ("processes-outer-harness", c_processes_outer_harness, {}),
        ("processes-ancestors", c_processes_ancestors, {}),
        ("processes-descendant-harness", c_processes_descendant, {}),
        ("processes-probe-owned", c_processes_probe_owned, {}),
        ("tool-before-run", c_tool_before_run, {}),
        ("tool-later", interval_closed("spawn-tool", [os.path.join(r.fixture_bin, "grove"), "-c", SLEEP]), {}),
        ("descendant-harness-before-run", session_tool(claude_child, r"remaining process \d+ .*Claude Code"), {}),
        ("descendant-harness-later", interval_closed("spawn-in-session", codex_child), {}),
        ("background-child-allowed", session_allowed(["/bin/sleep", "120"]), {}),
        ("caffeinate-child-allowed", session_allowed(["caffeinate", "-c", SLEEP]), {}),
        ("mcp-server-allowed", session_allowed(["node", os.path.join(r.fixture_bin, "lib/other-server.js")]), {}),
        ("mcp-server-tool-child", c_mcp_server_tool_child, {}),
        ("finish-ancestors-allowed", c_finish_ancestors, {}),
        ("plan-process-patterns", c_plan_process_patterns, {}),
    ]


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
        case.set_control({"actions": [{"point": point, "do": "spawn-tool",
                                       "argv": [fixture(case, "codex"), "-c", SLEEP]}]})
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
    expect(sorted(body.get("gates", {})) == ["4", "5", "6", "7"],
           f"the receipt lacks the gates: {sorted(body.get('gates', {}))}")
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


# --- the controls of owner-switch-program-k243 ------------------------------------

def plan_copy(case, name: str, mutate=None, record: bool = True) -> str:
    """A copy of the fixture plan, changed by mutate, with a passing record when record is set."""
    plan = load(case.r.plan, {})
    if mutate:
        mutate(plan)
    other = os.path.join(case.root, name)
    save(other, plan)
    if record:
        case.rehearse_record(digest_file(other), case.r.program_digest)
    return other


def receipt_of(case) -> dict:
    run = os.path.join(case.state()["backup"], "run")
    return load(os.path.join(run, next(n for n in os.listdir(run) if n.endswith("-receipt.json"))), {})


def c_plan_changed_after_load(case):
    """F1: a gate reads the plan again by its path, after the program admitted its bytes."""
    copy = os.path.join(case.root, "plan.json")
    shutil.copyfile(case.r.plan, copy)
    one_byte(case, "superpowers")
    changed = source_digest(keg_source(case, "superpowers"))
    case.set_control({"actions": [{"point": "command:grove-release", "do": "write-key", "path": copy,
                                   "key": ["accepted", "sources", "superpowers", "resourceDigest"],
                                   "value": changed}]})
    expect_gate_stop(case, "is not the plan that the program admitted", plan=copy)


def c_plan_paths_escape(case):
    """F4: a plan path that is absolute, holds .. or crosses a linked folder stops before any step."""
    outside = os.path.join(case.root, "outside")
    write(os.path.join(outside, "grove/SKILL.md"), "---\nname: grove\n---\nOutside HOME.\n")
    os.symlink(outside, os.path.join(case.home, ".claude/linked"))
    ident = subprocess.run([PY, "-I", case.r.program, "identity", os.path.join(outside, "grove")],
                           capture_output=True, text=True, check=True).stdout.strip()
    variants = {"absolute": os.path.join(outside, "grove"), "dotdot": "../outside/grove",
                "linked": ".claude/linked/grove"}
    for label, rel in variants.items():
        def mutate(plan, rel=rel):
            plan["parts"][0] = {"kind": "entry", "path": rel, "identity": ident}
        other = plan_copy(case, f"plan-{label}.json", mutate)
        expect_exit(case.run(plan=other), 3, "is not relative to HOME|without \\.\\.|leads outside HOME")
        expect(os.path.isfile(os.path.join(outside, "grove/SKILL.md")), f"the {label} path moved an entry outside HOME")
        expect(case.state()["backup"] is None, f"the {label} path wrote a backup")
    return ", ".join(variants)


def c_plan_incomplete(case):
    """F5: a rehearsed plan without a check that a step needs stops before any step."""
    def gate_4_other(plan):
        plan["gates"]["4"] = [{"name": "grove-release", "argv": fixture_command("probe-native")}]
    mutations = {
        "gates-empty": lambda p: p.__setitem__("gates", {}),
        "gate-4-other": gate_4_other,
        "gate-7-empty": lambda p: p["gates"].__setitem__("7", []),
        "probes-empty": lambda p: p.__setitem__("probes", []),
        "native-probe-missing": lambda p: p.pop("native_probe"),
        "mac-check-missing": lambda p: p.pop("mac_check"),
        "baseline-missing": lambda p: p.pop("baseline"),
        "outside-empty": lambda p: p.__setitem__("outside", []),
        "staging-missing": lambda p: p["accepted"].pop("staging"),
    }
    plans = {label: plan_copy(case, f"plan-{label}.json", mutate) for label, mutate in mutations.items()}
    before = case.snapshot()
    for label, other in plans.items():
        expect_exit(case.run(plan=other), 3, "the plan is incomplete")
    expect(case.snapshot() == before, "an incomplete plan changed the layout")
    expect(case.points() == [], f"an incomplete plan reached a hook point: {case.points()[:5]}")
    return ", ".join(mutations)


def c_staging_missing(case):
    """F6: step 4 stops when the retained staging HOME is missing."""
    one_byte(case, "design")

    def mutate(plan):
        plan["accepted"]["staging"]["home"] = os.path.join(case.root, "no-staging-home")
    other = plan_copy(case, "plan-no-staging.json", mutate, record=False)
    expect_gate_stop(case, "the retained staging HOME .* is missing", plan=other)
    expect(trials(case) == [], f"step 4 ran trials {trials(case)} without the staging HOME")


def c_reader_orphaned_in_probes(case):
    """F7: an orphan marker after step 7 counts as a failed probe."""
    marker = os.path.join(READERS["grove@linkuistics"], ".orphaned_at")
    case.set_control({"actions": [{"point": "command:probe-switched", "do": "touch", "path": marker,
                                   "text": "1792928060164"}]})
    expect_exit(case.run(), 3, "failed probe \\(the reader evidence of step 7 changed: .* holds .orphaned_at")
    st = case.state()
    expect(st["state"] == "native" and st["records"][-1] == "inverse-ended", f"a changed reader left {st['state']}")
    expect(case.receipts() == 0, "a changed reader copied a receipt")


def c_reader_moved_before_continuation(case):
    """F7: an update between an active interruption and its continuation counts as a failed probe."""
    case.set_control({"actions": [{"point": "switch-after-part:3", "do": "crash"}]})
    expect_exit(case.run(), 99)
    registry = os.path.join(case.home, ".claude/plugins/installed_plugins.json")
    body = load(registry, {})
    newer = os.path.join(case.home, ".claude/plugins/cache/linkuistics/grove/ffffffff")
    write(os.path.join(newer, ".claude-plugin/plugin.json"), "{}\n")
    body["plugins"]["grove@linkuistics"][0]["installPath"] = newer
    save_json(registry, body)
    expect_exit(case.run(), 3, "failed probe \\(the reader evidence of step 7 changed: .* lost its registration")
    st = case.state()
    expect(st["state"] == "native" and case.receipts() == 0, f"a moved registration left {st['state']}")


def c_receipt_evidence(case):
    """F8: the receipt holds the activated configuration and each retained byte set."""
    expect_exit(case.run(), 0, "result: complete")
    body = receipt_of(case)
    act = body.get("activated", {})

    def digest(rel):
        return subprocess.run([PY, "-I", case.r.program, "digest", os.path.join(case.home, rel)],
                              capture_output=True, text=True, check=True).stdout.strip()
    expect(act.get("config_rhai_digest") == digest(".config/ravel/config.rhai"), f"config.rhai: {act}")
    expect(act.get("ravel_settings_digest") == digest(RAVEL), "the receipt lacks the Ravel settings digest")
    expect(act.get("ravel") == {"path": os.path.join(case.root, "prefix/bin/ravel"),
                                "version": "ravel 2.0.0 (fixture)"}, f"the receipt names Ravel {act.get('ravel')}")
    expect((act.get("grove") or {}).get("version") == "1", f"the receipt names Grove {act.get('grove')}")
    sources = body.get("sources", {})
    expect(len(sources) == 10, f"the receipt names {len(sources)} sources")
    for name, (version, ids) in KEGS.items():
        for sid in ids:
            got = sources.get(sid, {})
            expect(got.get("version") == version and got.get("keg_path") == keg_source(case, sid)
                   and got.get("resourceDigest") == source_digest(keg_source(case, sid)) and got.get("snapshotRoot"),
                   f"the receipt's source {sid} is {got}")
    expect(body.get("trials") == {"reran": [], "results": []}, f"the receipt's trials are {body.get('trials')}")
    expect(body.get("backup") == case.state()["backup"], "the receipt does not name its backup")
    items = body.get("retained", [])
    kinds = sorted({i["item"] for i in items})
    expect(kinds == sorted(["retained entry", "kept copy", "Grove's Codex snapshot store",
                            "superseded plugin registration", "earlier marketplace registration",
                            "staging HOME", "candidate keg"]), f"the receipt retains {kinds}")
    expect(len([i for i in items if i["item"] == "retained entry"]) == 2, "the receipt lacks a retained entry")
    expect(len([i for i in items if i["item"] == "kept copy"]) == 2, "the receipt lacks a kept copy")
    expect(all(len(i["condition"]) == 5 for i in items if i["item"] not in ("staging HOME", "candidate keg")),
           "a purge item lacks its five conditions")
    expect([i.get("plugin") for i in items if i["item"] == "superseded plugin registration"] == ["grove@linkuistics"],
           "the receipt names another superseded plugin")
    again = body.get("readers", {}).get("again", [])
    expect(len(again) == 2 and all(r["present"] and r["registered"] and not r["orphaned"] for r in again),
           f"the receipt's second reading of the readers is {again}")
    with open(os.path.join(case.home, MANUAL)) as f:
        expect('"config_rhai_digest"' in f.read(), "the manual's receipt lacks the activated configuration")


def c_rehearsal_without_rerun(case):
    """F9: a record without a passing rerun of settings-writes.sh admits no prelude."""
    case.rehearse_record(case.r.plan_digest, case.r.program_digest, settings_writes=False)
    before = case.snapshot()
    expect_exit(case.run(), 3, "so no prelude runs")
    expect(case.snapshot() == before, "a record without the rerun admitted a prelude")
    expect(case.commands(0) == {}, f"a record without the rerun ran {case.commands(0)}")


def c_claude_version_differs(case):
    """F9: the hooks installation runs no Claude Code command with an unmeasured version."""
    case.set_control({"claude_version": "2.2.0 (Claude Code)"})
    expect_exit(case.run(), 3, "no passing rehearsal record measured the writes of that version")
    calls = case.commands(0)
    expect(not any(k.startswith("plugin install") or k.startswith("plugin marketplace add") for k in calls),
           f"an unmeasured version ran {calls}")


def review_controls() -> list:
    no_hooks, no_key = {"hooks": "absent"}, {"ravel": "absent-key"}
    return [
        ("plan-changed-after-load", c_plan_changed_after_load, {}),
        ("plan-paths-escape", c_plan_paths_escape, {}),
        ("plan-incomplete", c_plan_incomplete, {}),
        ("staging-home-missing", c_staging_missing, {}),
        ("reader-orphaned-in-probes", c_reader_orphaned_in_probes, {}),
        ("reader-moved-before-continuation", c_reader_moved_before_continuation, {}),
        ("receipt-evidence", c_receipt_evidence, {}),
        ("rehearsal-without-settings-writes", c_rehearsal_without_rerun, dict(no_key, rehearsed=False, **no_hooks)),
        ("claude-version-differs", c_claude_version_differs, no_hooks),
    ]


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
        ("outside-platform-subtree", c_outside_platform_subtree, {}),
        ("outside-platform-link", c_outside_platform_link, {}),
        ("cell-after-receipt-no-copy", c_cell_after_receipt_no_copy, {}),
        ("prepared-plan-differs", c_prepared_differs, {}),
    ]
    controls += process_controls(r)
    controls += prelude_controls()
    controls += gate_controls()
    controls += review_controls()
    controls += entry_controls()
    controls += native_off_controls()
    controls += serial_controls()
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
        ("native-tool-later", prelude_writer("prelude:native-settings", "the native settings"), no_key),
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
        ("hooks-tool-later", prelude_writer("prelude:hooks-installation", "the hooks installation"), no_hooks),
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


# --- the controls of owner-switch-entries-k92 -------------------------------------
# The owner's inverse, the merge entry, the purge check, the report of
# `mac-check` and the pending-activation record. Each builds on the complete
# fixture plan.

CLAUDE_SKILL_ENTRY, CODEX_SKILL_LINK = ".claude/skills/grove", ".codex/skills/grove"


def expect_reset(case, skip=()) -> None:
    """Each cell outside `skip` holds its earlier value after the owner's inverse."""
    settings = case.json(SETTINGS)["enabledPlugins"]
    if "superseded" not in skip:
        expect(settings["grove@linkuistics"] is True, "the owner's inverse left the superseded plugin disabled")
    if "hooks" not in skip:
        expect(settings[HOOKS_ID] is False, "the owner's inverse left the hooks plugin enabled")
    if "skills" not in skip:
        expect(case.json(RAVEL)["skills"] == OFF, "the owner's inverse left the switched skills value")
    for rel, name in ((CLAUDE_SKILL_ENTRY, "grove"), (CODEX_SKILL_LINK, "codex")):
        if name not in skip:
            expect(os.path.lexists(os.path.join(case.home, rel)), f"the owner's inverse did not move {rel} back")


def owner_inverse_to_changed(case, skip=()) -> str:
    """The owner's inverse leaves a third value and ends in the changed state; the next run stops there."""
    out = expect_exit(case.entry("inverse"), 3, "ended in the changed state")
    st = case.state()
    expect(not st["open"] and st["records"][-1] == "inverse-ended", f"the inverse did not end: {st['records']}")
    expect_reset(case, skip)
    expect("<!-- owner-switch-inverse " in case.manual(), "the owner's inverse added no line to the manual")
    expect_exit(case.run(), 3, "classification")
    expect(case.state()["state"] == "changed", "the next run did not find the changed state")
    return out


def c_owner_inverse_complete(case):
    run_to_complete(case)
    out = expect_exit(case.entry("inverse"), 0, "result: native")
    expect("command probe-native (step inverse): exit 0" in out, "the owner's inverse ran no native probe")
    expect("gate: no other Claude Code, Codex, Grove or Ravel process before the inverse" in out,
           "the owner's inverse ran no process inspection")
    expect_reset(case)
    st = case.state()
    expect(st["state"] == "native" and st["records"][-1] == "inverse-ended", f"unexpected {st['state']}")
    expect(case.manual().count("<!-- owner-switch-inverse ") == 1, "the manual lacks the inverse's line")


def c_owner_inverse_third_cell(when: str):
    def fn(case):
        if when == "after":
            run_to_complete(case)
        else:
            case.set_control({"actions": [{"point": "before-record:receipt", "do": "crash"}]})
            expect_exit(case.run(), 99)
        write_in_place(os.path.join(case.home, RAVEL), ["skills"], {"other": True})
        expect_exit(case.run(), 3, "drifted")
        out = owner_inverse_to_changed(case, skip=("skills",))
        expect('third value: part 3 .config/ravel/settings.json skills: {"other":true}; left' in out,
               "the owner's inverse did not report the third value that it left")
        expect(case.json(RAVEL)["skills"] == {"other": True}, "the owner's inverse changed the third value")
    return fn


def c_owner_inverse_entry(do: str, rel: str):
    def fn(case):
        run_to_complete(case)
        target = os.path.join(case.state()["backup"], "retained", rel)
        if do == "modify-entry":
            with open(os.path.join(target, "SKILL.md"), "a") as f:
                f.write("changed\n")
        else:
            os.remove(target)
            os.symlink("/elsewhere", target)
        expect_exit(case.run(), 3, "drifted")
        out = owner_inverse_to_changed(case)
        expect(f"third value: part {0 if rel == CLAUDE_SKILL_ENTRY else 1} {rel} moved back with a changed identity" in out,
               "the owner's inverse did not move the changed entry back and report it")
        expect(not os.path.lexists(target), "the changed entry stayed in the backup")
    return fn


def c_owner_inverse_occupied(case):
    run_to_complete(case)
    backup = case.state()["backup"]
    os.makedirs(os.path.join(case.home, CODEX_SKILL_LINK))
    expect_exit(case.run(), 3, "drifted")
    out = owner_inverse_to_changed(case, skip=())
    expect(f"third value: part 1 {CODEX_SKILL_LINK} is occupied while its retained entry exists; both are left" in out,
           "the owner's inverse did not report the occupied cell")
    expect(os.path.isdir(os.path.join(case.home, CODEX_SKILL_LINK)) and not os.path.islink(os.path.join(case.home, CODEX_SKILL_LINK)),
           "the owner's inverse changed the occupying entry")
    expect(os.path.islink(os.path.join(backup, "retained", CODEX_SKILL_LINK)), "the retained entry left the backup")


def c_owner_inverse_copy(case):
    case.set_control({"actions": [{"point": "switch-after-part:1", "do": "crash"}]})
    expect_exit(case.run(), 99)
    other = os.path.join(case.root, "program-changed.py")
    with open(case.r.program) as f, open(other, "w") as g:
        g.write(f.read() + "\n# changed\n")
    out = case.entry("inverse", program=other)[1]
    expect("continuation: running the program copy" in out and f"program: {case.r.program_digest}" in out,
           "the owner's inverse did not run the program copy of the latest backup")
    expect(case.state()["records"][-1] == "inverse-ended", "the copy did not end the inverse")


def c_owner_inverse_refusals(case):
    expect_exit(case.entry("inverse"), 3, "no complete backup exists")
    case.set_control({"fail": {"probe-switched": 1}})
    expect_exit(case.run(), 3, "the native probe passed")
    expect_exit(case.entry("inverse"), 3, "is not open")


def c_owner_inverse_writer(case):
    run_to_complete(case)
    sleeper(case, "codex")
    before = case.snapshot()
    expect_exit(case.entry("inverse"), 3, "remains before the inverse")
    expect(case.snapshot() == before and case.state()["state"] == "complete", "a closed interval changed the layout")


def flagged_switch_copy(case, actions: list) -> str:
    """Run the switch with outside writes before the exchange of the Claude Code settings; return the flagged copy."""
    case.set_control({"actions": actions})
    expect_exit(case.run(), 3, "the switch met a conflict")
    flagged = case.state()["flagged"]
    expect(len(flagged) == 1 and flagged[0]["of"] == SETTINGS, f"expected one flagged copy of {SETTINGS}: {flagged}")
    return flagged[0]["copy"]


BEFORE_EXCHANGE = "switch-before-exchange:2"


def c_merge_difference(case):
    copy = flagged_switch_copy(case, [
        {"point": BEFORE_EXCHANGE, "do": "write-key", "path": SETTINGS, "key": ["fault"], "value": 1},
        {"point": BEFORE_EXCHANGE, "do": "erase-key", "path": SETTINGS, "key": ["theme"]}])
    live = case.json(SETTINGS)
    expect("fault" not in live and live.get("theme") == "dark", "the outside write reached the live file")
    out = expect_exit(case.entry("merge", copy), 0, "record: merge 1")
    live = case.json(SETTINGS)
    expect(live.get("fault") == 1, "the merge did not add the key that the copy gained")
    expect("theme" not in live, "the merge did not erase the key that the copy lost")
    expect(live["enabledPlugins"]["grove@linkuistics"] is True, "the merge changed a cell")
    expect("replaced: .claude/settings.json theme: \"dark\" -> (absent)" in out, "the merge listed no erased key")
    expect(not case.state()["flagged"], "the merged copy is still flagged")
    run_to_complete(case)
    expect(case.json(SETTINGS).get("fault") == 1, "the switch lost the merged key")


def c_merge_then_change(case):
    copy = flagged_switch_copy(case, [
        {"point": BEFORE_EXCHANGE, "do": "write-key", "path": SETTINGS, "key": ["fault"], "value": 1}])
    expect_exit(case.entry("merge", copy), 0, "record: merge 1")
    write_in_place(copy, ["late"], 2)
    expect([f["copy"] for f in case.state()["flagged"]] == [copy], "a changed merged copy is not flagged")
    expect_exit(case.run(), 3, "a kept copy is flagged; the owner merges it first")
    out = expect_exit(case.entry("merge", copy), 0, "record: merge 2")
    expect(case.json(SETTINGS).get("late") == 2 and "replaced: .claude/settings.json late" in out,
           "the second merge did not take its difference from the first merge")
    run_to_complete(case)


def c_merge_return(case):
    copy = flagged_switch_copy(case, [
        {"point": BEFORE_EXCHANGE, "do": "write-key", "path": SETTINGS, "key": ["fault"], "value": 1}])
    with open(copy + ".read", "rb") as f:
        read = f.read()
    expect_exit(case.entry("merge", copy), 0, "record: merge 1")
    # A descriptor returns the merged copy to the content that the run read, in place.
    fd = os.open(copy, os.O_WRONLY | os.O_TRUNC)
    try:
        os.write(fd, read)
    finally:
        os.close(fd)
    expect(case.state()["flagged"], "a merged copy that returned to the content that the run read is not flagged")
    expect_exit(case.run(), 3, "a kept copy is flagged; the owner merges it first")
    out = expect_exit(case.entry("merge", copy), 0, "record: merge 2")
    expect("fault" not in case.json(SETTINGS) and "fault: 1 -> (absent)" in out,
           "the merge of the return did not erase the key")


def c_merge_settings_copy(case):
    case.set_control({"actions": [{"point": "native-read", "do": "write-key", "path": RAVEL,
                                   "key": ["fault"], "value": 1}]})
    expect_exit(case.run(), 3, "flagged")
    flagged = case.state()["flagged"]
    expect(len(flagged) == 1 and flagged[0]["backup"] is None, f"expected a flagged copy of a settings record: {flagged}")
    expect_exit(case.entry("merge", flagged[0]["copy"]), 0, "record: merge 1")
    expect(case.json(RAVEL).get("fault") == 1, "the merge did not reach the Ravel settings")
    expect(len([r for r in settings_records(case) if r["label"] == "merge"]) == 1, "the merge wrote no settings record")
    case.set_control({})
    run_to_complete(case)


def c_merge_held_copy(case):
    case.set_control({"actions": [{"point": "stub:install:read", "do": "write-key", "path": SETTINGS,
                                   "key": ["fault"], "value": 1}]})
    expect_exit(case.run(), 3, "flagged")
    flagged = case.state()["flagged"]
    expect(len(flagged) == 1 and flagged[0]["kind"] == "held", f"expected a flagged held copy: {flagged}")
    expect_exit(case.entry("merge", flagged[0]["copy"]), 0, "record: merge 1")
    expect(case.json(SETTINGS).get("fault") == 1, "the merge of the held copy did not reach the live file")
    case.set_control({})
    run_to_complete(case)


def c_merge_third_cell(case):
    copy = flagged_switch_copy(case, [
        {"point": BEFORE_EXCHANGE, "do": "write-key", "path": SETTINGS, "key": SUPERSEDED_PLUGIN, "value": "other"},
        {"point": BEFORE_EXCHANGE, "do": "write-key", "path": SETTINGS, "key": HOOKS_PLUGIN, "value": True}])
    expect_exit(case.entry("merge", copy), 0, "record: merge 1")
    plugins = case.json(SETTINGS)["enabledPlugins"]
    expect(plugins["grove@linkuistics"] == "other", "the merge did not take the copy's third value of a cell")
    expect(plugins[HOOKS_ID] is False, "the merge took a value that a run writes into a cell")
    expect_exit(case.run(), 3, "classification")


def c_merge_record_only(case):
    copy = flagged_switch_copy(case, [
        {"point": BEFORE_EXCHANGE, "do": "write-key", "path": SETTINGS, "key": ["fault"], "value": 1}])
    before = case.json(SETTINGS)
    expect_exit(case.entry("merge", copy, "--record-only"), 0, "without applying a difference")
    expect(case.json(SETTINGS) == before and not case.state()["flagged"], "--record-only changed the live file")
    expect_exit(case.entry("merge", copy), 3, "not flagged")
    expect_exit(case.entry("merge", os.path.join(case.home, SETTINGS)), 3, "is no kept copy")


def c_purge(do: str):
    def fn(case):
        if do == "open":
            case.set_control({"actions": [{"point": "switch-after-part:1", "do": "crash"}]})
            expect_exit(case.run(), 99)
            expect_exit(case.entry("purge-check"), 3, "is open")
            return
        run_to_complete(case)
        if do == "clean":
            out = expect_exit(case.entry("purge-check"), 0, "the conditions that the program reads hold")
            expect(f"retained entry {CLAUDE_SKILL_ENTRY}: keeps its recorded change identity" in out,
                   "the purge check did not read the receipt's retained entries")
            expect("the owner confirms: the owner decides that the rollback window is closed" in out,
                   "the purge check listed no condition of the owner")
            return
        with open(os.path.join(case.state()["backup"], "retained", CLAUDE_SKILL_ENTRY, "SKILL.md"), "a") as f:
            f.write("changed\n")
        expect_exit(case.entry("purge-check"), 3, "changed its identity since its move")
    return fn


def expect_mac_check(case, code: int, status: str, text: str, prepared: bool = True) -> str:
    got = case.mac_check(prepared)
    expect(got[0] == code and got[1] == status and text in got[2], f"mac-check reported {got}, not {code} {status} {text}")
    return got[2]


def c_mac_check_native(case):
    expect_mac_check(case, 0, "OK", "no switch is prepared", prepared=False)
    expect_mac_check(case, 0, "OK", "State native:")


def c_mac_check_complete(case):
    run_to_complete(case)
    expect_mac_check(case, 0, "OK", "State complete:")


def c_mac_check_open(point: str, state: str):
    def fn(case):
        case.set_control({"actions": [{"point": point, "do": "crash"}]})
        expect_exit(case.run(), 99)
        expect_mac_check(case, 10, "Needs attention", f"State {state}:")
    return fn


def c_mac_check_drifted(case):
    case.set_control({"actions": [{"point": "step-14", "do": "crash"}]})
    expect_exit(case.run(), 99)
    write_in_place(os.path.join(case.home, SETTINGS), SUPERSEDED_PLUGIN, True)
    expect_mac_check(case, 10, "Needs attention", "State drifted:")
    expect_exit(case.run(), 3, "drifted")
    expect(case.receipts() == 0, "a run copied a receipt for a drifted layout")


def c_mac_check_changed(case):
    write_in_place(os.path.join(case.home, SETTINGS), SUPERSEDED_PLUGIN, "other")
    expect_mac_check(case, 10, "Needs attention", "State changed:")


def c_mac_check_flagged(where: str):
    def fn(case):
        if where == "settings":
            c_native_descriptor(case)
            of = RAVEL
        elif where == "held":
            c_hooks_descriptor(case)
            of = SETTINGS
        else:
            flagged_switch_copy(case, [
                {"point": BEFORE_EXCHANGE, "do": "write-key", "path": SETTINGS, "key": ["fault"], "value": 1}])
            of = SETTINGS
        expect(where == "backup" or case.state()["backup"] is None, "a backup exists before mac-check reads the copy")
        expect_mac_check(case, 10, "Needs attention", f"Flagged kept copies of: {of}")
    return fn


def pending_body(case) -> dict:
    blocks = re.findall(r"<!-- owner-switch-pending (\S+) -->\n.*?```json\n(.*?)\n```", case.manual(), re.S)
    expect(blocks, "the manual holds no pending-activation record")
    return json.loads(blocks[-1][1])


def c_pending_after_step(case):
    case.set_control({"mise": {"linkuistics/taps/personal": {"installed_version": "0.9"}}})
    expect_exit(case.run(), 3, "step 5")
    expect_exit(case.entry("pending", "--grove-commit", "0123abcd"), 0, "pending-activation record")
    body = pending_body(case)
    expect(body["grove"]["version"] == "1" and body["grove"]["release_commit"] == "0123abcd"
           and body["grove"]["read"] == "from step 4 of the run", f"the record names the wrong Grove: {body['grove']}")
    expect(body["ravel"]["version"] == "ravel 2.0.0 (fixture)" and body["ravel"]["path"].endswith("/prefix/bin/ravel"),
           f"the record names the wrong Ravel: {body['ravel']}")
    expect(body["formula_versions"]["linkuistics/taps/personal"] == "0.9", "the record lacks a formula version")
    expect('"linkuistics/taps/personal"' in body["mise_status"]["output"], "the record lacks the mise status")
    expect([f["name"] for f in body["failed"]] == ["skill-formulae"] and "step 5" in body["failed"][0]["output"],
           f"the record lacks the failed step's output: {body['failed']}")
    expect(body["state"] == "native" and body["backups"] == [] and body["run"]["result"] == "stop",
           f"the record names the wrong state: {body['state']} {body['backups']}")


def c_pending_before_step_4(case):
    sleeper(case, "codex")
    expect_exit(case.run(), 3, "before step 1")
    expect_exit(case.entry("pending"), 0, "pending-activation record")
    body = pending_body(case)
    expect(body["grove"].get("version") == "1" and body["grove"]["read"].startswith("by this record"),
           f"the record did not read the Grove version itself: {body['grove']}")
    expect(body["grove"]["release_commit"] == "not supplied", "the record invented a release commit")
    expect("another Claude Code" in body["run"]["reason"], "the record lacks the stop's reason")


def c_pending_after_inverse(case):
    case.set_control({"fail": {"probe-switched": 1}})
    expect_exit(case.run(), 3, "the native probe passed")
    expect_exit(case.entry("pending"), 0, "pending-activation record")
    body = pending_body(case)
    expect(len(body["backups"]) == 1 and body["state"] == "native", f"unexpected {body['backups']} {body['state']}")
    expect([f["name"] for f in body["failed"]] == ["probe-switched"], f"the record lacks the failed probe: {body['failed']}")
    case.set_control({})
    expect_exit(case.run(), 0, "result: complete")
    pending = re.findall(r"<!-- owner-switch-pending (\S+) -->", case.manual())
    expect(f"It replaces the pending-activation records {pending[0]}." in case.manual(),
           "the receipt does not name the pending-activation record that it replaces")


def c_pending_drifted(case):
    case.set_control({"actions": [{"point": "step-14", "do": "crash"}]})
    expect_exit(case.run(), 99)
    write_in_place(os.path.join(case.home, RAVEL), ["skills"], {"other": True})
    os.makedirs(os.path.join(case.home, ".claude/skills/new-entry"))
    expect_exit(case.run(), 3, "drifted")
    expect_exit(case.entry("pending"), 0, "pending-activation record")
    body = pending_body(case)
    cells = {(c["path"], c.get("key")): c for c in body["cells"]}
    expect(body["state"] == "drifted", f"unexpected {body['state']}")
    expect(cells[(RAVEL, "skills")]["value"] == {"other": True} and cells[(RAVEL, "skills")]["status"] == "third",
           f"the record lacks the drifted cell's value: {body['cells']}")
    expect(cells[(SETTINGS, "enabledPlugins.grove@linkuistics")]["status"] == "switched", "the record lacks a switched cell")
    expect(".claude/skills/new-entry" in body["outside_changed"], "the record lacks the changed outside entry")


def entry_controls() -> list:
    return [
        ("owner-inverse-complete", c_owner_inverse_complete, {}),
        ("owner-inverse-third-cell-after-receipt", c_owner_inverse_third_cell("after"), {}),
        ("owner-inverse-third-cell-before-receipt", c_owner_inverse_third_cell("before"), {}),
        ("owner-inverse-entry-content", c_owner_inverse_entry("modify-entry", CLAUDE_SKILL_ENTRY), {}),
        ("owner-inverse-entry-retarget", c_owner_inverse_entry("retarget", CODEX_SKILL_LINK), {}),
        ("owner-inverse-occupied-path", c_owner_inverse_occupied, {}),
        ("owner-inverse-runs-backup-copy", c_owner_inverse_copy, {}),
        ("owner-inverse-refusals", c_owner_inverse_refusals, {}),
        ("owner-inverse-writer", c_owner_inverse_writer, {}),
        ("merge-difference", c_merge_difference, {}),
        ("merge-then-change", c_merge_then_change, {}),
        ("merge-return-through-descriptor", c_merge_return, {}),
        ("merge-settings-record-copy", c_merge_settings_copy, {"ravel": "absent-key"}),
        ("merge-held-copy", c_merge_held_copy, {"hooks": "absent"}),
        ("merge-third-cell", c_merge_third_cell, {}),
        ("merge-record-only", c_merge_record_only, {}),
        ("purge-check-clean", c_purge("clean"), {}),
        ("purge-check-retained-changed", c_purge("changed"), {}),
        ("purge-check-open-run", c_purge("open"), {}),
        ("mac-check-native", c_mac_check_native, {}),
        ("mac-check-complete", c_mac_check_complete, {}),
        ("mac-check-active", c_mac_check_open("before-record:receipt", "active"), {}),
        ("mac-check-interrupted", c_mac_check_open("switch-after-part:1", "interrupted"), {}),
        ("mac-check-drifted-after-receipt", c_mac_check_drifted, {}),
        ("mac-check-changed", c_mac_check_changed, {}),
        ("mac-check-flagged-settings-before-backup", c_mac_check_flagged("settings"), {"ravel": "absent-key"}),
        ("mac-check-flagged-held-before-backup", c_mac_check_flagged("held"), {"hooks": "absent"}),
        ("mac-check-flagged-backup-copy", c_mac_check_flagged("backup"), {}),
        ("pending-after-step", c_pending_after_step, {}),
        ("pending-before-step-4", c_pending_before_step_4, {}),
        ("pending-after-inverse", c_pending_after_inverse, {}),
        ("pending-drifted", c_pending_drifted, {}),
    ]


# --- native-off ("The native settings inside finish" and "Test seams") --------------

# --- the serial switch and its restore -------------------------------------------
# docs/specs/owner-skill-delivery.md, "Test seams": backup, caught failures,
# hard interruption, explicit restore, attempt identity and later drift.

# What restore leaves alone: the program's own state, the append-only manual
# and the plugin cache that `claude plugin install` wrote ("Rollback").
SERIAL_SKIP = (STATE, MANUAL, ".claude/plugins/cache/writing-hooks")
SERIAL_ARGS = {"hooks": "absent", "rehearsed": False}


def serial_view(case) -> dict:
    return {k: v for k, v in case.snapshot().items()
            if not any(k == p or k.startswith(p + "/") for p in SERIAL_SKIP)}


def expect_view(case, wanted: dict, what: str) -> None:
    got = serial_view(case)
    differ = sorted(k for k in set(got) | set(wanted) if got.get(k) != wanted.get(k))
    expect(not differ, f"{what}: these paths differ: {differ}")


def receipt_file(case, note: dict, suffix: str = "") -> str:
    return os.path.join(case.home, STATE, "receipts", f"{note['attempt']}{suffix}.json")


def expect_switched(case) -> None:
    expect(case.json(RAVEL)["skills"] == SWITCHED and case.json(RAVEL)["policy"] == "config.rhai",
           f"the Ravel settings hold {case.json(RAVEL)}")
    settings = case.json(SETTINGS)
    expect(settings["enabledPlugins"]["grove@linkuistics"] is False
           and settings["enabledPlugins"][HOOKS_ID] is True and settings["theme"] == "dark",
           f"settings.json holds {settings}")
    for rel in (".claude/skills/grove", ".codex/skills/grove"):
        expect(not os.path.lexists(os.path.join(case.home, rel)), f"{rel} is still active")


def c_serial_switch(case):
    path = os.path.join(case.home, RAVEL)
    os.chmod(path, 0o640)
    subprocess.run(["/usr/bin/xattr", "-w", "user.owner", "kept", path], check=True)
    case.serial_plan()
    before = serial_view(case)
    manual = case.manual()
    note = case.attempt()
    expect(os.stat(note["_path"]).st_mode & 0o777 == 0o444, "the attempt note is not read-only")
    expect(not os.path.lexists(note["backup"]) and "restore --backup" in note["restore"]
           and "progress" not in json.dumps(note), f"the attempt note holds {note}")
    expect_exit(case.switch(note), 0, "result: switched")
    expect_switched(case)
    calls = case.commands()
    expect(calls.get("plugin marketplace add") == 1 and calls.get("plugin install") == 1,
           f"the switch ran these Claude Code commands: {calls}")
    meta = subprocess.run(["/usr/bin/xattr", "-p", "user.owner", path], capture_output=True, text=True)
    expect(meta.stdout.strip() == "kept" and os.stat(path).st_mode & 0o777 == 0o640,
           "the ordinary write lost the Ravel settings' mode or attribute")
    manifest = load(os.path.join(note["backup"], "manifest.json"), {})
    items = {i["path"]: i for i in manifest.get("items", [])}
    expect(items[KNOWN_MARKETPLACES]["data"] is None and items[RAVEL]["data"],
           "the backup does not record the original absence and content")
    expect(sorted(manifest["evidence"]) == ["4", "5", "6", "7"], "the backup lacks the evidence of steps 4 to 7")
    programs = [n for _, _, names in os.walk(note["backup"]) for n in names if n.endswith(".py")]
    expect(not programs, f"the backup holds a program copy: {programs}")
    receipt = load(receipt_file(case, note), {})
    expect(sorted(receipt["restore_identities"]) == sorted(SERIAL_RESTORE)
           and all(a["holds"] for a in receipt["delivery_assertions"]),
           "the receipt does not keep restore identities and delivery assertions apart")
    retained = {r["item"] for r in receipt["retained"]}
    expect({"backup", "staging HOME", "candidate keg", "the original driver's Ravel keg"} <= retained,
           f"the receipt retains only {sorted(retained)}")
    expect(case.manual().startswith(manual) and f"owner-switch-attempt {note['attempt']} switched" in case.manual(),
           "the manual did not receive an appended success observation")
    out = expect_exit(case.literal(note["restore"]), 0, "restore: later")
    expect("recovery note: " in out, "an eligible later restore wrote no recovery backup")
    expect_view(case, before, "after the later restore")
    expect(os.stat(path).st_mode & 0o777 == 0o640, "restore lost the Ravel settings' mode")
    expect(not os.path.lexists(receipt_file(case, note)) and os.path.exists(receipt_file(case, note, ".invalid")),
           "restore left a valid receipt")
    text = case.manual()
    expect(text.startswith(manual) and f"{note['attempt']} correction" in text
           and f"{note['attempt']} restored" in text, "the manual lacks the correction and the native return")
    expect_exit(case.literal(note["restore"]), 0, "restored already")
    expect_view(case, before, "after a repeated restore")


def serial_failure(point: str, fail=None, actions=None, text="", check=None):
    """A caught failure at POINT, by a failing fixture command FAIL or by the hook's ACTIONS."""
    def fn(case):
        case.serial_plan()
        before = serial_view(case)
        manual = case.manual()
        note = case.attempt()
        if fail:
            control = {"fail": fail}
        else:
            control = {"actions": actions or [{"point": point, "do": "fail"}]}
        case.set_control(control)
        out = expect_exit(case.switch(note), 5, "restore: native delivery verified")
        if text:
            expect(re.search(text, out) is not None, f"expected {text!r} in output: {out[-1500:]}")
        if check:
            check(case, out)
        expect_view(case, before, "after the caught failure")
        expect_originals(case)
        expect(not os.path.lexists(receipt_file(case, note)), "a success receipt remains")
        text_now = case.manual()
        expect(text_now.startswith(manual) and f"{note['attempt']} restored" in text_now,
               "the manual lacks the native return")
        if point == "switch:final":
            expect(f"{note['attempt']} correction" in text_now, "the success observation received no correction")
        expect_exit(case.switch(note), 3, "the backup .+ of this attempt exists")
    return fn


def serial_crash(point: str):
    def fn(case):
        case.serial_plan()
        before = serial_view(case)
        note = case.attempt()
        case.set_control({"actions": [{"point": point, "do": "crash"}]})
        expect_exit(case.switch(note), 99)
        # No forward continuation: the note's own switch refuses, and its restore is the next action.
        expect_exit(case.switch(note), 3, "the backup .+ of this attempt exists")
        out = expect_exit(case.literal(note["restore"]), 0, "result: restored")
        expect_view(case, before, "after the explicit restore")
        return re.search(r"^restore: (immediate|later)$", out, re.M).group(1)
    return fn


def c_serial_backup_stopped(case):
    case.serial_plan()
    before = serial_view(case)
    note = case.attempt()
    case.set_control({"actions": [{"point": "backup:before-publish", "do": "fail"}]})
    expect_exit(case.switch(note), 3, "no switch-set write happened")
    expect_view(case, before, "after a failure before the backup's publication")
    expect(not os.path.lexists(note["backup"]), "an incomplete backup was published")
    case.set_control({"actions": [{"point": "backup:copy:3", "do": "crash"}]})
    expect_exit(case.switch(note), 99)
    expect_view(case, before, "after a crash inside the backup")
    expect(not os.path.lexists(note["backup"]), "an incomplete backup was published")
    # No backup was published, so the same note admits the switch.
    expect_exit(case.switch(note), 0, "result: switched")


def c_serial_attempt_mismatch(case):
    case.serial_plan()
    before = serial_view(case)
    first, second = case.attempt(), case.attempt()
    case.set_control({"actions": [{"point": "deactivate:.codex/skills/grove", "do": "crash"}]})
    expect_exit(case.switch(first), 99)
    middle = serial_view(case)
    expect_exit(case.restore(first["backup"], second), 3, "does not match its attempt note")
    other = os.path.join(case.root, "other-home")
    os.makedirs(other)
    expect_exit(case.restore(first["backup"], first, home=other), 3, "does not match its attempt note")
    expect_exit(case.switch(second), 3, "planned originals differ")
    expect_view(case, middle, "after the refusals")
    expect_exit(case.restore(first["backup"], first), 0, "result: restored")
    expect_view(case, before, "after the restore")


def c_serial_drift(case):
    case.serial_plan()
    note = case.attempt()
    expect_exit(case.switch(note), 0, "result: switched")
    with open(os.path.join(case.home, MANUAL), "a") as f:
        f.write("An unrelated note.\n")
    write_in_place(os.path.join(case.home, SETTINGS), ["model"], "opus")
    middle = serial_view(case)
    out = expect_exit(case.literal(note["restore"]), 3, "differ from the receipt")
    expect(".claude/settings.json" in out.split("receipt's post-switch identity", 1)[-1],
           "the refusal does not name the changed file")
    expect_view(case, middle, "after the refused later restore")
    expect(os.path.exists(receipt_file(case, note)), "a refused restore invalidated the receipt")


def c_serial_preflight(case):
    case.serial_plan(lambda plan: plan["switch"]["inputs"][1].update(digest="0" * 64))
    before = serial_view(case)
    note = case.attempt()
    expect_exit(case.switch(note), 3, "command inputs differ")
    case.serial_plan()
    note = case.attempt()
    write(os.path.join(case.home, ".claude/skills/extra/SKILL.md"), "---\nname: extra\n---\n")
    expect_exit(case.switch(note), 3, "inventory differs")
    shutil.rmtree(os.path.join(case.home, ".claude/skills/extra"))
    write_in_place(os.path.join(case.home, RAVEL), ["skills"], None)
    case.serial_plan()
    off = serial_view(case)
    note = case.attempt()
    expect_exit(case.switch(note), 3, "does not hold the off value")
    expect_view(case, off, "after the refusals")
    expect(not os.path.lexists(note["backup"]), "a refusal published a backup")
    return f"{len(before)} paths unchanged"


def expect_originals(case, skip=()) -> None:
    """Each planned file and entry has its planned original identity: content, mode, ACL, attributes and link text."""
    m = case.r.module()
    for w in load(case.serial, {})["switch"]["writes"]:
        if w["path"] in skip:
            continue
        wanted = "absent" if w["original"].get("absent") else w["original"]["identity"]
        got = m.restore_identity(os.path.join(case.home, w["path"]))
        expect(got == wanted, f"{w['path']} has the restore identity {got}, not its original {wanted}")


def expect_backup_complete(case, note: dict) -> None:
    """The published backup holds each original whole, or records its absence."""
    m = case.r.module()
    items = {i["path"]: i for i in load(os.path.join(note["backup"], "manifest.json"), {})["items"]}
    for w in load(case.serial, {})["switch"]["writes"]:
        item = items[w["path"]]
        if w["original"].get("absent"):
            expect(item["data"] is None and item["identity"] == "absent", f"the backup holds the absent {w['path']}")
            continue
        got = m.restore_identity(os.path.join(note["backup"], item["data"]))
        expect(got == w["original"]["identity"], f"the backup copy of {w['path']} has the identity {got}")


def acl_lines(path: str) -> str:
    return subprocess.run(["/bin/ls", "-led", path], capture_output=True, text=True, check=True).stdout


def xattr_of(path: str, name: str) -> str:
    return subprocess.run(["/usr/bin/xattr", "-p", name, path], capture_output=True, text=True).stdout.strip()


def mark_metadata(path: str, right: str) -> None:
    subprocess.run(["/bin/chmod", "+a", f"everyone allow {right}", path], check=True)
    subprocess.run(["/usr/bin/xattr", "-w", "user.owner", "kept", path], check=True)


def restored_record(case, note: dict) -> dict:
    return load(os.path.join(case.home, STATE, "restored", f"{note['attempt']}.json"), None)


def c_serial_backup_metadata(case):
    entry = os.path.join(case.home, ".claude/skills/grove")
    marked = {entry: "list", os.path.join(entry, "SKILL.md"): "read", os.path.join(case.home, RAVEL): "read",
              os.path.join(case.home, INSTALLED_PLUGINS): "read"}
    for path, right in marked.items():
        mark_metadata(path, right)
    case.serial_plan()
    before = serial_view(case)
    note = case.attempt()
    case.set_control({"actions": [{"point": "probes", "do": "crash"}]})
    expect_exit(case.switch(note), 99)
    ravel = os.path.join(case.home, RAVEL)
    expect(case.json(RAVEL)["skills"] == SWITCHED and "everyone allow read" in acl_lines(ravel)
           and xattr_of(ravel, "user.owner") == "kept", "the ordinary write lost the Ravel settings' ACL or attribute")
    expect_backup_complete(case, note)
    copy = os.path.join(note["backup"], "data/0")
    expect("everyone allow list" in acl_lines(copy) and xattr_of(copy, "user.owner") == "kept"
           and os.path.islink(os.path.join(note["backup"], "data/1")),
           "the backup lacks the folder's ACL or attribute, or the link")
    expect_exit(case.literal(note["restore"]), 0, "restore: immediate")
    expect_view(case, before, "after the restore")
    expect_originals(case)
    for path, right in marked.items():
        expect(f"everyone allow {right}" in acl_lines(path) and xattr_of(path, "user.owner") == "kept",
               f"restore lost the ACL or attribute of {path}")
    return "ACLs and attributes of an entry folder, its file and two settings files survive backup and restore"


def c_serial_backup_incomplete(case):
    case.serial_plan()
    before = serial_view(case)
    note = case.attempt()
    entry = os.path.join(case.home, ".claude/skills/grove")
    skill = os.path.join(entry, "SKILL.md")
    entry_mode, skill_mode = os.stat(entry).st_mode & 0o7777, os.stat(skill).st_mode & 0o7777
    with open(skill, "rb") as f:
        skill_bytes = f.read()
    incomplete = f"{STATE}/backups/.incomplete-{note['attempt']}"

    def refused(actions: list, text: str, what: str) -> None:
        case.set_control({"actions": actions})
        expect_exit(case.switch(note), 3, text)
        os.chmod(entry, entry_mode)
        os.chmod(skill, skill_mode)
        with open(skill, "wb") as f:
            f.write(skill_bytes)
        expect_view(case, before, what)
        expect(not os.path.lexists(note["backup"]), f"{what}: a backup was published")

    refused([{"point": "backup:copy:0", "do": "chmod", "path": ".claude/skills/grove/SKILL.md", "mode": "000"}],
            r"SKILL\.md cannot be read", "after a failed copy read")
    refused([{"point": "backup:copy:0", "do": "chmod", "path": ".claude/skills/grove", "mode": "000"}],
            r"skills/grove cannot be read", "after a failed folder metadata read")
    refused([{"point": "backup:verify", "do": "append-glob", "path": f"{incomplete}/data/0/SKILL.md"}],
            "the backup copy of .claude/skills/grove differs from the original", "after a changed copy")
    refused([{"point": "backup:verify", "do": "append-glob", "path": ".claude/skills/grove/SKILL.md"}],
            "the backup copy of .claude/skills/grove differs from the original", "after a changed original")
    os.chmod(skill, 0o000)
    refused([], r"SKILL\.md cannot be read", "after an unreadable original before the backup")
    # No backup was published, so the same note admits the switch.
    case.set_control({})
    expect_exit(case.switch(note), 0, "result: switched")
    return "failed reads and a copy or original that differs stop before publication with no write"


def c_serial_restore_interrupted(case):
    case.serial_plan()
    before = serial_view(case)
    note = case.attempt()
    case.set_control({"actions": [{"point": "write:.config/ravel/settings.json:replace", "do": "crash"}]})
    expect_exit(case.switch(note), 99)
    folder = os.path.join(case.home, ".config/ravel")
    expect(any(".owner-switch-" in n for n in os.listdir(folder)), "the crash left no temporary Ravel settings")
    for point in ("restore:.claude/settings.json", "restore:native-probes"):
        case.set_control({"actions": [{"point": point, "do": "crash"}]})
        expect_exit(case.literal(note["restore"]), 99)
        expect_exit(case.switch(note), 3, "the backup .+ of this attempt exists")
    case.set_control({})
    expect_exit(case.literal(note["restore"]), 0, "restore: immediate")
    expect_view(case, before, "after the repeated restore")
    expect_originals(case)
    return "a restore interrupted twice completes when the owner repeats the same command"


def serial_restore_off(ravel: str):
    """A constructed backup whose Ravel settings lack the off value: restore enforces it."""
    def fn(case):
        case.serial_plan()
        before = serial_view(case)
        note = case.attempt()
        # The switch refuses without the off value, so the program's own backup function makes the backup.
        m = case.r.module()
        plan = m.load_plan(case.serial)
        sw = m.switch_section(plan)
        layout = m.Layout(case.home, plan)
        with contextlib.redirect_stdout(io.StringIO()):
            m.backup_published(layout, m.create_backup(layout, plan, sw, m.load_note(note["_path"]), {}))
        save_json(os.path.join(case.home, RAVEL), dict(case.json(RAVEL) or {}, skills=SWITCHED))
        shutil.rmtree(os.path.join(case.home, ".claude/skills/grove"))
        expect_exit(case.literal(note["restore"]), 0, "restore: immediate")
        wanted = {"skills": OFF} if ravel == "absent-file" else {"policy": "config.rhai", "skills": OFF}
        expect(case.json(RAVEL) == wanted, f"the Ravel settings hold {case.json(RAVEL)}, not {wanted}")
        got = serial_view(case)
        differ = sorted(k for k in set(got) | set(before) if got.get(k) != before.get(k))
        expect(differ == [RAVEL], f"restore changed more than the Ravel settings: {differ}")
        expect_originals(case, skip=(RAVEL,))
        return f"the Ravel settings ({ravel}) gain only the off value"
    return fn


def c_serial_restore_io_failure(case):
    case.serial_plan()
    before = serial_view(case)
    note = case.attempt()
    case.set_control({"actions": [{"point": "deactivate:.codex/skills/grove", "do": "crash"}]})
    expect_exit(case.switch(note), 99)
    skills = os.path.join(case.home, ".claude/skills")
    mode = os.stat(skills).st_mode & 0o7777
    case.set_control({"actions": [{"point": "restore:.claude/skills/grove", "do": "chmod", "path": ".claude/skills",
                                   "mode": "555"}]})
    out = expect_exit(case.literal(note["restore"]), 4, r"failed path \.claude/skills/grove: ")
    expect(re.search(r"result: recovery incomplete\. Repeat: ", out) is not None and "restore: native delivery verified"
           not in out, f"a failed restore claimed success: {out[-1500:]}")
    expect(os.path.isdir(note["backup"]) and restored_record(case, note) is None
           and f"{note['attempt']} restored" not in case.manual(), "a failed restore removed the backup or recorded success")
    os.chmod(skills, mode)
    case.set_control({})
    expect_exit(case.literal(note["restore"]), 0, "result: restored")
    expect_view(case, before, "after the repeated restore")
    return "a permission failure names its path, keeps the backup and claims nothing"


def no_command(kind: str):
    def check(case, out):
        expect(not case.commands().get(kind), f"`claude {kind}` ran: {case.commands()}")
    return check


def c_serial_later_restore_interrupted(case):
    case.serial_plan()
    before = serial_view(case)
    note = case.attempt()
    expect_exit(case.switch(note), 0, "result: switched")
    case.set_control({"actions": [{"point": "restore:.claude/settings.json", "do": "crash"}]})
    out = expect_exit(case.literal(note["restore"]), 99)
    retry = re.search(r"^if this restore is interrupted, repeat under the same quiet interval: (.+)$", out, re.M)
    rnote = re.search(r"^recovery note: (.+)$", out, re.M)
    expect(retry is not None and rnote is not None and "--recovery" in retry.group(1),
           f"the later restore gave no recovery note or retry command: {out[-1500:]}")
    recovery = load(rnote.group(1), {})
    expect(os.path.isdir(recovery["backup"]), "the recovery backup is missing")
    middle = serial_view(case)
    # The note's own command now meets a layout that differs from the receipt.
    expect_exit(case.literal(note["restore"]), 3, "differ from the receipt")
    expect_view(case, middle, "after the refused restore")
    case.set_control({})
    expect_exit(case.literal(retry.group(1)), 0, "restore: retry")
    expect_view(case, before, "after the retried restore")
    expect(os.path.isdir(recovery["backup"]), "the retry removed the recovery backup")
    expect(os.path.exists(receipt_file(case, note, ".invalid")) and not os.path.lexists(receipt_file(case, note)),
           "the retry left a valid receipt")
    text = case.manual()
    expect(f"{note['attempt']} correction" in text and f"{note['attempt']} restored" in text,
           "the manual lacks the correction or the native return")
    expect_exit(case.literal(note["restore"]), 0, "restored already")
    return "an interrupted later restore keeps its recovery backup and completes through its retry command"


def c_serial_owner_after_finish(case):
    case.serial_plan()
    before = serial_view(case)
    note = case.attempt()
    finish = relay(case, "ravel") + relay(case, "share/claude/versions/2.1.319")
    case.set_control({"actions": [{"point": "deactivate:.codex/skills/grove", "do": "crash"}]})
    out = expect_exit(case.switch(note, via=finish), 99)
    expect(re.search(r"admitted ancestor process \d+ .*Ravel", out) is not None, "the finish lineage had no Ravel")
    middle = serial_view(case)
    owner = relay(case, "share/claude/versions/2.1.319")
    # A harness that outlived the ended finish is another session, with no exemption.
    leftover = sleeper(case, "share/claude/versions/2.1.319", title="claude")
    case.set_control({})
    expect_exit(case.restore(note["backup"], note, via=owner), 3, f"remaining process {leftover} ")
    expect_view(case, middle, "after the refused restore")
    os.kill(leftover, signal.SIGKILL)
    time.sleep(0.3)
    guidance = os.path.join(case.root, "guidance", "SKILL.md")
    write(guidance, "---\nname: grove-finish\n---\nThe retained instructions.\n")
    # Guidance that changes before the end of restore leaves recovery incomplete.
    case.set_control({"actions": [{"point": "restore:native-probes", "do": "touch", "path": guidance,
                                   "text": "changed\n"}]})
    expect_exit(case.restore(note["backup"], note, extra=["--guidance", guidance], via=owner), 4,
                "the operator's guidance changed or cannot be read")
    expect(restored_record(case, note) is None, "an incomplete restore wrote its restored record")
    case.set_control({})
    write(guidance, "---\nname: grove-finish\n---\nThe retained instructions.\n")
    out = expect_exit(case.restore(note["backup"], note, extra=["--guidance", guidance], via=owner), 0,
                      "result: restored")
    expect_view(case, before, "after the owner's restore")
    record = restored_record(case, note) or {}
    operator = record.get("operator") or []
    expect(any("Claude Code" in line for line in operator) and not any("Ravel" in line for line in operator),
           f"the restored record does not name the owner's own session: {operator}")
    expect([g["path"] for g in record.get("guidance") or []] == [guidance], f"the record lacks the guidance: {record}")
    return "after finish, the owner's own Claude Code session restores and the record names it"


def serial_manual_pending(point: str, success: bool):
    """An unwritable manual: configuration recovery completes, and the manual entry stays pending."""
    def fn(case):
        case.serial_plan()
        before = serial_view(case)
        manual = case.manual()
        note = case.attempt()
        actions = [{"point": point, "do": "chmod", "path": MANUAL, "mode": "444"}]
        if point == "switch:final":
            actions.append({"point": point, "do": "fail"})
        case.set_control({"actions": actions})
        out = expect_exit(case.switch(note), 4, "the configuration is restored and verified, but the manual entry is "
                                                 "pending")
        expect_view(case, before, "after the recovery")
        expect_originals(case)
        expect(not os.path.lexists(receipt_file(case, note)) and os.path.exists(receipt_file(case, note, ".invalid")),
               "a valid receipt remains")
        text = case.manual()
        expect(text.startswith(manual) and (f"{note['attempt']} switched" in text) == success
               and f"{note['attempt']} restored" not in text, f"the manual changed unexpectedly: {text}")
        os.chmod(os.path.join(case.home, MANUAL), 0o644)
        case.set_control({})
        expect_exit(case.literal(note["restore"]), 0, "restored already")
        text = case.manual()
        expect(text.startswith(manual) and f"{note['attempt']} restored" in text
               and (f"{note['attempt']} correction" in text) == success,
               f"the repeated restore did not append the pending entries: {text}")
        expect_exit(case.literal(note["restore"]), 0, "restored already")
        expect(case.manual() == text, "a third restore appended again")
        expect_view(case, before, "after the pending entries")
        return "the pending manual entries follow when the manual is writable again"
    return fn


def c_serial_probe_owned(case):
    case.serial_plan()
    before = serial_view(case)
    note = case.attempt()
    case.set_control({"probe_owned": True})
    out = expect_exit(case.switch(note), 4, "remaining probe-owned service")
    expect("restore: native delivery verified" not in out and "restored:" not in out,
           f"the switch restored while a probe-owned service ran: {out[-1500:]}")
    expect_switched(case)
    expect(os.path.isdir(note["backup"]) and not os.path.lexists(receipt_file(case, note)),
           "the backup is missing, or a receipt was written")
    case.kill()
    time.sleep(0.3)
    case.set_control({})
    expect_exit(case.literal(note["restore"]), 0, "restore: immediate")
    expect_view(case, before, "after the owner's restore")
    return "a live probe-owned service stops the switch with 4 and no restore"


def serial_probe_change(action: dict, text: str, extra=None):
    """A change during the probes: the final comparison fails, and no success receipt remains."""
    def fn(case):
        case.serial_plan()
        before = serial_view(case)
        note = case.attempt()
        case.set_control({"actions": [dict(action, point="probes")]})
        out = expect_exit(case.switch(note), 5, "the final comparison failed")
        expect(re.search(text, out) is not None, f"expected {text!r} in output: {out[-1500:]}")
        expect(not os.path.lexists(receipt_file(case, note)) and not os.path.lexists(receipt_file(case, note, ".invalid"))
               and f"{note['attempt']} switched" not in case.manual(), "a receipt or success observation was written")
        got = serial_view(case)
        differ = sorted(k for k in set(got) | set(before) if got.get(k) != before.get(k))
        expect(differ == sorted(extra or []), f"after the restore these paths differ: {differ}")
        return "the final comparison fails and restore verifies native delivery"
    return fn


def c_serial_backup_changed(case):
    case.serial_plan()
    before = serial_view(case)
    note = case.attempt()
    case.set_control({"actions": [{"point": "deactivate:.codex/skills/grove", "do": "crash"}]})
    expect_exit(case.switch(note), 99)
    middle = serial_view(case)
    m = case.r.module()
    paths = {name: os.path.join(note["backup"], name) for name in ("manifest.json", "plan.json", "data/0/SKILL.md")}
    saved = {}
    for name, path in paths.items():
        with open(path, "rb") as f:
            saved[name] = f.read()

    def refused(text: str, what: str) -> None:
        expect_exit(case.literal(note["restore"]), 3, text)
        expect_view(case, middle, what)
        for name, path in paths.items():
            with open(path, "wb") as f:
                f.write(saved[name])

    def manifest(change) -> None:
        body = json.loads(saved["manifest.json"])
        change(body)
        with open(paths["manifest.json"], "w") as f:
            json.dump(body, f)

    def changed_copy() -> None:
        with open(paths["data/0/SKILL.md"], "a") as f:
            f.write("changed\n")

    def matching(body) -> None:
        nodes = m.nodes_of(os.path.join(note["backup"], "data/0"))
        body["items"][0].update(nodes=nodes, identity=m.nodes_identity(nodes))

    changed_copy()
    refused("differs from its manifest", "after a changed copy")
    changed_copy()
    manifest(matching)
    refused("another identity than the plan's original", "after a changed copy with a matching manifest")
    changed_copy()
    manifest(lambda body: (matching(body), body.update(kind="recovery")))
    refused("is a recovery backup", "after a manifest that names another kind")
    manifest(lambda body: body["items"][0].update(path=".claude/skills/other"))
    refused("is not the plan's entry .claude/skills/grove", "after a manifest item outside the plan")
    manifest(lambda body: body["items"][0].update(data="../../.."))
    refused("names its copy as", "after a copy name outside the backup")
    plan = json.loads(saved["plan.json"])
    plan["switch"]["restore"].reverse()
    plan["switch"]["restore"] = [RAVEL] + [r for r in plan["switch"]["restore"] if r != RAVEL]
    data = (json.dumps(plan, indent=2) + "\n").encode()
    with open(paths["plan.json"], "wb") as f:
        f.write(data)
    manifest(lambda body: body["plan"].update(digest=hashlib.sha256(data).hexdigest()))
    refused("another plan than the attempt note names", "after a changed plan with its digest")
    expect_exit(case.literal(note["restore"]), 0, "result: restored")
    expect_view(case, before, "after the restore")
    return "each changed copy, manifest or plan refuses before any write"


def serial_controls() -> list:
    controls = [
        ("serial-switch-and-later-restore", c_serial_switch, SERIAL_ARGS),
        ("serial-backup-stopped", c_serial_backup_stopped, SERIAL_ARGS),
        ("serial-attempt-mismatch", c_serial_attempt_mismatch, SERIAL_ARGS),
        ("serial-later-drift", c_serial_drift, SERIAL_ARGS),
        ("serial-preflight-refusals", c_serial_preflight, SERIAL_ARGS),
        ("serial-fail-probe", serial_failure("probe", {"probe-switched": 1}), SERIAL_ARGS),
    ]
    failures = ["backup:published", "hooks:after:marketplace-add", "hooks:after:install",
                "write:.claude/settings.json:replace", "deactivate:.codex/skills/grove",
                "write:.config/ravel/settings.json:replace", "probes", "receipt:written",
                "manual:success observation", "switch:final"]
    controls += [(f"serial-fail-{pt}", serial_failure(pt), SERIAL_ARGS) for pt in failures]
    crashes = ["backup:published", "hooks:after:install", "write:.claude/settings.json:replace",
               "deactivate:.codex/skills/grove", "probes", "receipt:written", "switch:final"]
    controls += [(f"serial-crash-{pt}", serial_crash(pt), SERIAL_ARGS) for pt in crashes]
    # The cases of "Test seams" that owner-switch-restore-controls-k273 added.
    controls += [
        ("serial-backup-metadata", c_serial_backup_metadata, SERIAL_ARGS),
        ("serial-backup-incomplete", c_serial_backup_incomplete, SERIAL_ARGS),
        ("serial-backup-changed", c_serial_backup_changed, SERIAL_ARGS),
        ("serial-restore-interrupted", c_serial_restore_interrupted, SERIAL_ARGS),
        ("serial-restore-off-absent-file", serial_restore_off("absent-file"), dict(SERIAL_ARGS, ravel="absent-file")),
        ("serial-restore-off-absent-key", serial_restore_off("absent-key"), dict(SERIAL_ARGS, ravel="absent-key")),
        ("serial-restore-io-failure", c_serial_restore_io_failure, SERIAL_ARGS),
        ("serial-later-restore-interrupted", c_serial_later_restore_interrupted, SERIAL_ARGS),
        ("serial-owner-after-finish", c_serial_owner_after_finish, SERIAL_ARGS),
        ("serial-hooks-command-fails",
         serial_failure("hooks:install", {"claude-install": 1}, text=r"`claude plugin install \S+` failed"),
         SERIAL_ARGS),
        ("serial-hooks-unexpected-change",
         serial_failure("hooks:after:install", actions=[{"point": "hooks:after:install", "do": "write-key",
                                                         "path": SETTINGS, "key": ["model"], "value": "opus"}],
                        text="changed other keys of .claude/settings.json"), SERIAL_ARGS),
        ("serial-hooks-acl-appears",
         serial_failure("hooks:before:marketplace-add",
                        actions=[{"point": "hooks:before:marketplace-add", "do": "acl", "path": SETTINGS}],
                        text="holds an ACL, which `claude plugin marketplace add", check=no_command("plugin marketplace add")),
         SERIAL_ARGS),
        ("serial-hooks-attribute-appears",
         serial_failure("hooks:before:install", actions=[{"point": "hooks:before:install", "do": "xattr",
                                                          "path": SETTINGS}],
                        text="holds the extended attribute user.fault, which `claude plugin install",
                        check=no_command("plugin install")), SERIAL_ARGS),
        ("serial-manual-unwritable", serial_manual_pending("receipt:written", False), SERIAL_ARGS),
        ("serial-manual-correction-pending", serial_manual_pending("switch:final", True), SERIAL_ARGS),
        ("serial-probe-owned-live", c_serial_probe_owned, SERIAL_ARGS),
        ("serial-probe-changes-setting",
         serial_probe_change({"do": "write-key", "path": SETTINGS, "key": HOOKS_PLUGIN, "value": False},
                             r"the assertion \.claude/settings\.json"), SERIAL_ARGS),
        ("serial-probe-adds-entry",
         serial_probe_change({"do": "touch", "path": ".codex/skills/extra/SKILL.md"},
                             r"the inventory changed at \.codex/skills/extra",
                             [".codex/skills/extra", ".codex/skills/extra/SKILL.md"]), SERIAL_ARGS),
        ("serial-probe-changes-registration",
         serial_probe_change({"do": "erase-key", "path": INSTALLED_PLUGINS, "key": ["plugins", "grove@linkuistics"]},
                             "lost its registration"), SERIAL_ARGS),
    ]
    return controls


NATIVE_OFF_POINTS = ("native-off:begin", "native-off:read", "native-off:temp-partial",
                     "native-off:temp-synced", "native-off:replaced", "native-off:published")
NATIVE_OFF_TEMP = ".settings.json.native-off-"
# The accepted Ravel 2.0 build: docs/evaluations/owner-skill-delivery/settings_drafts.py.
BUILDS = os.path.join(HERE, "harness-trials", "builds.json")
ARCHIVES = os.path.join(HERE, "..", "target", "ravel-archives-2.0")


def settings_bytes(case):
    try:
        with open(os.path.join(case.home, RAVEL), "rb") as f:
            return f.read()
    except FileNotFoundError:
        return None


def parsed(data):
    """The JSON value of DATA, or None when DATA is absent or not whole JSON."""
    try:
        return json.loads(data) if data is not None else None
    except ValueError:
        return None


def off_of(before):
    """The complete result of native-off for the bytes BEFORE, or None when they are absent."""
    obj = json.loads(before) if before is not None else {}
    obj["skills"] = OFF
    return obj


def temps(case) -> list:
    folder = os.path.dirname(os.path.join(case.home, RAVEL))
    return sorted(n for n in os.listdir(folder) if n.startswith(NATIVE_OFF_TEMP)) if os.path.isdir(folder) else []


def expect_off(case, before) -> None:
    """The live file holds exactly the off value and BEFORE's other members, in order."""
    now = settings_bytes(case)
    expect(now is not None, "native-off left no settings file")
    obj = parsed(now)
    want = off_of(before)
    expect(isinstance(obj, dict) and list(obj) == list(want) and json.dumps(obj, sort_keys=True) == json.dumps(want, sort_keys=True),
           f"the settings hold {now!r}, not {want}")
    expect(not temps(case), f"a temporary file remains: {temps(case)}")
    expect(not os.path.exists(os.path.join(case.home, STATE, "backups")), "native-off wrote a switch backup")


def file_meta(path: str) -> dict:
    """The mode, owner, ACL and user extended attributes of a file, read by the system tools."""
    st = os.lstat(path)
    names = subprocess.run(["/usr/bin/xattr", path], capture_output=True, text=True).stdout.split()
    values = {n: subprocess.run(["/usr/bin/xattr", "-p", n, path], capture_output=True, text=True).stdout
              for n in names if n.startswith("user.")}
    acl = subprocess.run(["/bin/ls", "-le", path], capture_output=True, text=True).stdout.splitlines()[1:]
    return {"mode": oct(st.st_mode & 0o7777), "uid": st.st_uid, "gid": st.st_gid, "acl": acl, "xattrs": values}


def spawn_fixture(case, name: str, *args: str) -> None:
    """Start the fixture copy NAME of the interpreter, sleeping, as another process."""
    p = subprocess.Popen([os.path.join(case.r.fixture_bin, name), "-c", "import time; time.sleep(120)", *args],
                         env={"PYTHONHOME": sys.base_prefix}, start_new_session=True,
                         stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    record_pid(case.root, p.pid)
    time.sleep(0.2)
    expect(p.poll() is None, f"the fixture {name} ended at once")


def c_native_off_absent_file(case):
    out = expect_exit(case.native_off(), 0, r"\(the file was absent\)")
    expect_off(case, None)
    mode = oct(os.lstat(os.path.join(case.home, RAVEL)).st_mode & 0o7777)
    expect(mode == "0o644", f"the new settings file has mode {mode}")
    return "a new file holds only the off value, mode 0o644, with no switch backup"


def c_native_off_absent_key(case):
    path = os.path.join(case.home, RAVEL)
    os.chmod(path, 0o600)
    subprocess.run(["/usr/bin/xattr", "-w", "user.fault", "kept", path], check=True)
    subprocess.run(["/bin/chmod", "+a", "everyone allow read", path], check=True)
    before, meta = settings_bytes(case), file_meta(path)
    expect_exit(case.native_off(), 0, r"\(1 other keys kept\)")
    expect_off(case, before)
    after = file_meta(path)
    expect(after == meta, f"the metadata changed from {meta} to {after}")
    return f"the member policy, mode {meta['mode']}, ACL and user.fault are kept"


def c_native_off_existing(case):
    path = os.path.join(case.home, RAVEL)
    before, inode = case.snapshot(), os.lstat(path).st_ino
    expect_exit(case.native_off(), 0, "holds the off value; nothing changed")
    expect(case.snapshot() == before and os.lstat(path).st_ino == inode, "an existing off value was rewritten")


def native_off_refusal(body):
    def fn(case):
        path = os.path.join(case.home, RAVEL)
        if body == "link":
            target = os.path.join(case.root, "elsewhere-settings.json")
            write(target, '{"policy": "config.rhai"}\n')
            os.remove(path)
            os.symlink(target, path)
        else:
            with open(path, "wb") as f:
                f.write(body)
        before = case.snapshot()
        out = expect_exit(case.native_off(), 3, "native delivery is incomplete unless")
        expect(case.snapshot() == before, "a refusal changed the layout")
        return out.splitlines()[0][:200]
    return fn


def c_native_off_other_ravel(case):
    spawn_fixture(case, "ravel")
    before = case.snapshot()
    expect_exit(case.native_off(), 3, "another Ravel driver runs")
    expect(case.snapshot() == before, "native-off wrote while another Ravel process ran")
    case.kill()
    expect_exit(case.native_off(), 0)
    expect_off(case, b'{\n  "policy": "config.rhai"\n}\n')


def c_native_off_ancestor(case):
    # A helper whose arguments name ravel is not a Ravel process.
    spawn_fixture(case, "helper", "ravel", os.path.join(case.r.fixture_bin, "ravel"))
    before = settings_bytes(case)
    launcher = [os.path.join(case.r.fixture_bin, "ravel"), "-c",
                "import subprocess, sys; sys.exit(subprocess.run(sys.argv[1:]).returncode)"]
    expect_exit(case.native_off(via=launcher), 0, "result: off value in place")
    expect_off(case, before)
    return "a Ravel ancestor is exempt, and a helper whose arguments name ravel is not recognised"


def c_native_off_after_switch_stop(case):
    expect_exit(case.run(), 3, "so no prelude runs")
    expect(skills_cell(case) == "absent-key", "the stopped switch changed the settings")
    before = settings_bytes(case)
    expect_exit(case.native_off(), 0)
    expect_off(case, before)
    return "the switch stopped before any backup, and native-off then wrote the off value"


def c_native_off_print_command(case):
    before = case.snapshot()
    code, out = case.native_off("--print-command")
    expect(code == 0, f"--print-command exited {code}: {out}")
    doc = json.loads(out)
    expect(doc["argv"][1:4] == ["-I", case.r.program, "native-off"] and doc["home"] == case.home
           and doc["argv"][-1] == case.home and doc["settings"] == os.path.join(case.home, RAVEL),
           f"--print-command printed {doc}")
    expect(case.snapshot() == before, "--print-command changed the layout")
    expect_exit(case.native_off(argv=doc["argv"]), 0, "result: off value in place")
    expect_off(case, b'{\n  "policy": "config.rhai"\n}\n')
    return doc["command"]


def c_native_off_create_fails(case):
    folder = os.path.dirname(os.path.join(case.home, RAVEL))
    before = settings_bytes(case)
    os.chmod(folder, 0o555)
    try:
        expect_exit(case.native_off(), 4, "native delivery is incomplete: start no new default launch")
        expect(settings_bytes(case) == before and not temps(case), "the failed creation changed the folder")
    finally:
        os.chmod(folder, 0o755)
    expect_exit(case.native_off(), 0)
    expect_off(case, before)


def c_native_off_replace_fails(case):
    folder = os.path.dirname(os.path.join(case.home, RAVEL))
    case.set_control({"actions": [{"point": "native-off:temp-synced", "do": "chmod",
                                   "path": os.path.dirname(RAVEL), "mode": "555"}]})
    before = settings_bytes(case)
    try:
        expect_exit(case.native_off(), 4, "native delivery is incomplete: start no new default launch")
        expect(settings_bytes(case) == before, "the failed replacement changed the live file")
        left = temps(case)
        expect(len(left) == 1, f"expected the one temporary file that could not be removed, found {left}")
    finally:
        os.chmod(folder, 0o755)
    expect_exit(case.native_off(), 0, "removed the interrupted temporary file")
    expect_off(case, before)
    return f"the rename failed and left {left[0]}, which the rerun removed"


def native_off_crash(point: str):
    def fn(case):
        case.set_control({"actions": [{"point": point, "do": "crash"}]})
        before = settings_bytes(case)
        expect_exit(case.native_off(), 99)
        expect(point in case.points(), f"the point {point} was not reached")
        middle = settings_bytes(case)
        complete = middle != before and parsed(middle) == off_of(before)
        expect(middle == before or complete, f"the crash left {middle!r}")
        left = temps(case)
        expect_exit(case.native_off(), 0, "result: off value in place")
        expect_off(case, before)
        return f"after the crash: {'complete' if complete else 'original'}, temporary files {left}"
    return fn


def c_native_off_reader(case):
    """A reader that polls the settings during 20 publications sees only whole files."""
    path = os.path.join(case.home, RAVEL)
    original = settings_bytes(case)
    seen = {"original": 0, "complete": 0, "other": []}
    stop = threading.Event()

    def reader():
        while not stop.is_set():
            try:
                with open(path, "rb") as f:
                    data = f.read()
            except FileNotFoundError:
                seen["other"].append("absent")
                continue
            if data == original:
                seen["original"] += 1
            else:
                if parsed(data) == off_of(original):
                    seen["complete"] += 1
                else:
                    seen["other"].append(data[:80])

    thread = threading.Thread(target=reader)
    thread.start()
    try:
        for _ in range(20):
            expect_exit(case.native_off(), 0)
            with open(path + ".reset", "wb") as f:
                f.write(original)
            os.replace(path + ".reset", path)
    finally:
        stop.set()
        thread.join()
    expect(not seen["other"], f"a reader saw {seen['other'][:3]}")
    expect(seen["original"] and seen["complete"], f"the reader saw too little: {seen}")
    return f"{seen['original']} original and {seen['complete']} complete readings; none partial or absent"


def c_native_off_persistence(case):
    """Measure the publication on the volume of the account's HOME."""
    account = os.path.realpath(pwd.getpwuid(os.getuid()).pw_dir)
    home_dev, case_dev = os.stat(account).st_dev, os.stat(case.home).st_dev
    expect(home_dev == case_dev, f"the case HOME is on device {case_dev}, not on the HOME volume {home_dev}; "
                                 "choose an OUT on the HOME volume")
    before = settings_bytes(case)
    started = time.time()
    expect_exit(case.native_off(), 0, "persistence: temporary file F_FULLFSYNC; folder F_FULLFSYNC")
    seconds = time.time() - started
    fresh = subprocess.run([PY, "-I", "-c", "import sys; sys.stdout.buffer.write(open(sys.argv[1], 'rb').read())",
                            os.path.join(case.home, RAVEL)], capture_output=True).stdout
    expect(fresh == settings_bytes(case), "a fresh process read other bytes")
    expect_off(case, before)
    df = subprocess.run(["/bin/df", "-P", case.home], capture_output=True, text=True).stdout.splitlines()[-1].split()
    mounts = subprocess.run(["/sbin/mount"], capture_output=True, text=True).stdout
    kind = re.search(rf"^{re.escape(df[0])} on {re.escape(df[-1])} \((\w+)", mounts, re.M)
    return (f"device {df[0]} at {df[-1]} ({kind.group(1) if kind else 'unknown type'}), the HOME volume: "
            f"F_FULLFSYNC flushed the temporary file and the folder, and a fresh process read the "
            f"published bytes; the command took {seconds:.2f} s")


def c_native_off_accepted_build(case):
    """The published result resolves as `disabled` with the accepted 2.0 build."""
    with open(BUILDS) as f:
        want = json.load(f)["ravel2"]["binRavelSha256"]["Darwin-arm64"]
    folder = os.path.realpath(ARCHIVES)
    archives = sorted(n for n in os.listdir(folder) if n.endswith("-aarch64-apple-darwin.tar.xz")) \
        if os.path.isdir(folder) else []
    expect(len(archives) == 1, f"expected one darwin archive of the accepted build in {folder}, found {archives}")
    work = os.path.join(case.root, "accepted")
    os.makedirs(os.path.join(work, "archive"))
    subprocess.run(["/usr/bin/tar", "-xJf", os.path.join(folder, archives[0]), "-C",
                    os.path.join(work, "archive")], check=True)
    (top,) = os.listdir(os.path.join(work, "archive"))
    binary = os.path.join(work, "archive", top, "bin", "ravel")
    got = digest_file(binary)
    expect(got == want, f"{binary} is not the accepted 2.0 build: {got} != {want}")
    # A fixture keg whose default skill directory holds one source, so an
    # absent `skills` key resolves a source and the off value does not.
    keg = os.path.join(work, "brew", "Cellar", "ravel@2", "2.0.0")
    skills = os.path.join(work, "brew", "share", "ravel", "skills")
    os.makedirs(os.path.join(keg, "bin"))
    os.makedirs(os.path.join(keg, "share", "ravel"))
    os.makedirs(skills)
    shutil.copy2(binary, os.path.join(keg, "bin", "ravel"))
    save(os.path.join(keg, "share", "ravel", "installation.json"), {"schemaVersion": 1, "skillDirectory": skills})
    write(os.path.join(work, "source", "skills", "probe", "SKILL.md"),
          "---\nname: probe\ndescription: A fixture skill.\n---\n")
    os.symlink(os.path.join(work, "source"), os.path.join(skills, "fixture"))
    os.makedirs(os.path.join(work, "tmp"))

    def resolve():
        result = subprocess.run(["/usr/bin/env", "-i", f"HOME={case.home}", f"TMPDIR={work}/tmp", "PATH=/usr/bin:/bin",
                                 os.path.join(keg, "bin", "ravel"), "skills", "resolve", "--json"],
                                capture_output=True, text=True)
        try:
            return result.returncode, json.loads(result.stdout)
        except ValueError:
            return result.returncode, result.stdout + result.stderr

    # The fixture HOME's member `policy` is not a Ravel 2.0 setting. Settings
    # that the build accepts hold two of its own members instead.
    save_json(os.path.join(case.home, RAVEL), {"timeoutMs": 600000, "configEnv": ["OWNER_SWITCH_FIXTURE"]})
    before = resolve()
    expect(before[0] == 0 and isinstance(before[1], dict) and before[1].get("directories"),
           f"the absent key did not resolve the fixture source: {before}")
    expect_exit(case.native_off(), 0, r"\(2 other keys kept\)")
    after = resolve()
    expect(after == (0, {"schemaVersion": 1, "status": "disabled"}), f"the result resolves as {after}")
    os.remove(os.path.join(case.home, RAVEL))
    expect_exit(case.native_off(), 0, r"\(the file was absent\)")
    absent = resolve()
    expect(absent == (0, {"schemaVersion": 1, "status": "disabled"}), f"the new file resolves as {absent}")
    return (f"build {want[:12]}: the absent key resolved {len(before[1]['directories'])} directory; "
            "after native-off, both the kept members and the new file resolve as disabled")


def native_off_controls() -> list:
    no_file, no_key = {"ravel": "absent-file"}, {"ravel": "absent-key"}
    controls = [
        ("native-off-absent-file", c_native_off_absent_file, no_file),
        ("native-off-absent-key", c_native_off_absent_key, no_key),
        ("native-off-existing-off", c_native_off_existing, {}),
    ]
    refusals = {
        "malformed": b'{"policy": ',
        "repeated-member": b'{"policy": "a", "policy": "b"}\n',
        "non-finite": b'{"policy": NaN}\n',
        "array": b"[]\n",
        "null-skills": b'{"policy": "config.rhai", "skills": null}\n',
        "other-skills": b'{"skills": {"schemaVersion": 1, "sources": [{"id": "x", "path": "/x"}]}}\n',
        "link": "link",
    }
    controls += [(f"native-off-refuses-{name}", native_off_refusal(body), no_key) for name, body in refusals.items()]
    controls += [
        ("native-off-other-ravel", c_native_off_other_ravel, no_key),
        ("native-off-ravel-ancestor", c_native_off_ancestor, no_key),
        ("native-off-other-tool", c_native_off_other_tool, no_key),
        ("native-off-ravel-title", c_native_off_ravel_title, no_key),
        ("native-off-after-switch-stop", c_native_off_after_switch_stop, dict(no_key, rehearsed=False)),
        ("native-off-print-command", c_native_off_print_command, no_key),
        ("native-off-create-fails", c_native_off_create_fails, no_key),
        ("native-off-replace-fails", c_native_off_replace_fails, no_key),
    ]
    for ravel in ("absent-file", "absent-key"):
        for point in NATIVE_OFF_POINTS:
            controls.append((f"native-off-{ravel}-crash-{point.split(':')[1]}", native_off_crash(point),
                             {"ravel": ravel}))
    controls += [
        ("native-off-reader", c_native_off_reader, no_key),
        ("native-off-persistence", c_native_off_persistence, no_key),
        ("native-off-accepted-build", c_native_off_accepted_build, no_key),
    ]
    return controls


def cmd_rehearse(args) -> int:
    r = Rehearsal(args.out, args.program, args.mac_check)
    for name, fn, kw in all_controls(r):
        if args.only and not re.search(args.only, name):
            continue
        r.control(name, fn, **kw)
    # "Test seams": a passing record names the Claude Code version whose
    # writes its rerun measured, so a record without the rerun is partial.
    rerun = None if args.skip_settings_writes else settings_writes(r.out)
    controls_passed = all(x["passed"] for x in r.results)
    rerun_passed = rerun is None or rerun["passed"]
    passed = controls_passed and rerun is not None and rerun["passed"] and not args.only
    record = {
        "schema": "owner-switch-rehearsal/1",
        "passed": passed,
        "plan": r.plan, "plan_digest": r.plan_digest,
        "program": r.program, "program_digest": r.program_digest,
        "python": platform.python_version(), "macos": platform.mac_ver()[0],
        "controls": len(r.results), "failed": [x["control"] for x in r.results if not x["passed"]],
        "results": r.results,
        "settings_writes": rerun,
        "partial": bool(args.only) or rerun is None,
    }
    save(os.path.join(r.out, "record.json"), record)
    print(f"{len(r.results) - len(record['failed'])}/{len(r.results)} controls passed; record {r.out}/record.json")
    if rerun is not None:
        print(f"settings-writes.sh rerun with claude {rerun['claude_version']}: "
              f"{'passed' if rerun['passed'] else 'FAILED: ' + rerun['detail']}")
    else:
        print("settings-writes.sh did not run, so the record is partial and admits nothing")
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
    p.add_argument("--mac-check", default="", help=f"the owner's mac-check command (default {MAC_CHECK})")
    p.add_argument("--skip-settings-writes", action="store_true",
                   help="do not rerun settings-writes.sh with the real claude; the record is then partial")
    args = parser.parse_args()
    return cmd_rehearse(args)


if __name__ == "__main__":
    sys.exit(main())
