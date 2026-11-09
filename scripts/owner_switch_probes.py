#!/usr/bin/env python3
"""The probes of the owner switch, and its native probe.

docs/specs/owner-skill-delivery.md owns the rules: "The switch inside finish"
(step 7 and the probe observations), "Rollback" (the native probes), "Probe
process lifetime" and "Concurrent writers". The plan names the commands. Each
runs this file as `$python -I <state>/prepared/owner_switch_probes.py ...` in
HOME, with HOME set and OWNER_SWITCH_PLAN naming the plan.

    owner_switch_probes.py resolve --expect switched|disabled [--plan PLAN]
    owner_switch_probes.py session --harness claude|codex --route ravel|native [--plan PLAN]
                                   [--timeout S] [--keep]
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

session. One headless harness session through the measured launcher, in a new
work directory, with a fixed prompt. scripts/harness_trial.py, beside this
file, extracts the events from the stream and the session file. Each skill
read is a classified path or a tool result that holds the skill's `name:`
line and first heading. The result keeps each tool result only as its digest.
The Grove kind skill is grove-planning: a grove-core skill whose kind is not
finish. Claude Code runs claude-opus-5-5 and Codex gpt-6.1-sol, both at
medium effort.
    ravel   The prompt asks for a small Python command-line tool for other
            agents, then for one quoted line of grove-planning, then for
            using-testanyware, `testanyware llm-instructions` and
            `testanyware doctor`. The observations are:
            mandatoryFirst  The mandatory file comes before any skill search
                            and any task action.
            craftSkill      cli-tool-design or coding-style-python, from a
                            craft root of its snapshot, before the first edit.
                            The prompt names neither.
            kindSkill       grove-planning from the grove-core snapshot, and a
                            quoted line of that copy.
            testanyware     using-testanyware from its snapshot, and both
                            commands.
            hooks           Claude Code only: a SessionStart hook response
                            that holds "technical-communication".
            listing         The harness's own skill listing is not empty and
                            holds no superseded name.
            digests         The request view names exactly the plan's
                            accepted sources with their digests, and the
                            plan's mandatory file.
    native  The prompt asks only for one quoted line of grove-planning.
            listing         The listing holds grove-planning.
            kindSkill       A native read of grove-planning: the Skill tool,
                            or a native copy by path or by content. The
                            quoted line is a line of a native copy.
    The superseded names are the base names of the plan's `absent`
    assertions below a native skill root. On the ravel route, each skill of
    the view's search directories is one too. The quoted line that the
    prompt asks for differs between the snapshot copy and the native copy.
    Before the launch and after its cleanup, the probe reads each ordinary
    child of ~/.claude/skills, ~/.codex/skills and ~/.agents/skills by
    owner_switch.identity. It skips only ~/.claude/skills/synced and
    ~/.codex/skills/.system, and only while each is a real folder. Any other
    added, removed or changed child fails the probe ("Concurrent writers").
    The result's `writes` lists each entry that the session added, changed or
    removed at the top level of HOME and below WRITE_ROOTS. It names each
    change below a native skill root or an agent definition root. It judges
    none of them.
    The session also fails when the launch fails, when the leader exits
    non-zero, or when Ravel's result is not harness-exit-0.

launch. The measured launcher ("Probe process lifetime"), which the session
probes use, run on its own for the lifecycle measurements. It runs one
harness with PROMPT in the work directory. The native route runs the harness
directly. The ravel route runs `ravel start-session --kind owner-switch-probe
--config <work>/probe.rhai --prompt PROMPT --json`. Its headless script
selects `/bin/sh -c` with the same harness command. The shell writes Ravel's
request view to <work>/view.json and the harness's output to
<work>/stream.jsonl, because Ravel prints only the session result. It then
runs `<ravel> end-session harness-exit-<status>`, because Ravel 2.0 fails a
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
    When the probe itself fails, as when the process table cannot be read,
    it still ends the leader's group, reaps the leader and runs the cleanup.
    It keeps the original error. An owned process that still runs then
    appears in the failure, and the work folder stays, with the journal.
"""

from __future__ import annotations

import argparse
import ctypes
import hashlib
import json
import os
import re
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
import harness_trial as ht  # noqa: E402  (the harness adapters and the event classifier)

# A read inside a script, such as a Python heredoc, names no path, so only its
# result shows what the agent received. An agent reads a snapshot through a
# glob. A command can read a skill and touch the work folder at once. The
# staging trial runner sets the same three switches.
ht.KEEP_RESULTS = True
ht.EXPAND_GLOBS = True
ht.KEEP_CLASSES = True

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
CODEX_SANDBOX = "workspace-write"
CODEX_APPROVAL = "never"

