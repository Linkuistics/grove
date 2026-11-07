"""Draft the owner's real switch plan, and check it in a disposable layout.

The plan format is schema "owner-switch-plan/1" in the docstring of
scripts/owner_switch.py. docs/specs/owner-skill-delivery.md owns each
disposition: "Classes and dispositions", "Native residue", "Name collisions
and stale copies", "The switch inside finish", "Concurrent writers" and
"Rollback". "Owner switch plan draft" of docs/owner-skill-delivery-evidence.md
states each choice of this draft.

    python3 -I switch_plan_draft.py draft
    python3 -I switch_plan_draft.py check --out OUT

`draft` reads the real HOME and changes nothing there. It classifies each
child of the five discovery roots by its row in
docs/owner-skill-inventory-reference.tsv. A child whose class is owner-pack,
third-party-pack, stale-copy or dangling, with a disposition that starts with
"deactivate", becomes an entry part. Each other child must have a row with a
kept class, or be a container whose children have such rows. A child without
a row stops the draft. Each entry carries the change identity that
`owner_switch.py identity` prints. The accepted sources come from the table
"The ten Ravel sources" of docs/repository-moves-record.md, and they are
provisional. The script writes docs/owner-cutover/switch-plan-draft.json and
the part table docs/owner-cutover/switch-plan-parts.tsv.

`check` builds a disposable HOME in OUT/home from the draft. It copies each
entry that a part or an `outside` item names from the real HOME, and stops
unless each copy has the draft's change identity. It writes both settings
files with each cell at its earlier value. It then runs the program's loader
and `run --dry-run`, and five controls that must fail. Last, it lists the
real process tree and the job listing against the plan's patterns, with the
program's own classification of kept processes. OUT holds record.json,
dry-run.txt and processes.txt. OUT must not exist. The real HOME is read only.
"""

import argparse, contextlib, copy, hashlib, importlib.util, io, json, os, re, shutil, subprocess, sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", ".."))
PROGRAM = os.path.join(ROOT, "scripts", "owner_switch.py")
DRAFTS = os.path.join(ROOT, "docs", "owner-cutover")
PLAN_OUT = os.path.join(DRAFTS, "switch-plan-draft.json")
TABLE_OUT = os.path.join(DRAFTS, "switch-plan-parts.tsv")
INVENTORY = os.path.join(ROOT, "docs", "owner-skill-inventory-reference.tsv")
MOVES_RECORD = os.path.join(ROOT, "docs", "repository-moves-record.md")
HOME = os.path.realpath(os.path.expanduser("~"))

# The five discovery roots whose superseded children the switch moves.
ROOTS = (".claude/skills", ".codex/skills", ".agents/skills", ".pi/agent/skills", ".pi/agent/extensions")
MOVED_CLASSES = ("owner-pack", "third-party-pack", "stale-copy", "dangling")
KEPT_CLASSES = ("tool-managed", "platform-managed")
# Containers whose children have inventory rows of their own.
CONTAINERS = {".claude/skills/synced": "platform-managed", ".codex/skills/.system": "platform-managed"}

STATE = ".local/state/owner-switch"
PREPARED = STATE + "/prepared"
MANUAL = "Library/Mobile Documents/iCloud~md~obsidian/Documents/Brain/Configuration/AI Dev Tooling.md"
CLAUDE_SETTINGS = ".claude/settings.json"
RAVEL_SETTINGS = ".config/ravel/settings.json"

# "Classes and dispositions" and "Native residue": the plugins that the switch
# disables, and the two skill-free plugins that it enables.
DISABLED = ("grove@linkuistics", "linkuistics@linkuistics", "testanyware@linkuistics",
            "superpowers@claude-plugins-official", "quint-llm-kit@quint-llm-kit")
# The hooks prelude installs the first disabled. owner-pack-checkouts-k93
# installs the second disabled. Both are false before step 9.
ENABLED = ("technical-communication-hooks@writing-hooks", "guardrail@personal")

