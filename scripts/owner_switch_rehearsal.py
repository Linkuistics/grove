#!/usr/bin/env python3
"""Rehearse the owner switch program on a fixture plan in disposable layouts.

docs/specs/owner-skill-delivery.md, "Test seams", owns the controls. Each
control builds a new disposable HOME under OUT, runs scripts/owner_switch.py
under a fixture harness process, and checks the layout, the program's records
and its output. A hook acts at the program's named points: it crashes the
program, makes it fail, writes as another program or launches a process. The
real HOME is never changed, and the rehearsal reads only its `mac-check`
command and that command's folder. The dry run of the real plan in a
disposable copy of HOME is another instrument, scripts/owner_switch_dry_run.py
("The rehearsed plan"), and the dry-run controls below check it.

Usage:
    python3 -I owner_switch_rehearsal.py rehearse --out OUT [--program PATH] [--only REGEX]
                                                  [--mac-check PATH] [--dry-run PATH]

The record OUT/record.json names the digests of the plan and the program and
the result of each control. For each run of a control, it counts the Claude
Code commands that the run ran. No program reads the record: the switch
consumes prepared values, not a passing record. OUT/plan.json is the fixture
plan.

Each run finds stub `claude` and `brew` commands first on PATH. The stub
`claude` keeps its plugin state in the disposable HOME, and it writes
settings.json as Claude Code 2.1.319 does: one read, then a new file renamed
onto the path. The point "stub:<command>:read" lies between that read and
the rename.

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

Each case starts before the hooks installation and computes the plan fields
that depend on its HOME, because installed_plugins.json names absolute paths:
the write set with its originals, the inventories, the command inputs, the
assertions and the restore order.

The switch controls run `attempt`, `switch` and `restore`. They cover the full
switch with a later restore through the note's literal command and its
recovery backup, and a stop before the backup's publication. They also cover
an attempt mismatch, a later restore that drift refuses, and refusals before
any write. A caught failure or a crash at each named point completes them.
The hook action "fail" makes the hook exit 1, so the program meets a caught
failure at that point. Further switch controls cover ACLs and attributes
through backup and restore, failed reads and changed copies before
publication, and changed backup records. They cover interrupted and failed
restores, the off-value exception, an interrupted later restore and the
owner's restore after finish. They also cover Claude Code command failures,
an unwritable manual, probe-owned services and changes during the probes.
The hook action "append-glob" changes each file that a pattern names. The
fixture flag "probe_owned" makes the switched probe leave a service running:
"live" in a new session, "reparent" reparented to launchd, "moved" and
"reused" with a record whose executable or creation time differs, and
"journal" named only in the probe's journal by a probe that dies before its
result line. The off-value controls make their backup through the program's own
`create_backup`, because the switch refuses without the off value.

The gate controls run the switch with the evidence of steps 4 to 7 and the
baseline of step 3. They cover an eleventh source, a one-byte change of a
source outside and inside Grove's keg with its staging trials, a failed or
missing trial, a missing staging HOME and a source that changes during the
probes. They cover Grove's tap, link and version, and a Grove source outside
its private payload. They cover the skill formulae: a missing or undeclared
formula, an unlinked keg, a mise version that differs from the linked keg,
and an earlier or a later revision, each with and without the revision in
mise's report. A revision that mise omits passes when Homebrew links the
accepted revision. They cover the reader evidence before and during the
probes, and the baseline's anomaly rows. The plan controls cover a plan changed after its admission, paths that
leave HOME, each missing field or check, each removed field and each process,
MCP or job pattern, and a held Claude Code version that differs. The receipt
control reads the activated configuration, the sources, the readers and the
retained resources.

The process controls cover the narrow process inspection. One control passes
each measured and constructed launch form through the program's recognise().
The others run real processes: APFS clones of this interpreter in
OUT/fixture-bin under the names and paths of the four tools, of `node`,
`python3`, `caffeinate` and `helper`, with Node entry scripts, an npm bin link
and a wrapper script. They cover native and versioned executables, Node
launchers, reparented Codex daemons, unrelated arguments, review forms, a
deleted executable, exact ancestors, a descendant harness, a caffeinate
child, an MCP server and its tool child, and probe-owned records with a
changed creation time or executable. Through the switch, a tool or a
descendant harness before the switch refuses it, and one that starts after
the backup leaves recovery incomplete until the owner ends it and restores.
Each control checks that the fixture processes still run, because the
program signals none. OWNER_SWITCH_REHEARSAL_SCOPE limits recognition to that
folder in every run, because the machine's own tool processes share the
process table.

The native-off controls run `native-off` without a plan or a switch backup.
They cover an absent file, an absent key with its metadata, an existing off
value and each refused form. They cover another Ravel process, a process
titled ravel, a Ravel ancestor, other tools' processes, which do not stop
it, a refusal of the switch before its backup and --print-command. They
also cover a failed creation, a failed rename and a crash at each named
point. A reader polls the file during repeated publications. The
persistence control fails unless OUT is on the volume of the account's HOME.
The control of the accepted 2.0 build needs its darwin archive in
target/ravel-archives-2.0, and the digest of scripts/harness-trials/builds.json.

The status controls run the real `mac-check` command for the switch's
area, with HOME set to the case's HOME. That HOME links the checker's folder
and holds copies of the case's plan and the program in <state>/prepared, and
the stub `brew` comes first on PATH. That command file is the only file of
the account's HOME that the rehearsal reads. They cover an unprepared switch,
native and partial layouts, and unrelated edits after success that make
restore ineligible. They also cover a changed setting, entry, source or
reader after success, a stale receipt, a crash before the receipt, step 8's
`mac-check` during the switch, a pending manual entry and an interrupted
later restore. The purge controls cover a clean report with a reader marker,
a changed cache version and registration, and an unverified switch.

The dry-run controls prepare a fixture plan and program in the case's HOME,
which stands for the real HOME. The plan's probes run a probe program with a
local helper, and its `mac-check` is a shell script that runs a checker under
$HOME, in the form of the owner's. The case's Ravel settings have no "skills"
key, so the dry run's native-off writes the off value. Each control runs
scripts/owner_switch_dry_run.py into the case's folder with the stubs first
on PATH and OWNER_SWITCH_FIXTURE_PREFIX naming the case's prefix. They cover
the full run with its comparison, a subject that changes during the run, a
write into the source HOME and a failed probe. Against the case's HOME
afterwards, they cover equal content at a declared Grove keg prefix, the same
move without its declaration, a released source outside its private
payload, a changed helper at the same path, a changed
plan value, a changed source and a changed tool version. A last control
covers refusals before the run.
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
# Both marketplace files of Grove's keg name the private writing payload
# ("Hook adapters" of docs/specs/codex-skill-provisioning.md).
PLUGIN_SOURCE = "libexec/skill-sources/writing/craft/technical-communication"
PLUGIN_CACHE = ".claude/plugins/cache/writing-hooks/technical-communication-hooks/unknown"
# The fixture kegs of the ten sources: formula -> (full package version, source IDs).
# The four other formulae carry revision 1, as the packaging-only acceptance
# of docs/owner-skill-delivery-evidence.md published them.
FORMULA_OF_GROVE = "linkuistics/taps/grove"
KEGS = {
    "grove": ("1", ["design", "engineering", "grove-core", "requirements", "research", "writing"]),
    "personal": ("2026-11-06_1", ["personal"]),
    "testanyware": ("2.8.0_1", ["testanyware"]),
    "superpowers": ("7.0.0_1", ["superpowers"]),
    "quint-llm-kit": ("1.0.0_1", ["quint"]),
}
SOURCES = "libexec/skill-sources"
ALIASES = "share/ravel/skills"
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


def keg_payload(prefix: str, name: str, version: str, sid: str) -> str:
    """Add one source to a keg in the projected layout, and link its alias into the prefix.

    The payload is libexec/skill-sources/<id>, the keg's share/ravel/skills/<id>
    is a relative directory symlink to it, and the prefix's entry links that
    alias, as `brew link` does ("The keg layout" of
    docs/specs/codex-skill-provisioning.md). The payload's path is returned.
    """
    keg = os.path.join(prefix, "Cellar", name, version)
    payload = os.path.join(keg, SOURCES, sid)
    os.makedirs(payload, exist_ok=True)
    os.makedirs(os.path.join(keg, ALIASES), exist_ok=True)
    os.symlink(f"../../../{SOURCES}/{sid}", os.path.join(keg, ALIASES, sid))
    os.makedirs(os.path.join(prefix, ALIASES), exist_ok=True)
    os.symlink(f"../../../Cellar/{name}/{version}/{ALIASES}/{sid}", os.path.join(prefix, ALIASES, sid))
    return payload


def build_prefix(prefix: str) -> str:
    """A Homebrew prefix with the five skill formulae's kegs, linked as `brew link` links them.

    Grove's keg holds the writing-hooks marketplace, `bin/grove` and six
    sources. Each source is a private payload with a public alias. `bin/ravel`
    stands in for the promoted Ravel, and `var/homebrew/linked` records each
    linked keg, as Homebrew does.
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
            write(os.path.join(keg_payload(prefix, name, version, sid), f"skills/{sid}-skill/SKILL.md"),
                  f"---\nname: {sid}-skill\n---\nThe fixture source {sid}.\n")
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
    disabled, enabled or absent. The defaults are the layout after native-off and the hooks installation."""
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
# The switch's write set, with the values that it sets, and its restore order: the Ravel settings first.
SWITCH_WRITES = ((".claude/skills/grove", "entry", None), (".codex/skills/grove", "entry", None),
                 (".claude/settings.json", "file", [{"key": SUPERSEDED_PLUGIN, "value": False},
                                                    {"key": HOOKS_PLUGIN, "value": True}]),
                 (KNOWN_MARKETPLACES, "file", None), (INSTALLED_PLUGINS, "file", None),
                 (".config/ravel/settings.json", "file", [{"key": ["skills"], "value": SWITCHED}]))
SWITCH_RESTORE = (".config/ravel/settings.json", KNOWN_MARKETPLACES, INSTALLED_PLUGINS, ".claude/settings.json",
                  ".claude/skills/grove", ".codex/skills/grove")


def home_fields(program: str, home: str) -> dict:
    """The plan fields that depend on HOME: the whole write set with its originals, the expected
    inventories, the command inputs by content, the delivery assertions and the restore order.

    installed_plugins.json names absolute paths, so each case computes these fields for its own HOME.
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

    def write_of(rel: str, kind: str, values) -> dict:
        w = {"path": rel, "type": kind, "original": original(rel)}
        if values:
            w["set"] = values
        return w

    return {
        "writes": [write_of(rel, kind, values) for rel, kind, values in SWITCH_WRITES],
        "restore": list(SWITCH_RESTORE),
        "inventory": {"before": [root(".claude/skills"), root(".codex/skills")],
                      "after": [root(".claude/skills", ["grove"]), root(".codex/skills", ["grove"])]},
        "inputs": [{"role": "program", "path": "$program", "digest": digest_file(program)},
                   {"role": "fixture commands", "path": SELF, "digest": digest_file(SELF)}],
        "assertions": [{"path": ".config/ravel/settings.json", "key": ["skills"], "value": SWITCHED},
                       {"path": ".claude/settings.json", "key": SUPERSEDED_PLUGIN, "value": False},
                       {"path": ".claude/settings.json", "key": HOOKS_PLUGIN, "value": True},
                       {"path": ".claude/skills/grove", "absent": True},
                       {"path": ".codex/skills/grove", "absent": True}],
        # Native delivery: the off value, the superseded plugin enabled and the hooks plugin not enabled.
        # The native inventory holds the two earlier entries.
        "native_assertions": [{"path": ".config/ravel/settings.json", "key": ["skills"], "value": OFF},
                              {"path": ".claude/settings.json", "key": SUPERSEDED_PLUGIN, "value": True},
                              {"path": ".claude/settings.json", "key": HOOKS_PLUGIN,
                               "values": [False, {"$absent": True}]}],
        "tools": {"claude": FIXTURE_CLAUDE},
        # The case's own fixture Ravel stands for the original driver's keg, so the purge
        # report reads no file outside the case.
        "retained": [{"item": "the original driver's Ravel keg",
                      "path": os.path.join(os.path.dirname(home), "prefix/bin/ravel"),
                      "condition": ["its last driver ends"]}],
    }


