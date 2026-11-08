"""Draft the owner's real switch plan, and check it in a disposable layout.

The plan format is schema "owner-switch-plan/2" in the docstring of
scripts/owner_switch.py. docs/specs/owner-skill-delivery.md owns each
disposition: "Classes and dispositions", "Native residue", "Name collisions
and stale copies", "The switch inside finish", "Concurrent writers", "The
rehearsed plan", "Rollback" and "Current health and restore eligibility".
"Owner switch plan draft" of docs/owner-skill-delivery-evidence.md states
each choice of this draft.

    python3 -I switch_plan_draft.py draft
    python3 -I switch_plan_draft.py check --out OUT

`draft` reads the real HOME and changes nothing there. It runs no harness and
no tool, and it reads Claude Code's version from the path of its installed
executable. It classifies each child of the five discovery roots by its row
in docs/owner-skill-inventory-reference.tsv. A child whose class is
owner-pack, third-party-pack, stale-copy or dangling, with a disposition that
starts with "deactivate", becomes an entry write. Each other child must have
a row with a kept class, or be a container whose children have such rows. A
child without a row stops the draft. Each write carries the restore identity
of its original, or its absence, as `owner_switch.py restore-identity`
prints it. Each inventory item carries the change identity that
`owner_switch.py identity` prints. Each command input carries the digest of
its content. A prepared input that does not exist yet carries the digest of
the repository file that preparation copies there. The accepted sources come
from the table "The ten Ravel sources" of docs/repository-moves-record.md,
and they are provisional. The script writes
docs/owner-cutover/switch-plan-draft.json and the write table
docs/owner-cutover/switch-plan-writes.tsv.

`check` builds a disposable HOME in OUT/home from the draft. It copies each
write with the program's own metadata copy, and each inventory entry, from
the real HOME. It stops unless each copy has the draft's identity. It then
runs the program's loader and `report`, six plan controls that must be
refused, three layout changes that the report must show, and one change that
it must not show: a change inside .claude/skills/synced and
.codex/skills/.system. Last, it records the installed path and canonical
path of each executable of the four tools, and it runs the program's narrow
process inspection on the real process table. OUT holds record.json,
report.txt and processes.txt. OUT must not exist. The real HOME, launchd and
the process table are read only.
"""

import argparse, copy, hashlib, importlib.util, json, os, re, shutil, subprocess, sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", ".."))
PROGRAM = os.path.join(ROOT, "scripts", "owner_switch.py")
INVENTORY_SCRIPT = os.path.join(ROOT, "docs", "evaluations", "owner-skill-delivery", "inventory.py")
PROBES_SOURCE = os.path.join(ROOT, "scripts", "owner_switch_probes.py")
DRAFTS = os.path.join(ROOT, "docs", "owner-cutover")
PLAN_OUT = os.path.join(DRAFTS, "switch-plan-draft.json")
TABLE_OUT = os.path.join(DRAFTS, "switch-plan-writes.tsv")
INVENTORY = os.path.join(ROOT, "docs", "owner-skill-inventory-reference.tsv")
MOVES_RECORD = os.path.join(ROOT, "docs", "repository-moves-record.md")
HOME = os.path.realpath(os.path.expanduser("~"))

# The five discovery roots whose superseded children the switch deactivates.
ROOTS = (".claude/skills", ".codex/skills", ".agents/skills", ".pi/agent/skills", ".pi/agent/extensions")
MOVED_CLASSES = ("owner-pack", "third-party-pack", "stale-copy", "dangling")
KEPT_CLASSES = ("tool-managed", "platform-managed")
# Containers whose children have inventory rows of their own.
CONTAINERS = {".claude/skills/synced": "platform-managed", ".codex/skills/.system": "platform-managed"}

