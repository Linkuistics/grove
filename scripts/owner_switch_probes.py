#!/usr/bin/env python3
"""The probes of the owner switch, and its native probe.

docs/specs/owner-skill-delivery.md owns the rules: "The switch inside finish"
(step 7 and the probe observations), "Rollback" (the native probes), "Probe
process lifetime" and "Concurrent writers". The plan names the commands. Each
runs this file as `$python -I <state>/prepared/owner_switch_probes.py ...` in
HOME, with HOME set and OWNER_SWITCH_PLAN naming the plan.

    owner_switch_probes.py resolve --expect switched|disabled [--plan PLAN]
    owner_switch_probes.py session --harness claude|codex --route ravel|native [--plan PLAN]
    owner_switch_probes.py launch --harness claude|codex --route ravel|native --prompt TEXT
                                  [--model M] [--effort E] [--timeout S] [--settle S]
                                  [--interrupt-after S] [--crash-after S] [--keep]

Every subcommand prints `owner-switch-result: <JSON>` as its last line, with
its evidence. Exit status: 0 when the probe passed, 1 when it failed, 2 for a
usage error or a subcommand that this version does not have.

The plan. OWNER_SWITCH_PLAN names it, and OWNER_SWITCH_PLAN_DIGEST must then
name the SHA-256 of its bytes, as for the program's gates. --plan names a plan
without that digest, for a run outside the switch.

The environment. A harness, Ravel and each tool receive an allow-listed
environment: USER, LOGNAME, LANG, LC_ALL, LC_CTYPE, TERM, PATH and TMPDIR from
the probe's own environment, HOME, and DISABLE_AUTOUPDATER=1. So no loop
control variable, Claude Code location variable, CODEX_HOME, RAVEL_* name or
OWNER_SWITCH_* name reaches them. `brew` comes from PATH, and Ravel is
$(brew --prefix)/bin/ravel, as the program finds them.

The work directory. Each command runs in a new directory under /private/tmp,
outside HOME. The probe refuses one with a `.grove`, `.jj`, `.claude/skills`,
`.codex/skills` or `.agents/skills` in it or in an ancestor: Claude Code reads
`.claude/skills` in each parent folder of its working directory, and no probe
runs inside a Grove workspace. The probe removes the directory at its end.

resolve. `ravel skills resolve --json` with HOME's configuration.
    switched  The status is "resolved". The sources are exactly the plan's
              accepted sources, and each gives its accepted resourceDigest.
              The mandatory list is exactly the plan's mandatory files: each
              names its source, lies at that path below the source's
              snapshotRoot and can be read. The result records each digest.
    disabled  The status is "disabled".

session. session-probes-k281 adds the harness sessions. This version refuses
it with exit 2.

launch. The measured launcher ("Probe process lifetime"), which the session
probes use, run on its own for the lifecycle measurements. It runs one
harness with PROMPT in the work directory. The native route runs the harness
directly. The ravel route runs `ravel start-session --kind owner-switch-probe
--config <work>/probe.rhai --prompt PROMPT --json`. Its headless script
selects `/bin/sh -c` with the same harness command, which then runs
`<ravel> end-session harness-exit-<status>`, because Ravel 2.0 fails a
session whose managed process exits without a result. The owner's config.rhai
stays unread: it launches the interactive Codex TUI.
    Claude Code  claude -p --output-format stream-json --verbose --model M
                 --effort E --permission-mode bypassPermissions PROMPT
    Codex        codex exec --json --skip-git-repo-check --color never
                 --model M -c model_reasoning_effort="E" --sandbox
                 workspace-write -c approval_policy="never" PROMPT
The leader starts in a new session and process group. Before the launch the
probe records the creation time of each of the account's processes. It then
reads the process table every 50 ms until the leader exits and for SETTLE
seconds after. A process is owned when the probe sees it as the leader, as
a child of an owned process or as a member of the leader's group, and it did
not exist before the launch. An owned process counts only while its PID has
its recorded creation time, and the leader and its group only while the
leader is the probe's unreaped child. A child counts only when it was created
after its parent and before the table was read. Its record names its PID, creation
time ("<seconds>.<microseconds>"), canonical executable, HOME, CODEX_HOME,
parent, group, argument vector, current folder and the relation that proved
its creation. Each new or changed record goes at once to the journal that
OWNER_SWITCH_PROBE_JOURNAL names, one JSON object per line; without the
variable it goes to <work>/journal.jsonl. A process keeps its record after it
reparents to launchd.
    Before the launch, a Claude Code or Codex process of this configuration
    HOME refuses the launch without a signal: an existing daemon is never
    the probe's. After the leader exits, each owned process that still runs
    receives SIGTERM and then SIGKILL, through owner_switch.end_owned, which
    signals only while the PID, creation time and executable still equal
    the record. Cleanup repeats, up to three rounds, while a reading finds a
    new survivor. A new process that the probe did not record, but that names
    a Claude Code, Codex, Grove or Ravel executable, lies in a configuration
    folder of HOME or works in the work directory, is unattributed, unless
    it descends from the probe's own ancestors. An unattributed process, or
    an owned process that still runs after cleanup, fails the probe. Any
    other new process of the account that launchd adopted, with this HOME,
    goes to "reparentedWithoutRecord" as evidence only, because launchd
    starts the account's own agents with the same HOME.
    --interrupt-after sends SIGINT to the leader's group after S seconds, as
    an interrupt at a terminal does. --crash-after sends SIGKILL to the
    leader alone, which orphans its children. --timeout ends the leader's
    group with SIGTERM, then SIGKILL. A SIGINT, SIGTERM or SIGHUP to the
    probe ends the leader's group the same way and then runs the cleanup.
    The probe signals the leader's group only while the leader is its
    unreaped child, so the group ID cannot name another process.
"""

