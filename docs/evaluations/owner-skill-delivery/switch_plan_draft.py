"""Draft the owner's real switch plan, and check it in a disposable layout.

The plan format is schema "owner-switch-plan/2" in the docstring of
scripts/owner_switch.py. docs/specs/owner-skill-delivery.md owns each
disposition: "Classes and dispositions", "Native residue", "Name collisions
and stale copies", "The switch inside finish", "Concurrent writers", "The
rehearsed plan", "Rollback" and "Current health and restore eligibility".
"Owner switch plan draft" of docs/owner-skill-delivery-evidence.md states
each choice of this draft.

    python3 -I switch_plan_draft.py draft [--home HOME --out DIR]
    python3 -I switch_plan_draft.py check --out OUT
    python3 -I switch_plan_draft.py prepare

`draft` reads the real HOME and changes nothing there. It runs no harness and
no tool, and it reads Claude Code's version from the path of its installed
executable. It classifies each child of the five discovery roots by its row
in docs/owner-skill-inventory-reference.tsv. A child whose class is
owner-pack, third-party-pack, stale-copy or dangling, with a disposition that
starts with "deactivate", becomes an entry write. Each other child must have
a row with a kept class, or be a container whose children have such rows. A
child without a row stops the draft. Each write carries the restore identity
of its original, or its absence, as `owner_switch.py restore-identity`
prints it. The Ravel settings are the one exception: finish runs
`native-off` before the switch, so their original is the file that
`native-off` writes. The draft runs the program's `native-off` in a
disposable HOME that holds a copy of the real `.config/ravel`, and it plans
the identity of the result. The write table states each write's identity in
the real HOME now beside its planned original. Each inventory item carries
the change identity that `owner_switch.py identity` prints. Each command
input carries the digest of its content. Each prepared input carries the
digest of the repository file that `prepare` copies there, so the draft
names the prepared files before they exist. The accepted sources come from
the table "The accepted sources" in "Staging acceptance" of
docs/owner-skill-delivery-evidence.md, and the draft stops unless the table
"The ten Ravel sources" of docs/repository-moves-record.md names the same
digests. The accepted versions are full package versions. The draft stops
unless the table of "Publication" in "Packaging-only acceptance" of the
evidence names each one. The trials of each Grove source are the
`staging_trial.py rerun` commands of "Trials for gate 4" in that section.
They run the frozen copy of the staging runner in <state>/prepared/staging,
whose content stays the same through finish. The script writes
docs/owner-cutover/switch-plan-draft.json and the write table
docs/owner-cutover/switch-plan-writes.tsv.

`draft --home HOME --out DIR` drafts the same plan from another HOME, such
as the staging HOME of owner-staging-hooks-k251, into DIR/plan.json and
DIR/writes.tsv. A discovery root that HOME lacks holds no entry, and an
inventory path that HOME lacks is left out. The plugin
values, the accepted values and accepted.staging stay those of the real
draft, because the staging HOME stands for the real one.

`check` builds a disposable HOME in OUT/home from the draft. It copies each
write with the program's own metadata copy, and each inventory entry, from
the real HOME. It runs `native-off` there for the Ravel settings, as finish
does. It stops unless each copy has the draft's identity. It then
runs the program's loader and `report`, six plan controls that must be
refused, three layout changes that the report must show, and one change that
it must not show: a change inside .claude/skills/synced and
.codex/skills/.system. Last, it records the installed path and canonical
path of each executable of the four tools, and it runs the program's narrow
process inspection on the real process table. OUT holds record.json,
report.txt and processes.txt. OUT must not exist. The real HOME, launchd and
the process table are read only.

`prepare` writes the owner's prepared folder HOME/.local/state/owner-switch/
prepared from the draft and the repository: plan.json, the program
owner_switch.py, inventory.py, owner_switch_probes.py, harness_trial.py, the
comparison instrument owner_switch_dry_run.py, INSTRUCTIONS.md and the
frozen staging runner in staging/. It stops unless each copied input has its
planned digest. A folder that already holds the same bytes stays; a
different prepared file stops it, and nothing else in HOME changes.
"""

