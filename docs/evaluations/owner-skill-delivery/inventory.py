"""Print the owner's skill-installation baseline as TSV on standard output.

The script only reads. It walks the Claude Code, Codex and Pi discovery
roots, both Claude Code plugin registries and Codex's installed plugin cache.
It also reads the user hook and MCP files, the agent definitions and the tool
installations that skills need. A final sweep finds every SKILL.md under the four harness homes
and reports each one that no walked root or classified container holds. Paths
use ~ for HOME. The script prints no file content except SKILL.md frontmatter
names.

Change identity. The digest16 column holds the first 16 hexadecimal digits of
a SHA-256 over the state that defines the entry:

- A skill tree: each sorted relative path, the executable bit and content of
  each file, and the target of each file or directory link.
- A Claude Code plugin or Codex installed plugin: its whole version directory.
  The Claude Code plugin row leaves out the entry .in_use at the top level of
  that directory. Claude Code writes .in_use/<pid> there for each running
  session that loaded the version, and the session erases its marker when it
  ends. The marker holds the PID and the start time of that session, so it
  identifies a reader, not the plugin. Without the exclusion, the row would
  change with each session that runs the script. The file count leaves it out
  too. A nested .in_use and .orphaned_at stay in the digest.
- A user hook: its matcher, its definition and each file under HOME that its
  command names. One row exists for each hook.
- An MCP server: its definition. The script replaces values under env and
  header tables, and values of keys that name a token, secret, password or
  key, with their names. A credential rotation is therefore not a capability
  change.
- An agent definition, a Pi extension or a skill override: its content.
- A tool: its resolved executable, bundle metadata or clone revision.
- A coverage row: the sorted SKILL.md paths of its container.

A row that the script cannot observe says NEEDS-REVIEW. Built-in Claude Code
skills have no files, so their digest says "unverified" and the session's own
skill listing is their only evidence. No row holds a raw configuration value.

The design in docs/specs/owner-skill-delivery.md owns each classification and
destination. This script encodes them so that a later inventory can compare
entries one by one with docs/owner-skill-inventory.tsv.

Two exceptions of "Concurrent writers" in the design:

- The platform subtrees are only ~/.claude/skills/synced and
  ~/.codex/skills/.system. A child with either name in another root, and each
  other hidden child of a root, is an ordinary child. A platform subtree that
  is not a plain directory gets an UNCLASSIFIED row.
- An enabled guardrail@personal is the classified native residue only if the
  marketplace personal in ~/.claude/plugins/known_marketplaces.json has the
  source directory /opt/homebrew/opt/personal and the plugin holds only the
  guardrail skill. Claude Code loads the plugin from the directory that the
  marketplace's .claude-plugin/marketplace.json names, and the rule reads
  that directory and the plugin-cache copy at installPath. In each one,
  skills/ must hold only guardrail, and .claude-plugin/plugin.json must
  declare no skills path. The marketplace entry must declare no skills, and
  its directory must lie inside /opt/homebrew/opt/personal. Otherwise the
  rows say UNCLASSIFIED.

The hook adapters of Grove's keg are native residue too ("Hook adapters" of
docs/specs/codex-skill-provisioning.md). Each must hold no skill, and each
marketplace file must name the private payload
libexec/skill-sources/writing/craft/technical-communication:

- An enabled technical-communication-hooks@writing-hooks needs the Claude
  Code marketplace writing-hooks with the source directory
  /opt/homebrew/opt/grove, the `opt` path that Claude Code stores.
- A Codex cache entry of writing-hooks needs [marketplaces.writing-hooks] in
  ~/.codex/config.toml with the local source of the keg that
  /opt/homebrew/opt/grove names now. Codex stores that canonical path, so a
  registration of a removed or earlier keg says UNCLASSIFIED, and its note
  names the documented repair.
- A Pi extension link that names the writing alias under
  /opt/homebrew/share/ravel/skills must resolve into that private payload.

Otherwise the rows say UNCLASSIFIED.

Usage:
    python3 -I inventory.py > inventory.tsv
    python3 -I inventory.py compare BEFORE.tsv AFTER.tsv

`compare` keys each row by its harness, location and entry. It prints each
key that only one side holds, each key whose rows differ with the differing
columns, and each anomaly row of AFTER. It skips only the rows inside the two
platform subtrees. Exit status 0 means no difference and no anomaly.

Controls: controls.py beside this script builds a fixture HOME and checks that
each class of change changes its entry. Set CONTROL_ROOT to a scratch directory
that holds an unknown skill directory and a dangling link. The output must then
show one UNCLASSIFIED row and one dangling row under the harness "control".
Without CONTROL_ROOT, no row may say UNCLASSIFIED, UNASSIGNED, UNCOVERED or
NEEDS-REVIEW.
"""
import hashlib, json, os, plistlib, re, subprocess, sys, tomllib
H = os.path.expanduser("~")
def t(p): return p.replace(H, "~", 1) if p.startswith(H) else p
def h16(b): return hashlib.sha256(b).hexdigest()[:16]
def canon(o): return json.dumps(o, sort_keys=True, separators=(",", ":")).encode()
def ls(p): return sorted(os.listdir(p)) if os.path.isdir(p) else []
def xbit(p): return b"x" if os.stat(p).st_mode & 0o111 else b"-"
def fm(path):
    try: txt = open(path, encoding="utf-8").read()
    except Exception: return {}
    m = re.match(r"^---\n(.*?)\n---", txt, re.S)
    out = {}
    if m:
        for line in m.group(1).splitlines():
            for k in ("name", "harnesses", "disable-model-invocation"):
                if line.startswith(k + ":"): out[k] = line.split(":", 1)[1].strip().strip('"\'')
    return out