# "outside": native residue that must stay unchanged. No root that a platform
# writes, because a probe session can rewrite .claude/skills/synced and
# .codex/skills/.system, and step 14 would count that as a failed probe. No
# plugin version directory, because each session writes .in_use/<pid> there.
OUTSIDE_PATHS = (".claude/skills/codebase-memory", ".codex/skills/codebase-memory", ".agents/.grove")
OUTSIDE_ROOTS = (".agents/skills", ".pi/agent/skills", ".pi/agent/extensions", ".claude/agents", ".codex/agents")

# Step 1 and "Concurrent writers". A pattern matches the arguments of a process.
HARNESS = [r"^(\S*/)?claude(\s|$)"]
MCP = [
    # quint-kb: `node .../kb/dist/server.js`, from ~/.claude.json.
    r"^(\S*/)?node \S*/\.quint-llm-kit/repo/mcp-servers/kb/dist/server\.js$",
    # codebase-memory-mcp and its daemon child `--cbm-daemon-internal`.
    r"^(\S*/)?codebase-memory-mcp(\s|$)",
    # Playwright: `npx @playwright/mcp@latest` runs as `npm exec ...`, which
    # launches `node .../node_modules/.bin/playwright-mcp`.
    r"^npm exec @playwright/mcp@latest\s*$",
    r"^(\S*/)?npx (-y )?@playwright/mcp@latest\s*$",
    r"^(\S*/)?node \S*/node_modules/\.bin/playwright-mcp(\s|$)",
    # Not a server: Claude Code keeps the machine awake through this child.
    # The design's kept processes do not name it, and this field is the only
    # one that keeps a child of the harness process.
    r"^caffeinate -i -t \d+$",
]
WATCH = [
    r"^(\S*/)?(claude|codex|pi|grove|grove-llm|ravel|herdr|herdr-core|orca|mise|codebase-memory-mcp)(\s|$)",
    r"(^|\s)\S*/(@anthropic-ai/claude-code|@openai/codex|@mariozechner/pi-coding-agent)/",
    r"^/Applications/(Claude|ChatGPT|Herdr|Orca)\.app/",
]
JOBS = {
    "argv": ["/bin/sh", "-c", "/bin/launchctl list && { /usr/bin/crontab -l 2>/dev/null || true; }"],
    "patterns": [r"(?i)(anthropic|claude|codex|openai|chatgpt|herdr|\borca\b|\bmise\b|jdx|codebase-memory|grove|ravel)"],
}

PROBES_PROGRAM = PREPARED + "/owner_switch_probes.py"


def probe(name, *args):
    return {"name": name, "argv": ["$python", "-I", PROBES_PROGRAM, *args]}


PROBES = [probe("resolve-switched", "resolve", "--expect", "switched"),
          probe("claude-switched", "session", "--harness", "claude", "--route", "ravel"),
          probe("codex-switched", "session", "--harness", "codex", "--route", "ravel")]
NATIVE_PROBE = [probe("resolve-disabled", "resolve", "--expect", "disabled"),
                probe("claude-native", "session", "--harness", "claude", "--route", "native"),
                probe("codex-native", "session", "--harness", "codex", "--route", "native")]
BASELINE = {"name": "inventory", "argv": ["$python", "-I", PREPARED + "/inventory.py"]}
MAC_CHECK = {"name": "mac-check", "argv": [".local/bin/mac-check"]}
GATES = {"4": [{"name": "grove-release", "argv": ["$python", "-I", "$program", "gate", "4"]}],
         "5": [{"name": "skill-formulae", "argv": ["$python", "-I", "$program", "gate", "5"]}],
         "6": [{"name": "resolved-sources", "argv": ["$python", "-I", "$program", "gate", "6"]}],
         "7": [{"name": "reader-evidence", "argv": ["$python", "-I", "$program", "gate", "7"]}]}

# Provisional accepted values. The formula versions are those of "The commits
# of the pack record, the formulae and this record" in the moves record.
VERSIONS = {"linkuistics/taps/personal": "2026-11-06", "linkuistics/taps/testanyware": "2.8.0",
            "linkuistics/taps/superpowers": "7.0.0", "linkuistics/taps/quint-llm-kit": "1.0.0"}