from __future__ import annotations

import argparse
import ctypes
import hashlib
import json
import os
import shlex
import shutil
import signal
import subprocess
import sys
import tempfile
import time

HERE = os.path.dirname(os.path.realpath(__file__))
# -I leaves the script's folder off sys.path. The program lies beside this
# file both in the repository and in <state>/prepared.
sys.path.insert(0, HERE)
import owner_switch as osw  # noqa: E402  (process identity, the recogniser and end_owned)

RESULT_MARK = osw.RESULT_MARK
JOURNAL_VAR = osw.JOURNAL_VAR
PASSED_NAMES = ("USER", "LOGNAME", "LANG", "LC_ALL", "LC_CTYPE", "TERM", "PATH", "TMPDIR")
WORK_BASE = "/private/tmp"
# A Grove workspace, and the project skill roots that a harness lists from
# each parent folder of its working directory.
WORKSPACE_MARKS = (".grove", ".jj")
PROJECT_SKILL_ROOTS = (".claude/skills", ".codex/skills", ".agents/skills")
# The configuration folders of HOME where a harness or Ravel keeps its own
# executables and state. A new process that runs from one of them, and that
# the probe did not record, is unattributed.
CONFIG_FOLDERS = (".codex", ".claude", ".cache/ravel", ".local/share/claude")
PROBE_KIND = "owner-switch-probe"
DEFAULTS = {"claude": ("claude-sonnet-5-5", "low"), "codex": ("gpt-6.1-sol", "low")}
POLL_SECONDS = 0.05
# Generous margins, so a loaded machine cannot turn a slow harness into a
# failed probe ("Probe process lifetime" measures the real launches).
LAUNCH_TIMEOUT = 1800.0
SETTLE_SECONDS = 3.0
INTERRUPT_GRACE = 20.0
CLEANUP_ROUNDS = 3
CODEX_SANDBOX = "workspace-write"
CODEX_APPROVAL = "never"


class Fail(Exception):
    """The probe failed, or could not run, for the stated reason."""


def emit(result: dict) -> None:
    print(RESULT_MARK + json.dumps(result, sort_keys=True), flush=True)


def note(text: str) -> None:
    print(text, file=sys.stderr, flush=True)


# --- plan, environment and work directory ---------------------------------------

def load_plan(path_arg):
    """The plan of --plan, or the plan of OWNER_SWITCH_PLAN with the digest that the program admitted."""
    path = path_arg or os.environ.get(osw.PLAN_VAR)
    if not path:
        raise Fail(f"the probe needs --plan or {osw.PLAN_VAR}")
    try:
        plan = osw.load_plan(path)
    except (osw.Stop, OSError) as e:
        raise Fail(f"the plan {path} is unusable: {e}")
    if not path_arg:
        expected = os.environ.get(osw.PLAN_DIGEST_VAR)
        if plan["_digest"] != expected:
            raise Fail(f"the plan at {path} is not the plan that the program admitted: its digest is "
                       f"{plan['_digest']}, not {expected}")
    return plan


def probe_home() -> str:
    home = os.environ.get("HOME")
    if not home or not os.path.isabs(home):
        raise Fail("HOME is not set to an absolute path")
    return os.path.realpath(home)


def tool_env(home: str) -> dict:
    """The allow-listed environment of every harness, Ravel and tool command."""
    env = {k: os.environ[k] for k in PASSED_NAMES if k in os.environ}
    env.setdefault("PATH", "/usr/bin:/bin:/usr/sbin:/sbin")
    env.update(HOME=home, DISABLE_AUTOUPDATER="1")
    return env


def dropped_names() -> list:
    """The names of the probe's environment that no command receives, for the record."""
    return sorted(k for k in os.environ if k not in PASSED_NAMES and k != "HOME")


