#!/usr/bin/env python3
"""Run the owner's skill-delivery switch, its automatic inverse and its state.

docs/specs/owner-skill-delivery.md owns every rule this program applies: "The
switch inside finish", "Concurrent writers", "Resuming the switch", "The
rehearsed plan" and "Rollback". docs/models/owner-switch.qnt checks the
recovery rules. Change the model first when a recovery rule changes.

The program is one file, so the copy in each backup is exact. It uses only
the standard library and runs on Python 3.9 and later. It reaches
renamex_np, the ACL calls and the extended-attribute calls through ctypes,
so it runs only on macOS.

Usage:
    python3 -I owner_switch.py state    --plan PLAN [--home HOME] [--json]
    python3 -I owner_switch.py run      --plan PLAN [--home HOME] [--dry-run]
    python3 -I owner_switch.py identity PATH
    python3 -I owner_switch.py digest   PATH

`state` reads the state of the switch set and changes nothing. `run` runs the
switch from step 1, or continues an open run as the state table states.
`run --dry-run` reports the parts, the current value of each cell and the
inverse, runs no command and changes nothing. `identity` prints the change
identity of an entry, and `digest` prints the digest of a file, for a plan's
author. HOME defaults to the account's home directory. Each path in the plan
is relative to HOME.

Exit status: 0 when the run reached the complete or the native state that it
was asked for, 2 for a usage error, 3 for a stop. A stop prints "stop:" and
the reason, and it changes nothing more.

The plan (JSON, schema "owner-switch-plan/1"):
    state        directory of backups and rehearsal records, relative to HOME
    manual       the maintained manual, relative to HOME
    parts        ordered parts. {"kind": "entry", "path", "identity"} or
                 {"kind": "file", "path", "cells": [{"key": [...],
                 "earlier": V, "switched": V}]}. A value {"$absent": true}
                 is an absent key. Entries come first.
    outside      entries outside the switch set: {"path", "identity"}, or
                 {"root", "entries": {name: identity}} for each child of a
                 directory that no part names.
    session      {"harness": [regex], "mcp": [regex]}: the kept processes
    watch        [regex]: the programs of step 1, matched against arguments
    jobs         {"argv": [...], "patterns": [regex]}: the job listing
    gates        {"4": [command], "5": [...], "6": [...], "7": [...]}
    probes       [command]: the probes of step 10
    native_probe [command]: the native probe of Rollback
    mac_check    command: step 12
A command is {"name", "argv"}. It runs in HOME, without Grove's loop control
variables, with OWNER_SWITCH_HOME, OWNER_SWITCH_STATE and OWNER_SWITCH_STEP.

A passing rehearsal record is a JSON file in <state>/rehearsals with
"passed": true, "plan_digest" and "program_digest". Step 8 needs a record
that names the plan and the program. The preludes need a record that names
the program only, because a prelude applies no plan.

Step 3 erases each held link whose inode is still at the path. While the
latest complete backup's run is open, it runs no prelude. Otherwise it runs
the native settings and then the hooks installation. Their paths and values
are constants of this program, not of the plan:
    native settings     .config/ravel/settings.json, "skills" set to
                        {"schemaVersion": 1, "sources": []}
    hooks installation  technical-communication-hooks@writing-hooks from the
                        marketplace registered at $(brew --prefix)/opt/grove,
                        read and installed with `claude`, then disabled in
                        .claude/settings.json by an exchange
The program finds `claude` and `brew` on PATH. Each runs in HOME, with HOME
set to HOME and without Claude Code's five location variables.

A settings record is a directory in <state>/settings, named under a
temporary name until it is complete. It holds record.json and the file
`copy`. For an exchange, `copy` is the stage file and, after the exchange,
the kept copy. Its record.json holds the digests "read" and "stage".

For a Claude Code command, `copy` is the held link and record.json holds
"read", the digest of the one reading through it. A held link whose inode is still
at the path holds no kept copy. The owner's merge of a flagged copy is
`copy.merge-<n>.json` with its "digest", as in a backup.

Rehearsal hook: OWNER_SWITCH_REHEARSAL_HOOK names a command that the program
runs at each named point with the point as its argument. Exit 99 ends the
program at once, as a crash. The program refuses the hook when HOME is the
account's home directory, so a real run never honours it.
"""
from __future__ import annotations

import argparse
import ctypes
import datetime
import errno
import hashlib
import json
import os
import pwd
import re
import secrets
import stat
import subprocess
import sys

PLAN_SCHEMA = "owner-switch-plan/1"
REHEARSAL_SCHEMA = "owner-switch-rehearsal/1"
ABSENT = {"$absent": True}
UNREADABLE = {"$unreadable": True}
# docs/specs/harness-selection-and-execution.md#environment lists these.
LOOP_CONTROL = (
    "GROVE_SIGNAL_FILE", "GROVE_RUN_SIGNAL_FILE", "GROVE_LAUNCH_DIR",
    "HARNESS_DISPATCH_EXIT_FILE", "HARNESS_DISPATCH_RUN_ID",
    "HARNESS_DISPATCH_STATE_DIR", "RAVEL_EXIT_FILE", "GROVE_PROVIDER_EVIDENCE",
    "GROVE_HARNESS_PID", "GROVE_CLAUDE_PID",
)
HOOK_VAR = "OWNER_SWITCH_REHEARSAL_HOOK"
CRASH = 99

# The preludes: docs/specs/owner-skill-delivery.md, "The native settings
# inside finish" and "The hooks installation inside finish".
RAVEL_SETTINGS = ".config/ravel/settings.json"
OFF = {"schemaVersion": 1, "sources": []}
CLAUDE_SETTINGS = ".claude/settings.json"
HOOKS_MARKETPLACE = "writing-hooks"
HOOKS_PLUGIN = "technical-communication-hooks@writing-hooks"
HOOKS_KEY = ["enabledPlugins", HOOKS_PLUGIN]
# Claude Code drops each ACL and each extended attribute except this one
# when it rewrites settings.json ("Claude Code's own writes").
PROVENANCE = b"com.apple.provenance"
# "Settings isolation and the staging HOME": these five move Claude Code's
# configuration, sign-in store or plugin locations away from HOME.
CLAUDE_LOCATION_VARS = (
    "CLAUDE_CONFIG_DIR", "CLAUDE_SECURESTORAGE_CONFIG_DIR", "CLAUDE_CODE_PLUGIN_CACHE_DIR",
    "CLAUDE_CODE_PLUGIN_DIRS", "CLAUDE_CODE_PLUGIN_SEED_DIR",
)

PROGRAM_PATH = os.path.realpath(__file__)
with open(PROGRAM_PATH, "rb") as _f:
    PROGRAM_BYTES = _f.read()
PROGRAM_DIGEST = hashlib.sha256(PROGRAM_BYTES).hexdigest()


class Stop(Exception):
    """A stop: the run changes nothing more."""


class Conflict(Exception):
    """A failed read, comparison or rename inside a write interval."""


# --- macOS calls -----------------------------------------------------------

_libc = ctypes.CDLL(None, use_errno=True)
RENAME_SWAP, RENAME_EXCL = 0x2, 0x4
XATTR_NOFOLLOW = 0x1
ACL_TYPE_EXTENDED = 0x100
COPYFILE_METADATA = 0x7  # ACL, STAT and XATTR: copyfile(3)
_libc.renamex_np.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_uint]
_libc.flistxattr.argtypes = [ctypes.c_int, ctypes.c_char_p, ctypes.c_size_t, ctypes.c_int]
_libc.flistxattr.restype = ctypes.c_ssize_t
_libc.fgetxattr.argtypes = [ctypes.c_int, ctypes.c_char_p, ctypes.c_void_p,
                            ctypes.c_size_t, ctypes.c_uint32, ctypes.c_int]
