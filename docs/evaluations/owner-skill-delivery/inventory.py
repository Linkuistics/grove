"""Print the owner's skill-installation baseline as TSV on standard output.

The script only reads. It walks the Claude Code, Codex and Pi discovery
roots, both plugin registries, the user hook and MCP files and the known tool
installations. It writes paths with ~ for HOME and prints no file content
except SKILL.md frontmatter names.

The design in docs/specs/owner-skill-delivery.md owns each classification and
destination. This script encodes them so that a later inventory can compare
entries one by one with docs/owner-skill-inventory.tsv.

Usage:
    python3 -I inventory.py > inventory.tsv

Control: set CONTROL_ROOT to a scratch directory that holds an unknown skill
directory and a dangling link. The output must then show one UNCLASSIFIED
row and one dangling row under the harness "control". Without CONTROL_ROOT,
no row may say UNCLASSIFIED or UNASSIGNED.
"""
import hashlib, json, os, re, sys
H = os.path.expanduser("~")
def t(p): return p.replace(H, "~", 1) if p.startswith(H) else p
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
def digest(d):
    h = hashlib.sha256(); n = 0
    for root, dirs, files in os.walk(d):
        dirs.sort()
        for f in sorted(files):
            p = os.path.join(root, f); rel = os.path.relpath(p, d)
            if os.path.islink(p): h.update(b"L" + rel.encode() + os.readlink(p).encode())
            else:
                h.update(b"F" + rel.encode())
                with open(p, "rb") as fh: h.update(hashlib.sha256(fh.read()).digest())
            n += 1
    return h.hexdigest()[:16], n

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
    "using-testanyware": "testanyware CLI 2.7.0, TestAnyware.app, ~/.config/testanyware/config.rhai, VM data/state dirs",
    "quint-lang": "quint CLI 0.33.1; quint-kb MCP from ~/.quint-llm-kit/repo",
    "quint-modeling": "quint CLI 0.33.1; quint-kb MCP from ~/.quint-llm-kit/repo",
    "quint-execute-spec": "quint CLI 0.33.1; quint-kb MCP from ~/.quint-llm-kit/repo",
    "using-codebase-memory": "codebase-memory-mcp 0.10.6 MCP and CLI",
    "technical-communication": "python3; scripts/ste-lint.py; always-on hooks (Claude only)",
    "guardrail": "Claude Code frontmatter PreToolUse hook and /guardrail command",
    "machine-environment": "mise, task, Obsidian manual",
    "using-jujutsu": "jj",
    "using-superpowers": "SessionStart hook injection (Claude plugin)",
}
rows = []
def row(harness, location, entry, form, ident, dg, files, cls, owner, caps, dispo, note=""):
    rows.append([harness, t(location), entry, form, ident or "-", dg, str(files), cls, owner, caps or "-", dispo, note or "-"])

# --- Claude Code plugins
inst = json.load(open(f"{H}/.claude/plugins/installed_plugins.json"))
enabled = json.load(open(f"{H}/.claude/settings.json")).get("enabledPlugins", {})
for key, entries in sorted(inst["plugins"].items()):
    for e in entries:
        p = e["installPath"]; plugin, market = key.split("@")
        en = enabled.get(key)
        comps = []
        for k in ("skills", "commands", "agents"):
            if os.path.isdir(os.path.join(p, k)): comps.append(k)
        if os.path.isfile(os.path.join(p, "hooks", "hooks.json")): comps.append("hooks")
        if os.path.isfile(os.path.join(p, ".mcp.json")): comps.append("mcp")
        lsp = ""
        if market == "claude-plugins-official" and plugin.endswith("-lsp"): comps.append("lsp(marketplace entry)")
        if market == "linkuistics": cls, owner = "owner-pack", "Linkuistics/grove marketplace"
        elif plugin in ("superpowers", "quint-llm-kit"): cls, owner = "third-party-pack", market
        elif "skills" in comps: cls, owner = ("third-party-pack" if en else "disabled-third-party"), market
        else: cls, owner = "capability-only", market
        if not en: cls = "disabled-third-party" if market != "linkuistics" else cls
        if market == "linkuistics" and plugin == "linkuistics": dispo = "disable; hooks move to skill-free plugin; guardrail to native-only plugin"
        elif market == "linkuistics": dispo = "disable"
        elif plugin == "superpowers": dispo = "disable; SessionStart prose becomes Ravel mandatory file"
        elif plugin == "quint-llm-kit": dispo = "disable"
        elif not en: dispo = "outside (unchanged)"
        else: dispo = "native-keep"
        row("claude", os.path.dirname(os.path.dirname(p)), key, f"plugin v{e['version']} sha {(e.get('gitCommitSha') or '-')[:12]} enabled={en}", plugin, "-", 0, cls, owner, "+".join(comps), dispo)
        sk = os.path.join(p, "skills")
        if os.path.isdir(sk):
            for d in sorted(os.listdir(sk)):
                sd = os.path.join(sk, d)
                if not os.path.isfile(os.path.join(sd, "SKILL.md")): continue
                f = fm(os.path.join(sd, "SKILL.md")); dg, n = digest(sd)
                if market == "linkuistics":
                    tgt = DEST.get(d, "UNASSIGNED")
                    if d == "guardrail": dsp = "native-only (PersonalSkills Claude plugin; outside Ravel roots)"
                    elif d == "technical-communication": dsp = f"ravel:{tgt}; hooks via skill-free plugin"
                    else: dsp = f"ravel:{tgt}"
                elif plugin == "superpowers": dsp = "ravel:superpowers" + ("; mandatory file" if d == "using-superpowers" else "")
                elif plugin == "quint-llm-kit": dsp = "ravel:quint"
                elif not en: dsp = "outside (plugin disabled)"
                else: dsp = "UNCLASSIFIED"
                note = "; ".join(x for x in [("harnesses=" + f["harnesses"]) if f.get("harnesses") not in (None, "[any]") else "", "user-invoked only" if f.get("disable-model-invocation") == "true" else ""] if x)
                row("claude", sd.rsplit("/skills/", 1)[0] + "/skills", f"{plugin}:{d}", "plugin skill", f.get("name"), dg, n, cls if en else "disabled-third-party", owner, CAPS.get(d), dsp, note)