GROVE_SOURCES = ("design", "engineering", "grove-core", "requirements", "research", "writing")
# owner-staging-setup-k248 records the final paths.
STAGING = {"home": os.path.join(HOME, ".local/state/owner-staging/home"),
           "candidate_keg": os.path.join(HOME, ".local/state/owner-staging/candidate")}


def provisional_trial(sid):
    message = f"provisional staging trial of {sid}: owner-switch-plan-k254 replaces it"
    return {"name": f"provisional-{sid}", "argv": ["/bin/sh", "-c", f"echo '{message}' >&2; exit 1"]}


def load_program():
    spec = importlib.util.spec_from_file_location("owner_switch", PROGRAM)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def identity(path):
    out = subprocess.run([sys.executable, "-I", PROGRAM, "identity", path],
                         capture_output=True, text=True, check=True).stdout.strip()
    if not re.fullmatch(r"[0-9a-f]{64}", out):
        sys.exit(f"stop: {path} has no change identity ({out})")
    return out


def inventory_rows():
    rows = {}
    with open(INVENTORY, encoding="utf-8") as f:
        header = f.readline().rstrip("\n").split("\t")
        for number, line in enumerate(f, start=2):
            row = dict(zip(header, line.rstrip("\n").split("\t")))
            row["line"] = number
            rows.setdefault((row["location"], row["entry"]), []).append(row)
    return rows


def tilde(rel):
    return "~/" + rel


def accepted_sources():
    """The table "The ten Ravel sources" of the moves record."""
    text = open(MOVES_RECORD, encoding="utf-8").read()
    section = text.split("### The ten Ravel sources", 1)[1].split("\n### ", 1)[0]
    sources = {}
    for line in section.splitlines():
        cells = [c.strip() for c in line.strip().strip("|").split("|")]
        if len(cells) == 8 and re.fullmatch(r"`[a-z-]+`", cells[0]):
            sid, formula, digest = cells[0].strip("`"), cells[1].strip("`"), cells[7].strip("`")
            if re.fullmatch(r"[0-9a-f]{64}", digest):
                sources[sid] = {"formula": "linkuistics/taps/" + formula, "resourceDigest": digest}
    if sorted(sources) != sorted(GROVE_SOURCES + ("personal", "testanyware", "superpowers", "quint")):
        sys.exit(f"stop: the moves record names the sources {sorted(sources)}, not the ten")
    for formula, version in VERSIONS.items():
        name = formula.rsplit("/", 1)[1]
        if f"`{name}` `{version}`" not in text:
            sys.exit(f"stop: the moves record does not name {name} {version}")
    return sources


def switched_skills():
    with open(os.path.join(DRAFTS, "settings-switched.json")) as f:
        switched = json.load(f)["skills"]
    with open(os.path.join(DRAFTS, "settings-off.json")) as f:
        off = json.load(f)["skills"]
    return off, switched