STATE = ".local/state/owner-switch"
PREPARED = STATE + "/prepared"
MANUAL = "Library/Mobile Documents/iCloud~md~obsidian/Documents/Brain/Configuration/AI Dev Tooling.md"
CLAUDE_SETTINGS = ".claude/settings.json"
KNOWN_MARKETPLACES = ".claude/plugins/known_marketplaces.json"
INSTALLED_PLUGINS = ".claude/plugins/installed_plugins.json"
RAVEL_SETTINGS = ".config/ravel/settings.json"
ABSENT = {"$absent": True}

# "Classes and dispositions" and "Native residue": the plugins that the switch
# disables, and the two skill-free plugins that it enables.
DISABLED = ("grove@linkuistics", "linkuistics@linkuistics", "testanyware@linkuistics",
            "superpowers@claude-plugins-official", "quint-llm-kit@quint-llm-kit")
# The hooks installation installs the first disabled. owner-pack-checkouts-k93
# installs the second disabled. The switch enables both at step 6.
ENABLED = ("technical-communication-hooks@writing-hooks", "guardrail@personal")

# The inventory: native residue that must stay unchanged, and each ordinary
# child of the discovery roots. "Concurrent writers" compares each ordinary
# child of both native roots. A probe session can rewrite
# .claude/skills/synced and .codex/skills/.system, so no item names either
# container, and the program skips exactly those two. The agent folders are
# the D3 roots of owner-inventory-exceptions-k264. No plugin version
# directory, because each session writes .in_use/<pid> there.
INVENTORY_PATHS = (".agents/.grove",)
INVENTORY_ROOTS = ROOTS + (".claude/agents", ".codex/agents")

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
# Step 8 needs exit 0 from the switch's area: the full mac-check exits 10 for
# an unrelated manual review (owner-switch-status-k269).
MAC_CHECK = {"name": "mac-check", "argv": [".local/bin/mac-check", "--only", "Owner skill switch"]}
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


def program_line(command, path):
    out = subprocess.run([sys.executable, "-I", PROGRAM, command, path],
                         capture_output=True, text=True, check=True).stdout.strip()
    if not (out == "absent" or re.fullmatch(r"[0-9a-f]{64}", out)):
        sys.exit(f"stop: {path} has no {command} ({out})")
    return out


def identity(path):
    out = program_line("identity", path)
    if out == "absent":
        sys.exit(f"stop: {path} is absent")
    return out


def original(rel):
    found = program_line("restore-identity", os.path.join(HOME, rel))
    return {"absent": True} if found == "absent" else {"identity": found}


def file_digest(path):
    with open(path, "rb") as f:
        return hashlib.sha256(f.read()).hexdigest()


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


def held_claude():
    """The installed Claude Code version, from the canonical path of its executable, as `claude --version` prints it."""
    path = shutil.which("claude")
    real = os.path.realpath(path) if path else ""
    parts = real.split("/")
    if len(parts) < 3 or parts[-2] != "versions" or parts[-3] != "claude":
        sys.exit(f"stop: the installed claude {path} is not a versioned Claude Code executable ({real})")
    return f"{parts[-1]} (Claude Code)"


def original_ravel():
    """The keg of the installed Ravel, which the running driver uses until it ends."""
    path = shutil.which("ravel")
    real = os.path.realpath(path) if path else ""
    keg = os.path.dirname(os.path.dirname(real))
    if not re.search(r"/Cellar/ravel/[^/]+$", keg):
        sys.exit(f"stop: the installed ravel {path} is not in a Homebrew keg ({real})")
    return keg


def inputs():
    """Each file that a command runs or reads, by content.

    A prepared file that preparation copies from the repository carries the
    digest of its repository source until owner-switch-plan-k254 copies it.
    The probe program has no source yet; owner-switch-probes-k257 supplies it.
    """
    mac_check = os.path.join(HOME, ".local/bin/mac-check")
    rows = [{"role": "program", "path": "$program", "digest": file_digest(PROGRAM)},
            {"role": "baseline instrument", "path": os.path.join(HOME, PREPARED, "inventory.py"),
             "digest": file_digest(INVENTORY_SCRIPT)},
            {"role": "mac-check command", "path": mac_check, "digest": file_digest(mac_check)},
            {"role": "mac-check helper", "path": os.path.join(HOME, ".local/lib/machine-config/check.py"),
             "digest": file_digest(os.path.join(HOME, ".local/lib/machine-config/check.py"))}]
    if os.path.exists(PROBES_SOURCE):
        rows.append({"role": "probe program", "path": os.path.join(HOME, PROBES_PROGRAM),
                     "digest": file_digest(PROBES_SOURCE)})
    return rows