import argparse, copy, hashlib, importlib.util, json, os, re, shutil, subprocess, sys, tempfile

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", ".."))
PROGRAM = os.path.join(ROOT, "scripts", "owner_switch.py")
INVENTORY_SCRIPT = os.path.join(ROOT, "docs", "evaluations", "owner-skill-delivery", "inventory.py")
PROBES_SOURCE = os.path.join(ROOT, "scripts", "owner_switch_probes.py")
HARNESS_SOURCE = os.path.join(ROOT, "scripts", "harness_trial.py")
DRY_RUN_SOURCE = os.path.join(ROOT, "scripts", "owner_switch_dry_run.py")
INSTRUCTIONS_SOURCE = os.path.join(ROOT, "docs", "owner-cutover", "prepared-instructions.md")
DRAFTS = os.path.join(ROOT, "docs", "owner-cutover")
PLAN_OUT = os.path.join(DRAFTS, "switch-plan-draft.json")
TABLE_OUT = os.path.join(DRAFTS, "switch-plan-writes.tsv")
INVENTORY = os.path.join(ROOT, "docs", "owner-skill-inventory-reference.tsv")
MOVES_RECORD = os.path.join(ROOT, "docs", "repository-moves-record.md")
EVIDENCE = os.path.join(ROOT, "docs", "owner-skill-delivery-evidence.md")
HOME = os.path.realpath(os.path.expanduser("~"))
# True when `draft --home` names another HOME than the account's.
OTHER_HOME = False

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
# The interpreter of the owner's commands in INSTRUCTIONS.md: mac-check's
# machine interpreter, which no project's mise setting replaces.
OWNER_PYTHON = "/opt/homebrew/opt/python@3.14/bin/python3.14"

# The frozen staging runner of gate 4 ("Trials for gate 4" of the evidence).
# The runner finds its helpers by their places in the repository, so the
# copy keeps those relative paths. Each case file and the kind-skill tree
# are inputs too. The main checkout gains `rerun` only when finish moves
# main, and this workspace changes at the rebase, so neither has stable
# content through the switch.
STAGING_RUNNER = PREPARED + "/staging"
RUNNER_FILES = ("scripts/harness_trial.py", "scripts/harness-trials/builds.json",
                "scripts/harness-trials/trial-launch.sh",
                "docs/evaluations/owner-skill-delivery/staging_trial.py",
                "docs/evaluations/owner-skill-delivery/staging_home.py",
                "docs/evaluations/owner-skill-delivery/inventory.py",
                "docs/owner-cutover/settings-off.json", "docs/owner-cutover/settings-staging.json")
RUNNER_TREES = ("docs/evaluations/owner-skill-delivery/staging-cases",)
RUNNER = "docs/evaluations/owner-skill-delivery/staging_trial.py"


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

# The formula versions are the full package versions, with their revisions,
# of "Packaging-only acceptance" in docs/owner-skill-delivery-evidence.md. The
# tap published them on 2026-11-11.
VERSIONS = {"linkuistics/taps/personal": "2026-11-06_1", "linkuistics/taps/testanyware": "2.8.0_1",
            "linkuistics/taps/superpowers": "7.0.0_1", "linkuistics/taps/quint-llm-kit": "1.0.0_1"}
GROVE_SOURCES = ("design", "engineering", "grove-core", "requirements", "research", "writing")
# "The accepted staging paths" of "Staging acceptance" names both paths.
STAGING = {"home": os.path.join(HOME, ".local/state/owner-staging/home"),
           "candidate_keg": os.path.join(HOME, ".local/state/owner-staging/candidate")}
STAGING_RAVEL = os.path.join(HOME, ".local/state/owner-staging/ravel-2.0")
STAGING_TRIALS = os.path.join(HOME, ".local/state/owner-staging/trials")