def draft(_args):
    rows = inventory_rows()
    parts, table, kept = [], [], {}
    for root in ROOTS:
        for name in sorted(os.listdir(os.path.join(HOME, root))):
            rel = f"{root}/{name}"
            found = rows.get((tilde(root), name), [])
            if rel in CONTAINERS:
                children = [r for (loc, _), rs in rows.items()
                            if loc == tilde(rel) or loc.startswith(tilde(rel) + "/") for r in rs]
                if not children or any(r["class"] != CONTAINERS[rel] for r in children):
                    sys.exit(f"stop: the container {rel} has no rows, or rows of another class")
                kept[rel] = CONTAINERS[rel]
                continue
            if len(found) != 1:
                sys.exit(f"stop: {rel} has {len(found)} inventory rows; classify it before drafting")
            row = found[0]
            if row["class"] in MOVED_CLASSES and row["disposition"].startswith("deactivate"):
                ident = identity(os.path.join(HOME, rel))
                parts.append({"kind": "entry", "path": rel, "identity": ident})
                table.append([len(parts) - 1, "entry", rel, "", "", "", ident, row["line"], row["class"],
                              row["disposition"]])
            elif row["class"] in KEPT_CLASSES:
                kept[rel] = row["class"]
            else:
                sys.exit(f"stop: {rel} has class {row['class']}, disposition {row['disposition']}")
    plugin_rows = {r["entry"]: r for rs in rows.values() for r in rs if r["form"].startswith("plugin v")}
    cells = []
    for pid in DISABLED:
        row = plugin_rows[pid]
        if "enabled=True" not in row["form"] or not row["disposition"].startswith("disable"):
            sys.exit(f"stop: the inventory row of {pid} is not an enabled plugin that the switch disables")
        cells.append(({"key": ["enabledPlugins", pid], "earlier": True, "switched": False}, row))
    for pid in ENABLED:
        cells.append(({"key": ["enabledPlugins", pid], "earlier": False, "switched": True}, None))
    parts.append({"kind": "file", "path": CLAUDE_SETTINGS, "cells": [c for c, _ in cells]})
    for cell, row in cells:
        source = (row["line"], row["class"], row["disposition"]) if row else (
            "", "native residue", "installed disabled before step 9; enabled by the switch")
        table.append([len(parts) - 1, "cell", CLAUDE_SETTINGS, ".".join(cell["key"]), json.dumps(cell["earlier"]),
                      json.dumps(cell["switched"]), "", *source])
    off, switched = switched_skills()
    parts.append({"kind": "file", "path": RAVEL_SETTINGS,
                  "cells": [{"key": ["skills"], "earlier": off, "switched": switched}]})
    table.append([len(parts) - 1, "cell", RAVEL_SETTINGS, "skills", "settings-off.json",
                  "settings-switched.json", "", "", "Ravel settings",
                  "the native settings write the off value in step 3; the switch writes the switched value"])
    outside = []
    for rel in OUTSIDE_PATHS:
        outside.append({"path": rel, "identity": identity(os.path.join(HOME, rel))})
    moved = {p["path"] for p in parts}
    for root in OUTSIDE_ROOTS:
        entries = {}
        for name in sorted(os.listdir(os.path.join(HOME, root))):
            if f"{root}/{name}" not in moved:
                entries[name] = identity(os.path.join(HOME, root, name))
        outside.append({"root": root, "entries": entries})
    plan = {
        "schema": "owner-switch-plan/1",
        "state": STATE,
        "manual": MANUAL,
        "parts": parts,
        "outside": outside,
        "session": {"harness": HARNESS, "mcp": MCP},
        "watch": WATCH,
        "jobs": JOBS,
        "baseline": BASELINE,
        "gates": GATES,
        "accepted": {
            "sources": accepted_sources(),
            "versions": VERSIONS,
            "trials": {sid: [provisional_trial(sid)] for sid in GROVE_SOURCES},
            "staging": STAGING,
        },
        "probes": PROBES,
        "native_probe": NATIVE_PROBE,
        "mac_check": MAC_CHECK,
    }
    os.makedirs(DRAFTS, exist_ok=True)
    with open(PLAN_OUT, "w") as f:
        json.dump(plan, f, indent=2, ensure_ascii=False)
        f.write("\n")
    with open(TABLE_OUT, "w") as f:
        f.write("part\tkind\tpath\tcell\tearlier\tswitched\tidentity\tinventory_line\tclass\tdisposition\n")
        for row in table:
            f.write("\t".join(str(x) for x in row) + "\n")
    entries = sum(1 for p in parts if p["kind"] == "entry")
    print(f"draft: {entries} entry parts, {len(parts) - entries} file parts, {len(outside)} outside items")
    print(f"draft: kept in the roots: {', '.join(sorted(kept))}")
    print(f"wrote {os.path.relpath(PLAN_OUT, ROOT)} and {os.path.relpath(TABLE_OUT, ROOT)}")
    return 0


# --- check ------------------------------------------------------------------------

def copy_entry(rel, home):
    src, dst = os.path.join(HOME, rel), os.path.join(home, rel)
    os.makedirs(os.path.dirname(dst), exist_ok=True)
    if os.path.islink(src):
        os.symlink(os.readlink(src), dst)
    elif os.path.isdir(src):
        shutil.copytree(src, dst, symlinks=True)
    else:
        shutil.copy2(src, dst)


