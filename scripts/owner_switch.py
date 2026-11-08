#!/usr/bin/env python3
"""Run the owner's skill-delivery switch, its automatic inverse and its state.

docs/specs/owner-skill-delivery.md owns every rule this program applies: "The
switch inside finish", "Concurrent writers", "Resuming the switch", "The
rehearsed plan" and "Rollback". No model checks these rules.
The specification's section "The recovery model" states why.

The program is one file, so the copy in each backup is exact. It uses only
the standard library and runs on Python 3.9 and later. It reaches
renamex_np, the ACL calls and the extended-attribute calls through ctypes,
so it runs only on macOS.

Usage:
    python3 -I owner_switch.py state       --plan PLAN [--home HOME] [--json]
    python3 -I owner_switch.py run         --plan PLAN [--home HOME] [--dry-run]
    python3 -I owner_switch.py inverse     --plan PLAN [--home HOME]
    python3 -I owner_switch.py merge       --plan PLAN [--home HOME] COPY [--record-only]
    python3 -I owner_switch.py purge-check --plan PLAN [--home HOME]
    python3 -I owner_switch.py pending     --plan PLAN [--home HOME] [--grove-commit SHA]
    python3 -I owner_switch.py native-off  [--home HOME] [--print-command]
    python3 -I owner_switch.py processes   [--home HOME] [--owned FILE] [--json]
    python3 -I owner_switch.py identity    PATH
    python3 -I owner_switch.py digest      PATH
    python3 -I owner_switch.py gate        {4,5,6,7} [--plan PLAN] [--home HOME]

`state` reads the state of the switch set and changes nothing. While a run
is open, the program copy of its backup reads it. `run` runs the
switch from step 1, or continues an open run as the state table states.
`run --dry-run` reports the parts, the current value of each cell and the
inverse, runs no command and changes nothing. `identity` prints the change
identity of an entry, and `digest` prints the digest of a file, for a plan's
author. `gate` runs the check of one of steps 4 to 7, which a plan names as
a command. HOME defaults to the account's home directory. Each path in the
plan is relative to HOME, normalised and without "..". Before any step, the
program stops unless each path's parent folder resolves inside HOME.

Exit status: 0 when the run reached the complete or the native state that it
was asked for, 2 for a usage error, 3 for a stop. A stop prints "stop:" and
the reason, and it changes nothing more than its run log.

The owner's entries ("Rollback", "Concurrent writers" and "Purge"):
    inverse      The owner's inverse. The program copy of the latest backup
                 runs it, while that backup's run is open. It runs the
                 narrow process inspection. It resets each switched cell
                 and leaves each third value. It moves an entry back only onto an
                 absent original path, whatever the entry's change identity.
                 An occupied path is a third value: both entries stay, and
                 the program reports the cell. It then runs the native probe
                 and appends one line to the manual. It ends with 0 only in
                 the native state.
    merge        Merge the flagged kept copy COPY into its live file. The
                 difference between the copy and its last recorded content,
                 an erased key included, reaches the live file by a settings
                 record and the exchange. A cell takes the copy's value only
                 when no run writes that value there. The record
                 COPY.merge-<n>.json holds the copy's digest and bytes, and
                 that digest is from then on the only recorded content.
                 --record-only records a merge that the owner applied by
                 hand.
    purge-check  Read the `retained` list of each receipt, or the entry
                 parts of a backup without a receipt. Stop unless each
                 retained entry keeps its recorded change identity, no copy
                 is flagged and no run is open outside the complete state.
                 List the conditions that only the owner confirms. Change
                 nothing.
    pending      Append the pending-activation record to the manual, from
                 the latest run log. A `run` or `inverse` writes its run log
                 into <state>/runs when it ends or stops. The record names
                 the Grove version and keg, the Ravel path and version, each
                 installed formula version, the output of `mise -C HOME
                 bootstrap packages status --json`, the failed commands, each
                 backup, the state, each cell of a drifted or changed state,
                 each changed entry outside the switch set and each flagged
                 copy. Finish supplies the release commit. A later receipt
                 names each pending-activation record that it replaces.

native-off ("The native settings inside finish"). Finish runs it immediately
after promotion, before source acceptance and before any new default launch.
The owner runs the same command after a crash, or after a stop before the
switch's backup. It reads no plan, needs no backup and runs no harness. It
reads HOME/.config/ravel/settings.json, and it writes only that file:
    absent file          a new file that holds only the off value
    absent "skills"      the same members in the same order, then the off value
    the off value        no change
    another value, a malformed file, a link or another owner: a stop
A repeated member name or a non-finite number is a malformed file. Another
Ravel process stops it too, as the narrow process inspection below
recognises it. Only this process and its exact ancestors are exempt.
The new bytes go to a temporary sibling. It receives the original's mode,
owner, flags, ACL and extended attributes, except com.apple.provenance, and
F_FULLFSYNC flushes it. A check of its members and metadata comes before one
rename onto the live file. F_FULLFSYNC then flushes the folder, and a reading
confirms the bytes. The live file is never truncated. An interruption leaves
the original or the complete new file, and the same command completes
either; it removes each temporary sibling that an interruption left. Exit
status: 0 when the off value is in place, 3 for a stop with no write to the
settings, 4 when a write failed. After 3 or 4, native delivery is incomplete
and no new default launch may start. --print-command prints the exact
command, executable, program, HOME and settings path as one JSON object for
the prepared recovery instructions, and writes nothing.

The narrow process inspection ("The narrow process inspection" and "Probe
process lifetime"). Each gate of the switch, the hooks installation and the
inverse runs it. It reads the process table and signals no process. A
process is one of the four tools when:
    its executable (proc_pidpath) has the basename claude, codex, grove,
    grove-llm or ravel, or a basename that starts with "codex-", as Codex's
    helpers codex-exec-server and codex-code-mode-host do;
    its executable lies in Claude Code's .../claude/versions/ folder;
    or its executable is node, nodejs or bun and its title is claude or
    codex, or its entry script resolves into @anthropic-ai/claude-code or
    @openai/codex, or has the stem claude or codex.
The argument vector comes from KERN_PROCARGS2, so an argument with a space
stays one argument. The title is the basename of argv[0], or of its first
word when a process rewrote argv[0] into words, as Node servers do. A
relative entry script resolves against the process's own folder
(PROC_PIDVNODEPATHINFO). Only names in the program position count, so a
tool's name inside another argument is not recognition. The comparison is
case-sensitive: the desktop applications Claude and ChatGPT, Pi and every
other writer stay the owner's checklist obligations. When the executable is
unreadable, as for a binary that an upgrade deleted, the launch path in
KERN_PROCARGS2 can still identify a tool. Each other process with an
unreadable executable needs review. So does a process whose title, launch
path or script of a shell, Python, Perl or Ruby interpreter names a tool
without a known executable form of it. Zombies and processes that exit
during the inspection do not count. The program admits itself and its exact
ancestors up to the first harness (Claude Code or Codex), and the drivers
(Ravel and Grove) above it, with no descendant exemption. A second harness
ancestor is another session, so it and each tool above it stop the step. A
recognised process is also admitted when its PID, creation time
(PROC_PIDTBSDINFO, "<seconds>.<microseconds>"), canonical executable and
HOME equal a probe-owned record. Only inspections during probes may pass
such records; the inspection after probe cleanup and the one before restore
pass none. Each other recognised process, and each process that needs
review, stops the step. Processes of other accounts are recognised by their
executable only, because their arguments, folder and creation time need
privilege, and they never need review.
`processes` runs the inspection, prints each admitted and remaining process
and changes nothing. --owned names a JSON list of probe-owned records, each
{"pid", "start", "executable", "home"}, as `processes --json` reports them.
Exit status: 0 when no other process remains, 3 otherwise.
OWNER_SWITCH_REHEARSAL_SCOPE limits the inspection to executables inside one
folder. Like the rehearsal hook, the program refuses it when HOME is the
account's home directory.

The owner's layout: the real plan names .local/state/owner-switch as its
state. The prepared plan and program are <state>/prepared/plan.json and
<state>/prepared/owner_switch.py. `mac-check` runs that program's `state
--json` with that plan.

The plan (JSON, schema "owner-switch-plan/1"):
    state        directory of backups and rehearsal records, relative to HOME
    manual       the maintained manual, relative to HOME
    parts        ordered parts. {"kind": "entry", "path", "identity"} or
                 {"kind": "file", "path", "cells": [{"key": [...],
                 "earlier": V, "switched": V}]}. A value {"$absent": true}
                 is an absent key. Entries come first.
    outside      entries outside the switch set: {"path", "identity"}, or
                 {"root", "entries": {name: identity}} for each child of a
                 directory that no part names. The comparison skips only
                 the platform subtrees .claude/skills/synced and
                 .codex/skills/.system, each while it is a plain directory.
    baseline     command: the baseline instrument that step 3 reruns
    gates        {"4": [command], "5": [...], "6": [...], "7": [...]}
    accepted     the accepted values that the gates read:
                 sources   {id: {"formula", "resourceDigest"}}, exactly the
                           ten IDs of "Ravel sources"
                 versions  {formula: version} for each skill formula except
                           Grove's, whose accepted version step 4 admits
                 trials    {id: [command]}: the staging trials of a source
                 staging   {"home", "candidate_keg"}: the absolute paths of
                           the retained staging HOME and the candidate keg
    probes       [command]: the probes of step 10
    native_probe [command]: the native probe of Rollback
    mac_check    command: step 12
Each field is required. Each list of commands names at least one command, and
the list of each of steps 4 to 7 names that step's gate of this program. A
plan that holds "session", "watch" or "jobs" is refused: it holds no process,
MCP or job pattern. The
switch's steps are fixed, so no plan can omit one of their checks.
A command is {"name", "argv"}. The argv items "$python" and "$program" become
this program's interpreter and path, so a continuation runs the gates of the
program copy in its backup. A command runs in HOME, with HOME set to HOME. Its
environment holds none of Grove's loop control variables, Claude Code's five
location variables, CODEX_HOME or a RAVEL_* name. It receives
OWNER_SWITCH_HOME, OWNER_SWITCH_STATE, OWNER_SWITCH_STEP, OWNER_SWITCH_PLAN,
the path of the plan that names it, and OWNER_SWITCH_PLAN_DIGEST, the digest
of the bytes that the program admitted. Its last line of standard output may
be `owner-switch-result: <JSON object>`, which the evidence keeps as "result".

The gates. A plan names `["$python", "-I", "$program", "gate", "<step>"]`.
A gate that the program launches stops unless the plan at OWNER_SWITCH_PLAN
has the digest OWNER_SWITCH_PLAN_DIGEST.
    4  Stop unless the installed Grove is the tap's current version of
       linkuistics/taps/grove, with its keg linked and $(brew --prefix)/opt/grove
       reaching it. Resolve the drafted real settings in a disposable HOME
       with $(brew --prefix)/bin/ravel. For each of Grove's sources whose
       resourceDigest differs from its accepted digest, name the source and
       run its trials, with OWNER_SWITCH_SOURCE and OWNER_SWITCH_DIGEST. Each
       trial runs in the staging HOME, with HOME set to it and CODEX_HOME set
       to its own .codex tree. Stop if a source has no trial, a trial fails,
       or the staging HOME or the candidate keg is missing. The result
       admits the Grove version and each rerun source's digest.
    5  Stop unless `mise -C HOME bootstrap packages status --json` shows each
       skill formula declared, installed and at its accepted version.
    6  Resolve the drafted real settings again. Stop unless the resolved IDs
       are exactly the accepted ten and each resourceDigest is accepted.
    7  Record each registration of each plugin that enabledPlugins enables in
       .claude/settings.json, and of each directory that OWNER_SWITCH_READERS
       names (colon-separated; finish sets it from the cache paths that it
       read). Stop if a directory is missing, holds .orphaned_at, or was named
       but has no registration in .claude/plugins/installed_plugins.json.
The drafted real settings are the switched cells of the plan's part for
.config/ravel/settings.json. They must name no "sources". The program passes
step 4's admitted values to the later gates as OWNER_SWITCH_ADMITTED, and the
active continuation reads them from the step-4 evidence of its backup.

Step 3 runs the baseline command after the preludes. Each output row with a
field UNCLASSIFIED, UNASSIGNED, UNCOVERED or NEEDS-REVIEW counts as a changed
entry outside the switch set. The program prints each row whose field says
"unverified", for finish to compare with its own skill listing. `state` does
not run the baseline.

A passing rehearsal record is a JSON file in <state>/rehearsals with
"passed": true, "plan_digest", "program_digest" and "settings_writes", the
rerun of settings-writes.sh with "passed": true and its "claude_version".
Step 8 needs a record that names the plan and the program. The preludes
need a record that names the program only, because a prelude applies no
plan. The hooks installation runs a Claude Code command only when `claude
--version` prints a version that a passing record names.

Inside the probes, the run reads each version directory of step 7's evidence
in the backup again. A missing directory, an orphan marker or a lost
registration counts as a failed probe. The receipt holds the activated
configuration, each source's keg path, formula version and snapshot, the
trials of step 4, the reader evidence, the backup and each retained byte set
with its purge condition. The backup keeps the evidence of steps 4 to 7.

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
The program finds `claude` and `brew` on PATH. Each runs in HOME, with the
environment of a plan's command.

A settings record is a directory in <state>/settings, named under a
temporary name until it is complete. It holds record.json, the file `copy`
and the file `read`, the bytes that its reading read. For an exchange, `copy`
is the stage file and, after the exchange, the kept copy. Its record.json
holds the digests "read" and "stage". In a backup, the bytes that an exchange
read are <copy>.read beside the kept copy.

For a Claude Code command, `copy` is the held link and record.json holds
"read", the digest of the one reading through it. A held link whose inode is still
at the path holds no kept copy. The owner's merge of a flagged copy is
`copy.merge-<n>.json` with its "digest" and "content", as in a backup.

Rehearsal hook: OWNER_SWITCH_REHEARSAL_HOOK names a command that the program
runs at each named point with the point as its argument. Exit 99 ends the
program at once, as a crash. The program refuses the hook when HOME is the
account's home directory, so a real run never honours it.
"""
from __future__ import annotations

