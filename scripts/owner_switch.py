#!/usr/bin/env python3
"""Run the owner's skill-delivery switch, its restore and its read-only reports.

docs/specs/owner-skill-delivery.md owns every rule this program applies: "The
native settings inside finish", "The switch inside finish", "Concurrent
writers", "Resuming the switch", "The rehearsed plan" and "Rollback". No
model checks these rules. The specification's section "The recovery model"
states why.

The program is one file. It uses only the standard library and runs on
Python 3.9 and later. It reaches renamex_np, the ACL calls and the
extended-attribute calls through ctypes, so it runs only on macOS.

Usage:
    python3 -I owner_switch.py report       --plan PLAN [--home HOME]
    python3 -I owner_switch.py native-off   [--home HOME] [--print-command]
    python3 -I owner_switch.py attempt      --plan PLAN [--home HOME]
    python3 -I owner_switch.py switch       --plan PLAN --attempt NOTE [--home HOME] [--guidance PATH]...
    python3 -I owner_switch.py restore      --backup DIR --attempt NOTE [--home HOME] [--recovery NOTE]
                                            [--guidance PATH]...
    python3 -I owner_switch.py status       --plan PLAN [--home HOME] [--json]
    python3 -I owner_switch.py purge-report --plan PLAN [--home HOME]
    python3 -I owner_switch.py processes    [--home HOME] [--owned FILE] [--json]
    python3 -I owner_switch.py gate         {4,5,6,7} [--plan PLAN] [--home HOME]
    python3 -I owner_switch.py identity     PATH
    python3 -I owner_switch.py digest       PATH
    python3 -I owner_switch.py restore-identity PATH

`report` is the read-only plan report. It lists each write of the plan with
its planned original and its current restore identity, each value that the
switch sets with its current value, and the restore order. It also compares
both expected inventories, evaluates both lists of delivery assertions,
compares each command input with its planned digest and lists each command.
It runs no command and changes nothing. `native-off`, `attempt`, `switch`
and `restore` are the switch and its recovery, which the section "The
switch" below states. `status` and `purge-report` read the switch's records,
as the section "Status and the purge report" states. `processes` runs the
narrow process inspection. `gate` runs the check of one of steps 4 to 7,
which a plan names as a command. `identity` prints the change identity of an
entry, `digest` prints the digest of a file, and `restore-identity` prints
the whole-file identity of a path, for a plan's author.

HOME defaults to the account's home directory. Each path in the plan is
relative to HOME, normalised and without "..". Before any step, the program
stops unless each path's parent folder resolves inside HOME.

Exit status: 0 for success or a report, 2 for a usage error, 3 for a stop or
a refusal before any switch-set write. `switch` and `restore` also exit with
4 when recovery is incomplete and the backup stays, and `switch` exits with
5 when the switch failed and restore verified native delivery.

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
process lifetime"). The switch runs it before its first write, after the
backup, before its final comparison and inside restore. It reads the process table and signals no process. A
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
<state>/prepared/owner_switch.py. `mac-check` runs that program's `status
--json` with that plan.

The plan (JSON, schema "owner-switch-plan/2"):
    state        directory of attempt notes, backups, receipts and restored
                 records, relative to HOME
    manual       the maintained manual, relative to HOME
    writes       [{"path", "type": "file"|"entry", "original":
                 {"absent": true} | {"identity": restore identity}}]: the
                 whole write set with each original's content, metadata or
                 absence. It names settings.json, known_marketplaces.json,
                 installed_plugins.json and the Ravel settings. The switch
                 deactivates each entry. A file may hold "set": [{"key":
                 [...], "value": V}], the values that the switch writes; a
                 value {"$absent": true} erases its key. The Ravel settings
                 set "skills" to a value that names no "sources".
    restore      the paths of writes in restore order, the Ravel settings first
    inventory    {"before": [...], "after": [...]}: the expected inventories.
                 Each item is {"path", "identity"} with a change identity, or
                 {"root", "entries": {name: identity}} for each child of a
                 root. A comparison skips only the platform subtrees
                 .claude/skills/synced and .codex/skills/.system, each while
                 it is a plain directory.
    inputs       [{"role", "path", "digest"}]: each command input by content.
                 "$program" names this program
    assertions   [{"path", "key", "value"} | {"path", "absent": true}]: the
                 delivery assertions of the switched layout, which the
                 receipt keeps apart from the restore identities. In place
                 of "value", "values" lists each accepted value
    native_assertions
                 the delivery assertions of native delivery, in the same
                 form. `status` reads them
    tools        {"claude": the held `claude --version` line}
    retained     optional [{"item", "path", "condition": [...]}]: further
                 retained resources for the receipt, such as old binaries
    baseline     command: the baseline instrument of step 3
    gates        {"4": [command], "5": [...], "6": [...], "7": [...]}
    accepted     the accepted values that the gates read:
                 sources   {id: {"formula", "resourceDigest"}}, exactly the
                           ten IDs of "Ravel sources"
                 versions  {formula: version} for each skill formula except
                           Grove's, whose accepted version step 4 admits
                 trials    {id: [command]}: the staging trials of a source
                 staging   {"home", "candidate_keg"}: the absolute paths of
                           the retained staging HOME and the candidate keg
    probes       [command]: the probes of step 7
    native_probe [command]: the native probes of restore
    mac_check    command: the `mac-check` of step 8
Each field except "retained" is required. Each list of commands names at
least one command, and the list of each of steps 4 to 7 names that step's
gate of this program. The switch's steps are fixed, so no plan can omit one
of their checks. Any other field is refused, so a plan holds no process, MCP
or job pattern, no earlier value of a key and no recovery field of an entry.
A command is {"name", "argv"}. The argv items "$python" and "$program" become
this program's interpreter and path. A command runs in HOME, with HOME set to
HOME. Its environment holds none of Grove's loop control variables, Claude
Code's five location variables, CODEX_HOME or a RAVEL_* name. It receives
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
The drafted real settings are the values that the plan sets in
.config/ravel/settings.json. The program passes step 4's admitted values to
the later gates as OWNER_SWITCH_ADMITTED.

The baseline. Each output row of the baseline command with a field
UNCLASSIFIED, UNASSIGNED, UNCOVERED or NEEDS-REVIEW counts as a changed entry
outside the switch set. The program prints each row whose field says
"unverified", for finish to compare with its own skill listing.

The hooks installation. Its paths and values are constants of this program,
not of the plan: technical-communication-hooks@writing-hooks from the
marketplace registered at $(brew --prefix)/opt/grove, read and installed
with `claude`, then disabled in .claude/settings.json by an ordinary write.
The program finds `claude` and `brew` on PATH. Each runs in HOME, with the
environment of a plan's command.

Rehearsal hook: OWNER_SWITCH_REHEARSAL_HOOK names a command that the program
runs at each named point with the point as its argument. Exit 99 ends the
program at once, as a crash. The program refuses the hook when HOME is the
account's home directory, so a real switch never honours it. The fixture
suite scripts/owner_switch_rehearsal.py uses it.

The switch. No operation continues from a run state: a crash leaves the
backup, and its attempt note names the restore.
    attempt   Preparation. Write the immutable attempt note
              <state>/attempts/<id>.json, mode 0444. It names the attempt
              ID, canonical HOME, the plan's path and digest, the program's
              path and digest, the planned command inputs, the observed
              tool versions, the backup <state>/backups/<id>, and the
              literal switch, restore and native-off commands. It records
              no progress. The operator verifies the note before `switch`.
    switch    Steps 1 to 9 of "The switch inside finish", with the note:
              1-3  Refuse unless the note fits HOME, the plan and the
                   program and its backup is absent. Refuse unless each
                   command input has its planned digest, `claude --version`
                   prints the plan's and the note's version, the narrow
                   process inspection is clear, each planned original has
                   its planned identity, the inventory is the expected one,
                   the Ravel settings hold the off value, the baseline
                   instrument reports no changed entry, and the gates of
                   steps 4 to 7 pass. A Claude Code command that must run
                   refuses an ACL or another attribute than
                   com.apple.provenance on settings.json,
                   known_marketplaces.json or installed_plugins.json, since
                   the commands replace all three. Each --guidance
                   file is recorded by digest.
              4    Copy each planned file and entry into the backup with
                   its content, mode, ACL, extended attributes and link
                   text, or record its absence. Verify the originals
                   against the copies, then publish the backup by one
                   rename. The backup holds the plan, the evidence of
                   steps 4 to 7 and the note's digest, and no program.
                   Inspect the processes and compare the originals again.
              5    Run `claude plugin marketplace add` and `claude plugin
                   install` serially, only when needed. After each command,
                   reread settings.json and refuse a change of another key.
                   Then write the plugin's disabled value.
              6    Move each entry of writes into <state>/discard/<id> and
                   erase it. Write the values of each file's "set", the
                   Ravel settings last. Each write keeps the file's other
                   keys and metadata, and the program rereads it.
              7    Run the probes. Each probe names the processes that it
                   created, as {"owned": [{"pid", "start", "executable",
                   "home"}]} in its result and as one such object per line
                   in the journal that OWNER_SWITCH_PROBE_JOURNAL names,
                   in <state>/probes. Send SIGTERM, then SIGKILL, to each
                   record that still runs, and only while its PID,
                   creation time and executable still equal the record.
                   One that still runs, or whose executable changed,
                   stops the switch with 4, and the backup stays. Inspect
                   the processes again.
              8    Run mac_check and the gate of step 6 again. Evaluate the
                   delivery assertions, the expected inventory after the
                   switch, the reader evidence and the guidance digests.
                   Record the restore identity of each restore destination.
              9    Write <state>/receipts/<id>.json and append the success
                   observation to the manual, then compare the layout again.
              A caught failure after step 4 invalidates the receipt,
              restores and exits with 5, or with 4 when restore fails. A
              crash leaves the backup: the next action is the note's
              restore command, and a repeated switch with that note refuses.
    restore   Verify each copy of the backup against its manifest, and the
              manifest's items against the plan's write set by path, type
              and copy name. A switch backup's items must also have the
              plan's original identities. Verify the backup against the
              note's digest, plan digest, kind, attempt ID, backup path and
              HOME. Refuse a mismatch with no write. Restore records each
              --guidance file by digest. The restored record names the
              admitted ancestors as the operator and keeps the guidance,
              which must still be readable at the end.

              With a valid receipt, this is a later restore: any restore
              destination whose identity differs from the receipt's
              refuses before the first write. An eligible later restore
              first writes a recovery note and a recovery backup of the
              current layout. Its retry command adds --recovery, which
              accepts each path at either the recovery backup's identity
              or the restored identity. After a completed restore, a
              repeated restore writes no restore destination while the
              layout stays the same. It only appends each manual entry
              that the earlier restore could not append.

              Restore writes each destination whole, or removes it for an
              original absence, in the order of the plan's restore, the Ravel
              settings first. It then runs native-off, verifies each
              destination against the backup, runs the native probes,
              invalidates the receipt, writes <state>/restored/<id>.json
              and appends to the manual. A success observation without a
              correction receives one.
Hook points: switch:start, backup:copy:<n>, backup:verify,
backup:before-publish, backup:published, hooks:before:<label>,
hooks:after:<label>, write:<path>:before, write:<path>:replace,
deactivate:<path>, probes, receipt:written, manual:<what>, switch:final,
restore:start, restore:<path> and restore:native-probes, besides gate:<point>
and command:<name>.

Status and the purge report ("Current health and restore eligibility" and
"Purge"). Both read the switch's records in <state> and the layout, and write
nothing. Exit status: 0 for a report, 3 when the plan is unusable or, for
`purge-report`, when purge is blocked.
    status    The latest switch attempt with a published backup governs.
              Delivery health is one of:
                native    no attempt, or a restored one: the native
                          assertions and inventory.before hold
                switched  the attempt has a valid receipt of the prepared
                          plan, and the assertions, inventory.after,
                          the receipt's reader evidence and the sources hold
                restore   a published backup has no receipt and no restored
                          record, or a recovery backup waits for its retry
                review    anything else: a partial layout, a failed
                          assertion, stale evidence or a record without its
                          attempt note
              The sources: `$(brew --prefix)/bin/ravel skills resolve` of the
              current Ravel skills value in a disposable HOME, compared by
              ID and resourceDigest with the receipt's. Restore eligibility
              is separate: eligible, ineligible with each restore
              destination that differs from the receipt's post-switch
              identity, unavailable when the backup does not verify,
              required with each restore command, or none. A restored
              attempt whose manual lacks its entries is a pending manual
              entry. The switch runs `mac-check` at its step 8, before its
              receipt: when the switch of the governing attempt's note is an
              ancestor of this process, its layout is evaluated as switched
              from the plan and the backup's evidence, and its restore is
              "in-progress". A crashed switch is no ancestor. --json prints
              {"health", "attempt", "receipt", "plan_digest", "problems",
              "restore": {"status", "differing", "commands"},
              "pending_manual": [{"attempt", "command"}]}.
    purge-report
              Block unless the health is switched. Compare each retained
              resource of the valid receipt with its recorded identity: a
              path by its change identity without the top-level .in_use
              reader markers, a registration by the digest of its entry.
              List each condition that only the owner establishes.
"""
from __future__ import annotations

import argparse
import contextlib
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
import signal
import subprocess
import sys
import tempfile
import time

PLAN_SCHEMA = "owner-switch-plan/2"
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