# "Trials for gate 4" of "Staging acceptance": each Grove source's trials, as
# (trial, harness). None is the source trial, which launches no model.
TRIALS = {"grove-core": [("sources", None), ("kind-skill", "claude"), ("kind-skill", "codex"),
                         ("stale-copy", "claude"), ("stale-copy", "codex")],
          "requirements": [("sources", None)],
          "design": [("sources", None), ("discovery", "claude"), ("discovery", "codex")],
          "engineering": [("sources", None), ("delegate", "claude"), ("delegate", "codex"),
                          ("compaction", "claude"), ("compaction", "codex"),
                          ("codebase-memory-mcp", "claude"), ("codebase-memory-mcp", "codex")],
          "research": [("sources", None)],
          "writing": [("sources", None), ("native-residue", "claude")]}
# "Rerun commands": the Claude Code compaction case fails with the staging
# default model, so it runs with the owner's impl model and effort.
TRIAL_OPTIONS = {("compaction", "claude"): ["--model", "claude-opus-5-5", "--effort", "high"]}


def rerun_trial(trial, harness):
    """One command of accepted.trials: the frozen runner's `rerun`, as "Trials for gate 4" states it."""
    argv = ["$python", "-I", os.path.join(HOME, STAGING_RUNNER, RUNNER), "rerun",
            "--home", STAGING["home"], "--candidate", STAGING["candidate_keg"],
            "--ravel", STAGING_RAVEL, "--trials", STAGING_TRIALS, "--trial", trial]
    if harness:
        argv += ["--harness", harness]
    argv += TRIAL_OPTIONS.get((trial, harness), [])
    return {"name": f"{trial}-{harness}" if harness else "sources", "argv": argv}


def runner_files():
    """Each file of the frozen staging runner: its path inside the copy, and its repository source."""
    out = [(rel, os.path.join(ROOT, rel)) for rel in RUNNER_FILES]
    for tree in RUNNER_TREES:
        for folder, dirs, files in os.walk(os.path.join(ROOT, tree)):
            dirs[:] = sorted(d for d in dirs if d != "__pycache__")
            for name in sorted(files):
                src = os.path.join(folder, name)
                out.append((os.path.relpath(src, ROOT), src))
    return out


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


def native_off_original():
    """The Ravel settings that native-off writes from the real .config/ravel, as a planned original.

    owner-staging-hooks-k251 found that the switch refuses an absent
    original, because finish runs native-off before it. native-off is
    deterministic, so a disposable HOME computes its result.
    """
    module = load_program()
    work = tempfile.mkdtemp(prefix="native-off-")
    try:
        home = os.path.join(work, "home")
        os.makedirs(os.path.join(home, ".config"))
        real = os.path.join(HOME, os.path.dirname(RAVEL_SETTINGS))
        if os.path.isdir(real) and not os.path.islink(real):
            module.copy_tree(real, os.path.join(home, os.path.dirname(RAVEL_SETTINGS)))
        result = subprocess.run([sys.executable, "-I", PROGRAM, "native-off", "--home", home],
                                capture_output=True, text=True, env=disposable_env(work))
        if result.returncode != 0:
            sys.exit(f"stop: native-off in a disposable HOME exited {result.returncode}: "
                     f"{(result.stdout + result.stderr).strip()[-400:]}")
        return {"identity": program_line("restore-identity", os.path.join(home, RAVEL_SETTINGS))}
    finally:
        shutil.rmtree(work, ignore_errors=True)


def disposable_env(work):
    """The environment of native-off in a disposable HOME.

    native-off refuses while another Ravel process runs, which protects the
    real settings. A disposable HOME has no other writer, so the process
    inspection there covers only an empty folder. The program refuses that
    scope when HOME is the account's home directory.
    """
    scope = os.path.join(work, "no-processes")
    os.makedirs(scope, exist_ok=True)
    return dict(os.environ, OWNER_SWITCH_REHEARSAL_SCOPE=scope)


def real_now(rel):
    return program_line("restore-identity", os.path.join(HOME, rel))


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