def digest(d, skip=()):
    h = hashlib.sha256(); n = 0
    for root, dirs, files in os.walk(d):
        if root == d and skip:
            dirs[:] = [x for x in dirs if x not in skip]; files = [x for x in files if x not in skip]
        dirs.sort()
        links = [x for x in dirs if os.path.islink(os.path.join(root, x))]
        for f in sorted(files + links):
            p = os.path.join(root, f); rel = os.path.relpath(p, d).encode()
            if os.path.islink(p): h.update(b"L" + rel + b"\0" + os.readlink(p).encode() + b"\0")
            else:
                with open(p, "rb") as fh: h.update(b"F" + xbit(p) + rel + b"\0" + hashlib.sha256(fh.read()).digest())
            n += 1
    return h.hexdigest()[:16], n
# Claude Code writes .in_use/<pid> into each plugin version directory that a
# running session loaded, and erases it when that session ends. The marker
# names a reader, not the plugin, so the plugin row leaves it out.
READER_MARKERS = (".in_use",)
def filedigest(p):
    with open(p, "rb") as fh: return h16(xbit(p) + fh.read())

# "Concurrent writers" of the design: the only two platform subtrees, each at
# its exact root. A child with the same name in another root is an ordinary
# child. The walker writes the rows inside a subtree at one location, and
# compare() skips exactly those locations. The row of a subtree that is not a
# plain directory sits at the root's location, so compare() still sees it.
PLATFORM_SUBTREES = {("~/.claude/skills", "synced"), ("~/.codex/skills", ".system")}
SKIPPED = {("claude", "~/.claude/skills/synced/<account>"), ("codex", "~/.codex/skills/.system")}
ANOMALIES = ("UNCLASSIFIED", "UNASSIGNED", "UNCOVERED", "NEEDS-REVIEW")
def compare(before_path, after_path):
    """Print each difference between two inventories; return 0 only if there is none."""
    sides, skipped = [], 0
    for path in (before_path, after_path):
        with open(path, encoding="utf-8") as fh: lines = fh.read().splitlines()
        if not lines or lines[0] != "\t".join(HEADER): sys.exit(f"compare: {path} is not an inventory")
        keyed = {}
        for line in lines[1:]:
            r = line.split("\t")
            if (r[0], r[1]) in SKIPPED: skipped += 1; continue
            keyed.setdefault((r[0], r[1], r[2]), []).append(r)
        sides.append({k: sorted(v) for k, v in keyed.items()})
    before, after = sides; found = 0
    for k in sorted(before.keys() | after.keys()):
        if k not in after: print("removed\t" + "\t".join(k)); found += 1
        elif k not in before: print("added\t" + "\t".join(k)); found += 1
        elif before[k] != after[k]:
            cols = sorted({HEADER[i] for b, a in zip(before[k], after[k]) for i in range(len(HEADER)) if b[i] != a[i]})
            print("changed\t" + "\t".join(k) + "\t" + (",".join(cols) or "row count")); found += 1
    for k, rs in sorted(after.items()):
        for r in rs:
            if any(f in ANOMALIES for f in r): print("anomaly\t" + "\t".join(r)); found += 1
    print(f"compare: {found} differences; {skipped} rows inside the platform subtrees skipped", file=sys.stderr)
    return 1 if found else 0
HEADER = ["harness", "location", "entry", "form", "identity", "digest16", "files", "class", "source_owner", "capabilities", "disposition", "note"]
if sys.argv[1:2] == ["compare"]:
    if len(sys.argv) != 4: sys.exit("usage: inventory.py compare BEFORE.tsv AFTER.tsv")
    sys.exit(compare(sys.argv[2], sys.argv[3]))

DEST = {}
for s in "grove configure-grove grove-finish grove-planning grove-review-planning grove-integrate-review-planning".split(): DEST[s] = "grove-core"
for s in "grove-requirements grove-review-requirements grove-integrate-review-requirements".split(): DEST[s] = "requirements"
for s in "grove-design grove-review-design grove-integrate-review-design decision-records codebase-design cli-tool-design model-led-development".split(): DEST[s] = "design"
for s in "grove-impl grove-prototype grove-review-impl grove-review-prototype grove-integrate-review-impl grove-integrate-review-prototype coding-style-bash coding-style-elixir coding-style-python coding-style-rust coding-style-swift coding-style-typescript using-codebase-memory simplify-project doubt-driven-development".split(): DEST[s] = "engineering"
for s in "grove-research-a grove-research-b grove-combine-research".split(): DEST[s] = "research"
for s in "grove-draft grove-copy-edit grove-art grove-proof technical-communication writing-code-walkthroughs".split(): DEST[s] = "writing"
for s in "machine-environment using-jujutsu guardrail authoring-conventions".split(): DEST[s] = "personal"
DEST["using-testanyware"] = "testanyware"
CAPS = {
    "using-testanyware": "testanyware CLI, TestAnyware.app, ~/.config/testanyware/config.rhai, VM data/state dirs (see tool rows)",
    "quint-lang": "quint CLI; quint-kb MCP from ~/.quint-llm-kit/repo (see tool rows)",
    "quint-modeling": "quint CLI; quint-kb MCP from ~/.quint-llm-kit/repo (see tool rows)",
    "quint-execute-spec": "quint CLI; quint-kb MCP from ~/.quint-llm-kit/repo (see tool rows)",
    "using-codebase-memory": "codebase-memory-mcp MCP and CLI (see tool rows)",
    "technical-communication": "python3; scripts/ste-lint.py; always-on hooks (Claude only)",
    "guardrail": "Claude Code frontmatter PreToolUse hook and /guardrail command",
    "machine-environment": "mise, task, Obsidian manual",
    "using-jujutsu": "jj",
    "using-superpowers": "SessionStart hook injection (Claude plugin)",
}
rows = []
def row(harness, location, entry, form, ident, dg, files, cls, owner, caps, dispo, note=""):
    rows.append([harness, t(location), entry, form, ident or "-", dg, str(files), cls, owner, caps or "-", dispo, note or "-"])