import argparse
import base64
import ctypes
import datetime
import errno
import fcntl
import hashlib
import json
import math
import os
import pwd
import re
import secrets
import shlex
import stat
import shutil
import subprocess
import sys
import tempfile

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
PLAN_VAR = "OWNER_SWITCH_PLAN"
PLAN_DIGEST_VAR = "OWNER_SWITCH_PLAN_DIGEST"
ADMITTED_VAR = "OWNER_SWITCH_ADMITTED"
READERS_VAR = "OWNER_SWITCH_READERS"
RESULT_MARK = "owner-switch-result: "
# The bytes that an exchange in a backup read, beside its kept copy.
READ_SUFFIX = ".read"

# The gates: docs/specs/owner-skill-delivery.md, steps 4 to 7 of "The switch
# inside finish", "Ravel sources" and "Skill formulae".
FORMULA_OF_GROVE = "linkuistics/taps/grove"
SOURCE_IDS = ("design", "engineering", "grove-core", "personal", "quint", "requirements",
              "research", "superpowers", "testanyware", "writing")
# docs/evaluations/owner-skill-delivery/inventory.py: a row that counts as a
# changed entry, and the digest of a row that only finish's listing verifies.
ANOMALIES = ("UNCLASSIFIED", "UNASSIGNED", "UNCOVERED", "NEEDS-REVIEW")
UNVERIFIED = "unverified"

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
# Codex keeps its configuration, hooks, plugins and skills under CODEX_HOME.
CODEX_HOME = "CODEX_HOME"
RAVEL_CONFIG = ".config/ravel/config.rhai"
# "Purge": the five conditions of each retained byte set.
PURGE_CONDITIONS = (
    "no session that ran during the switch remains",
    "the driver's group is reaped",
    "no kept copy is flagged",
    "each retained entry keeps the change identity that its move recorded",
    "the owner decides that the rollback window is closed",
)
# Grove 22.8's Codex snapshot store ("Resource lifetime").
CODEX_SNAPSHOT_STORE = ".agents/.grove"

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
    try:
        plan = json.loads(raw)
    except ValueError:
        raise Stop(f"{path} is not JSON")
    if not isinstance(plan, dict) or plan.get("schema") != PLAN_SCHEMA:
        raise Stop(f"{path} is not a plan of schema {PLAN_SCHEMA}")
    validate_plan(plan)
    plan["_digest"] = hashlib.sha256(raw).hexdigest()
    plan["_bytes"] = raw
    plan["_path"] = os.path.abspath(path)
    return plan


def plan_path_problem(rel) -> str:
    """Why a plan path is not a normalised path relative to HOME, or ""."""
    if not isinstance(rel, str) or not rel:
        return "is not a path"
    if os.path.isabs(rel) or rel.startswith("~"):
        return "is not relative to HOME"
    if any(c in ("", ".", "..") for c in rel.split("/")):
        return "is not a normalised relative path without .."
    return ""


# "The narrow process inspection": the plan holds no process, MCP or job pattern.
REMOVED_PLAN_FIELDS = ("session", "watch", "jobs")


def validate_plan(plan: dict) -> None:
    """Refuse a plan that lacks a field, a check or a HOME-relative path.

    Each step of "The switch inside finish" is fixed, so no plan can omit
    the commands of steps 3 to 7, 10, 11 and 12. Each gate list names the
    program's own gate of its step.
    """
    problems = []

    def path(where: str, rel) -> None:
        why = plan_path_problem(rel)
        if why:
            problems.append(f"{where} {rel!r} {why}")

    def command(where: str, value) -> None:
        if (not isinstance(value, dict) or not isinstance(value.get("name"), str)
                or not isinstance(value.get("argv"), list) or not value["argv"]
                or not all(isinstance(a, str) for a in value["argv"])):
            problems.append(f"{where} is not a command with a name and an argv")

    def commands(where: str, value) -> None:
        if not isinstance(value, list) or not value:
            problems.append(f"{where} names no command")
            return
        for i, c in enumerate(value):
            command(f"{where}[{i}]", c)

    path("state", plan.get("state"))
    path("manual", plan.get("manual"))
    parts = plan.get("parts")
    if not isinstance(parts, list) or not parts:
        problems.append("parts names no part")
        parts = []
    seen = set()
    for i, part in enumerate(parts):
        if not isinstance(part, dict) or part.get("kind") not in ("entry", "file"):
            problems.append(f"part {i} is neither an entry nor a file")
            continue
        path(f"part {i}", part.get("path"))
        if part.get("path") in seen:
            problems.append(f"part {i} names {part.get('path')!r} twice")
        seen.add(part.get("path"))
        if part["kind"] == "entry":
            if not re.fullmatch(r"[0-9a-f]{64}", str(part.get("identity"))):
                problems.append(f"part {i} has no change identity")
            continue
        cells = part.get("cells")
        if not isinstance(cells, list) or not cells:
            problems.append(f"part {i} names no cell")
            continue
        for cell in cells:
            key = cell.get("key") if isinstance(cell, dict) else None
            if (not isinstance(key, list) or not key or not all(isinstance(k, str) for k in key)
                    or "earlier" not in cell or "switched" not in cell):
                problems.append(f"part {i} has a cell without a key, an earlier and a switched value")
    kinds = [p.get("kind") for p in parts if isinstance(p, dict)]
    if kinds != sorted(kinds, key=lambda k: k != "entry"):
        problems.append("the plan's entry parts must come before its file parts")
    outside = plan.get("outside")
    if not isinstance(outside, list) or not outside:
        problems.append("outside names no entry outside the switch set")
        outside = []
    for i, item in enumerate(outside):
        if isinstance(item, dict) and "root" in item:
            path(f"outside {i}", item["root"])
            if not isinstance(item.get("entries"), dict):
                problems.append(f"outside {i} has no entries")
            else:
                for name in item["entries"]:
                    if not name or "/" in name or name in (".", ".."):
                        problems.append(f"outside {i} names the child {name!r}")
        elif isinstance(item, dict) and "path" in item and "identity" in item:
            path(f"outside {i}", item["path"])
        else:
            problems.append(f"outside {i} is neither a path with an identity nor a root with entries")
    removed = [name for name in REMOVED_PLAN_FIELDS if name in plan]
    if removed:
        problems.append(f"the plan holds {', '.join(removed)}: the narrow process inspection is built in, "
                        "and the owner's checklist replaces process, MCP and job patterns")
    command("baseline", plan.get("baseline"))
    gates = plan.get("gates")
    if not isinstance(gates, dict) or sorted(gates) != ["4", "5", "6", "7"]:
        problems.append("gates must name exactly the steps 4, 5, 6 and 7")
    else:
        for step in ("4", "5", "6", "7"):
            commands(f"gates.{step}", gates[step])
            if not any(isinstance(c, dict) and c.get("argv") == ["$python", "-I", "$program", "gate", step]
                       for c in gates[step] if isinstance(gates[step], list)):
                problems.append(f"gates.{step} does not run the program's gate of step {step}")
    if not isinstance(plan.get("accepted"), dict):
        problems.append("accepted is missing")
    else:
        try:
            staging_of(plan)
        except Stop as e:
            problems.append(str(e))
    commands("probes", plan.get("probes"))
    commands("native_probe", plan.get("native_probe"))
    command("mac_check", plan.get("mac_check"))
    if problems:
        raise Stop("the plan is incomplete: " + "; ".join(problems))


def account_home() -> str:
    return os.path.realpath(pwd.getpwuid(os.getuid()).pw_dir)


class Layout:
    def __init__(self, home: str, plan: dict):
        self.home = os.path.realpath(home)
        self.contain(plan)
        self.state = os.path.join(self.home, plan["state"])
        self.backups = os.path.join(self.state, "backups")
        self.rehearsals = os.path.join(self.state, "rehearsals")
        self.settings = os.path.join(self.state, "settings")
        self.manual = os.path.join(self.home, plan["manual"])

    def path(self, rel: str) -> str:
        return os.path.join(self.home, rel)

    def contain(self, plan: dict) -> None:
        """Stop unless each path of the plan stays inside HOME.

        The loader refused absolute paths and `..`. A linked parent folder
        can still lead outside HOME, so each path's parent must resolve
        inside it. The last component is not followed: an entry can be a
        link.
        """
        rels = [plan["state"], plan["manual"]] + [p["path"] for p in plan["parts"]]
        for item in plan["outside"]:
            # A root's children are read below it, so the root itself is a parent.
            rels += [item["root"] + "/child"] if "root" in item else [item["path"]]
        for rel in rels:
            parent = os.path.realpath(os.path.dirname(self.path(rel)))
            if parent != self.home and not parent.startswith(self.home + os.sep):
                raise Stop(f"the plan path {rel} leads outside HOME through {parent}")


def log(line: str) -> None:
    print(line, flush=True)


def hook(layout: Layout, point: str) -> None:
    hook_at(layout.home, point)


def hook_at(home: str, point: str) -> None:
    command = os.environ.get(HOOK_VAR)
    if not command:
        return
    if home == account_home():
        raise Stop(f"{HOOK_VAR} is set while HOME is the account's home directory")
    result = subprocess.run([command, point], env=dict(os.environ, OWNER_SWITCH_HOME=home))
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

    def evidence(self, step: str) -> list:
        """The command results of one step that step 8 kept in this backup."""
        try:
            with open(os.path.join(self.path, "evidence", f"step-{step}.json")) as f:
                body = json.load(f)
        except (OSError, ValueError):
            return []
        return body if isinstance(body, list) else []


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
    return [m["digest"] for m in merge_records(copy)]


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


def flagged_summary(record: dict) -> dict:
    return {"copy": record["_copy"], "of": record["path"], "backup": record["_backup"],
            "kind": "held" if record.get("kind") == "held" else "kept"}


# --- the merge entry ----------------------------------------------------------------

def merge_records(copy: str) -> list:
    """The merge records of a kept copy, oldest first, with their file names."""
    folder, base = os.path.split(copy)
    names = sorted(n for n in os.listdir(folder) if n.startswith(base + ".merge-") and n.endswith(".json"))
    out = []
    for n in names:
        with open(os.path.join(folder, n)) as f:
            out.append(dict(json.load(f), _name=n))
    return out


def recorded_base(record: dict) -> bytes:
    """The last recorded content of a kept copy, as bytes.

    Before a merge, that is the content that the run read. After a merge,
    it is the content that the last merge recorded.
    """
    merges = merge_records(record["_copy"])
    if merges:
        if "content" not in merges[-1]:
            raise Stop(f"the last merge of {record['_copy']} recorded no content; merge by hand, "
                       "then record the merge with --record-only")
        return base64.b64decode(merges[-1]["content"])
    if record["_backup"] is not None:
        read = record["_copy"] + READ_SUFFIX
    else:
        read = os.path.join(os.path.dirname(record["_copy"]), "read")
    try:
        with open(read, "rb") as f:
            return f.read()
    except FileNotFoundError:
        raise Stop(f"{record['_copy']} keeps no bytes of what its run read; merge by hand, "
                   "then record the merge with --record-only")


def known_cells(layout: Layout, record: dict, prepared: dict) -> dict:
    """Each cell key of the copy's file, with the values that a run writes there.

    A cell takes the copy's value only when it is none of these values.
    """
    plan = prepared
    if record["_backup"] is not None:
        plan = Backup(os.path.join(layout.backups, record["_backup"])).plan
    known = {}
    for p in (plan, prepared):
        for part in p["parts"]:
            if part["kind"] == "file" and part["path"] == record["path"]:
                for cell in part["cells"]:
                    known.setdefault(tuple(cell["key"]), []).extend([cell["earlier"], cell["switched"]])
    if record["path"] == RAVEL_SETTINGS:
        known.setdefault(("skills",), []).extend([ABSENT, OFF])
    if record["path"] == CLAUDE_SETTINGS:
        known.setdefault(tuple(HOOKS_KEY), []).extend([ABSENT, True, False])
    return known


def merge_difference(base: dict, copy: dict, known: dict) -> list:
    """The copy's difference from its base, as (key, value) pairs; ABSENT erases a key.

    Each other key that the copy gained, changed or lost is in the
    difference. A cell is in it only when the copy holds a value that no run
    writes there.
    """
    ops = []
    for key, values in sorted(known.items()):
        value = get_cell(copy, list(key))
        if not same(value, get_cell(base, list(key))) and not any(same(value, v) for v in values):
            ops.append((list(key), value))

    def walk(b: dict, c: dict, path: tuple) -> None:
        for k in sorted(set(b) | set(c)):
            p = path + (k,)
            if p in known:
                continue
            bv, cv = b.get(k, ABSENT), c.get(k, ABSENT)
            if same(bv, cv):
                continue
            holds_cell = any(len(key) > len(p) and key[:len(p)] == p for key in known)
            if isinstance(bv, dict) and isinstance(cv, dict):
                walk(bv, cv, p)
            elif holds_cell and isinstance(bv, dict) and same(cv, ABSENT):
                walk(bv, {}, p)
            elif holds_cell and same(bv, ABSENT) and isinstance(cv, dict):
                walk({}, cv, p)
            elif holds_cell:
                raise Stop(f"the copy replaces {key_name(list(p))}, which holds a cell; merge by hand, "
                           "then record the merge with --record-only")
            else:
                ops.append((list(p), cv))
    walk(base, copy, ())
    return ops