def inside(path: str, root: str) -> bool:
    return path == root or path.startswith(root.rstrip("/") + "/")


def work_problem(work: str, home: str) -> str:
    """Why WORK cannot be a probe's work directory, or ""."""
    if inside(work, home) or inside(home, work):
        return f"the work directory {work} and HOME {home} overlap"
    folder = work
    while True:
        for mark in WORKSPACE_MARKS + PROJECT_SKILL_ROOTS:
            if os.path.lexists(os.path.join(folder, mark)):
                return f"the work directory {work} lies inside {os.path.join(folder, mark)}"
        parent = os.path.dirname(folder)
        if parent == folder:
            return ""
        folder = parent


def make_work(home: str) -> str:
    work = os.path.realpath(tempfile.mkdtemp(prefix="owner-switch-probe-", dir=WORK_BASE))
    problem = work_problem(work, home)
    if problem:
        shutil.rmtree(work, ignore_errors=True)
        raise Fail(problem)
    return work


def brew_prefix(env: dict, work: str) -> str:
    brew = shutil.which("brew", path=env["PATH"])
    if not brew:
        raise Fail("no brew on PATH, so the promoted Ravel cannot be found")
    done = subprocess.run([brew, "--prefix"], env=env, cwd=work, capture_output=True, text=True)
    prefix = done.stdout.strip()
    if done.returncode != 0 or not prefix:
        raise Fail(f"brew --prefix failed: {(done.stdout + done.stderr).strip()[-500:]}")
    return prefix


def first_line(argv: list, env: dict, cwd: str) -> str:
    try:
        done = subprocess.run(argv, env=env, cwd=cwd, capture_output=True, text=True, timeout=120)
    except (OSError, subprocess.TimeoutExpired) as e:
        return f"unavailable: {e}"
    text = (done.stdout or done.stderr).strip()
    return text.splitlines()[0] if text else f"exit {done.returncode}"


def file_digest(path: str) -> str:
    with open(path, "rb") as f:
        return hashlib.sha256(f.read()).hexdigest()


# --- resolve ----------------------------------------------------------------------

def plan_mandatory(plan: dict) -> list:
    """The mandatory files that the plan's switched Ravel settings name: [{"source", "path"}]."""
    for write in plan["writes"]:
        if write["path"] == osw.RAVEL_SETTINGS:
            for cell in write.get("set") or []:
                if cell.get("key") == ["skills"] and isinstance(cell.get("value"), dict):
                    return list(cell["value"].get("mandatory") or [])
    raise Fail(f"the plan sets no skills value in {osw.RAVEL_SETTINGS}")


def resolve_problems(doc, expect: str, accepted: dict, mandatory: list) -> tuple:
    """The problems of one resolve document, and the per-source and per-file records."""
    if not isinstance(doc, dict):
        return ["the resolution printed no JSON object"], [], []
    status = doc.get("status")
    if expect == "disabled":
        return ([] if status == "disabled" else [f"the status is {status!r}, not 'disabled'"]), [], []
    problems = [] if status == "resolved" else [f"the status is {status!r}, not 'resolved'"]
    found = {s.get("id"): s for s in doc.get("sources") or [] if isinstance(s, dict)}
    sources = []
    for sid in sorted(set(found) | set(accepted)):
        source, want = found.get(sid), (accepted.get(sid) or {}).get("resourceDigest")
        got = source.get("resourceDigest") if source else None
        sources.append({"id": sid, "resourceDigest": got, "accepted": want,
                        "originPath": source.get("originPath") if source else None})
        if source is None:
            problems.append(f"the accepted source {sid} is missing")
        elif want is None:
            problems.append(f"the source {sid} is not an accepted source")
        elif got != want:
            problems.append(f"the source {sid} gives resourceDigest {got}, not the accepted {want}")
    files = []
    listed = [m for m in doc.get("mandatory") or [] if isinstance(m, dict)]
    for want in mandatory:
        root = (found.get(want.get("source")) or {}).get("snapshotRoot")
        expected = os.path.realpath(os.path.join(root, want["path"])) if isinstance(root, str) else None
        match = [m for m in listed if m.get("source") == want.get("source") and isinstance(m.get("path"), str)
                 and expected and os.path.realpath(m["path"]) == expected]
        record = {"source": want.get("source"), "path": want.get("path"), "resolved": expected}
        if not match:
            problems.append(f"the mandatory file {want.get('source')}:{want.get('path')} is not listed at {expected}")
        else:
            try:
                record["sha256"] = file_digest(match[0]["path"])
            except OSError as e:
                problems.append(f"the mandatory file {match[0]['path']} cannot be read: {e}")
        files.append(record)
    extra = len(listed) - sum(1 for f in files if f.get("resolved"))
    if extra > 0:
        problems.append(f"the resolution lists {len(listed)} mandatory files, and the plan names {len(mandatory)}")
    return problems, sources, files