def need_json(path):
    try:
        with open(path, encoding="utf-8") as fh: return json.load(fh)
    except FileNotFoundError:
        row("input", os.path.dirname(path), os.path.basename(path), "required input absent", "-", "-", 0, "input", "-", "-", "NEEDS-REVIEW")
        return {}
def need_toml(path):
    try:
        with open(path, "rb") as fh: return tomllib.load(fh)
    except FileNotFoundError:
        row("input", os.path.dirname(path), os.path.basename(path), "required input absent", "-", "-", 0, "input", "-", "-", "NEEDS-REVIEW")
        return {}
walked = []

# --- Claude Code plugins
inst = need_json(f"{H}/.claude/plugins/installed_plugins.json")
claude_settings = need_json(f"{H}/.claude/settings.json")
enabled = claude_settings.get("enabledPlugins", {})
# "Native residue" and "Concurrent writers": an enabled guardrail@personal is a
# classified native capability only from the personal keg's registered
# marketplace, with guardrail as its only skill. The registration stores the
# path that `claude plugin marketplace add "$(brew --prefix)/opt/personal"`
# received (docs/specs/codex-skill-provisioning.md, "Hook adapters").
# controls.py runs a copy of this script in which this literal names a fixture
# keg. The literal must therefore stay the only spelling of the origin.
RESIDUE = "guardrail@personal"
PERSONAL_PREFIX = "/opt/homebrew/opt/personal"
PERSONAL_SOURCE = {"source": "directory", "path": PERSONAL_PREFIX}
try:
    with open(f"{H}/.claude/plugins/known_marketplaces.json", encoding="utf-8") as fh: known = json.load(fh)
except (OSError, ValueError): known = {}
def read_json(path):
    """The parsed file; {} if it is absent, None if it cannot be read as a JSON object."""
    try:
        with open(path, encoding="utf-8") as fh: obj = json.load(fh)
    except FileNotFoundError: return {}
    except (OSError, ValueError): return None
    return obj if isinstance(obj, dict) else None
def sole_guardrail(d):
    """Why the plugin directory d gives Claude Code a skill other than guardrail, or None.
    A manifest skills path adds to the skills/ scan: "How each key combines with its
    default location" in https://code.claude.com/docs/en/plugins-reference."""
    manifest = read_json(os.path.join(d, ".claude-plugin", "plugin.json"))
    if manifest is None: return "unreadable .claude-plugin/plugin.json"
    if "skills" in manifest: return "plugin.json declares skills " + json.dumps(manifest["skills"])
    extra = [x for x in ls(os.path.join(d, "skills")) if x != "guardrail"]
    if extra: return "skills other than guardrail: " + ", ".join(extra)
    if not os.path.isfile(os.path.join(d, "skills", "guardrail", "SKILL.md")): return "no guardrail skill"
    return None
def residue_problem(p):
    """Why guardrail@personal with the cache copy p is not the classified residue, or None."""
    src = known.get("personal", {}).get("source") if isinstance(known.get("personal"), dict) else None
    if src != PERSONAL_SOURCE: return f"marketplace personal is not registered from {PERSONAL_PREFIX}"
    # Claude Code loads a directory marketplace's plugin in place, from the
    # entry's source under the marketplace root, not from the cache copy.
    market = os.path.join(PERSONAL_PREFIX, ".claude-plugin", "marketplace.json")
    plugins = (read_json(market) or {}).get("plugins")
    name = RESIDUE.split("@")[0]
    entry = next((e for e in plugins if isinstance(e, dict) and e.get("name") == name), None) if isinstance(plugins, list) else None
    if entry is None: return f"{market} names no {name} plugin"
    if not isinstance(entry.get("source"), str): return f"the {name} entry of {market} names no directory"
    # A marketplace entry's skills append to the plugin's: "Marketplace entries
    # and the manifest" in https://code.claude.com/docs/en/plugins-reference.
    if "skills" in entry: return f"the {name} entry of {market} declares skills"
    loaded = os.path.normpath(os.path.join(PERSONAL_PREFIX, entry["source"]))
    if not loaded.startswith(PERSONAL_PREFIX + os.sep): return f"the loaded directory {loaded} lies outside {PERSONAL_PREFIX}"
    for label, d in (("loaded directory", loaded), ("plugin-cache copy", p)):
        problem = sole_guardrail(d)
        if problem: return f"{label} {t(d)}: {problem}"
    return None