_libc.fgetxattr.restype = ctypes.c_ssize_t
_libc.acl_get_fd_np.argtypes = [ctypes.c_int, ctypes.c_int]
_libc.acl_get_fd_np.restype = ctypes.c_void_p
_libc.acl_to_text.argtypes = [ctypes.c_void_p, ctypes.c_void_p]
_libc.acl_to_text.restype = ctypes.c_void_p
_libc.acl_free.argtypes = [ctypes.c_void_p]
_libc.fcopyfile.argtypes = [ctypes.c_int, ctypes.c_int, ctypes.c_void_p, ctypes.c_uint32]


def _oserror(what: str, path: str) -> OSError:
    code = ctypes.get_errno()
    return OSError(code, f"{what}: {os.strerror(code)}", path)


def renamex(src: str, dst: str, flags: int) -> None:
    """renamex_np(2). RENAME_SWAP exchanges two paths, RENAME_EXCL refuses an existing target."""
    if _libc.renamex_np(os.fsencode(src), os.fsencode(dst), flags) != 0:
        raise _oserror("renamex_np", dst)


def _xattrs(fd: int) -> list:
    size = _libc.flistxattr(fd, None, 0, 0)
    if size < 0:
        raise _oserror("flistxattr", str(fd))
    if size == 0:
        return []
    buf = ctypes.create_string_buffer(size)
    size = _libc.flistxattr(fd, buf, size, 0)
    out = []
    for name in sorted(n for n in buf.raw[:size].split(b"\0") if n):
        vsize = _libc.fgetxattr(fd, name, None, 0, 0, 0)
        if vsize < 0:
            raise _oserror("fgetxattr", name.decode())
        value = ctypes.create_string_buffer(max(vsize, 1))
        vsize = _libc.fgetxattr(fd, name, value, vsize, 0, 0)
        out.append((name, value.raw[:vsize]))
    return out


def _acl_text(fd: int) -> bytes:
    acl = _libc.acl_get_fd_np(fd, ACL_TYPE_EXTENDED)
    if not acl:
        if ctypes.get_errno() == errno.ENOENT:
            return b""
        raise _oserror("acl_get_fd_np", str(fd))
    try:
        text = _libc.acl_to_text(acl, None)
        try:
            return ctypes.string_at(text)
        finally:
            _libc.acl_free(text)
    finally:
        _libc.acl_free(acl)


def copy_metadata(src_fd: int, dst_fd: int) -> None:
    """Give dst the mode, ACL and extended attributes of src."""
    if _libc.fcopyfile(src_fd, dst_fd, None, COPYFILE_METADATA) != 0:
        raise _oserror("fcopyfile", str(dst_fd))


# --- digests and identities -------------------------------------------------

def _frame(h, tag: bytes, data: bytes) -> None:
    h.update(tag + b" %d\n" % len(data) + data + b"\n")


def read_fd(fd: int) -> bytes:
    chunks, offset = [], 0
    while True:
        chunk = os.pread(fd, 1 << 20, offset)
        if not chunk:
            return b"".join(chunks)
        chunks.append(chunk)
        offset += len(chunk)


def fd_reading_full(fd: int) -> dict:
    """One reading of an open file: its digest, content, ACL and attribute names.

    The digest covers the content, the mode, the ACL and the extended
    attributes, all read through one descriptor and therefore one inode.
    """
    st = os.fstat(fd)
    content = read_fd(fd)
    acl = _acl_text(fd)
    xattrs = _xattrs(fd)
    h = hashlib.sha256()
    _frame(h, b"mode", b"%o" % (st.st_mode & 0o7777))
    _frame(h, b"acl", acl)
    for name, value in xattrs:
        _frame(h, b"xattr " + name, value)
    _frame(h, b"content", content)
    return {"digest": h.hexdigest(), "content": content, "acl": acl,
            "xattrs": [name for name, _ in xattrs], "inode": (st.st_dev, st.st_ino)}


def fd_reading(fd: int):
    """One reading of an open file: its digest and its content."""
    reading = fd_reading_full(fd)
    return reading["digest"], reading["content"]


def inode_of(path: str):
    """The device and inode at a path, without following a link, or None."""
    try:
        st = os.lstat(path)
    except FileNotFoundError:
        return None
    return (st.st_dev, st.st_ino)


def path_digest(path: str):
    """The digest of a regular file, or None when the path is absent."""
    try:
        fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW)
    except FileNotFoundError:
        return None
    try:
        return fd_reading(fd)[0]
    finally:
        os.close(fd)


def identity(path: str):
    """The change identity of an entry, or None when nothing is at the path.

    It covers each relative path, the type, the executable bit and content of
    each file, and the target of each link, without following a link.
    """
    if not os.path.lexists(path):
        return None
    h = hashlib.sha256()

    def walk(p: str, rel: str) -> None:
        st = os.lstat(p)
        if stat.S_ISLNK(st.st_mode):
            _frame(h, b"link " + rel.encode(), os.fsencode(os.readlink(p)))
        elif stat.S_ISDIR(st.st_mode):
            _frame(h, b"dir", rel.encode())
            for name in sorted(os.listdir(p)):
                walk(os.path.join(p, name), f"{rel}/{name}")
        elif stat.S_ISREG(st.st_mode):
            x = b"x" if st.st_mode & 0o111 else b"-"
            with open(p, "rb") as f:
                _frame(h, b"file " + x + b" " + rel.encode(), f.read())
        else:
            _frame(h, b"other", rel.encode() + b" %o" % stat.S_IFMT(st.st_mode))

    walk(path, ".")
    return h.hexdigest()


def canon(value) -> str:
    return json.dumps(value, sort_keys=True, separators=(",", ":"))


def same(a, b) -> bool:
    return canon(a) == canon(b)


def show(value) -> str:
    if same(value, ABSENT):
        return "(absent)"
    if same(value, UNREADABLE):
        return "(unreadable)"
    return canon(value)


# --- cells ------------------------------------------------------------------

def get_cell(obj, key: list):
    if same(obj, UNREADABLE):
        return UNREADABLE
    for k in key:
        if not isinstance(obj, dict) or k not in obj:
            return ABSENT
        obj = obj[k]
    return obj


def set_cell(obj: dict, key: list, value) -> None:
    for k in key[:-1]:
        obj = obj.setdefault(k, {})
        if not isinstance(obj, dict):
            raise Conflict(f"key {'.'.join(key)} crosses a value that is not an object")
    if same(value, ABSENT):
        obj.pop(key[-1], None)
    else:
        obj[key[-1]] = value


def dump_json(obj) -> bytes:
    return (json.dumps(obj, indent=2, ensure_ascii=False) + "\n").encode()


def parse_json(content: bytes):
    try:
        obj = json.loads(content.decode())
    except (UnicodeDecodeError, ValueError):
        return UNREADABLE
    return obj if isinstance(obj, dict) else UNREADABLE


def key_name(key: list) -> str:
    return ".".join(key)


# --- the plan and the layout ----------------------------------------------------

def load_plan(path: str) -> dict:
    with open(path, "rb") as f:
        raw = f.read()
    plan = json.loads(raw)
    if plan.get("schema") != PLAN_SCHEMA:
        raise Stop(f"{path} is not a plan of schema {PLAN_SCHEMA}")
    kinds = [p["kind"] for p in plan["parts"]]
    if kinds != sorted(kinds, key=lambda k: k != "entry"):
        raise Stop("the plan's entry parts must come before its file parts")
    plan["_digest"] = hashlib.sha256(raw).hexdigest()
    plan["_bytes"] = raw
    return plan


def account_home() -> str:
    return os.path.realpath(pwd.getpwuid(os.getuid()).pw_dir)


