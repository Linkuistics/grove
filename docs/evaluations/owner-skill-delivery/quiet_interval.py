#!/usr/bin/env python3
"""Measure the quiet interval of the owner switch beside an idle, signed-in Claude Code parent.

docs/specs/owner-skill-delivery.md owns the rule: "Claude Code's own writes"
and the item of "Test seams" that measures the complete switch beside an idle
signed-in Claude Code parent with the owner's MCP servers. The section
"Quiet interval in staging" of docs/owner-skill-delivery-evidence.md records
each run. owner-staging-hooks-k251 built this tool.

    python3 -I quiet_interval.py prefix  --work W --candidate KEG --ravel PREFIX
    python3 -I quiet_interval.py seed    --home H --work W
    python3 -I quiet_interval.py prepare --home H --work W
    python3 -I quiet_interval.py run     --home H --work W [--settle S] [--model M] [--no-parent]
    python3 -I quiet_interval.py watch   --home H --out FILE
    python3 -I quiet_interval.py analyse --home H --work W
    python3 -I quiet_interval.py controls --out OUT

W is the measurement folder outside every working copy. H is a staging HOME,
never the account's home directory.

prefix. W/prefix stands for $(brew --prefix): bin/ravel is a clone of the
accepted 2.0 build, share/ravel/installation.json names
W/prefix/share/ravel/skills, and that folder links each of the ten sources:
Grove's six in the candidate keg and the four others in their installed
kegs. Cellar/grove/22.8.0 links the candidate keg, and opt/grove links that.
W/bin holds two stubs. `brew` answers only --prefix, --cellar, --version and
`info --json=v2 linkuistics/taps/grove`. `mise` answers only
`-C HOME bootstrap packages status --json`, with each skill formula at its
accepted version. Each other call exits 97. Each call goes to W/stub-calls.log.
The stubs replace the metadata queries of the switch's gates 4 and 5 and
nothing else: resolution, the probes and Claude Code's commands are real.

seed. The native starting state in H. Each entry write of
docs/owner-cutover/switch-plan-draft.json is copied from the account's HOME
into H, following links, so that no new link leads out of H. A dangling link
becomes a link to H/.k251-absent/<its text>, which stays dangling inside H.
APFS clones of ~/.local/bin/codebase-memory-mcp and ~/.quint-llm-kit follow,
because the baseline instrument reads both relative to HOME. The staging
codebase-memory-mcp entry gains CBM_CACHE_DIR, the account daemon's cache
folder, because the daemon refuses a client with another folder. Claude Code's
own commands then register the `linkuistics` and `quint-llm-kit`
marketplaces from GitHub and install the five plugins that the switch
disables. The Ravel settings move to W/settings-staging.json, because native
delivery has no `skills` key and native-off refuses another value.

prepare. Copy mac-check and its helper into H, create the manual, run
native-off, because finish runs it before the switch and the plan's original
of the Ravel settings is therefore the off value, draft the staging plan with `switch_plan_draft.py draft --home H --out W/plan`, and copy
the plan, the program, the probe program, harness_trial.py and inventory.py
into H/.local/state/owner-switch/prepared. The program's `report` must pass.

run. Run native-off and attempt in H. Start `watch`. Start the parent in its
own tmux server (socket k251): the held Claude Code with HOME set to H, in a
new folder under /private/tmp, with only an allow-listed environment, in the
default permission mode with one allow rule for W/run-switch.sh. After
the parent shows its prompt, wait S seconds (default 120), read its MCP
status with /mcp, and record the loaded plugin version directories, the
registrations and the metadata of each watched file. Then type one prompt:
the parent runs W/run-switch.sh with its Bash tool and waits. The script runs
`switch` with PATH starting at W/bin, OWNER_SWITCH_REHEARSAL_SCOPE set to an
empty folder (the inspection cannot admit the parent beside the operator's
own session), OWNER_SWITCH_REHEARSAL_HOOK set to W/bin/mark (each hook point
and its time go to W/marks.log) and OWNER_SWITCH_READERS set to the loaded
version directories. After the switch exits and S/4 more seconds, the parent
ends with /exit, `watch` stops and `analyse` runs. --no-parent runs the same
switch without a parent, for a clone of H.

watch. Until SIGTERM, a kqueue watch on H/.claude, H/.claude/plugins,
H/.config/ravel and the four watched files writes one JSON line per change to
FILE: each entry that appears, disappears or changes inode in a watched
folder, and each watched file whose inode, size, modification time or change
time changed, with its SHA-256, mode, ACL and extended attribute names. Each
new content goes to FILE.blobs/<sha256>. Claude Code names its settings
temporary settings.json.tmp.<pid>.<hex>, so the line keeps that PID with its
ancestry from one process listing. Registry temporaries name no PID.

analyse. Classify each change by the marks and the ancestry. A settings write
whose PID descends from the switch is the program's. The program's own writes
use .<name>.owner-switch-<hex>. A registry write inside a Claude Code command
or a probe is the program's window; one outside every program window is
unsolicited. Write W/record.json and W/summary.md. Exit status: 0 when no
unsolicited write, no lost metadata, no changed loaded version directory or
registration and no new .orphaned_at marker was found and the hooks plugin is
installed at the end; 1 otherwise.

controls. Wrong subjects in a scratch HOME, with no harness: a settings
temporary that this process names with its own PID, a registry written in
place, and a registry replaced without its extended attribute. The recorder
must see each, with the PID's ancestry, and `analyse` must report each and
fail. Exit status: 0 when every control comes back dirty as required.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import select
import shutil
import signal
import subprocess
import sys
import time

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", ".."))
sys.path.insert(0, os.path.join(ROOT, "scripts"))
import owner_switch as osw  # noqa: E402

ACCOUNT_HOME = os.path.realpath(os.path.expanduser("~"))
DRAFT = os.path.join(ROOT, "docs", "owner-cutover", "switch-plan-draft.json")
STAGING_SETTINGS = os.path.join(ROOT, "docs", "owner-cutover", "settings-staging.json")
GENERATOR = os.path.join(os.path.dirname(os.path.abspath(__file__)), "switch_plan_draft.py")
PREPARED_FILES = {
    "owner_switch.py": os.path.join(ROOT, "scripts", "owner_switch.py"),
    "owner_switch_probes.py": os.path.join(ROOT, "scripts", "owner_switch_probes.py"),
    "harness_trial.py": os.path.join(ROOT, "scripts", "harness_trial.py"),
    "inventory.py": os.path.join(os.path.dirname(os.path.abspath(__file__)), "inventory.py"),
}
STATE = ".local/state/owner-switch"
MANUAL = "Library/Mobile Documents/iCloud~md~obsidian/Documents/Brain/Configuration/AI Dev Tooling.md"
WATCHED_NAMES = ("settings.json", "known_marketplaces.json", "installed_plugins.json")
WATCHED = (".claude/settings.json", ".claude/plugins/known_marketplaces.json",
           ".claude/plugins/installed_plugins.json", ".config/ravel/settings.json")
FOLDERS = (".claude", ".claude/plugins", ".config/ravel")
GROVE_SOURCES = ("design", "engineering", "grove-core", "requirements", "research", "writing")
OTHER_SOURCES = {"personal": "personal", "quint": "quint-llm-kit", "superpowers": "superpowers",
                 "testanyware": "testanyware"}
FORMULAE = {"linkuistics/taps/personal": "2026-11-06", "linkuistics/taps/testanyware": "2.8.0",
            "linkuistics/taps/superpowers": "7.0.0", "linkuistics/taps/quint-llm-kit": "1.0.0"}
GROVE_VERSION = "22.8.0"
RAVEL_SHA256 = "3a9a9129573e4d827a317f6b793ba990044080a31a7b667b64cc153a2f974a0b"
MARKETPLACES = ("Linkuistics/grove", "quint-co/quint-llm-kit")
SUPERSEDED = ("grove@linkuistics", "linkuistics@linkuistics", "testanyware@linkuistics",
              "superpowers@claude-plugins-official", "quint-llm-kit@quint-llm-kit")
HOOKS_PLUGIN = "technical-communication-hooks@writing-hooks"
# The baseline instrument reads these tools relative to HOME; the MCP entries name the real paths.
TOOL_COPIES = (".local/bin/codebase-memory-mcp", ".quint-llm-kit")
CBM_CACHE = os.path.join(ACCOUNT_HOME, ".cache", "codebase-memory-mcp")
TMUX = ["tmux", "-L", "k251"]
SETTINGS_TEMP = re.compile(r"^settings\.json\.tmp\.(\d+)\.[0-9a-f]+$")
ENV_KEEP = ("USER", "LOGNAME", "LANG", "LC_ALL", "TMPDIR")


def stop(message: str) -> None:
    sys.exit(f"stop: {message}")


def staging_home(path: str) -> str:
    home = os.path.realpath(path)
    if home == ACCOUNT_HOME or not os.path.isdir(home):
        stop(f"{path} is the account's home directory or no folder")
    return home


def sha256(path: str) -> str:
    with open(path, "rb") as f:
        return hashlib.sha256(f.read()).hexdigest()


def write_json(path: str, value) -> None:
    with open(path, "w") as f:
        json.dump(value, f, indent=2, sort_keys=True)
        f.write("\n")


def base_env(home: str, path: str | None = None) -> dict:
    env = {k: os.environ[k] for k in ENV_KEEP if k in os.environ}
    env.update(HOME=home, PATH=path or os.environ.get("PATH", "/usr/bin:/bin"), TERM="xterm-256color",
               DISABLE_AUTOUPDATER="1")
    return env


def claude_path() -> str:
    real = os.path.realpath(shutil.which("claude") or "")
    if "/claude/versions/" not in real:
        stop(f"claude on PATH is not a versioned Claude Code executable: {real}")
    return real


# --- prefix -------------------------------------------------------------------------

def cmd_prefix(args) -> int:
    work = os.path.realpath(args.work)
    prefix, stubs = os.path.join(work, "prefix"), os.path.join(work, "bin")
    if os.path.exists(prefix) or os.path.exists(stubs):
        stop(f"{prefix} or {stubs} exists")
    keg = os.path.join(os.path.realpath(args.candidate), "brew", "Cellar", "grove", GROVE_VERSION)
    ravel = os.path.join(os.path.realpath(args.ravel), "bin", "ravel")
    if sha256(ravel) != RAVEL_SHA256:
        stop(f"{ravel} is not the accepted 2.0 build")
    skills = os.path.join(prefix, "share", "ravel", "skills")
    for d in (os.path.join(prefix, "bin"), skills, os.path.join(prefix, "Cellar", "grove"),
              os.path.join(prefix, "opt"), os.path.join(work, "scope"), stubs):
        os.makedirs(d)
    subprocess.run(["cp", "-c", "-p", ravel, os.path.join(prefix, "bin", "ravel")], check=True)
    write_json(os.path.join(prefix, "share", "ravel", "installation.json"),
               {"schemaVersion": 1, "skillDirectory": skills})
    for sid in GROVE_SOURCES:
        os.symlink(os.path.join(keg, "share", "ravel", "skills", sid), os.path.join(skills, sid))
    for sid in OTHER_SOURCES:
        os.symlink(os.path.realpath(os.path.join("/opt/homebrew/share/ravel/skills", sid)), os.path.join(skills, sid))
    os.symlink(keg, os.path.join(prefix, "Cellar", "grove", GROVE_VERSION))
    os.symlink(os.path.join("..", "Cellar", "grove", GROVE_VERSION), os.path.join(prefix, "opt", "grove"))
    info = {"formulae": [{"name": "grove", "full_name": "linkuistics/taps/grove",
                          "versions": {"stable": GROVE_VERSION}, "revision": 0,
                          "installed": [{"version": GROVE_VERSION}], "linked_keg": GROVE_VERSION}]}
    write_json(os.path.join(stubs, "brew-info.json"), info)
    packages = [{"package": "linkuistics/taps/grove", "desired_state": "present", "state": "installed",
                 "installed_version": GROVE_VERSION}]
    packages += [{"package": f, "desired_state": "present", "state": "installed", "installed_version": v}
                 for f, v in FORMULAE.items()]
    write_json(os.path.join(stubs, "mise-status.json"), {"brew": {"packages": packages}})
    log = os.path.join(work, "stub-calls.log")
    real_brew = subprocess.run(["brew", "--version"], capture_output=True, text=True).stdout.splitlines()[:1]
    stub = {
        "brew": f"""#!/bin/sh