# "Hook adapters" of docs/specs/codex-skill-provisioning.md and "The hook
# adapters on the projected keg" of docs/homebrew-skill-delivery-evidence.md:
# Claude Code stores the `opt` path that `claude plugin marketplace add
# "$(brew --prefix)/opt/grove"` received, and Codex stores the canonical path
# of the keg. controls.py runs a copy of this script in which these literals
# name a fixture prefix, so each must stay the only spelling of its path.
GROVE_PREFIX = "/opt/homebrew/opt/grove"
SKILL_DIRECTORY = "/opt/homebrew/share/ravel/skills"
HOOKS_MARKET = "writing-hooks"
HOOKS_PAYLOAD = "libexec/skill-sources/writing/craft/technical-communication"
CLAUDE_HOOKS = "technical-communication-hooks@writing-hooks"
CODEX_HOOKS = "technical-communication@writing-hooks"
HOOKS_OWNER = "Grove keg marketplace writing-hooks"
def skill_free(d):
    """Why the plugin directory d gives a harness a skill, or None."""
    if not os.path.isdir(d): return "no plugin directory"
    if os.path.isdir(os.path.join(d, "skills")): return "it holds skills/"
    for m in (".claude-plugin/plugin.json", ".codex-plugin/plugin.json"):
        manifest = read_json(os.path.join(d, m))
        if manifest is None: return f"unreadable {m}"
        if "skills" in manifest: return f"{m} declares skills " + json.dumps(manifest["skills"])
    return None
def hooks_entry(root, manifest, name):
    """The plugin entry NAME of the marketplace file ROOT/MANIFEST, or a reason."""
    plugins = (read_json(os.path.join(root, manifest)) or {}).get("plugins")
    entry = next((e for e in plugins if isinstance(e, dict) and e.get("name") == name), None) if isinstance(plugins, list) else None
    return entry if entry is not None else f"{os.path.join(root, manifest)} names no {name} plugin"
def claude_hooks_problem(p):
    """Why the enabled Claude Code hooks plugin with the cache copy p is not the classified residue, or None."""
    src = known.get(HOOKS_MARKET, {}).get("source") if isinstance(known.get(HOOKS_MARKET), dict) else None
    if src != {"source": "directory", "path": GROVE_PREFIX}: return f"marketplace {HOOKS_MARKET} is not registered from {GROVE_PREFIX}"
    entry = hooks_entry(GROVE_PREFIX, ".claude-plugin/marketplace.json", CLAUDE_HOOKS.split("@")[0])
    if isinstance(entry, str): return entry
    if entry.get("source") != "./" + HOOKS_PAYLOAD: return f"the marketplace entry names {entry.get('source')}, not ./{HOOKS_PAYLOAD}"
    for label, d in (("loaded directory", os.path.join(GROVE_PREFIX, HOOKS_PAYLOAD)), ("plugin-cache copy", p)):
        problem = skill_free(d)
        if problem: return f"{label} {t(d)}: {problem}"
    return None
active_claude = set()
for key, entries in sorted(inst.get("plugins", {}).items()):
    for e in entries:
        p = e["installPath"]; plugin, market = key.split("@")
        active_claude.add(os.path.realpath(p)); walked.append(p)
        en = enabled.get(key)
        residue = en is True and key in (RESIDUE, CLAUDE_HOOKS)
        problem = (residue_problem(p) if key == RESIDUE else claude_hooks_problem(p)) if residue else None
        comps = []
        for k in ("skills", "commands", "agents"):
            if os.path.isdir(os.path.join(p, k)): comps.append(k)
        if os.path.isfile(os.path.join(p, "hooks", "hooks.json")): comps.append("hooks")
        if os.path.isfile(os.path.join(p, ".mcp.json")): comps.append("mcp")
        if market == "claude-plugins-official" and plugin.endswith("-lsp"): comps.append("lsp(marketplace entry)")
        if market == "linkuistics": cls, owner = "owner-pack", "Linkuistics/grove marketplace"
        elif plugin in ("superpowers", "quint-llm-kit"): cls, owner = "third-party-pack", market
        elif "skills" in comps: cls, owner = ("third-party-pack" if en else "disabled-third-party"), market
        else: cls, owner = "capability-only", market
        if not en: cls = "disabled-third-party" if market != "linkuistics" else cls
        if residue: cls, owner = ("unknown", "unknown") if problem else ("native-residue", "personal keg marketplace" if key == RESIDUE else HOOKS_OWNER)
        if residue: dispo = "UNCLASSIFIED" if problem else "native-keep (native residue)"
        elif market == "linkuistics" and plugin == "linkuistics": dispo = "disable; hooks move to skill-free plugin; guardrail to native-only plugin"
        elif market == "linkuistics": dispo = "disable"
        elif plugin == "superpowers": dispo = "disable; SessionStart prose becomes Ravel mandatory file"
        elif plugin == "quint-llm-kit": dispo = "disable"
        elif not en: dispo = "outside (unchanged)"
        else: dispo = "native-keep"
        orphan = "yes" if os.path.exists(os.path.join(p, ".orphaned_at")) else "no"
        if os.path.isdir(p): pdg, pn = digest(p, READER_MARKERS)
        else: pdg, pn, dispo = "-", 0, "NEEDS-REVIEW"
        row("claude", os.path.dirname(os.path.dirname(p)), key, f"plugin v{e['version']} sha {(e.get('gitCommitSha') or '-')[:12]} enabled={en} orphan={orphan}", plugin, pdg, pn, cls, owner, "+".join(comps), dispo, problem)
        sk = os.path.join(p, "skills")
        for d in ls(sk):
            sd = os.path.join(sk, d)
            if not os.path.isfile(os.path.join(sd, "SKILL.md")): continue
            f = fm(os.path.join(sd, "SKILL.md")); dg, n = digest(sd)
            if market == "linkuistics":
                tgt = DEST.get(d, "UNASSIGNED")
                if d == "guardrail": dsp = "native-only (PersonalSkills Claude plugin; outside Ravel roots)"
                elif d == "technical-communication": dsp = f"ravel:{tgt}; hooks via skill-free plugin"
                else: dsp = f"ravel:{tgt}"
            elif residue: dsp = "UNCLASSIFIED" if problem else "native-keep (native guardrail; outside Ravel roots)"
            elif plugin == "superpowers": dsp = "ravel:superpowers" + ("; mandatory file" if d == "using-superpowers" else "")
            elif plugin == "quint-llm-kit": dsp = "ravel:quint"
            elif not en: dsp = "outside (plugin disabled)"
            else: dsp = "UNCLASSIFIED"
            note = "; ".join(x for x in [("harnesses=" + f["harnesses"]) if f.get("harnesses") not in (None, "[any]") else "", "user-invoked only" if f.get("disable-model-invocation") == "true" else ""] if x)
            row("claude", sd.rsplit("/skills/", 1)[0] + "/skills", f"{plugin}:{d}", "plugin skill", f.get("name"), dg, n, cls if en else "disabled-third-party", owner, CAPS.get(d), dsp, note)