def apply_difference(obj: dict, ops: list) -> list:
    replaced = []
    for key, value in ops:
        old = get_cell(obj, key)
        if same(value, ABSENT):
            parent = get_cell(obj, key[:-1]) if len(key) > 1 else obj
            if isinstance(parent, dict) and key[-1] in parent:
                del parent[key[-1]]
                replaced.append((key_name(key), old, value))
        elif not same(old, value):
            set_cell(obj, key, value)
            replaced.append((key_name(key), old, value))
    return replaced


def merge_command(args) -> int:
    """Merge a flagged kept copy into its live file, and record the merge.

    "Concurrent writers": the merge applies the difference between the copy
    and its last recorded content, an erased key included. Its record holds
    the digest of the copy, which is from then on the only recorded content.
    """
    prepared = load_plan(args.plan)
    layout = Layout(args.home, prepared)
    wanted = os.path.realpath(args.copy)
    found = [r for r in kept_copy_records(layout) if os.path.realpath(r["_copy"]) == wanted]
    if not found:
        raise Stop(f"{args.copy} is no kept copy of a complete backup or settings record")
    record = found[0]
    if not is_flagged(record):
        raise Stop(f"{args.copy} holds its recorded content, so it is not flagged and needs no merge")
    fd = os.open(record["_copy"], os.O_RDONLY | os.O_NOFOLLOW)
    try:
        digest, content = fd_reading(fd)
    finally:
        os.close(fd)
    applied = []
    if args.record_only:
        log(f"merge: recording {args.copy} without applying a difference, as the owner asked")
    else:
        base, copy = parse_json(recorded_base(record)), parse_json(content)
        if same(base, UNREADABLE) or same(copy, UNREADABLE):
            raise Stop(f"{args.copy} or its recorded content is not a JSON object; merge by hand, "
                       "then record the merge with --record-only")
        ops = merge_difference(base, copy, known_cells(layout, record, prepared))
        if ops:
            applied = record_exchange(layout, record["path"], lambda obj: apply_difference(obj, ops), "merge")
        if not applied:
            log(f"merge: the difference of {args.copy} changes no key of {record['path']}")
    number = len(merge_records(record["_copy"])) + 1
    body = {"digest": digest, "content": base64.b64encode(content).decode(), "at": now(),
            "record_only": bool(args.record_only),
            "applied": [{"key": k, "from": old, "to": new} for k, old, new in applied]}
    write_exclusive(f"{record['_copy']}.merge-{number:03d}.json", dump_json(body))
    log(f"record: merge {number} of {args.copy}, with the digest {digest}")
    return 0


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


# "Concurrent writers": the only subtrees that a comparison skips. A probe
# session can rewrite them. Their ordinary siblings, and a link or file in
# place of either, are still compared.
PLATFORM_SUBTREES = (".claude/skills/synced", ".codex/skills/.system")


def platform_subtree(layout: Layout, rel: str) -> bool:
    p = layout.path(rel)
    return rel in PLATFORM_SUBTREES and os.path.isdir(p) and not os.path.islink(p)


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
                if platform_subtree(layout, rel):
                    continue
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


def find_state(layout: Layout, prepared: dict, baseline=()) -> dict:
    """Find the state of the switch set from the layout, the run record and a plan.

    baseline holds the rows of step 3's baseline instrument that count as
    changed entries outside the switch set.
    """
    backup = latest_backup(layout)
    is_open = bool(backup and backup.is_open())
    plan = backup.plan if is_open else prepared
    layout.contain(plan)
    parts =[part_status(layout, p, backup if is_open else None) for p in plan["parts"]]
    statuses = [p["status"] for p in parts]
    outside = outside_changes(layout, plan) + list(baseline)
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


# --- the narrow process inspection ---------------------------------------------
# docs/specs/owner-skill-delivery.md, "The narrow process inspection" and "Probe
# process lifetime". The owner's checklist covers every other writer.

SCOPE_VAR = "OWNER_SWITCH_REHEARSAL_SCOPE"
_libproc = ctypes.CDLL("/usr/lib/libproc.dylib", use_errno=True)
_libproc.proc_pidpath.argtypes = [ctypes.c_int, ctypes.c_void_p, ctypes.c_uint32]
_libproc.proc_pidinfo.argtypes = [ctypes.c_int, ctypes.c_int, ctypes.c_uint64, ctypes.c_void_p, ctypes.c_int]
_libc.sysctl.argtypes = [ctypes.POINTER(ctypes.c_int), ctypes.c_uint, ctypes.c_void_p,
                         ctypes.POINTER(ctypes.c_size_t), ctypes.c_void_p, ctypes.c_size_t]
# <sys/sysctl.h>: CTL_KERN, KERN_ARGMAX and KERN_PROCARGS2. <sys/proc_info.h>:
# PROC_PIDTBSDINFO and the layout of struct proc_bsdinfo.
CTL_KERN, KERN_ARGMAX, KERN_PROCARGS2 = 1, 8, 49
PROC_PIDTBSDINFO = 3
# PROC_PIDVNODEPATHINFO gives struct proc_vnodepathinfo: the current folder's
# vnode_info (152 bytes), then its path (MAXPATHLEN), then the root folder's.
PROC_PIDVNODEPATHINFO, VNODE_INFO_SIZE, MAXPATHLEN = 9, 152, 1024
SZOMB = 5


class _BsdInfo(ctypes.Structure):
    _fields_ = [("flags", ctypes.c_uint32), ("status", ctypes.c_uint32), ("xstatus", ctypes.c_uint32),
                ("pid", ctypes.c_uint32), ("ppid", ctypes.c_uint32), ("uid", ctypes.c_uint32),
                ("gid", ctypes.c_uint32), ("ruid", ctypes.c_uint32), ("rgid", ctypes.c_uint32),
                ("svuid", ctypes.c_uint32), ("svgid", ctypes.c_uint32), ("rfu_1", ctypes.c_uint32),
                ("comm", ctypes.c_char * 16), ("name", ctypes.c_char * 32), ("nfiles", ctypes.c_uint32),
                ("pgid", ctypes.c_uint32), ("pjobc", ctypes.c_uint32), ("e_tdev", ctypes.c_uint32),
                ("e_tpgid", ctypes.c_uint32), ("nice", ctypes.c_int32), ("start_tvsec", ctypes.c_uint64),
                ("start_tvusec", ctypes.c_uint64)]


# The executable basenames of the four tools. Grove has two executables.
# Codex installs its helpers codex-exec-server and codex-code-mode-host beside
# codex, so each basename that starts with "codex-" is Codex too.
TOOLS = {"claude": "Claude Code", "codex": "Codex", "grove": "Grove", "grove-llm": "Grove", "ravel": "Ravel"}
CODEX_HELPER = "codex-"
HARNESSES = ("Claude Code", "Codex")
# The two tools that also run as Node launchers, and their npm packages.
NODE_TOOLS = {"claude": "Claude Code", "codex": "Codex"}
NODE_PACKAGES = {"@anthropic-ai/claude-code": "Claude Code", "@openai/codex": "Codex"}
NODE_RUNNERS = ("node", "nodejs", "bun")
# Node options that take the next argument as their value, and options after
# which no entry script follows.
NODE_VALUE_OPTIONS = ("-r", "--require", "--import", "--loader", "--experimental-loader", "-C",
                      "--conditions", "--env-file", "--env-file-if-exists", "--title", "--inspect-port",
                      "--input-type", "--watch-path", "--disable-warning")
NODE_NO_SCRIPT = ("-e", "--eval", "-p", "--print", "-", "-i", "--interactive")
# Interpreters whose script argument can carry a tool's name, as a wrapper.
SHELLS = ("sh", "bash", "zsh", "dash", "ksh", "fish")
SHELL_VALUE_OPTIONS = ("--rcfile", "--init-file", "-o", "-O", "+o", "+O")
PYTHON_VALUE_OPTIONS = ("-X", "-W")
SCRIPT_SUFFIXES = (".js", ".mjs", ".cjs", ".sh", ".py")
# Claude Code's native installer keeps each version as .../claude/versions/<version>.


def executable_path(pid: int):
    """The canonical path of a process's executable (proc_pidpath), or None."""
    buf = ctypes.create_string_buffer(4096)
    if _libproc.proc_pidpath(pid, buf, 4096) <= 0:
        return None
    return os.fsdecode(buf.value)


def process_start(pid: int):
    """The creation time of a process as "<seconds>.<microseconds>", or None.

    PROC_PIDTBSDINFO reads it only for the account's own processes.
    """
    info = _BsdInfo()
    if _libproc.proc_pidinfo(pid, PROC_PIDTBSDINFO, 0, ctypes.byref(info), ctypes.sizeof(info)) != ctypes.sizeof(info):
        return None
    return f"{info.start_tvsec}.{info.start_tvusec:06d}"


def process_status(pid: int):
    """The BSD status of one of the account's processes (SZOMB for a zombie), or None when it is gone."""
    info = _BsdInfo()
    if _libproc.proc_pidinfo(pid, PROC_PIDTBSDINFO, 0, ctypes.byref(info), ctypes.sizeof(info)) != ctypes.sizeof(info):
        return None
    return info.status


def process_folder(pid: int):
    """The current folder of one of the account's processes, or None."""
    size = 2 * (VNODE_INFO_SIZE + MAXPATHLEN)
    buf = ctypes.create_string_buffer(size)
    if _libproc.proc_pidinfo(pid, PROC_PIDVNODEPATHINFO, 0, buf, size) != size:
        return None
    path = buf.raw[VNODE_INFO_SIZE:VNODE_INFO_SIZE + MAXPATHLEN].split(b"\0", 1)[0]
    return os.fsdecode(path) or None


def _argmax() -> int:
    value, size = ctypes.c_int(0), ctypes.c_size_t(ctypes.sizeof(ctypes.c_int))
    mib = (ctypes.c_int * 2)(CTL_KERN, KERN_ARGMAX)
    if _libc.sysctl(mib, 2, ctypes.byref(value), ctypes.byref(size), None, 0) != 0:
        return 1 << 20
    return value.value


def process_arguments(pid: int):
    """The launch path, the argument vector and HOME of a process (KERN_PROCARGS2), or (None, None, None).

    The kernel keeps the argument vector as separate strings, so an argument
    with a space stays one argument. A process can rewrite its own vector, as
    Node does for its title. The environment follows the vector, and only its
    HOME is read. The kernel pads the launch path with NUL bytes, so an empty
    argv[0] cannot be told from the padding.
    """
    size = ctypes.c_size_t(_argmax())
    buf = ctypes.create_string_buffer(size.value)
    mib = (ctypes.c_int * 3)(CTL_KERN, KERN_PROCARGS2, pid)
    if _libc.sysctl(mib, 3, buf, ctypes.byref(size), None, 0) != 0 or size.value < 4:
        return None, None, None
    raw = buf.raw[:size.value]
    argc = int.from_bytes(raw[:4], sys.byteorder)
    rest = raw[4:]
    end = rest.find(b"\0")
    if end < 0:
        return None, None, None
    launch = os.fsdecode(rest[:end])
    items = rest[end:].lstrip(b"\0").split(b"\0")
    home = None
    for item in items[argc:]:
        if not item:
            break
        if item.startswith(b"HOME="):
            home = os.fsdecode(item[5:])
            break
    return launch, [os.fsdecode(x) for x in items[:argc]], home


def process_table() -> dict:
    """Each process as {pid: {"ppid", "uid", "zombie"}}, from ps."""
    result = subprocess.run(["/bin/ps", "-axww", "-o", "pid=,ppid=,uid=,stat="], capture_output=True, text=True)
    if result.returncode != 0:
        raise Stop(f"the process listing failed: {result.stderr.strip()}")
    out = result.stdout
    procs = {}
    for line in out.splitlines():
        parts = line.split()
        if len(parts) >= 4:
            procs[int(parts[0])] = {"ppid": int(parts[1]), "uid": int(parts[2]), "zombie": "Z" in parts[3]}
    return procs


def _stem(path: str) -> str:
    name = os.path.basename(path)
    for suffix in SCRIPT_SUFFIXES:
        if name.endswith(suffix):
            return name[:-len(suffix)]
    return name


def _script(argv: list, value_options=(), no_script=()):
    """The first argument after argv[0] that is not an option, or None."""
    i = 1
    while i < len(argv):
        arg = argv[i]
        if arg in no_script:
            return None
        if arg == "--":
            return argv[i + 1] if i + 1 < len(argv) else None
        if arg.startswith("-"):
            i += 2 if arg in value_options else 1
            continue
        return arg
    return None


def _tool_executable(path):
    """The tool and the form of an executable path, or None."""
    if not path:
        return None
    name = os.path.basename(path)
    if name in TOOLS:
        return TOOLS[name], "native executable"
    if name.startswith(CODEX_HELPER):
        return "Codex", "helper executable"
    parts = path.split("/")
    if len(parts) >= 3 and parts[-2] == "versions" and parts[-3] == "claude":
        return "Claude Code", "versioned executable"
    return None


def _role(tool: str, argv: list) -> str:
    """The subcommand of a recognised process, for the report."""
    rest = argv[1:]
    if tool == "Codex" and "app-server" in rest:
        i = rest.index("app-server")
        if rest[i + 1:i + 2] == ["daemon"]:
            return "app-server daemon"
        return "managed app-server" if "--managed-daemon" in rest else "app-server"
    first = _title(argv)
    if first and first not in TOOLS and not first.startswith(("node", "-")):
        return f"as {first}"
    return ""