# The session probes. The owner's policy runs each Grove kind on Claude Code
# with claude-opus-5-5 at medium effort or higher. A probe inside finish must
# not fail for a skipped skill search ("The failures at low effort" of the
# evidence record).
SESSION_DEFAULTS = {"claude": ("claude-opus-5-5", "medium"), "codex": ("gpt-6.1-sol", "medium")}
# The Grove kind skill: a skill of the grove-core snapshot whose kind is not
# finish. grove-core holds only the planning kinds besides finish.
KIND_SKILL = "grove-planning"
KIND_SOURCE = "grove-core"
# The craft skills that the probe's unnamed task needs: a Python command-line
# tool that other agents run. The hooks' instruction names
# technical-communication, so a read of it shows no discovery.
CRAFT_SKILLS = ("cli-tool-design", "coding-style-python")
TESTANYWARE_SKILL = "using-testanyware"
TESTANYWARE_SOURCE = "testanyware"
TESTANYWARE_COMMANDS = {"llm-instructions": re.compile(r"\btestanyware\s+llm-instructions\b"),
                        "doctor": re.compile(r"\btestanyware\s+doctor\b")}
# The SessionStart hook of technical-communication-hooks@writing-hooks prints
# the always-on instruction, which names its skill.
HOOK_MARK = "technical-communication"
# The classes of an event that looks for a skill or acts on the task. The
# mandatory obligation of Ravel's discovery instructions comes before each
# (staging_trial.py, GATED).
GATED = {"listing", "skill", "kind-skill", "supporting", "native", "planted", "source-origin", "action",
         "command", "work", "delegate", "native-skill-tool"}
# A read of a skill: its SKILL.md by path, or a result that holds its marks.
SKILL_CLASSES = {"skill", "kind-skill", "content"}
# A quoted part of a line shorter than this proves no read of that copy.
QUOTE_MIN_CHARS = 20
# The native skill roots of HOME. The switch's absent assertions lie in them.
NATIVE_SKILL_ROOTS = (".claude/skills", ".codex/skills", ".agents/skills", ".pi/agent/skills")
# The roots whose ordinary children each probe compares, with the one exact
# platform-managed subtree of each ("Concurrent writers").
COMPARED_ROOTS = {".claude/skills": "synced", ".codex/skills": ".system", ".agents/skills": None}
AGENT_ROOTS = (".claude/agents", ".codex/agents")
# The folders where a harness session, its MCP servers, its hooks, Ravel or
# TestAnyware can write. The write record walks each, and the top level of
# HOME. The technical-communication hooks keep a reply log in their state
# folder.
WRITE_ROOTS = (".claude", ".codex", ".agents", ".pi", ".config/ravel", ".config/testanyware", ".cache/ravel",
               ".local/share/claude", ".local/state/ravel", ".local/state/technical-communication", ".npm/_npx",
               "Library/Keychains", "Library/Application Support/TestAnyware")
RAVEL_PROMPT = (
    "This session is an automated probe of skill delivery. Do these steps in this order, and ask no question.\n\n"
    "1. In the current directory, write a small Python command-line tool, `tally.py`, that other AI agents will "
    "run. It counts the lines, words and bytes of the files that it is given. When it is written, run it once "
    "on itself.\n"
    f"2. Load the `{KIND_SKILL}` skill. Do not follow its procedure. Quote, word for word, the first line of "
    "its body that begins with two asterisks, on a line of its own that begins with `QUOTE: `.\n"
    "3. Load the `using-testanyware` skill. Then run `testanyware llm-instructions` and `testanyware doctor`. "
    "Start, stop or change no virtual machine. A failed doctor check is acceptable.\n\n"
    "End with a short reply that repeats the `QUOTE: ` line."
)
NATIVE_PROMPT = (
    "This session is an automated probe of skill discovery. Ask no question.\n\n"
    f"Load the `{KIND_SKILL}` skill. Do not follow its procedure. Quote, word for word, the first line of its "
    "body that begins with two asterisks, on a line of its own that begins with `QUOTE: `. Do nothing else."
)


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
        roots = [(key[0], key[1], rec["relation"]) for key, rec in self.records.items()
                 if osw.process_start(key[0]) == key[1]]
        if self.leader_unreaped():
            roots.append((self.leader, osw.process_start(self.leader), "leader"))
        seen = osw.descendants(procs, roots, snapshot)
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