for name, value in sorted(claude_settings.get("skillOverrides", {}).items()):
    row("claude", "~/.claude/settings.json skillOverrides", name, f"skill override {value}", name, h16(canon([name, value])), 0, "tool-managed" if name == "codebase-memory" else "unknown", "owner configuration", "-", "native-keep" if name == "codebase-memory" else "UNCLASSIFIED")

# --- filesystem discovery roots
def walk_root(harness, root, cls_fn):
    rp = os.path.expanduser(root)
    if not os.path.isdir(rp): return
    walked.append(rp)
    for e in sorted(os.listdir(rp)):
        p = os.path.join(rp, e)
        if os.path.islink(p):
            tgt = os.readlink(p); real = os.path.realpath(p); ok = os.path.exists(p)
            form = f"link -> {t(tgt)}" + ("" if ok else " (DANGLING)")
        elif os.path.isdir(p): real, ok, form = p, True, "directory"
        else: continue
        if (root, e) in PLATFORM_SUBTREES:
            if form != "directory":
                row(harness, root, e, form, "-", "-", 0, "unknown", "unknown", "-", "UNCLASSIFIED", "a platform subtree must be a plain directory")
                continue
            for s in sorted(os.listdir(real)):
                sp = os.path.join(real, s)
                if s.startswith(".") or not os.path.isdir(sp): continue
                if e == "synced":
                    for s2 in sorted(os.listdir(sp)):
                        sp2 = os.path.join(sp, s2)
                        if os.path.isfile(os.path.join(sp2, "SKILL.md")):
                            dg, n = digest(sp2)
                            row(harness, root + "/synced/<account>", s2, "synced directory", fm(os.path.join(sp2, "SKILL.md")).get("name"), dg, n, "platform-managed", "claude.ai account sync", "-", "native-keep")
                    continue
                dg, n = digest(sp)
                row(harness, root + "/.system", s, "system directory", fm(os.path.join(sp, "SKILL.md")).get("name"), dg, n, "platform-managed", "Codex system skills", "-", "native-keep")
            continue
        f = fm(os.path.join(real, "SKILL.md")) if ok else {}
        dg, n = digest(real) if ok else ("-", 0)
        cls, owner, dispo, note = cls_fn(e, form, ok, real)
        row(harness, root, e, form, f.get("name"), dg, n, cls, owner, CAPS.get(e), dispo, note)

SNAP = f"{H}/.agents/.grove/"
def codex_cls(e, form, ok, real):
    if not ok: return "dangling", "terminal-browser cask (absent)" if e == "terminal-browser" else "unknown", "deactivate (dangling; no reader)", ""
    if real.startswith(SNAP):
        tgt = DEST.get(e, "UNASSIGNED")
        return "owner-pack", "Grove 22.8 provisioning snapshot", f"deactivate after writer stops; ravel:{tgt}", "duplicate of the same snapshot entry in the other Codex user root"
    if real.startswith(f"{H}/.quint-llm-kit/"): return "third-party-pack", "quint-llm-kit clone ~/.quint-llm-kit/repo", "deactivate; ravel:quint", "clone retained for quint-kb MCP"
    if e == "codebase-memory": return "tool-managed", "codebase-memory-mcp installer", "native-keep", "same bytes in both Codex user roots; overlaps using-codebase-memory triggers"
    return "stale-copy", "superpowers copy (no recorded provenance)", "deactivate; ravel:superpowers", "differs from superpowers 7.0.0 when digest differs"
def claude_cls(e, form, ok, real):
    if not ok: return "dangling", "terminal-browser cask (absent)", "deactivate (dangling; no reader)", ""
    if e == "codebase-memory": return "tool-managed", "codebase-memory-mcp installer", "native-keep", "disabled by skillOverrides; differs from the Codex copy"
    if e == "grove": return "stale-copy", "earlier Grove personal copy", "deactivate; ravel:grove-core", "duplicates grove:grove identity; contains retired SIGNAL files"
    return "unknown", "unknown", "UNCLASSIFIED", ""
def pi_cls(e, form, ok, real):
    if not ok: return "dangling", "plugins/install.sh (moved checkout)", "deactivate (dangling; no reader)", "Pi is outside owner policy routes"
    if e == "codebase-memory": return "tool-managed", "codebase-memory-mcp installer", "native-keep", ""
    return "unknown", "unknown", "UNCLASSIFIED", ""
walk_root("claude", "~/.claude/skills", claude_cls)
walk_root("codex", "~/.codex/skills", codex_cls)
walk_root("codex", "~/.agents/skills", codex_cls)
walk_root("pi", "~/.pi/agent/skills", pi_cls)
if os.environ.get("CONTROL_ROOT"): walk_root("control", os.environ["CONTROL_ROOT"], claude_cls)