def build_layout(plan, home):
    """A disposable HOME that holds each named entry, and each cell at its earlier value."""
    wanted = [(p["path"], p["identity"]) for p in plan["parts"] if p["kind"] == "entry"]
    for item in plan["outside"]:
        if "root" in item:
            os.makedirs(os.path.join(home, item["root"]), exist_ok=True)
            wanted += [(f"{item['root']}/{n}", i) for n, i in item["entries"].items()]
        else:
            wanted.append((item["path"], item["identity"]))
    drift = []
    for rel, ident in wanted:
        copy_entry(rel, home)
        if identity(os.path.join(home, rel)) != ident:
            drift.append(rel)
    if drift:
        sys.exit("stop: the real HOME changed since the draft: " + ", ".join(drift))
    for part in plan["parts"]:
        if part["kind"] != "file":
            continue
        body = {}
        for cell in part["cells"]:
            node = body
            for k in cell["key"][:-1]:
                node = node.setdefault(k, {})
            node[cell["key"][-1]] = cell["earlier"]
        path = os.path.join(home, part["path"])
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "w") as f:
            json.dump(body, f, indent=2)
            f.write("\n")
    manual = os.path.join(home, plan["manual"])
    os.makedirs(os.path.dirname(manual), exist_ok=True)
    with open(manual, "w") as f:
        f.write("# AI Dev Tooling\n")
    return len(wanted)


def program(*args):
    result = subprocess.run([sys.executable, "-I", PROGRAM, *args], capture_output=True, text=True)
    return result.returncode, result.stdout + result.stderr


def write_plan(plan, path):
    with open(path, "w") as f:
        json.dump(plan, f, indent=2, ensure_ascii=False)
        f.write("\n")
    return path


def controls(plan, out, home):
    """Each control must fail: a broken draft must not be admitted, and a changed layout is not native."""
    folder = os.path.join(out, "controls")
    os.makedirs(folder)
    results = []

    def refused(name, mutate, needle):
        bad = copy.deepcopy(plan)
        mutate(bad)
        code, text = program("run", "--dry-run", "--plan", write_plan(bad, os.path.join(folder, name + ".json")),
                             "--home", home)
        ok = code == 3 and needle in text
        results.append({"control": name, "exit": code, "passed": ok, "output": text.strip()[-600:]})

    refused("no-mac-check", lambda p: p.pop("mac_check"), "mac_check is not a command")
    refused("absolute-part-path", lambda p: p["parts"][0].update(path=os.path.join(HOME, p["parts"][0]["path"])),
            "is not relative to HOME")
    refused("gate-4-missing", lambda p: p["gates"].update({"4": [MAC_CHECK]}),
            "does not run the program's gate of step 4")

    def changed(name, act, undo):
        act()
        try:
            code, text = program("run", "--dry-run", "--plan", PLAN_OUT, "--home", home)
        finally:
            undo()
        ok = code == 0 and "state: changed" in text
        results.append({"control": name, "exit": code, "passed": ok, "output": text.strip()[:600]})

    stale = os.path.join(home, ".codex/skills/brainstorming/SKILL.md")
    original = open(stale, "rb").read()

    def restore():
        with open(stale, "wb") as f:
            f.write(original)

    def touch():
        with open(stale, "ab") as f:
            f.write(b"\n")

    changed("changed-entry", touch, restore)
    extra = os.path.join(home, ".agents/skills/new-native-skill")
    changed("new-outside-child", lambda: os.makedirs(extra), lambda: os.rmdir(extra))
    return results