def _title(argv: list) -> str:
    """The program name in argv[0]. A title with spaces, as a Node process rewrites it, gives its first word."""
    first = argv[0] if argv else ""
    words = first.split()
    return os.path.basename(words[0] if len(words) > 1 else first)


def recognise(executable, launch, argv, resolve=os.path.realpath):
    """Classify one process as a tool, as needing review, or as neither.

    EXECUTABLE is the canonical path (proc_pidpath), LAUNCH the path that
    started the process and ARGV its argument vector (KERN_PROCARGS2). Either
    can be None when it is unreadable. The result is None, {"tool", "form",
    "role"} or {"review", "names"}. Only names in the program position count:
    a tool's name inside another argument is never recognition.
    """
    argv = argv or []
    path = executable or launch
    unread = "" if executable else ", executable unreadable, by its launch path"
    found = _tool_executable(path)
    if found:
        return {"tool": found[0], "form": found[1] + unread, "role": _role(found[0], argv)}
    first = _title(argv)
    named = {n for n in (first, os.path.basename(launch or "")) if n in TOOLS}
    runner = os.path.basename(path or "")
    if runner in NODE_RUNNERS:
        if first in NODE_TOOLS:
            tool = NODE_TOOLS[first]
            return {"tool": tool, "form": f"Node launcher titled {first}" + unread, "role": _role(tool, argv)}
        script = _script(argv, NODE_VALUE_OPTIONS, NODE_NO_SCRIPT)
        if script:
            real = resolve(script)
            for package, tool in NODE_PACKAGES.items():
                if f"/node_modules/{package}/" in real:
                    return {"tool": tool, "form": f"Node launcher of {real}" + unread, "role": _role(tool, argv)}
            for candidate in (script, real):
                if _stem(candidate) in NODE_TOOLS:
                    tool = NODE_TOOLS[_stem(candidate)]
                    return {"tool": tool, "form": f"Node launcher of {real}" + unread, "role": _role(tool, argv)}
                if _stem(candidate) in TOOLS:
                    named.add(_stem(candidate))
    elif runner in SHELLS or runner.startswith("python") or runner in ("perl", "ruby"):
        if runner in SHELLS:
            script = _script(argv, SHELL_VALUE_OPTIONS, ("-c",))
        else:
            script = _script(argv, PYTHON_VALUE_OPTIONS if runner.startswith("python") else (), ("-c", "-m", "-e"))
        if script and _stem(script) in TOOLS:
            named.add(_stem(script))
    if named:
        names = ", ".join(sorted(named))
        return {"review": f"the launch form names {names}, but the executable {path or 'unreadable'} "
                          f"is not a known form of it", "names": sorted(TOOLS[n] for n in named)}
    if path is None:
        return {"review": "the executable identity and the argument vector are unreadable", "names": []}
    if executable is None:
        return {"review": f"the executable is unreadable, so its identity is unknown; it was launched as {launch}",
                "names": []}
    return None


def ancestors_of(procs: dict, pid: int) -> list:
    """The exact ancestors of PID, nearest first."""
    out = []
    while pid in procs and procs[pid]["ppid"] not in (0, pid):
        pid = procs[pid]["ppid"]
        out.append(pid)
    return out


def rehearsal_scope(home: str):
    """The folder of SCOPE_VAR with a trailing separator, or None."""
    scope = os.environ.get(SCOPE_VAR)
    if scope and os.path.realpath(home) == account_home():
        raise Stop(f"{SCOPE_VAR} is set while HOME is the account's home directory")
    return os.path.realpath(scope) + os.sep if scope else None


def load_owned(path):
    """The probe-owned processes of a record file: [{"pid", "start", "executable", "home"}]."""
    if not path:
        return []
    try:
        with open(path, "rb") as f:
            records = json.loads(f.read())
    except (OSError, ValueError) as e:
        raise Stop(f"the probe-owned process record {path} is unreadable: {e}")
    if not isinstance(records, list) or not all(
            isinstance(r, dict) and isinstance(r.get("pid"), int) and isinstance(r.get("start"), str)
            and isinstance(r.get("executable"), str) and isinstance(r.get("home"), str) for r in records):
        raise Stop(f"the probe-owned process record {path} is not a list of pid, start, executable and home")
    return records


def inspect_processes(home: str, owned=()) -> dict:
    """Recognise each Claude Code, Codex, Grove and Ravel process, and admit only the operator's.

    The program is admitted, and so are its exact ancestors up to the first
    harness (Claude Code or Codex) and the drivers (Ravel and Grove) above it.
    A second harness ancestor is another session: it and each tool ancestor
    above it count as other processes. A recognised process is also admitted
    when its PID, creation time, canonical executable and HOME equal a record
    of OWNED. Only inspections during probes pass OWNED; the inspection after
    probe cleanup and the one before restore pass none. Each other recognised
    process is "other", and each process that needs review is "review". The
    program reads the process table and signals nothing. Processes of other
    accounts are recognised by their executable only, because their
    arguments, folder and creation time need privilege.
    """
    scope = rehearsal_scope(home)
    procs = process_table()
    me, uid = os.getpid(), os.getuid()
    owned_ids = {(r["pid"], r["start"], r["executable"], r["home"]) for r in owned}
    found = {}
    for pid in sorted(procs):
        info = procs[pid]
        if info["zombie"] or pid == me:
            continue
        mine = info["uid"] == uid
        exe = executable_path(pid)
        launch, argv, env_home = process_arguments(pid) if mine else (None, None, None)
        if exe is None:
            # An exited process or a zombie fails both reads; neither writes anything.
            if not mine or process_status(pid) in (None, SZOMB):
                continue
        if scope and not (exe or launch or "").startswith(scope):
            continue
        folder = process_folder(pid) if mine else None

        def resolve(script, folder=folder):
            return os.path.realpath(os.path.join(folder, script) if folder else script)

        result = recognise(exe, launch, argv, resolve)
        if result is not None:
            found[pid] = dict(result, pid=pid, ppid=info["ppid"], executable=exe or launch,
                              start=process_start(pid) if mine else None, home=env_home,
                              args=" ".join(" ".join(argv or []).split())[:300])
    lineage = {me}
    harnesses = 0
    for pid in ancestors_of(procs, me):
        item = found.get(pid)
        if item is not None and item.get("tool") in HARNESSES:
            harnesses += 1
        if harnesses > 1:
            if item is not None:
                item["outer"] = True
            continue
        lineage.add(pid)
    report = {"admitted": [], "other": [], "review": []}
    for pid, item in sorted(found.items()):
        if pid in lineage:
            item["admitted_as"] = "ancestor"
            report["admitted"].append(item)
        elif "tool" in item and (pid, item["start"], item["executable"], item["home"]) in owned_ids:
            item["admitted_as"] = "probe-owned"
            report["admitted"].append(item)
        elif "tool" in item:
            report["other"].append(item)
        else:
            report["review"].append(item)
    return report


def describe(item: dict) -> str:
    what = f"{item['tool']} ({item['form']}{', ' + item['role'] if item.get('role') else ''})" if "tool" in item \
        else f"review: {item['review']}"
    outer = "outer harness ancestor, " if item.get("outer") else ""
    return f"process {item['pid']} (parent {item['ppid']}) {outer}{what}: {item['args'] or item['executable']}"


def process_gate(layout: Layout, point: str, owned=()) -> None:
    """Stop before POINT unless the narrow process inspection admits each tool process."""
    hook(layout, f"gate:{point}")
    report = inspect_processes(layout.home, owned)
    for item in report["admitted"]:
        log(f"  admitted {item['admitted_as']} {describe(item)}")
    for item in report["other"] + report["review"]:
        log(f"  remaining {describe(item)}")
    if report["other"] or report["review"]:
        raise Stop(f"another Claude Code, Codex, Grove or Ravel process, or a process that needs review, "
                   f"remains before {point}; end it through its owning interface, and the program signals none")
    log(f"gate: no other Claude Code, Codex, Grove or Ravel process before {point}")


def processes_command(args) -> int:
    """The `processes` entry: run the inspection, print it and change nothing."""
    home = os.path.realpath(args.home or account_home())
    report = inspect_processes(home, load_owned(args.owned))
    clear = not (report["other"] or report["review"])
    if args.json:
        print(json.dumps(dict(report, clear=clear), indent=2, sort_keys=True))
        return 0 if clear else 3
    for item in report["admitted"]:
        log(f"admitted {item['admitted_as']} {describe(item)}")
    for item in report["other"] + report["review"]:
        log(f"remaining {describe(item)}")
    log(f"result: {'clear' if clear else 'refused'}")
    return 0 if clear else 3


# --- commands -----------------------------------------------------------------

def subject_env(base: dict) -> dict:
    """An environment that selects no other subject than its HOME.

    It drops Grove's loop control variables, Claude Code's five location
    variables, CODEX_HOME and each RAVEL_* name. Each of them can point a
    harness or a resolution at another configuration than HOME's.
    """
    return {k: v for k, v in base.items()
            if k not in LOOP_CONTROL and k not in CLAUDE_LOCATION_VARS and k != CODEX_HOME
            and not k.startswith("RAVEL_")}


def command_env(layout: Layout, step: str, plan=None, admitted=None) -> dict:
    env = {k: v for k, v in subject_env(os.environ).items()
           if k not in (HOOK_VAR, PLAN_VAR, PLAN_DIGEST_VAR, ADMITTED_VAR)}
    env.update(HOME=layout.home, OWNER_SWITCH_HOME=layout.home, OWNER_SWITCH_STATE=layout.state,
               OWNER_SWITCH_STEP=step)
    if plan is not None:
        # A gate reads the plan by its path, and stops unless the bytes there
        # still have the digest that the program admitted.
        env[PLAN_VAR] = plan["_path"]
        env[PLAN_DIGEST_VAR] = plan["_digest"]
    if admitted:
        env[ADMITTED_VAR] = json.dumps(admitted, sort_keys=True)
    return env


def command_argv(argv: list) -> list:
    """Replace "$python" and "$program" with this program's interpreter and path."""
    return [{"$python": sys.executable, "$program": PROGRAM_PATH}.get(a, a) for a in argv]


def command_result(stdout: str):
    """The object of the last `owner-switch-result:` line, if any."""
    for line in reversed(stdout.splitlines()):
        if line.startswith(RESULT_MARK):
            try:
                value = json.loads(line[len(RESULT_MARK):])
            except ValueError:
                return None
            return value if isinstance(value, dict) else None
    return None


def run_command_full(layout: Layout, command: dict, step: str, plan=None, admitted=None):
    """Run one command of a plan, and return its evidence and its standard output."""
    hook(layout, f"command:{command['name']}")
    result = subprocess.run(command_argv(command["argv"]), cwd=layout.home,
                            env=command_env(layout, step, plan, admitted), capture_output=True, text=True)
    output = (result.stdout + result.stderr)[-20000:]
    log(f"command {command['name']} (step {step}): exit {result.returncode}")
    parsed = command_result(result.stdout)
    if result.returncode != 0 or parsed is not None:
        # A failed command's tail, and a gate's report without its result line.
        lines = [x for x in output.strip().splitlines() if not x.startswith(RESULT_MARK)]
        for line in lines[-40:]:
            log(f"  | {line}")
    evidence = {"name": command["name"], "argv": command["argv"], "exit": result.returncode, "output": output}
    if parsed is not None:
        evidence["result"] = parsed
    RUN_LOG["commands"].append(dict(evidence, step=step))
    return evidence, result.stdout


def run_command(layout: Layout, command: dict, step: str, plan=None, admitted=None) -> dict:
    return run_command_full(layout, command, step, plan, admitted)[0]


def run_commands(layout: Layout, plan: dict, commands: list, step: str, admitted=None):
    results = [run_command(layout, c, step, plan, admitted) for c in commands]
    failed = [r for r in results if r["exit"] != 0]
    return results, failed


def admitted_of(results: list) -> dict:
    """The values that step 4's results admit for the later gates."""
    admitted = {}
    for r in results:
        value = (r.get("result") or {}).get("admitted")
        if isinstance(value, dict):
            admitted.update(value)
    return admitted


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
    # The merge entry takes the difference from the bytes that the run read.
    write_exclusive(os.path.join(backup.stage, copy + READ_SUFFIX), content)
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
    process_gate(layout, "the inverse")
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
    _, failed = run_commands(layout, backup.plan, backup.plan["native_probe"], "11")
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
        elif body is None and set(os.listdir(folder)) - {"read"}:
            continue
        for leftover in (os.path.join(folder, "read"), record):
            if os.path.exists(leftover):
                os.unlink(leftover)
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
    """Change one cell by a settings record, the atomic exchange and the comparison."""
    def mutate(obj: dict) -> list:
        old = get_cell(obj, key)
        if not same(old, earlier):
            raise Stop(f"{rel} {key_name(key)} holds {show(old)}, not {show(earlier)}")
        set_cell(obj, key, new)
        return [(key_name(key), old, new)]
    record_exchange(layout, rel, mutate, label, {"key": key})