# --- Codex marketplace sources (catalogs of bundled and runtime plugins)
for base, label in ((f"{H}/.codex/.tmp/bundled-marketplaces/openai-bundled", "openai-bundled"), (f"{H}/.cache/codex-runtimes/codex-primary-runtime/plugins/openai-primary-runtime", "openai-primary-runtime")):
    pl = os.path.join(base, "plugins")
    if not os.path.isdir(pl): continue
    walked.append(pl)
    for plug in ls(pl):
        sk = os.path.join(pl, plug, "skills")
        for s in ls(sk):
            sp = os.path.join(sk, s)
            if os.path.isfile(os.path.join(sp, "SKILL.md")):
                dg, n = digest(sp)
                row("codex", base, f"{plug}@{label}:{s}", "marketplace source skill", fm(os.path.join(sp, "SKILL.md")).get("name"), dg, n, "platform-managed", "Codex " + label, "-", "native-keep")

# --- Codex installed plugin cache: every marketplace, plugin and cached version
codex_cfg = need_toml(f"{H}/.codex/config.toml")
pstate = {k: v.get("enabled") for k, v in codex_cfg.get("plugins", {}).items() if isinstance(v, dict)}
CODEX_MARKETS = {"openai-bundled": "Codex openai-bundled", "openai-primary-runtime": "Codex openai-primary-runtime", "openai-curated-remote": "Codex remote install (ChatGPT account)"}
pc = f"{H}/.codex/plugins/cache"
def codex_hooks_problem(key, vd):
    """Why a Codex cache entry of writing-hooks is not the classified residue, or None."""
    if key != CODEX_HOOKS: return f"{HOOKS_MARKET} offers no plugin {key.split('@')[0]}"
    mk = codex_cfg.get("marketplaces", {}).get(HOOKS_MARKET)
    keg = os.path.realpath(GROVE_PREFIX)
    if not isinstance(mk, dict) or mk.get("source_type") != "local" or mk.get("source") != keg:
        got = mk.get("source") if isinstance(mk, dict) else None
        return (f"[marketplaces.{HOOKS_MARKET}] names {got}, not the linked keg {keg}; run "
                f"`codex plugin marketplace add \"$(brew --prefix)/opt/grove\"`, then `codex plugin add {CODEX_HOOKS}`")
    entry = hooks_entry(keg, ".agents/plugins/marketplace.json", CODEX_HOOKS.split("@")[0])
    if isinstance(entry, str): return entry
    if entry.get("source") != {"source": "local", "path": "./" + HOOKS_PAYLOAD}:
        return f"the marketplace entry names {json.dumps(entry.get('source'))}, not the local ./{HOOKS_PAYLOAD}"
    problem = skill_free(vd)
    return f"cache copy {t(vd)}: {problem}" if problem else None
for market in ls(pc):
    if market.startswith(".") or not os.path.isdir(os.path.join(pc, market)): continue
    known = market in CODEX_MARKETS
    cls, owner = ("platform-managed", CODEX_MARKETS[market]) if known else ("unknown", "unknown")
    for plug in ls(os.path.join(pc, market)):
        pd = os.path.join(pc, market, plug)
        if plug.startswith(".") or not os.path.isdir(pd): continue
        key = f"{plug}@{market}"
        if key in pstate: state = "enabled=" + str(pstate[key]).lower()
        elif os.path.isfile(os.path.join(pd, ".codex-remote-plugin-install.json")): state = "remote install record; no config.toml entry"
        else: state = "no local record"
        names = [v for v in ls(pd) if not v.startswith(".")]
        vers = [v for v in names if os.path.isdir(os.path.join(pd, v)) and not os.path.islink(os.path.join(pd, v))]
        links = "".join(f"; link {v} -> {os.path.basename(os.readlink(os.path.join(pd, v)))}" for v in names if os.path.islink(os.path.join(pd, v)))
        for v in vers:
            vd = os.path.join(pd, v); walked.append(vd)
            dg, n = digest(vd)
            comps = "+".join(k for k in ("skills", "assets", "scripts", ".mcp.json", "hooks") if os.path.exists(os.path.join(vd, k))) or "-"
            vcls, vowner, dispo, note = cls, owner, "native-keep" if known else "UNCLASSIFIED", ""
            if market == HOOKS_MARKET:
                note = codex_hooks_problem(key, vd)
                vcls, vowner, dispo = ("unknown", "unknown", "UNCLASSIFIED") if note else ("native-residue", HOOKS_OWNER, "native-keep (native residue)")
            row("codex", vd, key, f"installed plugin v{v} {state}; cached versions={len(vers)}{links}", plug, dg, n, vcls, vowner, comps, dispo, note)
            sk = os.path.join(vd, "skills")
            for s in ls(sk):
                sp = os.path.join(sk, s)
                if os.path.isfile(os.path.join(sp, "SKILL.md")):
                    sdg, sn = digest(sp)
                    row("codex", sk, f"{key}:{s}", f"installed plugin skill {state}", fm(os.path.join(sp, "SKILL.md")).get("name"), sdg, sn, cls, owner, "-", "native-keep" if known else "UNCLASSIFIED")