# The gates: docs/specs/owner-skill-delivery.md, steps 4 to 7 of "The switch
# inside finish", "Ravel sources" and "Skill formulae".
FORMULA_OF_GROVE = "linkuistics/taps/grove"
SOURCE_IDS = ("design", "engineering", "grove-core", "personal", "quint", "requirements",
              "research", "superpowers", "testanyware", "writing")
# docs/evaluations/owner-skill-delivery/inventory.py: a row that counts as a
# changed entry, and the digest of a row that only finish's listing verifies.
ANOMALIES = ("UNCLASSIFIED", "UNASSIGNED", "UNCOVERED", "NEEDS-REVIEW")
UNVERIFIED = "unverified"

# docs/specs/owner-skill-delivery.md, "The native settings inside finish"
# and "The hooks installation inside finish".
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
# Grove 22.8's Codex snapshot store ("Resource lifetime").
CODEX_SNAPSHOT_STORE = ".agents/.grove"

PROGRAM_PATH = os.path.realpath(__file__)
with open(PROGRAM_PATH, "rb") as _f:
    PROGRAM_BYTES = _f.read()
PROGRAM_DIGEST = hashlib.sha256(PROGRAM_BYTES).hexdigest()


class Stop(Exception):
    """A stop: the program changes nothing more, or restores what it changed."""


class Conflict(Exception):
    """A key of a written value crosses a value that is not an object."""


# --- macOS calls -----------------------------------------------------------

_libc = ctypes.CDLL(None, use_errno=True)
RENAME_EXCL = 0x4
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
    """renamex_np(2). RENAME_EXCL refuses an existing target."""
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


def identity(path: str, skip=()):
    """The change identity of an entry, or None when nothing is at the path.

    It covers each relative path, the type, the executable bit and content of
    each file, and the target of each link, without following a link. SKIP
    names direct children of the entry that the identity leaves out.
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
                if rel == "." and name in skip:
                    continue
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


PLAN_FIELDS = ("schema", "state", "manual", "writes", "restore", "inventory", "inputs", "assertions",
               "native_assertions", "tools", "retained", "baseline", "gates", "accepted", "probes",
               "native_probe", "mac_check")
DIGEST = re.compile(r"[0-9a-f]{64}")


def is_key(key) -> bool:
    return isinstance(key, list) and bool(key) and all(isinstance(k, str) for k in key)


def validate_plan(plan: dict) -> None:
    """Refuse a plan that lacks a field, a check or a HOME-relative path, or holds another field.

    Each step of "The switch inside finish" is fixed, so no plan can omit
    the baseline, the gates, the probes or `mac-check`. Each gate list names
    the program's own gate of its step. A field outside PLAN_FIELDS, such
    as a process, MCP or job pattern, is refused.
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

    other = [k for k in plan if k not in PLAN_FIELDS]
    if other:
        problems.append(f"the plan holds {', '.join(other)}, which schema {PLAN_SCHEMA} does not name; "
                        "the narrow process inspection is built in, and the owner's checklist replaces "
                        "process, MCP and job patterns")
    path("state", plan.get("state"))
    path("manual", plan.get("manual"))

    writes = plan.get("writes")
    if not isinstance(writes, list) or not writes:
        problems.append("writes names no write")
        writes = []
    types = {}
    for i, w in enumerate(writes):
        if not isinstance(w, dict) or w.get("type") not in ("file", "entry"):
            problems.append(f"writes[{i}] is neither a file nor an entry")
            continue
        path(f"writes[{i}]", w.get("path"))
        if w.get("path") in types:
            problems.append(f"writes names {w.get('path')!r} twice")
        types[w.get("path")] = w["type"]
        original = w.get("original")
        if not (same(original, {"absent": True}) or (isinstance(original, dict) and list(original) == ["identity"]
                                                       and DIGEST.fullmatch(str(original["identity"])))):
            problems.append(f"writes[{i}] has no original: {{\"absent\": true}} or {{\"identity\": <digest>}}")
        extra = sorted(set(w) - {"path", "type", "original", "set"})
        if extra or (w["type"] == "entry" and "set" in w):
            problems.append(f"writes[{i}] holds {', '.join(extra) or 'set'}: an entry is deactivated, "
                            "and a file names only the values that the switch sets")
        values = w.get("set", [])
        if not isinstance(values, list) or not all(isinstance(v, dict) and sorted(v) == ["key", "value"]
                                                   and is_key(v["key"]) for v in values):
            problems.append(f"writes[{i}].set is not a list of keys with values")
    for rel in HOOK_FILES + (RAVEL_SETTINGS,):
        if types.get(rel) != "file":
            problems.append(f"writes does not name the file {rel}")
    restore = plan.get("restore")
    if not isinstance(restore, list) or sorted(map(str, restore)) != sorted(map(str, types)):
        problems.append("restore must name each path of writes once")
    elif restore[0] != RAVEL_SETTINGS:
        problems.append(f"restore must restore {RAVEL_SETTINGS} first")

    inventory = plan.get("inventory")
    if not isinstance(inventory, dict) or sorted(inventory) != ["after", "before"]:
        problems.append("inventory must name the expected inventories before and after")
    else:
        for when in ("before", "after"):
            items = inventory[when]
            if not isinstance(items, list) or not items:
                problems.append(f"inventory.{when} names no entry")
                continue
            for i, item in enumerate(items):
                where = f"inventory.{when}[{i}]"
                if isinstance(item, dict) and isinstance(item.get("entries"), dict) and "root" in item:
                    path(where, item["root"])
                    for name in item["entries"]:
                        if not name or "/" in name or name in (".", ".."):
                            problems.append(f"{where} names the child {name!r}")
                elif isinstance(item, dict) and "path" in item and "identity" in item:
                    path(where, item["path"])
                else:
                    problems.append(f"{where} is neither a path with an identity nor a root with entries")

    inputs = plan.get("inputs")
    if not isinstance(inputs, list) or not any(isinstance(r, dict) and r.get("path") == "$program" for r in inputs):
        problems.append("inputs must name the program as \"$program\"")
        inputs = []
    for i, row in enumerate(inputs):
        if (not isinstance(row, dict) or not isinstance(row.get("role"), str)
                or not (row.get("path") == "$program" or (isinstance(row.get("path"), str)
                                                          and os.path.isabs(row["path"])))
                or not DIGEST.fullmatch(str(row.get("digest")))):
            problems.append(f"inputs[{i}] needs a role, an absolute path or \"$program\", and a digest")
    for name in ("assertions", "native_assertions"):
        assertions = plan.get(name)
        if not isinstance(assertions, list) or not assertions:
            problems.append(f"{name} names no delivery assertion")
            assertions = []
        for i, a in enumerate(assertions):
            keyed = (isinstance(a, dict) and is_key(a.get("key")) and ("value" in a) != ("values" in a)
                     and (isinstance(a.get("values"), list) and a["values"] if "values" in a else True))
            absent = isinstance(a, dict) and a.get("absent") is True and "key" not in a
            if not (keyed or absent) or a.get("path") not in types:
                problems.append(f"{name}[{i}] needs a path of writes, and a key with a value, "
                                "a non-empty list of values, or \"absent\": true")
    tools = plan.get("tools")
    if not isinstance(tools, dict) or not isinstance(tools.get("claude"), str):
        problems.append("tools must name the held Claude Code version as \"claude\"")
    retained = plan.get("retained", [])
    for i, row in enumerate(retained if isinstance(retained, list) else [None]):
        if not isinstance(row, dict) or not isinstance(row.get("item"), str) or not isinstance(row.get("condition"),
                                                                                                list):
            problems.append(f"retained[{i}] needs an item and a condition list")

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
    if not problems:
        try:
            drafted_settings(plan)
        except Stop as e:
            problems.append(str(e))
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
        rels = [plan["state"], plan["manual"]] + [w["path"] for w in plan["writes"]]
        for when in ("before", "after"):
            # A root's children are read below it, so the root itself is a parent.
            rels += [i["root"] + "/child" if "root" in i else i["path"] for i in plan["inventory"][when]]
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


# --- exclusive writes and times ----------------------------------------------------

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


def now() -> str:
    return datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%S.%fZ")


# --- readings and the platform subtrees ----------------------------------------

def read_key(layout: Layout, rel: str, key: list):
    """The value at KEY of the JSON file REL: ABSENT for an absent file or key, UNREADABLE for another file."""
    try:
        fd = os.open(layout.path(rel), os.O_RDONLY | os.O_NOFOLLOW)
    except FileNotFoundError:
        return ABSENT
    except OSError:
        return UNREADABLE
    try:
        obj = parse_json(fd_reading(fd)[1])
    finally:
        os.close(fd)
    return get_cell(obj, key)


# "Concurrent writers": the only subtrees that a comparison skips. A probe
# session can rewrite them. Their ordinary siblings, and a link or file in
# place of either, are still compared.
PLATFORM_SUBTREES = (".claude/skills/synced", ".codex/skills/.system")


def platform_subtree(layout: Layout, rel: str) -> bool:
    p = layout.path(rel)
    return rel in PLATFORM_SUBTREES and os.path.isdir(p) and not os.path.islink(p)


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
    return report


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
           if k not in (HOOK_VAR, PLAN_VAR, PLAN_DIGEST_VAR, ADMITTED_VAR, JOURNAL_VAR)}
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


def run_command_full(layout: Layout, command: dict, step: str, plan=None, admitted=None, extra=None):
    """Run one command of a plan, and return its evidence and its standard output."""
    hook(layout, f"command:{command['name']}")
    env = command_env(layout, step, plan, admitted)
    env.update(extra or {})
    result = subprocess.run(command_argv(command["argv"]), cwd=layout.home, env=env, capture_output=True, text=True)
    return command_evidence(command, step, result.returncode, result.stdout, result.stderr), result.stdout


def command_evidence(command: dict, step: str, code: int, stdout: str, stderr: str) -> dict:
    """The evidence of one finished command, with its tail in the log when it failed or reported."""
    output = (stdout + stderr)[-20000:]
    log(f"command {command['name']} (step {step}): exit {code}")
    parsed = command_result(stdout)
    if code != 0 or parsed is not None:
        # A failed command's tail, and a gate's report without its result line.
        lines = [x for x in output.strip().splitlines() if not x.startswith(RESULT_MARK)]
        for line in lines[-40:]:
            log(f"  | {line}")
    evidence = {"name": command["name"], "argv": command["argv"], "exit": code, "output": output}
    if parsed is not None:
        evidence["result"] = parsed
    return evidence


def run_command(layout: Layout, command: dict, step: str, plan=None, admitted=None) -> dict:
    return run_command_full(layout, command, step, plan, admitted)[0]


def run_commands(layout: Layout, plan: dict, commands: list, step: str, admitted=None):
    results = [run_command(layout, c, step, plan, admitted) for c in commands]
    failed = [r for r in results if r["exit"] != 0]
    return results, failed


# "Probe process lifetime": each probe writes a record of each process that
# it creates to the journal that this variable names, as soon as it sees the
# process. The program reads the journal after the probe, so a probe that
# dies before its result line still leaves its records.
JOURNAL_VAR = "OWNER_SWITCH_PROBE_JOURNAL"


# A probe ends its own launch within LAUNCH_TIMEOUT (1800 s) and its cleanup
# grace. PROBE_TIMEOUT is a generous outer bound, and PROBE_STOP_GRACE is the
# time that a probe receives after SIGTERM to end its harness and clean up.
PROBE_TIMEOUT, PROBE_STOP_GRACE = 3600.0, 180.0


def run_probe(layout: Layout, plan: dict, command: dict, step: str, journal: str) -> dict:
    """Run one probe with JOURNAL. An interrupt or the timeout sends the probe SIGTERM, so it cleans up.

    After an interrupt, the probe is stopped and the interrupt continues:
    run_probes then ends the records of this probe and of each earlier one.
    After the timeout, the probe counts as failed.
    """
    hook(layout, f"command:{command['name']}")
    env = command_env(layout, step, plan)
    env[JOURNAL_VAR] = journal
    launched = time.time()
    # Files rather than pipes: a service that inherited a pipe would hold it
    # open after the probe exits, and the program waits for the probe alone.
    with tempfile.TemporaryFile() as out, tempfile.TemporaryFile() as err:
        p = subprocess.Popen(command_argv(command["argv"]), cwd=layout.home, env=env, stdout=out, stderr=err)
        try:
            code = p.wait(timeout=PROBE_TIMEOUT)
        except BaseException as e:
            p.send_signal(signal.SIGTERM)
            try:
                p.wait(timeout=PROBE_STOP_GRACE)
            except subprocess.TimeoutExpired:
                p.kill()
                p.wait()
            if not isinstance(e, subprocess.TimeoutExpired):
                raise
            log(f"command {command['name']}: ran longer than {PROBE_TIMEOUT:.0f} seconds, and received SIGTERM")
            code = 124
        out.seek(0)
        err.seek(0)
        stdout = out.read().decode(errors="replace")
        stderr = err.read().decode(errors="replace")
    evidence = command_evidence(command, step, code, stdout, stderr)
    evidence["launched"] = launched
    evidence["journal"] = journal_records(journal, launched)
    return evidence


def journal_records(journal: str, launched: float) -> list:
    """The records of a probe's journal for processes created after the probe started."""
    return [r for r in read_journal(journal) if created_after(r, launched)]


def created_after(rec: dict, launched: float) -> bool:
    try:
        return float(rec["start"]) >= launched - 1.0
    except (TypeError, ValueError):
        return False