def record_exchange(layout: Layout, rel: str, mutate, label: str, extra=None) -> list:
    """Change a JSON file by a settings record, the atomic exchange and the comparison.

    `mutate` changes the object that one reading read, and returns the
    replaced values. The record holds the digest of what this reading read
    and of the stage file, and the file `read` holds the bytes that it read.
    It is complete under its own name before the exchange. After the
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
        replaced = mutate(obj)
        if not replaced:
            return []
        hook(layout, f"{label}-read")
        os.mkdir(tmp, 0o700)
        with open(os.path.join(tmp, "read"), "wb") as f:
            f.write(content)
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
    record = dict(extra or {}, kind="exchange", path=rel, label=label,
                  read=read_digest, stage=path_digest(stage), at=now())
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
    for key, old, new in replaced:
        log(f"replaced: {rel} {key}: {show(old)} -> {show(new)}")
    hook(layout, f"{label}-compared")
    return replaced


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
        process_gate(layout, "the native settings")
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
    process_gate(layout, "the native settings")
    prelude_exchange(layout, RAVEL_SETTINGS, ["skills"], ABSENT, OFF, "native")


# --- native-off ("The native settings inside finish") ------------------------

# The temporary sibling of the settings file. A rerun removes each one that an
# interruption left, because no reader of the settings reads these names.
NATIVE_OFF_TEMP = ".settings.json.native-off-"
# fcntl(2): F_FULLFSYNC asks the drive to flush its cache, which fsync(2) on
# macOS does not do.
F_FULLFSYNC = getattr(fcntl, "F_FULLFSYNC", 51)


class Incomplete(Exception):
    """A failed write of native-off: the off value is not established."""


def native_off_argv(home: str) -> list:
    """The exact command of native-off for HOME, with this interpreter and this program."""
    return [sys.executable, "-I", PROGRAM_PATH, "native-off", "--home", home]


def other_ravel_drivers(home: str) -> list:
    """Each Ravel process except this process and its exact ancestors.

    The narrow process inspection recognises the process. A launch form that
    names Ravel without a Ravel executable counts too, because it needs
    review. An argument that only contains the name is not recognition.
    """
    report = inspect_processes(home)
    return [describe(item) for item in report["other"] + report["review"]
            if item.get("tool") == "Ravel" or "Ravel" in item.get("names", [])]


def strict_object(content: bytes):
    """The JSON object of a settings file and None, or None and the problem.

    A repeated member name or a non-finite number makes the file malformed:
    a rewrite would lose a member or write a value that is not JSON.
    """
    def pairs(items):
        names = [name for name, _ in items]
        if len(set(names)) != len(names):
            raise ValueError("a member name is repeated")
        return dict(items)

    def number(text):
        value = float(text)
        if not math.isfinite(value):
            raise ValueError(f"the number {text} is not finite")
        return value

    def constant(name):
        raise ValueError(f"{name} is not JSON")

    try:
        obj = json.loads(content.decode("utf-8"), object_pairs_hook=pairs, parse_float=number,
                         parse_constant=constant)
    except UnicodeDecodeError:
        return None, "it is not UTF-8"
    except ValueError as e:
        return None, str(e)
    if not isinstance(obj, dict):
        return None, "it is not a JSON object"
    return obj, None


def file_metadata(fd: int) -> dict:
    """The metadata that native-off keeps: mode, owner, flags, ACL and extended attributes."""
    st = os.fstat(fd)
    return {"mode": oct(st.st_mode & 0o7777), "uid": st.st_uid, "gid": st.st_gid, "flags": st.st_flags,
            "acl": _acl_text(fd).decode(errors="replace"),
            "xattrs": {n.decode(errors="replace"): v.hex() for n, v in _xattrs(fd) if n != PROVENANCE}}


def full_sync(fd: int) -> str:
    """Flush a file or folder to the drive, and name the method that the filesystem took."""
    try:
        fcntl.fcntl(fd, F_FULLFSYNC)
        return "F_FULLFSYNC"
    except OSError as e:
        if e.errno not in (errno.ENOTSUP, errno.EINVAL, errno.ENOTTY):
            raise
    os.fsync(fd)
    return "fsync, because the filesystem refused F_FULLFSYNC"


def write_all(fd: int, data: bytes) -> None:
    while data:
        data = data[os.write(fd, data):]


def remove_stale(folder: str) -> None:
    """Remove each temporary sibling that an interrupted native-off left."""
    try:
        names = sorted(n for n in os.listdir(folder) if n.startswith(NATIVE_OFF_TEMP))
    except FileNotFoundError:
        return
    for name in names:
        try:
            os.unlink(os.path.join(folder, name))
            log(f"native-off: removed the interrupted temporary file {name}")
        except OSError as e:
            log(f"native-off: the interrupted temporary file {name} remains: {e.strerror}")


def read_settings(path: str):
    """The bytes and metadata of the settings file, or None when it is absent."""
    try:
        # O_NONBLOCK: a FIFO at the path must not block the open.
        fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
    except FileNotFoundError:
        return None
    except OSError as e:
        if e.errno == errno.ELOOP:
            raise Stop(f"{RAVEL_SETTINGS} is a link; review it, and replace nothing")
        raise Stop(f"{RAVEL_SETTINGS} could not be read: {e.strerror}")
    try:
        if not stat.S_ISREG(os.fstat(fd).st_mode):
            raise Stop(f"{RAVEL_SETTINGS} is not a regular file; review it, and replace nothing")
        return read_fd(fd), file_metadata(fd)
    finally:
        os.close(fd)


def native_off(home: str) -> None:
    """Give the Ravel settings the off value as `skills`, and change nothing else.

    "The native settings inside finish" owns the rules. The new bytes go to a
    temporary sibling, which receives the original's metadata and is flushed
    to the drive and checked. One rename then replaces the live file, which
    is never truncated. An interruption leaves the original or the complete
    new file, and the same command completes either.
    """
    hook_at(home, "native-off:begin")
    path = os.path.join(home, RAVEL_SETTINGS)
    folder = os.path.dirname(path)
    real = os.path.realpath(folder)
    if real != home and not real.startswith(home + os.sep):
        raise Stop(f"{os.path.dirname(RAVEL_SETTINGS)} leads outside HOME through {real}")
    found = read_settings(path)
    if found is None:
        obj, original = None, None
        new = {"skills": OFF}
    else:
        content, meta = found
        obj, problem = strict_object(content)
        if obj is None:
            raise Stop(f"{RAVEL_SETTINGS} is malformed: {problem}; review it, and replace nothing")
        if same(obj.get("skills", ABSENT), OFF):
            log(f"native-off: {RAVEL_SETTINGS} holds the off value; nothing changed")
            remove_stale(folder)
            return
        if "skills" in obj:
            raise Stop(f"{RAVEL_SETTINGS} holds skills {show(obj['skills'])}, not the off value; "
                       "review it, and replace nothing")
        if meta["uid"] != os.geteuid():
            raise Stop(f"{RAVEL_SETTINGS} belongs to user {meta['uid']}, so a new file cannot keep its owner")
        original = (content, meta)
        new = dict(obj, skills=OFF)
    drivers = other_ravel_drivers(home)
    if drivers:
        for line in drivers:
            log(f"  remaining {line}")
        raise Stop("another Ravel driver runs; end it, pause each writer of the settings, "
                   "and signal no session")
    hook_at(home, "native-off:read")
    remove_stale(folder)
    try:
        os.makedirs(folder, 0o755, exist_ok=True)
    except OSError as e:
        raise Incomplete(f"{os.path.dirname(RAVEL_SETTINGS)} could not be created: {e.strerror}")
    data = dump_json(new)
    syncs = publish_settings(home, path, data, obj, original)
    kept = f"{len(obj)} other keys kept" if obj is not None else "the file was absent"
    log(f"native-off: wrote the off value to {RAVEL_SETTINGS} ({kept})")
    log(f"persistence: temporary file {syncs[0]}; folder {syncs[1]}")


def publish_settings(home: str, path: str, data: bytes, obj, original) -> list:
    """Publish DATA at PATH through a checked, flushed temporary sibling and one rename."""
    folder = os.path.dirname(path)
    tmp = os.path.join(folder, NATIVE_OFF_TEMP + secrets.token_hex(6))
    try:
        # Read and write: check_publication reads the flushed bytes back.
        tfd = os.open(tmp, os.O_RDWR | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600)
    except OSError as e:
        raise Incomplete(f"the temporary file {os.path.basename(tmp)} could not be created: {e.strerror}")
    syncs, published = [], False
    try:
        half = len(data) // 2
        write_all(tfd, data[:half])
        hook_at(home, "native-off:temp-partial")
        write_all(tfd, data[half:])
        if original is None:
            os.fchmod(tfd, 0o644)
        else:
            lfd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
            try:
                if read_fd(lfd) != original[0]:
                    raise Stop(f"{RAVEL_SETTINGS} changed while native-off ran; pause its writers, "
                               "and run the command again")
                copy_metadata(lfd, tfd)
            finally:
                os.close(lfd)
            # fcopyfile copies the modification time too, and the content is new.
            os.utime(tfd, None)
        syncs.append(full_sync(tfd))
        check_publication(tfd, data, obj, original)
        hook_at(home, "native-off:temp-synced")
        if original is None:
            try:
                renamex(tmp, path, RENAME_EXCL)
            except OSError as e:
                if e.errno == errno.EEXIST:
                    raise Stop(f"{RAVEL_SETTINGS} appeared while native-off ran; pause its writers, "
                               "and run the command again")
                raise
        else:
            current = read_settings(path)
            if current is None or current[0] != original[0]:
                raise Stop(f"{RAVEL_SETTINGS} changed while native-off ran; pause its writers, "
                           "and run the command again")
            os.rename(tmp, path)
        published = True
    except OSError as e:
        raise Incomplete(f"the publication of {RAVEL_SETTINGS} failed: {e.strerror}")
    finally:
        os.close(tfd)
        if not published:
            try:
                os.unlink(tmp)
            except OSError as e:
                log(f"native-off: the temporary file {os.path.basename(tmp)} remains: {e.strerror}; "
                    "a rerun removes it")
    hook_at(home, "native-off:replaced")
    try:
        dfd = os.open(folder, os.O_RDONLY)
        try:
            syncs.append(full_sync(dfd))
        finally:
            os.close(dfd)
        hook_at(home, "native-off:published")
        current = read_settings(path)
    except OSError as e:
        raise Incomplete(f"the folder of {RAVEL_SETTINGS} could not be flushed or read: {e.strerror}")
    if current is None or current[0] != data:
        raise Incomplete(f"{RAVEL_SETTINGS} does not hold the published bytes after the rename")
    return syncs


def check_publication(fd: int, data: bytes, obj, original) -> None:
    """Check the flushed temporary file before it replaces the live file."""
    content = read_fd(fd)
    new, problem = strict_object(content)
    if content != data or new is None:
        raise Incomplete(f"the temporary file does not hold the prepared bytes: {problem or 'they differ'}")
    if not same(new.get("skills", ABSENT), OFF):
        raise Incomplete("the temporary file lacks the off value")
    if obj is not None:
        if list(new) != list(obj) + ["skills"]:
            raise Incomplete("the temporary file does not keep the original members in order")
        changed = [k for k in obj if not same(obj[k], new[k])]
        if changed:
            raise Incomplete(f"the temporary file changed the members {', '.join(changed)}")
    meta = file_metadata(fd)
    want = original[1] if original is not None else None
    if want is None:
        if meta["mode"] != oct(0o644):
            raise Incomplete(f"the temporary file has mode {meta['mode']}, not 0o644")
    elif meta != want:
        differ = sorted(k for k in want if want[k] != meta[k])
        raise Incomplete(f"the temporary file does not keep the original {', '.join(differ)}")


def native_off_command(args) -> int:
    home = os.path.realpath(args.home)
    argv = native_off_argv(home)
    rerun = shlex.join(argv)
    if args.print_command:
        print(json.dumps({"argv": argv, "command": rerun, "executable": sys.executable,
                          "program": PROGRAM_PATH, "home": home,
                          "settings": os.path.join(home, RAVEL_SETTINGS)}, indent=2))
        return 0
    try:
        native_off(home)
    except Stop as e:
        log(f"stop: {e}")
        log(f"native delivery is incomplete unless {RAVEL_SETTINGS} holds the off value: start no "
            f"new default launch. Resolve the stop, then run: {rerun}")
        return 3
    except Incomplete as e:
        log(f"native-off failed: {e}")
        log(f"native delivery is incomplete: start no new default launch. Run the same command again: {rerun}")
        return 4
    log("result: off value in place")
    return 0


def claude_env(layout: Layout) -> dict:
    """The environment of `claude` and `brew`: command_env drops the location variables."""
    return command_env(layout, "3")


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
    with open(os.path.join(tmp, "read"), "wb") as f:
        f.write(reading["content"])
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
    process_gate(layout, "the hooks installation")
    if state["kind"] == "absent":
        check_claude_version(layout)
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


# --- the gates of steps 4 to 7 ---------------------------------------------------

def emit(result: dict) -> None:
    print(RESULT_MARK + json.dumps(result, sort_keys=True), flush=True)


def tool(argv: list, env=None, cwd=None):
    try:
        return subprocess.run(argv, cwd=cwd, env=env, capture_output=True, text=True)
    except OSError as e:
        raise Stop(f"{argv[0]} cannot run: {e.strerror}")


def tool_line(argv: list) -> str:
    result = tool(argv)
    line = result.stdout.strip()
    if result.returncode != 0 or not line:
        raise Stop(f"{' '.join(argv)} failed: {(result.stdout + result.stderr).strip()[-2000:]}")
    return line


def tool_json(argv: list, env=None, cwd=None):
    result = tool(argv, env, cwd)
    if result.returncode != 0:
        raise Stop(f"{' '.join(argv[:4])} failed with exit {result.returncode}: "
                   f"{(result.stdout + result.stderr).strip()[-2000:]}")
    try:
        return json.loads(result.stdout)
    except ValueError:
        raise Stop(f"{' '.join(argv[:4])} printed no JSON")


class Gate:
    """What a gate reads: HOME, the plan that names it and step 4's admitted values."""

    def __init__(self, args):
        home = args.home or os.environ.get("OWNER_SWITCH_HOME")
        plan_path = args.plan or os.environ.get(PLAN_VAR)
        if not home or not plan_path:
            raise Stop(f"a gate needs --home and --plan, or the program's OWNER_SWITCH_HOME and {PLAN_VAR}")
        self.home = os.path.realpath(home)
        self.plan = load_plan(plan_path)
        if not args.plan:
            # The program launched this gate: the plan must be the bytes that it admitted.
            expected = os.environ.get(PLAN_DIGEST_VAR)
            if not expected:
                raise Stop(f"a gate that reads {PLAN_VAR} needs {PLAN_DIGEST_VAR}")
            if self.plan["_digest"] != expected:
                raise Stop(f"the plan at {plan_path} is not the plan that the program admitted: "
                           f"its digest is {self.plan['_digest']}, not {expected}")
        accepted = self.plan["accepted"]
        self.sources = accepted.get("sources")
        if not isinstance(self.sources, dict) or sorted(self.sources) != sorted(SOURCE_IDS):
            raise Stop(f"the plan's accepted sources must be exactly {', '.join(SOURCE_IDS)}")
        for sid, source in self.sources.items():
            if (not isinstance(source, dict) or not isinstance(source.get("formula"), str)
                    or not re.fullmatch(r"[0-9a-f]{64}", str(source.get("resourceDigest")))):
                raise Stop(f"the plan's accepted source {sid} needs a formula and a resourceDigest")
        self.versions = accepted.get("versions") or {}
        self.trials = accepted.get("trials") or {}
        self.staging = staging_of(self.plan)
        try:
            self.admitted = json.loads(os.environ.get(ADMITTED_VAR) or "{}")
        except ValueError:
            raise Stop(f"{ADMITTED_VAR} is not JSON")

    def accepted_digest(self, sid: str) -> str:
        return (self.admitted.get("digests") or {}).get(sid) or self.sources[sid]["resourceDigest"]