class Layout:
    def __init__(self, home: str, plan: dict):
        self.home = os.path.realpath(home)
        self.state = os.path.join(self.home, plan["state"])
        self.backups = os.path.join(self.state, "backups")
        self.rehearsals = os.path.join(self.state, "rehearsals")
        self.settings = os.path.join(self.state, "settings")
        self.manual = os.path.join(self.home, plan["manual"])

    def path(self, rel: str) -> str:
        return os.path.join(self.home, rel)


def log(line: str) -> None:
    print(line, flush=True)


def hook(layout: Layout, point: str) -> None:
    command = os.environ.get(HOOK_VAR)
    if not command:
        return
    if layout.home == account_home():
        raise Stop(f"{HOOK_VAR} is set while HOME is the account's home directory")
    result = subprocess.run([command, point], env=dict(os.environ, OWNER_SWITCH_HOME=layout.home))
    if result.returncode == CRASH:
        sys.stdout.flush()
        os._exit(CRASH)
    if result.returncode != 0:
        raise Stop(f"the rehearsal hook failed at {point}")


# --- backups and the run record ---------------------------------------------------

def write_exclusive(path: str, data: bytes) -> None:
    """Write a file under a temporary name and rename it into place exclusively."""
    tmp = os.path.join(os.path.dirname(path), f".tmp-{secrets.token_hex(6)}")
    fd = os.open(tmp, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
    try:
        os.write(fd, data)
        os.fsync(fd)
    finally:
        os.close(fd)
    renamex(tmp, path, RENAME_EXCL)


class Backup:
    def __init__(self, path: str):
        self.path = path
        self.id = os.path.basename(path)
        self.plan = load_plan(os.path.join(path, "plan.json"))
        self.program = os.path.join(path, "program.py")
        with open(self.program, "rb") as f:
            self.program_digest = hashlib.sha256(f.read()).hexdigest()
        self.stage = os.path.join(path, "stage")
        self.retained = os.path.join(path, "retained")
        self.run = os.path.join(path, "run")

    def records(self) -> list:
        names = sorted(n for n in os.listdir(self.run) if not n.startswith("."))
        return ["switch-begun"] + [n.split("-", 1)[1][:-len(".json")] for n in names]

    def last(self) -> str:
        return self.records()[-1]

    def is_open(self) -> bool:
        return self.last() != "inverse-ended"

    def add_record(self, layout: Layout, kind: str, data: dict) -> None:
        hook(layout, f"before-record:{kind}")
        number = len(self.records())
        body = dict(data, kind=kind, at=now())
        write_exclusive(os.path.join(self.run, f"{number:02d}-{kind}.json"), dump_json(body))
        log(f"record: {kind} in {self.id}")
        hook(layout, f"after-record:{kind}")

    def record_body(self, kind: str) -> dict:
        for name in sorted(os.listdir(self.run)):
            if name.endswith(f"-{kind}.json"):
                with open(os.path.join(self.run, name)) as f:
                    body = json.load(f)
        return body

    def retained_path(self, rel: str) -> str:
        return os.path.join(self.retained, rel)


def complete_backups(layout: Layout) -> list:
    if not os.path.isdir(layout.backups):
        return []
    return sorted(n for n in os.listdir(layout.backups) if not n.startswith("."))


def latest_backup(layout: Layout):
    names = complete_backups(layout)
    return Backup(os.path.join(layout.backups, names[-1])) if names else None


def now() -> str:
    return datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%S.%fZ")


# --- kept copies ----------------------------------------------------------------

def kept_copy_records(layout: Layout) -> list:
    """Each kept copy of each complete backup and settings record, with its record."""
    out = []
    for name in complete_backups(layout):
        stage = os.path.join(layout.backups, name, "stage")
        if not os.path.isdir(stage):
            continue
        for rec in sorted(n for n in os.listdir(stage) if n.endswith(".record.json")):
            with open(os.path.join(stage, rec)) as f:
                body = json.load(f)
            body["_copy"] = os.path.join(stage, body["copy"])
            body["_backup"] = name
            out.append(body)
    for name in settings_records(layout):
        folder = os.path.join(layout.settings, name)
        with open(os.path.join(folder, "record.json")) as f:
            body = json.load(f)
        body["_copy"] = os.path.join(folder, "copy")
        body["_backup"] = None
        body["_live"] = layout.path(body["path"])
        out.append(body)
    return out


def settings_records(layout: Layout) -> list:
    """The complete settings records, oldest first."""
    if not os.path.isdir(layout.settings):
        return []
    return sorted(n for n in os.listdir(layout.settings) if not n.startswith(".")
                  and os.path.isfile(os.path.join(layout.settings, n, "record.json")))


def merge_digests(copy: str) -> list:
    """The digests that the owner's merges recorded, oldest first."""
    folder, base = os.path.split(copy)
    names = sorted(n for n in os.listdir(folder) if n.startswith(base + ".merge-") and n.endswith(".json"))
    out = []
    for n in names:
        with open(os.path.join(folder, n)) as f:
            out.append(json.load(f)["digest"])
    return out


def is_flagged(record: dict) -> bool:
    """A kept copy is flagged when it holds no recorded content.

    After a merge, the merge's digest is the only recorded content. A held
    link holds a kept copy only once another inode replaced it at the path:
    before that, a write through the link is a write to the live file.
    """
    if record.get("kind") == "held":
        if not os.path.lexists(record["_copy"]):
            return False
        if inode_of(record["_copy"]) == inode_of(record["_live"]):
            return False
    digest = path_digest(record["_copy"])
    merges = merge_digests(record["_copy"])
    if merges:
        return digest != merges[-1]
    return digest not in [record[k] for k in ("read", "stage") if k in record]


def flagged_copies(layout: Layout) -> list:
    return [r for r in kept_copy_records(layout) if is_flagged(r)]


def report_flagged(layout: Layout) -> list:
    flagged = flagged_copies(layout)
    for r in flagged:
        what = "held copy" if r.get("kind") == "held" else "kept copy"
        log(f"flagged: {what} {r['_copy']} of {r['path']} holds no recorded content")
    return flagged


# --- part readings --------------------------------------------------------------

def read_file_cells(layout: Layout, part: dict) -> list:
    path = layout.path(part["path"])
    try:
        fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW)
    except FileNotFoundError:
        return [ABSENT for _ in part["cells"]]
    except OSError:
        return [UNREADABLE for _ in part["cells"]]
    try:
        obj = parse_json(fd_reading(fd)[1])
    finally:
        os.close(fd)
    return [get_cell(obj, c["key"]) for c in part["cells"]]


def part_status(layout: Layout, part: dict, backup) -> dict:
    """Classify one part as earlier, switched or third."""
    if part["kind"] == "file":
        values = read_file_cells(layout, part)
        cells = []
        for cell, value in zip(part["cells"], values):
            if same(value, cell["switched"]):
                status = "switched"
            elif same(value, cell["earlier"]):
                status = "earlier"
            else:
                status = "third"
            cells.append({"key": key_name(cell["key"]), "value": value, "status": status})
        statuses = {c["status"] for c in cells}
        overall = statuses.pop() if len(statuses) == 1 else "third"
        return {"status": overall, "cells": cells}
    original = identity(layout.path(part["path"]))
    retained = identity(backup.retained_path(part["path"])) if backup else None
    expected = part["identity"]
    if original == expected and retained is None:
        status = "earlier"
    elif backup and original is None and retained == expected:
        status = "switched"
    else:
        status = "third"
    return {"status": status, "original": original, "retained": retained}


def outside_changes(layout: Layout, plan: dict) -> list:
    """Each entry outside the switch set that differs from the plan's manifest."""
    changed = []
    part_paths = {p["path"] for p in plan["parts"]}
    for item in plan.get("outside", []):
        if "root" in item:
            root = layout.path(item["root"])
            names = set(os.listdir(root)) if os.path.isdir(root) else set()
            names = {n for n in names if os.path.join(item["root"], n) not in part_paths}
            for name in sorted(names | set(item["entries"])):
                rel = os.path.join(item["root"], name)
                if identity(layout.path(rel)) != item["entries"].get(name):
                    changed.append(rel)
        elif identity(layout.path(item["path"])) != item["identity"]:
            changed.append(item["path"])
    return changed