def resolve_command(args) -> dict:
    home = probe_home()
    plan = load_plan(args.plan)
    accepted = plan["accepted"]["sources"]
    mandatory = plan_mandatory(plan) if args.expect == "switched" else []
    env = tool_env(home)
    work = make_work(home)
    try:
        ravel = os.path.join(brew_prefix(env, work), "bin", "ravel")
        version = first_line([ravel, "--version"], env, work)
        done = subprocess.run([ravel, "skills", "resolve", "--json"], env=env, cwd=work, capture_output=True,
                              text=True, timeout=600)
    finally:
        shutil.rmtree(work, ignore_errors=True)
    try:
        doc = json.loads(done.stdout)
    except ValueError:
        doc = None
    problems, sources, files = resolve_problems(doc, args.expect, accepted, mandatory)
    if done.returncode != 0:
        problems.insert(0, f"{ravel} skills resolve --json exited {done.returncode}: {done.stderr.strip()[-800:]}")
    return {"probe": "resolve", "expect": args.expect, "home": home, "ravel": ravel,
            "ravelCanonical": os.path.realpath(ravel), "version": version,
            "status": doc.get("status") if isinstance(doc, dict) else None, "sources": sources,
            "mandatory": files, "planDigest": plan["_digest"], "work": work,
            "droppedNames": dropped_names(), "problems": problems, "ok": not problems}


# --- process tracking ---------------------------------------------------------------

def process_table() -> dict:
    """Each process as {pid: {"ppid", "pgid", "uid", "zombie"}}, from ps."""
    done = subprocess.run(["/bin/ps", "-axww", "-o", "pid=,ppid=,pgid=,uid=,stat="], capture_output=True,
                          text=True)
    if done.returncode != 0:
        raise Fail(f"the process listing failed: {done.stderr.strip()}")
    procs = {}
    for line in done.stdout.splitlines():
        parts = line.split()
        if len(parts) >= 5:
            procs[int(parts[0])] = {"ppid": int(parts[1]), "pgid": int(parts[2]), "uid": int(parts[3]),
                                    "zombie": "Z" in parts[4]}
    return procs


def process_environment(pid: int, names: tuple) -> dict:
    """The values of NAMES in a process's environment (KERN_PROCARGS2), for the account's own processes."""
    size = ctypes.c_size_t(osw._argmax())
    buf = ctypes.create_string_buffer(size.value)
    mib = (ctypes.c_int * 3)(osw.CTL_KERN, osw.KERN_PROCARGS2, pid)
    if osw._libc.sysctl(mib, 3, buf, ctypes.byref(size), None, 0) != 0 or size.value < 4:
        return {}
    raw = buf.raw[:size.value]
    argc = int.from_bytes(raw[:4], sys.byteorder)
    rest = raw[4:]
    end = rest.find(b"\0")
    if end < 0:
        return {}
    items = rest[end:].lstrip(b"\0").split(b"\0")
    wanted = {n.encode() + b"=": n for n in names}
    found = {}
    for item in items[argc:]:
        if not item:
            break
        for prefix, name in wanted.items():
            if item.startswith(prefix) and name not in found:
                found[name] = os.fsdecode(item[len(prefix):])
    return found


def identity(pid: int) -> dict:
    """What the probe records of one of the account's processes, or {} when it is gone."""
    start = osw.process_start(pid)
    if start is None:
        return {}
    launch, argv, home = osw.process_arguments(pid)
    env = process_environment(pid, ("CODEX_HOME",))
    return {"pid": pid, "start": start, "executable": osw.executable_path(pid) or launch, "home": home,
            "codexHome": env.get("CODEX_HOME"), "argv": (argv or [])[:12], "folder": osw.process_folder(pid)}


def ancestry(procs: dict, pid: int) -> list:
    out = []
    while pid in procs and procs[pid]["ppid"] not in (0, pid):
        pid = procs[pid]["ppid"]
        out.append(pid)
    return out


