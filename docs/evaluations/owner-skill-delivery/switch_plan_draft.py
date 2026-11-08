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
and `run --dry-run`, seven controls that must fail, and one control that must
pass: a change inside .claude/skills/synced and .codex/skills/.system. It
stops unless the draft holds no process, MCP or job pattern ("session",
"watch" or "jobs"). Last, it records the installed path and canonical path of
each executable of the four tools, and it runs the program's narrow process
inspection on the real process table. OUT holds record.json, dry-run.txt and
processes.txt. OUT must not exist. The real HOME, launchd and the process
table are read only.
"""

import argparse, copy, hashlib, importlib.util, json, os, re, shutil, subprocess, sys

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

# "outside": native residue that must stay unchanged. "Concurrent writers"
# compares each ordinary child of both native roots. A probe session can
# rewrite .claude/skills/synced and .codex/skills/.system, so their entries
# name neither container, and the program skips exactly those two. No plugin
# version directory, because each session writes .in_use/<pid> there.
OUTSIDE_PATHS = (".agents/.grove",)
OUTSIDE_ROOTS = (".claude/skills", ".codex/skills", ".agents/skills", ".pi/agent/skills", ".pi/agent/extensions",
                 ".claude/agents", ".codex/agents")

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
            if f"{root}/{name}" not in moved and f"{root}/{name}" not in CONTAINERS:
                entries[name] = identity(os.path.join(HOME, root, name))
        outside.append({"root": root, "entries": entries})
    plan = {
        "schema": "owner-switch-plan/1",
        "state": STATE,
        "manual": MANUAL,
        "parts": parts,
        "outside": outside,
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
    """A broken draft must not be admitted, and a changed layout is not native, except inside a platform subtree."""
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
    refused("process-patterns", lambda p: p.update(watch=[r"^(\S*/)?codex(\s|$)"]),
            "the plan holds watch")

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
    sibling = os.path.join(home, ".codex/skills/quint-lang")
    changed("new-sibling-of-system", lambda: os.makedirs(sibling), lambda: os.rmdir(sibling))

    # A change confined to the two platform subtrees leaves the layout native.
    inside = [os.path.join(home, ".claude/skills/synced"), os.path.join(home, ".codex/skills/.system")]
    for d in inside:
        os.makedirs(os.path.join(d, "probe-skill"))
    try:
        code, text = program("run", "--dry-run", "--plan", PLAN_OUT, "--home", home)
    finally:
        for d in inside:
            shutil.rmtree(d)
    results.append({"control": "inside-platform-subtrees", "exit": code, "passed": code == 0 and "state: native" in text,
                    "output": text.strip()[:600]})
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
    patterns = [name for name in module.REMOVED_PLAN_FIELDS if name in plan]
    tools, listing, text = process_listing(module)
    with open(os.path.join(out, "processes.txt"), "w") as f:
        f.write(text)
    record = {
        "plan": os.path.relpath(PLAN_OUT, ROOT), "plan_digest": hashlib.sha256(raw).hexdigest(),
        "program_digest": module.PROGRAM_DIGEST, "python": sys.version.split()[0],
        "entries_copied": copied, "dry_run": dry, "controls": results, "process_patterns": patterns,
        "installed_tools": tools, "processes": listing,
    }
    with open(os.path.join(out, "record.json"), "w") as f:
        f.write(json.dumps(record, indent=2).replace(out, "<OUT>"))
        f.write("\n")
    passed = code == 0 and dry["native"] and listed and all(r["passed"] for r in results) and not patterns
    print(f"plan {record['plan_digest']}, program {record['program_digest']}")
    print(f"dry run: exit {code}, native {dry['native']}, each part and its inverse listed {listed}")
    for r in results:
        print(f"control {r['control']}: {'passed' if r['passed'] else 'FAILED'} (exit {r['exit']})")
    print(f"process patterns in the draft: {', '.join(patterns) or 'none'}")
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