# --- Pi extensions
pe = f"{H}/.pi/agent/extensions"
for e in ls(pe):
    p = os.path.join(pe, e); ok = os.path.exists(p)
    form = (f"link -> {t(os.readlink(p))}" + ("" if ok else " (DANGLING)")) if os.path.islink(p) else "file"
    dg = (filedigest(p) if os.path.isfile(p) else digest(p)[0]) if ok else "-"
    cls = "dangling" if not ok else "tool-managed"
    owner = {"cbmem.ts": "codebase-memory-mcp", "herdr-agent-state.ts": "herdr", "technical-communication.ts": "plugins/install.sh"}.get(e, "Orca")
    dispo, note = "deactivate (dangling; no reader)" if not ok else "native-keep", ""
    # The documented Pi link names the public writing alias, which resolves
    # into the private payload of the linked Grove keg.
    alias = os.path.join(SKILL_DIRECTORY, "writing") + os.sep
    if os.path.islink(p) and os.readlink(p).startswith(alias):
        payload = os.path.join(os.path.realpath(GROVE_PREFIX), "libexec", "skill-sources", "writing") + os.sep
        if ok and os.path.realpath(p).startswith(payload): cls, owner = "native-residue", "Grove keg, through the writing alias"
        else: cls, owner, dispo, note = "unknown", "unknown", "UNCLASSIFIED", f"the writing alias does not resolve into {t(payload)}"
    row("pi", pe, e, form, "-", dg, 0, cls, owner, "Pi extension", dispo, note)

# --- user hooks: one row per hook, with the files under HOME that it names
PATHTOK = re.compile(r"""(?:\$\{HOME-?\}|\$HOME|~)(/[^\s"';|&)]+)""")
def hook_rows(harness, label, hooks, owner_of):
    ordinal = {}
    for ev, groups in hooks.items():
        for g in groups:
            for hk in g.get("hooks", []):
                c = hk.get("command", "")
                owner = owner_of(c)
                ordinal[(ev, owner)] = ordinal.get((ev, owner), 0) + 1
                h = hashlib.sha256(canon({"matcher": g.get("matcher"), "hook": hk}))
                owned = sorted({H + m for m in PATHTOK.findall(c) if os.path.isfile(H + m)})
                for s in owned: h.update(t(s).encode() + b"\0" + filedigest(s).encode())
                row(harness, label, f"{ev}:{owner}:{ordinal[(ev, owner)]}", f"user hook; owned files={len(owned)}", "-", h.hexdigest()[:16], len(owned), "tool-managed" if owner != "unknown" else "unknown", owner, "native hook", "native-keep" if owner != "unknown" else "UNCLASSIFIED")
hook_rows("claude", "~/.claude/settings.json hooks", claude_settings.get("hooks", {}),
          lambda c: "Orca" if "ORCA_AGENT_HOOK" in c else ("herdr" if "herdr" in c else ("codebase-memory-mcp" if "cbm-" in c else "unknown")))
hook_rows("codex", "~/.codex/hooks.json", need_json(f"{H}/.codex/hooks.json").get("hooks", {}),
          lambda c: "Orca" if "orca-agent-hook" in c else ("herdr" if "herdr" in c else ("codebase-memory-mcp" if "codebase-memory-mcp" in c else "unknown")))

# --- agent definitions and MCP servers
def agent_rows(harness, d, form):
    for a in ls(d):
        p = os.path.join(d, a)
        dg = filedigest(p) if os.path.isfile(p) else digest(p)[0]
        row(harness, d, a, form, "-", dg, 1, "tool-managed", "codebase-memory-mcp installer", "agent", "native-keep")
agent_rows("claude", f"{H}/.claude/agents", "subagent definition")
agent_rows("codex", f"{H}/.codex/agents", "agent definition")
SECRET_TABLES = {"env", "headers", "http_headers", "env_http_headers"}
def mcp_identity(d):
    clean = {}
    for k, v in d.items():
        if k in SECRET_TABLES and isinstance(v, dict): clean[k] = sorted(v)
        elif re.search(r"token|secret|password|key", k, re.I): clean[k] = "<present>"
        else: clean[k] = v
    return h16(canon(clean))
for n, d in sorted(need_json(f"{H}/.claude.json").get("mcpServers", {}).items()):
    row("claude", "~/.claude.json mcpServers", n, "MCP server", "-", mcp_identity(d), 0, "capability-only", "owner configuration", "MCP", "native-keep")
for n, d in sorted(codex_cfg.get("mcp_servers", {}).items()):
    row("codex", "~/.codex/config.toml mcp_servers", n, "MCP server", "-", mcp_identity(d), 0, "capability-only", "owner configuration", "MCP", "native-keep")
for n, d in codex_cfg.get("plugins", {}).items():
    en = str(d.get("enabled")).lower() if isinstance(d, dict) else "-"
    cls, owner = ("native-residue", HOOKS_OWNER) if n == CODEX_HOOKS else ("platform-managed", "Codex bundled marketplaces")
    row("codex", "~/.codex/config.toml plugins", n, f"plugin enabled={en}", "-", h16(canon(d)), 0, cls, owner, "plugin", "native-keep")

# --- retained storage and tools, observed now
snapdir = f"{H}/.agents/.grove/snapshots"
snaps = ls(snapdir)
cur = os.path.basename(os.path.realpath(H + "/.agents/.grove/current"))
row("codex", "~/.agents/.grove", "snapshots", f"{len(snaps)} snapshot directories; current -> {cur}", "-", h16(canon([snaps, cur])), len(snaps), "owner-pack", "Grove 22.8 provisioning", "lazy-read targets", "retain until readers and writer stop")
def tool_row(path, label, owner, caps):
    p = os.path.expanduser(path)
    if not os.path.exists(p):
        row("tool", os.path.dirname(path), label, "absent", "-", "-", 0, "tool-managed", owner, caps, "NEEDS-REVIEW", "executable not found"); return
    real = os.path.realpath(p)
    m = re.search(r"/Cellar/([^/]+)/([^/]+)/", real)
    ver = f"Homebrew {m.group(1)} {m.group(2)}" if m else "version not recorded; identity by digest"
    row("tool", os.path.dirname(path), label, f"{ver}; resolves to {t(real)}", "-", filedigest(real), 1, "tool-managed", owner, caps, "native-keep")