class Tracker:
    """The processes that one launch creates: "Probe process lifetime"."""

    def __init__(self, journal: str):
        self.journal = journal
        self.uid = os.getuid()
        self.me = os.getpid()
        procs = process_table()
        self.baseline = {(pid, osw.process_start(pid)) for pid, p in procs.items() if p["uid"] == self.uid}
        self.mine = set(ancestry(procs, self.me)) - {1}
        self.started = time.time()
        self.leader = None
        self.leader_proc = None
        self.reparented = []
        self.records = {}
        self.order = []

    def write(self, record: dict) -> None:
        with open(self.journal, "a") as f:
            f.write(json.dumps(record, sort_keys=True) + "\n")
            f.flush()
            os.fsync(f.fileno())

    def leader_unreaped(self) -> bool:
        """Whether the leader is this process's unreaped child, so its PID and group ID name nothing else."""
        return self.leader_proc is not None and self.leader_proc.returncode is None

    def poll(self) -> dict:
        """Read the process table once and record each owned process that it shows.

        A root is a record whose PID still has the recorded creation time, or
        the leader while it is unreaped. A child counts only when it was
        created after its parent and before the table was read, so a PID
        reused between the table and the identity read never counts. The
        leader's group counts only while the leader is unreaped.
        """
        snapshot = time.time()
        procs = process_table()
        if self.leader is None:
            return procs
        children = {}
        for pid, p in procs.items():
            children.setdefault(p["ppid"], []).append(pid)
        stack = [(key[0], key[1], rec["relation"]) for key, rec in self.records.items()
                 if osw.process_start(key[0]) == key[1]]
        if self.leader_unreaped():
            stack.append((self.leader, osw.process_start(self.leader), "leader"))
        seen = {}
        while stack:
            pid, start, relation = stack.pop()
            if pid in seen or pid not in procs or start is None:
                continue
            seen[pid] = (relation, start)
            for child in children.get(pid, []):
                child_start = osw.process_start(child)
                if child_start is not None and float(start) <= float(child_start) <= snapshot:
                    stack.append((child, child_start, "descendant"))
        if self.leader_unreaped():
            for pid, p in procs.items():
                if p["pgid"] == self.leader and pid not in seen:
                    start = osw.process_start(pid)
                    if start is not None and float(start) <= snapshot:
                        seen[pid] = ("group", start)
        now = time.time()
        for pid, (relation, start) in seen.items():
            p = procs[pid]
            if p["uid"] != self.uid or p["zombie"] or pid == self.me:
                continue
            ident = identity(pid)
            if not ident or ident["start"] != start:
                continue
            key = (pid, start)
            if key in self.baseline:
                continue
            rec = self.records.get(key)
            if rec is None:
                rec = dict(ident, ppid=p["ppid"], pgid=p["pgid"], relation=relation, firstSeen=now, lastSeen=now)
                self.records[key] = rec
                self.order.append(key)
                self.write(rec)
            else:
                rec.update(lastSeen=now, ppid=p["ppid"])
                if ident["executable"] != rec["executable"] or ident["argv"] != rec["argv"]:
                    rec.update(executable=ident["executable"], argv=ident["argv"], home=ident["home"],
                               codexHome=ident["codexHome"])
                    self.write(rec)
        for key, rec in self.records.items():
            if not rec.get("gone") and osw.owned_state(rec) == "gone":
                rec["gone"] = now
        return procs

    def owned(self) -> list:
        return [self.records[k] for k in self.order]

    def unattributed(self, procs: dict, home: str, work: str) -> list:
        """New processes that the probe did not record but that a probe launch can explain."""
        folders = [os.path.join(home, f) for f in CONFIG_FOLDERS]
        out = []
        for pid, p in procs.items():
            if p["uid"] != self.uid or p["zombie"] or pid == self.me:
                continue
            if set(ancestry(procs, pid)) & (self.mine | {self.me}):
                continue
            ident = identity(pid)
            key = (pid, ident.get("start"))
            if not ident or key in self.baseline or key in self.records or float(ident["start"]) < self.started - 1:
                continue
            exe = ident["executable"] or ""
            tool = osw.recognise(osw.executable_path(pid), osw.process_arguments(pid)[0], ident["argv"]) or {}
            reasons = []
            if "tool" in tool:
                reasons.append(f"it is {tool['tool']} ({tool['form']})")
            if any(inside(exe, f) for f in folders):
                reasons.append("its executable lies in a configuration folder of HOME")
            if ident["folder"] and inside(os.path.realpath(ident["folder"]), work):
                reasons.append("it works in the work directory")
            if reasons:
                out.append(dict(ident, ppid=p["ppid"], pgid=p["pgid"], reasons=reasons))
            elif p["ppid"] == 1 and ident["home"] and os.path.realpath(ident["home"]) == home:
                # Evidence only: launchd starts the account's own agents with this HOME too.
                self.reparented.append(dict(ident, ppid=p["ppid"], pgid=p["pgid"]))
        return out


def config_home_of(item: dict, home: str) -> str:
    return item.get("codexHome") or os.path.join(item.get("home") or home, ".codex")