def classify(rows):
    """The entry writes of the five roots, the kept children and their table rows."""
    entries, table, kept = [], [], {}
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
                write = {"path": rel, "type": "entry", "original": original(rel)}
                entries.append(write)
                table.append([rel, "entry", show_original(write), "", "", row["line"], row["class"],
                              row["disposition"]])
            elif row["class"] in KEPT_CLASSES:
                kept[rel] = row["class"]
            else:
                sys.exit(f"stop: {rel} has class {row['class']}, disposition {row['disposition']}")
    return entries, table, kept


def show_original(write):
    return "absent" if write["original"].get("absent") else write["original"]["identity"]


def settings_values(rows):
    """The values that the switch sets in settings.json, with their table sources."""
    plugin_rows = {r["entry"]: r for rs in rows.values() for r in rs if r["form"].startswith("plugin v")}
    values = []
    for pid in DISABLED:
        row = plugin_rows[pid]
        if "enabled=True" not in row["form"] or not row["disposition"].startswith("disable"):
            sys.exit(f"stop: the inventory row of {pid} is not an enabled plugin that the switch disables")
        values.append(({"key": ["enabledPlugins", pid], "value": False}, (row["line"], row["class"],
                                                                          row["disposition"])))
    for pid in ENABLED:
        values.append(({"key": ["enabledPlugins", pid], "value": True},
                       ("", "native residue", "installed disabled before the switch; enabled at step 6")))
    return values


def inventory(entries):
    """The expected inventories before and after the switch. The deactivated entries leave the roots."""
    gone = {w["path"] for w in entries}
    before, after = [], []
    for rel in INVENTORY_PATHS:
        item = {"path": rel, "identity": identity(os.path.join(HOME, rel))}
        before.append(item)
        after.append(item)
    for root in INVENTORY_ROOTS:
        children = {}
        for name in sorted(os.listdir(os.path.join(HOME, root))):
            if f"{root}/{name}" not in CONTAINERS:
                children[name] = identity(os.path.join(HOME, root, name))
        before.append({"root": root, "entries": children})
        after.append({"root": root, "entries": {n: i for n, i in children.items() if f"{root}/{n}" not in gone}})
    return {"before": before, "after": after}