def published_versions():
    """The table of "Publication" in "Packaging-only acceptance" of the evidence: formula -> full version."""
    text = open(EVIDENCE, encoding="utf-8").read()
    section = text.split("\n## Packaging-only acceptance\n", 1)[1].split("\n## ", 1)[0]
    section = section.split("\n### Publication\n", 1)[1].split("\n### ", 1)[0]
    found = {}
    for line in section.splitlines():
        m = re.fullmatch(r"\| `([a-z-]+)` \| `([0-9][0-9A-Za-z.-]*_[1-9][0-9]*)` \|", line.strip())
        if m:
            found[m.group(1)] = m.group(2)
    return found


def staging_accepted():
    """The table of "The accepted sources" in "Staging acceptance": source -> (formula, digest)."""
    text = open(EVIDENCE, encoding="utf-8").read()
    section = text.split("\n## Staging acceptance\n", 1)[1].split("\n## ", 1)[0]
    section = section.split("\n### The accepted sources\n", 1)[1].split("\n### ", 1)[0]
    found = {}
    for line in section.splitlines():
        cells = [c.strip() for c in line.strip().strip("|").split("|")]
        if len(cells) == 6 and re.fullmatch(r"`[a-z-]+`", cells[0]):
            digest = cells[5].strip("`")
            if re.fullmatch(r"[0-9a-f]{64}", digest):
                found[cells[0].strip("`")] = (cells[1].strip("`"), digest)
    return found


def accepted_sources():
    """The accepted sources of "Staging acceptance", which the moves record's table "The ten Ravel sources" repeats."""
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
    accepted = staging_accepted()
    for sid, source in sources.items():
        if accepted.get(sid) != (source["formula"], source["resourceDigest"]):
            sys.exit(f"stop: \"Staging acceptance\" accepts {sid} as {accepted.get(sid)}, and the moves record "
                     f"names {source['formula']} {source['resourceDigest']}")
    published = published_versions()
    for formula, version in VERSIONS.items():
        name = formula.rsplit("/", 1)[1]
        if published.get(name) != version:
            sys.exit(f"stop: the table \"Published full package version\" of the evidence names {name} "
                     f"{published.get(name)}, not {version}")
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


def prepared_files():
    """Each file of the prepared folder that a command runs or reads: (role, path relative to HOME, source)."""
    rows = [("program", PREPARED + "/owner_switch.py", PROGRAM),
            ("baseline instrument", PREPARED + "/inventory.py", INVENTORY_SCRIPT),
            ("probe program", PROBES_PROGRAM, PROBES_SOURCE),
            # The probe program imports owner_switch and harness_trial from its own folder.
            ("probe helper harness_trial.py", PREPARED + "/harness_trial.py", HARNESS_SOURCE)]
    for rel, src in runner_files():
        role = "staging runner" if rel == RUNNER else f"staging runner file {rel}"
        rows.append((role, f"{STAGING_RUNNER}/{rel}", src))
    return rows


def inputs():
    """Each file that a command runs or reads, by content.

    Each prepared file carries the digest of the repository file that
    `prepare` copies there. "$program" names the prepared program. The
    interpreters and the external tools are the dry run's rows ("The
    rehearsed plan"), not inputs of the plan.
    """
    mac_check = os.path.join(HOME, ".local/bin/mac-check")
    helper = os.path.join(HOME, ".local/lib/machine-config/check.py")
    rows = [{"role": "program", "path": "$program", "digest": file_digest(PROGRAM)}]
    rows += [{"role": role, "path": os.path.join(HOME, rel), "digest": file_digest(src)}
             for role, rel, src in prepared_files() if role != "program"]
    rows += [{"role": "mac-check command", "path": mac_check, "digest": file_digest(mac_check)},
             {"role": "mac-check helper", "path": helper, "digest": file_digest(helper)}]
    return rows