def staging_of(plan: dict) -> dict:
    """The retained staging HOME and candidate keg that the plan's accepted values name."""
    staging = plan["accepted"].get("staging")
    if (not isinstance(staging, dict)
            or not all(isinstance(staging.get(k), str) and os.path.isabs(staging[k])
                       and os.path.normpath(staging[k]) == staging[k] for k in ("home", "candidate_keg"))):
        raise Stop("the plan's accepted staging must name the absolute paths \"home\" and \"candidate_keg\"")
    return staging


def staging_env(gate: "Gate") -> dict:
    """Step 4: the environment of a staging trial, in the retained staging HOME.

    "Settings isolation and the staging HOME": each staging launch sets HOME
    to the staging HOME and CODEX_HOME to its own .codex tree, and inherits
    none of Claude Code's five location variables.
    """
    home = gate.staging["home"]
    codex = os.path.join(home, ".codex")
    if os.path.realpath(home) == gate.home:
        raise Stop(f"step 4: the staging HOME {home} is the switch's HOME")
    if not os.path.isdir(home) or os.path.islink(codex) or not os.path.isdir(codex):
        raise Stop(f"step 4: the retained staging HOME {home} with its own .codex tree is missing")
    if not os.path.isdir(gate.staging["candidate_keg"]):
        raise Stop(f"step 4: the retained candidate keg {gate.staging['candidate_keg']} is missing")
    env = subject_env(os.environ)
    env.update(HOME=home, CODEX_HOME=codex)
    return env


def drafted_settings(plan: dict) -> dict:
    """The drafted real Ravel settings: the switched cells of the plan's Ravel settings part."""
    parts = [p for p in plan["parts"] if p["kind"] == "file" and p["path"] == RAVEL_SETTINGS]
    if len(parts) != 1:
        raise Stop(f"the plan needs one part for {RAVEL_SETTINGS}")
    settings = {}
    for cell in parts[0]["cells"]:
        set_cell(settings, cell["key"], cell["switched"])
    skills = settings.get("skills")
    if not isinstance(skills, dict) or "sources" in skills:
        raise Stop("the drafted settings must hold a skills value that names no sources, "
                   "so that Ravel reads the default skill directory")
    return settings


def erase_tree(path: str) -> None:
    """Erase a disposable HOME. Ravel publishes each snapshot read-only."""
    for folder, _, _ in os.walk(path):
        os.chmod(folder, 0o700)
    shutil.rmtree(path, ignore_errors=True)


def resolve(prefix: str, settings: dict) -> dict:
    """`ravel skills resolve --json` with the promoted ravel, in a disposable HOME."""
    ravel = os.path.join(prefix, "bin", "ravel")
    home = tempfile.mkdtemp(prefix="owner-switch-resolve-")
    try:
        os.makedirs(os.path.join(home, os.path.dirname(RAVEL_SETTINGS)))
        with open(os.path.join(home, RAVEL_SETTINGS), "wb") as f:
            f.write(dump_json(settings))
        # Ravel reads its settings and its cache from HOME, and no RAVEL_*
        # variable may steer the resolution.
        env = {k: v for k, v in os.environ.items() if not k.startswith("RAVEL_")}
        env["HOME"] = home
        version = tool([ravel, "--version"], env, home).stdout.strip()
        doc = tool_json([ravel, "skills", "resolve", "--json"], env, home)
    finally:
        erase_tree(home)
    if not isinstance(doc, dict) or doc.get("status") != "resolved":
        raise Stop(f"{ravel} skills resolve --json did not resolve: {canon(doc)[:2000]}")
    sources = {}
    for source in doc.get("sources") or []:
        if not isinstance(source, dict) or not all(isinstance(source.get(k), str)
                                                   for k in ("id", "resourceDigest", "originPath")):
            raise Stop(f"{ravel} skills resolve --json gave a source without id, resourceDigest or originPath")
        sources[source["id"]] = source
    log(f"resolved with {ravel} ({version}): {', '.join(sorted(sources))}")
    return {"ravel": ravel, "version": version, "sources": sources}


def installed_grove() -> dict:
    """Step 4: the installed Grove is the tap's current version, and opt/grove reaches its keg."""
    prefix = tool_line(["brew", "--prefix"])
    cellar = tool_line(["brew", "--cellar"])
    info = tool_json(["brew", "info", "--json=v2", FORMULA_OF_GROVE])
    try:
        formula = info["formulae"][0]
        stable = formula["versions"]["stable"]
        revision = formula.get("revision") or 0
        installed = [i["version"] for i in formula.get("installed") or []]
        linked = formula.get("linked_keg")
    except (KeyError, IndexError, TypeError):
        raise Stop(f"brew info --json=v2 {FORMULA_OF_GROVE} has no formula with a stable version")
    version = f"{stable}_{revision}" if revision else stable
    keg = os.path.join(cellar, "grove", version)
    opt = os.path.join(prefix, "opt", "grove")
    problems = []
    if version not in installed:
        problems.append(f"the tap's current version {version} is not installed (installed: "
                        f"{', '.join(installed) or 'none'})")
    if linked != version:
        problems.append(f"the linked keg is {linked}, not the tap's current version {version}")
    if not os.path.isdir(keg) or os.path.realpath(opt) != os.path.realpath(keg):
        problems.append(f"{opt} reaches {os.path.realpath(opt)}, not the keg {keg}")
    else:
        line = tool([os.path.join(opt, "bin", "grove"), "--version"]).stdout.strip()
        if line != f"grove {stable}":
            problems.append(f"{opt}/bin/grove --version printed {line!r}, not 'grove {stable}'")
    if problems:
        raise Stop("step 4: the installed Grove is not the latest published release: " + "; ".join(problems))
    log(f"step 4: Grove {version} is the tap's current version, and {opt} reaches its keg")
    return {"version": version, "keg": os.path.realpath(keg), "opt": opt, "prefix": prefix}


def gate_4(gate: Gate) -> None:
    grove = installed_grove()
    resolution = resolve(grove["prefix"], drafted_settings(gate.plan))
    own = sorted(sid for sid, s in gate.sources.items() if s["formula"] == FORMULA_OF_GROVE)
    if not own:
        raise Stop(f"the plan accepts no source of {FORMULA_OF_GROVE}")
    report, rerun = {}, []
    for sid in own:
        got = resolution["sources"].get(sid)
        if got is None:
            raise Stop(f"step 4: Grove's source {sid} does not resolve")
        keg_source = os.path.join(grove["keg"], "share", "ravel", "skills", sid)
        if os.path.realpath(got["originPath"]) != os.path.realpath(keg_source):
            raise Stop(f"step 4: the source {sid} comes from {got['originPath']}, not from {keg_source}")
        accepted = gate.sources[sid]["resourceDigest"]
        report[sid] = {"accepted": accepted, "installed": got["resourceDigest"], "originPath": got["originPath"]}
        if got["resourceDigest"] == accepted:
            log(f"step 4: {sid} gives its accepted digest {accepted}")
        else:
            rerun.append(sid)
            log(f"step 4: the staging trials of {sid} must run again: accepted {accepted}, "
                f"installed {got['resourceDigest']}")
    missing = [sid for sid in rerun if not gate.trials.get(sid)]
    if missing:
        raise Stop(f"step 4: the plan names no staging trial for {', '.join(missing)}")
    trials, digests = [], {}
    # The trials run in the retained staging HOME, outside every Grove
    # workspace and without Grove's loop control variables: the environment
    # rule of step 10 and the isolation rules of the staging HOME.
    base = staging_env(gate) if rerun else {}
    for sid in rerun:
        for command in gate.trials[sid]:
            env = dict(base, OWNER_SWITCH_SOURCE=sid, OWNER_SWITCH_DIGEST=report[sid]["installed"])
            result = tool(command_argv(command["argv"]), env, gate.staging["home"])
            trials.append({"source": sid, "name": command["name"], "argv": command["argv"],
                           "exit": result.returncode, "output": (result.stdout + result.stderr)[-4000:]})
            log(f"step 4: trial {command['name']} of {sid}: exit {result.returncode}")
            if result.returncode != 0:
                raise Stop(f"step 4: the staging trial {command['name']} of {sid} failed")
        digests[sid] = report[sid]["installed"]
    emit({"step": "4", "grove": grove, "ravel": resolution["version"], "ravel_path": resolution["ravel"],
          "sources": report, "reran": rerun,
          "trials": trials, "admitted": {"grove_version": grove["version"], "digests": digests}})


def gate_5(gate: Gate) -> None:
    grove_version = gate.admitted.get("grove_version")
    if not grove_version:
        raise Stop("step 5 needs the Grove version that step 4 admitted")
    formulae = sorted({s["formula"] for s in gate.sources.values()})
    stray = sorted(set(gate.versions) - set(formulae))
    unknown = [f for f in formulae if f != FORMULA_OF_GROVE and not gate.versions.get(f)]
    if stray or FORMULA_OF_GROVE in gate.versions or unknown:
        raise Stop("the plan's accepted versions must name each skill formula except Grove's, and no other")
    doc = tool_json(["mise", "-C", gate.home, "bootstrap", "packages", "status", "--json"], cwd=gate.home)
    rows = {}
    if isinstance(doc, dict) and isinstance(doc.get("brew"), dict):
        rows = {r.get("package"): r for r in doc["brew"].get("packages") or [] if isinstance(r, dict)}
    problems, installed = [], {}
    for name in formulae:
        wanted = grove_version if name == FORMULA_OF_GROVE else gate.versions[name]
        row = rows.get(name)
        if row is None or row.get("desired_state") != "present":
            problems.append(f"{name} is not declared")
        elif row.get("state") != "installed":
            problems.append(f"{name} is {row.get('state')}, not installed")
        elif row.get("installed_version") != wanted:
            problems.append(f"{name} is at {row.get('installed_version')}, not its accepted version {wanted}")
        else:
            installed[name] = wanted
            log(f"step 5: {name} is installed at its accepted version {wanted}")
    if problems:
        raise Stop("step 5: " + "; ".join(problems))
    emit({"step": "5", "formulae": installed})


def gate_6(gate: Gate) -> None:
    resolution = resolve(tool_line(["brew", "--prefix"]), drafted_settings(gate.plan))
    got, wanted = set(resolution["sources"]), set(gate.sources)
    if got != wanted:
        raise Stop(f"step 6: the resolved sources are not the accepted ten: extra "
                   f"{', '.join(sorted(got - wanted)) or 'none'}, missing {', '.join(sorted(wanted - got)) or 'none'}")
    problems, report = [], {}
    for sid in sorted(got):
        source = resolution["sources"][sid]
        accepted = gate.accepted_digest(sid)
        report[sid] = {k: source.get(k) for k in ("resourceDigest", "originPath", "snapshotRoot")}
        if source["resourceDigest"] != accepted:
            problems.append(f"{sid} gives {source['resourceDigest']}, not its accepted {accepted}")
    if problems:
        raise Stop("step 6: " + "; ".join(problems))
    log("step 6: the ten sources resolve with their accepted digests")
    emit({"step": "6", "ravel": resolution["version"], "ravel_path": resolution["ravel"], "sources": report})