def is_prefix(statuses: list) -> bool:
    """The switched parts are a prefix of the plan's order and the others are earlier."""
    seen_earlier = False
    for s in statuses:
        if s == "third":
            return False
        if s == "earlier":
            seen_earlier = True
        elif seen_earlier:
            return False
    return True


def find_state(layout: Layout, prepared: dict) -> dict:
    """Find the state of the switch set from the layout, the run record and a plan."""
    backup = latest_backup(layout)
    is_open = bool(backup and backup.is_open())
    plan = backup.plan if is_open else prepared
    parts = [part_status(layout, p, backup if is_open else None) for p in plan["parts"]]
    statuses = [p["status"] for p in parts]
    outside = outside_changes(layout, plan)
    last = backup.last() if backup else None
    if is_open:
        all_switched = all(s == "switched" for s in statuses)
        if last == "receipt":
            state = "complete" if all_switched else "drifted"
        elif last == "switch-begun" and all_switched:
            state = "active"
        elif last in ("switch-begun", "inverse-begun") and is_prefix(statuses):
            state = "interrupted"
        else:
            state = "drifted"
    else:
        native = all(s == "earlier" for s in statuses) and not outside
        state = "native" if native else "changed"
    return {
        "state": state, "open": is_open, "backup": backup, "plan": plan,
        "parts": parts, "outside_changed": outside,
        "records": backup.records() if backup else [],
        "prepared_differs": bool(is_open and plan["_digest"] != prepared["_digest"]),
    }


def report_state(st: dict) -> None:
    backup = st["backup"]
    log(f"state: {st['state']}" + (f" (backup {backup.id}, records {', '.join(st['records'])})" if backup else ""))
    for i, (part, status) in enumerate(zip(st["plan"]["parts"], st["parts"])):
        if part["kind"] == "file":
            for c in status["cells"]:
                log(f"  part {i} {part['path']} {c['key']}: {show(c['value'])} ({c['status']})")
        else:
            log(f"  part {i} {part['path']}: {status['status']}")
    for rel in st["outside_changed"]:
        log(f"  changed outside the switch set: {rel}")
    if st["prepared_differs"]:
        log("  the prepared plan differs from the open run's plan; apply it only after the inverse")


# --- processes ----------------------------------------------------------------

def list_processes() -> dict:
    out = subprocess.run(["/bin/ps", "-axww", "-o", "pid=,ppid=,args="],
                         capture_output=True, text=True, check=True).stdout
    procs = {}
    for line in out.splitlines():
        parts = line.strip().split(None, 2)
        if len(parts) >= 2:
            procs[int(parts[0])] = (int(parts[1]), parts[2] if len(parts) > 2 else "")
    return procs


def descendants(procs: dict, root: int) -> set:
    children = {}
    for pid, (ppid, _) in procs.items():
        children.setdefault(ppid, []).append(pid)
    out, todo = set(), [root]
    while todo:
        for child in children.get(todo.pop(), []):
            if child not in out:
                out.add(child)
                todo.append(child)
    return out


def matches(patterns: list, text: str) -> bool:
    return any(re.search(p, text) for p in patterns)


def process_gate(layout: Layout, plan: dict, point: str) -> None:
    """List processes against the kept processes before a write interval.

    The kept processes are the program, its ancestors (the shell, the harness
    and the driver chain) and the harness process's MCP servers. Each other
    process that the plan watches, and each other descendant of the session,
    keeps the interval closed. So does each job that the plan watches.
    """
    hook(layout, f"gate:{point}")
    procs = list_processes()
    me = os.getpid()
    ancestors, pid = [], me
    while pid in procs and procs[pid][0] not in (0, pid):
        pid = procs[pid][0]
        ancestors.append(pid)
    session = plan.get("session", {})
    harness = next((a for a in ancestors if matches(session.get("harness", []), procs[a][1])), None)
    root = harness if harness is not None else os.getppid()
    kept = {me} | set(ancestors) | descendants(procs, me)
    if harness is not None:
        for pid, (ppid, args) in procs.items():
            if ppid == harness and matches(session.get("mcp", []), args):
                kept |= {pid} | descendants(procs, pid)
    session_tree = descendants(procs, root)
    others = []
    for pid, (_, args) in sorted(procs.items()):
        if pid in kept:
            continue
        if pid in session_tree:
            others.append(f"process {pid} of the session: {args}")
        elif matches(plan.get("watch", []), args):
            others.append(f"process {pid}: {args}")
    jobs = plan.get("jobs")
    if jobs:
        result = subprocess.run(jobs.get("argv", ["/bin/launchctl", "list"]), capture_output=True,
                                text=True, env=command_env(layout, "gate"))
        if result.returncode != 0:
            raise Stop(f"the job listing failed before {point}: {result.stderr.strip()}")
        for line in result.stdout.splitlines():
            if matches(jobs.get("patterns", []), line):
                others.append(f"job: {line.strip()}")
    if others:
        for line in others:
            log(f"  remaining {line}")
        raise Stop(f"other processes or jobs remain before {point}; ask the owner, and signal no session")
    log(f"gate: no other process before {point}")


# --- commands -----------------------------------------------------------------

def command_env(layout: Layout, step: str) -> dict:
    env = {k: v for k, v in os.environ.items() if k not in LOOP_CONTROL and k != HOOK_VAR}
    env.update(OWNER_SWITCH_HOME=layout.home, OWNER_SWITCH_STATE=layout.state, OWNER_SWITCH_STEP=step)
    return env


def run_command(layout: Layout, command: dict, step: str) -> dict:
    hook(layout, f"command:{command['name']}")
    result = subprocess.run(command["argv"], cwd=layout.home, env=command_env(layout, step),
                            capture_output=True, text=True)
    output = (result.stdout + result.stderr)[-20000:]
    log(f"command {command['name']} (step {step}): exit {result.returncode}")
    return {"name": command["name"], "argv": command["argv"], "exit": result.returncode, "output": output}


def run_commands(layout: Layout, commands: list, step: str):
    results = [run_command(layout, c, step) for c in commands]
    failed = [r for r in results if r["exit"] != 0]
    return results, failed


# --- parts ----------------------------------------------------------------------