tool_row("/opt/homebrew/bin/testanyware", "testanyware", "Homebrew tap", "required by using-testanyware")
app = "/Applications/TestAnyware.app/Contents/Info.plist"
if os.path.isfile(app):
    with open(app, "rb") as fh: raw = fh.read()
    info = plistlib.loads(raw)
    row("tool", "/Applications", "TestAnyware.app", f"app {info.get('CFBundleShortVersionString')} build {info.get('CFBundleVersion')}", "-", h16(raw), 1, "tool-managed", "TestAnyware installer", "required by using-testanyware", "native-keep")
else:
    row("tool", "/Applications", "TestAnyware.app", "absent", "-", "-", 0, "tool-managed", "TestAnyware installer", "required by using-testanyware", "NEEDS-REVIEW", "bundle not found")
tool_row("/opt/homebrew/bin/quint", "quint", "Homebrew", "required by quint skills")
tool_row("~/.local/bin/codebase-memory-mcp", "codebase-memory-mcp", "its self-updater", "MCP, hooks and skill installer")
clone = f"{H}/.quint-llm-kit/repo"
try:
    git = ["git", "--no-optional-locks", "-C", clone]
    head = subprocess.run(git + ["rev-parse", "HEAD"], capture_output=True, text=True, check=True).stdout.strip()
    dirty = subprocess.run(git + ["status", "--porcelain"], capture_output=True, text=True, check=True).stdout
    changed = sorted(line[3:] for line in dirty.splitlines())
    row("tool", "~/.quint-llm-kit", "repo", f"git clone of quint-co/quint-llm-kit at {head}; changed paths: {', '.join(changed) or 'none'}", "-", h16(canon([head, dirty])), 0, "third-party-pack", "quint-llm-kit installer", "quint-kb MCP server and Codex skill links", "native-keep (MCP); skills via ravel:quint")
except (OSError, subprocess.CalledProcessError):
    row("tool", "~/.quint-llm-kit", "repo", "clone not readable", "-", "-", 0, "third-party-pack", "quint-llm-kit installer", "quint-kb MCP server and Codex skill links", "NEEDS-REVIEW")

# --- Claude Code built-in skills: no files on disk; observed in a session's skill listing
for s in "artifact-capabilities artifact-design artifact-diagramming claude-api claude-in-chrome code-review dataviz fewer-permission-prompts init keybindings-help loop plugin-authoring run schedule security-review simplify update-config workflow-authoring".split():
    row("claude", "Claude Code 2.1.316 built-in", s, "built-in skill (session listing)", s, "unverified", 0, "platform-managed", "Claude Code", "-", "native-keep")

# --- coverage: every SKILL.md under the four harness homes, by container
CONTAINERS = (
    (f"{H}/.claude/plugins/marketplaces/", "Claude Code marketplace clones (install sources)"),
    (f"{H}/.agents/.grove/snapshots/", "Grove 22.8 snapshot store (targets of Codex links)"),
    (f"{H}/.codex/.tmp/plugins/", "openai-curated marketplace clone (install source)"),
    (f"{H}/.codex/vendor_imports/", "Codex curated skill catalog (skill-installer source)"),
    (f"{H}/.codex/plugins/.remote-plugin-install-staging/", "Codex remote-install staging bundle"),
)
cover = {}
roots = [r.rstrip("/") + "/" for r in walked]
ccache = f"{H}/.claude/plugins/cache/"
for home in (".claude", ".codex", ".agents", ".pi"):
    for root, dirs, files in os.walk(os.path.join(H, home)):
        dirs[:] = sorted(d for d in dirs if d != "node_modules")
        if "SKILL.md" not in files: continue
        p = os.path.join(root, "SKILL.md")
        if any(p.startswith(r) for r in roots): continue
        if p.startswith(ccache):
            parts = p[len(ccache):].split("/")
            key = ("inactive", ccache + "/".join(parts[:2]))
        else:
            key = next((("container", c, why) for c, why in CONTAINERS if p.startswith(c)), ("uncovered", root))
        cover.setdefault(key, []).append(t(p))
for key, paths in sorted(cover.items()):
    paths.sort(); dg = h16("\n".join(paths).encode())
    if key[0] == "inactive":
        pd = key[1]
        vers = [v for v in ls(pd) if os.path.isdir(os.path.join(pd, v)) and os.path.realpath(os.path.join(pd, v)) not in active_claude]
        orph = sum(os.path.exists(os.path.join(pd, v, ".orphaned_at")) for v in vers)
        row("coverage", pd, "inactive cached versions", f"{len(vers)} versions outside installed_plugins.json; orphaned={orph}", "-", dg, len(paths), "not-a-discovery-root", "Claude Code plugin cache", "-", "outside (lazy-read targets until removed)")
    elif key[0] == "container":
        row("coverage", key[1], os.path.basename(key[1].rstrip("/")), key[2], "-", dg, len(paths), "not-a-discovery-root", "platform or installer", "-", "outside (not a discovery root)")
    else:
        row("coverage", key[1], os.path.basename(key[1]), "SKILL.md outside every walked root and container", fm(os.path.join(key[1], "SKILL.md")).get("name"), dg, len(paths), "unknown", "unknown", "-", "UNCOVERED")

w = sys.stdout
w.write("\t".join(HEADER) + "\n")
for r in rows: w.write("\t".join(x.replace("\t", " ") for x in r) + "\n")