# The staging stub of brew (quiet_interval.py prefix). It answers only the gates' metadata queries.
printf '%s brew %s\\n' "$(date -u +%FT%TZ)" "$*" >>'{log}'
case "$*" in
  --prefix) echo '{prefix}' ;;
  --cellar) echo '{prefix}/Cellar' ;;
  --version) echo 'Homebrew staging stub of {(real_brew or ["?"])[0]}' ;;
  'info --json=v2 linkuistics/taps/grove') cat '{stubs}/brew-info.json' ;;
  *) echo "staging brew stub: unexpected call: $*" >&2; exit 97 ;;
esac
""",
        "mise": f"""#!/bin/sh
# The staging stub of mise (quiet_interval.py prefix). It answers only gate 5's status query.
printf '%s mise %s\\n' "$(date -u +%FT%TZ)" "$*" >>'{log}'
if [ "$1" = -C ] && [ "$3 $4 $5 $6" = "bootstrap packages status --json" ]; then cat '{stubs}/mise-status.json'; exit 0; fi
echo "staging mise stub: unexpected call: $*" >&2; exit 97
""",
        "mark": f"""#!/bin/sh
# The rehearsal hook of the staging run: the point and its time, and nothing else.
python3 -I -c 'import sys,time; print(time.time_ns(), sys.argv[1])' "$1" >>'{work}/marks.log'
""",
    }
    for name, text in stub.items():
        path = os.path.join(stubs, name)
        with open(path, "w") as f:
            f.write(text)
        os.chmod(path, 0o755)
    print(f"prefix {prefix}\nstubs {stubs}")
    return 0


# --- seed ---------------------------------------------------------------------------

def dangling_target(home: str, src: str) -> str:
    """A link text inside HOME that names nothing, for a dangling link of the account's HOME."""
    text = os.readlink(src) if os.path.islink(src) else src
    return os.path.join(home, ".k251-absent", text.lstrip("/"))