# Strings that each pattern list must match, or must not match.
PATTERN_CASES = [
    ("watch", True, "/Applications/Claude.app/Contents/MacOS/Claude"),
    ("watch", True, "/Applications/ChatGPT.app/Contents/MacOS/ChatGPT"),
    ("watch", True, "/opt/homebrew/bin/codex exec --json hello"),
    ("watch", True, "node /opt/homebrew/lib/node_modules/@mariozechner/pi-coding-agent/dist/cli.js"),
    ("watch", True, "/opt/homebrew/bin/mise dot watch"),
    ("watch", True, "/opt/homebrew/bin/ravel skills resolve --json"),
    ("watch", True, "/Users/a/.local/bin/herdr-core serve"),
    ("watch", False, "/Applications/OrcaSlicer.app/Contents/MacOS/OrcaSlicer"),
    ("watch", False, "/opt/homebrew/bin/task serve DIR=/Users/a/Development/AgenticOrchestration/grove PORT=1"),
    ("watch", False, "python3 -m http.server 8778 --directory /Users/a/Ravel.skill-directory-delivery/docs"),
    ("harness", True, "claude --resume"),
    ("harness", True, "/Users/a/.local/bin/claude -p hello"),
    ("harness", False, "/Applications/Claude.app/Contents/MacOS/Claude"),
    ("mcp", False, "/opt/homebrew/bin/node /tmp/other-server.js"),
    ("mcp", False, "/bin/sleep 100"),
    ("mcp", False, "/bin/zsh -c ls"),
    ("jobs", True, "65402\t1\tdev.mise.mise-history"),
    ("jobs", False, "-\t0\tcom.bambulab.orcaslicer.helper"),
]


def pattern_cases(module, plan):
    lists = {"watch": plan["watch"], "harness": plan["session"]["harness"], "mcp": plan["session"]["mcp"],
             "jobs": plan["jobs"]["patterns"]}
    return [{"list": name, "text": text, "expected": want, "passed": module.matches(lists[name], text) == want}
            for name, want, text in PATTERN_CASES]


def short(args, home):
    return args.replace(home, "~")[:140]


def process_listing(module, plan, layout_home):
    """The real process tree against the plan's patterns, as the program's step 1 reads it."""
    procs = module.list_processes()
    me = os.getpid()
    ancestors, pid = [], me
    while pid in procs and procs[pid][0] not in (0, pid):
        pid = procs[pid][0]
        ancestors.append(pid)
    session = plan["session"]
    harness = next((a for a in ancestors if module.matches(session["harness"], procs[a][1])), None)
    lines = ["# The real process tree against the draft's patterns",
             "", f"listing process: {me}", "", "## Ancestors (kept: the program's shell, the harness and the driver chain)"]
    for a in ancestors:
        mark = "harness" if a == harness else "ancestor"
        lines.append(f"{a}\t{mark}\t{short(procs[a][1], HOME)}")
    kept = {me} | set(ancestors) | module.descendants(procs, me)
    servers = []
    if harness is not None:
        todo = [harness]
        while todo:
            parent = todo.pop()
            for p, (ppid, args) in sorted(procs.items()):
                if ppid == parent and p not in kept and module.matches(session["mcp"], args):
                    kept.add(p)
                    servers.append(p)
                    todo.append(p)
    lines += ["", "## Children of the harness and their descendants"]
    tree = module.descendants(procs, harness) if harness else set()
    for p in sorted(tree):
        if p in module.descendants(procs, me) or p == me:
            continue
        pattern = next((x for x in session["mcp"] if re.search(x, procs[p][1])), None)
        if p in servers:
            mark = f"kept by mcp {pattern}"
        elif p in ancestors:
            mark = "ancestor, kept"
        else:
            mark = "another process of the session"
        lines.append(f"{p}\tparent {procs[p][0]}\t{mark}\t{short(procs[p][1], HOME)}")
    others = [p for p in sorted(procs) if p not in kept and p not in tree and module.matches(plan["watch"], procs[p][1])]
    lines += ["", "## Other processes that a watch pattern matches (each would stop step 2 now)"]
    for p in others:
        pattern = next(x for x in plan["watch"] if re.search(x, procs[p][1]))
        lines.append(f"{p}\twatch {pattern}\t{short(procs[p][1], HOME)}")
    jobs = subprocess.run(plan["jobs"]["argv"], capture_output=True, text=True)
    matched = [l.strip() for l in jobs.stdout.splitlines() if module.matches(plan["jobs"]["patterns"], l)]
    lines += ["", f"## Jobs that a job pattern matches (listing exit {jobs.returncode})"] + matched
    # The program's own gate, from the same position, must name the same others.
    admitted = module.load_plan(PLAN_OUT)
    layout = module.Layout(layout_home, admitted)
    buf = io.StringIO()
    with contextlib.redirect_stdout(buf):
        try:
            module.process_gate(layout, admitted, "listing")
            stopped = False
        except module.Stop:
            stopped = True
    gate_lines = [l.strip() for l in buf.getvalue().splitlines() if l.strip().startswith("remaining")]
    lines += ["", f"## The program's process gate from the same position (stopped: {stopped})"]
    lines += [short(l, HOME) for l in gate_lines]
    session_others = [p for p in tree if p not in kept]
    return {"harness": harness, "servers": servers, "session_others": session_others, "watched": others,
            "jobs": matched, "gate_stopped": stopped, "gate_lines": len(gate_lines)}, "\n".join(lines) + "\n"