def ending_shell(ravel: str, view: str, stream: str) -> str:
    """The `sh -c` script of a ravel-route launch: keep the view, run the harness, then end the session.

    Its first argument is Ravel's request view as JSON, which the script
    writes to VIEW. The harness's standard output goes to STREAM, because
    Ravel prints only the session result. Ravel 2.0's start-session fails
    with session_result_missing when the managed process exits without
    `ravel end-session RESULT`. The probe ends the session itself, so the
    result does not depend on the agent.
    """
    return (f"printf '%s' \"$0\" >{shlex.quote(view)}; \"$@\" >{shlex.quote(stream)}; status=$?; exec "
            + shlex.quote(ravel) + ' end-session "harness-exit-$status"')


def probe_script(harness: str, model: str, effort: str, ravel: str, view: str, stream: str) -> str:
    """The headless Ravel selection script of a ravel-route launch: the native route's command, then end-session."""
    program, fixed = harness_argv(harness, model, effort, "")
    add_dirs = ('    if request.skills != () { for source in request.skills.sources '
                '{ args += ["--add-dir", source.snapshotRoot]; } }\n') if harness == "claude" else ""
    provider = "anthropic" if harness == "claude" else "openai"
    return ("// Written by owner_switch_probes.py for one probe launch.\n"
            "export const SCHEMA_VERSION = 4;\n"
            f'export const VERSION = "{PROBE_KIND}";\n\n'
            "fn select(request, context) {\n"
            "    let view = #{ kind: request.kind, skills: request.skills };\n"
            f'    let args = ["-c", {json.dumps(ending_shell(ravel, view, stream))}, ravel::json_encode(view), '
            f"{json.dumps(program)}];\n" + add_dirs +
            f"    args += [{', '.join(json.dumps(a) for a in fixed[:-1])}];\n"
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
        # Set when a failure of the probe leaves an owned process unresolved: the work folder stays.
        self.keep_work = False

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
            f.write(probe_script(self.harness, self.model, self.effort, ravel, self.view_path(),
                                 self.stream_path()))
        return [ravel, "start-session", "--kind", PROBE_KIND, "--config", script, "--prompt", self.prompt, "--json"]

    def view_path(self) -> str:
        """Where a ravel-route launch keeps Ravel's request view."""
        return os.path.join(self.work, "view.json")

    def stream_path(self) -> str:
        """The harness's event stream: the leader's output on the native route, a file of its own on the ravel route."""
        return os.path.join(self.work, "stdout" if self.route == "native" else "stream.jsonl")

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
        record and the next round ends it (owner_switch.end_in_rounds, which
        the switch uses too).
        """
        return osw.end_in_rounds(self.tracker.owned, self.tracker.poll)

    def failure_cleanup(self) -> list:
        """After a failure of the probe itself: end each owned process that still runs; return each that remains.

        When the rounds cannot run, as when the process table cannot be
        read, the recorded processes are ended without a watch.
        """
        try:
            return self.cleanup()["remaining"]
        except Exception as e:
            note(f"launch: the cleanup rounds failed ({e}); the recorded processes are ended without a watch")
            survivors = [dict(r) for r in self.tracker.owned() if osw.owned_state(r) != "gone"]
            return osw.end_owned(survivors)["remaining"]

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
        except BaseException as e:
            # A failure of the probe itself still ends the leader's group, reaps the leader and ends
            # each owned process, so a recorded service outside the group does not survive it.
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
            try:
                remaining = self.failure_cleanup()
            except Exception as cleanup:
                remaining = [{"pid": "?", "executable": f"the cleanup failed: {cleanup}", "state": "unknown"}]
            if remaining:
                # The journal is the only record of these processes when it lies in the work folder.
                self.keep_work = True
                report = ("; after it, these owned processes still run: "
                          + ", ".join(f"{r['pid']} ({r['executable']}, {r['state']})" for r in remaining)
                          + f". The journal {journal} keeps their records")
                note(f"launch: {report[2:]}")
                if isinstance(e, Fail):
                    raise Fail(f"{e}{report}") from e
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
    launch = None
    try:
        launch = Launch(args.harness, args.route, args.prompt, home, work, args.model, args.effort,
                        timeout=args.timeout, settle=args.settle, interrupt_after=args.interrupt_after,
                        crash_after=args.crash_after)
        result = launch.run()
        stdout, stderr = launch.output()
        version = first_line([result["argv"][0], "--version"], launch.env, work)
    finally:
        if not args.keep and not (launch is not None and launch.keep_work):
            shutil.rmtree(work, ignore_errors=True)
    result.update(probe="launch", version=version, droppedNames=dropped_names(), kept=bool(args.keep),
                  stdoutTail=stdout[-4000:], stderrTail=stderr[-4000:], ok=not result["problems"])
    return result


# --- session ----------------------------------------------------------------------

def plan_superseded(plan: dict) -> set:
    """The superseded skill names: each native entry that the plan's switched assertions name absent."""
    names = set()
    for assertion in plan.get("assertions") or []:
        path = assertion.get("path") or ""
        if assertion.get("absent") and os.path.dirname(path) in NATIVE_SKILL_ROOTS:
            names.add(os.path.basename(path))
    return names


def view_skills(view: dict) -> dict:
    """{skill name: [{"source", "path", "craft"}]} for each SKILL.md of the view's search directories."""
    out = {}
    for directory in (view.get("skills") or {}).get("directories") or []:
        base = directory.get("path") or ""
        if not os.path.isdir(base):
            continue
        for name in sorted(os.listdir(base)):
            path = os.path.join(base, name, "SKILL.md")
            if os.path.isfile(path):
                out.setdefault(name, []).append({"source": directory.get("source"), "path": os.path.realpath(path),
                                                 "craft": os.path.basename(base.rstrip("/")) == "craft"})
    return out


def native_copies(home: str, skill: str) -> list:
    """Each native SKILL.md of SKILL in HOME: the user roots and Claude Code's plugin cache."""
    found = [os.path.join(home, root, skill, "SKILL.md") for root in NATIVE_SKILL_ROOTS]
    cache = os.path.join(home, ".claude", "plugins", "cache")
    if os.path.isdir(cache):
        for market in sorted(os.listdir(cache)):
            for plugin in sorted(os.listdir(os.path.join(cache, market)) if os.path.isdir(
                    os.path.join(cache, market)) else []):
                folder = os.path.join(cache, market, plugin)
                for version in sorted(os.listdir(folder)) if os.path.isdir(folder) else []:
                    found.append(os.path.join(folder, version, "skills", skill, "SKILL.md"))
    return [p for p in found if os.path.isfile(p)]


def skill_marks(path: str, skill: str):
    """The `name:` line and the first heading of a SKILL.md's body, or None. A listing of front matter holds only the first."""
    try:
        with open(path, errors="replace") as f:
            text = f.read()
    except OSError:
        return None
    body = text.split("---", 2)[2] if text.startswith("---") else text
    heading = next((line.strip() for line in body.splitlines() if line.startswith("# ")), None)
    return (f"name: {skill}", heading) if heading else None


def mark_reads(desc: dict, files: list) -> None:
    """Add `contentReads` to each event whose result holds both marks of one of FILES, then drop each full result.

    FILES are {"source", "skill", "path", ...}. The record keeps each result only as its digest and length.
    """
    marks = [(f, skill_marks(f["path"], f["skill"])) for f in files]
    for transcript in desc.get("transcripts") or []:
        for event in transcript.get("events") or []:
            result = event.pop("resultFull", None)
            if result is None:
                continue
            found = [dict(f) for f, pair in marks if pair and all(m in result for m in pair)]
            if found:
                event["contentReads"] = found
            event["result"] = {"sha256": hashlib.sha256(result.encode()).hexdigest(), "chars": len(result)}


def main_events(desc: dict) -> list:
    transcript = next((t for t in desc.get("transcripts") or [] if t.get("main")), None)
    return [e for e in (transcript or {}).get("events") or [] if not e.get("sidechain")]


def reads_of(event: dict) -> list:
    """Each classified read of one event: its first class, the rest under `more`, and each content read."""
    first = {k: event.get(k) for k in ("class", "skill", "source", "path")}
    content = [{"class": "content", **{k: c.get(k) for k in ("skill", "source", "path")}}
               for c in event.get("contentReads") or []]
    return [first, *event.get("more", []), *content]


def native_listing(desc: dict) -> list:
    """The skill names that the harness itself listed at startup."""
    names = []
    for item in [t.get("startup") for t in desc.get("transcripts") or [] if t.get("main")] + [desc.get("startup")]:
        item = item or {}
        names += list((item.get("init") or {}).get("skills") or [])
        names += list((item.get("skillListing") or {}).get("names") or [])
    return sorted({n.strip() for n in names if isinstance(n, str) and n.strip()})


def bare(name: str) -> str:
    return name.split(":")[-1]


def quoted(events: list) -> str:
    """The text after the last `QUOTE:` in the agent's own text, or ""."""
    found = ""
    for event in events:
        if event.get("class") == "text":
            for line in str(event.get("target") or "").splitlines():
                at = line.find("QUOTE:")
                if at >= 0:
                    found = line[at + len("QUOTE:"):].strip()
    if len(found) > 2 and found[0] == found[-1] and found[0] in "`\"'":
        found = found[1:-1].strip()
    return found


def quote_in(quote: str, path: str) -> bool:
    """Whether QUOTE is a whole line of the file at PATH, or a long enough part of it."""
    try:
        with open(path, errors="replace") as f:
            text = f.read()
    except OSError:
        return False
    if not quote:
        return False
    return quote in {line.strip() for line in text.splitlines()} or (len(quote) >= QUOTE_MIN_CHARS and quote in text)


def quote_line(path: str) -> str:
    """The line that the probe asks for: the first line of the body that begins with `**`."""
    try:
        with open(path, errors="replace") as f:
            text = f.read()
    except OSError:
        return ""
    body = text.split("---", 2)[2] if text.startswith("---") else text
    return next((line.strip() for line in body.splitlines() if line.startswith("**")), "")


def first_action(events: list):
    return next((e for e in events if e.get("class") == "action"), None)


def mandatory_verdict(events: list, mandatory_skills: set) -> dict:
    """The mandatory file comes before any skill search and any task action. A file of a mandatory skill belongs to that read."""
    first = next((i for i, e in enumerate(events) if e.get("class") == "mandatory"), None)
    if first is None:
        return {"passed": False, "reason": "the agent never read the mandatory file"}
    early = [e for e in events[:first] if e.get("class") in GATED
             and not (e.get("class") == "supporting" and e.get("skill") in mandatory_skills)]
    if early:
        shown = [f"#{e['seq']} {e.get('class')} {str(e.get('target'))[:80]}" for e in early]
        return {"passed": False, "mandatorySeq": events[first]["seq"],
                "reason": f"events before the mandatory read: {shown}"}
    return {"passed": True, "mandatorySeq": events[first]["seq"]}


def craft_verdict(events: list, skills: dict, prompt: str) -> dict:
    """The agent reads a craft skill that the task needs and the prompt does not name, before its first edit."""
    craft = {(c["source"], name) for name, copies in skills.items() for c in copies
             if c["craft"] and name in CRAFT_SKILLS and name not in prompt}
    read = next(((e, r) for e in events for r in reads_of(e)
                 if r.get("class") in SKILL_CLASSES and (r.get("source"), r.get("skill")) in craft), None)
    action = first_action(events)
    verdict = {"skill": read[1].get("skill") if read else None, "source": read[1].get("source") if read else None,
               "readSeq": read[0]["seq"] if read else None, "actionSeq": action["seq"] if action else None}
    if not read:
        reason = f"the agent read none of the craft skills {', '.join(CRAFT_SKILLS)} from its snapshot"
    elif not action:
        reason = "the agent made no edit, so the read precedes no action"
    elif read[0]["seq"] >= action["seq"]:
        reason = "the craft skill was read at or after the first edit"
    else:
        reason = None
    return dict(verdict, passed=reason is None, reason=reason)


def kind_verdict(events: list, skills: dict) -> dict:
    """The agent reads the Grove kind skill from the grove-core snapshot, and quotes a line of that copy."""
    copy = next((c["path"] for c in skills.get(KIND_SKILL, []) if c["source"] == KIND_SOURCE), None)
    read = next((e for e in events for r in reads_of(e) if r.get("class") in SKILL_CLASSES
                 and r.get("skill") == KIND_SKILL and r.get("source") == KIND_SOURCE), None)
    quote = quoted(events)
    verdict = {"skill": KIND_SKILL, "source": KIND_SOURCE, "copy": copy, "readSeq": read["seq"] if read else None,
               "quote": quote, "asked": quote_line(copy) if copy else None}
    if not copy:
        reason = f"the view's {KIND_SOURCE} snapshot holds no {KIND_SKILL}"
    elif not read:
        reason = f"no read of {KIND_SKILL} from the {KIND_SOURCE} snapshot"
    elif not quote_in(quote, copy):
        reason = f"the quoted line {quote[:120]!r} is no line of the {KIND_SOURCE} copy"
    else:
        reason = None
    return dict(verdict, passed=reason is None, reason=reason)


def native_kind_verdict(events: list, home: str) -> dict:
    """The agent reads the Grove kind skill natively, and quotes a line of a native copy."""
    copies = native_copies(home, KIND_SKILL)
    real = {os.path.realpath(p) for p in copies}

    def native_read(event: dict) -> bool:
        if event.get("tool") == "Skill":
            return bare(str(event.get("target") or "")) == KIND_SKILL and not event.get("error")
        paths = [r.get("path") for r in reads_of(event)] + list(event.get("paths") or [])
        return any(p and os.path.realpath(p) in real for p in paths) or any(
            c.get("path") in real for c in event.get("contentReads") or [])

    read = next((e for e in events if native_read(e)), None)
    quote = quoted(events)
    matched = [p for p in copies if quote_in(quote, p)]
    verdict = {"skill": KIND_SKILL, "copies": copies, "readSeq": read["seq"] if read else None,
               "readTool": read.get("tool") if read else None, "quote": quote, "quotedCopies": matched,
               "asked": quote_line(copies[0]) if copies else None}
    if not copies:
        reason = f"HOME holds no native copy of {KIND_SKILL}"
    elif not read:
        reason = f"no native read of {KIND_SKILL}"
    elif not matched:
        reason = f"the quoted line {quote[:120]!r} is no line of a native copy"
    else:
        reason = None
    return dict(verdict, passed=reason is None, reason=reason)


def testanyware_verdict(events: list) -> dict:
    """The agent reads using-testanyware from its source and runs the command reference and doctor."""
    read = next((e for e in events for r in reads_of(e) if r.get("class") in SKILL_CLASSES
                 and r.get("skill") == TESTANYWARE_SKILL and r.get("source") == TESTANYWARE_SOURCE), None)
    runs = {}
    for label, pattern in TESTANYWARE_COMMANDS.items():
        hit = next((e for e in events if e.get("tool") in ht.SHELL_TOOLS and pattern.search(str(e.get("target")))),
                   None)
        runs[label] = hit["seq"] if hit else None
    missing = [label for label, seq in runs.items() if seq is None]
    verdict = {"readSeq": read["seq"] if read else None, "commands": runs}
    if not read:
        reason = f"no read of {TESTANYWARE_SKILL} from the {TESTANYWARE_SOURCE} snapshot"
    elif missing:
        reason = "the agent did not run " + " or ".join(f"testanyware {m}" for m in missing)
    else:
        reason = None
    return dict(verdict, passed=reason is None, reason=reason)


def hooks_verdict(stream: list) -> dict:
    """Claude Code ran the technical-communication SessionStart hook: its response holds the instruction."""
    responses = [e for e in stream if e.get("type") == "system" and e.get("subtype") == "hook_response"]
    shown = [{"event": e.get("hook_event"), "name": e.get("hook_name"), "exit": e.get("exit_code"),
              "outcome": e.get("outcome"), "marked": HOOK_MARK in str(e.get("output") or "")} for e in responses]
    hit = [s for s in shown if s["event"] == "SessionStart" and s["marked"] and s["exit"] == 0]
    reason = None if hit else (f"no SessionStart hook response held {HOOK_MARK!r}; responses: "
                               + json.dumps(shown)[:600])
    return {"responses": shown, "passed": reason is None, "reason": reason}


def listing_verdict(listed: list, superseded: set, route: str) -> dict:
    """The ravel route lists no superseded skill. The native route lists the kind skill."""
    hits = sorted({bare(n) for n in listed} & superseded)
    verdict = {"count": len(listed), "listedSuperseded": hits}
    if not listed:
        reason = "the session shows no native skill listing, so discovery cannot be judged"
    elif route == "ravel" and hits:
        reason = f"native discovery listed superseded skills {hits}"
    elif route == "native" and KIND_SKILL not in {bare(n) for n in listed}:
        reason = f"native discovery did not list {KIND_SKILL}"
    else:
        reason = None
    return dict(verdict, passed=reason is None, reason=reason)


def digest_verdict(view: dict, accepted: dict, mandatory: list) -> dict:
    """The launch's sources are exactly the accepted sources with their digests, and it names the mandatory file."""
    skills = view.get("skills") or {}
    found = {s.get("id"): s.get("resourceDigest") for s in skills.get("sources") or [] if isinstance(s, dict)}
    problems = []
    for sid in sorted(set(found) | set(accepted)):
        want = (accepted.get(sid) or {}).get("resourceDigest")
        if sid not in found:
            problems.append(f"the accepted source {sid} is missing")
        elif want is None:
            problems.append(f"the source {sid} is not an accepted source")
        elif found[sid] != want:
            problems.append(f"the source {sid} gives resourceDigest {found[sid]}, not the accepted {want}")
    listed = {(m.get("source"), os.path.basename(os.path.dirname(m.get("path") or ""))) for m in
              skills.get("mandatory") or [] if isinstance(m, dict)}
    for want in mandatory:
        if (want.get("source"), os.path.basename(os.path.dirname(want.get("path") or ""))) not in listed:
            problems.append(f"the launch names no mandatory file {want.get('source')}:{want.get('path')}")
    return {"sources": found, "passed": not problems, "reason": "; ".join(problems) or None}


def session_verdicts(harness: str, route: str, desc: dict, view: dict, stream: list, plan: dict, home: str,
                     prompt: str) -> dict:
    """Each observation of one session: {name: verdict}. The native route needs no Ravel view."""
    events = main_events(desc)
    superseded = plan_superseded(plan)
    out = {}
    if route == "ravel":
        skills = view_skills(view)
        superseded |= set(skills)
        mandatory_skills = {os.path.basename(os.path.dirname(m.get("path") or ""))
                            for m in (view.get("skills") or {}).get("mandatory") or []}
        out["mandatoryFirst"] = mandatory_verdict(events, mandatory_skills)
        out["craftSkill"] = craft_verdict(events, skills, prompt)
        out["kindSkill"] = kind_verdict(events, skills)
        out["testanyware"] = testanyware_verdict(events)
        if harness == "claude":
            out["hooks"] = hooks_verdict(stream)
        out["listing"] = listing_verdict(native_listing(desc), superseded, route)
        out["digests"] = digest_verdict(view, plan["accepted"]["sources"], plan_mandatory(plan))
    else:
        out["listing"] = listing_verdict(native_listing(desc), superseded, route)
        out["kindSkill"] = native_kind_verdict(events, home)
    return out


def skill_roots(home: str) -> dict:
    """Each ordinary child of the native skill roots as {root: {"entries": {name: identity}, ...}}.

    Only the exact platform-managed subtree of a root is skipped, and only
    while it is a real folder ("Concurrent writers").
    """
    out = {}
    for rel, skipped in COMPARED_ROOTS.items():
        base = os.path.join(home, rel)
        entries, skips = {}, []
        if os.path.isdir(base):
            for name in sorted(os.listdir(base)):
                path = os.path.join(base, name)
                if name == skipped and os.path.isdir(path) and not os.path.islink(path):
                    skips.append(name)
                    continue
                entries[name] = osw.identity(path)
        out[rel] = {"present": os.path.lexists(base),
                    "link": os.readlink(base) if os.path.islink(base) else None,
                    "entries": entries, "skipped": skips}
    return out


def root_changes(before: dict, after: dict) -> list:
    """Each difference between two readings of the native skill roots."""
    changes = []
    for rel in COMPARED_ROOTS:
        b, a = before.get(rel) or {}, after.get(rel) or {}
        if (b.get("present"), b.get("link")) != (a.get("present"), a.get("link")):
            changes.append({"path": rel, "change": "root"})
        old, new = b.get("entries") or {}, a.get("entries") or {}
        for name in sorted(set(old) | set(new)):
            if name not in old:
                changes.append({"path": f"{rel}/{name}", "change": "added"})
            elif name not in new:
                changes.append({"path": f"{rel}/{name}", "change": "removed"})
            elif old[name] != new[name]:
                changes.append({"path": f"{rel}/{name}", "change": "changed"})
    return changes


def file_state(home: str) -> dict:
    """{relative path: (type, size, mtime_ns, inode, mode, link)} below each written-file root of HOME."""
    state = {}

    def add(path: str) -> None:
        try:
            st = os.lstat(path)
        except OSError:
            return
        link = os.readlink(path) if os.path.islink(path) else None
        kind = "link" if link is not None else "dir" if os.path.isdir(path) else "file"
        state[os.path.relpath(path, home)] = (kind, 0 if kind == "dir" else st.st_size,
                                              0 if kind == "dir" else st.st_mtime_ns, st.st_ino, st.st_mode, link)

    for name in os.listdir(home):
        add(os.path.join(home, name))
    for rel in WRITE_ROOTS:
        top = os.path.join(home, rel)
        if not os.path.isdir(top) or os.path.islink(top):
            add(top)
            continue
        for folder, dirs, files in os.walk(top):
            for name in dirs + files:
                add(os.path.join(folder, name))
    return state


def written(before: dict, after: dict) -> dict:
    """Each entry that a session added, changed or removed, and whether a skill copy or agent definition is one."""
    changes = []
    for rel in sorted(set(before) | set(after)):
        if before.get(rel) == after.get(rel):
            continue
        change = "added" if rel not in before else "removed" if rel not in after else "changed"
        kind = (after.get(rel) or before.get(rel))[0]
        changes.append({"path": rel, "change": change, "type": kind})
    skills = [c["path"] for c in changes if any(inside(c["path"], r) for r in NATIVE_SKILL_ROOTS)]
    agents = [c["path"] for c in changes if any(inside(c["path"], r) for r in AGENT_ROOTS)]
    return {"scope": ["each top-level entry of HOME", *WRITE_ROOTS], "changes": changes,
            "skillCopiesChanged": skills, "agentDefinitionsChanged": agents}


def session_prompt(route: str) -> str:
    return RAVEL_PROMPT if route == "ravel" else NATIVE_PROMPT


def compact_events(events: list) -> list:
    """The record of each event, without any result text."""
    keys = ("seq", "tool", "class", "skill", "source", "error")
    return [dict({k: e.get(k) for k in keys if e.get(k) is not None}, target=str(e.get("target") or "")[:160],
                 **({"contentReads": [f"{c.get('source')}:{c.get('skill')}" for c in e["contentReads"]]}
                    if e.get("contentReads") else {})) for e in events]


def session_command(args) -> dict:
    home = probe_home()
    plan = load_plan(args.plan)
    prompt = session_prompt(args.route)
    model, effort = SESSION_DEFAULTS[args.harness]
    roots_before = skill_roots(home)
    files_before = file_state(home)
    work = make_work(home)
    launch = None
    try:
        launch = Launch(args.harness, args.route, prompt, home, work, model, effort, timeout=args.timeout)
        result = launch.run()
        stdout, stderr = launch.output()
        try:
            with open(launch.stream_path(), "rb") as f:
                raw = f.read()
        except OSError:
            raw = b""
        view = {}
        if args.route == "ravel":
            try:
                with open(launch.view_path()) as f:
                    view = json.load(f) or {}
            except (OSError, ValueError):
                view = {}
        stream = ht.read_jsonl_bytes(raw)
        adapter = ht.adapter_named(args.harness)
        desc = adapter.describe("probe", view, {}, stream, hashlib.sha256(raw).hexdigest(), ht.Path(home),
                                ht.Path(work), {}, prompt=prompt)
        files = [dict(c, skill=name) for name, copies in view_skills(view).items() for c in copies]
        files += [{"source": "native", "skill": KIND_SKILL, "path": os.path.realpath(p)}
                  for p in native_copies(home, KIND_SKILL)]
        mark_reads(desc, files)
        observations = session_verdicts(args.harness, args.route, desc, view, stream, plan, home, prompt)
        program = shutil.which(harness_argv(args.harness, model, effort, "")[0], path=launch.env["PATH"])
        version = first_line([program, "--version"], launch.env, work) if program else "unavailable"
    finally:
        if not args.keep and not (launch is not None and launch.keep_work):
            shutil.rmtree(work, ignore_errors=True)
    roots = root_changes(roots_before, skill_roots(home))
    writes = written(files_before, file_state(home))
    problems = list(result["problems"])
    leader = result["leader"]
    if leader["exit"] != 0:
        problems.append(f"the leader exited {leader['exit']}")
    if args.route == "ravel" and stdout.strip().splitlines()[-1:] != ["harness-exit-0"]:
        problems.append(f"Ravel did not end the session with harness-exit-0: {stdout.strip()[-300:]!r}")
    for name, verdict in observations.items():
        if not verdict["passed"]:
            problems.append(f"{name}: {verdict['reason']}")
    for change in roots:
        problems.append(f"the native skill root entry {change['path']} was {change['change']} during the probe")
    return {"probe": "session", "harness": args.harness, "route": args.route, "home": home, "model": model,
            "effort": effort, "harnessVersion": version, "prompt": prompt, "planDigest": plan["_digest"],
            "launch": {k: result[k] for k in ("argv", "leader", "events", "owned", "survivors", "cleanup",
                                              "unattributed", "reparentedWithoutRecord", "journal")},
            "view": {"kind": view.get("kind"), "sources": (observations.get("digests") or {}).get("sources")},
            "sessionId": desc.get("sessionId"),
            "transcripts": [{k: t.get(k) for k in ("file", "sha256", "main")} for t in desc.get("transcripts") or []],
            "events": compact_events(main_events(desc)), "listing": native_listing(desc),
            "observations": observations, "rootChanges": roots,
            "rootsCompared": {rel: {"entries": len(r["entries"]), "skipped": r["skipped"]}
                              for rel, r in roots_before.items()},
            "writes": writes, "work": work, "kept": bool(args.keep), "droppedNames": dropped_names(),
            "stderrTail": stderr[-2000:], "problems": problems, "ok": not problems}


# --- entry ------------------------------------------------------------------------

def parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(prog="owner_switch_probes.py", description=__doc__.split("\n\n")[0])
    sub = p.add_subparsers(dest="command", required=True)
    r = sub.add_parser("resolve", help="ravel skills resolve with HOME's configuration")
    r.add_argument("--expect", choices=("switched", "disabled"), required=True)
    r.add_argument("--plan")
    s = sub.add_parser("session", help="one harness session probe and its observations")
    s.add_argument("--harness", choices=("claude", "codex"), required=True)
    s.add_argument("--route", choices=("ravel", "native"), required=True)
    s.add_argument("--plan")
    s.add_argument("--timeout", type=float, default=LAUNCH_TIMEOUT)
    s.add_argument("--keep", action="store_true", help="keep the work directory")
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
    commands = {"resolve": resolve_command, "session": session_command, "launch": launch_command}
    try:
        result = commands[args.command](args)
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