def join_cbm_daemon(home: str) -> None:
    """Point the staging codebase-memory-mcp entry at the account daemon's cache folder.

    The server is one daemon per account. A client whose cache folder differs
    from the running daemon's refuses to start, and a staging HOME gives it
    another folder. Finish's own parent is a client of the same daemon.
    """
    path = os.path.join(home, ".claude.json")
    doc = json.load(open(path))
    entry = doc.get("mcpServers", {}).get("codebase-memory-mcp")
    if entry is not None and entry.get("env", {}).get("CBM_CACHE_DIR") != CBM_CACHE:
        entry.setdefault("env", {})["CBM_CACHE_DIR"] = CBM_CACHE
        tmp = path + ".k251"
        with open(tmp, "w") as f:
            json.dump(doc, f, indent=2)
        shutil.copystat(path, tmp)
        os.rename(tmp, path)


def cmd_seed(args) -> int:
    home, work = staging_home(args.home), os.path.realpath(args.work)
    os.makedirs(work, exist_ok=True)
    plan = json.load(open(DRAFT))
    seeded = []
    for w in (w for w in plan["writes"] if w["type"] == "entry"):
        src, dst = os.path.join(ACCOUNT_HOME, w["path"]), os.path.join(home, w["path"])
        if os.path.lexists(dst):
            stop(f"{dst} exists already")
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        if os.path.exists(src):
            subprocess.run(["cp", "-c", "-R", "-L", "-p", src, dst], check=True)
            kind = "link-copy" if os.path.islink(src) else "copy"
        else:
            # The baseline instrument classifies any dangling link as dangling, whatever its text.
            os.symlink(dangling_target(home, src), dst)
            kind = "dangling"
        seeded.append({"path": w["path"], "kind": kind})
    for rel in TOOL_COPIES:
        dst = os.path.join(home, rel)
        if not os.path.lexists(dst):
            os.makedirs(os.path.dirname(dst), exist_ok=True)
            subprocess.run(["cp", "-c", "-R", "-p", os.path.join(ACCOUNT_HOME, rel), dst], check=True)
            seeded.append({"path": rel, "kind": "tool-copy"})
    join_cbm_daemon(home)
    env = base_env(home)
    commands = [["plugin", "marketplace", "add", m] for m in MARKETPLACES]
    commands += [["plugin", "install", p] for p in SUPERSEDED]
    ran = []
    for c in commands:
        done = subprocess.run([claude_path(), *c], env=env, cwd="/private/tmp", capture_output=True, text=True)
        ran.append({"argv": c, "exit": done.returncode, "output": (done.stdout + done.stderr)[-600:]})
        print(f"claude {' '.join(c)}: exit {done.returncode}")
        if done.returncode != 0:
            print(done.stdout + done.stderr)
            stop("a seeding command failed")
    ravel = os.path.join(home, ".config", "ravel", "settings.json")
    moved = None
    if os.path.exists(ravel):
        moved = os.path.join(work, "settings-staging.json")
        shutil.move(ravel, moved)
    settings = json.load(open(os.path.join(home, ".claude", "settings.json")))
    registry = json.load(open(os.path.join(home, ".claude", "plugins", "installed_plugins.json")))
    plugins = {p: {"enabled": settings.get("enabledPlugins", {}).get(p),
                   "registration": registry["plugins"].get(p)} for p in SUPERSEDED}
    write_json(os.path.join(work, "seed.json"), {"entries": seeded, "commands": ran, "plugins": plugins,
                                                  "ravel_settings_moved_to": moved})
    print(f"seeded {len(seeded)} entries and {len(SUPERSEDED)} plugins; record {work}/seed.json")
    return 0


# --- prepare ------------------------------------------------------------------------