def run_probes(layout: Layout, plan: dict, commands: list, step: str):
    """Run the probes of STEP, each with a new journal in <state>/probes, and keep each journal's records.

    When a probe raises, as on an interrupt or a command that cannot start,
    the records of every probe started so far are ended before the error
    continues: the completed probes' results and journals, and the current
    probe's journal. A cleanup failure then leaves recovery incomplete. It
    replaces an ordinary error, so the switch cannot restore beside a live
    service. An interrupt continues after the logged failure.
    """
    folder = os.path.join(layout.state, "probes")
    os.makedirs(folder, exist_ok=True)
    results, current = [], None
    try:
        for c in commands:
            current = (os.path.join(folder, f"{step}-{c['name']}-{time.time_ns()}.jsonl"), time.time())
            results.append(run_probe(layout, plan, c, step, current[0]))
            current = None
    except BaseException as e:
        owned = probe_owned(results) + (journal_records(*current) if current else [])
        try:
            end_probe_services(layout, owned)
        except Unresolved as cleanup:
            if isinstance(e, Exception):
                raise cleanup from e
            log(f"  after the interrupt: {cleanup}")
        raise
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


# --- JSON files -------------------------------------------------------------------


def load_json_file(path: str):
    try:
        with open(path) as f:
            body = json.load(f)
    except (OSError, ValueError):
        return None
    return body if isinstance(body, dict) else None


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
    cell = read_key(layout, CLAUDE_SETTINGS, HOOKS_KEY)
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
    """The drafted real Ravel settings: the values that the plan sets in the Ravel settings."""
    settings = {}
    for w in plan["writes"]:
        if w["path"] == RAVEL_SETTINGS:
            for v in w.get("set", []):
                set_cell(settings, v["key"], v["value"])
    skills = settings.get("skills")
    if not isinstance(skills, dict) or "sources" in skills:
        raise Stop(f"the plan must set a skills value in {RAVEL_SETTINGS} that names no sources, "
                   "so that Ravel reads the default skill directory")
    return settings


def erase_tree(path: str) -> None:
    """Erase a disposable HOME. Ravel publishes each snapshot read-only."""
    for folder, _, _ in os.walk(path):
        os.chmod(folder, 0o700)
    shutil.rmtree(path, ignore_errors=True)


def resolve(prefix: str, settings: dict, base=None) -> dict:
    """`ravel skills resolve --json` with the promoted ravel, in a disposable HOME.

    BASE is the environment to start from, the program's own by default.
    """
    ravel = os.path.join(prefix, "bin", "ravel")
    home = tempfile.mkdtemp(prefix="owner-switch-resolve-")
    try:
        os.makedirs(os.path.join(home, os.path.dirname(RAVEL_SETTINGS)))
        with open(os.path.join(home, RAVEL_SETTINGS), "wb") as f:
            f.write(dump_json(settings))
        # Ravel reads its settings and its cache from HOME, and no RAVEL_*
        # variable may steer the resolution.
        env = {k: v for k, v in (os.environ if base is None else base).items() if not k.startswith("RAVEL_")}
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


# --- the baseline instrument and the manual ---------------------------------------

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


# --- the switch and its restore ----------------------------------------------
# docs/specs/owner-skill-delivery.md: "The switch inside finish", "The hooks
# installation inside finish", "Claude Code's own writes", "Concurrent
# writers", "Resuming the switch" and "Rollback".

ATTEMPT_SCHEMA = "owner-switch-attempt/1"
BACKUP_SCHEMA = "owner-switch-backup/1"
RECEIPT_SCHEMA = "owner-switch-receipt/1"
KNOWN_MARKETPLACES = ".claude/plugins/known_marketplaces.json"
INSTALLED_PLUGINS = ".claude/plugins/installed_plugins.json"
# The files that the tested Claude Code commands change ("The hooks installation inside finish").
HOOK_FILES = (CLAUDE_SETTINGS, KNOWN_MARKETPLACES, INSTALLED_PLUGINS)
# Discovery roots: no attempt note or backup may lie inside one, or inside an inventory root.
DISCOVERY_ROOTS = (".claude/skills", ".codex/skills", ".agents/skills")
# Exit statuses of attempt, switch and restore.
EXIT_REFUSED = 3      # refused before any switch-set write
EXIT_INCOMPLETE = 4   # recovery incomplete: the backup stays, and the named paths need the owner
EXIT_RESTORED = 5     # the switch failed, and restore verified native delivery
WRITE_TEMP = ".owner-switch-"
SERIAL_PURGE = (
    "no session that ran during the switch remains",
    "the driver's group is reaped",
    "each retained resource keeps its recorded identity",
    "the owner decides that the rollback window is closed",
)


class Unresolved(Exception):
    """Recovery is incomplete: the backup stays, and the owner resolves the named cause first."""


# --- identities of whole files and entries ------------------------------------------

def meta_digest(fd: int, content: bytes) -> str:
    """The digest of a node's mode, ACL, extended attributes and content.

    It leaves out com.apple.provenance, which macOS gives each new file of
    this process ("Claude Code's own writes").
    """
    st = os.fstat(fd)
    h = hashlib.sha256()
    _frame(h, b"mode", b"%o" % (st.st_mode & 0o7777))
    _frame(h, b"acl", _acl_text(fd))
    for name, value in _xattrs(fd):
        if name != PROVENANCE:
            _frame(h, b"xattr " + name, value)
    _frame(h, b"content", content)
    return h.hexdigest()