def build_plan(program: str, out: str) -> str:
    """The fixture plan, with the fields of a HOME before the hooks installation, as each case starts."""
    # The retained staging HOME, with its own .codex tree, and the candidate keg.
    os.makedirs(os.path.join(out, "staging-home", ".codex"), exist_ok=True)
    os.makedirs(os.path.join(out, "candidate-keg"), exist_ok=True)
    probe = os.path.join(out, "plan-home")
    prefix = os.path.join(out, "plan-prefix")
    for folder in (probe, prefix):
        if os.path.exists(folder):
            shutil.rmtree(folder)
    build_home(probe, build_prefix(prefix), hooks="absent")
    sources = {sid: {"formula": f"linkuistics/taps/{name}",
                     "resourceDigest": source_digest(os.path.join(prefix, "share/ravel/skills", sid))}
               for name, (_, ids) in KEGS.items() for sid in ids}
    plan = {
        "schema": "owner-switch-plan/2",
        "state": STATE,
        "manual": MANUAL,
        **home_fields(program, probe),
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
    shutil.rmtree(probe)
    shutil.rmtree(prefix)
    path = os.path.join(out, "plan.json")
    with open(path, "w") as f:
        json.dump(plan, f, indent=2)
        f.write("\n")
    return path


# --- fixture commands, the hook and the harness ----------------------------------

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
    owned_mode = load(os.path.join(root, "control.json"), {}).get("probe_owned")
    if name == "probe-switched" and owned_mode:
        # A service that the probe proves it created and leaves running: "Probe process lifetime".
        # "reparent" starts it through an intermediate that exits, so its parent becomes launchd.
        # "moved" and "reused" report a record whose executable or creation time differs, as for a
        # reused PID. "journal" writes the record only to the journal, and the probe dies without
        # its result line.
        if owned_mode == "reparent":
            code = ("import subprocess, sys; p = subprocess.Popen([sys.executable, '-I', '-c', sys.argv[1]], "
                    "start_new_session=True, stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL, "
                    "stderr=subprocess.DEVNULL); print(p.pid)")
            pid = int(subprocess.run([PY, "-I", "-c", code, SLEEP], capture_output=True, text=True,
                                     check=True).stdout)
        else:
            pid = subprocess.Popen([PY, "-I", "-c", SLEEP], start_new_session=True, stdin=subprocess.DEVNULL,
                                   stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL).pid
        record_pid(root, pid)
        with open(os.path.join(root, "probe-owned"), "w") as f:
            f.write(f"{pid}\n")
        start = program_module(os.path.join(HERE, "owner_switch.py")).process_start(pid)
        rec = {"pid": pid, "start": start, "executable": os.path.realpath(PY), "home": home}
        if owned_mode == "moved":
            rec["executable"] = "/nonexistent/python3"
        elif owned_mode == "reused":
            # A creation time after the probe began but not the process's own, as for a reused PID.
            seconds, micros = start.split(".")
            rec["start"] = f"{int(seconds) + 1}.{micros}"
        if owned_mode == "journal":
            with open(os.environ["OWNER_SWITCH_PROBE_JOURNAL"], "a") as f:
                f.write(json.dumps(rec) + "\n")
            print("probe-switched: died before its result line")
            return 1
        print("owner-switch-result: " + json.dumps({"owned": [rec]}))
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
        elif do == "touch":
            write(path, action.get("text", ""))
        elif do == "append-glob":
            # Change each existing file that the pattern names, such as a copy inside an unpublished backup.
            for p in glob.glob(path):
                with open(p, "a") as f:
                    f.write("changed\n")
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
        elif do == "eleventh-source":
            add_eleventh_source(os.path.join(root, "prefix"))
        else:
            raise SystemExit(f"unknown action {do}")
    return 99 if crash else (1 if failing else 0)


def fixture_prefix() -> str:
    """The case's fixture Homebrew prefix. A dry run names it, because its disposable HOME lies elsewhere."""
    return os.environ.get("OWNER_SWITCH_FIXTURE_PREFIX") or os.path.join(case_root(), "prefix")


def cmd_stub_brew(argv: list) -> int:
    prefix = fixture_prefix()
    if argv == ["--version"]:
        print("Homebrew 7.0.12 (fixture)")
        return 0
    if argv == ["--prefix"]:
        print(prefix)
        return 0
    if argv == ["--cellar"]:
        print(os.path.join(prefix, "Cellar"))
        return 0
    names = {f"linkuistics/taps/{n}": n for n in KEGS}
    if argv[:2] == ["info", "--json=v2"] and len(argv) == 3 and argv[2] in names:
        # The fields of `brew info --json=v2` that Homebrew 7.0.12 gave on
        # 2026-11-07, with the full package version as each keg's name.
        name = names[argv[2]]
        control = load(os.path.join(case_root(), "control.json"), {})
        current = control.get("tap_grove", "1") if name == "grove" else KEGS[name][0]
        stable, _, revision = current.partition("_")
        linked = os.path.join(prefix, "var/homebrew/linked", name)
        cellar = os.path.join(prefix, "Cellar", name)
        print(json.dumps({"formulae": [{
            "name": name, "full_name": argv[2], "tap": "linkuistics/taps",
            "versions": {"stable": stable, "head": None, "bottle": False},
            "revision": int(revision or 0),
            "linked_keg": os.path.basename(os.readlink(linked)) if os.path.islink(linked) else None,
            "installed": [{"version": v} for v in sorted(os.listdir(cellar))] if os.path.isdir(cellar) else [],
        }], "casks": []}))
        return 0
    print(f"stub brew: unsupported {argv}", file=sys.stderr)
    return 2


def cmd_stub_mise(argv: list) -> int:
    """`mise -C HOME bootstrap packages status --json`, in the shape of mise 2026.11.2."""
    home = os.environ.get("OWNER_SWITCH_HOME", "")
    if argv == ["--version"]:
        print("2026.11.2 macos-arm64 (fixture)")
        return 0
    if argv != ["-C", home, "bootstrap", "packages", "status", "--json"] or os.environ.get("HOME") != home:
        print(f"stub mise: unsupported {argv} with HOME {os.environ.get('HOME')}", file=sys.stderr)
        return 2
    prefix = fixture_prefix()
    override = load(os.path.join(case_root(), "control.json"), {}).get("mise", {})
    rows = [{"package": "jq", "requested_version": "latest", "desired_state": "present", "state": "installed",
             "installed_version": "1.8.1"}]
    for name in KEGS:
        formula = f"linkuistics/taps/{name}"
        linked = os.path.join(prefix, "var/homebrew/linked", name)
        # mise 2026.11.2 reports the full package version, with its revision.
        row = {"package": formula, "requested_version": "latest", "desired_state": "present",
               "state": "installed",
               "installed_version": os.path.basename(os.readlink(linked)) if os.path.islink(linked) else None}
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
    def __init__(self, rehearsal, name: str, control=None, ravel="off", hooks="disabled"):
        self.r = rehearsal
        self.name = name
        self.root = os.path.join(rehearsal.out, "cases", name)
        if os.path.exists(self.root):
            shutil.rmtree(self.root)
        self.home = os.path.join(self.root, "home")
        self.marketplace = build_prefix(os.path.join(self.root, "prefix"))
        build_home(self.home, self.marketplace, ravel, hooks)
        self.set_control(control or {})
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

    def set_control(self, control: dict) -> None:
        save(os.path.join(self.root, "control.json"), control)
        for name in ("fired.json", "failed.json"):
            p = os.path.join(self.root, name)
            if os.path.exists(p):
                os.remove(p)

    def serial_plan(self, mutate=None) -> str:
        """The fixture plan with the fields of this case's HOME, as `self.serial`."""
        plan = load(self.r.plan, {})
        plan.update(home_fields(self.r.program, self.home))
        if mutate:
            mutate(plan)
        self.serial = os.path.join(self.root, "serial-plan.json")
        save_json(self.serial, plan)
        return self.serial

    def invoke(self, argv: list, via=None, env=None) -> tuple:
        """Run ARGV under the fixture harness process, with the hook, the stubs and the fixture scope.

        A Claude Code location variable, CODEX_HOME and a RAVEL_* name in the
        environment must not reach a command: each fixture command and the
        stub refuse them, and so does a staging launch. VIA names launchers
        that become ancestors of ARGV.
        """
        env = dict(clean_env(), OWNER_SWITCH_REHEARSAL_HOOK=self.r.hook,
                   PATH=self.r.stubs + os.pathsep + os.environ.get("PATH", ""),
                   CLAUDE_CONFIG_DIR=os.path.join(self.root, "elsewhere"),
                   CODEX_HOME=os.path.join(self.root, "elsewhere", ".codex"),
                   RAVEL_SKILL_DIRECTORIES=os.path.join(self.root, "elsewhere"),
                   OWNER_SWITCH_REHEARSAL_SCOPE=self.r.fixture_bin, PYTHONHOME=sys.base_prefix, **(env or {}))
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

    def switch(self, note: dict, via=None, env=None, plan=None) -> tuple:
        return self.invoke([PY, "-I", self.r.program, "switch", "--plan", plan or self.serial, "--home", self.home,
                            "--attempt", note["_path"]], via=via, env=env)

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

    def link_checker(self) -> None:
        lib = os.path.join(self.home, ".local/lib/machine-config")
        if not os.path.lexists(lib):
            os.makedirs(os.path.dirname(lib), exist_ok=True)
            os.symlink(self.r.machine_config, lib)

    def prepare(self, plan=None, program: bool = True) -> None:
        """Copy PLAN, the case's serial plan by default, and the program into <state>/prepared."""
        folder = os.path.join(self.home, STATE, "prepared")
        os.makedirs(folder, exist_ok=True)
        shutil.copyfile(plan or self.serial, os.path.join(folder, "plan.json"))
        if program:
            shutil.copyfile(self.r.program, os.path.join(folder, "owner_switch.py"))

    def mac_check(self) -> tuple:
        """The real `mac-check`, for the switch's area only, with HOME set to this case's HOME.

        The case's HOME links the checker's folder, and `prepare` copies the
        plan and the program into `prepared/`, as in the owner's layout. The
        stub `brew` comes first on PATH, so the status resolves the case's
        fixture sources.
        """
        self.link_checker()
        env = dict(clean_env(), HOME=self.home, PATH=self.r.stubs + os.pathsep + os.environ.get("PATH", ""))
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

    def json(self, rel: str) -> dict:
        return load(os.path.join(self.home, rel), None)

    def snapshot(self) -> dict:
        """Every entry of the HOME, except the owner's prepared copies and the checker's folder.

        `mac-check` reads both. Neither is part of the layout that a refusal leaves unchanged.
        """
        out = {}
        skip = (os.path.join(STATE, "prepared"), ".local/lib")
        for folder, dirs, files in os.walk(self.home):
            for name in sorted(dirs + files):
                p = os.path.join(folder, name)
                rel = os.path.relpath(p, self.home)
                if any(rel == s or rel.startswith(s + "/") for s in skip):
                    continue
                if os.path.isdir(p) and not os.path.islink(p) and any(s.startswith(rel + "/") for s in skip):
                    # A folder above a skipped path.
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


class Rehearsal:
    def __init__(self, out: str, program: str, mac_check: str = "", dry_run: str = ""):
        self.dry_run = os.path.realpath(dry_run or DRY_RUN)
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
        except Exception as e:  # An error in a control fails that control only, and the suite continues.
            detail, ok = f"error: {type(e).__name__}: {e}", False
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


def process_controls() -> list:
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
    ]