# --- filesystem discovery roots
def walk_root(harness, root, cls_fn):
    rp = os.path.expanduser(root)
    if not os.path.isdir(rp): return
    for e in sorted(os.listdir(rp)):
        p = os.path.join(rp, e)
        if e.startswith(".") and e != ".system": continue
        if os.path.islink(p):
            tgt = os.readlink(p); real = os.path.realpath(p); ok = os.path.exists(p)
            form = f"link -> {t(tgt)}" + ("" if ok else " (DANGLING)")
        elif os.path.isdir(p): real, ok, form = p, True, "directory"
        else: continue
        if e == ".system" or e == "synced":
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

# --- Codex bundled/runtime plugin skills (platform-managed)
for base, label in ((f"{H}/.codex/.tmp/bundled-marketplaces/openai-bundled", "openai-bundled"), (f"{H}/.cache/codex-runtimes/codex-primary-runtime/plugins/openai-primary-runtime", "openai-primary-runtime")):
    pl = os.path.join(base, "plugins")
    if not os.path.isdir(pl): continue
    for plug in sorted(os.listdir(pl)):
        sk = os.path.join(pl, plug, "skills")
        if not os.path.isdir(sk): continue
        for s in sorted(os.listdir(sk)):
            sp = os.path.join(sk, s)
            if os.path.isfile(os.path.join(sp, "SKILL.md")):
                dg, n = digest(sp)
                row("codex", base, f"{plug}@{label}:{s}", "bundled plugin skill", fm(os.path.join(sp, "SKILL.md")).get("name"), dg, n, "platform-managed", "Codex " + label, "-", "native-keep")

# --- Pi extensions
pe = f"{H}/.pi/agent/extensions"
for e in sorted(os.listdir(pe)):
    p = os.path.join(pe, e); ok = os.path.exists(p)
    form = (f"link -> {t(os.readlink(p))}" + ("" if ok else " (DANGLING)")) if os.path.islink(p) else "file"
    cls = "dangling" if not ok else "tool-managed"
    owner = {"cbmem.ts": "codebase-memory-mcp", "herdr-agent-state.ts": "herdr", "technical-communication.ts": "plugins/install.sh"}.get(e, "Orca")
    row("pi", pe, e, form, "-", "-", 0, cls, owner, "Pi extension", "deactivate (dangling; no reader)" if not ok else "native-keep")

# --- hooks, agents, MCP (names only)
st = json.load(open(f"{H}/.claude/settings.json"))
seen = set()
for ev, groups in st.get("hooks", {}).items():
    for g in groups:
        for hk in g.get("hooks", []):
            c = hk.get("command", "")
            owner = "Orca" if "ORCA_AGENT_HOOK" in c else ("herdr" if "herdr" in c else ("codebase-memory-mcp" if "cbm-" in c else "unknown"))
            k = (ev, owner)
            if k in seen: continue
            seen.add(k)
            row("claude", "~/.claude/settings.json hooks", f"{ev}:{owner}", "user hook", "-", "-", 0, "tool-managed", owner, "native hook", "native-keep")