def nodes_of(path: str):
    """Each node of the file or entry at PATH, without following a link, or None when nothing is there."""
    if not os.path.lexists(path):
        return None
    out = []

    def walk(p: str, rel: str) -> None:
        st = os.lstat(p)
        if stat.S_ISLNK(st.st_mode):
            out.append({"rel": rel, "type": "link", "link": os.readlink(p)})
        elif stat.S_ISDIR(st.st_mode):
            fd = os.open(p, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW)
            try:
                out.append({"rel": rel, "type": "dir", "digest": meta_digest(fd, b"")})
            finally:
                os.close(fd)
            for name in sorted(os.listdir(p)):
                walk(os.path.join(p, name), f"{rel}/{name}")
        elif stat.S_ISREG(st.st_mode):
            fd = os.open(p, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
            try:
                out.append({"rel": rel, "type": "file", "digest": meta_digest(fd, read_fd(fd))})
            finally:
                os.close(fd)
        else:
            raise Stop(f"{p} is neither a file, a folder nor a link, so no backup can hold it")

    try:
        walk(path, ".")
    except OSError as e:
        # A failed content or metadata read is a refusal, never a partial identity.
        raise Stop(f"{e.filename or path} cannot be read: {e.strerror}")
    return out


def nodes_identity(nodes) -> str:
    return "absent" if nodes is None else hashlib.sha256(canon(nodes).encode()).hexdigest()


def restore_identity(path: str) -> str:
    """The whole-file identity of a restore destination: "absent", or the digest of its nodes."""
    return nodes_identity(nodes_of(path))


def copy_tree(src: str, dst: str) -> None:
    """Copy the entry SRC to the new path DST: each node's content, mode, ACL and attributes, and each link's text."""
    st = os.lstat(src)
    if stat.S_ISLNK(st.st_mode):
        os.symlink(os.readlink(src), dst)
    elif stat.S_ISDIR(st.st_mode):
        os.mkdir(dst, 0o700)
        for name in sorted(os.listdir(src)):
            copy_tree(os.path.join(src, name), os.path.join(dst, name))
        # The folder's own metadata comes last, because its mode can forbid new children.
        sfd = os.open(src, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW)
        dfd = os.open(dst, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW)
        try:
            copy_metadata(sfd, dfd)
        finally:
            os.close(sfd)
            os.close(dfd)
    elif stat.S_ISREG(st.st_mode):
        sfd = os.open(src, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
        try:
            dfd = os.open(dst, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600)
            try:
                write_all(dfd, read_fd(sfd))
                copy_metadata(sfd, dfd)
                os.fsync(dfd)
            finally:
                os.close(dfd)
        finally:
            os.close(sfd)
    else:
        raise Stop(f"{src} is neither a file, a folder nor a link, so no backup can hold it")


def discard_entry(path: str, discard: str) -> None:
    """Move the entry at PATH out of its folder at once, then erase it inside the discard folder."""
    os.makedirs(discard, 0o700, exist_ok=True)
    gone = os.path.join(discard, f"gone-{secrets.token_hex(6)}")
    os.rename(path, gone)
    if os.path.isdir(gone) and not os.path.islink(gone):
        erase_tree(gone)
    else:
        os.unlink(gone)


def strict_file(path: str):
    """The JSON object of a settings file, {} when it is absent, or a stop for any other content."""
    try:
        fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
    except FileNotFoundError:
        return {}
    except OSError as e:
        raise Stop(f"{path} could not be read: {e.strerror}")
    try:
        if not stat.S_ISREG(os.fstat(fd).st_mode):
            raise Stop(f"{path} is not a regular file")
        content = read_fd(fd)
    finally:
        os.close(fd)
    obj, problem = strict_object(content)
    if obj is None:
        raise Stop(f"{path} is malformed: {problem}")
    return obj


def ravel_skills(layout: Layout):
    return strict_file(layout.path(RAVEL_SETTINGS)).get("skills", ABSENT)


# --- the plan report ----------------------------------------------------------------

def report_command(args) -> int:
    """The read-only plan report ("The switch inside finish"): the write set, the current values and the commands.

    It reads the plan, the files of its write set, its inventory roots and
    its command inputs. It runs no command and changes nothing.
    """
    plan = load_plan(args.plan)
    layout = Layout(args.home, plan)
    log(f"program: {PROGRAM_DIGEST}")
    log(f"plan: {plan['_path']} ({plan['_digest']})")
    log(f"home: {layout.home}")
    log("writes, in plan order:")
    for w in plan["writes"]:
        planned = "absent" if w["original"].get("absent") else w["original"]["identity"]
        current = restore_identity(layout.path(w["path"]))
        now_text = "its original" if current == planned else f"differs: {current}"
        log(f"  {w['type']} {w['path']}: original {planned}; now {now_text}")
        if w["type"] == "entry":
            log("    the switch deactivates it")
        for v in w.get("set", []):
            log(f"    set {key_name(v['key'])}: now {show(read_key(layout, w['path'], v['key']))}, "
                f"switched {show(v['value'])}")
    log("restore order: " + ", ".join(plan["restore"]))
    for when in ("before", "after"):
        changed = inventory_changes(layout, plan["inventory"][when])
        log(f"inventory {when}: " + ("matches" if not changed else "differs at " + ", ".join(changed)))
    for name in ("native_assertions", "assertions"):
        for a in assertion_results(layout, plan[name]):
            what = key_name(a["key"]) if "key" in a else "absent"
            log(f"{name}: {a['path']} {what}: {'holds' if a['holds'] else 'observed ' + show(a['observed'])}")
    log("command inputs:")
    for r in input_rows(plan):
        state = "its planned digest" if r["digest"] == r["planned"] else f"differs: {r['digest'] or 'unreadable'}"
        log(f"  {r['role']} {r['path']}: {state}")
    log(f"held Claude Code: {plan['tools']['claude']}")
    log("commands:")
    named = [("3", [plan["baseline"]])] + [(s, plan["gates"][s]) for s in ("4", "5", "6", "7")]
    named += [("7", plan["probes"]), ("8", [plan["mac_check"]]), ("restore", plan["native_probe"])]
    for step, commands in named:
        for c in commands:
            log(f"  step {step} {c['name']}: {shlex.join(command_argv(c['argv']))}")
    log("result: reported; nothing written")
    return 0


# --- the attempt note ---------------------------------------------------------------

def file_digest(path: str):
    try:
        with open(path, "rb") as f:
            return hashlib.sha256(f.read()).hexdigest()
    except OSError:
        return None


def input_rows(plan: dict) -> list:
    """Each command input of the plan, with its planned and its current content digest."""
    rows = []
    for row in plan["inputs"]:
        path = PROGRAM_PATH if row["path"] == "$program" else row["path"]
        rows.append({"role": row["role"], "path": path, "planned": row["digest"], "digest": file_digest(path)})
    return rows


def observed_tools(layout: Layout) -> dict:
    result = run_tool(layout, ["claude", "--version"])
    return {"claude": result.stdout.strip() if result.returncode == 0 else None,
            "python": sys.version.split()[0], "interpreter": sys.executable}


def under(path: str, roots: list) -> str:
    """The first root that holds PATH, or ""."""
    real = os.path.realpath(path)
    for root in roots:
        r = os.path.realpath(root)
        if real == r or real.startswith(r + os.sep):
            return root
    return ""


def placement_problem(layout: Layout, plan: dict, path: str) -> str:
    """Why PATH may not hold an attempt note or a backup, or ""."""
    if ".grove" in os.path.realpath(path).split(os.sep):
        return "it lies inside a .grove folder"
    roots = [layout.path(r) for r in DISCOVERY_ROOTS]
    for when in ("before", "after"):
        roots += [layout.path(i.get("root") or i["path"]) for i in plan["inventory"][when]]
    roots += [layout.path(w["path"]) for w in plan["writes"]]
    root = under(path, roots)
    if root:
        return f"it lies inside the discovery, inventory or write path {root}"
    real = os.path.realpath(path)
    if not real.startswith(layout.home + os.sep):
        return "it lies outside HOME"
    return ""


def new_attempt_id() -> str:
    return datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%S%fZ") + "-" + secrets.token_hex(3)


def program_argv(*args: str) -> list:
    return [sys.executable, "-I", PROGRAM_PATH, *args]


def write_note(layout: Layout, plan: dict, note: dict) -> str:
    folder = os.path.join(layout.state, "attempts")
    path = os.path.join(folder, note["attempt"] + ".json")
    for p in (path, note["backup"]):
        why = placement_problem(layout, plan, p)
        if why:
            raise Stop(f"{p} cannot hold the attempt's records: {why}")
    os.makedirs(folder, 0o700, exist_ok=True)
    write_exclusive(path, dump_json(note))
    os.chmod(path, 0o444)
    return path


def attempt_command(args) -> int:
    """Preparation: write the immutable attempt note that names the switch and its restore."""
    plan = load_plan(args.plan)
    layout = Layout(args.home, plan)
    attempt = new_attempt_id()
    backup = os.path.join(layout.state, "backups", attempt)
    note_path = os.path.join(layout.state, "attempts", attempt + ".json")
    note = {
        "schema": ATTEMPT_SCHEMA, "kind": "switch", "attempt": attempt, "home": layout.home,
        "plan": {"path": plan["_path"], "digest": plan["_digest"]},
        "program": {"path": PROGRAM_PATH, "digest": PROGRAM_DIGEST},
        "inputs": [{"role": r["role"], "path": r["path"], "digest": r["planned"]} for r in input_rows(plan)],
        "tools": observed_tools(layout),
        "backup": backup,
        "switch": shlex.join(program_argv("switch", "--plan", plan["_path"], "--home", layout.home,
                                          "--attempt", note_path)),
        "restore": shlex.join(program_argv("restore", "--backup", backup, "--attempt", note_path,
                                           "--home", layout.home)),
        "native_off": shlex.join(native_off_argv(layout.home)),
        "created": now(),
    }
    path = write_note(layout, plan, note)
    log(f"attempt note: {path}")
    log(f"switch: {note['switch']}")
    log(f"restore: {note['restore']}")
    log(f"native-off: {note['native_off']}")
    return 0


def load_note(path: str) -> dict:
    try:
        with open(path, "rb") as f:
            raw = f.read()
        note = json.loads(raw)
    except (OSError, ValueError) as e:
        raise Stop(f"the attempt note {path} cannot be read: {e}")
    if not isinstance(note, dict) or note.get("schema") != ATTEMPT_SCHEMA:
        raise Stop(f"{path} is not an attempt note of schema {ATTEMPT_SCHEMA}")
    for key in ("kind", "attempt", "home", "backup", "restore"):
        if not isinstance(note.get(key), str):
            raise Stop(f"the attempt note {path} has no {key}")
    note["_path"] = os.path.realpath(path)
    note["_digest"] = hashlib.sha256(raw).hexdigest()
    return note


# --- the backup ---------------------------------------------------------------------

def create_backup(layout: Layout, plan: dict, note: dict, evidence: dict) -> dict:
    """Step 4: one complete backup of the whole write set, verified before its publication.

    The backup holds data, never a program: each planned file and entry with
    its content, mode, ACL, extended attributes and link text, or its
    absence. It also holds the plan and the evidence of the attempt.
    """
    dest = note["backup"]
    if os.path.lexists(dest):
        raise Stop(f"the backup destination {dest} exists; a new attempt needs a new attempt note")
    parent = os.path.dirname(dest)
    os.makedirs(parent, 0o700, exist_ok=True)
    if os.stat(parent).st_dev != os.stat(layout.home).st_dev:
        raise Stop(f"the backup folder {parent} is not on HOME's volume")
    tmp = os.path.join(parent, f".incomplete-{os.path.basename(dest)}")
    if os.path.lexists(tmp):
        erase_tree(tmp)
    os.mkdir(tmp, 0o700)
    os.mkdir(os.path.join(tmp, "data"))
    items = []
    try:
        for i, w in enumerate(plan["writes"]):
            hook(layout, f"backup:copy:{i}")
            src = layout.path(w["path"])
            nodes = nodes_of(src)
            if nodes is not None:
                copy_tree(src, os.path.join(tmp, "data", str(i)))
            items.append({"path": w["path"], "type": w["type"], "data": f"data/{i}" if nodes is not None else None,
                          "nodes": nodes, "identity": nodes_identity(nodes)})
        manifest = {
            "schema": BACKUP_SCHEMA, "attempt": note["attempt"], "kind": note["kind"], "home": layout.home,
            "note": {"path": note["_path"], "digest": note["_digest"]},
            "plan": {"path": plan["_path"], "digest": plan["_digest"]}, "program_digest": PROGRAM_DIGEST,
            "evidence": evidence, "items": items, "created": now(),
        }
        with open(os.path.join(tmp, "plan.json"), "wb") as f:
            f.write(plan["_bytes"])
        with open(os.path.join(tmp, "manifest.json"), "wb") as f:
            f.write(dump_json(manifest))
            f.flush()
            full_sync(f.fileno())
        hook(layout, "backup:verify")
        for item in items:
            copy = nodes_of(os.path.join(tmp, item["data"])) if item["data"] else None
            original = nodes_of(layout.path(item["path"]))
            if canon(copy) != canon(item["nodes"]) or canon(original) != canon(item["nodes"]):
                raise Stop(f"the backup copy of {item['path']} differs from the original")
        hook(layout, "backup:before-publish")
        renamex(tmp, dest, RENAME_EXCL)
    except OSError as e:
        raise Stop(f"the backup could not be completed: {e}")
    # Nothing after the publication may stop here: each later failure belongs
    # to the caller, which restores ("Resuming the switch").
    log(f"backup: {dest}")
    return dict(manifest, path=os.path.realpath(dest), plan_doc=plan)


def backup_published(layout: Layout, backup: dict) -> None:
    """Flush the backup's folder after its publication."""
    dfd = os.open(os.path.dirname(backup["path"]), os.O_RDONLY)
    try:
        full_sync(dfd)
    finally:
        os.close(dfd)
    hook(layout, "backup:published")


def load_backup(path: str) -> dict:
    """A published backup, with each copy verified against its manifest."""
    path = os.path.realpath(path)
    try:
        with open(os.path.join(path, "manifest.json"), "rb") as f:
            manifest = json.loads(f.read())
    except (OSError, ValueError) as e:
        raise Stop(f"{path} holds no readable backup manifest: {e}")
    if not isinstance(manifest, dict) or manifest.get("schema") != BACKUP_SCHEMA:
        raise Stop(f"{path} is not a backup of schema {BACKUP_SCHEMA}")
    plan = load_plan(os.path.join(path, "plan.json"))
    if plan["_digest"] != (manifest.get("plan") or {}).get("digest"):
        raise Stop(f"the plan in {path} is not the plan that its manifest names")
    check_manifest(path, manifest, plan)
    for item in manifest["items"]:
        copy = nodes_of(os.path.join(path, item["data"])) if item["data"] else None
        if canon(copy) != canon(item["nodes"]) or nodes_identity(copy) != item["identity"]:
            raise Stop(f"the backup copy of {item['path']} in {path} differs from its manifest")
    return dict(manifest, path=path, plan_doc=plan)


def check_manifest(path: str, manifest: dict, plan: dict) -> None:
    """Stop unless the manifest's items are the plan's write set, in its order.

    Each item names its write by path and type, and its copy only as
    data/<n>. A switch backup's item identities must also equal the plan's
    originals, which the switch verified before it made the backup. The
    note names the plan by digest, so the chain from the immutable note to
    each copy has no link that a changed manifest alone can satisfy. A
    recovery backup holds the layout that its later restore compared, which
    no earlier record names.
    """
    items = manifest.get("items")
    writes = plan["writes"]
    if not isinstance(items, list) or len(items) != len(writes):
        raise Stop(f"the manifest of {path} does not name the plan's {len(writes)} writes")
    for i, (item, w) in enumerate(zip(items, writes)):
        if not isinstance(item, dict) or item.get("path") != w["path"] or item.get("type") != w["type"]:
            raise Stop(f"item {i} of the manifest of {path} is not the plan's {w['type']} {w['path']}")
        if item.get("data") not in (None, f"data/{i}") or (item["data"] is None) != (item.get("nodes") is None):
            raise Stop(f"item {i} of the manifest of {path} names its copy as {item.get('data')!r}")
        if manifest.get("kind") == "switch":
            wanted = "absent" if w["original"].get("absent") else w["original"]["identity"]
            if item.get("identity") != wanted:
                raise Stop(f"the manifest of {path} records {w['path']} at another identity than the plan's "
                           f"original")


# --- ordinary writes and the hooks installation ------------------------------------

def write_cells(layout: Layout, rel: str, cells: list) -> None:
    """One ordinary write: set CELLS in the current file, keep its other keys and its metadata, and reread it."""
    path = layout.path(rel)
    hook(layout, f"write:{rel}:before")
    try:
        sfd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
    except FileNotFoundError:
        sfd = None
    try:
        content = read_fd(sfd) if sfd is not None else None
        obj = strict_file(path) if sfd is not None else {}
        for key, value in cells:
            set_cell(obj, key, value)
        data = dump_json(obj)
        folder = os.path.dirname(path)
        os.makedirs(folder, exist_ok=True)
        tmp = os.path.join(folder, f".{os.path.basename(path)}{WRITE_TEMP}{secrets.token_hex(6)}")
        tfd = os.open(tmp, os.O_RDWR | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600)
        published = False
        try:
            write_all(tfd, data)
            if sfd is not None:
                copy_metadata(sfd, tfd)
                os.utime(tfd, None)
            else:
                os.fchmod(tfd, 0o644)
            full_sync(tfd)
            if sfd is not None and strict_file(path) != strict_object(content)[0]:
                raise Stop(f"{rel} changed while the switch wrote it; its writer must be paused")
            hook(layout, f"write:{rel}:replace")
            os.rename(tmp, path)
            published = True
        finally:
            os.close(tfd)
            if not published and os.path.lexists(tmp):
                os.unlink(tmp)
    finally:
        if sfd is not None:
            os.close(sfd)
    now_obj = strict_file(path)
    wrong = [key_name(k) for k, v in cells if not same(get_cell(now_obj, k), v)]
    if wrong:
        raise Stop(f"{rel} does not hold the written values of {', '.join(wrong)}")
    log(f"write: {rel}: {', '.join(f'{key_name(k)} = {show(v)}' for k, v in cells)}")


def without(obj: dict, keys: list) -> dict:
    out = json.loads(canon(obj))
    for key in keys:
        set_cell(out, key, ABSENT)
    return out


def lost_metadata(layout: Layout) -> str:
    """What a Claude Code command would drop: an ACL or another attribute than provenance, with its file.

    Measured with Claude Code 2.1.322 on 2026-11-10 (owner-staging-hooks-k251):
    `claude plugin marketplace add` and `claude plugin install` replace
    settings.json and both registries, and each new file keeps only
    com.apple.provenance. So each file of HOOK_FILES is checked.
    """
    for rel in HOOK_FILES:
        try:
            fd = os.open(layout.path(rel), os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
        except FileNotFoundError:
            continue
        try:
            acl = _acl_text(fd)
            others = [n.decode(errors="replace") for n, _ in _xattrs(fd) if n != PROVENANCE]
        finally:
            os.close(fd)
        if acl:
            return f"{rel} holds an ACL"
        if others:
            return f"{rel} holds the extended attribute " + ", ".join(others)
    return ""


def claude_command(layout: Layout, args: list, label: str, allowed: list) -> None:
    """Run one Claude Code command while the switch waits, then read the settings again."""
    hook(layout, f"hooks:before:{label}")
    lost = lost_metadata(layout)
    if lost:
        raise Stop(f"{lost}, which `claude {' '.join(args)}` would drop")
    before = strict_file(layout.path(CLAUDE_SETTINGS))
    result = run_tool(layout, ["claude"] + args)
    if result.returncode != 0:
        raise Stop(f"`claude {' '.join(args)}` failed: {(result.stdout + result.stderr).strip()[-2000:]}")
    hook(layout, f"hooks:after:{label}")
    after = strict_file(layout.path(CLAUDE_SETTINGS))
    if not same(without(after, allowed), without(before, allowed)):
        raise Stop(f"`claude {' '.join(args)}` changed other keys of {CLAUDE_SETTINGS} than "
                   f"{', '.join(key_name(k) for k in allowed)}")


def hooks_installation(layout: Layout, expected: str) -> None:
    """Step 5: register writing-hooks, install its plugin if absent, and disable it."""
    state = hooks_state(layout, expected)
    if state["kind"] == "other":
        raise Stop(f"the hooks installation found that {state['reason']}")
    if state["kind"] == "absent":
        if not state["registered"]:
            claude_command(layout, ["plugin", "marketplace", "add", expected], "marketplace-add",
                          [["extraKnownMarketplaces", HOOKS_MARKETPLACE]])
            state = hooks_state(layout, expected)
            if state["kind"] != "absent" or not state["registered"]:
                raise Stop(f"after `claude plugin marketplace add`, {state['reason']}")
        claude_command(layout, ["plugin", "install", HOOKS_PLUGIN], "install", [HOOKS_KEY])
        state = hooks_state(layout, expected)
    if state["kind"] == "enabled":
        write_cells(layout, CLAUDE_SETTINGS, [(HOOKS_KEY, False)])
        state = hooks_state(layout, expected)
    if state["kind"] != "disabled":
        raise Stop(f"the hooks installation ended with {state['reason']}")
    log(f"hooks installation: {state['reason']}")


# --- comparisons and probe services -------------------------------------------------

def inventory_changes(layout: Layout, items: list) -> list:
    """Each entry that differs from an expected inventory. Only the platform subtrees are skipped."""
    changed = []
    for item in items:
        if "root" in item:
            root = layout.path(item["root"])
            names = set(os.listdir(root)) if os.path.isdir(root) else set()
            for name in sorted(names | set(item["entries"])):
                rel = os.path.join(item["root"], name)
                if platform_subtree(layout, rel):
                    continue
                if identity(layout.path(rel)) != item["entries"].get(name):
                    changed.append(rel)
        elif identity(layout.path(item["path"])) != item["identity"]:
            changed.append(item["path"])
    return changed


def original_changes(layout: Layout, plan: dict) -> list:
    """Each planned original that differs from the plan: content, metadata, link text or absence."""
    out = []
    for w in plan["writes"]:
        wanted = "absent" if w["original"].get("absent") else w["original"]["identity"]
        if restore_identity(layout.path(w["path"])) != wanted:
            out.append(w["path"])
    return out


def assertion_results(layout: Layout, assertions: list) -> list:
    """Evaluate delivery assertions. An assertion with "values" holds when any of them is observed."""
    out = []
    for a in assertions:
        if a.get("absent"):
            observed = "absent" if not os.path.lexists(layout.path(a["path"])) else "present"
            holds = observed == "absent"
        else:
            try:
                observed = get_cell(strict_file(layout.path(a["path"])), a["key"])
            except Stop as e:
                observed = f"unreadable: {e}"
            holds = any(same(observed, v) for v in (a["values"] if "values" in a else [a["value"]]))
        out.append(dict(a, observed=observed, holds=holds))
    return out


def assertion_problems(results: list) -> list:
    return [f"the assertion {a['path']} {key_name(a['key']) if 'key' in a else 'absent'} observed "
            f"{show(a['observed'])}" for a in results if not a["holds"]]


def owned_record(rec) -> bool:
    """A record that can name an owned process: a PID above 1, a creation time, an executable and a HOME field.

    The executable is null when the probe could not read it. Such a record
    keeps its cleanup obligation: owned_state calls it "changed", so it
    authorises no signal, and the switch reports recovery incomplete while
    it runs.
    """
    return (isinstance(rec, dict) and type(rec.get("pid")) is int and rec["pid"] > 1
            and isinstance(rec.get("start"), str) and "executable" in rec
            and (rec["executable"] is None or isinstance(rec["executable"], str)) and "home" in rec)


def probe_owned(results: list) -> list:
    """The probe-owned process records of the probes: each result's "owned" list and each probe's journal.

    A later record of the same PID and creation time replaces an earlier
    one, because a process can exec another executable after the probe
    first saw it.
    """
    owned = {}
    for r in results:
        for rec in list((r.get("result") or {}).get("owned") or []) + list(r.get("journal") or []):
            # A process that existed before the probe started is never the probe's.
            if owned_record(rec) and ("launched" not in r or created_after(rec, r["launched"])):
                owned[(rec["pid"], rec["start"])] = rec
    return list(owned.values())


def read_journal(path: str) -> list:
    """The process records of a probe journal: one JSON object per line. A torn last line is skipped."""
    records = []
    try:
        with open(path, "rb") as f:
            lines = f.read().splitlines()
    except OSError:
        return []
    for line in lines:
        try:
            rec = json.loads(line)
        except ValueError:
            continue
        if owned_record(rec):
            records.append(rec)
    return records


# "Probe process lifetime": how long an owned process may take to end after
# SIGTERM, and after SIGKILL. A generous margin keeps a loaded machine from
# turning a slow exit into recovery incomplete.
TERM_GRACE, KILL_GRACE = 20.0, 10.0


def owned_state(rec: dict) -> str:
    """"gone", "running" or "changed" for one probe-owned record.

    "gone": no live process has the PID with the recorded creation time. The
    process ended, is a zombie, or the PID now names another process.
    "running": the process has the recorded PID, creation time and
    executable. "changed": it has the PID and creation time, but its
    executable differs or cannot be read, so its identity is not confirmed.
    """
    pid = rec["pid"]
    start = process_start(pid)
    if start is None:
        # The PID names no process, or one that this account cannot read, as
        # after an exec of a setuid program. Only the first is gone.
        try:
            os.kill(pid, 0)
        except ProcessLookupError:
            return "gone"
        except PermissionError:
            return "changed"
        return "gone" if zombie(pid) else "changed"
    if start != rec["start"] or process_status(pid) == SZOMB:
        return "gone"
    executable = executable_path(pid) or process_arguments(pid)[0]
    # An executable that cannot be read, now or when the probe recorded the
    # process, confirms nothing. A Codex 0.167.0 leader once read as
    # unreadable for its whole life (owner-switch-probes-k257).
    if executable is None or rec.get("executable") is None:
        return "changed"
    return "running" if executable == rec["executable"] else "changed"


def zombie(pid: int) -> bool:
    out = subprocess.run(["/bin/ps", "-o", "stat=", "-p", str(pid)], capture_output=True, text=True).stdout
    return "Z" in out or not out.strip()


def end_owned(records: list, term_grace: float = TERM_GRACE, kill_grace: float = KILL_GRACE,
              send=os.kill, watch=None) -> dict:
    """End each running probe-owned process, and verify its exit.

    The identity of a record is checked again immediately before each
    signal, so PID reuse never authorises one: a PID with another creation
    time or another executable is never signalled. SIGTERM comes first, then
    SIGKILL after TERM_GRACE. A process that still runs after KILL_GRACE,
    and a "changed" one, remains. SEND is the signal call, a test seam.
    WATCH, when given, runs during each wait, so the probe records a child
    that a process creates while it ends.
    """
    signalled = []

    def signal_running(sig) -> None:
        for rec in records:
            if owned_state(rec) == "running":
                try:
                    send(rec["pid"], sig)
                except OSError:
                    # Gone, or another account's process since the check: either way not this record.
                    continue
                signalled.append(dict(rec, signal=signal.Signals(sig).name))

    def wait(seconds: float) -> None:
        deadline = time.monotonic() + seconds
        while time.monotonic() < deadline and any(owned_state(r) == "running" for r in records):
            if watch is not None:
                watch()
            time.sleep(0.05)

    signal_running(signal.SIGTERM)
    wait(term_grace)
    signal_running(signal.SIGKILL)
    wait(kill_grace)
    states = [(rec, owned_state(rec)) for rec in records]
    return {"signalled": signalled,
            "ended": [rec for rec, state in states if state == "gone"],
            "remaining": [dict(rec, state=state) for rec, state in states if state != "gone"]}


def descendants(procs: dict, roots, snapshot: float) -> dict:
    """{pid: (relation, start)} for each root and each process created after its parent and before SNAPSHOT.

    PROCS is a process table read at SNAPSHOT, and ROOTS gives (pid, start,
    relation) for each process whose identity the caller confirmed. A child
    counts only when its creation time lies between its parent's and the
    reading, so a PID reused between the table and the identity read never
    counts ("Probe process lifetime").
    """
    children = {}
    for pid, p in procs.items():
        children.setdefault(p["ppid"], []).append(pid)
    stack, seen = list(roots), {}
    while stack:
        pid, start, relation = stack.pop()
        if pid in seen or pid not in procs or start is None:
            continue
        seen[pid] = (relation, start)
        for child in children.get(pid, []):
            child_start = process_start(child)
            if child_start is not None and float(start) <= float(child_start) <= snapshot:
                stack.append((child, child_start, "descendant"))
    return seen


def descendant_watch(records: list):
    """A WATCH for end_owned: append to RECORDS each child that a recorded process created after itself.

    A root is a record whose PID still has its recorded creation time. The
    switch uses this when it ends a probe's records after the probe, so a
    child that a service starts while it ends gains a record and a later
    round ends it. A failed process listing raises Stop.
    """
    uid = os.getuid()

    def watch() -> None:
        snapshot = time.time()
        procs = process_table()
        known = {(r["pid"], r["start"]) for r in records}
        roots = [(r["pid"], r["start"], "record") for r in records if process_start(r["pid"]) == r["start"]]
        for pid, (relation, start) in descendants(procs, roots, snapshot).items():
            p = procs[pid]
            if (pid, start) in known or p["uid"] != uid or p["zombie"]:
                continue
            launch, argv, home = process_arguments(pid)
            rec = {"pid": pid, "start": start, "executable": executable_path(pid) or launch, "home": home,
                   "argv": (argv or [])[:12], "ppid": p["ppid"], "relation": relation}
            if process_start(pid) != start:
                continue
            records.append(rec)
            log(f"  recorded probe-owned process {pid} ({rec['executable']}), a child of {p['ppid']}")
    return watch


# How many rounds of ending a cleanup takes while each finds a new survivor,
# and how long it watches for new children after each round.
CLEANUP_ROUNDS, CLEANUP_OBSERVE = 3, 1.0


def end_in_rounds(current, watch, rounds: int = CLEANUP_ROUNDS, observe: float = CLEANUP_OBSERVE) -> dict:
    """End each running record that CURRENT returns, in rounds, until a round finds no running one.

    WATCH records each child that a process creates. It runs before each
    round, during each wait and for OBSERVE seconds after each round, so a
    child created during a grace period gains a record and the next round
    ends it. The rounds stop at ROUNDS. Each record that is not "gone" at
    the end remains, with its state.
    """
    signalled = []
    for _ in range(rounds):
        watch()
        survivors = [dict(r) for r in current() if owned_state(r) == "running"]
        if not survivors:
            break
        signalled += end_owned(survivors, watch=watch)["signalled"]
        deadline = time.monotonic() + observe
        while time.monotonic() < deadline:
            watch()
            time.sleep(0.05)
    remaining = []
    for r in current():
        state = owned_state(r)
        if state != "gone":
            remaining.append(dict(r, state=state))
    return {"signalled": signalled, "remaining": remaining}


def end_probe_services(layout: Layout, owned: list) -> None:
    """"Probe process lifetime": end each probe-owned process that still runs, and verify its exit.

    Each probe reports the processes that it created, in its result and in
    its journal. The probe ends them itself. A record that still runs here
    is ended through end_owned, which never signals a reused PID, in rounds
    that record each child a recorded process creates (descendant_watch). A
    process that still runs afterwards, or whose identity is not confirmed,
    leaves recovery incomplete, and so does a cleanup that cannot read the
    process table.
    """
    records = [dict(r) for r in owned]
    try:
        outcome = end_in_rounds(lambda: records, descendant_watch(records))
    except Stop as e:
        raise Unresolved(f"the program cannot follow the probe-owned processes while it ends them ({e}); the "
                         "owner ends each recorded process through its owning interface before restore") from e
    for r in outcome["signalled"]:
        log(f"  sent {r['signal']} to probe-owned process {r['pid']} ({r['executable']}, HOME {r['home']})")
    for r in outcome["remaining"]:
        log(f"  remaining probe-owned process {r['pid']} ({r['executable']}, HOME {r['home']}): {r['state']}")
    if outcome["remaining"]:
        raise Unresolved("a probe-owned process still runs after the probes and their cleanup; the owner ends "
                         "it through its owning interface before restore")
    log(f"probe-owned processes: {len(records)} recorded, none runs")


def recheck_reader_list(layout: Layout, readers: list) -> dict:
    """Read each version directory of the reader evidence again: present, without orphan marker, registered."""
    registry = load_json_file(layout.path(INSTALLED_PLUGINS)) or {}
    plugins = registry.get("plugins") if isinstance(registry.get("plugins"), dict) else {}
    out, problems = [], []
    for reader in readers:
        path = reader.get("path")
        if not isinstance(path, str):
            continue
        registered = any(isinstance(e, dict) and e.get("installPath") == path
                         for e in plugins.get(reader.get("plugin")) or [])
        orphaned = os.path.lexists(os.path.join(path, ".orphaned_at"))
        present = os.path.isdir(path)
        out.append({"plugin": reader.get("plugin"), "path": path, "present": present, "orphaned": orphaned,
                    "registered": registered})
        if not present:
            problems.append(f"the version directory {path} of {reader.get('plugin')} is missing")
        elif orphaned:
            problems.append(f"the version directory {path} of {reader.get('plugin')} holds .orphaned_at")
        elif not registered:
            problems.append(f"the version directory {path} of {reader.get('plugin')} lost its registration")
    return {"readers": out, "problems": problems}


def gate_result(results: list) -> dict:
    found = [r.get("result") for r in results if isinstance(r.get("result"), dict)]
    return found[0] if found else {}


# --- the switch ---------------------------------------------------------------------

def receipt_path(layout: Layout, attempt: str, suffix: str = "") -> str:
    return os.path.join(layout.state, "receipts", f"{attempt}{suffix}.json")


def restored_path(layout: Layout, attempt: str) -> str:
    return os.path.join(layout.state, "restored", f"{attempt}.json")


def append_verified(layout: Layout, text: str, what: str) -> None:
    """Append TEXT to the maintained manual, then read it back. The manual is never rewritten."""
    hook(layout, f"manual:{what}")
    append_manual(layout, text, what)
    try:
        with open(layout.manual, encoding="utf-8") as f:
            ok = f.read().endswith(text)
    except OSError as e:
        raise Stop(f"the manual could not be read back after the {what}: {e.strerror}")
    if not ok:
        raise Stop(f"the manual does not end with the {what} after the append")
    log(f"manual: appended the {what}")


def manual_marker(attempt: str, what: str) -> str:
    return f"<!-- owner-switch-attempt {attempt} {what} -->"


def manual_text(layout: Layout) -> str:
    try:
        with open(layout.manual, encoding="utf-8") as f:
            return f.read()
    except OSError:
        return ""


def preflight(layout: Layout, plan: dict, note: dict, args) -> dict:
    """Steps 1 to 3: no switch-set write happens here."""
    problems = []
    if note["kind"] != "switch":
        problems.append(f"the attempt note is a {note['kind']} note, not a switch note")
    if note["home"] != layout.home:
        problems.append(f"the attempt note names HOME {note['home']}, not {layout.home}")
    if (note.get("plan") or {}).get("digest") != plan["_digest"]:
        problems.append("the attempt note names another plan")
    if (note.get("program") or {}).get("digest") != PROGRAM_DIGEST:
        problems.append("the attempt note names another program")
    if note["backup"] != os.path.join(layout.state, "backups", note["attempt"]):
        problems.append(f"the attempt note names the backup {note['backup']}, outside the plan's state")
    if os.path.lexists(note["backup"]):
        problems.append(f"the backup {note['backup']} of this attempt exists: a new attempt needs a new note, "
                        f"and an earlier interrupted attempt needs its restore first")
    for p in (note["_path"], note["backup"]):
        why = placement_problem(layout, plan, p)
        if why:
            problems.append(f"{p}: {why}")
    if problems:
        raise Stop("the attempt note does not fit this switch: " + "; ".join(problems))
    rows = input_rows(plan)
    differ = [f"{r['role']} {r['path']}" for r in rows if r["digest"] != r["planned"]]
    if differ:
        raise Stop("command inputs differ from the plan: " + ", ".join(differ))
    tools = observed_tools(layout)
    if tools["claude"] != plan["tools"]["claude"] or tools["claude"] != (note.get("tools") or {}).get("claude"):
        raise Stop(f"claude --version printed {tools['claude']!r}; the plan holds {plan['tools']['claude']!r} and "
                   f"the attempt note {(note.get('tools') or {}).get('claude')!r}")
    operator = process_gate(layout, "the switch")
    changed = original_changes(layout, plan)
    if changed:
        raise Stop("planned originals differ from the plan: " + ", ".join(changed))
    changed = inventory_changes(layout, plan["inventory"]["before"])
    if changed:
        raise Stop("the inventory differs from the plan's expected inventory: " + ", ".join(changed))
    if not same(ravel_skills(layout), OFF):
        raise Stop(f"{RAVEL_SETTINGS} does not hold the off value; run native-off first: {note.get('native_off')}")
    expected = marketplace_path(layout)
    hooks = hooks_state(layout, expected)
    if hooks["kind"] == "other":
        raise Stop(f"the hooks installation found that {hooks['reason']}")
    if hooks["kind"] == "absent":
        lost = lost_metadata(layout)
        if lost:
            raise Stop(f"{lost}, which Claude Code's commands would drop")
    rows_changed = baseline_rows(layout, plan)
    if rows_changed:
        raise Stop("the baseline instrument reports changed entries: " + "; ".join(rows_changed))
    evidence, admitted = {}, {}
    for step in ("4", "5", "6", "7"):
        results, failed = run_commands(layout, plan, plan["gates"][step], step, admitted)
        evidence[step] = results
        if failed:
            raise Stop(f"step {step} failed: " + ", ".join(f["name"] for f in failed))
        if step == "4":
            admitted = admitted_of(results)
    guidance = []
    for p in args.guidance or []:
        digest = file_digest(p)
        if digest is None:
            raise Stop(f"the operator's guidance {p} cannot be read")
        guidance.append({"path": os.path.abspath(p), "digest": digest})
    return {"evidence": evidence, "admitted": admitted, "inputs": rows, "tools": tools, "marketplace": expected,
            "guidance": guidance, "operator": [describe(i) for i in operator["admitted"]]}


def deactivations(plan: dict) -> list:
    return [w["path"] for w in plan["writes"] if w["type"] == "entry"]


def switched_cells(plan: dict) -> list:
    """The values that each file write sets, in plan order, with the Ravel settings last."""
    files = [w for w in plan["writes"] if w["type"] == "file" and w.get("set")]
    files.sort(key=lambda w: w["path"] == RAVEL_SETTINGS)
    return [(w["path"], [(v["key"], v["value"]) for v in w["set"]]) for w in files]


def superseded_plugins(plan: dict) -> list:
    """The plugins that the plan disables in settings.json."""
    return sorted({v["key"][1] for w in plan["writes"] if w["path"] == CLAUDE_SETTINGS for v in w.get("set", [])
                   if len(v["key"]) == 2 and v["key"][0] == "enabledPlugins" and v["value"] is False})


def final_comparison(layout: Layout, plan: dict, ctx: dict) -> dict:
    assertions = assertion_results(layout, plan["assertions"])
    inventory = inventory_changes(layout, plan["inventory"]["after"])
    readers = recheck_reader_list(layout, gate_result(ctx["evidence"]["7"]).get("readers") or [])
    guidance = [g["path"] for g in ctx["guidance"] if file_digest(g["path"]) != g["digest"]]
    problems = assertion_problems(assertions)
    problems += [f"the inventory changed at {rel}" for rel in inventory]
    problems += readers["problems"]
    problems += [f"the operator's guidance {p} changed or cannot be read" for p in guidance]
    return {"assertions": assertions, "inventory": inventory, "readers": readers, "problems": problems}


def retained_resources(layout: Layout, plan: dict, note: dict, ctx: dict) -> list:
    purge = list(SERIAL_PURGE)
    items = [{"item": "backup", "path": note["backup"], "condition": purge},
             {"item": "attempt note", "path": note["_path"], "condition": purge}]
    for reader in gate_result(ctx["evidence"]["7"]).get("readers") or []:
        if isinstance(reader.get("path"), str):
            items.append({"item": "frozen cache version", "plugin": reader.get("plugin"), "path": reader["path"],
                          "condition": purge})
    superseded = superseded_plugins(plan)
    for pid in superseded:
        items.append({"item": "superseded plugin registration", "plugin": pid, "condition": purge})
    for market in sorted({pid.split("@", 1)[1] for pid in superseded if "@" in pid}):
        items.append({"item": "earlier marketplace registration", "marketplace": market, "condition": purge})
    for sid, source in sorted((gate_result(ctx["evidence"]["6"]).get("sources") or {}).items()):
        items.append({"item": "retained source", "source": sid, "path": source.get("originPath"),
                      "resourceDigest": source.get("resourceDigest"), "condition": purge})
    items.append({"item": "Grove's Codex snapshot store", "path": layout.path(CODEX_SNAPSHOT_STORE),
                  "condition": purge})
    staging = staging_of(plan)
    after = ["this receipt is written"]
    items.append({"item": "staging HOME", "path": staging["home"], "condition": after})
    items.append({"item": "candidate keg", "path": staging["candidate_keg"], "condition": after})
    items += [dict(row) for row in plan.get("retained", [])]
    # The purge report compares these identities ("Purge"). A resource that
    # may go once this receipt is written needs none.
    for item in items:
        if item["condition"] != after:
            item["identity"] = retained_item_identity(layout, item)
    return items


def switch_receipt(layout: Layout, plan: dict, note: dict, ctx: dict, probes: list, mac: dict,
                   comparison: dict, identities: dict) -> dict:
    step4, step5, step6 = (gate_result(ctx["evidence"][s]) for s in ("4", "5", "6"))
    return {
        "schema": RECEIPT_SCHEMA, "attempt": note["attempt"], "home": layout.home, "at": now(),
        "operator": ctx["operator"],
        "activated": {"grove": step4.get("grove"),
                      "ravel": {"path": step6.get("ravel_path"), "version": step6.get("ravel")},
                      "formula_versions": step5.get("formulae") or {}},
        "sources": step6.get("sources") or {},
        "plan": {"path": plan["_path"], "digest": plan["_digest"]},
        "program_digest": PROGRAM_DIGEST, "inputs": ctx["inputs"], "tools": ctx["tools"],
        "probes": probes, "mac_check": mac,
        "readers": {"recorded": gate_result(ctx["evidence"]["7"]).get("readers") or [],
                    "again": comparison["readers"]["readers"]},
        "guidance": ctx["guidance"],
        # Delivery assertions govern current health; restore identities govern
        # later whole-file restore eligibility ("Current health and restore eligibility").
        "delivery_assertions": comparison["assertions"],
        "restore_identities": identities,
        "attempt_note": note["_path"], "backup": note["backup"], "restore_command": note["restore"],
        "retained": retained_resources(layout, plan, note, ctx),
    }


def apply_switch(layout: Layout, plan: dict, note: dict, backup: dict, ctx: dict) -> dict:
    """Steps 4 (its end) to 9, after the backup's publication."""
    backup_published(layout, backup)
    process_gate(layout, "the first switch write")
    changed = original_changes(layout, plan)
    if changed:
        raise Stop("planned originals changed after the backup: " + ", ".join(changed))
    if not same(ravel_skills(layout), OFF):
        raise Stop(f"{RAVEL_SETTINGS} lost the off value before the hooks installation")
    hooks_installation(layout, ctx["marketplace"])
    discard = os.path.join(layout.state, "discard", note["attempt"])
    for rel in deactivations(plan):
        hook(layout, f"deactivate:{rel}")
        if os.path.lexists(layout.path(rel)):
            discard_entry(layout.path(rel), discard)
        log(f"deactivated: {rel}")
    for rel, cells in switched_cells(plan):
        write_cells(layout, rel, cells)
    hook(layout, "probes")
    probes, failed = run_probes(layout, plan, plan["probes"], "7")
    end_probe_services(layout, probe_owned(probes))
    if failed:
        raise Stop("probe failed: " + ", ".join(f["name"] for f in failed))
    process_gate(layout, "the final comparison")
    mac = run_command(layout, plan["mac_check"], "8", plan)
    if mac["exit"] != 0:
        raise Stop(f"the switch area of {plan['mac_check']['name']} did not pass")
    again, failed = run_commands(layout, plan, plan["gates"]["6"], "8", ctx["admitted"])
    if failed:
        raise Stop("the sources no longer resolve with their accepted digests")
    comparison = final_comparison(layout, plan, ctx)
    if comparison["problems"]:
        raise Stop("the final comparison failed: " + "; ".join(comparison["problems"]))
    identities = {rel: restore_identity(layout.path(rel)) for rel in plan["restore"]}
    receipt = switch_receipt(layout, plan, note, ctx, probes, mac, comparison, identities)
    path = receipt_path(layout, note["attempt"])
    os.makedirs(os.path.dirname(path), 0o700, exist_ok=True)
    data = dump_json(receipt)
    write_exclusive(path, data)
    hook(layout, "receipt:written")
    if file_digest(path) != hashlib.sha256(data).hexdigest():
        raise Stop(f"the receipt {path} does not hold the written bytes")
    log(f"receipt: {path}")
    append_verified(layout, (
        f"\n{manual_marker(note['attempt'], 'switched')}\n"
        f"## Owner skill switch {note['attempt']}\n\n"
        f"Ravel delivers the skills since {receipt['at']}. The receipt is `{path}`, and the backup is "
        f"`{note['backup']}`. This observation holds only while `mac-check` verifies the switched layout. "
        f"Before purge, run the purge report.\n\n"
        f"Restore, which refuses when any restore destination has changed since this switch:\n\n"
        f"```sh\n{note['restore']}\n```\n"), "success observation")
    hook(layout, "switch:final")
    final = final_comparison(layout, plan, ctx)
    drift = [rel for rel in plan["restore"] if restore_identity(layout.path(rel)) != identities[rel]]
    if final["problems"] or drift:
        raise Stop("the final layout changed after the receipt: " + "; ".join(final["problems"] + drift))
    return receipt


def switch_command(args) -> int:
    plan = load_plan(args.plan)
    layout = Layout(args.home, plan)
    note = load_note(args.attempt)
    log(f"program: {PROGRAM_DIGEST}")
    log(f"plan: {plan['_digest']}")
    log(f"attempt: {note['attempt']} ({note['_path']})")
    hook(layout, "switch:start")
    try:
        ctx = preflight(layout, plan, note, args)
        backup = create_backup(layout, plan, note, {s: ctx["evidence"][s] for s in ("4", "5", "6", "7")})
    except Stop as e:
        log(f"stop: {e}")
        log("result: refused; no switch-set write happened, and native delivery stays")
        return EXIT_REFUSED
    try:
        apply_switch(layout, plan, note, backup, ctx)
    except Unresolved as e:
        log(f"recovery incomplete: {e}")
        log(f"the backup {note['backup']} stays. After the owner resolves it, run: {note['restore']}")
        return EXIT_INCOMPLETE
    except (Stop, Conflict, Incomplete, OSError) as e:
        log(f"failure: {e}")
        invalidate_receipt(layout, note["attempt"], f"the switch failed: {e}")
        code = restore_from(layout, backup, note, f"the switch failed: {e}", ctx["operator"], ctx["guidance"])
        if code == 0:
            log("result: failed; restore verified native delivery")
            return EXIT_RESTORED
        log(f"result: failed; recovery incomplete. Repeat: {note['restore']}")
        return EXIT_INCOMPLETE
    log("result: switched")
    return 0


# --- restore ------------------------------------------------------------------------

def invalidate_receipt(layout: Layout, attempt: str, reason: str) -> None:
    path = receipt_path(layout, attempt)
    if os.path.lexists(path):
        os.rename(path, receipt_path(layout, attempt, ".invalid"))
        log(f"receipt: invalidated {path}: {reason}")


def check_backup_note(backup: dict, note: dict, home: str) -> None:
    problems = []
    if backup["attempt"] != note["attempt"]:
        problems.append(f"the backup belongs to attempt {backup['attempt']}, the note to {note['attempt']}")
    if backup["note"]["digest"] != note["_digest"]:
        problems.append("the backup records another attempt note")
    if backup["plan"]["digest"] != (note.get("plan") or {}).get("digest"):
        problems.append("the backup holds another plan than the attempt note names")
    if backup.get("kind") != note["kind"]:
        problems.append(f"the backup is a {backup.get('kind')} backup, and the note a {note['kind']} note")
    if backup["home"] != home or note["home"] != home:
        problems.append(f"the backup names HOME {backup['home']} and the note {note['home']}, not {home}")
    if os.path.realpath(note["backup"]) != backup["path"]:
        problems.append(f"the note names the backup {note['backup']}, not {backup['path']}")
    if problems:
        raise Stop("the backup does not match its attempt note and HOME: " + "; ".join(problems))


def ravel_restored(layout: Layout, backup: dict, item: dict) -> bool:
    """The Ravel settings equal the backed-up file, except for the enforced off value."""
    if restore_identity(layout.path(item["path"])) == item["identity"]:
        return True
    try:
        current = strict_file(layout.path(item["path"]))
    except Stop:
        return False
    wanted = strict_file(os.path.join(backup["path"], item["data"])) if item["data"] else {}
    return same(current, dict(wanted, skills=OFF))


def restore_item(layout: Layout, backup: dict, item: dict, discard: str) -> None:
    """Restore one whole file or entry, or its absence."""
    path = layout.path(item["path"])
    hook(layout, f"restore:{item['path']}")
    os.makedirs(discard, 0o700, exist_ok=True)
    if item["data"] is None:
        if os.path.lexists(path):
            discard_entry(path, discard)
        return
    build = os.path.join(discard, f"build-{secrets.token_hex(6)}")
    copy_tree(os.path.join(backup["path"], item["data"]), build)
    if canon(nodes_of(build)) != canon(item["nodes"]):
        raise Incomplete(f"the restored copy of {item['path']} differs from the backup")
    os.makedirs(os.path.dirname(path), exist_ok=True)
    if os.path.lexists(path) and (item["type"] == "entry" or (os.path.isdir(path) and not os.path.islink(path))):
        discard_entry(path, discard)
    os.rename(build, path)
    dfd = os.open(os.path.dirname(path), os.O_RDONLY)
    try:
        full_sync(dfd)
    finally:
        os.close(dfd)


def remove_write_temps(layout: Layout, rels: list) -> None:
    """Remove each temporary sibling that an interrupted ordinary write left."""
    for folder in sorted({os.path.dirname(layout.path(r)) for r in rels}):
        try:
            names = os.listdir(folder)
        except OSError:
            continue
        for name in names:
            if WRITE_TEMP in name and name.startswith("."):
                os.unlink(os.path.join(folder, name))
                log(f"restore: removed the interrupted temporary file {os.path.join(folder, name)}")


def restore_from(layout: Layout, backup: dict, note: dict, reason: str, operator=(), guidance=()) -> int:
    """Restore each destination whole, enforce the off value, verify, run the native probes and append.

    OPERATOR names the admitted ancestors of the restoring process, and
    GUIDANCE each file that the operator read, by path and digest. The
    restored record keeps both, so a restore after finish names its own
    operator ("Resuming the switch").
    """
    plan = backup["plan_doc"]
    items = {item["path"]: item for item in backup["items"]}
    discard = os.path.join(layout.state, "discard", backup["attempt"])
    remove_write_temps(layout, plan["restore"])
    if os.path.isdir(discard):
        erase_tree(discard)
    failed = []
    for rel in plan["restore"]:
        try:
            restore_item(layout, backup, items[rel], discard)
            if rel == RAVEL_SETTINGS:
                native_off(layout.home)
            log(f"restored: {rel}")
        except (Stop, Incomplete, OSError) as e:
            failed.append(f"{rel}: {e}")
    for rel in plan["restore"]:
        item = items[rel]
        try:
            ok = ravel_restored(layout, backup, item) if rel == RAVEL_SETTINGS else \
                restore_identity(layout.path(rel)) == item["identity"]
        except Stop as e:
            ok = False
            failed.append(f"{rel}: {e}")
        if not ok and not any(f.startswith(rel + ":") for f in failed):
            failed.append(f"{rel}: differs from the backup after restore")
    try:
        if not same(ravel_skills(layout), OFF):
            failed.append(f"{RAVEL_SETTINGS}: does not hold the off value")
    except Stop as e:
        failed.append(f"{RAVEL_SETTINGS}: {e}")
    if failed:
        for line in failed:
            log(f"  failed path {line}")
        log(f"restore: incomplete; the backup {backup['path']} stays")
        return EXIT_INCOMPLETE
    try:
        hook(layout, "restore:native-probes")
        probes, probe_failed = run_probes(layout, plan, plan["native_probe"], "restore")
        end_probe_services(layout, probe_owned(probes))
        process_gate(layout, "the end of restore")
    except (Stop, Unresolved) as e:
        log(f"restore: the native probes are incomplete: {e}")
        return EXIT_INCOMPLETE
    if probe_failed:
        log("restore: native probe failed: " + ", ".join(f["name"] for f in probe_failed))
        return EXIT_INCOMPLETE
    unread = [g["path"] for g in guidance if file_digest(g["path"]) != g["digest"]]
    if unread:
        log("restore: the operator's guidance changed or cannot be read: " + ", ".join(unread))
        return EXIT_INCOMPLETE
    invalidate_receipt(layout, backup["attempt"], reason)
    marker = restored_path(layout, backup["attempt"])
    os.makedirs(os.path.dirname(marker), 0o700, exist_ok=True)
    if not os.path.lexists(marker):
        write_exclusive(marker, dump_json({"attempt": backup["attempt"], "backup": backup["path"], "at": now(),
                                           "reason": reason, "operator": list(operator),
                                           "guidance": list(guidance)}))
    code = append_restore_entries(layout, backup, reason)
    if code == 0:
        log("restore: native delivery verified")
    return code


def append_restore_entries(layout: Layout, backup: dict, reason: str) -> int:
    """Append each manual entry of this restore that the manual lacks, found by its marker.

    A restore whose append failed has its restored record already, so a
    repeated restore appends the pending entries here ("The switch inside
    finish": the pending correction stays an explicit obligation).
    """
    attempt, text = backup["attempt"], manual_text(layout)
    entries = []
    if manual_marker(attempt, "switched") in text and manual_marker(attempt, "correction") not in text:
        entries.append(("correction", f"\n{manual_marker(attempt, 'correction')}\n"
                                      f"The success observation of owner skill switch {attempt} no "
                                      f"longer holds: {reason}.\n"))
    if manual_marker(attempt, "restored") not in text:
        entries.append(("native return", f"\n{manual_marker(attempt, 'restored')}\n"
                                         f"## Native skill delivery restored from {attempt}\n\n"
                                         f"Restore from `{backup['path']}` verified native delivery at {now()}: "
                                         f"{reason}. The record `{restored_path(layout, attempt)}` names the "
                                         f"operator.\n"))
    try:
        for what, entry in entries:
            append_verified(layout, entry, what)
    except Stop as e:
        log(f"restore: the configuration is restored and verified, but the manual entry is pending: {e}")
        return EXIT_INCOMPLETE
    return 0


def restore_eligibility(layout: Layout, plan: dict, backup: dict, recovery) -> str:
    """Decide which restore runs: immediate, later, retry or done. A refusal writes nothing."""
    items = {item["path"]: item for item in backup["items"]}
    if recovery is not None:
        rbackup = recovery["backup_doc"]
        ritems = {item["path"]: item for item in rbackup["items"]}
        differ = []
        for rel in plan["restore"]:
            current = restore_identity(layout.path(rel))
            if current in (ritems[rel]["identity"], items[rel]["identity"]):
                continue
            if rel == RAVEL_SETTINGS and ravel_restored(layout, backup, items[rel]):
                continue
            differ.append(rel)
        if differ:
            raise Stop("the interrupted restore cannot be retried: these paths match neither its recovery backup "
                       "nor the restored layout: " + ", ".join(differ))
        return "retry"
    receipt = load_json_file(receipt_path(layout, backup["attempt"]))
    if receipt is not None:
        wanted = receipt.get("restore_identities") or {}
        differ = [rel for rel in plan["restore"] if restore_identity(layout.path(rel)) != wanted.get(rel)]
        if differ:
            raise Stop("these restore destinations differ from the receipt's post-switch identity, so a whole-file "
                       "restore would discard their changes; review them and prepare a revised native return: "
                       + ", ".join(differ))
        return "later"
    if os.path.lexists(restored_path(layout, backup["attempt"])):
        differ = [rel for rel in plan["restore"]
                  if not (ravel_restored(layout, backup, items[rel]) if rel == RAVEL_SETTINGS
                          else restore_identity(layout.path(rel)) == items[rel]["identity"])]
        if differ:
            raise Stop("this backup was restored already, and these paths changed since: " + ", ".join(differ))
        return "done"
    return "immediate"


def recovery_backup(layout: Layout, plan: dict, backup: dict, note: dict) -> dict:
    """Before an eligible later restore writes: keep the compared layout in a new recovery backup and its note."""
    plan = backup["plan_doc"]
    attempt = new_attempt_id()
    rbackup = os.path.join(layout.state, "backups", attempt)
    rnote_path = os.path.join(layout.state, "attempts", attempt + ".json")
    rnote = {
        "schema": ATTEMPT_SCHEMA, "kind": "recovery", "attempt": attempt, "of": backup["attempt"],
        "home": layout.home, "plan": {"path": plan["_path"], "digest": plan["_digest"]},
        "program": {"path": PROGRAM_PATH, "digest": PROGRAM_DIGEST}, "backup": rbackup,
        "restore": shlex.join(program_argv("restore", "--backup", backup["path"], "--attempt", note["_path"],
                                           "--home", layout.home, "--recovery", rnote_path)),
        "created": now(),
    }
    write_note(layout, plan, rnote)
    rnote = load_note(rnote_path)
    log(f"recovery note: {rnote_path}")
    log(f"if this restore is interrupted, repeat under the same quiet interval: {rnote['restore']}")
    backup_published(layout, create_backup(layout, plan, rnote, {}))
    return rnote


def restore_command(args) -> int:
    home = os.path.realpath(args.home)
    try:
        backup = load_backup(args.backup)
        note = load_note(args.attempt)
        check_backup_note(backup, note, home)
        plan = backup["plan_doc"]
        layout = Layout(home, plan)
        recovery = None
        if args.recovery:
            recovery = load_note(args.recovery)
            if recovery["kind"] != "recovery" or recovery.get("of") != backup["attempt"] or recovery["home"] != home:
                raise Stop(f"{args.recovery} is not the recovery note of attempt {backup['attempt']} in {home}")
            recovery["backup_doc"] = load_backup(recovery["backup"])
            check_backup_note(recovery["backup_doc"], recovery, home)
        log(f"restore: attempt {backup['attempt']} from {backup['path']}")
        hook(layout, "restore:start")
        report = process_gate(layout, "restore")
        operator = [describe(i) for i in report["admitted"]]
        guidance = []
        for p in args.guidance or []:
            digest = file_digest(p)
            if digest is None:
                raise Stop(f"the operator's guidance {p} cannot be read")
            guidance.append({"path": os.path.abspath(p), "digest": digest})
        mode = restore_eligibility(layout, plan, backup, recovery)
        log(f"restore: {mode}")
        if mode == "done":
            # Only a manual entry that an earlier restore of this backup could not append remains.
            done = load_json_file(restored_path(layout, backup["attempt"])) or {}
            code = append_restore_entries(layout, backup, done.get("reason") or "the owner restored")
            log("result: restored already; no restore destination written" if code == 0
                else f"result: the manual entry is still pending. Repeat: {args_command(args)}")
            return code
        if mode == "later":
            recovery_backup(layout, plan, backup, note)
    except Stop as e:
        log(f"stop: {e}")
        log("result: refused; restore wrote nothing")
        return EXIT_REFUSED
    reason = {"immediate": "the owner restored after a failed or interrupted switch",
              "later": "the owner returned to native delivery",
              "retry": "the owner repeated an interrupted restore"}[mode]
    code = restore_from(layout, backup, note, reason, operator, guidance)
    log("result: restored" if code == 0 else f"result: recovery incomplete. Repeat: {args_command(args)}")
    return code


def args_command(args) -> str:
    argv = ["restore", "--backup", args.backup, "--attempt", args.attempt, "--home", os.path.realpath(args.home)]
    if args.recovery:
        argv += ["--recovery", args.recovery]
    for p in args.guidance or []:
        argv += ["--guidance", p]
    return shlex.join(program_argv(*argv))

# --- status and the purge report ----------------------------------------------------
# docs/specs/owner-skill-delivery.md, "Current health and restore eligibility" and
# "Purge". Both entries read the switch's records and the layout, and write nothing.

# Claude Code's documented reader markers in a cache version directory ("Resource lifetime").
READER_MARKERS = ".in_use"
# The purge conditions that the program itself establishes; the owner establishes the others.
PROGRAM_CONDITIONS = ("each retained resource keeps its recorded identity", "this receipt is written")


def registration_identity(layout: Layout, rel: str, key: str) -> str:
    """The digest of one registration's JSON entry in a Claude Code registry file."""
    doc = load_json_file(layout.path(rel))
    if doc is not None and rel == INSTALLED_PLUGINS:
        doc = doc.get("plugins") if isinstance(doc.get("plugins"), dict) else None
    value = UNREADABLE if doc is None else doc.get(key, ABSENT)
    return hashlib.sha256(canon(value).encode()).hexdigest()


def retained_item_identity(layout: Layout, item: dict):
    """The identity of one retained resource, or None for a resource without a path or registration.

    A path's identity is its change identity without the top-level reader
    markers, which a live reader adds and removes ("Inventory controls").
    """
    if item["item"] == "superseded plugin registration":
        return registration_identity(layout, INSTALLED_PLUGINS, item["plugin"])
    if item["item"] == "earlier marketplace registration":
        return registration_identity(layout, KNOWN_MARKETPLACES, item["marketplace"])
    if isinstance(item.get("path"), str):
        try:
            return identity(item["path"], skip=(READER_MARKERS,)) or "absent"
        except OSError as e:
            # An unreadable resource cannot keep its identity, so the comparison fails.
            return f"unreadable: {e.strerror}"
    return None


def json_names(folder: str, suffix: str = ".json") -> list:
    try:
        return sorted(n for n in os.listdir(folder) if n.endswith(suffix) and not n.startswith("."))
    except FileNotFoundError:
        return []


def switch_records(layout: Layout) -> dict:
    """Every record of the switch in the state folder, and each record without its attempt note."""
    notes, recoveries, problems = {}, [], []
    for name in json_names(os.path.join(layout.state, "attempts")):
        try:
            note = load_note(os.path.join(layout.state, "attempts", name))
        except Stop as e:
            problems.append(str(e))
            continue
        if note["kind"] == "recovery":
            recoveries.append(note)
        else:
            notes[note["attempt"]] = note
    receipts = {n[:-len(".json")] for n in json_names(os.path.join(layout.state, "receipts"))
                if not n.endswith(".invalid.json")}
    restored = {n[:-len(".json")] for n in json_names(os.path.join(layout.state, "restored"))}
    for attempt in sorted((receipts | restored) - set(notes)):
        problems.append(f"the receipt or restored record of attempt {attempt} has no attempt note")
    known = {os.path.realpath(n["backup"]) for n in list(notes.values()) + recoveries}
    for name in sorted(os.listdir(layout.backups)) if os.path.isdir(layout.backups) else []:
        path = os.path.join(layout.backups, name)
        manifest = load_json_file(os.path.join(path, "manifest.json"))
        if (not name.startswith(".") and manifest is not None and manifest.get("schema") == BACKUP_SCHEMA
                and os.path.realpath(path) not in known):
            problems.append(f"the backup {path} has no attempt note")
    attempts = []
    for attempt, note in sorted(notes.items()):
        if attempt in restored:
            st = "restored"
        elif attempt in receipts:
            st = "switched"
        elif os.path.isdir(note["backup"]):
            st = "open"
        else:
            st = "unused"
        attempts.append({"attempt": attempt, "note": note, "state": st})
    return {"attempts": attempts, "recoveries": recoveries, "restored": restored, "problems": problems}


def running_switch(note: dict) -> bool:
    """Whether an ancestor of this process is the switch of NOTE, as at its step 8.

    A crashed switch is no ancestor, so its layout never counts as a switch
    in progress.
    """
    try:
        procs = process_table()
    except Stop:
        return False
    for pid in ancestors_of(procs, os.getpid()):
        _, argv, _ = process_arguments(pid)
        if not argv or "switch" not in argv or "--attempt" not in argv[:-1]:
            continue
        given = argv[argv.index("--attempt") + 1]
        if os.path.realpath(os.path.join(process_folder(pid) or "/", given)) == note["_path"]:
            return True
    return False


def source_problems(layout: Layout, expected: dict) -> list:
    """Resolve the current Ravel skills value in a disposable HOME and compare each source with EXPECTED."""
    try:
        skills = ravel_skills(layout)
        env = command_env(layout, "status")
        result = tool(["brew", "--prefix"], env)
        prefix = result.stdout.strip()
        if result.returncode != 0 or not prefix:
            raise Stop(f"brew --prefix failed: {(result.stdout + result.stderr).strip()[-500:]}")
        got = resolve(prefix, {} if same(skills, ABSENT) else {"skills": skills}, env)["sources"]
    except Stop as e:
        return [f"the sources cannot be resolved: {e}"]
    problems = []
    for sid in sorted(set(got) | set(expected)):
        if sid not in got:
            problems.append(f"the source {sid} no longer resolves")
        elif sid not in expected:
            problems.append(f"the source {sid} resolves, and the switch accepted no such source")
        elif got[sid]["resourceDigest"] != (expected[sid] or {}).get("resourceDigest"):
            problems.append(f"the source {sid} gives {got[sid]['resourceDigest']}, not its switched "
                            f"{(expected[sid] or {}).get('resourceDigest')}")
    return problems


def switched_problems(layout: Layout, plan: dict, readers: list, sources: dict) -> list:
    """Delivery health of a switched layout: assertions, inventory, reader evidence and sources."""
    problems = assertion_problems(assertion_results(layout, plan["assertions"]))
    problems += [f"the inventory differs from the switched inventory at {rel}"
                 for rel in inventory_changes(layout, plan["inventory"]["after"])]
    problems += recheck_reader_list(layout, readers)["problems"]
    return problems + source_problems(layout, sources)


def native_problems(layout: Layout, plan: dict) -> list:
    problems = assertion_problems(assertion_results(layout, plan["native_assertions"]))
    return problems + [f"the inventory differs from the native inventory at {rel}"
                       for rel in inventory_changes(layout, plan["inventory"]["before"])]


def pending_manual(layout: Layout, attempt: str) -> bool:
    """Whether the manual lacks an entry of a completed restore of ATTEMPT."""
    text = manual_text(layout)
    return (manual_marker(attempt, "restored") not in text
            or (manual_marker(attempt, "switched") in text and manual_marker(attempt, "correction") not in text))


def status_report(layout: Layout, plan: dict) -> dict:
    """Delivery health, restore eligibility and pending manual entries ("Current health and restore eligibility").

    Health is native, switched, restore or review. A receipt names what to
    verify, and never stands for the current layout.
    """
    records = switch_records(layout)
    problems = list(records["problems"])
    restore = {"status": "none", "differing": [], "commands": []}
    pending = [{"attempt": a["attempt"], "command": a["note"]["restore"]} for a in records["attempts"]
               if a["state"] == "restored" and pending_manual(layout, a["attempt"])]
    live = [a for a in records["attempts"] if a["state"] != "unused"]
    governing = live[-1] if live else None
    report = {"health": "review", "attempt": governing["attempt"] if governing else None, "receipt": None,
              "plan_digest": plan["_digest"], "problems": problems, "restore": restore, "pending_manual": pending}
    for a in live[:-1]:
        if a["state"] == "switched":
            problems.append(f"the receipt of the earlier attempt {a['attempt']} is still valid")
    waiting = [a for a in live if a["state"] == "open"]
    retries = [r for r in records["recoveries"]
               if r.get("of") not in records["restored"] and os.path.isdir(r["backup"])]
    if governing and governing["state"] == "open" and running_switch(governing["note"]) and not retries:
        # The switch runs `mac-check` at step 8, before its receipt exists.
        waiting = waiting[:-1]
        restore["status"] = "in-progress"
        try:
            backup = load_backup(governing["note"]["backup"])
        except Stop as e:
            problems.append(str(e))
            backup = None
        if backup is not None:
            if backup["plan"]["digest"] != plan["_digest"]:
                problems.append("the running attempt's plan is not the prepared plan")
            else:
                problems += switched_problems(layout, plan, gate_result(backup["evidence"]["7"]).get("readers") or [],
                                              gate_result(backup["evidence"]["6"]).get("sources") or {})
        if not problems and not waiting:
            report["health"] = "switched"
        return report
    if waiting or retries:
        report["health"] = "restore"
        restore["status"] = "required"
        restore["commands"] = [a["note"]["restore"] for a in waiting] + [r["restore"] for r in retries]
        return report
    if problems:
        return report
    if governing and governing["state"] == "switched":
        note = governing["note"]
        receipt = load_json_file(receipt_path(layout, note["attempt"]))
        if receipt is None or receipt.get("schema") != RECEIPT_SCHEMA:
            problems.append(f"the receipt of attempt {note['attempt']} cannot be read")
            return report
        report["receipt"] = receipt
        if (receipt.get("plan") or {}).get("digest") != plan["_digest"]:
            problems.append("the prepared plan is not the plan of the receipt, so its evidence is stale")
            return report
        problems += switched_problems(layout, plan, (receipt.get("readers") or {}).get("recorded") or [],
                                      receipt.get("sources") or {})
        if not problems:
            report["health"] = "switched"
        wanted = receipt.get("restore_identities") or {}
        for rel in plan["restore"]:
            try:
                if restore_identity(layout.path(rel)) != wanted.get(rel):
                    restore["differing"].append(rel)
            except Stop:
                restore["differing"].append(rel)
        try:
            check_backup_note(load_backup(note["backup"]), note, layout.home)
            restore["status"] = "ineligible" if restore["differing"] else "eligible"
        except Stop as e:
            restore["status"] = "unavailable"
            restore["reason"] = str(e)
        restore["commands"] = [note["restore"]]
        return report
    problems += native_problems(layout, plan)
    if not problems:
        report["health"] = "native"
    return report


def status_command(args) -> int:
    plan = load_plan(args.plan)
    layout = Layout(args.home, plan)
    if args.json:
        # Only the JSON object reaches standard output; each log line of a resolution goes to standard error.
        with contextlib.redirect_stdout(sys.stderr):
            report = status_report(layout, plan)
        out = dict(report, receipt=receipt_path(layout, report["attempt"]) if report["receipt"] else None)
        print(json.dumps(out, indent=2, sort_keys=True))
        return 0
    report = status_report(layout, plan)
    log(f"health: {report['health']}" + (f" (attempt {report['attempt']})" if report["attempt"] else ""))
    for p in report["problems"]:
        log(f"  {p}")
    restore = report["restore"]
    log(f"restore: {restore['status']}")
    for rel in restore["differing"]:
        log(f"  differs from the receipt's post-switch identity: {rel}")
    if restore.get("reason"):
        log(f"  {restore['reason']}")
    for command in restore["commands"]:
        log(f"  command: {command}")
    for entry in report["pending_manual"]:
        log(f"pending manual entry of attempt {entry['attempt']}; repeat: {entry['command']}")
    return 0


def purge_report_command(args) -> int:
    """Compare the retained identities of the valid receipt, and list the owner's conditions ("Purge")."""
    plan = load_plan(args.plan)
    layout = Layout(args.home, plan)
    report = status_report(layout, plan)
    blocks = []
    if report["health"] != "switched":
        blocks.append(f"the switch is not verified: its health is {report['health']}")
    for p in report["problems"]:
        log(f"  status: {p}")
    receipt = report["receipt"]
    if receipt is None:
        blocks.append("no valid receipt names the retained resources")
    owner = []
    for item in (receipt or {}).get("retained") or []:
        what = " ".join(str(item[k]) for k in ("item", "plugin", "marketplace", "source", "path") if item.get(k))
        conditions = [c for c in item.get("condition") or [] if c not in PROGRAM_CONDITIONS]
        owner += [c for c in conditions if c not in owner]
        if item.get("condition") == ["this receipt is written"]:
            log(f"retained {what}: removable, because this receipt is written")
        elif item.get("identity") is None:
            log(f"retained {what}: no recorded identity to compare")
        elif retained_item_identity(layout, item) != item["identity"]:
            blocks.append(f"the retained {what} changed since the receipt")
        else:
            log(f"retained {what}: keeps its recorded identity")
    for condition in owner:
        log(f"the owner establishes: {condition}")
    for b in blocks:
        log(f"  purge blocked: {b}")
    if blocks:
        log("result: purge is blocked")
        return EXIT_REFUSED
    log("result: the conditions that the program reads hold; the owner establishes the others")
    return 0


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n", 1)[0])
    sub = parser.add_subparsers(dest="command", required=True)
    p = sub.add_parser("report")
    p.add_argument("--plan", required=True)
    p.add_argument("--home", default=account_home())
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
    sub.add_parser("restore-identity").add_argument("path")
    p = sub.add_parser("attempt")
    p.add_argument("--plan", required=True)
    p.add_argument("--home", default=account_home())
    p = sub.add_parser("switch")
    p.add_argument("--plan", required=True)
    p.add_argument("--home", default=account_home())
    p.add_argument("--attempt", required=True, help="the attempt note that `attempt` wrote")
    p.add_argument("--guidance", action="append", help="a guidance file that the operator read; repeatable")
    p = sub.add_parser("restore")
    p.add_argument("--backup", required=True)
    p.add_argument("--attempt", required=True, help="the attempt note of the backup's attempt")
    p.add_argument("--home", default=account_home())
    p.add_argument("--recovery", help="the recovery note of an interrupted later restore")
    p.add_argument("--guidance", action="append", help="a guidance file that the operator read; repeatable")
    p = sub.add_parser("status")
    p.add_argument("--plan", required=True)
    p.add_argument("--home", default=account_home())
    p.add_argument("--json", action="store_true")
    p = sub.add_parser("purge-report")
    p.add_argument("--plan", required=True)
    p.add_argument("--home", default=account_home())
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
        if args.command == "restore-identity":
            print(restore_identity(args.path))
            return 0
        try:
            return {"report": report_command, "attempt": attempt_command, "switch": switch_command,
                    "restore": restore_command, "status": status_command,
                    "purge-report": purge_report_command}[args.command](args)
        except Stop as e:
            log(f"stop: {e}")
            log("result: refused; nothing written")
            return EXIT_REFUSED
    except Stop as e:
        log(f"stop: {e}")
        return EXIT_REFUSED


if __name__ == "__main__":
    sys.exit(main())