def cmd_prepare(args) -> int:
    home, work = staging_home(args.home), os.path.realpath(args.work)
    for rel in (".local/bin/mac-check", ".local/lib/machine-config/check.py"):
        dst = os.path.join(home, rel)
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        shutil.copy2(os.path.join(ACCOUNT_HOME, rel), dst)
    manual = os.path.join(home, MANUAL)
    if not os.path.exists(manual):
        os.makedirs(os.path.dirname(manual), exist_ok=True)
        with open(manual, "w") as f:
            f.write("# AI Dev Tooling\n\nThe staging copy of the maintained manual, for owner-staging-hooks-k251.\n")
    # Finish runs native-off before the switch, so the planned original of the Ravel settings is the
    # off value that native-off writes, not the absent file of native delivery before promotion.
    subprocess.run(program(home, "native-off", "--home", home), env=switch_env(home, work), check=True)
    subprocess.run([sys.executable, "-I", GENERATOR, "draft", "--home", home, "--out", os.path.join(work, "plan")],
                   check=True)
    prepared = os.path.join(home, STATE, "prepared")
    os.makedirs(prepared, exist_ok=True)
    shutil.copy2(os.path.join(work, "plan", "plan.json"), os.path.join(prepared, "plan.json"))
    for name, src in PREPARED_FILES.items():
        shutil.copy2(src, os.path.join(prepared, name))
    done = subprocess.run([sys.executable, "-I", os.path.join(prepared, "owner_switch.py"), "report", "--plan",
                           os.path.join(prepared, "plan.json"), "--home", home],
                          env=switch_env(home, work), capture_output=True, text=True)
    with open(os.path.join(work, "report.txt"), "w") as f:
        f.write(done.stdout + done.stderr)
    print(f"report: exit {done.returncode}; {work}/report.txt")
    # The switch refuses unless the inventory before and each planned original match the plan.
    blocking = [l for l in done.stdout.splitlines() if l.startswith("inventory before: differs") or "; now differs:" in l]
    for line in blocking:
        print(f"stop: the report shows {line[:300]}")
    return done.returncode or (1 if blocking else 0)


# --- the switch's environment --------------------------------------------------------

def switch_env(home: str, work: str, readers=()) -> dict:
    env = base_env(home, os.path.join(work, "bin") + os.pathsep + os.environ.get("PATH", ""))
    env["OWNER_SWITCH_REHEARSAL_SCOPE"] = os.path.join(work, "scope")
    env["OWNER_SWITCH_REHEARSAL_HOOK"] = os.path.join(work, "bin", "mark")
    if readers:
        env["OWNER_SWITCH_READERS"] = os.pathsep.join(readers)
    return env


def program(home: str, *args) -> list:
    return [sys.executable, "-I", os.path.join(home, STATE, "prepared", "owner_switch.py"), *args]


# --- watch --------------------------------------------------------------------------

def file_state(path: str) -> dict | None:
    try:
        fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
    except OSError:
        return None
    try:
        st = os.fstat(fd)
        data = osw.read_fd(fd)
        acl = osw._acl_text(fd)
        xattrs = sorted(n.decode(errors="replace") for n, _ in osw._xattrs(fd))
    finally:
        os.close(fd)
    return {"inode": st.st_ino, "size": st.st_size, "mtime": st.st_mtime_ns, "ctime": st.st_ctime_ns,
            "mode": oct(st.st_mode & 0o7777), "acl": acl.decode(errors="replace") if acl else "",
            "xattrs": xattrs, "sha256": hashlib.sha256(data).hexdigest(), "_data": data}


def ancestry() -> dict:
    done = subprocess.run(["/bin/ps", "-axww", "-o", "pid=,ppid=,lstart=,comm="], capture_output=True, text=True)
    table = {}
    for line in done.stdout.splitlines():
        parts = line.split(None, 7)
        if len(parts) >= 8:
            table[int(parts[0])] = {"ppid": int(parts[1]), "start": " ".join(parts[2:7]), "comm": parts[7]}
    return table


def chain(table: dict, pid: int) -> list:
    out, seen = [], set()
    while pid in table and pid not in seen and pid > 1:
        seen.add(pid)
        out.append({"pid": pid, "comm": os.path.basename(table[pid]["comm"]), "start": table[pid]["start"]})
        pid = table[pid]["ppid"]
    return out