# --- the fixture sources and trials ---------------------------------------------------

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
    add_eleventh_source(os.path.join(case.root, "prefix"))


def add_eleventh_source(prefix: str) -> None:
    write(os.path.join(keg_payload(prefix, "extra", "9", "eleventh"), "skills/eleventh/SKILL.md"),
          "---\nname: eleventh\n---\n")


def trials(case) -> list:
    try:
        with open(os.path.join(case.root, "trials.log")) as f:
            return f.read().split()[::2]
    except FileNotFoundError:
        return []


# --- the switch and its restore -------------------------------------------
# docs/specs/owner-skill-delivery.md, "Test seams": backup, caught failures,
# hard interruption, explicit restore, attempt identity and later drift.

# What restore leaves alone: the program's own state, the append-only manual
# and the plugin cache that `claude plugin install` wrote ("Rollback").
SERIAL_SKIP = (STATE, MANUAL, ".claude/plugins/cache/writing-hooks")
SERIAL_ARGS = {"hooks": "absent"}


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
    # Step 6 writes the switched Ravel settings last.
    writes = [p.split(":")[1] for p in case.points() if p.startswith("write:") and p.endswith(":replace")]
    expect(writes and writes[-1] == RAVEL and SETTINGS in writes,
           f"step 6 wrote in this order: {writes}")
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
    expect(sorted(receipt["restore_identities"]) == sorted(SWITCH_RESTORE)
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
    case.serial_plan(lambda plan: plan["inputs"][1].update(digest="0" * 64))
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
    for w in load(case.serial, {})["writes"]:
        if w["path"] in skip:
            continue
        wanted = "absent" if w["original"].get("absent") else w["original"]["identity"]
        got = m.restore_identity(os.path.join(case.home, w["path"]))
        expect(got == wanted, f"{w['path']} has the restore identity {got}, not its original {wanted}")


def expect_backup_complete(case, note: dict) -> None:
    """The published backup holds each original whole, or records its absence."""
    m = case.r.module()
    items = {i["path"]: i for i in load(os.path.join(note["backup"], "manifest.json"), {})["items"]}
    for w in load(case.serial, {})["writes"]:
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
    # No mark on a file that the hooks commands replace: the program refuses that layout before any
    # write (serial-hooks-registry-acl-appears and serial-hooks-registry-attribute-appears).
    marked = {entry: "list", os.path.join(entry, "SKILL.md"): "read", os.path.join(case.home, RAVEL): "read"}
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
    return "ACLs and attributes of an entry folder, its file and the Ravel settings survive backup and restore"


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
        layout = m.Layout(case.home, plan)
        with contextlib.redirect_stdout(io.StringIO()):
            m.backup_published(layout, m.create_backup(layout, plan, m.load_note(note["_path"]), {}))
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


def probe_owned_pid(case) -> int:
    with open(os.path.join(case.root, "probe-owned")) as f:
        return int(f.read())


def serial_probe_owned_ended(mode: str):
    """A probe-owned service that the probe leaves running: the switch ends it and succeeds."""
    def fn(case):
        case.serial_plan()
        note = case.attempt()
        case.set_control({"probe_owned": mode})
        out = expect_exit(case.switch(note), 0, "sent SIGTERM to probe-owned process")
        pid = probe_owned_pid(case)
        expect(not alive(pid), f"the probe-owned service {pid} still runs after the switch")
        expect("probe-owned processes: 1 recorded, none runs" in out, f"no cleanup verdict: {out[-1500:]}")
        expect_switched(case)
        expect(os.path.lexists(receipt_file(case, note)), "the switch wrote no receipt")
        return f"the switch ends a {mode} probe-owned service by its record and verifies its exit"
    return fn


def c_serial_probe_owned_moved(case):
    """A record whose executable differs: the program signals nothing and stops with 4."""
    case.serial_plan()
    before = serial_view(case)
    note = case.attempt()
    case.set_control({"probe_owned": "moved"})
    out = expect_exit(case.switch(note), 4, "remaining probe-owned process")
    pid = probe_owned_pid(case)
    expect(alive(pid), "the program signalled a process whose executable differs from its record")
    expect("sent SIG" not in out, f"the program sent a signal: {out[-1500:]}")
    expect("restore: native delivery verified" not in out and "restored:" not in out,
           f"the switch restored while a probe-owned process ran: {out[-1500:]}")
    expect_switched(case)
    expect(os.path.isdir(note["backup"]) and not os.path.lexists(receipt_file(case, note)),
           "the backup is missing, or a receipt was written")
    case.kill()
    time.sleep(0.3)
    case.set_control({})
    expect_exit(case.literal(note["restore"]), 0, "restore: immediate")
    expect_view(case, before, "after the owner's restore")
    return "a record with another executable authorises no signal, and the switch stops with 4 and no restore"


def c_serial_probe_owned_reused(case):
    """A record whose creation time differs, as for a reused PID: no signal, and the record counts as ended."""
    case.serial_plan()
    note = case.attempt()
    case.set_control({"probe_owned": "reused"})
    out = expect_exit(case.switch(note), 0, "probe-owned processes: 1 recorded, none runs")
    pid = probe_owned_pid(case)
    expect(alive(pid), "the program signalled a reused PID")
    expect("sent SIG" not in out, f"the program sent a signal: {out[-1500:]}")
    case.kill()
    return "a reused PID authorises no signal: the recorded process counts as ended"


def c_serial_probe_owned_journal(case):
    """A probe that dies before its result line: its journal still names its service."""
    case.serial_plan()
    before = serial_view(case)
    note = case.attempt()
    case.set_control({"probe_owned": "journal"})
    out = expect_exit(case.switch(note), 5, "sent SIGTERM to probe-owned process")
    pid = probe_owned_pid(case)
    expect(not alive(pid), f"the journal's service {pid} still runs")
    expect("probe failed: probe-switched" in out, f"the failed probe did not stop the switch: {out[-1500:]}")
    expect_view(case, before, "after the switch restored")
    return "the journal of a probe that died names its service, which the switch ends before it restores"


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
    plan["restore"].reverse()
    plan["restore"] = [RAVEL] + [r for r in plan["restore"] if r != RAVEL]
    data = (json.dumps(plan, indent=2) + "\n").encode()
    with open(paths["plan.json"], "wb") as f:
        f.write(data)
    manifest(lambda body: body["plan"].update(digest=hashlib.sha256(data).hexdigest()))
    refused("another plan than the attempt note names", "after a changed plan with its digest")
    expect_exit(case.literal(note["restore"]), 0, "result: restored")
    expect_view(case, before, "after the restore")
    return "each changed copy, manifest or plan refuses before any write"


# --- status and the purge report ---------------------------------------------------
# docs/specs/owner-skill-delivery.md, "Current health and restore eligibility",
# "Purge" and "Test seams". Each status control reads the real `mac-check`.

def expect_mac_check(case, code: int, status: str, *texts: str) -> str:
    got = case.mac_check()
    expect(got[0] == code and got[1] == status and all(t in got[2] for t in texts),
           f"mac-check reported {got}, not {code} {status} {texts}")
    return got[2]


NATIVE = "Native skill delivery is verified"
SWITCHED_OK = "the switched layout is verified"
NEEDS_RESTORE = "needs its restore"
REVIEW = "needs review"


def status_json(case, plan=None) -> dict:
    """The program's `status --json` for the case, with the stubs on PATH."""
    out = expect_exit(case.invoke([PY, "-I", case.r.program, "status", "--json", "--plan", plan or case.serial,
                                   "--home", case.home]), 0)
    return json.loads(out[:out.rindex("}") + 1][out.index("{"):])


def purge_report(case) -> tuple:
    return case.invoke([PY, "-I", case.r.program, "purge-report", "--plan", case.serial, "--home", case.home])


def switched_case(case) -> dict:
    """Prepare the case's plan, switch, and return the attempt note."""
    case.serial_plan()
    case.prepare()
    note = case.attempt()
    expect_exit(case.switch(note), 0, "result: switched")
    return note


def c_status_not_prepared(case):
    expect_mac_check(case, 0, "OK", "not prepared", "no delivery state is verified")
    os.makedirs(os.path.join(case.home, STATE, "attempts"))
    expect_mac_check(case, 10, "Needs attention", "plan is missing")
    case.serial_plan()
    case.prepare(program=False)
    expect_mac_check(case, 10, "Needs attention", "program is missing")
    case.prepare()
    expect_mac_check(case, 0, "OK", NATIVE)
    return "no prepared plan reports an unprepared switch, never a verified native state"


def c_status_native(case):
    case.serial_plan()
    case.prepare()
    expect_mac_check(case, 0, "OK", NATIVE)
    body = case.json(RAVEL)
    del body["skills"]
    save_json(os.path.join(case.home, RAVEL), body)
    expect_mac_check(case, 10, "Needs attention", REVIEW)
    # A plan whose native assertion accepts the absent key, as before promotion.
    def either(plan):
        plan["native_assertions"][0] = {"path": RAVEL, "key": ["skills"],
                                                  "values": [OFF, {"$absent": True}]}
    case.serial_plan(either)
    case.prepare()
    expect_mac_check(case, 0, "OK", NATIVE)
    return "the native assertions govern native health, and a list of values accepts each of them"


def c_status_partial_native(case):
    case.serial_plan()
    case.prepare()
    os.unlink(os.path.join(case.home, ".codex/skills/grove"))
    expect_mac_check(case, 10, "Needs attention", REVIEW)
    st = status_json(case)
    expect(st["health"] == "review" and any(".codex/skills/grove" in p for p in st["problems"]),
           f"the status of a partial layout is {st}")
    return "a partial layout without an attempt requires review"


def c_status_switched_edits(case):
    note = switched_case(case)
    expect_mac_check(case, 0, "OK", SWITCHED_OK, "restore is eligible")
    write_in_place(os.path.join(case.home, SETTINGS), ["model"], "opus")
    write_in_place(os.path.join(case.home, SETTINGS), ["hooks"], {"Stop": [{"hooks": [
        {"type": "command", "command": "true"}]}]})
    with open(os.path.join(case.home, MANUAL), "a") as f:
        f.write("An unrelated note.\n")
    expect_mac_check(case, 0, "OK", SWITCHED_OK, "ineligible", "differ from the receipt: .claude/settings.json.")
    st = status_json(case)
    expect(st["health"] == "switched" and st["restore"]["differing"] == [SETTINGS],
           f"the status after unrelated edits is {st}")
    middle = serial_view(case)
    expect_exit(case.literal(note["restore"]), 3, "differ from the receipt")
    expect_view(case, middle, "after the refused later restore")
    return "unrelated edits keep delivery health and make whole-file restore ineligible"


def status_delivery_change(change):
    """A changed delivery assertion, entry, source or reader after success: the receipt stays, health needs review."""
    def fn(case):
        note = switched_case(case)
        expect_mac_check(case, 0, "OK", SWITCHED_OK)
        change(case)
        expect(os.path.exists(receipt_file(case, note)), "the change invalidated the receipt")
        expect_mac_check(case, 10, "Needs attention", REVIEW)
        return "a valid receipt alone never shows current success"
    return fn


def change_setting(case):
    write_in_place(os.path.join(case.home, SETTINGS), SUPERSEDED_PLUGIN, True)


def change_entry(case):
    write(os.path.join(case.home, ".claude/skills/extra/SKILL.md"), "---\nname: extra\n---\n")


def change_source(case):
    path = os.path.join(case.root, "prefix/Cellar/grove/1", SOURCES, "design/skills/design-skill/SKILL.md")
    with open(path, "a") as f:
        f.write("x")


def change_reader(case):
    write(os.path.join(case.home, READERS["grove@linkuistics"], ".orphaned_at"), "1792928060164")


def c_status_stale(case):
    switched_case(case)
    stale = os.path.join(case.root, "stale-plan.json")
    plan = load(case.serial, {})
    plan["tools"]["note"] = "a later preparation"
    save_json(stale, plan)
    case.prepare(stale)
    expect_mac_check(case, 10, "Needs attention", REVIEW)
    st = status_json(case, stale)
    expect(any("stale" in p for p in st["problems"]), f"the status of a stale receipt is {st}")
    return "a receipt of another plan than the prepared plan is stale evidence"


def c_status_crash(case):
    case.serial_plan()
    case.prepare()
    note = case.attempt()
    case.set_control({"actions": [{"point": "probes", "do": "crash"}]})
    expect_exit(case.switch(note), 99)
    case.set_control({})
    expect_switched(case)
    expect_mac_check(case, 10, "Needs attention", NEEDS_RESTORE)
    st = status_json(case)
    expect(st["restore"] == {"status": "required", "differing": [], "commands": [note["restore"]]},
           f"the status of a crashed attempt is {st}")
    expect_exit(case.literal(note["restore"]), 0, "result: restored")
    expect_mac_check(case, 0, "OK", NATIVE)
    return "a complete switched layout without its receipt needs restore, not success"


def c_status_in_progress(case):
    case.serial_plan(lambda plan: plan.update(mac_check={
        "name": "mac-check", "argv": [case.r.mac_check, "--only", "Owner skill switch"]}))
    case.prepare()
    case.link_checker()
    note = case.attempt()
    expect_exit(case.switch(note), 0, "result: switched")
    receipt = load(receipt_file(case, note), {})
    output = receipt["mac_check"]["output"]
    expect(SWITCHED_OK in output and "still running" in output,
           f"step 8 read this switch area: {output}")
    expect_mac_check(case, 0, "OK", SWITCHED_OK, "restore is eligible")
    return "step 8's mac-check verifies the running attempt's switched layout"


def c_status_pending_manual(case):
    case.serial_plan()
    case.prepare()
    note = case.attempt()
    case.set_control({"actions": [{"point": "receipt:written", "do": "chmod", "path": MANUAL, "mode": "444"}]})
    expect_exit(case.switch(note), 4, "the manual entry is pending")
    case.set_control({})
    expect_mac_check(case, 10, "Needs attention", NATIVE, "manual entry is pending")
    os.chmod(os.path.join(case.home, MANUAL), 0o644)
    expect_exit(case.literal(note["restore"]), 0, "restored already")
    expect_mac_check(case, 0, "OK", NATIVE)
    return "a restore's pending manual entry is reported beside native delivery"


def c_status_later_restore_interrupted(case):
    note = switched_case(case)
    case.set_control({"actions": [{"point": "restore:.claude/settings.json", "do": "crash"}]})
    out = expect_exit(case.literal(note["restore"]), 99)
    case.set_control({})
    retry = re.search(r"^if this restore is interrupted, repeat under the same quiet interval: (.+)$", out, re.M)
    expect(retry is not None, f"the later restore gave no retry command: {out[-1500:]}")
    expect_mac_check(case, 10, "Needs attention", NEEDS_RESTORE)
    st = status_json(case)
    expect(st["restore"]["commands"] == [retry.group(1)], f"the status names {st['restore']}")
    expect_exit(case.literal(retry.group(1)), 0, "restore: retry")
    expect_mac_check(case, 0, "OK", NATIVE)
    return "an interrupted later restore needs its retry command"


def c_purge_report_clean(case):
    switched_case(case)
    write(os.path.join(case.home, READERS["grove@linkuistics"], ".in_use", "12345"), "")
    out = expect_exit(purge_report(case), 0, "the conditions that the program reads hold")
    for text in ("retained frozen cache version grove@linkuistics", "keeps its recorded identity",
                 "retained superseded plugin registration grove@linkuistics",
                 "the owner establishes: the owner decides that the rollback window is closed",
                 "the owner establishes: its last driver ends", "removable, because this receipt is written"):
        expect(text in out, f"the purge report lacks {text!r}: {out[-2000:]}")
    expect("the owner establishes: each retained resource keeps" not in out,
           "the purge report hands the owner a condition that the program reads")
    return "a reader marker keeps each retained identity, and the owner's conditions are listed"


def purge_changed(change, what: str):
    def fn(case):
        switched_case(case)
        change(case)
        expect_mac_check(case, 0, "OK", SWITCHED_OK)
        expect_exit(purge_report(case), 3, f"purge blocked: the retained {what} .* changed since the receipt")
        return f"a changed {what} blocks purge"
    return fn


def change_cache(case):
    write(os.path.join(case.home, READERS["grove@linkuistics"], ".claude-plugin/plugin.json"), '{"changed": 1}\n')


def change_registration(case):
    path = os.path.join(case.home, INSTALLED_PLUGINS)
    body = case.json(INSTALLED_PLUGINS)
    body["plugins"]["grove@linkuistics"][0]["version"] = "changed"
    save_json(path, body)


def c_purge_report_unverified(case):
    case.serial_plan()
    expect_exit(purge_report(case), 3, "no valid receipt")
    note = switched_case(case)
    change_setting(case)
    out = expect_exit(purge_report(case), 3, "the switch is not verified: its health is review")
    expect(os.path.exists(receipt_file(case, note)), "the purge report changed the receipt")
    return "an unverified switch blocks purge"


def status_controls() -> list:
    return [
        ("status-not-prepared", c_status_not_prepared, SERIAL_ARGS),
        ("status-native", c_status_native, SERIAL_ARGS),
        ("status-partial-native", c_status_partial_native, SERIAL_ARGS),
        ("status-switched-unrelated-edits", c_status_switched_edits, SERIAL_ARGS),
        ("status-switched-setting-changed", status_delivery_change(change_setting), SERIAL_ARGS),
        ("status-switched-entry-added", status_delivery_change(change_entry), SERIAL_ARGS),
        ("status-switched-source-changed", status_delivery_change(change_source), SERIAL_ARGS),
        ("status-switched-reader-orphaned", status_delivery_change(change_reader), SERIAL_ARGS),
        ("status-stale-receipt", c_status_stale, SERIAL_ARGS),
        ("status-crash-before-receipt", c_status_crash, SERIAL_ARGS),
        ("status-step-8-in-progress", c_status_in_progress, SERIAL_ARGS),
        ("status-pending-manual", c_status_pending_manual, SERIAL_ARGS),
        ("status-later-restore-interrupted", c_status_later_restore_interrupted, SERIAL_ARGS),
        ("purge-report-clean", c_purge_report_clean, SERIAL_ARGS),
        ("purge-report-cache-changed", purge_changed(change_cache, "frozen cache version"), SERIAL_ARGS),
        ("purge-report-registration-changed",
         purge_changed(change_registration, "superseded plugin registration"), SERIAL_ARGS),
        ("purge-report-unverified", c_purge_report_unverified, SERIAL_ARGS),
    ]


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
        ("serial-hooks-registry-acl-appears",
         serial_failure("hooks:before:marketplace-add",
                        actions=[{"point": "hooks:before:marketplace-add", "do": "acl", "path": INSTALLED_PLUGINS}],
                        text=".claude/plugins/installed_plugins.json holds an ACL, which `claude plugin marketplace add",
                        check=no_command("plugin marketplace add")), SERIAL_ARGS),
        ("serial-hooks-registry-attribute-appears",
         serial_failure("hooks:before:install", actions=[{"point": "hooks:before:install", "do": "xattr",
                                                          "path": KNOWN_MARKETPLACES}],
                        text=".claude/plugins/known_marketplaces.json holds the extended attribute user.fault, "
                             "which `claude plugin install", check=no_command("plugin install")), SERIAL_ARGS),
        ("serial-manual-unwritable", serial_manual_pending("receipt:written", False), SERIAL_ARGS),
        ("serial-manual-correction-pending", serial_manual_pending("switch:final", True), SERIAL_ARGS),
        ("serial-probe-owned-live", serial_probe_owned_ended("live"), SERIAL_ARGS),
        ("serial-probe-owned-reparented", serial_probe_owned_ended("reparent"), SERIAL_ARGS),
        ("serial-probe-owned-moved", c_serial_probe_owned_moved, SERIAL_ARGS),
        ("serial-probe-owned-reused", c_serial_probe_owned_reused, SERIAL_ARGS),
        ("serial-probe-owned-journal", c_serial_probe_owned_journal, SERIAL_ARGS),
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
    case.serial_plan()
    note = case.attempt()
    before = settings_bytes(case)
    expect_exit(case.switch(note), 3, "does not hold the off value; run native-off first")
    expect(settings_bytes(case) == before and not os.path.lexists(note["backup"]),
           "the refused switch changed the settings or published a backup")
    expect_exit(case.literal(note["native_off"]), 0, "result: off value in place")
    expect_off(case, before)
    return "the switch refused before any backup, and the note's native-off then wrote the off value"


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
    write(os.path.join(keg_payload(os.path.join(work, "brew"), "fixture", "1", "fixture"), "skills", "probe",
                       "SKILL.md"), "---\nname: probe\ndescription: A fixture skill.\n---\n")
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
        ("native-off-after-switch-stop", c_native_off_after_switch_stop, no_key),
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


# --- the gates, the baseline, the plan and the processes through the switch ---------
# docs/specs/owner-skill-delivery.md, "Test seams": source controls, the narrow
# process inspection and the plan's completeness. Each refusal happens before
# any switch-set write.

def switch_refused(case, text: str, mutate=None, env=None, via=None) -> str:
    """The switch refuses with exit 3 and TEXT, publishes no backup and changes no path."""
    case.serial_plan(mutate)
    note = case.attempt()
    before = serial_view(case)
    out = expect_exit(case.switch(note, env=env, via=via), 3, text)
    expect_view(case, before, "after the refusal")
    expect(not os.path.lexists(note["backup"]), "a refusal published a backup")
    expect("switch:final" not in case.points(), "a refusal reached the end of the switch")
    return out


def switched(case, env=None, via=None) -> tuple:
    """A complete switch: its attempt note and its output."""
    case.serial_plan()
    note = case.attempt()
    out = expect_exit(case.switch(note, env=env, via=via), 0, "result: switched")
    return note, out


def backup_evidence(note: dict, step: str) -> list:
    return load(os.path.join(note["backup"], "manifest.json"), {})["evidence"][step]


def plan_refused(case, label: str, mutate, text: str) -> None:
    """The switch refuses a changed plan at its loader, before its first hook point."""
    case.serial_plan()
    note = case.attempt()
    bad = load(case.serial, {})
    mutate(bad)
    other = os.path.join(case.root, f"plan-{label}.json")
    save_json(other, bad)
    before = serial_view(case)
    mark = len(case.points())
    expect_exit(case.switch(note, plan=other), 3, text)
    expect_view(case, before, f"after the plan {label}")
    expect(not os.path.lexists(note["backup"]), f"the plan {label} published a backup")
    expect(case.points()[mark:] == [], f"the plan {label} reached a hook point: {case.points()[mark:][:5]}")


def c_gates_evidence(case):
    note, out = switched(case)
    expect("compare with finish's own skill listing: claude | built-in | debug" in out,
           "step 3 did not print the built-in row for finish's listing")
    four = backup_evidence(note, "4")[0]["result"]
    expect(four["admitted"] == {"grove_version": "1", "grove_binary_version": "1", "grove_revision": 0,
                                "digests": {}} and four["reran"] == [],
           f"step 4 admitted {four.get('admitted')}")
    six = backup_evidence(note, "6")[0]["result"]
    expect(sorted(six["sources"]) == sorted(load(case.r.plan, {})["accepted"]["sources"]), "step 6 kept other sources")
    readers = {r["plugin"]: r for r in backup_evidence(note, "7")[0]["result"]["readers"]}
    expect(sorted(readers) == sorted(READERS) and not any(r["orphaned"] for r in readers.values()),
           f"step 7 recorded {sorted(readers)}")
    expect(readers["grove@linkuistics"]["path"] == os.path.join(case.home, READERS["grove@linkuistics"]),
           "step 7 did not name the registered version directory")
    expect(trials(case) == [], f"step 4 ran trials {trials(case)} with each digest accepted")


def c_eleventh_source(case):
    eleventh_source(case)
    switch_refused(case, "step 6: the resolved sources are not the accepted ten: extra eleventh")


def c_one_byte_other(case):
    one_byte(case, "superpowers")
    switch_refused(case, "step 6: superpowers gives [0-9a-f]{64}, not its accepted")
    expect(trials(case) == [], "a source outside Grove's keg made step 4 rerun trials")


def c_one_byte_grove(case):
    one_byte(case, "design")
    digest = source_digest(keg_source(case, "design"))
    note, out = switched(case)
    expect("the staging trials of design must run again" in out, "step 4 did not name the changed Grove source")
    expect(trials(case) == ["design"], f"step 4 ran the trials {trials(case)}, not those of design")
    four = backup_evidence(note, "4")[0]["result"]
    expect(four["reran"] == ["design"] and four["admitted"]["digests"] == {"design": digest},
           f"step 4 admitted {four['admitted']}")


def c_trial_fails(case):
    one_byte(case, "design")
    case.set_control({"fail": {"trial": 1}})
    switch_refused(case, "step 4: the staging trial trial-design of design failed")


def c_no_trial(case):
    one_byte(case, "writing")
    switch_refused(case, "step 4: the plan names no staging trial for writing",
                   mutate=lambda plan: plan["accepted"]["trials"].pop("writing"))


def c_staging_missing(case):
    one_byte(case, "design")
    switch_refused(case, "the retained staging HOME .* is missing",
                   mutate=lambda plan: plan["accepted"]["staging"].update(
                       home=os.path.join(case.root, "no-staging-home")))
    expect(trials(case) == [], f"step 4 ran trials {trials(case)} without the staging HOME")


def c_tap_newer(case):
    case.set_control({"tap_grove": "2"})
    switch_refused(case, "the tap's current version 2 is not installed")


def c_linked_older(case):
    """The tap's current version 2 is installed, and Homebrew still links version 1."""
    prefix = os.path.join(case.root, "prefix")
    shutil.copytree(os.path.join(prefix, "Cellar/grove/1"), os.path.join(prefix, "Cellar/grove/2"), symlinks=True)
    case.set_control({"tap_grove": "2"})
    switch_refused(case, "the linked keg is 1, not the tap's current version 2")


def c_grove_source_outside_payload(case):
    """Grove's keg holds design as a real directory at its alias, as the earlier layout did."""
    alias = os.path.join(case.root, "prefix/Cellar/grove/1", ALIASES, "design")
    payload = os.path.realpath(alias)
    os.remove(alias)
    shutil.copytree(payload, alias, symlinks=True)
    switch_refused(case, f"step 4: the source design comes from .*/{ALIASES}/design, not from "
                         f".*/{SOURCES}/design")


def c_opt_elsewhere(case):
    prefix = os.path.join(case.root, "prefix")
    shutil.copytree(os.path.join(prefix, "Cellar/grove/1"), os.path.join(prefix, "Cellar/grove/0"), symlinks=True)
    os.remove(os.path.join(prefix, "opt/grove"))
    os.symlink("../Cellar/grove/0", os.path.join(prefix, "opt/grove"))
    switch_refused(case, "opt/grove reaches .*Cellar/grove/0, not the keg")


def c_grove_version(case):
    write_script(os.path.join(case.root, "prefix/Cellar/grove/1/bin/grove"), "#!/bin/sh\necho 'grove 0'\n")
    switch_refused(case, "printed 'grove 0', not 'grove 1'")


def c_grove_revision(case):
    """The tap revises Grove: step 4 admits the package revision apart from the binary version."""
    relink_formula(case, "grove", "1_1")
    case.set_control({"tap_grove": "1_1"})
    note, _ = switched(case)
    four = backup_evidence(note, "4")[0]["result"]
    expect(four["admitted"] == {"grove_version": "1_1", "grove_binary_version": "1", "grove_revision": 1,
                                "digests": {}}, f"step 4 admitted {four.get('admitted')}")
    five = backup_evidence(note, "5")[0]["result"]
    expect(five["formulae"].get("linkuistics/taps/grove") == "1_1", f"step 5 accepted {five.get('formulae')}")


def c_formula(override: dict, text: str):
    def fn(case):
        case.set_control({"mise": override})
        switch_refused(case, text)
    return fn


def relink_formula(case, name: str, version: str) -> None:
    """Install an equal copy of a formula's keg at VERSION and link it, as `brew upgrade` leaves it.

    The earlier keg stays installed and unlinked. Each source keeps its
    bytes, so only the package version differs.
    """
    prefix = os.path.join(case.root, "prefix")
    old = os.path.join(prefix, "Cellar", name, KEGS[name][0])
    subprocess.run(["/bin/cp", "-pPR", old, os.path.join(prefix, "Cellar", name, version)], check=True)
    links = {os.path.join(prefix, "opt", name): f"../Cellar/{name}/{version}",
             os.path.join(prefix, "var/homebrew/linked", name): f"../../../Cellar/{name}/{version}"}
    for sid in KEGS[name][1]:
        links[os.path.join(prefix, ALIASES, sid)] = f"../../../Cellar/{name}/{version}/{ALIASES}/{sid}"
    for link, target in links.items():
        os.remove(link)
        os.symlink(target, link)


def c_formula_relinked(name: str, version: str, text: str, override=None):
    """Homebrew links another full package version of NAME, with equal source bytes."""
    def fn(case):
        relink_formula(case, name, version)
        if override:
            case.set_control({"mise": override})
        switch_refused(case, text)
        expect(trials(case) == [], "a formula outside Grove's keg made step 4 rerun trials")
    return fn


def c_formula_revision_omitted(case):
    """mise omits the revision, and Homebrew links the accepted revision: step 5 passes."""
    case.set_control({"mise": {"linkuistics/taps/personal": {"installed_version": "2026-11-06"}}})
    note, out = switched(case)
    five = backup_evidence(note, "5")[0]["result"]
    expect(five["formulae"]["linkuistics/taps/personal"] == "2026-11-06_1",
           f"step 5 admitted {five['formulae']}")
    expect("linkuistics/taps/personal is installed and linked at its accepted version 2026-11-06_1" in out,
           "step 5 did not name the accepted revision")


def c_formula_unlinked(case):
    os.remove(os.path.join(case.root, "prefix/var/homebrew/linked/personal"))
    case.set_control({"mise": {"linkuistics/taps/personal": {"installed_version": "2026-11-06_1"}}})
    switch_refused(case, "step 5: linkuistics/taps/personal is installed, and Homebrew links no keg of it "
                         r"\(installed: 2026-11-06_1\)")


def c_reader_orphaned(case):
    write(os.path.join(case.home, READERS["grove@linkuistics"], ".orphaned_at"), "1792928060164")
    switch_refused(case, "step 7: the version directory .* of grove@linkuistics holds .orphaned_at")


def c_reader_missing(case):
    shutil.rmtree(os.path.join(case.home, READERS["context7@claude-plugins-official"]))
    switch_refused(case, "step 7: the version directory .* of context7@claude-plugins-official is missing")


def c_reader_unregistered(case):
    old = os.path.join(case.home, ".claude/plugins/cache/linkuistics/grove/99999999")
    write(os.path.join(old, "skills/grove-finish/SKILL.md"), "---\nname: grove-finish\n---\n")
    switch_refused(case, "which finish read, has no registration", env={"OWNER_SWITCH_READERS": old})


def c_reader_named(case):
    read = os.path.join(case.home, READERS["grove@linkuistics"], ".claude-plugin")
    note, _ = switched(case, env={"OWNER_SWITCH_READERS": read})
    named = [r for r in backup_evidence(note, "7")[0]["result"]["readers"] if r["reason"] == "read"]
    expect(len(named) == 1 and named[0]["read"] == read and named[0]["plugin"] == "grove@linkuistics",
           f"step 7 did not record the directory that finish read: {named}")


BASELINE_ROW = "control\t~/scratch\tunknown\tskill\tunknown\t0123456789abcdef\t1\tunknown\t-\t-\tUNCLASSIFIED"


def c_baseline_anomaly(case):
    case.set_control({"baseline_rows": [BASELINE_ROW]})
    switch_refused(case, r"the baseline instrument reports changed entries: baseline row: control \| ~/scratch")


def c_baseline_fails(case):
    case.set_control({"fail": {"baseline": 1}})
    switch_refused(case, "the baseline instrument baseline failed")


def c_plan_changed_after_load(case):
    """A gate reads the plan again by its path, after the program admitted its bytes."""
    one_byte(case, "superpowers")
    changed = source_digest(keg_source(case, "superpowers"))
    case.serial_plan()
    case.set_control({"actions": [{"point": "command:grove-release", "do": "write-key", "path": case.serial,
                                   "key": ["accepted", "sources", "superpowers", "resourceDigest"],
                                   "value": changed}]})
    note = case.attempt()
    before = serial_view(case)
    expect_exit(case.switch(note), 3, "is not the plan that the program admitted")
    expect_view(case, before, "after the changed plan")
    expect(not os.path.lexists(note["backup"]), "a changed plan published a backup")


def c_plan_paths_escape(case):
    """A plan path that is absolute, holds .. or crosses a linked folder stops before any step."""
    outside = os.path.join(case.root, "outside")
    write(os.path.join(outside, "grove/SKILL.md"), "---\nname: grove\n---\nOutside HOME.\n")
    os.symlink(outside, os.path.join(case.home, ".claude/linked"))
    variants = {"absolute": os.path.join(outside, "grove"), "dotdot": "../outside/grove",
                "linked": ".claude/linked/grove"}
    for label, rel in variants.items():
        def mutate(plan, rel=rel):
            for w in plan["writes"] + plan["assertions"]:
                if w["path"] == ".claude/skills/grove":
                    w["path"] = rel
            plan["restore"][plan["restore"].index(".claude/skills/grove")] = rel
        plan_refused(case, label, mutate, "is not relative to HOME|without \\.\\.|leads outside HOME")
        expect(os.path.isfile(os.path.join(outside, "grove/SKILL.md")), f"the {label} path moved an entry outside HOME")
    return ", ".join(variants)


def c_plan_incomplete(case):
    """A plan without a check that a step needs, or with a removed field, stops before any step."""
    def gate_4_other(plan):
        plan["gates"]["4"] = [{"name": "grove-release", "argv": fixture_command("probe-native")}]

    def earlier_value(plan):
        plan["writes"][2]["set"][0]["earlier"] = True

    def entry_identity(plan):
        plan["writes"][0]["identity"] = "0" * 64

    def entry_set(plan):
        plan["writes"][0]["set"] = [{"key": ["x"], "value": 1}]

    def ravel_sources(plan):
        plan["writes"][-1]["set"][0]["value"] = OFF

    mutations = {
        "gates-empty": (lambda p: p.__setitem__("gates", {}), "gates must name exactly"),
        "gate-4-other": (gate_4_other, "does not run the program's gate of step 4"),
        "gate-7-empty": (lambda p: p["gates"].__setitem__("7", []), "gates.7 names no command"),
        "probes-empty": (lambda p: p.__setitem__("probes", []), "probes names no command"),
        "native-probe-missing": (lambda p: p.pop("native_probe"), "native_probe names no command"),
        "mac-check-missing": (lambda p: p.pop("mac_check"), "mac_check is not a command"),
        "baseline-missing": (lambda p: p.pop("baseline"), "baseline is not a command"),
        "inventory-empty": (lambda p: p["inventory"].__setitem__("after", []), "inventory.after names no entry"),
        "staging-missing": (lambda p: p["accepted"].pop("staging"), "accepted staging must name"),
        "restore-order": (lambda p: p["restore"].reverse(), "must restore .config/ravel/settings.json first"),
        "hook-file-missing": (lambda p: p.__setitem__("writes", [w for w in p["writes"]
                                                                 if w["path"] != KNOWN_MARKETPLACES]),
                              "writes does not name the file .claude/plugins/known_marketplaces.json"),
        "earlier-value": (earlier_value, r"writes\[2\]\.set is not a list of keys with values"),
        "entry-identity": (entry_identity, r"writes\[0\] holds identity"),
        "entry-set": (entry_set, r"writes\[0\] holds set"),
        "ravel-sources": (ravel_sources, "names no sources"),
        "parts": (lambda p: p.__setitem__("parts", []), "the plan holds parts, which schema"),
        "outside": (lambda p: p.__setitem__("outside", []), "the plan holds outside, which schema"),
    }
    for label, (mutate, text) in mutations.items():
        plan_refused(case, label, mutate, f"the plan is incomplete: .*{text}")
    return ", ".join(mutations)


def c_plan_process_patterns(case):
    """The plan holds no process, MCP or job pattern."""
    old = {"session": {"harness": ["claude"], "mcp": []}, "watch": ["codex"],
           "jobs": {"argv": ["/bin/launchctl", "list"], "patterns": ["mise"]}}
    for name in old:
        plan_refused(case, name, lambda p, n=name: p.__setitem__(n, old[n]),
                     f"the plan holds {name}, which schema owner-switch-plan/2 does not name")
    return ", ".join(old)


def c_claude_version_differs(case):
    """The switch runs no Claude Code command with a version that the plan does not hold."""
    case.set_control({"claude_version": "2.2.0 (Claude Code)"})
    switch_refused(case, "claude --version printed '2.2.0 \\(Claude Code\\)'; the plan holds")
    expect(case.commands() == {}, f"a refused switch ran {case.commands()}")


def c_receipt_evidence(case):
    """The receipt holds the activated configuration, the sources, the readers and each retained resource."""
    note, _ = switched(case)
    body = load(receipt_file(case, note), {})
    act = body.get("activated", {})
    expect(act.get("ravel") == {"path": os.path.join(case.root, "prefix/bin/ravel"),
                                "version": "ravel 2.0.0 (fixture)"}, f"the receipt names Ravel {act.get('ravel')}")
    expect((act.get("grove") or {}).get("version") == "1", f"the receipt names Grove {act.get('grove')}")
    sources = body.get("sources", {})
    expect(sorted(sources) == sorted(sid for _, ids in KEGS.values() for sid in ids),
           f"the receipt names the sources {sorted(sources)}")
    for sid, got in sources.items():
        expect(got.get("resourceDigest") == source_digest(keg_source(case, sid)),
               f"the receipt's source {sid} is {got}")
    expect(body.get("backup") == note["backup"] and body.get("restore_command") == note["restore"],
           "the receipt does not name its backup and its restore")
    kinds = sorted({i["item"] for i in body.get("retained", [])})
    wanted = ["attempt note", "backup", "candidate keg", "earlier marketplace registration",
              "frozen cache version", "Grove's Codex snapshot store", "retained source", "staging HOME",
              "superseded plugin registration", "the original driver's Ravel keg"]
    expect(kinds == sorted(wanted), f"the receipt retains {kinds}")
    again = body.get("readers", {}).get("again", [])
    expect(len(again) == len(READERS) and all(r["present"] and r["registered"] and not r["orphaned"] for r in again),
           f"the receipt's second reading of the readers is {again}")


def c_tool_before_switch(case):
    pid = sleeper(case, "codex", "app-server", "daemon", "pid-update-loop", reparent=True)
    out = switch_refused(case, "remains before the switch")
    expect_reported(out, pid, "remaining", "Codex", "app-server daemon")
    expect_alive([pid])


def tool_later(do: str, argv=None):
    """A tool that starts after the backup stops the switch. Restore cannot verify while it runs."""
    def fn(case):
        case.serial_plan()
        before = serial_view(case)
        note = case.attempt()
        action = {"point": "gate:the first switch write", "do": do}
        if argv:
            action["argv"] = argv
        case.set_control({"actions": [action]})
        expect_exit(case.switch(note), 4, "recovery incomplete")
        expect(not os.path.lexists(receipt_file(case, note)), "a success receipt remains")
        case.kill()
        expect_exit(case.literal(note["restore"]), 0, "result: restored")
        expect_view(case, before, "after the explicit restore")
    return fn


def session_tool(spec: dict, text: str):
    """A process of the fixture harness's session stops the switch when it is one of the tools."""
    def fn(case):
        case.set_control({"session": [spec]})
        time.sleep(0.3)
        switch_refused(case, text)
    return fn


def session_allowed(argv):
    """A process of the session that is none of the tools needs no exemption."""
    def fn(case):
        case.set_control({"session": [[fixture(case, a) if a in FIXTURE_NAMES else a for a in argv]]})
        time.sleep(0.3)
        switched(case)
    return fn


def c_mcp_server_tool_child(case):
    """No descendant exemption: a tool under an MCP server of the session stops the switch."""
    server = fixture(case, "lib/server-with-child.js")
    ready = os.path.join(case.root, "server-ready")
    write(server, child(case, "codex") + f"open({ready!r}, 'w').close(); " + SLEEP + "\n")
    case.set_control({"session": [[fixture(case, "node"), server]]})
    # The harness starts the server beside the program, so the program waits for its child.
    wait = (f"import os, subprocess, sys, time\nfor _ in range(200):\n    if os.path.exists({ready!r}): break\n"
            "    time.sleep(0.05)\nsys.exit(subprocess.run(sys.argv[1:]).returncode)")
    out = switch_refused(case, "remains before the switch", via=[PY, "-I", "-c", wait])
    expect(re.search(r"remaining process \d+ \(parent \d+\) Codex", out) is not None, f"no Codex child: {out}")


def c_finish_ancestors(case):
    """The switch admits its exact Ravel driver and Claude Code harness ancestors."""
    via = relay(case, "ravel") + relay(case, "share/claude/versions/2.1.319")
    _, out = switched(case, via=via)
    expect(re.search(r"admitted ancestor process \d+ .*Ravel", out) is not None, "no admitted Ravel ancestor")


def c_source_changes_in_probes(case):
    """Step 8 resolves the sources again: an eleventh source during the probes fails the switch."""
    serial_failure("probes", actions=[{"point": "probes", "do": "eleventh-source"}],
                   text="the sources no longer resolve with their accepted digests")(case)


def gate_controls() -> list:
    return [
        ("gates-evidence", c_gates_evidence, SERIAL_ARGS),
        ("source-eleventh-stops-step-6", c_eleventh_source, SERIAL_ARGS),
        ("source-one-byte-stops-step-6", c_one_byte_other, SERIAL_ARGS),
        ("source-one-byte-grove-reruns-trials", c_one_byte_grove, SERIAL_ARGS),
        ("source-grove-trial-fails", c_trial_fails, SERIAL_ARGS),
        ("source-grove-no-trial", c_no_trial, SERIAL_ARGS),
        ("source-changes-during-probes", c_source_changes_in_probes, SERIAL_ARGS),
        ("staging-home-missing", c_staging_missing, SERIAL_ARGS),
        ("grove-tap-newer", c_tap_newer, SERIAL_ARGS),
        ("grove-linked-older", c_linked_older, SERIAL_ARGS),
        ("grove-opt-elsewhere", c_opt_elsewhere, SERIAL_ARGS),
        ("grove-source-outside-payload", c_grove_source_outside_payload, SERIAL_ARGS),
        ("grove-version-differs", c_grove_version, SERIAL_ARGS),
        ("grove-revision-admitted", c_grove_revision, SERIAL_ARGS),
        ("formula-missing", c_formula({"linkuistics/taps/personal": {"state": "missing"}},
                                      "step 5: linkuistics/taps/personal is missing, not installed"), SERIAL_ARGS),
        ("formula-undeclared", c_formula({"linkuistics/taps/testanyware": None},
                                         "step 5: linkuistics/taps/testanyware is not declared"), SERIAL_ARGS),
        ("formula-mise-disagrees", c_formula({"linkuistics/taps/superpowers": {"installed_version": "9"}},
                                             "step 5: linkuistics/taps/superpowers: mise reports 9, and Homebrew "
                                             "links 7.0.0_1"), SERIAL_ARGS),
        ("formula-revision-earlier",
         c_formula_relinked("personal", "2026-11-06", "step 5: linkuistics/taps/personal is at 2026-11-06, not "
                                                      "its accepted version 2026-11-06_1"), SERIAL_ARGS),
        ("formula-revision-earlier-mise-omits",
         c_formula_relinked("testanyware", "2.8.0", "step 5: linkuistics/taps/testanyware is at 2.8.0, not its "
                                                    "accepted version 2.8.0_1",
                            {"linkuistics/taps/testanyware": {"installed_version": "2.8.0"}}), SERIAL_ARGS),
        ("formula-revision-later",
         c_formula_relinked("superpowers", "7.0.0_2", "step 5: linkuistics/taps/superpowers is at 7.0.0_2, not "
                                                      "its accepted version 7.0.0_1"), SERIAL_ARGS),
        ("formula-revision-later-mise-omits",
         c_formula_relinked("quint-llm-kit", "1.0.0_2", "step 5: linkuistics/taps/quint-llm-kit is at 1.0.0_2, "
                                                        "not its accepted version 1.0.0_1",
                            {"linkuistics/taps/quint-llm-kit": {"installed_version": "1.0.0"}}), SERIAL_ARGS),
        ("formula-revision-omitted-passes", c_formula_revision_omitted, SERIAL_ARGS),
        ("formula-unlinked", c_formula_unlinked, SERIAL_ARGS),
        ("reader-orphaned", c_reader_orphaned, SERIAL_ARGS),
        ("reader-missing", c_reader_missing, SERIAL_ARGS),
        ("reader-named-unregistered", c_reader_unregistered, SERIAL_ARGS),
        ("reader-named", c_reader_named, SERIAL_ARGS),
        ("reader-orphaned-in-probes",
         serial_probe_change({"do": "touch", "path": os.path.join(READERS["grove@linkuistics"], ".orphaned_at"),
                              "text": "1792928060164"}, "holds .orphaned_at",
                             [os.path.join(READERS["grove@linkuistics"], ".orphaned_at")]), SERIAL_ARGS),
        ("baseline-anomaly", c_baseline_anomaly, SERIAL_ARGS),
        ("baseline-fails", c_baseline_fails, SERIAL_ARGS),
        ("plan-changed-after-load", c_plan_changed_after_load, SERIAL_ARGS),
        ("plan-paths-escape", c_plan_paths_escape, SERIAL_ARGS),
        ("plan-incomplete", c_plan_incomplete, SERIAL_ARGS),
        ("plan-process-patterns", c_plan_process_patterns, SERIAL_ARGS),
        ("claude-version-differs", c_claude_version_differs, SERIAL_ARGS),
        ("receipt-evidence", c_receipt_evidence, SERIAL_ARGS),
    ]


def switch_process_controls(r) -> list:
    codex_child = {"argv": [os.path.join(r.fixture_bin, "codex"), "-c", SLEEP], "executable": None}
    claude_child = {"argv": ["claude", "-c", SLEEP],
                    "executable": os.path.join(r.fixture_bin, "share/claude/versions/2.1.319")}
    return [
        ("tool-before-switch", c_tool_before_switch, SERIAL_ARGS),
        ("tool-later", tool_later("spawn-tool", [os.path.join(r.fixture_bin, "grove"), "-c", SLEEP]), SERIAL_ARGS),
        ("descendant-harness-before-switch", session_tool(claude_child, r"remaining process \d+ .*Claude Code"),
         SERIAL_ARGS),
        ("descendant-harness-later", tool_later("spawn-in-session", codex_child), SERIAL_ARGS),
        ("background-child-allowed", session_allowed(["/bin/sleep", "120"]), SERIAL_ARGS),
        ("caffeinate-child-allowed", session_allowed(["caffeinate", "-c", SLEEP]), SERIAL_ARGS),
        ("mcp-server-allowed", session_allowed(["node", os.path.join(r.fixture_bin, "lib/other-server.js")]),
         SERIAL_ARGS),
        ("mcp-server-tool-child", c_mcp_server_tool_child, SERIAL_ARGS),
        ("finish-ancestors-allowed", c_finish_ancestors, SERIAL_ARGS),
    ]


# --- the dry-run instrument ---------------------------------------------------------
# "The rehearsed plan" and the two dry-run cases of "Test seams". Each control
# prepares a fixture plan and program in the case's HOME, which stands for the
# real HOME, runs scripts/owner_switch_dry_run.py into the case's folder and
# compares the result with the case's HOME.

DRY_RUN = os.path.join(HERE, "owner_switch_dry_run.py")
PROBE_SCRIPT = f"{STATE}/prepared/fixture_probe.py"
PROBE_HELPER = f"{STATE}/prepared/fixture_probe_helper.py"
FIXTURE_MAC_CHECK = ".local/bin/mac-check"
FIXTURE_CHECKER = ".local/lib/fixture-check/check.py"
DRY_ARGS = {"ravel": "absent-key", "hooks": "absent"}
DRY_STEPS = ["native-off", "attempt", "switch", "restore"]


def dry_run_files(case) -> None:
    """A probe program with a local helper, and a mac-check script that runs a checker under $HOME.

    The mac-check script has the form of the owner's: a shell script that runs
    an interpreter by its absolute path with a checker under $HOME.
    """
    write(os.path.join(case.home, PROBE_SCRIPT),
          "import os\nimport sys\n\nsys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))\n"
          "import fixture_probe_helper  # noqa: E402\n\nfixture_probe_helper.main(sys.argv[1:])\n")
    write(os.path.join(case.home, PROBE_HELPER),
          f"import os\n\n\ndef main(args):\n    os.execv({PY!r}, [{PY!r}, '-I', {SELF!r}, 'fixture-command'] + args)\n")
    write_script(os.path.join(case.home, FIXTURE_MAC_CHECK),
                 f"#!/bin/sh\nexec '{PY}' -I \"$HOME/{FIXTURE_CHECKER}\" \"$@\"\n")
    write(os.path.join(case.home, FIXTURE_CHECKER),
          f"import os\n\nos.execv({PY!r}, [{PY!r}, '-I', {SELF!r}, 'fixture-command', 'mac-check'])\n")


def dry_run_prepare(case, mutate=None) -> None:
    """Prepare the fixture plan and program in the case's HOME, as the owner prepares the real ones.

    The case's Ravel settings have no "skills" key, as in the real HOME before
    finish. The plan's original of those settings is the layout after
    native-off, so the setup runs native-off, computes the plan and then puts
    the original bytes back. The dry run's own native-off then produces the
    planned original.
    """
    path = os.path.join(case.home, RAVEL)
    with open(path, "rb") as f:
        original = f.read()
    expect_exit(case.native_off(), 0, "off value")
    dry_run_files(case)

    def fields(plan):
        plan["probes"] = [{"name": "probe-switched", "argv": ["$python", "-I", PROBE_SCRIPT, "probe-switched"]}]
        plan["native_probe"] = [{"name": "probe-native", "argv": ["$python", "-I", PROBE_SCRIPT, "probe-native"]}]
        plan["mac_check"] = {"name": "mac-check", "argv": [FIXTURE_MAC_CHECK, "--only", "Owner skill switch"]}
        plan["inputs"] += [{"role": role, "path": os.path.join(case.home, rel),
                            "digest": digest_file(os.path.join(case.home, rel))}
                           for role, rel in (("probe program", PROBE_SCRIPT), ("probe helper", PROBE_HELPER),
                                             ("mac-check command", FIXTURE_MAC_CHECK),
                                             ("mac-check helper", FIXTURE_CHECKER))]
        if mutate:
            mutate(plan)
    case.serial_plan(fields)
    with open(path, "r+b") as f:
        f.truncate()
        f.write(original)
    case.prepare()


def dry_env(case) -> dict:
    """The instrument's environment: the fixture prefix, and variables that it must not pass on."""
    return dict(clean_env(), OWNER_SWITCH_FIXTURE_PREFIX=os.path.join(case.root, "prefix"),
                CLAUDE_CONFIG_DIR=os.path.join(case.root, "elsewhere"),
                CODEX_HOME=os.path.join(case.root, "elsewhere", ".codex"),
                RAVEL_SKILL_DIRECTORIES=os.path.join(case.root, "elsewhere"),
                OWNER_SWITCH_REHEARSAL_HOOK=os.path.join(case.root, "no-such-hook"))


def dry_invoke(case, argv: list, label: str) -> tuple:
    result = subprocess.run([PY, "-I", case.r.dry_run] + argv, env=dry_env(case), capture_output=True, text=True,
                            timeout=900)
    out = result.stdout + result.stderr
    case.outputs.append(out)
    case.runs.append({"exit": result.returncode, "claude": {}})
    with open(os.path.join(case.root, "runs.log"), "a") as f:
        f.write(f"$ owner_switch_dry_run.py {label} (exit {result.returncode})\n{out}\n")
    return result.returncode, out


def dry_run(case, *extra: str, out: str = "dry") -> tuple:
    """Run the dry run of the case's prepared plan into the case folder OUT; return the exit, output and result."""
    folder = os.path.join(case.root, out)
    code, text = dry_invoke(case, ["run", "--home", case.home, "--out", folder, "--path", case.r.stubs,
                                   "--python", PY, "--fixture-scope", case.r.fixture_bin, *extra], "run")
    return code, text, load(os.path.join(folder, "result.json"), {})


def dry_compare(case, *extra: str, out: str = "dry", name: str = "comparison") -> tuple:
    folder = os.path.join(case.root, out)
    record = os.path.join(folder, f"{name}.json")
    code, text = dry_invoke(case, ["compare", "--result", os.path.join(folder, "result.json"), "--home", case.home,
                                   "--path", case.r.stubs, "--python", PY, "--out", record, *extra], "compare")
    return code, text, load(record, {})


def verdicts(record: dict) -> dict:
    return {r["logical"]: r for r in record.get("rows", [])}


def expect_dry_passed(code: int, text: str, result: dict) -> None:
    expect(code == 0 and result.get("passed") is True, f"the dry run did not pass (exit {code}): {text[-2500:]}")


def release_grove_keg(case) -> str:
    """Install an equal copy of Grove's keg at another version path, as a release of the same content."""
    prefix = os.path.join(case.root, "prefix")
    released = os.path.join(prefix, "Cellar/grove/1-released")
    subprocess.run(["/bin/cp", "-pPR", os.path.join(prefix, "Cellar/grove/1"), released], check=True)
    links = {os.path.join(prefix, "opt/grove"): "../Cellar/grove/1-released",
             os.path.join(prefix, "var/homebrew/linked/grove"): "../../../Cellar/grove/1-released"}
    for sid in KEGS["grove"][1]:
        links[os.path.join(prefix, ALIASES, sid)] = f"../../../Cellar/grove/1-released/{ALIASES}/{sid}"
    for link, target in links.items():
        os.remove(link)
        os.symlink(target, link)
    return released


KEG_ROWS_OF_GROVE = ["tool:grove"] + [f"source:{sid}" for sid in KEGS["grove"][1]]


def c_dry_run_full(case):
    dry_run_prepare(case)
    before = case.snapshot()
    code, text, result = dry_run(case)
    expect_dry_passed(code, text, result)
    expect(result["fixture"] and "no real-harness acceptance" in result["limits"][0],
           f"the fixture result does not state its limit: {result['limits'][:1]}")
    expect([s["step"] for s in result["steps"]] == DRY_STEPS and all(s["passed"] for s in result["steps"]),
           f"the dry run ran these steps: {[(s['step'], s['exit']) for s in result['steps']]}")
    native = result["native_off"]
    expect(native["backups_before"] == [] and native["skills_after"] == OFF
           and native["settings_before"] != native["settings_after"],
           f"native-off did not write the off value without a backup: {native}")
    probes = [(p["phase"], p["name"], p["exit"]) for p in result["probes"]]
    expect(probes == [("switch", "probe-switched", 0), ("restore", "probe-native", 0)],
           f"the per-probe outcomes are {probes}")
    rc = result["restore_comparison"]
    items = {r["item"] for r in rc["rows"]}
    expect(rc["equal"] and "restore .config/ravel/settings.json" in items
           and "inventory .codex/skills/grove" in items, f"the restore comparison is {rc}")
    expect(result["valid"] and not result["changed"], f"subjects changed: {result['changed']}")
    expect(result["source_unchanged"]["equal"], f"the source HOME changed: {result['source_unchanged']['moved']}")
    expect(case.snapshot() == before, "the dry run changed the case's HOME, which stands for the real HOME")
    rows = {r["logical"]: r for r in result["inputs"]}
    wanted = ["plan", "program", "interpreter:$python", "input:probe helper", "input:fixture commands",
              f"command:probes/probe-switched:2", "command:mac_check/mac-check:0",
              "helper:command:probes/probe-switched:2 fixture_probe_helper",
              f"helper:command:mac_check/mac-check:0 $HOME/{FIXTURE_CHECKER}",
              f"helper:command:mac_check/mac-check:0 {PY}", "interpreter:command:mac_check/mac-check:0",
              "tool:claude", "tool:brew", "tool:mise", "tool:ravel", "tool:grove", "source:grove-core",
              "source:personal"]
    missing = [w for w in wanted if w not in rows]
    expect(not missing, f"the input table lacks {missing}; it holds {sorted(rows)}")
    expect(rows["tool:claude"].get("version") == FIXTURE_CLAUDE and rows["tool:ravel"].get("version")
           == "ravel 2.0.0 (fixture)" and rows["tool:grove"].get("version") == "grove 1",
           f"the tool versions are {[(k, v.get('version')) for k, v in rows.items() if k.startswith('tool:')]}")
    expect(all(r["digest"] for r in result["inputs"]), "a row has no content digest")
    expect(rows["helper:command:probes/probe-switched:2 fixture_probe_helper"]["path"].startswith(result["dry_home"]),
           "the helper row does not name the executed path in the disposable HOME")
    expect(not result["undeclared"], f"the plan's inputs miss {result['undeclared']}")
    code, text, record = dry_compare(case)
    expect(code == 0 and record.get("equal"), f"the comparison with the unchanged HOME failed: {text[-2500:]}")
    v = verdicts(record)
    expect(v["plan"]["verdict"] == "substituted home" and v["input:probe helper"]["verdict"] == "substituted home"
           and v["tool:claude"]["verdict"] == "equal", f"the verdicts are {[(k, x['verdict']) for k, x in v.items()]}")
    return f"{len(result['inputs'])} rows; {len(rc['rows'])} restore items equal"


def c_dry_run_subject_changes(case):
    dry_run_prepare(case)
    hook = os.path.join(case.root, "change-helper.sh")
    write_script(hook, f"#!/bin/sh\nif [ \"$1\" = switch:final ]; then printf '# changed\\n' >> "
                       f"\"$OWNER_SWITCH_HOME/{PROBE_HELPER}\"; fi\nexit 0\n")
    code, text, result = dry_run(case, "--fixture-hook", hook)
    expect(code == 1 and result.get("passed") is False and result.get("valid") is False,
           f"a changed subject did not invalidate the run (exit {code}): {text[-1500:]}")
    changed = {c["logical"] for c in result["changed"]}
    expect({"input:probe helper", "helper:command:probes/probe-switched:2 fixture_probe_helper"} <= changed,
           f"the run names these changed subjects: {sorted(changed)}")
    expect(all(s["passed"] for s in result["steps"]), "the steps did not all pass, so the control proves nothing")
    code, text, _ = dry_compare(case)
    expect(code == 1 and "did not pass" in text, f"the comparison accepted an invalid run: {text[-1500:]}")


def c_dry_run_declared_prefix(case):
    dry_run_prepare(case)
    keg = os.path.join(case.root, "prefix/Cellar/grove/1")
    expect_dry_passed(*dry_run(case, "--prefix", f"grove-keg={keg}"))
    released = release_grove_keg(case)
    code, text, record = dry_compare(case, "--prefix", f"grove-keg={released}")
    expect(code == 0 and record.get("equal"), f"equal content at the declared prefix failed: {text[-2500:]}")
    v = verdicts(record)
    wrong = [(k, v[k]["verdict"]) for k in KEG_ROWS_OF_GROVE if v[k]["verdict"] != "substituted grove-keg"]
    expect(not wrong, f"these rows are not the declared substitution: {wrong}")
    paths = {f"source:{sid}": os.path.join(released, SOURCES, sid) for sid in KEGS["grove"][1]}
    paths["source:personal"] = os.path.join(case.root, "prefix/Cellar/personal", KEGS["personal"][0], SOURCES,
                                            "personal")
    wrong = [(k, v[k]["real_path"]) for k, path in paths.items() if v[k]["real_path"] != os.path.realpath(path)]
    expect(not wrong, f"these source rows do not name the private payload: {wrong}")


def c_dry_run_source_outside_payload(case):
    """The released keg holds design as a real directory at its alias: the same bytes at another path."""
    dry_run_prepare(case)
    keg = os.path.join(case.root, "prefix/Cellar/grove/1")
    expect_dry_passed(*dry_run(case, "--prefix", f"grove-keg={keg}"))
    released = release_grove_keg(case)
    alias = os.path.join(released, ALIASES, "design")
    os.remove(alias)
    shutil.copytree(os.path.join(released, SOURCES, "design"), alias, symlinks=True)
    code, text, record = dry_compare(case, "--prefix", f"grove-keg={released}")
    row = verdicts(record)["source:design"]
    expect(code == 1 and row["verdict"] == "differs" and row["reasons"] == [
        f"unknown substitution: {os.path.realpath(os.path.join(keg, SOURCES, 'design'))} -> "
        f"{os.path.realpath(alias)}"],
        f"a source outside its private payload passed (exit {code}): {row}")


def c_dry_run_unknown_substitution(case):
    dry_run_prepare(case)
    keg = os.path.join(case.root, "prefix/Cellar/grove/1")
    expect_dry_passed(*dry_run(case, "--prefix", f"grove-keg={keg}"))
    release_grove_keg(case)
    code, text, record = dry_compare(case)
    v = verdicts(record)
    unknown = [k for k in KEG_ROWS_OF_GROVE if any("unknown substitution" in r for r in v[k]["reasons"])]
    expect(code == 1 and unknown == KEG_ROWS_OF_GROVE, f"an undeclared prefix passed (exit {code}): {unknown}: {text[-1500:]}")
    code, text, _ = dry_compare(case, "--prefix", f"brew={case.root}", name="undeclared-name")
    expect(code == 2 and "not a declared substitution" in text, f"an unknown name was accepted: {text[-800:]}")
    code, text, _ = dry_compare(case, "--prefix", f"ravel={case.root}", name="undeclared-ravel")
    expect(code == 2 and "declared no such substitution" in text,
           f"a substitution that the dry run did not declare was accepted: {text[-800:]}")


def c_dry_run_changed_helper(case):
    dry_run_prepare(case)
    code, text, result = dry_run(case)
    expect_dry_passed(code, text, result)
    with open(os.path.join(case.home, PROBE_HELPER), "a") as f:
        f.write("# changed\n")
    code, text, record = dry_compare(case)
    v = verdicts(record)
    key = "helper:command:probes/probe-switched:2 fixture_probe_helper"
    expect(key in v, f"the comparison has no row {key!r} (exit {code}): {sorted(v)}")
    row = v[key]
    expect(code == 1 and row["verdict"] == "differs" and len(row["reasons"]) == 1
           and row["reasons"][0].startswith("content differs") and v["input:probe helper"]["verdict"] == "differs",
           f"a changed helper at the same path passed (exit {code}): {row}")
    real = {r["logical"]: r for r in record["real_inputs"]}
    dry = {r["logical"]: r for r in result["inputs"]}
    expect(real[key]["digest"] != dry[key]["digest"], "the recorded content identity did not change")


def c_dry_run_changed_plan(case):
    dry_run_prepare(case)
    expect_dry_passed(*dry_run(case))
    plan_path = os.path.join(case.home, STATE, "prepared/plan.json")
    plan = load(plan_path, {})
    plan["retained"][0]["condition"] = ["its last driver ends", "a changed condition"]
    save_json(plan_path, plan)
    code, text, record = dry_compare(case)
    expect(code == 1 and "/retained/0/condition" in " ".join(record.get("plan_differences", []))
           and verdicts(record)["plan"]["verdict"] == "differs",
           f"a changed plan value passed (exit {code}): {text[-1500:]}")


def c_dry_run_changed_source(case):
    dry_run_prepare(case)
    expect_dry_passed(*dry_run(case))
    one_byte(case, "personal")
    code, text, record = dry_compare(case)
    row = verdicts(record)["source:personal"]
    expect(code == 1 and row["verdict"] == "differs" and "content differs" in row["reasons"][0],
           f"a changed source passed (exit {code}): {row}")


def c_dry_run_tool_version(case):
    dry_run_prepare(case)
    expect_dry_passed(*dry_run(case))
    case.set_control({"claude_version": "2.1.320 (Claude Code)"})
    code, text, record = dry_compare(case)
    row = verdicts(record)["tool:claude"]
    expect(code == 1 and any(r.startswith("version differs") for r in row["reasons"]),
           f"a changed tool version passed (exit {code}): {row}")


def c_dry_run_failed_probe(case):
    def failing(plan):
        plan["probes"][0]["argv"][-1] = "probe-native"
    dry_run_prepare(case, failing)
    before = case.snapshot()
    code, text, result = dry_run(case)
    steps = [(s["step"], s["exit"]) for s in result.get("steps", [])]
    expect(code == 1 and result.get("passed") is False and steps[-1] == ("switch", 5),
           f"a failed probe did not fail the dry run (exit {code}): {steps}")
    probes = [(p["phase"], p["name"], p["exit"]) for p in result["probes"]]
    expect(("switch", "probe-switched", 1) in probes and ("restore", "probe-native", 0) in probes,
           f"the per-probe outcomes are {probes}")
    expect(case.snapshot() == before, "the failed dry run changed the case's HOME")


def c_dry_run_source_written(case):
    """A write that leaves the disposable HOME for the source HOME must fail the run."""
    dry_run_prepare(case)
    target = os.path.join(case.home, ".codex/skills/codebase-memory/SKILL.md")
    hook = os.path.join(case.root, "write-source.sh")
    write_script(hook, f"#!/bin/sh\nif [ \"$1\" = switch:final ]; then printf 'changed\\n' >> '{target}'; fi\nexit 0\n")
    code, text, result = dry_run(case, "--fixture-hook", hook)
    moved = result.get("source_unchanged", {}).get("moved", [])
    expect(code == 1 and result.get("passed") is False and "~/.codex/skills" in moved,
           f"a write into the source HOME passed (exit {code}): {moved}: {text[-1500:]}")


def c_dry_run_refusals(case):
    dry_run_prepare(case)
    busy = os.path.join(case.root, "busy")
    write(os.path.join(busy, "file"), "x\n")
    code, text, _ = dry_run(case, out="busy")
    expect(code == 3 and "new or empty folder" in text, f"a used OUT was accepted: {text[-800:]}")
    code, text = dry_invoke(case, ["run", "--home", case.home, "--out", os.path.join(case.home, STATE, "dry"),
                                   "--path", case.r.stubs], "run inside")
    expect(code == 3 and "inside the copied path" in text, f"an OUT inside a copied path was accepted: {text[-800:]}")
    code, text, _ = dry_run(case, "--prefix", f"home={case.root}", out="home-prefix")
    expect(code == 2 and "not a declared substitution" in text, f"a home prefix was accepted: {text[-800:]}")
    os.remove(os.path.join(case.home, STATE, "prepared/plan.json"))
    code, text, _ = dry_run(case, out="no-plan")
    expect(code == 3 and "cannot be read" in text, f"a missing prepared plan was accepted: {text[-800:]}")


def dry_run_controls() -> list:
    return [
        ("dry-run-full", c_dry_run_full, DRY_ARGS),
        ("dry-run-subject-changes", c_dry_run_subject_changes, DRY_ARGS),
        ("dry-run-declared-prefix", c_dry_run_declared_prefix, DRY_ARGS),
        ("dry-run-unknown-substitution", c_dry_run_unknown_substitution, DRY_ARGS),
        ("dry-run-source-outside-payload", c_dry_run_source_outside_payload, DRY_ARGS),
        ("dry-run-changed-helper", c_dry_run_changed_helper, DRY_ARGS),
        ("dry-run-changed-plan", c_dry_run_changed_plan, DRY_ARGS),
        ("dry-run-changed-source", c_dry_run_changed_source, DRY_ARGS),
        ("dry-run-tool-version", c_dry_run_tool_version, DRY_ARGS),
        ("dry-run-failed-probe", c_dry_run_failed_probe, DRY_ARGS),
        ("dry-run-source-written", c_dry_run_source_written, DRY_ARGS),
        ("dry-run-refusals", c_dry_run_refusals, DRY_ARGS),
    ]


def all_controls(r) -> list:
    return (process_controls() + switch_process_controls(r) + gate_controls() + native_off_controls()
            + serial_controls() + status_controls() + dry_run_controls())


def cmd_rehearse(args) -> int:
    r = Rehearsal(args.out, args.program, args.mac_check, args.dry_run)
    for name, fn, kw in all_controls(r):
        if args.only and not re.search(args.only, name):
            continue
        r.control(name, fn, **kw)
    passed = all(x["passed"] for x in r.results)
    record = {
        "schema": "owner-switch-rehearsal/2",
        "passed": passed and not args.only,
        "plan": r.plan, "plan_digest": r.plan_digest,
        "program": r.program, "program_digest": r.program_digest,
        "dry_run": r.dry_run, "dry_run_digest": digest_file(r.dry_run),
        "python": platform.python_version(), "macos": platform.mac_ver()[0],
        "controls": len(r.results), "failed": [x["control"] for x in r.results if not x["passed"]],
        "results": r.results,
        "partial": bool(args.only),
    }
    save(os.path.join(r.out, "record.json"), record)
    print(f"{len(r.results) - len(record['failed'])}/{len(r.results)} controls passed; record {r.out}/record.json")
    return 0 if passed else 1


def main() -> int:
    if len(sys.argv) > 1 and sys.argv[1] == "fixture-command":
        return cmd_fixture(sys.argv[2])
    if len(sys.argv) > 1 and sys.argv[1] == "hook":
        return cmd_hook(sys.argv[2])
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
    p.add_argument("--dry-run", default="", help="the dry-run instrument (default scripts/owner_switch_dry_run.py)")
    args = parser.parse_args()
    return cmd_rehearse(args)


if __name__ == "__main__":
    sys.exit(main())