def existing_harnesses(home: str) -> list:
    """Each Claude Code or Codex process that uses this configuration HOME before the launch."""
    procs = process_table()
    me = os.getpid()
    mine = set(ancestry(procs, me))
    out = []
    for pid, p in procs.items():
        if p["uid"] != os.getuid() or p["zombie"] or pid == me or pid in mine:
            continue
        exe = osw.executable_path(pid)
        launch, argv, env_home = osw.process_arguments(pid)
        tool = osw.recognise(exe, launch, argv) or {}
        if tool.get("tool") not in osw.HARNESSES:
            continue
        codex_home = process_environment(pid, ("CODEX_HOME",)).get("CODEX_HOME")
        path = exe or launch or ""
        uses = (env_home and os.path.realpath(env_home) == home) or inside(path, home) or (
            codex_home and inside(os.path.realpath(codex_home), home))
        if uses:
            out.append({"pid": pid, "executable": path, "tool": tool["tool"], "role": tool.get("role", ""),
                        "args": " ".join(argv or [])[:200]})
    return out


# --- the launcher -----------------------------------------------------------------

def harness_argv(harness: str, model: str, effort: str, prompt: str, add_dirs=()) -> tuple:
    """The program and the arguments of one headless harness launch."""
    if harness == "claude":
        args = []
        for d in add_dirs:
            args += ["--add-dir", d]
        return "claude", args + ["-p", "--output-format", "stream-json", "--verbose", "--model", model,
                                 "--effort", effort, "--permission-mode", "bypassPermissions", prompt]
    return "codex", ["exec", "--json", "--skip-git-repo-check", "--color", "never", "--model", model,
                     "-c", f'model_reasoning_effort="{effort}"', "--sandbox", CODEX_SANDBOX,
                     "-c", f'approval_policy="{CODEX_APPROVAL}"', prompt]


def ending_shell(ravel: str) -> str:
    """The `sh -c` script of a ravel-route launch: run the harness, then end the session with its status.

    Ravel 2.0's start-session fails with session_result_missing when the
    managed process exits without `ravel end-session RESULT`. The probe ends
    the session itself, so the result does not depend on the agent.
    """
    return '"$0" "$@"; status=$?; exec ' + shlex.quote(ravel) + ' end-session "harness-exit-$status"'


def probe_script(harness: str, model: str, effort: str, ravel: str) -> str:
    """The headless Ravel selection script of a ravel-route launch: the native route's command, then end-session."""
    program, fixed = harness_argv(harness, model, effort, "")
    fixed = ["-c", ending_shell(ravel), program] + fixed[:-1]
    add_dirs = ('    if request.skills != () { for source in request.skills.sources '
                '{ args += ["--add-dir", source.snapshotRoot]; } }\n') if harness == "claude" else ""
    provider = "anthropic" if harness == "claude" else "openai"
    head, tail = fixed[:3], fixed[3:]
    return ("// Written by owner_switch_probes.py for one probe launch.\n"
            "export const SCHEMA_VERSION = 4;\n"
            f'export const VERSION = "{PROBE_KIND}";\n\n'
            "fn select(request, context) {\n"
            f"    let args = [{', '.join(json.dumps(a) for a in head)}];\n" + add_dirs +
            f"    args += [{', '.join(json.dumps(a) for a in tail)}];\n"
            "    args.push(request.prompt);\n"
            f'    #{{ status: "selected", program: "/bin/sh", args: args, provider: "{provider}", '
            f'model: {json.dumps(model)}, effort: {json.dumps(effort)}, reason: "owner switch probe" }}\n'
            "}\n")