def cmd_watch(args) -> int:
    home = staging_home(args.home)
    out = open(args.out, "w", buffering=1)
    blobs = args.out + ".blobs"
    os.makedirs(blobs, exist_ok=True)
    stopping = []
    signal.signal(signal.SIGTERM, lambda *_: stopping.append(1))
    kq = select.kqueue()
    folder_fds, file_fds = {}, {}
    for rel in FOLDERS:
        path = os.path.join(home, rel)
        os.makedirs(path, exist_ok=True)
        fd = os.open(path, os.O_RDONLY)
        folder_fds[rel] = fd
        kq.control([select.kevent(fd, filter=select.KQ_FILTER_VNODE, flags=select.KQ_EV_ADD | select.KQ_EV_CLEAR,
                                  fflags=select.KQ_NOTE_WRITE)], 0)
    vflags = (select.KQ_NOTE_WRITE | select.KQ_NOTE_EXTEND | select.KQ_NOTE_ATTRIB | select.KQ_NOTE_DELETE
              | select.KQ_NOTE_RENAME)

    def arm(rel):
        old = file_fds.pop(rel, None)
        if old is not None:
            os.close(old)
        try:
            fd = os.open(os.path.join(home, rel), os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
        except OSError:
            return
        file_fds[rel] = fd
        kq.control([select.kevent(fd, filter=select.KQ_FILTER_VNODE, flags=select.KQ_EV_ADD | select.KQ_EV_CLEAR,
                                  fflags=vflags)], 0)

    def listing(rel):
        names = {}
        try:
            for e in os.scandir(os.path.join(home, rel)):
                try:
                    names[e.name] = e.inode()
                except OSError:
                    pass
        except OSError:
            pass
        return names

    def emit(kind, **fields):
        out.write(json.dumps(dict(time=time.time_ns(), kind=kind, **fields), sort_keys=True) + "\n")

    def keep(state):
        if state is None:
            return None
        data = state.pop("_data")
        blob = os.path.join(blobs, state["sha256"])
        if not os.path.exists(blob):
            with open(blob, "wb") as f:
                f.write(data)
        return state

    dirs = {rel: listing(rel) for rel in FOLDERS}
    files = {}
    for rel in WATCHED:
        files[rel] = keep(file_state(os.path.join(home, rel)))
        arm(rel)
    emit("start", folders=dirs, files=files, pid=os.getpid())
    print("watch: ready", flush=True)
    while not stopping:
        try:
            events = kq.control(None, 16, 0.2)
        except InterruptedError:
            continue
        if not events:
            continue
        for rel in FOLDERS:
            cur = listing(rel)
            added = {n: i for n, i in cur.items() if n not in dirs[rel]}
            gone = sorted(n for n in dirs[rel] if n not in cur)
            changed = {n: [dirs[rel][n], i] for n, i in cur.items() if n in dirs[rel] and dirs[rel][n] != i}
            if added or gone or changed:
                writers = {}
                for name in added:
                    m = SETTINGS_TEMP.match(name)
                    if m:
                        writers[name] = int(m.group(1))
                procs = chain_table = None
                if writers:
                    chain_table = ancestry()
                    procs = {n: chain(chain_table, p) for n, p in writers.items()}
                emit("folder", folder=rel, added=added, gone=gone, changed=changed, writers=procs)
            dirs[rel] = cur
        for rel in WATCHED:
            state = file_state(os.path.join(home, rel))
            old = files[rel]
            key = None if state is None else (state["inode"], state["size"], state["mtime"], state["ctime"])
            okey = None if old is None else (old["inode"], old["size"], old["mtime"], old["ctime"])
            if key != okey:
                state = keep(state)
                emit("file", path=rel, before=old, after=state)
                files[rel] = state
                if state is None or old is None or state["inode"] != old["inode"]:
                    arm(rel)
    emit("end", files=files)
    return 0


# --- snapshots ----------------------------------------------------------------------

def registry(home: str) -> dict:
    try:
        return json.load(open(os.path.join(home, ".claude/plugins/installed_plugins.json")))
    except (OSError, ValueError):
        return {}


def loaded_dirs(home: str, pid: int) -> list:
    """Each plugin version directory that holds the parent's .in_use/<pid> marker."""
    out = []
    cache = os.path.join(home, ".claude", "plugins", "cache")
    for market in sorted(os.listdir(cache)) if os.path.isdir(cache) else []:
        for plugin in sorted(os.listdir(os.path.join(cache, market))):
            base = os.path.join(cache, market, plugin)
            for version in sorted(os.listdir(base)) if os.path.isdir(base) else []:
                d = os.path.join(base, version)
                if os.path.isdir(os.path.join(d, ".in_use")) and str(pid) in os.listdir(os.path.join(d, ".in_use")):
                    out.append(d)
    return out


def snapshot(home: str, parent: int | None, dirs: list) -> dict:
    reg = registry(home).get("plugins", {})
    cache = os.path.join(home, ".claude", "plugins", "cache")
    orphans = []
    for top, subdirs, filenames in os.walk(cache):
        if ".orphaned_at" in filenames or ".orphaned_at" in subdirs:
            orphans.append(os.path.join(top, ".orphaned_at"))
    by_path = {}
    for pid, entries in reg.items():
        for e in entries if isinstance(entries, list) else []:
            if isinstance(e, dict) and isinstance(e.get("installPath"), str):
                by_path[os.path.realpath(e["installPath"])] = {"plugin": pid, "entry": e}
    dirs_state = {}
    for d in dirs:
        markers = sorted(os.listdir(os.path.join(d, ".in_use"))) if os.path.isdir(os.path.join(d, ".in_use")) else []
        dirs_state[d] = {"identity": osw.identity(d, skip=(osw.READER_MARKERS,)), "in_use": markers,
                         "registration": by_path.get(os.path.realpath(d))}
    files = {}
    for rel in WATCHED:
        st = file_state(os.path.join(home, rel))
        if st:
            st.pop("_data")
        files[rel] = st
    known = {}
    try:
        known = json.load(open(os.path.join(home, ".claude/plugins/known_marketplaces.json")))
    except (OSError, ValueError):
        pass
    return {"time": time.time_ns(), "parent": parent, "dirs": dirs_state, "orphans": orphans, "files": files,
            "known_marketplaces": known, "hooks_registration": reg.get(HOOKS_PLUGIN)}


def subtree(pid: int) -> list:
    table = ancestry()
    kids = {}
    for p, row in table.items():
        kids.setdefault(row["ppid"], []).append(p)
    out, todo = [], [pid]
    while todo:
        p = todo.pop()
        if p in table:
            argv = subprocess.run(["/bin/ps", "-o", "args=", "-p", str(p)], capture_output=True, text=True).stdout
            out.append({"pid": p, "ppid": table[p]["ppid"], "start": table[p]["start"], "args": argv.strip()[:300]})
        todo += kids.get(p, [])
    return out


# --- run ----------------------------------------------------------------------------

def trust_folder(home: str, folder: str) -> None:
    """Record the folder trust of the parent's work folder, as accepting the prompt would.

    Claude Code 2.1.322 shows the prompt with "No, exit" selected.
    """
    path = os.path.join(home, ".claude.json")
    doc = json.load(open(path))
    doc.setdefault("projects", {}).setdefault(folder, {})["hasTrustDialogAccepted"] = True
    tmp = path + ".k251"
    with open(tmp, "w") as f:
        json.dump(doc, f, indent=2)
    shutil.copystat(path, tmp)
    os.rename(tmp, path)


def tmux(*args, check=True) -> str:
    return subprocess.run(TMUX + list(args), capture_output=True, text=True, check=check).stdout


def pane() -> str:
    return tmux("capture-pane", "-p", "-t", "parent", check=False)


def wait_for(predicate, timeout: float, what: str, step: float = 1.0):
    end = time.time() + timeout
    while time.time() < end:
        got = predicate()
        if got:
            return got
        time.sleep(step)
    stop(f"timed out waiting for {what}")


def say(work: str, text: str) -> None:
    line = f"{time.strftime('%H:%M:%S')} {text}"
    print(line, flush=True)
    with open(os.path.join(work, "run.log"), "a") as f:
        f.write(line + "\n")


def cmd_run(args) -> int:
    home, work = staging_home(args.home), os.path.realpath(args.work)
    prepared = os.path.join(home, STATE, "prepared")
    plan = os.path.join(prepared, "plan.json")
    if not os.path.exists(plan) or os.path.exists(os.path.join(work, "watch.jsonl")):
        stop("prepare first, in a work folder without an earlier run")
    held = claude_path()
    env = switch_env(home, work)
    for name, argv in (("native-off", program(home, "native-off", "--home", home)),
                       ("attempt", program(home, "attempt", "--plan", plan, "--home", home))):
        done = subprocess.run(argv, env=env, capture_output=True, text=True)
        with open(os.path.join(work, f"{name}.log"), "w") as f:
            f.write(done.stdout + done.stderr)
        say(work, f"{name}: exit {done.returncode}")
        if done.returncode != 0:
            print(done.stdout + done.stderr)
            stop(f"{name} failed")
    attempts = os.path.join(home, STATE, "attempts")
    note = max((os.path.join(attempts, n) for n in os.listdir(attempts)), key=os.path.getmtime)
    watch = subprocess.Popen([sys.executable, "-I", os.path.abspath(__file__), "watch", "--home", home,
                              "--out", os.path.join(work, "watch.jsonl")], stdout=subprocess.PIPE, text=True)
    if watch.stdout.readline().strip() != "watch: ready":
        stop("the watcher did not start")
    say(work, f"watch: pid {watch.pid}")
    parent = None
    meta = {"claude": held, "claude_version": subprocess.run([held, "--version"], capture_output=True,
                                                              text=True).stdout.strip(),
            "note": note, "model": args.model, "settle": args.settle}
    try:
        readers, guidance = [], []
        if not args.no_parent:
            parent_work = os.path.join("/private/tmp", f"k251-parent-{os.getpid()}")
            os.makedirs(parent_work)
            trust_folder(home, os.path.realpath(parent_work))
            penv = base_env(home)
            penv.update(BASH_DEFAULT_TIMEOUT_MS="3600000", BASH_MAX_TIMEOUT_MS="3600000")
            launcher = os.path.join(work, "parent.sh")
            # Default permission mode: one allow rule admits only the switch script, and nothing else.
            with open(launcher, "w") as f:
                f.write("#!/bin/sh\ncd '%s' || exit 1\nexec /usr/bin/env -i %s '%s' --model '%s' --effort low "
                        "--permission-mode default --allowedTools 'Bash(/bin/sh %s)'\n" % (
                            parent_work, " ".join(f"{k}='{v}'" for k, v in sorted(penv.items())), held, args.model,
                            os.path.join(work, "run-switch.sh")))
            os.chmod(launcher, 0o755)
            tmux("new-session", "-d", "-s", "parent", "-x", "200", "-y", "60", launcher)
            parent = int(tmux("list-panes", "-t", "parent", "-F", "#{pane_pid}").split()[0])
            meta.update(parent=parent, parent_work=parent_work, parent_env=sorted(penv))
            say(work, f"parent: pid {parent} in {parent_work}")
            for _ in range(30):
                text = pane()
                if "trust this folder" in text:
                    stop("the parent shows the folder trust prompt, whose default answer exits")
                # The prompt line, or the footer of the default or the vim editor mode.
                if "? for shortcuts" in text or "-- INSERT --" in text or re.search(r"(?m)^❯ ", text):
                    break
                time.sleep(2)
            else:
                stop("the parent showed no prompt within 60 seconds")
            with open(os.path.join(work, "pane-start.txt"), "w") as f:
                f.write(pane())
            say(work, f"parent: prompt shown; settling {args.settle} s")
            time.sleep(args.settle)
            tmux("send-keys", "-t", "parent", "/mcp")
            time.sleep(1)
            tmux("send-keys", "-t", "parent", "Enter")
            time.sleep(4)
            with open(os.path.join(work, "pane-mcp.txt"), "w") as f:
                f.write(pane())
            tmux("send-keys", "-t", "parent", "Escape")
            time.sleep(2)
            readers = loaded_dirs(home, parent)
            for d in readers:
                cand = os.path.join(d, "skills", "grove-finish", "SKILL.md")
                if os.path.exists(cand):
                    guidance.append(cand)
            meta.update(parent_tree_before=subtree(parent), readers=readers, guidance=guidance)
        write_json(os.path.join(work, "before.json"), snapshot(home, parent, readers))
        script = os.path.join(work, "run-switch.sh")
        senv = switch_env(home, work, readers)
        argv = program(home, "switch", "--plan", plan, "--attempt", note, "--home", home)
        for g in guidance:
            argv += ["--guidance", g]
        with open(script, "w") as f:
            f.write("#!/bin/sh\necho $$ >'%s/switch.pid'\n/usr/bin/env -i %s %s >'%s/switch.log' 2>&1\n"
                    "status=$?\necho $status >'%s/switch.exit'\necho \"switch exit $status\"\n" % (
                        work, " ".join(f"{k}='{v}'" for k, v in sorted(senv.items())),
                        " ".join(f"'{a}'" for a in argv), work, work))
        os.chmod(script, 0o755)
        start = time.time_ns()
        if args.no_parent:
            subprocess.run(["/bin/sh", script])
        else:
            prompt = (f"Run exactly this one command with the Bash tool, with timeout 3600000, and wait for it: "
                      f"`/bin/sh {script}` Run nothing else, read no file, and then reply with the last line "
                      f"that it printed")
            tmux("send-keys", "-t", "parent", "-l", prompt)
            time.sleep(1)
            tmux("send-keys", "-t", "parent", "Enter")
            say(work, "parent: prompt sent")
            wait_for(lambda: os.path.exists(os.path.join(work, "switch.exit")), 3600, "the switch", 5)
            say(work, f"switch: exit {open(os.path.join(work, 'switch.exit')).read().strip()}")
            time.sleep(max(15, args.settle / 4))
            with open(os.path.join(work, "pane-end.txt"), "w") as f:
                f.write(pane())
            meta["parent_tree_after"] = subtree(parent)
        meta["switch_started"], meta["switch_ended"] = start, time.time_ns()
        write_json(os.path.join(work, "after.json"), snapshot(home, parent, readers))
        if parent:
            tmux("send-keys", "-t", "parent", "/exit")
            time.sleep(1)
            tmux("send-keys", "-t", "parent", "Enter")
            for _ in range(30):
                if subprocess.run(["kill", "-0", str(parent)], capture_output=True).returncode != 0:
                    break
                time.sleep(1)
            meta["parent_ended"] = time.time_ns()
            say(work, "parent: ended")
    finally:
        tmux("kill-server", check=False)
        time.sleep(1)
        watch.send_signal(signal.SIGTERM)
        watch.wait(timeout=30)
        write_json(os.path.join(work, "meta.json"), meta)
    return cmd_analyse(args)


# --- analyse ------------------------------------------------------------------------

def marks(work: str) -> list:
    path = os.path.join(work, "marks.log")
    out = []
    for line in open(path) if os.path.exists(path) else []:
        t, point = line.rstrip("\n").split(" ", 1)
        out.append((int(t), point))
    return out


def segment(ms: list, t: int) -> str:
    label = "before the switch"
    for when, point in ms:
        if when <= t:
            label = point
    return label


def program_window(label: str) -> bool:
    return label.startswith(("hooks:before:", "command:", "probes", "write:", "deactivate:", "gate:",
                             "receipt:", "manual:", "backup:"))


def cmd_analyse(args) -> int:
    home, work = staging_home(args.home), os.path.realpath(args.work)
    meta = json.load(open(os.path.join(work, "meta.json")))
    ms = marks(work)
    switch_pid = int(open(os.path.join(work, "switch.pid")).read()) if os.path.exists(
        os.path.join(work, "switch.pid")) else None
    parent = meta.get("parent")
    lines = [json.loads(l) for l in open(os.path.join(work, "watch.jsonl"))]
    # The interval: from the switch's first hook point to its last. The parent is idle before and after
    # it too, so a settings or registry write by anything but the program fails at any time.
    points = dict((p, t) for t, p in reversed(ms))
    start = points.get("switch:start", meta["switch_started"])
    end = max([t for t, p in ms if p == "switch:final"] or [meta["switch_ended"]])
    writes, others, problems = [], [], []
    for ev in lines:
        if ev["kind"] != "folder":
            continue
        for name in ev["added"]:
            phase = "interval" if start <= ev["time"] <= end else ("before" if ev["time"] < start else "after")
            label = segment(ms, ev["time"])
            watched = name.lstrip(".").startswith(WATCHED_NAMES)
            if not watched or name.endswith(".lock"):
                others.append({"time": ev["time"], "folder": ev["folder"], "entry": name, "phase": phase,
                               "segment": label, "kind": "lock" if name.endswith(".lock") else "other entry"})
                continue
            pids = (ev.get("writers") or {}).get(name)
            if name.startswith(".") and ".owner-switch-" in name:
                who = "program (its own write)"
            elif pids is not None:
                seq = [p["pid"] for p in pids]
                if switch_pid and switch_pid in seq:
                    who = "program (a command: " + " < ".join(p["comm"] for p in pids[:4]) + ")"
                elif parent and parent in seq:
                    who = "parent or its servers"
                else:
                    who = "unattributed (pid %s)" % (seq[0] if seq else "gone")
            else:
                who = "program window" if phase == "interval" and program_window(label) else "outside a window"
            writes.append({"time": ev["time"], "folder": ev["folder"], "entry": name, "phase": phase,
                           "segment": label, "writer": who})
            if not who.startswith("program"):
                problems.append(f"unsolicited write: {ev['folder']}/{name} during {label} ({who})")
    file_events = [ev for ev in lines if ev["kind"] == "file"]
    renames = []
    for ev in file_events:
        b, a = ev["before"], ev["after"]
        kind = ("created" if b is None else "removed" if a is None else
                "replaced" if a["inode"] != b["inode"] else "in place" if a["sha256"] != b["sha256"] else "metadata")
        renames.append({"time": ev["time"], "path": ev["path"], "kind": kind, "segment": segment(ms, ev["time"]),
                        "sha256": a and a["sha256"], "xattrs_before": b and b["xattrs"],
                        "xattrs_after": a and a["xattrs"], "acl_before": b and b["acl"], "acl_after": a and a["acl"]})
        if kind != "metadata" and not (start <= ev["time"] <= end and program_window(segment(ms, ev["time"]))):
            problems.append(f"{kind} write of {ev['path']} outside a program window")
        if b and a and (set(b["xattrs"]) - set(a["xattrs"]) - {"com.apple.provenance"} or (b["acl"] and not a["acl"])):
            problems.append(f"{ev['path']} lost metadata at {segment(ms, ev['time'])}")
    before = json.load(open(os.path.join(work, "before.json")))
    after = json.load(open(os.path.join(work, "after.json")))
    loaded = []
    for d, b in before["dirs"].items():
        a = after["dirs"].get(d)
        same = a is not None and a["identity"] == b["identity"] and a["registration"] == b["registration"]
        loaded.append({"dir": d, "identity_before": b["identity"], "identity_after": a and a["identity"],
                       "registration_same": a is not None and a["registration"] == b["registration"],
                       "in_use_before": b["in_use"], "in_use_after": a and a["in_use"]})
        if not same:
            problems.append(f"the loaded version directory {d} or its registration changed")
    registered = {os.path.realpath(e["installPath"]) for es in registry(home).get("plugins", {}).values()
                  if isinstance(es, list) for e in es if isinstance(e, dict) and isinstance(e.get("installPath"), str)}
    orphans = []
    for marker in after["orphans"]:
        d = os.path.dirname(marker)
        st = os.lstat(marker) if os.path.lexists(marker) else None
        row = {"marker": marker, "before": marker in before["orphans"], "loaded": d in before["dirs"],
               "registered": os.path.realpath(d) in registered,
               "mtime": st and st.st_mtime_ns, "segment": st and segment(ms, st.st_mtime_ns),
               "content": open(marker).read().strip() if st and os.path.isfile(marker) else None}
        orphans.append(row)
        if not row["before"] or row["loaded"] or row["registered"]:
            problems.append(f"an orphan marker appeared or lies in a loaded or registered directory: {marker}")
    if not after.get("hooks_registration"):
        problems.append(f"{HOOKS_PLUGIN} is not installed at the end")
    exit_code = open(os.path.join(work, "switch.exit")).read().strip() if os.path.exists(
        os.path.join(work, "switch.exit")) else "none"
    if exit_code != "0":
        problems.append(f"the switch exited {exit_code}")
    record = {"schema": "owner-staging-quiet-interval/1", "home": home, "meta": meta, "switch_exit": exit_code,
              "interval": [start, end], "marks": ms, "temporaries": writes, "other_entries": others,
              "file_changes": renames, "loaded": loaded, "orphans": orphans, "metadata_before": before["files"], "metadata_after": after["files"],
              "hooks_registration": after.get("hooks_registration"), "problems": problems,
              "tool_sha256": sha256(os.path.abspath(__file__)),
              "run_tool_sha256": open(os.path.join(work, "run-tool.sha256")).read().split()[0]
              if os.path.exists(os.path.join(work, "run-tool.sha256")) else sha256(os.path.abspath(__file__))}
    write_json(os.path.join(work, "record.json"), record)
    with open(os.path.join(work, "summary.md"), "w") as f:
        f.write(f"# Quiet interval\n\nClaude Code {meta.get('claude_version')}; parent {parent}; switch exit "
                f"{exit_code}\n\n## Temporaries\n\n")
        for w in writes:
            f.write(f"- {w['phase']} {w['segment']}: {w['folder']}/{w['entry']} by {w['writer']}\n")
        f.write("\n## Locks and other entries\n\n")
        for o in others:
            f.write(f"- {o['phase']} {o['segment']}: {o['folder']}/{o['entry']} ({o['kind']})\n")
        f.write("\n## Orphan markers\n\n")
        for o in orphans:
            f.write(f"- {o['marker']}: before {o['before']}, loaded {o['loaded']}, registered {o['registered']}, "
                    f"last written during {o['segment']}\n")
        f.write("\n## File changes\n\n")
        for r in renames:
            f.write(f"- {r['segment']}: {r['path']} {r['kind']}\n")
        f.write("\n## Loaded version directories\n\n")
        for l in loaded:
            f.write(f"- {l['dir']}: identity {'same' if l['identity_before'] == l['identity_after'] else 'CHANGED'}, "
                    f"registration {'same' if l['registration_same'] else 'CHANGED'}\n")
        f.write("\n## Problems\n\n" + ("".join(f"- {p}\n" for p in problems) or "none\n"))
    print(open(os.path.join(work, "summary.md")).read())
    return 0 if not problems else 1


# --- controls -----------------------------------------------------------------------

def cmd_controls(args) -> int:
    """Wrong subjects that the recorder must see and that `analyse` must report. No harness runs."""
    out = os.path.realpath(args.out)
    if os.path.exists(out):
        stop(f"{out} exists")
    home = os.path.join(out, "home")
    for rel in FOLDERS:
        os.makedirs(os.path.join(home, rel))
    for rel in WATCHED:
        with open(os.path.join(home, rel), "w") as f:
            f.write("{}\n")
    subprocess.run(["/usr/bin/xattr", "-w", "user.k251", "1", os.path.join(home, WATCHED[1])], check=True)
    watch = subprocess.Popen([sys.executable, "-I", os.path.abspath(__file__), "watch", "--home", home,
                              "--out", os.path.join(out, "watch.jsonl")], stdout=subprocess.PIPE, text=True)
    if watch.stdout.readline().strip() != "watch: ready":
        stop("the watcher did not start")
    t0 = time.time_ns()
    time.sleep(0.3)
    # 1. A settings write by a process outside the switch, named as Claude Code names its temporaries.
    claude_dir = os.path.join(home, ".claude")
    tmp = os.path.join(claude_dir, f"settings.json.tmp.{os.getpid()}.c0ffee")
    with open(tmp, "w") as f:
        f.write('{"model": "outside"}\n')
    time.sleep(0.05)
    os.rename(tmp, os.path.join(claude_dir, "settings.json"))
    time.sleep(0.3)
    # 2. A registry write in place.
    with open(os.path.join(home, WATCHED[2]), "a") as f:
        f.write(" ")
    time.sleep(0.3)
    # 3. A registry replaced by a file without its extended attribute.
    tmp = os.path.join(home, WATCHED[1] + ".tmp.0badf00d")
    with open(tmp, "w") as f:
        f.write('{"x": 1}\n')
    os.rename(tmp, os.path.join(home, WATCHED[1]))
    time.sleep(0.3)
    t1 = time.time_ns()
    watch.send_signal(signal.SIGTERM)
    watch.wait(timeout=30)
    with open(os.path.join(out, "marks.log"), "w") as f:
        f.write(f"{t0} switch:start\n{t1} switch:final\n")
    with open(os.path.join(out, "switch.pid"), "w") as f:
        f.write("1\n")
    with open(os.path.join(out, "switch.exit"), "w") as f:
        f.write("0\n")
    empty = {"dirs": {}, "orphans": [], "files": {}, "hooks_registration": {"control": True}}
    write_json(os.path.join(out, "before.json"), empty)
    write_json(os.path.join(out, "after.json"), empty)
    write_json(os.path.join(out, "meta.json"), {"switch_started": t0, "switch_ended": t1, "parent": None})
    with contextlib_redirect(os.path.join(out, "analyse.txt")):
        code = cmd_analyse(argparse.Namespace(home=home, work=out))
    record = json.load(open(os.path.join(out, "record.json")))
    wanted = {
        "settings temporary with the writer's PID": any(
            w["entry"].startswith("settings.json.tmp.") and w["writer"].startswith("unattributed (pid")
            for w in record["temporaries"]),
        "unsolicited settings write reported": any("unsolicited write: .claude/settings.json.tmp." in p
                                                   for p in record["problems"]),
        "in-place registry write reported": any("in place write of .claude/plugins/installed_plugins.json" in p
                                                for p in record["problems"]),
        "lost extended attribute reported": any("known_marketplaces.json lost metadata" in p
                                                for p in record["problems"]),
        "analyse fails": code == 1,
    }
    for name, ok in wanted.items():
        print(f"{'PASS' if ok else 'FAIL'} {name}")
    write_json(os.path.join(out, "controls.json"), wanted)
    return 0 if all(wanted.values()) else 1


class contextlib_redirect:
    """Send standard output to a file for the length of a block."""

    def __init__(self, path):
        self.path = path

    def __enter__(self):
        self.file, self.saved = open(self.path, "w"), sys.stdout
        sys.stdout = self.file

    def __exit__(self, *exc):
        sys.stdout = self.saved
        self.file.close()


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n", 1)[0])
    sub = parser.add_subparsers(dest="command", required=True)
    p = sub.add_parser("prefix")
    p.add_argument("--work", required=True)
    p.add_argument("--candidate", required=True)
    p.add_argument("--ravel", required=True)
    for name in ("seed", "prepare", "analyse"):
        p = sub.add_parser(name)
        p.add_argument("--home", required=True)
        p.add_argument("--work", required=True)
    p = sub.add_parser("run")
    p.add_argument("--home", required=True)
    p.add_argument("--work", required=True)
    p.add_argument("--settle", type=float, default=120)
    p.add_argument("--model", default="claude-opus-5-5")
    p.add_argument("--no-parent", action="store_true")
    p = sub.add_parser("watch")
    p.add_argument("--home", required=True)
    p.add_argument("--out", required=True)
    sub.add_parser("controls").add_argument("--out", required=True)
    args = parser.parse_args()
    return {"prefix": cmd_prefix, "seed": cmd_seed, "prepare": cmd_prepare, "run": cmd_run, "watch": cmd_watch,
            "analyse": cmd_analyse, "controls": cmd_controls}[args.command](args)


if __name__ == "__main__":
    sys.exit(main())