def gate_7(gate: Gate) -> None:
    settings = load_json_file(os.path.join(gate.home, CLAUDE_SETTINGS))
    registry = load_json_file(os.path.join(gate.home, ".claude/plugins/installed_plugins.json"))
    if settings is None or registry is None or not isinstance(registry.get("plugins"), dict):
        raise Stop("step 7 cannot read settings.json and installed_plugins.json")
    entries = [(pid, e) for pid, es in sorted(registry["plugins"].items()) if isinstance(es, list)
               for e in es if isinstance(e, dict) and isinstance(e.get("installPath"), str)]
    enabled = settings.get("enabledPlugins") if isinstance(settings.get("enabledPlugins"), dict) else {}
    readers, problems = [], []
    for pid in sorted(p for p, v in enabled.items() if v is True):
        mine = [e for p, e in entries if p == pid]
        if not mine:
            readers.append({"plugin": pid, "registration": None, "reason": "enabled"})
        readers += [{"plugin": pid, "registration": e, "path": e["installPath"], "reason": "enabled"} for e in mine]
    for named in [p for p in os.environ.get(READERS_VAR, "").split(os.pathsep) if p]:
        real = os.path.realpath(named)
        mine = [(p, e) for p, e in entries if real == os.path.realpath(e["installPath"])
                or real.startswith(os.path.realpath(e["installPath"]) + os.sep)]
        if not mine:
            problems.append(f"{named}, which finish read, has no registration in installed_plugins.json")
            continue
        readers.append({"plugin": mine[0][0], "registration": mine[0][1], "path": mine[0][1]["installPath"],
                        "reason": "read", "read": named})
    for reader in readers:
        path = reader.get("path")
        if path is None:
            continue
        reader["orphaned"] = os.path.lexists(os.path.join(path, ".orphaned_at"))
        if not os.path.isdir(path):
            problems.append(f"the version directory {path} of {reader['plugin']} is missing")
        elif reader["orphaned"]:
            problems.append(f"the version directory {path} of {reader['plugin']} holds .orphaned_at")
        else:
            log(f"step 7: {reader['plugin']} reads {path}")
    if problems:
        raise Stop("step 7: " + "; ".join(problems))
    emit({"step": "7", "readers": readers})


def gate_command(args) -> int:
    gate = Gate(args)
    {"4": gate_4, "5": gate_5, "6": gate_6, "7": gate_7}[args.step](gate)
    return 0


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
                and rec.get("program_digest") == PROGRAM_DIGEST and measured_version(rec)):
            out.append(rec)
    return out


def measured_version(rec: dict):
    """The Claude Code version whose writes a record's rerun of settings-writes.sh measured.

    "Test seams": the rehearsal reruns settings-writes.sh with the version
    that it uses and names that version. A record without a passing rerun
    certifies no write method.
    """
    rerun = rec.get("settings_writes")
    if not isinstance(rerun, dict) or rerun.get("passed") is not True:
        return None
    version = rerun.get("claude_version")
    return version if isinstance(version, str) and version else None


def check_claude_version(layout: Layout) -> None:
    """Stop unless `claude --version` names a version that a passing record measured."""
    result = run_tool(layout, ["claude", "--version"])
    version = result.stdout.strip()
    measured = sorted({measured_version(rec) for rec in passing_rehearsals(layout)})
    if result.returncode != 0 or version not in measured:
        raise Stop(f"claude --version printed {version!r}, and no passing rehearsal record measured the "
                   f"writes of that version (measured: {', '.join(measured) or 'none'})")
    log(f"hooks installation: Claude Code {version} is the version that the rehearsal measured")


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


def recheck_readers(layout: Layout, backup: Backup) -> dict:
    """Inside the probes: read each version directory of step 7's evidence again.

    "Resource lifetime" keeps each version that a live reader uses
    registered and current until purge. A directory that is missing, holds
    .orphaned_at or lost its registration counts as a failed probe.
    """
    registry = load_json_file(os.path.join(layout.home, ".claude/plugins/installed_plugins.json")) or {}
    plugins = registry.get("plugins") if isinstance(registry.get("plugins"), dict) else {}
    readers, problems = [], []
    for result in gate_results(backup.evidence("7")):
        for reader in (result or {}).get("readers") or []:
            path = reader.get("path")
            if not isinstance(path, str):
                continue
            registered = any(isinstance(e, dict) and e.get("installPath") == path
                             for e in plugins.get(reader.get("plugin")) or [])
            orphaned = os.path.lexists(os.path.join(path, ".orphaned_at"))
            present = os.path.isdir(path)
            readers.append({"plugin": reader.get("plugin"), "path": path, "present": present,
                            "orphaned": orphaned, "registered": registered})
            if not present:
                problems.append(f"the version directory {path} of {reader.get('plugin')} is missing")
            elif orphaned:
                problems.append(f"the version directory {path} of {reader.get('plugin')} holds .orphaned_at")
            elif not registered:
                problems.append(f"the version directory {path} of {reader.get('plugin')} lost its registration")
    return {"readers": readers, "problems": problems}


def retained_items(layout: Layout, backup: Backup) -> list:
    """Each retained byte set with its purge condition, for the receipt ("Purge")."""
    plan = backup.plan
    purge = list(PURGE_CONDITIONS)
    items = []
    for part in plan["parts"]:
        if part["kind"] == "entry":
            items.append({"item": "retained entry", "path": backup.retained_path(part["path"]),
                          "original": part["path"], "identity": part["identity"], "condition": purge})
    for r in kept_copy_records(layout):
        items.append({"item": "kept copy", "path": r["_copy"], "of": r["path"], "condition": purge})
    items.append({"item": "Grove's Codex snapshot store", "path": layout.path(CODEX_SNAPSHOT_STORE),
                  "condition": purge})
    superseded = sorted({c["key"][1] for p in plan["parts"] if p["kind"] == "file" and p["path"] == CLAUDE_SETTINGS
                         for c in p["cells"] if len(c["key"]) == 2 and c["key"][0] == "enabledPlugins"
                         and c["switched"] is False and c["earlier"] is True})
    for pid in superseded:
        items.append({"item": "superseded plugin registration", "plugin": pid, "condition": purge})
    for market in sorted({pid.split("@", 1)[1] for pid in superseded if "@" in pid}):
        items.append({"item": "earlier marketplace registration", "marketplace": market, "condition": purge})
    staging = staging_of(plan)
    after = ["this receipt is written"]
    items.append({"item": "staging HOME", "path": staging["home"], "condition": after})
    items.append({"item": "candidate keg", "path": staging["candidate_keg"], "condition": after})
    return items


def receipt_body(layout: Layout, backup: Backup, gates: dict, probes: list, mac_check: dict, cells: list,
                 readers: dict) -> dict:
    """The receipt: the last paragraph of "The switch inside finish"."""
    plan = backup.plan

    def result(step: str) -> dict:
        found = [r for r in gate_results(backup.evidence(step)) if isinstance(r, dict)]
        return found[0] if found else {}

    step4, step5 = result("4"), result("5")
    step6 = next((r for r in gates.get("6 again", []) if isinstance(r, dict)), None) or result("6")
    formula_versions = step5.get("formulae") or {}
    sources = {}
    for sid, source in sorted(plan["accepted"]["sources"].items()):
        resolved = (step6.get("sources") or {}).get(sid) or {}
        sources[sid] = {"formula": source["formula"], "version": formula_versions.get(source["formula"]),
                        "keg_path": resolved.get("originPath"), "resourceDigest": resolved.get("resourceDigest"),
                        "snapshotRoot": resolved.get("snapshotRoot")}
    return {
        "activated": {
            "grove": step4.get("grove"),
            "ravel": {"path": step6.get("ravel_path"), "version": step6.get("ravel")},
            "config_rhai_digest": path_digest(layout.path(RAVEL_CONFIG)),
            "ravel_settings_digest": path_digest(layout.path(RAVEL_SETTINGS)),
            "claude_settings_digest": path_digest(layout.path(CLAUDE_SETTINGS)),
        },
        "sources": sources,
        "formula_versions": formula_versions,
        "trials": {"reran": step4.get("reran") or [], "results": step4.get("trials") or []},
        "plan_digest": plan["_digest"], "program_digest": backup.program_digest,
        "mac_check": mac_check, "probes": probes, "cells": cells, "gates": gates,
        "readers": {"step 7": result("7").get("readers") or [], "again": readers["readers"]},
        "backup": backup.path,
        "retained": retained_items(layout, backup),
    }


def cell_values(layout: Layout, plan: dict, backup: Backup) -> list:
    return [part_status(layout, p, backup) for p in plan["parts"]]


def all_switched(statuses: list) -> bool:
    return all(s["status"] == "switched" for s in statuses)


def apply_switch(layout: Layout, backup: Backup) -> None:
    """Step 9: the parts in their order, in one write interval."""
    plan = backup.plan
    process_gate(layout, "step 9")
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


def gate_results(results: list) -> list:
    return [r.get("result") for r in results]


def continue_after_switch(layout: Layout, backup: Backup, resumed: bool, baseline=()) -> None:
    """The active continuation: step 6 again, then the probes and steps 12 to 14.

    A fresh run has just passed step 6, so it does not run it again.
    """
    plan = backup.plan
    gates = {step: gate_results(backup.evidence(step)) for step in ("4", "5", "6", "7")}
    if resumed:
        changed = outside_changes(layout, plan) + list(baseline)
        if changed:
            failed_probe(layout, backup, "changed outside the switch set: " + ", ".join(changed))
        results, failed = run_commands(layout, plan, plan["gates"]["6"], "6", admitted_of(backup.evidence("4")))
        if failed:
            failed_probe(layout, backup, "step 6 failed: " + ", ".join(f["name"] for f in failed))
        gates["6 again"] = gate_results(results)
    before = cell_values(layout, plan, backup)
    probes, failed = run_commands(layout, plan, plan["probes"], "10")
    if failed:
        failed_probe(layout, backup, "probe failed: " + ", ".join(f["name"] for f in failed))
    hook(layout, "readers-again")
    readers = recheck_readers(layout, backup)
    if readers["problems"]:
        failed_probe(layout, backup, "the reader evidence of step 7 changed: " + "; ".join(readers["problems"]))
    mac_check = run_command(layout, plan["mac_check"], "12", plan)
    after = cell_values(layout, plan, backup)
    if not all_switched(after) or canon(before) != canon(after):
        failed_probe(layout, backup, "a cell changed while the probes ran")
    if report_flagged(layout):
        raise Stop("a kept copy is flagged at step 13")
    backup.add_record(layout, "receipt", receipt_body(layout, backup, gates, probes, mac_check, after, readers))
    step_14(layout, backup, baseline)


def manual_has_receipt(layout: Layout, backup: Backup) -> bool:
    try:
        with open(layout.manual, encoding="utf-8") as f:
            return f"<!-- owner-switch-receipt {backup.id} -->" in f.read()
    except FileNotFoundError:
        return False


def step_14(layout: Layout, backup: Backup, baseline=()) -> None:
    """Compare the entries outside the switch set, then copy the receipt."""
    hook(layout, "step-14")
    changed = outside_changes(layout, backup.plan) + list(baseline)
    if changed:
        failed_probe(layout, backup, "changed outside the switch set: " + ", ".join(changed))
    receipt = backup.record_body("receipt")
    # The manual stays append-only, so the receipt names each pending-activation record that it replaces.
    pending = pending_records(layout)
    replaces = ("It replaces the pending-activation records " + ", ".join(pending) + ".\n\n") if pending else ""
    text = (f"\n<!-- owner-switch-receipt {backup.id} -->\n"
            f"## Owner skill switch receipt {backup.id}\n\n"
            "This receipt describes the layout only while `mac-check` reports the switch complete.\n\n"
            + replaces +
            "Before purge, run the program copy of this backup with `purge-check`.\n\n"
            "```json\n" + json.dumps(receipt, indent=2, sort_keys=True) + "\n```\n")
    append_manual(layout, text, "receipt")
    log(f"receipt: copied into {layout.manual}")


def reexec_copy(backup: Backup, args, command=("run",)) -> None:
    """A continuation of an open backup runs the program copy in that backup."""
    if backup.program_digest == PROGRAM_DIGEST:
        return
    if command[0] != "state":
        log(f"continuation: running the program copy of {backup.id}")
    argv = [sys.executable, "-I", backup.program, *command, "--plan", os.path.abspath(args.plan),
            "--home", os.path.abspath(args.home)]
    sys.stdout.flush()
    os.execv(sys.executable, argv)


def baseline_rows(layout: Layout, plan: dict) -> list:
    """Step 3: rerun the baseline instrument, and return the rows that count as changed entries."""
    command = plan["baseline"]
    evidence, stdout = run_command_full(layout, command, "3", plan)
    if evidence["exit"] != 0:
        raise Stop(f"the baseline instrument {command['name']} failed: {evidence['output'][-2000:]}")
    rows = []
    for line in stdout.splitlines():
        fields = line.split("\t")
        if any(f in ANOMALIES for f in fields):
            rows.append("baseline row: " + " | ".join(fields))
        elif UNVERIFIED in fields:
            log("baseline: compare with finish's own skill listing: " + " | ".join(fields))
    for row in rows:
        log(f"  {row}")
    return rows