def check(args):
    out = os.path.abspath(args.out)
    if os.path.exists(out):
        sys.exit(f"stop: {out} exists")
    os.makedirs(out)
    module = load_program()
    raw = open(PLAN_OUT, "rb").read()
    plan = json.loads(raw)
    admitted = module.load_plan(PLAN_OUT)
    module.drafted_settings(admitted)
    home = os.path.join(out, "home")
    os.makedirs(home)
    copied = build_layout(plan, home)
    module.Layout(home, admitted)
    code, text = program("run", "--dry-run", "--plan", PLAN_OUT, "--home", home)
    with open(os.path.join(out, "dry-run.txt"), "w") as f:
        f.write(text.replace(out, "<OUT>"))
    switch_section = text.split("switch, in order:", 1)[-1].split("inverse, in order:", 1)[0]
    inverse_section = text.split("inverse, in order:", 1)[-1]
    listed = all(f"part {i} {p['path']}" in switch_section and f"part {i} {p['path']}" in inverse_section
                 for i, p in enumerate(plan["parts"]))
    dry = {"exit": code, "native": "state: native" in text, "each_part_and_inverse": listed,
           "unrehearsed_reported": "rehearsed: no" in text}
    results = controls(plan, out, home)
    cases = pattern_cases(module, plan)
    listing, text = process_listing(module, plan, home)
    with open(os.path.join(out, "processes.txt"), "w") as f:
        f.write(text)
    record = {
        "plan": os.path.relpath(PLAN_OUT, ROOT), "plan_digest": hashlib.sha256(raw).hexdigest(),
        "program_digest": module.PROGRAM_DIGEST, "python": sys.version.split()[0],
        "entries_copied": copied, "dry_run": dry, "controls": results, "pattern_cases": cases,
        "processes": listing,
    }
    with open(os.path.join(out, "record.json"), "w") as f:
        f.write(json.dumps(record, indent=2).replace(out, "<OUT>"))
        f.write("\n")
    passed = (code == 0 and dry["native"] and listed and all(r["passed"] for r in results)
              and all(c["passed"] for c in cases))
    print(f"plan {record['plan_digest']}, program {record['program_digest']}")
    print(f"dry run: exit {code}, native {dry['native']}, each part and its inverse listed {listed}")
    for r in results:
        print(f"control {r['control']}: {'passed' if r['passed'] else 'FAILED'} (exit {r['exit']})")
    for c in cases:
        if not c["passed"]:
            print(f"pattern case FAILED: {c['list']} should {'' if c['expected'] else 'not '}match {c['text']!r}")
    print(f"pattern cases: {sum(c['passed'] for c in cases)} of {len(cases)} passed")
    print(f"processes: harness {listing['harness']}, {len(listing['servers'])} kept servers, "
          f"{len(listing['session_others'])} other session processes, {len(listing['watched'])} watched, "
          f"{len(listing['jobs'])} jobs; the program's gate stopped: {listing['gate_stopped']}")
    print("result: " + ("passed" if passed else "FAILED"))
    return 0 if passed else 1


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n", 1)[0])
    sub = parser.add_subparsers(dest="command", required=True)
    sub.add_parser("draft")
    sub.add_parser("check").add_argument("--out", required=True)
    args = parser.parse_args()
    return draft(args) if args.command == "draft" else check(args)


if __name__ == "__main__":
    sys.exit(main())