class Launch:
    """One measured harness launch: its leader, its owned processes and their cleanup."""

    def __init__(self, harness: str, route: str, prompt: str, home: str, work: str, model=None, effort=None,
                 timeout=LAUNCH_TIMEOUT, settle=SETTLE_SECONDS, interrupt_after=None, crash_after=None):
        self.harness, self.route, self.prompt, self.home, self.work = harness, route, prompt, home, work
        default_model, default_effort = DEFAULTS[harness]
        self.model, self.effort = model or default_model, effort or default_effort
        self.timeout, self.settle = timeout, settle
        self.interrupt_after, self.crash_after = interrupt_after, crash_after
        self.env = tool_env(home)
        self.signalled = None
        self.events = []

    def argv(self) -> list:
        if self.route == "native":
            program, args = harness_argv(self.harness, self.model, self.effort, self.prompt)
            path = shutil.which(program, path=self.env["PATH"])
            if not path:
                raise Fail(f"no {program} on PATH")
            return [path] + args
        script = os.path.join(self.work, "probe.rhai")
        ravel = os.path.join(brew_prefix(self.env, self.work), "bin", "ravel")
        with open(script, "w") as f:
            f.write(probe_script(self.harness, self.model, self.effort, ravel))
        return [ravel, "start-session", "--kind", PROBE_KIND, "--config", script, "--prompt", self.prompt, "--json"]

    def on_signal(self, signum, frame) -> None:
        self.signalled = signum

    def signal_group(self, leader, sig) -> None:
        """Signal the leader's group, only while the leader is this process's unreaped child."""
        if leader.poll() is None:
            try:
                os.killpg(leader.pid, sig)
                self.events.append({"at": round(time.time() - self.t0, 3), "group": signal.Signals(sig).name})
            except ProcessLookupError:
                pass

    def end_group(self, leader, first=signal.SIGTERM) -> None:
        """FIRST to the leader's group, then SIGTERM and SIGKILL, each after a grace period."""
        for sig in dict.fromkeys((first, signal.SIGTERM, signal.SIGKILL)):
            self.signal_group(leader, sig)
            deadline = time.monotonic() + INTERRUPT_GRACE
            while leader.poll() is None and time.monotonic() < deadline:
                self.tracker.poll()
                time.sleep(POLL_SECONDS)
            if leader.poll() is not None:
                return

    def cleanup(self) -> dict:
        """End each owned process that still runs, in rounds, until a reading shows no new survivor.

        A survivor can create a child during its grace period. Each round
        reads the process table again for a second, so such a child gains a
        record and the next round ends it. The rounds stop at CLEANUP_ROUNDS.
        """
        signalled, remaining = [], []
        for _ in range(CLEANUP_ROUNDS):
            survivors = [dict(r) for r in self.tracker.owned() if osw.owned_state(r) != "gone"]
            if not survivors:
                remaining = []
                break
            outcome = osw.end_owned(survivors, watch=self.tracker.poll)
            signalled += outcome["signalled"]
            remaining = outcome["remaining"]
            deadline = time.monotonic() + 1.0
            while time.monotonic() < deadline:
                self.tracker.poll()
                time.sleep(POLL_SECONDS)
        else:
            remaining = [dict(r, state=osw.owned_state(r)) for r in self.tracker.owned()
                         if osw.owned_state(r) != "gone"]
        return {"signalled": signalled, "remaining": remaining}

    def run(self) -> dict:
        journal = os.environ.get(JOURNAL_VAR) or os.path.join(self.work, "journal.jsonl")
        existing = existing_harnesses(self.home)
        if existing:
            raise Fail("a Claude Code or Codex process of this configuration HOME runs before the launch, and "
                       "the probe signals none: " + "; ".join(f"{e['pid']} {e['tool']} {e['role']} {e['executable']}"
                                                         for e in existing))
        argv = self.argv()
        self.tracker = Tracker(journal)
        out = open(os.path.join(self.work, "stdout"), "wb")
        err = open(os.path.join(self.work, "stderr"), "wb")
        previous = {s: signal.signal(s, self.on_signal) for s in (signal.SIGINT, signal.SIGTERM, signal.SIGHUP)}
        self.t0 = time.time()
        ended_by = None
        leader = None
        try:
            leader = subprocess.Popen(argv, cwd=self.work, env=self.env, stdin=subprocess.DEVNULL, stdout=out,
                                      stderr=err, start_new_session=True)
            self.tracker.leader = leader.pid
            self.tracker.leader_proc = leader
            while leader.poll() is None:
                self.tracker.poll()
                elapsed = time.time() - self.t0
                if self.signalled is not None:
                    ended_by = f"probe received {signal.Signals(self.signalled).name}"
                    self.end_group(leader)
                elif self.interrupt_after is not None and elapsed >= self.interrupt_after:
                    ended_by = "interrupt"
                    self.interrupt_after = None
                    self.end_group(leader, signal.SIGINT)
                elif self.crash_after is not None and elapsed >= self.crash_after:
                    ended_by = "crash"
                    self.crash_after = None
                    if leader.poll() is None:
                        leader.send_signal(signal.SIGKILL)
                        self.events.append({"at": round(elapsed, 3), "leader": "SIGKILL"})
                elif elapsed >= self.timeout:
                    ended_by = "timeout"
                    self.end_group(leader)
                time.sleep(POLL_SECONDS)
            exit_code = leader.wait()
            leader_ended = time.time()
            deadline = time.monotonic() + self.settle
            while time.monotonic() < deadline:
                self.tracker.poll()
                time.sleep(POLL_SECONDS)
            survivors = [dict(r) for r in self.tracker.owned() if osw.owned_state(r) != "gone"]
            cleanup = self.cleanup()
            procs = self.tracker.poll()
            unattributed = self.tracker.unattributed(procs, self.home, self.work)
        except BaseException:
            # A failure of the probe itself still ends the leader's group and reaps the leader.
            if leader is not None and leader.poll() is None:
                try:
                    self.end_group(leader)
                except Exception:
                    pass
                if leader.poll() is None:
                    # Still the unreaped child, so its PID and group ID name nothing else.
                    try:
                        os.killpg(leader.pid, signal.SIGKILL)
                    except OSError:
                        pass
                leader.wait()
            raise
        finally:
            for s, handler in previous.items():
                signal.signal(s, handler)
            out.close()
            err.close()
        problems = []
        for r in cleanup["remaining"]:
            problems.append(f"the owned process {r['pid']} ({r['executable']}) still runs after cleanup: {r['state']}")
        for u in unattributed:
            problems.append(f"the process {u['pid']} ({u['executable']}) appeared without a record: "
                            + "; ".join(u["reasons"]))
        if self.signalled is not None:
            problems.append(f"the probe received {signal.Signals(self.signalled).name}")
        if ended_by == "timeout":
            problems.append(f"the harness ran longer than {self.timeout} seconds")
        return {"harness": self.harness, "route": self.route, "model": self.model, "effort": self.effort,
                "argv": argv, "work": self.work, "home": self.home, "journal": journal,
                "leader": {"pid": leader.pid, "exit": exit_code, "endedBy": ended_by,
                           "seconds": round(leader_ended - self.t0, 3)},
                "events": self.events, "owned": self.tracker.owned(),
                "survivors": [{k: r[k] for k in ("pid", "start", "executable", "argv")} for r in survivors],
                "cleanup": {"signalled": [{"pid": r["pid"], "signal": r["signal"], "executable": r["executable"]}
                                          for r in cleanup["signalled"]],
                            "remaining": [{"pid": r["pid"], "state": r["state"]} for r in cleanup["remaining"]]},
                "unattributed": unattributed, "reparentedWithoutRecord": self.tracker.reparented,
                "problems": problems}

    def output(self) -> tuple:
        with open(os.path.join(self.work, "stdout"), "rb") as f:
            stdout = f.read().decode(errors="replace")
        with open(os.path.join(self.work, "stderr"), "rb") as f:
            stderr = f.read().decode(errors="replace")
        return stdout, stderr