def exchange_file(layout: Layout, backup: Backup, index: int, part: dict, phase: str, wanted,
                  changed=None) -> list:
    """Publish one file part by stage file, exchange and comparison.

    `wanted` maps each cell to its new value, or None to leave it. It returns
    the replaced values. A failed read or comparison raises Conflict.
    """
    path = layout.path(part["path"])
    try:
        fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW)
    except OSError as e:
        raise Conflict(f"part {index} {part['path']}: {e.strerror}")
    try:
        read_digest, content = fd_reading(fd)
        obj = parse_json(content)
        if same(obj, UNREADABLE):
            raise Conflict(f"part {index} {part['path']} is not a JSON object")
        replaced = []
        for cell in part["cells"]:
            new = wanted(cell, get_cell(obj, cell["key"]))
            if new is not None:
                replaced.append((key_name(cell["key"]), get_cell(obj, cell["key"]), new))
                set_cell(obj, cell["key"], new)
        if not replaced:
            return []
        hook(layout, f"{phase}-read:{index}")
        token = secrets.token_hex(4)
        copy = f"{phase}-{index:03d}-{token}"
        stage_path = os.path.join(backup.stage, copy)
        sfd = os.open(stage_path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
        try:
            os.write(sfd, dump_json(obj))
            os.fsync(sfd)
            copy_metadata(fd, sfd)
        finally:
            os.close(sfd)
        stage_digest = path_digest(stage_path)
    finally:
        os.close(fd)
    record = {"path": part["path"], "part": index, "phase": phase, "copy": copy,
              "read": read_digest, "stage": stage_digest}
    write_exclusive(os.path.join(backup.stage, copy + ".record.json"), dump_json(record))
    hook(layout, f"{phase}-before-exchange:{index}")
    try:
        renamex(stage_path, path, RENAME_SWAP)
    except OSError as e:
        raise Conflict(f"part {index} {part['path']}: exchange failed: {e.strerror}")
    if changed is not None:
        changed.add(index)
    hook(layout, f"{phase}-exchanged:{index}")
    if path_digest(stage_path) != read_digest:
        raise Conflict(f"part {index} {part['path']}: the kept copy {stage_path} differs from what the run read")
    for key, old, new in replaced:
        log(f"replaced: part {index} {part['path']} {key}: {show(old)} -> {show(new)}")
    return replaced


def switch_part(layout: Layout, backup: Backup, index: int, part: dict, changed: set) -> None:
    hook(layout, f"switch-before-part:{index}")
    if part["kind"] == "file":
        def wanted(cell, value):
            if not same(value, cell["earlier"]):
                raise Conflict(f"part {index} {part['path']} {key_name(cell['key'])} holds "
                               f"{show(value)}, not its earlier value")
            return cell["switched"]
        exchange_file(layout, backup, index, part, "switch", wanted, changed)
    else:
        src, dst = layout.path(part["path"]), backup.retained_path(part["path"])
        if identity(src) != part["identity"] or os.path.lexists(dst):
            raise Conflict(f"part {index} {part['path']} is not at its earlier value")
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        hook(layout, f"switch-read:{index}")
        try:
            renamex(src, dst, RENAME_EXCL)
        except OSError as e:
            raise Conflict(f"part {index} {part['path']}: move failed: {e.strerror}")
        changed.add(index)
        hook(layout, f"switch-exchanged:{index}")
        if identity(dst) != part["identity"]:
            raise Conflict(f"part {index} {part['path']}: the moved entry changed its identity")
        log(f"replaced: part {index} {part['path']}: moved into {dst}")
    hook(layout, f"switch-after-part:{index}")


def invert_part(layout: Layout, backup: Backup, index: int, part: dict) -> None:
    hook(layout, f"inverse-before-part:{index}")
    if part["kind"] == "file":
        def wanted(cell, value):
            if same(value, cell["switched"]):
                return cell["earlier"]
            if not same(value, cell["earlier"]):
                log(f"third value: part {index} {part['path']} {key_name(cell['key'])}: {show(value)}; left")
            return None
        exchange_file(layout, backup, index, part, "inverse", wanted)
    else:
        src, dst = backup.retained_path(part["path"]), layout.path(part["path"])
        if not os.path.lexists(src):
            if identity(dst) != part["identity"]:
                log(f"third value: part {index} {part['path']}; left")
            return
        if os.path.lexists(dst):
            log(f"third value: part {index} {part['path']} is occupied while its retained entry exists; "
                "both are left")
            return
        hook(layout, f"inverse-read:{index}")
        try:
            renamex(src, dst, RENAME_EXCL)
        except OSError as e:
            raise Conflict(f"part {index} {part['path']}: move back failed: {e.strerror}")
        if identity(dst) != part["identity"]:
            log(f"third value: part {index} {part['path']} moved back with a changed identity")
        log(f"replaced: part {index} {part['path']}: moved back from {src}")
    hook(layout, f"inverse-after-part:{index}")


def run_inverse(layout: Layout, backup: Backup, limit=None) -> None:
    """The inverse of the backup's plan, in its own write interval."""
    plan = backup.plan
    process_gate(layout, plan, "the inverse")
    if backup.last() != "inverse-begun":
        backup.add_record(layout, "inverse-begun", {})
    for index in reversed(range(len(plan["parts"]))):
        if limit is not None and index not in limit:
            continue
        try:
            invert_part(layout, backup, index, plan["parts"][index])
        except Conflict as e:
            raise Stop(f"the inverse stopped at a conflict: {e}; the next run resumes it")
    statuses = [part_status(layout, p, backup) for p in plan["parts"]]
    switched = [i for i, s in enumerate(statuses) if s["status"] == "switched"
                or any(c["status"] == "switched" for c in s.get("cells", []))]
    if switched:
        raise Stop(f"parts {switched} still hold a switched value after the inverse")
    backup.add_record(layout, "inverse-ended", {})


def failed_probe(layout: Layout, backup: Backup, reason: str) -> None:
    """Step 11: apply the inverse, then verify the native route."""
    log(f"failed probe: {reason}")
    run_inverse(layout, backup)
    _, failed = run_commands(layout, backup.plan.get("native_probe", []), "11")
    if failed:
        raise Stop(f"the inverse ran after a failed probe ({reason}), and the native probe failed: "
                   + ", ".join(f["name"] for f in failed))
    raise Stop(f"the inverse ran after a failed probe ({reason}); the native probe passed")


# --- the preludes of step 3 -------------------------------------------------------

def new_record_name(label: str) -> str:
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%S%fZ")
    return f"{stamp}-{label}-{secrets.token_hex(3)}"


def erase_held_links(layout: Layout) -> None:
    """Erase each held link whose inode is still at the path.

    Such a link holds no kept copy, because the path still names its inode.
    A stop or a crash before a command's rename leaves one. A record whose
    link is gone, or a temporary record with no file, holds nothing either.
    """
    if not os.path.isdir(layout.settings):
        return
    live = layout.path(CLAUDE_SETTINGS)
    for name in sorted(os.listdir(layout.settings)):
        folder = os.path.join(layout.settings, name)
        if not os.path.isdir(folder) or os.path.islink(folder):
            continue
        record = os.path.join(folder, "record.json")
        body = load_json_file(record)
        if body is not None and body.get("kind") != "held":
            continue
        copy = os.path.join(folder, "copy")
        if os.path.lexists(copy):
            if inode_of(copy) != inode_of(live):
                continue
            os.unlink(copy)
        elif body is None and os.listdir(folder):
            continue
        if os.path.exists(record):
            os.unlink(record)
        try:
            os.rmdir(folder)
        except OSError:
            continue
        log(f"erased: held link {folder}")


def load_json_file(path: str):
    try:
        with open(path) as f:
            body = json.load(f)
    except (OSError, ValueError):
        return None
    return body if isinstance(body, dict) else None


def refuse(layout: Layout, prepared: dict, reason: str) -> None:
    """Item 4 of a prelude: change nothing, and ask the owner."""
    report_state(find_state(layout, prepared))
    raise Stop(f"{reason}; stop for classification, and ask the owner")


def prelude_exchange(layout: Layout, rel: str, key: list, earlier, new, label: str) -> None:
    """Change one cell by a settings record, the atomic exchange and the comparison.

    The record holds the digest of what this reading read and of the stage
    file. It is complete under its own name before the exchange. After the
    exchange, the stage name holds the kept copy. A difference at the
    comparison flags that copy, and it is a stop.
    """
    path = layout.path(rel)
    try:
        fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW)
    except OSError as e:
        raise Stop(f"{rel} could not be read: {e.strerror}")
    os.makedirs(layout.settings, exist_ok=True)
    name = new_record_name(label)
    tmp = os.path.join(layout.settings, f".tmp-{name}")
    try:
        read_digest, content = fd_reading(fd)
        obj = parse_json(content)
        if same(obj, UNREADABLE):
            raise Stop(f"{rel} is not a JSON object")
        old = get_cell(obj, key)
        if not same(old, earlier):
            raise Stop(f"{rel} {key_name(key)} holds {show(old)}, not {show(earlier)}")
        set_cell(obj, key, new)
        hook(layout, f"{label}-read")
        os.mkdir(tmp, 0o700)
        stage = os.path.join(tmp, "copy")
        sfd = os.open(stage, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
        try:
            os.write(sfd, dump_json(obj))
            os.fsync(sfd)
            copy_metadata(fd, sfd)
        finally:
            os.close(sfd)
    finally:
        os.close(fd)
    record = {"kind": "exchange", "path": rel, "label": label, "key": key,
              "read": read_digest, "stage": path_digest(stage), "at": now()}
    with open(os.path.join(tmp, "record.json"), "wb") as f:
        f.write(dump_json(record))
        f.flush()
        os.fsync(f.fileno())
    folder = os.path.join(layout.settings, name)
    renamex(tmp, folder, RENAME_EXCL)
    log(f"record: settings record {folder}")
    hook(layout, f"{label}-recorded")
    copy = os.path.join(folder, "copy")
    try:
        renamex(copy, path, RENAME_SWAP)
    except OSError as e:
        raise Stop(f"{rel}: the exchange failed: {e.strerror}")
    hook(layout, f"{label}-exchanged")
    if path_digest(copy) != read_digest:
        raise Stop(f"the kept copy {copy} of {rel} differs from what the run read; it is flagged")
    log(f"replaced: {rel} {key_name(key)}: {show(old)} -> {show(new)}")
    hook(layout, f"{label}-compared")


def native_settings(layout: Layout, prepared: dict) -> None:
    """The native settings: give the Ravel settings the off value as `skills`."""
    hook(layout, "prelude:native-settings")
    path = layout.path(RAVEL_SETTINGS)
    try:
        fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW)
    except FileNotFoundError:
        fd = None
    except OSError as e:
        refuse(layout, prepared, f"the native settings could not read {RAVEL_SETTINGS}: {e.strerror}")
    if fd is None:
        process_gate(layout, prepared, "the native settings")
        os.makedirs(layout.settings, exist_ok=True)
        os.makedirs(os.path.dirname(path), exist_ok=True)
        stage = os.path.join(layout.settings, f".create-{new_record_name('native')}")
        sfd = os.open(stage, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o644)
        try:
            os.write(sfd, dump_json({"skills": OFF}))
            os.fsync(sfd)
        finally:
            os.close(sfd)
        hook(layout, "native-before-create")
        try:
            renamex(stage, path, RENAME_EXCL)
        except OSError as e:
            raise Stop(f"the native settings could not create {RAVEL_SETTINGS}: {e.strerror}")
        log(f"replaced: {RAVEL_SETTINGS} skills: (absent file) -> {show(OFF)}")
        hook(layout, "native-created")
        return
    try:
        obj = parse_json(fd_reading(fd)[1])
    finally:
        os.close(fd)
    if same(obj, UNREADABLE):
        refuse(layout, prepared, f"the native settings found {RAVEL_SETTINGS} not a JSON object")
    value = get_cell(obj, ["skills"])
    if same(value, OFF):
        log(f"native settings: {RAVEL_SETTINGS} holds the off value")
        return
    if not same(value, ABSENT):
        refuse(layout, prepared, f"the native settings found skills {show(value)} in {RAVEL_SETTINGS}")
    process_gate(layout, prepared, "the native settings")
    prelude_exchange(layout, RAVEL_SETTINGS, ["skills"], ABSENT, OFF, "native")