for a in sorted(os.listdir(f"{H}/.claude/agents")):
    row("claude", "~/.claude/agents", a, "subagent definition", "-", "-", 0, "tool-managed", "codebase-memory-mcp installer", "agent", "native-keep")
for n in sorted(json.load(open(f"{H}/.claude.json")).get("mcpServers", {})):
    row("claude", "~/.claude.json mcpServers", n, "MCP server", "-", "-", 0, "capability-only", "owner configuration", "MCP", "native-keep")
cj = json.load(open(f"{H}/.codex/hooks.json")).get("hooks", {})
seen = set()
for ev, groups in cj.items():
    for g in groups:
        for hk in g.get("hooks", []):
            c = hk.get("command", "")
            owner = "Orca" if "orca-agent-hook" in c else ("herdr" if "herdr" in c else ("codebase-memory-mcp" if "codebase-memory-mcp" in c else "unknown"))
            if (ev, owner) in seen: continue
            seen.add((ev, owner))
            row("codex", "~/.codex/hooks.json", f"{ev}:{owner}", "user hook", "-", "-", 0, "tool-managed", owner, "native hook", "native-keep")
for a in sorted(os.listdir(f"{H}/.codex/agents")):
    row("codex", "~/.codex/agents", a, "agent definition", "-", "-", 0, "tool-managed", "codebase-memory-mcp installer", "agent", "native-keep")
txt = open(f"{H}/.codex/config.toml").read()
for n in sorted(set(re.findall(r'^\[mcp_servers\.([A-Za-z0-9_-]+)\]', txt, re.M))):
    row("codex", "~/.codex/config.toml mcp_servers", n, "MCP server", "-", "-", 0, "capability-only", "owner configuration", "MCP", "native-keep")
for n, en in re.findall(r'^\[plugins\."([^"]+)"\]\nenabled = (true|false)', txt, re.M):
    row("codex", "~/.codex/config.toml plugins", n, f"plugin enabled={en}", "-", "-", 0, "platform-managed", "Codex bundled marketplaces", "plugin", "native-keep")

# --- retained storage and tools
snaps = sorted(os.listdir(f"{H}/.agents/.grove/snapshots"))
row("codex", "~/.agents/.grove", "snapshots", f"{len(snaps)} snapshot directories; current -> {os.path.basename(os.path.realpath(H + '/.agents/.grove/current'))}", "-", "-", len(snaps), "owner-pack", "Grove 22.8 provisioning", "lazy-read targets", "retain until readers and writer stop")
row("tool", "/opt/homebrew/bin", "testanyware", "Homebrew linkuistics/taps/testanyware 2.7.0", "-", "-", 0, "tool-managed", "Homebrew tap", "required by using-testanyware", "native-keep")
row("tool", "/Applications", "TestAnyware.app", "app 2.7.0", "-", "-", 0, "tool-managed", "TestAnyware installer", "required by using-testanyware", "native-keep")
row("tool", "/opt/homebrew/bin", "quint", "quint 0.33.1", "-", "-", 0, "tool-managed", "Homebrew", "required by quint skills", "native-keep")
row("tool", "~/.local/bin", "codebase-memory-mcp", "codebase-memory-mcp 0.10.6", "-", "-", 0, "tool-managed", "its self-updater", "MCP, hooks and skill installer", "native-keep")
row("tool", "~/.quint-llm-kit", "repo", "git clone of quint-co/quint-llm-kit at cc75369f741af7d490936f82002c2d28e3b3d78d (dirty package-lock)", "-", "-", 0, "third-party-pack", "quint-llm-kit installer", "quint-kb MCP server and Codex skill links", "native-keep (MCP); skills via ravel:quint")


# --- Claude Code built-in skills: no files on disk; observed in this session's skill listing
for s in "artifact-capabilities artifact-design artifact-diagramming claude-api claude-in-chrome code-review dataviz fewer-permission-prompts init keybindings-help loop plugin-authoring run schedule security-review simplify update-config workflow-authoring".split():
    row("claude", "Claude Code 2.1.316 built-in", s, "built-in skill (session listing)", s, "-", 0, "platform-managed", "Claude Code", "-", "native-keep")

hdr = ["harness", "location", "entry", "form", "identity", "digest16", "files", "class", "source_owner", "capabilities", "disposition", "note"]
w = sys.stdout
w.write("\t".join(hdr) + "\n")
for r in rows: w.write("\t".join(x.replace("\t", " ") for x in r) + "\n")