def launch_command(args) -> dict:
    home = probe_home()
    work = make_work(home)
    try:
        launch = Launch(args.harness, args.route, args.prompt, home, work, args.model, args.effort,
                        timeout=args.timeout, settle=args.settle, interrupt_after=args.interrupt_after,
                        crash_after=args.crash_after)
        result = launch.run()
        stdout, stderr = launch.output()
        version = first_line([result["argv"][0], "--version"], launch.env, work)
    finally:
        if not args.keep:
            shutil.rmtree(work, ignore_errors=True)
    result.update(probe="launch", version=version, droppedNames=dropped_names(), kept=bool(args.keep),
                  stdoutTail=stdout[-4000:], stderrTail=stderr[-4000:], ok=not result["problems"])
    return result


# --- entry ------------------------------------------------------------------------

def parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(prog="owner_switch_probes.py", description=__doc__.split("\n\n")[0])
    sub = p.add_subparsers(dest="command", required=True)
    r = sub.add_parser("resolve", help="ravel skills resolve with HOME's configuration")
    r.add_argument("--expect", choices=("switched", "disabled"), required=True)
    r.add_argument("--plan")
    s = sub.add_parser("session", help="one harness session probe (session-probes-k281)")
    s.add_argument("--harness", choices=("claude", "codex"), required=True)
    s.add_argument("--route", choices=("ravel", "native"), required=True)
    s.add_argument("--plan")
    m = sub.add_parser("launch", help="one measured harness launch")
    m.add_argument("--harness", choices=("claude", "codex"), required=True)
    m.add_argument("--route", choices=("ravel", "native"), required=True)
    m.add_argument("--prompt", required=True)
    m.add_argument("--model")
    m.add_argument("--effort")
    m.add_argument("--timeout", type=float, default=LAUNCH_TIMEOUT)
    m.add_argument("--settle", type=float, default=SETTLE_SECONDS)
    m.add_argument("--interrupt-after", type=float)
    m.add_argument("--crash-after", type=float)
    m.add_argument("--keep", action="store_true", help="keep the work directory")
    return p


def main(argv=None) -> int:
    args = parser().parse_args(argv)
    if args.command == "session":
        emit({"probe": "session", "ok": False, "problems": ["session-probes-k281 adds the session probes; "
                                                            "this version of the probe program has none"]})
        return 2
    try:
        result = resolve_command(args) if args.command == "resolve" else launch_command(args)
    except Fail as e:
        note(f"{args.command}: {e}")
        emit({"probe": args.command, "ok": False, "problems": [str(e)]})
        return 1
    for problem in result["problems"]:
        note(f"{args.command}: {problem}")
    note(f"{args.command}: {'passed' if result['ok'] else 'failed'}")
    emit(result)
    return 0 if result["ok"] else 1


if __name__ == "__main__":
    sys.exit(main())