def classify(rows):
    """The entry writes of the five roots, the kept children and their table rows."""
    entries, table, kept = [], [], {}
    for root in ROOTS:
        if not os.path.isdir(os.path.join(HOME, root)):
            continue
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
        if OTHER_HOME and not os.path.lexists(os.path.join(HOME, rel)):
            continue
        item = {"path": rel, "identity": identity(os.path.join(HOME, rel))}
        before.append(item)
        after.append(item)
    for root in INVENTORY_ROOTS:
        children = {}
        names = os.listdir(os.path.join(HOME, root)) if OTHER_HOME and os.path.isdir(os.path.join(HOME, root)) \
            else [] if OTHER_HOME else os.listdir(os.path.join(HOME, root))
        for name in sorted(names):
            if f"{root}/{name}" not in CONTAINERS:
                children[name] = identity(os.path.join(HOME, root, name))
        before.append({"root": root, "entries": children})
        after.append({"root": root, "entries": {n: i for n, i in children.items() if f"{root}/{n}" not in gone}})
    return {"before": before, "after": after}


def draft(args):
    global HOME, PLAN_OUT, TABLE_OUT, OTHER_HOME
    if bool(args.home) != bool(args.out):
        sys.exit("usage: draft --home and --out go together")
    if args.home:
        HOME, OTHER_HOME = os.path.realpath(args.home), True
        os.makedirs(args.out, exist_ok=True)
        PLAN_OUT, TABLE_OUT = os.path.join(args.out, "plan.json"), os.path.join(args.out, "writes.tsv")
    rows = inventory_rows()
    entries, table, kept = classify(rows)
    settings = settings_values(rows)
    off, switched = switched_skills()
    files = [
        {"path": CLAUDE_SETTINGS, "type": "file", "original": original(CLAUDE_SETTINGS),
         "set": [v for v, _ in settings]},
        {"path": KNOWN_MARKETPLACES, "type": "file", "original": original(KNOWN_MARKETPLACES)},
        {"path": INSTALLED_PLUGINS, "type": "file", "original": original(INSTALLED_PLUGINS)},
        {"path": RAVEL_SETTINGS, "type": "file", "original": native_off_original(),
         "set": [{"key": ["skills"], "value": switched}]},
    ]
    for value, source in settings:
        table.append([CLAUDE_SETTINGS, "file", show_original(files[0]), ".".join(value["key"]),
                      json.dumps(value["value"]), *source])
    for w in files[1:3]:
        table.append([w["path"], "file", show_original(w), "", "", "", "Claude Code registry",
                      "written by the hooks installation's commands"])
    table.append([RAVEL_SETTINGS, "file", show_original(files[3]), "skills", "settings-switched.json", "",
                  "Ravel settings", "native-off writes the off value before the switch, so the original is its "
                                    "file; the switch writes the switched value last; restore writes the original "
                                    "whole and then runs native-off, which enforces the off value over an older "
                                    "backup's absent key, the one exception to whole-file restoration"])
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
            "trials": {sid: [rerun_trial(t, h) for t, h in TRIALS[sid]] for sid in GROVE_SOURCES},
            "staging": STAGING,
        },
        "probes": PROBES,
        "native_probe": NATIVE_PROBE,
        "mac_check": MAC_CHECK,
    }
    os.makedirs(os.path.dirname(PLAN_OUT), exist_ok=True)
    with open(PLAN_OUT, "w") as f:
        json.dump(plan, f, indent=2, ensure_ascii=False)
        f.write("\n")
    # The identity of each write in the real HOME now, beside its planned original.
    now = {w["path"]: real_now(w["path"]) for w in writes}
    with open(TABLE_OUT, "w") as f:
        f.write("path\ttype\toriginal\tnow\tkey\tswitched\tinventory_line\tclass\tdisposition\n")
        for row in table:
            current = now[row[0]]
            f.write("\t".join(str(x) for x in [*row[:3], "same" if current == row[2] else current, *row[3:]])
                    + "\n")
    print(f"draft: {len(entries)} entry writes, {len(files)} file writes, "
          f"{len(plan['inventory']['before'])} inventory items, {len(plan['inputs'])} command inputs")
    differ = sorted(p for p, i in now.items() if i != show_original(next(w for w in writes if w["path"] == p)))
    print(f"draft: writes whose real identity now differs from the planned original: {', '.join(differ) or 'none'}")
    print(f"draft: kept in the roots: {', '.join(sorted(kept))}")
    print(f"wrote {PLAN_OUT if OTHER_HOME else os.path.relpath(PLAN_OUT, ROOT)} and "
          f"{TABLE_OUT if OTHER_HOME else os.path.relpath(TABLE_OUT, ROOT)}")
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
        if w["path"] == RAVEL_SETTINGS:
            # Finish runs native-off before the switch, so the planned original is its result.
            if os.path.lexists(src):
                os.makedirs(os.path.dirname(dst), exist_ok=True)
                module.copy_tree(src, dst)
            code, text = program("native-off", "--home", home, env=disposable_env(os.path.dirname(home)))
            if code != 0 or module.restore_identity(dst) != w["original"].get("identity"):
                drift.append(f"{w['path']} (native-off exit {code})")
            continue
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