def claude_env(layout: Layout) -> dict:
    env = command_env(layout, "3")
    for name in CLAUDE_LOCATION_VARS:
        env.pop(name, None)
    env["HOME"] = layout.home
    return env


def run_tool(layout: Layout, argv: list):
    result = subprocess.run(argv, cwd=layout.home, env=claude_env(layout), capture_output=True, text=True)
    log(f"command {' '.join(argv)}: exit {result.returncode}")
    return result


def claude_list(layout: Layout, args: list) -> list:
    result = run_tool(layout, ["claude"] + args)
    if result.returncode != 0:
        raise Stop(f"claude {' '.join(args)} failed: {(result.stdout + result.stderr).strip()[-2000:]}")
    try:
        rows = json.loads(result.stdout)
    except ValueError:
        rows = None
    if not isinstance(rows, list) or not all(isinstance(r, dict) for r in rows):
        raise Stop(f"claude {' '.join(args)} printed no JSON list")
    return rows


def marketplace_path(layout: Layout) -> str:
    result = run_tool(layout, ["brew", "--prefix"])
    prefix = result.stdout.strip()
    if result.returncode != 0 or not prefix:
        raise Stop("brew --prefix failed")
    return os.path.join(prefix, "opt", "grove")


def hooks_state(layout: Layout, expected: str) -> dict:
    """The hooks plugin: absent, enabled, disabled or other.

    Measured with Claude Code 2.1.319 on 2026-11-07: each row of `claude
    plugin list --json` holds "id", "scope" and "enabled", and each row of
    `claude plugin marketplace list --json` holds "name", "source" and the
    "path" as it was registered. The plugin's cell in settings.json decides
    whether it is enabled, so a third value there is the other state.
    """
    plugins = [r for r in claude_list(layout, ["plugin", "list", "--json"]) if r.get("id") == HOOKS_PLUGIN]
    markets = [r for r in claude_list(layout, ["plugin", "marketplace", "list", "--json"])
               if r.get("name") == HOOKS_MARKETPLACE]
    cell = read_file_cells(layout, {"path": CLAUDE_SETTINGS, "cells": [{"key": HOOKS_KEY}]})[0]
    registered = (len(markets) == 1 and markets[0].get("source") == "directory"
                  and markets[0].get("path") == expected)
    out = {"registered": registered, "cell": cell}
    if markets and not registered:
        return dict(out, kind="other", reason=f"{HOOKS_MARKETPLACE} is registered other than at {expected}")
    if same(cell, UNREADABLE):
        return dict(out, kind="other", reason=f"{CLAUDE_SETTINGS} is not a readable JSON object")
    if not plugins:
        if same(cell, ABSENT) or same(cell, False):
            return dict(out, kind="absent", reason="the plugin is absent")
        return dict(out, kind="other", reason=f"the plugin is absent and its cell holds {show(cell)}")
    row = plugins[0]
    if len(plugins) > 1 or row.get("scope") != "user" or not registered:
        return dict(out, kind="other", reason=f"the plugin is installed other than from {expected} for the user")
    if same(cell, True) and row.get("enabled") is True:
        return dict(out, kind="enabled", reason="the plugin is installed and enabled")
    if same(cell, False) and row.get("enabled") is False:
        return dict(out, kind="disabled", reason="the plugin is installed and disabled")
    return dict(out, kind="other", reason=f"the plugin's cell holds {show(cell)}")


def write_held_record(layout: Layout, label: str):
    """Step 1 of a command: a settings record with a held link and one reading through it."""
    path = layout.path(CLAUDE_SETTINGS)
    try:
        st = os.lstat(path)
    except FileNotFoundError:
        raise Stop(f"{CLAUDE_SETTINGS} is absent, so no held link can keep it")
    if not stat.S_ISREG(st.st_mode):
        raise Stop(f"{CLAUDE_SETTINGS} is not a regular file")
    os.makedirs(layout.settings, exist_ok=True)
    name = new_record_name(label)
    tmp = os.path.join(layout.settings, f".tmp-{name}")
    os.mkdir(tmp, 0o700)
    held = os.path.join(tmp, "copy")
    os.link(path, held, follow_symlinks=False)
    fd = os.open(held, os.O_RDONLY | os.O_NOFOLLOW)
    try:
        reading = fd_reading_full(fd)
    finally:
        os.close(fd)
    record = {"kind": "held", "path": CLAUDE_SETTINGS, "label": label, "read": reading["digest"],
              "inode": list(reading["inode"]), "at": now()}
    with open(os.path.join(tmp, "record.json"), "wb") as f:
        f.write(dump_json(record))
        f.flush()
        os.fsync(f.fileno())
    folder = os.path.join(layout.settings, name)
    renamex(tmp, folder, RENAME_EXCL)
    log(f"record: held link {folder}")
    return folder, reading