def draft(_args):
    rows = inventory_rows()
    entries, table, kept = classify(rows)
    settings = settings_values(rows)
    off, switched = switched_skills()
    files = [
        {"path": CLAUDE_SETTINGS, "type": "file", "original": original(CLAUDE_SETTINGS),
         "set": [v for v, _ in settings]},
        {"path": KNOWN_MARKETPLACES, "type": "file", "original": original(KNOWN_MARKETPLACES)},
        {"path": INSTALLED_PLUGINS, "type": "file", "original": original(INSTALLED_PLUGINS)},
        {"path": RAVEL_SETTINGS, "type": "file", "original": original(RAVEL_SETTINGS),
         "set": [{"key": ["skills"], "value": switched}]},
    ]
    for value, source in settings:
        table.append([CLAUDE_SETTINGS, "file", show_original(files[0]), ".".join(value["key"]),
                      json.dumps(value["value"]), *source])
    for w in files[1:3]:
        table.append([w["path"], "file", show_original(w), "", "", "", "Claude Code registry",
                      "written by the hooks installation's commands"])
    table.append([RAVEL_SETTINGS, "file", show_original(files[3]), "skills", "settings-switched.json", "",
                  "Ravel settings", "native-off writes the off value before the switch; the switch writes the "
                                    "switched value last"])
    writes = entries + files
    restore = [RAVEL_SETTINGS, KNOWN_MARKETPLACES, INSTALLED_PLUGINS, CLAUDE_SETTINGS] + [w["path"] for w in entries]
    assertions = [{"path": RAVEL_SETTINGS, "key": ["skills"], "value": switched}]
    assertions += [{"path": CLAUDE_SETTINGS, "key": v["key"], "value": v["value"]} for v, _ in settings]
    assertions += [{"path": w["path"], "absent": True} for w in entries]
    # Before promotion, the real Ravel settings may lack "skills" (owner-switch-status-k269).
    native = [{"path": RAVEL_SETTINGS, "key": ["skills"], "values": [off, ABSENT]}]
    native += [{"path": CLAUDE_SETTINGS, "key": ["enabledPlugins", pid], "value": True} for pid in DISABLED]
    native += [{"path": CLAUDE_SETTINGS, "key": ["enabledPlugins", pid], "values": [False, ABSENT]}
               for pid in ENABLED]
    plan = {
        "schema": "owner-switch-plan/2",
        "state": STATE,
        "manual": MANUAL,
        "writes": writes,
        "restore": restore,
        "inventory": inventory(entries),
        "inputs": inputs(),
        "assertions": assertions,
        "native_assertions": native,
        "tools": {"claude": held_claude()},
        "retained": [{"item": "the original driver's Ravel keg", "path": original_ravel(),
                      "condition": ["its last driver ends"]}],
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
        f.write("path\ttype\toriginal\tkey\tswitched\tinventory_line\tclass\tdisposition\n")
        for row in table:
            f.write("\t".join(str(x) for x in row) + "\n")
    print(f"draft: {len(entries)} entry writes, {len(files)} file writes, "
          f"{len(plan['inventory']['before'])} inventory items, {len(plan['inputs'])} command inputs")
    if not os.path.exists(PROBES_SOURCE):
        print("draft: the probe program has no source yet, so the inputs do not name it")
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


def build_layout(module, plan, home):
    """A disposable HOME that holds each write with its metadata, and each inventory entry."""
    drift = []
    for w in plan["writes"]:
        src, dst = os.path.join(HOME, w["path"]), os.path.join(home, w["path"])
        if w["original"].get("absent"):
            if os.path.lexists(src):
                drift.append(w["path"])
            continue
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        module.copy_tree(src, dst)
        if module.restore_identity(dst) != w["original"]["identity"]:
            drift.append(w["path"])
    written = {w["path"] for w in plan["writes"]}
    copied = 0
    for item in plan["inventory"]["before"]:
        if "root" in item:
            os.makedirs(os.path.join(home, item["root"]), exist_ok=True)
            wanted = [(f"{item['root']}/{n}", i) for n, i in item["entries"].items()]
        else:
            wanted = [(item["path"], item["identity"])]
        for rel, ident in wanted:
            if rel not in written:
                copy_entry(rel, home)
            copied += 1
            if identity(os.path.join(home, rel)) != ident:
                drift.append(rel)
    if drift:
        sys.exit("stop: the real HOME changed since the draft: " + ", ".join(drift))
    manual = os.path.join(home, plan["manual"])
    os.makedirs(os.path.dirname(manual), exist_ok=True)
    with open(manual, "w") as f:
        f.write("# AI Dev Tooling\n")
    return len(plan["writes"]), copied


def program(*args):
    result = subprocess.run([sys.executable, "-I", PROGRAM, *args], capture_output=True, text=True)
    return result.returncode, result.stdout + result.stderr


def write_plan(plan, path):
    with open(path, "w") as f:
        json.dump(plan, f, indent=2, ensure_ascii=False)
        f.write("\n")
    return path


def controls(plan, out, home):
    """A broken draft must not be admitted, and the report must show a changed layout, except inside a platform subtree."""
    folder = os.path.join(out, "controls")
    os.makedirs(folder)
    results = []

    def refused(name, mutate, needle):
        bad = copy.deepcopy(plan)
        mutate(bad)
        code, text = program("report", "--plan", write_plan(bad, os.path.join(folder, name + ".json")),
                             "--home", home)
        ok = code == 3 and needle in text
        results.append({"control": name, "exit": code, "passed": ok, "output": text.strip()[-600:]})

    def earlier(p):
        p["writes"][-1]["set"][0]["earlier"] = {"schemaVersion": 1, "sources": []}

    refused("no-mac-check", lambda p: p.pop("mac_check"), "mac_check is not a command")
    refused("absolute-write-path", lambda p: p["writes"][0].update(path=os.path.join(HOME, p["writes"][0]["path"])),
            "is not relative to HOME")
    refused("gate-4-missing", lambda p: p["gates"].update({"4": [MAC_CHECK]}),
            "does not run the program's gate of step 4")
    refused("process-patterns", lambda p: p.update(watch=[r"^(\S*/)?codex(\s|$)"]), "the plan holds watch")
    refused("earlier-value", earlier, ".set is not a list of keys with values")
    refused("parts", lambda p: p.update(parts=[]), "the plan holds parts")

    def shown(name, act, undo, needle):
        act()
        try:
            code, text = program("report", "--plan", PLAN_OUT, "--home", home)
        finally:
            undo()
        lines = [line for line in text.splitlines() if re.search(needle, line)]
        ok = code == 0 and bool(lines)
        results.append({"control": name, "exit": code, "passed": ok, "output": "\n".join(lines)[-600:]})

    # Only a real folder inside the disposable HOME: many entries are links into the real HOME, and the
    # control must never write through one.
    entry = next(w["path"] for w in plan["writes"] if w["type"] == "entry" and w["path"].startswith(".codex/skills/")
                 and not os.path.islink(os.path.join(home, w["path"])) and os.path.isdir(os.path.join(home, w["path"])))
    skill = os.path.join(home, entry, "SKILL.md")
    if os.path.islink(skill) or not os.path.realpath(skill).startswith(os.path.realpath(home) + os.sep):
        sys.exit(f"stop: {skill} leads outside the disposable HOME")
    saved = open(skill, "rb").read()

    def put_back():
        with open(skill, "wb") as f:
            f.write(saved)

    def touch():
        with open(skill, "ab") as f:
            f.write(b"\n")

    shown("changed-entry", touch, put_back, rf"^  entry {re.escape(entry)}: original \S+; now differs: ")
    extra = os.path.join(home, ".agents/skills/new-native-skill")
    shown("new-inventory-child", lambda: os.makedirs(extra), lambda: os.rmdir(extra),
          r"^inventory before: differs at .*\.agents/skills/new-native-skill")
    sibling = os.path.join(home, ".codex/skills/quint-lang")
    shown("new-sibling-of-system", lambda: os.makedirs(sibling), lambda: os.rmdir(sibling),
          r"^inventory before: differs at .*\.codex/skills/quint-lang")

    # A change confined to the two platform subtrees leaves the inventory as expected.
    inside = [os.path.join(home, ".claude/skills/synced"), os.path.join(home, ".codex/skills/.system")]
    for d in inside:
        os.makedirs(os.path.join(d, "probe-skill"))
    try:
        code, text = program("report", "--plan", PLAN_OUT, "--home", home)
    finally:
        for d in inside:
            shutil.rmtree(os.path.join(d, "probe-skill"))
    results.append({"control": "inside-platform-subtrees", "exit": code,
                    "passed": code == 0 and "inventory before: matches" in text, "output": text.strip()[-600:]})
    return results


# "The narrow process inspection": preparation records the installed paths of
# the four tools. The canonical path names the version of each.
TOOL_EXECUTABLES = ("claude", "codex", "grove", "grove-llm", "ravel")


def short(text, home):
    return text.replace(home, "~")[:200]


def installed_tools():
    """Each tool executable on PATH, with its canonical path."""
    out = {}
    for name in TOOL_EXECUTABLES:
        path = shutil.which(name)
        out[name] = {"path": path and short(path, HOME), "canonical": path and short(os.path.realpath(path), HOME)}
    return out


def process_listing(module):
    """The program's narrow process inspection of the real process table, reading only."""
    report = module.inspect_processes(HOME)
    lines = ["# The narrow process inspection of the real process table", "",
             f"listing process: {os.getpid()}", "", "## Installed executables of the four tools"]
    tools = installed_tools()
    for name, item in tools.items():
        lines.append(f"{name}\t{item['path']}\t{item['canonical']}")
    for title, key in (("Admitted: the exact ancestors of the listing process", "admitted"),
                       ("Other tool processes (each would stop the switch now)", "other"),
                       ("Processes that need review (each would stop the switch now)", "review")):
        lines += ["", f"## {title}"]
        lines += [short(module.describe(item), HOME) for item in report[key]]
    summary = {"admitted": [{"pid": i["pid"], "tool": i.get("tool")} for i in report["admitted"]],
               "other": len(report["other"]), "review": len(report["review"]),
               "forms": sorted({f"{i['tool']}: {i['form'].split(',')[0]}" for k in ("admitted", "other")
                                for i in report[k]})}
    return tools, summary, "\n".join(lines) + "\n"


def check(args):
    out = os.path.abspath(args.out)
    if os.path.exists(out):
        sys.exit(f"stop: {out} exists")
    os.makedirs(out)
    module = load_program()
    raw = open(PLAN_OUT, "rb").read()
    plan = json.loads(raw)
    admitted = module.load_plan(PLAN_OUT)
    home = os.path.join(out, "home")
    os.makedirs(home)
    writes, copied = build_layout(module, plan, home)
    module.Layout(home, admitted)
    code, text = program("report", "--plan", PLAN_OUT, "--home", home)
    with open(os.path.join(out, "report.txt"), "w") as f:
        f.write(text.replace(out, "<OUT>"))
    listed = all(f"{w['type']} {w['path']}: original" in text for w in plan["writes"])
    originals = all(f"{w['type']} {w['path']}: original {show_original(w)}; now its original" in text
                    for w in plan["writes"])
    native = [line for line in text.splitlines() if line.startswith("native_assertions: ")]
    report = {"exit": code, "each_write_listed": listed, "each_original_in_place": originals,
              "inventory_before_matches": "inventory before: matches" in text,
              "native_assertions_hold": bool(native) and all(line.endswith(": holds") for line in native)}
    results = controls(plan, out, home)
    removed = [k for k in plan if k not in module.PLAN_FIELDS]
    tools, listing, text = process_listing(module)
    with open(os.path.join(out, "processes.txt"), "w") as f:
        f.write(text)
    record = {
        "plan": os.path.relpath(PLAN_OUT, ROOT), "plan_digest": hashlib.sha256(raw).hexdigest(),
        "program_digest": module.PROGRAM_DIGEST, "python": sys.version.split()[0],
        "writes_copied": writes, "inventory_entries_copied": copied, "report": report, "controls": results,
        "fields_outside_the_schema": removed, "installed_tools": tools, "processes": listing,
    }
    with open(os.path.join(out, "record.json"), "w") as f:
        f.write(json.dumps(record, indent=2).replace(out, "<OUT>"))
        f.write("\n")
    passed = (code == 0 and all(v for k, v in report.items() if k != "exit")
              and all(r["passed"] for r in results) and not removed)
    print(f"plan {record['plan_digest']}, program {record['program_digest']}")
    print(f"report: exit {code}, each write listed {listed}, each original in place {originals}, "
          f"inventory before matches {report['inventory_before_matches']}, "
          f"native assertions hold {report['native_assertions_hold']}")
    for r in results:
        print(f"control {r['control']}: {'passed' if r['passed'] else 'FAILED'} (exit {r['exit']})")
    print(f"fields outside the schema: {', '.join(removed) or 'none'}")
    print(f"processes: {len(listing['admitted'])} admitted ancestors, {listing['other']} other tool processes, "
          f"{listing['review']} that need review")
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