def program(*args, env=None):
    result = subprocess.run([sys.executable, "-I", PROGRAM, *args], capture_output=True, text=True, env=env)
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


# --- prepare ----------------------------------------------------------------------

def prepare(_args):
    """Write HOME/<state>/prepared from the draft and the repository, and change nothing else."""
    plan = json.load(open(PLAN_OUT, encoding="utf-8"))
    files = [(rel, src) for _, rel, src in prepared_files()]
    files += [(PREPARED + "/plan.json", PLAN_OUT), (PREPARED + "/owner_switch_dry_run.py", DRY_RUN_SOURCE),
              (PREPARED + "/INSTRUCTIONS.md", INSTRUCTIONS_SOURCE)]
    planned = {os.path.join(HOME, PREPARED, "owner_switch.py") if r["path"] == "$program" else r["path"]: r["digest"]
               for r in plan["inputs"]}
    problems = []
    for rel, src in files:
        dst = os.path.join(HOME, rel)
        want = planned.get(dst)
        if want is not None and file_digest(src) != want:
            problems.append(f"{os.path.relpath(src, ROOT)} has another digest than the plan's input {dst}")
        if os.path.lexists(dst) and (os.path.islink(dst) or file_digest(dst) != file_digest(src)):
            problems.append(f"{dst} exists with other content")
    missing = sorted(set(planned) - {os.path.join(HOME, rel) for rel, _ in files} - {
        r["path"] for r in plan["inputs"] if not r["path"].startswith(os.path.join(HOME, PREPARED) + os.sep)})
    if missing:
        problems.append("the plan names prepared inputs that prepare does not copy: " + ", ".join(missing))
    if problems:
        sys.exit("stop: " + "; ".join(problems))
    written = 0
    for rel, src in files:
        dst = os.path.join(HOME, rel)
        if os.path.exists(dst):
            continue
        os.makedirs(os.path.dirname(dst), 0o755, exist_ok=True)
        tmp = dst + ".prepare-tmp"
        shutil.copyfile(src, tmp)
        os.chmod(tmp, 0o755 if os.access(src, os.X_OK) else 0o644)
        os.replace(tmp, dst)
        written += 1
    for rel, src in files:
        if file_digest(os.path.join(HOME, rel)) != file_digest(src):
            sys.exit(f"stop: {os.path.join(HOME, rel)} does not read back as {src}")
    print(f"prepare: {len(files)} files in {os.path.join(HOME, PREPARED)}, {written} written, "
          f"{len(files) - written} already equal")
    print(f"prepare: plan {file_digest(PLAN_OUT)}")
    return 0


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n", 1)[0])
    sub = parser.add_subparsers(dest="command", required=True)
    d = sub.add_parser("draft")
    d.add_argument("--home")
    d.add_argument("--out")
    sub.add_parser("check").add_argument("--out", required=True)
    sub.add_parser("prepare")
    args = parser.parse_args()
    return {"draft": draft, "check": check, "prepare": prepare}[args.command](args)


if __name__ == "__main__":
    sys.exit(main())