def held_command(layout: Layout, prepared: dict, expected: str, args: list, label: str, needed) -> None:
    """Run one Claude Code command in the six steps of "Claude Code's own writes"."""
    hook(layout, f"hooks-before-held:{label}")
    folder, reading = write_held_record(layout, label)
    hook(layout, f"hooks-held:{label}")
    others = [n.decode(errors="replace") for n in reading["xattrs"] if n != PROVENANCE]
    if reading["acl"] or others:
        what = "an ACL" if reading["acl"] else "the extended attribute " + ", ".join(others)
        raise Stop(f"{CLAUDE_SETTINGS} holds {what}, which `claude {' '.join(args)}` would drop")
    state = hooks_state(layout, expected)
    if state["kind"] == "other":
        refuse(layout, prepared, f"the hooks installation found that {state['reason']}")
    if not needed(state):
        log(f"hooks installation: `claude {' '.join(args)}` is no longer needed")
        erase_held_links(layout)
        return
    hook(layout, f"hooks-command:{label}")
    result = run_tool(layout, ["claude"] + args)
    if result.returncode != 0:
        raise Stop(f"`claude {' '.join(args)}` failed: {(result.stdout + result.stderr).strip()[-2000:]}")
    path = layout.path(CLAUDE_SETTINGS)
    held = os.path.join(folder, "copy")
    if inode_of(path) == inode_of(held):
        raise Stop(f"`claude {' '.join(args)}` replaced no inode at {CLAUDE_SETTINGS}, so the held link "
                   "proves nothing")
    hook(layout, f"hooks-renamed:{label}")
    if path_digest(held) != reading["digest"]:
        raise Stop(f"the held copy {held} of {CLAUDE_SETTINGS} differs from its record; it is flagged")
    hook(layout, f"hooks-compared:{label}")


def hooks_installation(layout: Layout, prepared: dict) -> None:
    """The hooks installation: install the hooks plugin disabled."""
    hook(layout, "prelude:hooks-installation")
    expected = marketplace_path(layout)
    state = hooks_state(layout, expected)
    if state["kind"] == "disabled":
        log(f"hooks installation: {state['reason']}")
        return
    if state["kind"] == "other":
        refuse(layout, prepared, f"the hooks installation found that {state['reason']}")
    process_gate(layout, prepared, "the hooks installation")
    if state["kind"] == "absent":
        if not state["registered"]:
            held_command(layout, prepared, expected, ["plugin", "marketplace", "add", expected],
                         "marketplace-add", lambda s: s["kind"] == "absent" and not s["registered"])
        held_command(layout, prepared, expected, ["plugin", "install", HOOKS_PLUGIN],
                     "install", lambda s: s["kind"] == "absent" and s["registered"])
    hook(layout, "hooks-before-disable")
    cell = read_file_cells(layout, {"path": CLAUDE_SETTINGS, "cells": [{"key": HOOKS_KEY}]})[0]
    if same(cell, False):
        log("hooks installation: the plugin's cell already holds false; nothing written")
        return
    if not same(cell, True):
        refuse(layout, prepared, f"the hooks installation found the plugin's cell {show(cell)}")
    prelude_exchange(layout, CLAUDE_SETTINGS, HOOKS_KEY, True, False, "hooks-disable")


def step_3_preludes(layout: Layout, prepared: dict) -> None:
    """Step 3 up to the state: held links, the run record and the preludes."""
    erase_held_links(layout)
    backup = latest_backup(layout)
    if backup and backup.is_open():
        log(f"preludes: the run of {backup.id} is open, so no prelude runs")
        return
    if not passing_rehearsals(layout):
        raise Stop("no passing rehearsal record names this program, so no prelude runs")
    try:
        native_settings(layout, prepared)
        hooks_installation(layout, prepared)
    except Stop:
        report_flagged(layout)
        raise


# --- the switch -----------------------------------------------------------------

def passing_rehearsals(layout: Layout) -> list:
    """Each passing rehearsal record that names this program."""
    if not os.path.isdir(layout.rehearsals):
        return []
    out = []
    for name in sorted(os.listdir(layout.rehearsals)):
        if not name.endswith(".json"):
            continue
        try:
            with open(os.path.join(layout.rehearsals, name)) as f:
                rec = json.load(f)
        except (OSError, ValueError):
            continue
        if (isinstance(rec, dict) and rec.get("schema") == REHEARSAL_SCHEMA and rec.get("passed") is True
                and rec.get("program_digest") == PROGRAM_DIGEST):
            out.append(rec)
    return out


def rehearsed(layout: Layout, plan: dict) -> bool:
    """Step 8: a passing record names the plan and this program."""
    return any(rec.get("plan_digest") == plan["_digest"] for rec in passing_rehearsals(layout))


def write_backup(layout: Layout, plan: dict, evidence: dict) -> Backup:
    """Step 8: a new backup, complete only at its last rename."""
    os.makedirs(layout.backups, exist_ok=True)
    backup_id = datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%S%fZ")
    tmp = os.path.join(layout.backups, f".incomplete-{backup_id}")
    os.mkdir(tmp, 0o700)
    for sub in ("stage", "retained", "run", "evidence"):
        os.mkdir(os.path.join(tmp, sub))
    with open(os.path.join(tmp, "plan.json"), "wb") as f:
        f.write(plan["_bytes"])
    with open(os.path.join(tmp, "program.py"), "wb") as f:
        f.write(PROGRAM_BYTES)
    for step, results in evidence.items():
        with open(os.path.join(tmp, "evidence", f"step-{step}.json"), "wb") as f:
            f.write(dump_json(results))
    hook(layout, "before-backup-rename")
    final = os.path.join(layout.backups, backup_id)
    renamex(tmp, final, RENAME_EXCL)
    log(f"record: switch-begun in {backup_id}")
    hook(layout, "after-backup-rename")
    return Backup(final)


def cell_values(layout: Layout, plan: dict, backup: Backup) -> list:
    return [part_status(layout, p, backup) for p in plan["parts"]]


def all_switched(statuses: list) -> bool:
    return all(s["status"] == "switched" for s in statuses)


def apply_switch(layout: Layout, backup: Backup) -> None:
    """Step 9: the parts in their order, in one write interval."""
    plan = backup.plan
    process_gate(layout, plan, "step 9")
    changed = set()
    try:
        for index, part in enumerate(plan["parts"]):
            switch_part(layout, backup, index, part, changed)
        statuses = cell_values(layout, plan, backup)
        if not all_switched(statuses):
            raise Conflict("a cell does not hold its switched value at the final reading")
        if flagged_copies(layout):
            raise Conflict("a kept copy is flagged at the final reading")
    except Conflict as e:
        log(f"conflict: {e}")
        report_flagged(layout)
        run_inverse(layout, backup, limit=changed)
        raise Stop(f"the switch met a conflict and inverted its own parts: {e}")


def continue_after_switch(layout: Layout, backup: Backup, rerun_step6: bool) -> None:
    """Step 6 again when resuming, then the probes and steps 12 to 14."""
    plan = backup.plan
    if rerun_step6:
        _, failed = run_commands(layout, plan.get("gates", {}).get("6", []), "6")
        if failed:
            failed_probe(layout, backup, "step 6 failed: " + ", ".join(f["name"] for f in failed))
    before = cell_values(layout, plan, backup)
    probes, failed = run_commands(layout, plan.get("probes", []), "10")
    if failed:
        failed_probe(layout, backup, "probe failed: " + ", ".join(f["name"] for f in failed))
    mac_check = run_command(layout, plan["mac_check"], "12") if plan.get("mac_check") else None
    after = cell_values(layout, plan, backup)
    if not all_switched(after) or canon(before) != canon(after):
        failed_probe(layout, backup, "a cell changed while the probes ran")
    if report_flagged(layout):
        raise Stop("a kept copy is flagged at step 13")
    backup.add_record(layout, "receipt", {
        "plan_digest": plan["_digest"], "program_digest": backup.program_digest,
        "mac_check": mac_check, "probes": probes, "cells": after,
    })
    step_14(layout, backup)