def run_switch(args) -> int:
    prepared = load_plan(args.plan)
    layout = Layout(args.home, prepared)
    RUN_LOG.update(layout=layout, command="run", plan=prepared)
    log(f"program: {PROGRAM_DIGEST}")
    log(f"plan: {prepared['_digest']}")
    hook(layout, "start")
    process_gate(layout, "step 1")
    step_3_preludes(layout, prepared)
    applying = latest_backup(layout)
    applying = applying.plan if applying and applying.is_open() else prepared
    baseline = baseline_rows(layout, applying)
    while True:
        st = find_state(layout, prepared, baseline)
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
                step_14(layout, backup, baseline)
            log("result: complete")
            return 0
        if state == "active":
            continue_after_switch(layout, backup, resumed=True, baseline=baseline)
            log("result: complete")
            return 0
        break  # native
    if latest_backup(layout):
        _, failed = run_commands(layout, prepared, prepared["native_probe"], "3")
        if failed:
            raise Stop("the native probe failed: " + ", ".join(f["name"] for f in failed))
    evidence, admitted = {}, {}
    for step in ("4", "5", "6", "7"):
        results, failed = run_commands(layout, prepared, prepared["gates"][step], step, admitted)
        evidence[step] = results
        if failed:
            raise Stop(f"step {step} failed: " + ", ".join(f["name"] for f in failed))
        if step == "4":
            admitted = admitted_of(results)
    if not rehearsed(layout, prepared):
        raise Stop("no passing rehearsal record names the prepared plan and this program")
    if flagged:
        raise Stop("a kept copy is flagged; the owner merges it first")
    # The receipt names each formula version, so step 5's evidence stays too.
    backup = write_backup(layout, prepared, evidence)
    apply_switch(layout, backup)
    continue_after_switch(layout, backup, resumed=False)
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
        log(f"step 3: baseline {plan['baseline']['name'] if plan.get('baseline') else '(none)'}")
        for step in ("4", "5", "6", "7"):
            names = [c["name"] for c in plan.get("gates", {}).get(step, [])]
            log(f"step {step}: {', '.join(names) or '(no command)'}")
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
    if st["open"]:
        # The copy in the open backup reads the state that its continuation acts on.
        reexec_copy(st["backup"], args, ("state", "--json") if args.json else ("state",))
    flagged = flagged_copies(layout)
    if args.json:
        out = {
            "state": st["state"], "open": st["open"], "records": st["records"],
            "backup": st["backup"].path if st["backup"] else None,
            "plan_digest": st["plan"]["_digest"], "prepared_differs": st["prepared_differs"],
            "parts": st["parts"], "outside_changed": st["outside_changed"],
            "flagged": [flagged_summary(r) for r in flagged],
            "incomplete_backups": sorted(n for n in os.listdir(layout.backups) if n.startswith(".incomplete-"))
            if os.path.isdir(layout.backups) else [],
        }
        print(json.dumps(out, indent=2, sort_keys=True))
    else:
        report_state(st)
        report_flagged(layout)
    return 0


def all_cells(st: dict) -> list:
    """Each cell and entry of a state, with its value and its status."""
    out = []
    for part, status in zip(st["plan"]["parts"], st["parts"]):
        if part["kind"] == "file":
            out += [{"path": part["path"], "key": c["key"], "value": c["value"], "status": c["status"]}
                    for c in status["cells"]]
        else:
            out.append({"path": part["path"], "original": status["original"], "retained": status["retained"],
                        "status": status["status"]})
    return out


def third_cells(st: dict) -> list:
    """Each cell and entry of a state that holds neither its earlier nor its switched value."""
    return [c for c in all_cells(st) if c["status"] == "third"]


def append_manual(layout: Layout, text: str, what: str) -> None:
    """Append to the maintained manual. The manual stays append-only."""
    try:
        fd = os.open(layout.manual, os.O_WRONLY | os.O_APPEND)
    except OSError as e:
        raise Stop(f"the {what} could not be written into the manual: {e.strerror}")
    try:
        os.write(fd, text.encode())
        os.fsync(fd)
    finally:
        os.close(fd)


def inverse_command(args) -> int:
    """The owner's inverse: "Rollback", run by the program copy of the latest backup.

    It runs the narrow process inspection. It resets each switched cell and
    leaves each third value. It moves an entry back only onto an absent original
    path, whatever the entry's change identity. Then it runs the native
    probe and appends one line to the maintained manual.
    """
    prepared = load_plan(args.plan)
    layout = Layout(args.home, prepared)
    RUN_LOG.update(layout=layout, command="inverse", plan=prepared)
    backup = latest_backup(layout)
    if backup is None:
        raise Stop("no complete backup exists, so no inverse can return to native delivery")
    reexec_copy(backup, args, ("inverse",))
    log(f"program: {PROGRAM_DIGEST}")
    st = find_state(layout, prepared)
    report_state(st)
    report_flagged(layout)
    if not st["open"]:
        raise Stop(f"the run of {backup.id} is not open, so the inverse has no switched cell to reset")
    run_inverse(layout, backup)
    _, failed = run_commands(layout, backup.plan, backup.plan["native_probe"], "inverse")
    after = find_state(layout, prepared)
    report_state(after)
    left = third_cells(after)
    for cell in left:
        log(f"third value left: {canon(cell)}")
    line = {"backup": backup.id, "at": now(), "state": after["state"],
            "native_probe": "failed: " + ", ".join(f["name"] for f in failed) if failed else "passed",
            "third_values_left": left}
    append_manual(layout, f"\n<!-- owner-switch-inverse {backup.id} --> The owner's inverse ran: "
                          f"`{canon(line)}`\n", "line of the owner's inverse")
    log(f"manual: the owner's inverse is recorded in {layout.manual}")
    probe = ("; the native probe failed: " + ", ".join(f["name"] for f in failed)) if failed else ""
    if after["state"] != "native":
        raise Stop(f"the inverse ended in the {after['state']} state; stop for classification{probe}")
    if failed:
        raise Stop("the inverse ended in the native state" + probe)
    log("result: native")
    return 0


def purge_command(args) -> int:
    """Check the purge conditions that the program can read ("Purge").

    It reads the `retained` list of each receipt, or the entry parts of a
    backup that holds no receipt. It changes nothing.
    """
    prepared = load_plan(args.plan)
    layout = Layout(args.home, prepared)
    problems = []
    st = find_state(layout, prepared)
    if st["open"] and st["state"] != "complete":
        # A complete switch keeps its run open. Any other open run still needs its backup for the inverse.
        problems.append(f"the run of {st['backup'].id} is open in the {st['state']} state; the inverse "
                        "still needs its backup")
    for name in complete_backups(layout):
        backup = Backup(os.path.join(layout.backups, name))
        if "receipt" in backup.records():
            items = backup.record_body("receipt").get("retained") or []
        else:
            items = [{"item": "retained entry", "path": backup.retained_path(p["path"]), "original": p["path"],
                      "identity": p["identity"]} for p in backup.plan["parts"] if p["kind"] == "entry"]
        for item in items:
            if item.get("item") != "retained entry":
                log(f"{name}: {item.get('item')} {item.get('path') or item.get('plugin') or item.get('marketplace')}")
                continue
            now_id = identity(item["path"])
            if now_id is None:
                log(f"{name}: retained entry {item['original']}: absent from the backup")
            elif now_id != item["identity"]:
                problems.append(f"the retained entry {item['path']} changed its identity since its move")
            else:
                log(f"{name}: retained entry {item['original']}: keeps its recorded change identity")
    for r in flagged_copies(layout):
        problems.append(f"the {'held' if r.get('kind') == 'held' else 'kept'} copy {r['_copy']} is flagged")
    for condition in (PURGE_CONDITIONS[0], PURGE_CONDITIONS[1], PURGE_CONDITIONS[4]):
        log(f"the owner confirms: {condition}")
    for p in problems:
        log(f"  purge condition fails: {p}")
    if problems:
        raise Stop("the purge conditions do not hold")
    log("result: the conditions that the program reads hold")
    return 0


# --- the run log and the pending-activation record ----------------------------------

RUN_LOG = {"layout": None, "command": None, "plan": None, "commands": []}


def write_run_log(result: str, reason: str) -> None:
    """Keep the evidence of a stopped or ended run for the pending-activation record."""
    layout = RUN_LOG["layout"]
    if layout is None:
        return
    folder = os.path.join(layout.state, "runs")
    os.makedirs(folder, exist_ok=True)
    body = {"command": RUN_LOG["command"], "result": result, "reason": reason, "at": now(),
            "program_digest": PROGRAM_DIGEST, "plan_digest": (RUN_LOG["plan"] or {}).get("_digest"),
            "commands": RUN_LOG["commands"]}
    write_exclusive(os.path.join(folder, new_record_name(result) + ".json"), dump_json(body))


def latest_run_log(layout: Layout):
    folder = os.path.join(layout.state, "runs")
    names = sorted(n for n in os.listdir(folder) if n.endswith(".json")) if os.path.isdir(folder) else []
    return load_json_file(os.path.join(folder, names[-1])) if names else None


def pending_records(layout: Layout) -> list:
    try:
        with open(layout.manual, encoding="utf-8") as f:
            return re.findall(r"<!-- owner-switch-pending (\S+) -->", f.read())
    except FileNotFoundError:
        return []


def pending_command(args) -> int:
    """Items 2 and 3 after "The switch inside finish": the pending-activation record."""
    prepared = load_plan(args.plan)
    layout = Layout(args.home, prepared)
    # The record's own readings run in HOME, with the environment of a plan's command.
    env = command_env(layout, "pending")
    os.environ.clear()
    os.environ.update(env)
    run = latest_run_log(layout) or {}
    results = {c.get("name"): c.get("result") for c in run.get("commands") or [] if c.get("result")}
    step4 = next((r for r in results.values() if r.get("step") == "4"), None)
    if step4:
        grove = dict(step4.get("grove") or {}, read="from step 4 of the run")
        ravel = {"path": step4.get("ravel_path"), "version": step4.get("ravel"), "read": "from step 4 of the run"}
    else:
        try:
            grove = dict(installed_grove(), read="by this record, because the run did not pass step 4")
        except Stop as e:
            grove = {"error": str(e), "read": "by this record, because the run did not pass step 4"}
        prefix = tool(["brew", "--prefix"]).stdout.strip()
        path = os.path.join(prefix, "bin", "ravel")
        version = tool([path, "--version"]).stdout.strip() if os.path.exists(path) else None
        ravel = {"path": path, "version": version, "read": "by this record, because the run did not pass step 4"}
    status = tool(["mise", "-C", layout.home, "bootstrap", "packages", "status", "--json"], cwd=layout.home)
    try:
        doc = json.loads(status.stdout)
    except ValueError:
        doc = None
    rows = {}
    if isinstance(doc, dict) and isinstance(doc.get("brew"), dict):
        rows = {r.get("package"): r for r in doc["brew"].get("packages") or [] if isinstance(r, dict)}
    formulae = sorted({s.get("formula") for s in (prepared["accepted"].get("sources") or {}).values()
                       if isinstance(s, dict)} - {None})
    failed = [c for c in run.get("commands") or [] if c.get("exit") != 0]
    st = find_state(layout, prepared)
    body = {
        "grove": dict(grove, release_commit=args.grove_commit or "not supplied"),
        "ravel": ravel,
        "formula_versions": {f: (rows.get(f) or {}).get("installed_version") for f in formulae},
        "mise_status": {"exit": status.returncode, "output": (status.stdout + status.stderr)[-20000:]},
        "run": {k: run.get(k) for k in ("command", "result", "reason", "at", "program_digest", "plan_digest")}
        if run else "no run log exists",
        "failed": [{k: c.get(k) for k in ("name", "exit", "output")} for c in failed],
        "backups": [os.path.join(layout.backups, n) for n in complete_backups(layout)],
        "state": st["state"],
        "cells": all_cells(st) if st["state"] in ("drifted", "changed") else [],
        "outside_changed": st["outside_changed"],
        "flagged": [flagged_summary(r) for r in flagged_copies(layout)],
    }
    stamp = new_record_name("pending")
    append_manual(layout, f"\n<!-- owner-switch-pending {stamp} -->\n"
                          f"## Owner skill switch pending activation {stamp}\n\n"
                          "The switch stopped or was inverted. The receipt of a later switch replaces this record.\n\n"
                          "```json\n" + json.dumps(body, indent=2, sort_keys=True) + "\n```\n",
                  "pending-activation record")
    log(f"pending-activation record: {stamp} in {layout.manual}")
    return 0


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n", 1)[0])
    sub = parser.add_subparsers(dest="command", required=True)
    for name in ("state", "run", "inverse", "merge", "purge-check", "pending"):
        p = sub.add_parser(name)
        p.add_argument("--plan", required=True)
        p.add_argument("--home", default=account_home())
        if name == "state":
            p.add_argument("--json", action="store_true")
        elif name == "run":
            p.add_argument("--dry-run", action="store_true")
        elif name == "merge":
            p.add_argument("copy", help="the flagged kept copy, as `state` or a run reports it")
            p.add_argument("--record-only", action="store_true",
                           help="record a merge that the owner applied by hand")
        elif name == "pending":
            p.add_argument("--grove-commit", help="the release commit of the published Grove")
    p = sub.add_parser("native-off")
    p.add_argument("--home", default=account_home())
    p.add_argument("--print-command", action="store_true",
                   help="print the exact command for the prepared recovery instructions, and write nothing")
    p = sub.add_parser("processes")
    p.add_argument("--home", default=account_home())
    p.add_argument("--owned", help="a JSON list of probe-owned processes: pid, start, executable and home")
    p.add_argument("--json", action="store_true")
    sub.add_parser("identity").add_argument("path")
    sub.add_parser("digest").add_argument("path")
    p = sub.add_parser("gate")
    p.add_argument("step", choices=("4", "5", "6", "7"))
    p.add_argument("--plan")
    p.add_argument("--home")
    args = parser.parse_args(argv)
    try:
        if args.command == "gate":
            return gate_command(args)
        if args.command == "native-off":
            return native_off_command(args)
        if args.command == "processes":
            return processes_command(args)
        if args.command == "identity":
            print(identity(args.path) or "absent")
            return 0
        if args.command == "digest":
            print(path_digest(args.path) or "absent")
            return 0
        if args.command == "state":
            return state_command(args)
        if args.command == "merge":
            return merge_command(args)
        if args.command == "purge-check":
            return purge_command(args)
        if args.command == "pending":
            return pending_command(args)
        if args.command == "inverse":
            code = inverse_command(args)
        else:
            if args.dry_run:
                return dry_run(args)
            code = run_switch(args)
        write_run_log("ended", "")
        return code
    except Stop as e:
        log(f"stop: {e}")
        write_run_log("stop", str(e))
        return 3


if __name__ == "__main__":
    sys.exit(main())
