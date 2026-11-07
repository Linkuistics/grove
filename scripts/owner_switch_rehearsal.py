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

The record OUT/record.json names the digests of the plan and the program and
the result of each control. It is a passing rehearsal record of the program
only when every control passed. OUT/plan.json is the fixture plan.

This slice covers the controls of the switch, the inverse, detection,
interruptions, entries, step 14, the processes and the rehearsal condition.
The preludes, the gates of steps 4 to 7, the owner's inverse and the merge
entry add their controls in later slices.
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
GROVE_PLUGIN = ["enabledPlugins", "grove@linkuistics"]
HOOKS_PLUGIN = ["enabledPlugins", "technical-communication-hooks@writing-hooks"]


# --- the fixture layout --------------------------------------------------------

def write(path: str, text: str) -> None:
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w") as f:
        f.write(text)


def build_home(home: str) -> None:
    write(os.path.join(home, ".claude/settings.json"), json.dumps({
        "enabledPlugins": {"grove@linkuistics": True,
                           "technical-communication-hooks@writing-hooks": False,
                           "context7@claude-plugins-official": True},
        "theme": "dark",
    }, indent=2) + "\n")
    write(os.path.join(home, ".config/ravel/settings.json"),
          json.dumps({"skills": OFF, "policy": "config.rhai"}, indent=2) + "\n")
    write(os.path.join(home, ".claude/skills/grove/SKILL.md"), "---\nname: grove\n---\nThe earlier copy.\n")
    os.makedirs(os.path.join(home, ".codex/skills"), exist_ok=True)
    os.symlink("../../.grove-snapshot/skills/grove", os.path.join(home, ".codex/skills/grove"))
    write(os.path.join(home, ".grove-snapshot/skills/grove/SKILL.md"), "---\nname: grove\n---\nSnapshot.\n")
    write(os.path.join(home, ".codex/skills/codebase-memory/SKILL.md"), "---\nname: codebase-memory\n---\n")
    write(os.path.join(home, MANUAL), "# AI Dev Tooling\n")


def fixture_command(name: str) -> list:
    return [PY, "-I", SELF, "fixture-command", name]


def build_plan(program: str, out: str) -> str:
    probe = os.path.join(out, "plan-home")
    if os.path.exists(probe):
        shutil.rmtree(probe)
    build_home(probe)

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
                {"key": GROVE_PLUGIN, "earlier": True, "switched": False},
                {"key": HOOKS_PLUGIN, "earlier": False, "switched": True},
            ]},
            {"kind": "file", "path": ".config/ravel/settings.json", "cells": [
                {"key": ["skills"], "earlier": OFF, "switched": SWITCHED},
            ]},
        ],
        "outside": [
            {"root": ".codex/skills", "entries": {"codebase-memory": ident(".codex/skills/codebase-memory")}},
            {"root": ".claude/skills", "entries": {}},
        ],
        "session": {"harness": [HARNESS_MARK], "mcp": []},
        "watch": [WRITER_MARK],
        "jobs": {"argv": fixture_command("jobs"), "patterns": [JOB_MARK]},
        "gates": {s: [{"name": f"gate-{s}", "argv": fixture_command(f"gate-{s}")}] for s in ("4", "5", "6", "7")},
        "probes": [{"name": "probe-switched", "argv": fixture_command("probe-switched")}],
        "native_probe": [{"name": "probe-native", "argv": fixture_command("probe-native")}],
        "mac_check": {"name": "mac-check", "argv": fixture_command("mac-check")},
    }
    shutil.rmtree(probe)
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
    def __init__(self, rehearsal, name: str, control=None, rehearsed=True):
        self.r = rehearsal
        self.name = name
        self.root = os.path.join(rehearsal.out, "cases", name)
        if os.path.exists(self.root):
            shutil.rmtree(self.root)
        self.home = os.path.join(self.root, "home")
        build_home(self.home)
        self.set_control(control or {})
        if rehearsed:
            self.rehearse_record(rehearsal.plan_digest, rehearsal.program_digest)
        self.outputs = []

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

    def run(self, program=None, plan=None, dry=False) -> tuple:
        args = [PY, "-I", program or self.r.program, "run", "--plan", plan or self.r.plan, "--home", self.home]
        if dry:
            args.append("--dry-run")
        env = dict(clean_env(), OWNER_SWITCH_REHEARSAL_HOOK=self.r.hook)
        result = subprocess.run([PY, "-I", SELF, "harness", HARNESS_MARK, self.root, "--"] + args,
                                env=env, capture_output=True, text=True, timeout=300)
        out = result.stdout + result.stderr
        self.outputs.append(out)
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
                             "seconds": round(time.time() - started, 2)})
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
    write_in_place(os.path.join(case.home, SETTINGS), GROVE_PLUGIN, "other")
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
    write_in_place(path, GROVE_PLUGIN, False)
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
    write_in_place(os.path.join(case.home, SETTINGS), GROVE_PLUGIN, True)
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
    plan["parts"] = plan["parts"][:3]
    other = os.path.join(case.root, "plan-changed.json")
    save(other, plan)
    out = expect_exit(case.run(plan=other), 3, "the prepared plan differs from the open run's plan")
    expect("record: inverse-ended" in out, "the open run's plan did not apply to its inverse")
    expect("no passing rehearsal record" in out, "the unrehearsed prepared plan passed step 8")


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
    return controls


def cmd_rehearse(args) -> int:
    r = Rehearsal(args.out, args.program)
    for name, fn, kw in all_controls(r):
        if args.only and not re.search(args.only, name):
            continue
        r.control(name, fn, **kw)
    passed = all(x["passed"] for x in r.results) and not args.only
    record = {
        "schema": "owner-switch-rehearsal/1",
        "passed": passed,
        "plan": r.plan, "plan_digest": r.plan_digest,
        "program": r.program, "program_digest": r.program_digest,
        "python": platform.python_version(), "macos": platform.mac_ver()[0],
        "controls": len(r.results), "failed": [x["control"] for x in r.results if not x["passed"]],
        "results": r.results,
        "partial": bool(args.only),
    }
    save(os.path.join(r.out, "record.json"), record)
    print(f"{len(r.results) - len(record['failed'])}/{len(r.results)} controls passed; record {r.out}/record.json")
    return 0 if all(x["passed"] for x in r.results) else 1


def main() -> int:
    if len(sys.argv) > 1 and sys.argv[1] == "fixture-command":
        return cmd_fixture(sys.argv[2])
    if len(sys.argv) > 1 and sys.argv[1] == "hook":
        return cmd_hook(sys.argv[2])
    if len(sys.argv) > 1 and sys.argv[1] == "holder":
        return cmd_holder(sys.argv[2], sys.argv[3])
    if len(sys.argv) > 1 and sys.argv[1] == "harness":
        sep = sys.argv.index("--")
        return cmd_harness(sys.argv[3], sys.argv[sep + 1:])
    parser = argparse.ArgumentParser(description=__doc__.split("\n", 1)[0])
    sub = parser.add_subparsers(dest="command", required=True)
    p = sub.add_parser("rehearse")
    p.add_argument("--out", required=True)
    p.add_argument("--program", default=os.path.join(HERE, "owner_switch.py"))
    p.add_argument("--only", help="run only the controls whose name matches; the record is then partial")
    args = parser.parse_args()
    return cmd_rehearse(args)


if __name__ == "__main__":
    sys.exit(main())