def manual_has_receipt(layout: Layout, backup: Backup) -> bool:
    try:
        with open(layout.manual, encoding="utf-8") as f:
            return f"<!-- owner-switch-receipt {backup.id} -->" in f.read()
    except FileNotFoundError:
        return False


def step_14(layout: Layout, backup: Backup) -> None:
    """Compare the entries outside the switch set, then copy the receipt."""
    hook(layout, "step-14")
    changed = outside_changes(layout, backup.plan)
    if changed:
        failed_probe(layout, backup, "changed outside the switch set: " + ", ".join(changed))
    receipt = backup.record_body("receipt")
    text = (f"\n<!-- owner-switch-receipt {backup.id} -->\n"
            f"## Owner skill switch receipt {backup.id}\n\n"
            "This receipt describes the layout only while `mac-check` reports the switch complete.\n\n"
            "```json\n" + json.dumps(receipt, indent=2, sort_keys=True) + "\n```\n")
    try:
        fd = os.open(layout.manual, os.O_WRONLY | os.O_APPEND)
    except OSError as e:
        raise Stop(f"the receipt could not be written into the manual: {e.strerror}")
    try:
        os.write(fd, text.encode())
        os.fsync(fd)
    finally:
        os.close(fd)
    log(f"receipt: copied into {layout.manual}")


def reexec_copy(backup: Backup, args) -> None:
    """A continuation of an open backup runs the program copy in that backup."""
    if backup.program_digest == PROGRAM_DIGEST:
        return
    log(f"continuation: running the program copy of {backup.id}")
    argv = [sys.executable, "-I", backup.program, "run", "--plan", os.path.abspath(args.plan),
            "--home", os.path.abspath(args.home)]
    sys.stdout.flush()
    os.execv(sys.executable, argv)


def run_switch(args) -> int:
    prepared = load_plan(args.plan)
    layout = Layout(args.home, prepared)
    log(f"program: {PROGRAM_DIGEST}")
    log(f"plan: {prepared['_digest']}")
    hook(layout, "start")
    process_gate(layout, prepared, "step 1")
    step_3_preludes(layout, prepared)
    while True:
        st = find_state(layout, prepared)
        report_state(st)
        flagged = report_flagged(layout)
        backup = st["backup"]
        if st["open"]:
            reexec_copy(backup, args)
        state = st["state"]
        if state == "interrupted":
            run_inverse(layout, backup)
            continue
        if state == "drifted":
            raise Stop("the switch set is drifted; change nothing, and ask the owner")
        if state == "changed":
            raise Stop("the switch set differs from the prepared plan; stop for classification")
        if state == "complete":
            if not manual_has_receipt(layout, backup):
                step_14(layout, backup)
            log("result: complete")
            return 0
        if state == "active":
            continue_after_switch(layout, backup, rerun_step6=True)
            log("result: complete")
            return 0
        break  # native
    if latest_backup(layout):
        _, failed = run_commands(layout, prepared.get("native_probe", []), "3")
        if failed:
            raise Stop("the native probe failed: " + ", ".join(f["name"] for f in failed))
    evidence = {}
    for step in ("4", "5", "6", "7"):
        results, failed = run_commands(layout, prepared.get("gates", {}).get(step, []), step)
        evidence[step] = results
        if failed:
            raise Stop(f"step {step} failed: " + ", ".join(f["name"] for f in failed))
    if not rehearsed(layout, prepared):
        raise Stop("no passing rehearsal record names the prepared plan and this program")
    if flagged:
        raise Stop("a kept copy is flagged; the owner merges it first")
    backup = write_backup(layout, prepared, {k: evidence[k] for k in ("4", "6", "7")})
    apply_switch(layout, backup)
    continue_after_switch(layout, backup, rerun_step6=False)
    log("result: complete")
    return 0


def dry_run(args) -> int:
    prepared = load_plan(args.plan)
    layout = Layout(args.home, prepared)
    st = find_state(layout, prepared)
    report_state(st)
    report_flagged(layout)
    plan = st["plan"]
    if st["open"]:
        log("preludes: none, because a run is open")
    else:
        ravel = read_file_cells(layout, {"path": RAVEL_SETTINGS, "cells": [{"key": ["skills"]}]})[0]
        hooks = read_file_cells(layout, {"path": CLAUDE_SETTINGS, "cells": [{"key": HOOKS_KEY}]})[0]
        log(f"preludes: the native settings read skills {show(ravel)} and write {show(OFF)} if it is absent")
        log(f"preludes: the hooks installation reads {key_name(HOOKS_KEY)} {show(hooks)}, installs "
            f"{HOOKS_PLUGIN} if `claude plugin list` lacks it, and writes false")
        log(f"preludes: rehearsed program: {'yes' if passing_rehearsals(layout) else 'no'}")
    log("switch, in order:")
    for i, part in enumerate(plan["parts"]):
        if part["kind"] == "file":
            for c in part["cells"]:
                log(f"  part {i} {part['path']} {key_name(c['key'])}: {show(c['earlier'])} -> {show(c['switched'])}")
        else:
            log(f"  part {i} {part['path']}: move into the backup's retained tree")
    log("inverse, in order:")
    for i in reversed(range(len(plan["parts"]))):
        part = plan["parts"][i]
        if part["kind"] == "file":
            for c in part["cells"]:
                log(f"  part {i} {part['path']} {key_name(c['key'])}: {show(c['switched'])} -> {show(c['earlier'])}")
        else:
            log(f"  part {i} {part['path']}: move back, if the original path is absent")
    log(f"rehearsed: {'yes' if rehearsed(layout, prepared) else 'no'}")
    return 0


def state_command(args) -> int:
    prepared = load_plan(args.plan)
    layout = Layout(args.home, prepared)
    st = find_state(layout, prepared)
    flagged = flagged_copies(layout)
    if args.json:
        out = {
            "state": st["state"], "open": st["open"], "records": st["records"],
            "backup": st["backup"].path if st["backup"] else None,
            "plan_digest": st["plan"]["_digest"], "prepared_differs": st["prepared_differs"],
            "parts": st["parts"], "outside_changed": st["outside_changed"],
            "flagged": [r["_copy"] for r in flagged],
            "incomplete_backups": sorted(n for n in os.listdir(layout.backups) if n.startswith(".incomplete-"))
            if os.path.isdir(layout.backups) else [],
        }
        print(json.dumps(out, indent=2, sort_keys=True))
    else:
        report_state(st)
        report_flagged(layout)
    return 0


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n", 1)[0])
    sub = parser.add_subparsers(dest="command", required=True)
    for name in ("state", "run"):
        p = sub.add_parser(name)
        p.add_argument("--plan", required=True)
        p.add_argument("--home", default=account_home())
        if name == "state":
            p.add_argument("--json", action="store_true")
        else:
            p.add_argument("--dry-run", action="store_true")
    sub.add_parser("identity").add_argument("path")
    sub.add_parser("digest").add_argument("path")
    args = parser.parse_args(argv)
    try:
        if args.command == "identity":
            print(identity(args.path) or "absent")
            return 0
        if args.command == "digest":
            print(path_digest(args.path) or "absent")
            return 0
        if args.command == "state":
            return state_command(args)
        return dry_run(args) if args.dry_run else run_switch(args)
    except Stop as e:
        log(f"stop: {e}")
        return 3


if __name__ == "__main__":
    sys.exit(main())
