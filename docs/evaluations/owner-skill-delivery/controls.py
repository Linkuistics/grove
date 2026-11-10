"""Check that the inventory instrument sees each class of change.

The script builds a fixture HOME in a temporary directory and runs
inventory.py against it. It then builds a fresh fixture for each case, applies
one change and compares the output entry by entry with the unchanged run. Each
case must change, add or mark the entry that it names. Five null cases
must leave the output unchanged: a repeated run, a rotated MCP credential, and
an added, changed or erased .in_use marker of a running Claude Code session.
The fixture's plugin version directory holds one marker of an earlier session,
as the owner's baseline did. Two cases change that directory in other places,
including a nested .in_use, and must change the plugin row.

The cases of "Test seams" in docs/specs/owner-skill-delivery.md follow. A
superseded skill beside each platform subtree, at both native roots, through
a link, as a nested copy and as a hidden child, must fail the instrument's
`compare`. So must a link in place of synced, and a child named synced or
.system in the other native root. A change inside synced or .system alone
must pass it, and must still change that subtree's rows. The native guardrail
from its sole-skill personal plugin must pass. An extra skill in the
directory that Claude Code loads or in the plugin-cache copy must fail. So
must a skills path that a plugin manifest or the marketplace entry declares,
a loaded directory outside the keg, a marketplace with no guardrail entry,
another origin and no registration. Each case also checks the verdict of
`compare` against the unchanged run. The unchanged fixture must hold no
anomaly row.

The fixture stands in for the owner's HOME, so the script writes only inside
its temporary directory. The instrument still reads the absolute tool paths
under /opt/homebrew and /Applications, which are the same for every case.

The instrument accepts the guardrail plugin only from the fixed origin
/opt/homebrew/opt/personal, which a fixture cannot write. Each case therefore
builds a fixture keg beside its HOME and runs a copy of the instrument in which
that literal names the fixture keg. The instrument has no input that changes
its origin, so an owner's run cannot widen the exception.

Usage:
    python3 -I controls.py [--instrument PATH]

Exit status 0 means that every control passed. --instrument runs the same
cases against another revision of the instrument, such as the reviewed one.
A revision that cannot see a class of change must fail its case.
"""
import json, os, shutil, subprocess, sys, tempfile
ANOMALIES = ("UNCLASSIFIED", "UNASSIGNED", "UNCOVERED", "NEEDS-REVIEW")

HERE = os.path.dirname(os.path.abspath(__file__))
instrument = os.path.join(HERE, "inventory.py")
if sys.argv[1:2] == ["--instrument"]: instrument = os.path.abspath(sys.argv[2])

def write(home, rel, text, mode=0o644):
    p = os.path.join(home, rel)
    os.makedirs(os.path.dirname(p), exist_ok=True)
    with open(p, "w", encoding="utf-8") as fh: fh.write(text)
    os.chmod(p, mode)
def jwrite(home, rel, obj): write(home, rel, json.dumps(obj, indent=1))
def edit_json(home, rel, fn):
    p = os.path.join(home, rel)
    with open(p, encoding="utf-8") as fh: obj = json.load(fh)
    fn(obj); jwrite(home, rel, obj)

PLUGIN = ".claude/plugins/cache/linkuistics/demo/1"
SKILL = PLUGIN + "/skills/grove"
PLUGIN_ROW = ("claude", "~/.claude/plugins/cache/linkuistics", "demo@linkuistics")
MARKER = PLUGIN + "/.in_use/1001"
MARKER_V1 = '{"pid":1001,"procStart":"Sat Nov  7 09:00:00 2026"}'
HOOK = {"matcher": "startup", "hooks": [{"type": "command", "command": "\"$HOME/.claude/hooks/herdr-agent-state.sh\" start", "timeout": 5}]}
SYNCED = ".claude/skills/synced/account-1/docs"
SYSTEM = ".codex/skills/.system/skill-creator"
GUARDRAIL = ".claude/plugins/cache/personal/guardrail/unknown"
GUARDRAIL_ROW = ("claude", "~/.claude/plugins/cache/personal", "guardrail@personal")
ORIGIN = "/opt/homebrew/opt/personal"
KEG_PLUGIN = "libexec/plugins/guardrail"
GUARDRAIL_SKILL = "---\nname: guardrail\n---\nGuardrail.\n"
SUPERSEDED = "---\nname: brainstorming\n---\nSuperseded copy.\n"
CODEX_TOML = '[mcp_servers.quint-kb]\ncommand = "node"\nargs = ["kb.js"]\n\n[plugins."visualize@openai-bundled"]\nenabled = false\n'

def keg_of(home):
    """The fixture keg that stands in for /opt/homebrew/opt/personal, beside HOME."""
    return os.path.join(os.path.dirname(home), "keg")

def build(home):
    jwrite(home, ".claude/plugins/installed_plugins.json", {"version": 2, "plugins": {"demo@linkuistics": [
        {"scope": "user", "installPath": os.path.join(home, PLUGIN), "version": "1", "gitCommitSha": "0" * 40}]}})
    write(home, PLUGIN + "/.claude-plugin/plugin.json", '{"name": "demo"}\n')
    write(home, MARKER, MARKER_V1)
    write(home, SKILL + "/SKILL.md", "---\nname: grove\n---\nFixture skill.\n")
    write(home, SKILL + "/scripts/run.sh", "#!/bin/sh\nexit 0\n", 0o755)
    write(home, SKILL + "/assets/note.txt", "asset\n")
    os.symlink("scripts", os.path.join(home, SKILL, "shared"))
    jwrite(home, ".claude/settings.json", {"enabledPlugins": {"demo@linkuistics": True}, "skillOverrides": {"codebase-memory": "off"}, "hooks": {"SessionStart": [HOOK]}})
    write(home, ".claude/hooks/herdr-agent-state.sh", "#!/bin/sh\nexit 0\n", 0o755)
    write(home, ".claude/agents/demo.md", "agent v1\n")
    os.makedirs(os.path.join(home, ".claude/skills"))
    jwrite(home, ".claude.json", {"mcpServers": {"quint-kb": {"type": "stdio", "command": "node", "args": ["kb.js"], "env": {"KB_TOKEN": "credential-1"}}}})
    write(home, ".codex/config.toml", CODEX_TOML)
    jwrite(home, ".codex/hooks.json", {"hooks": {}})
    write(home, ".codex/agents/demo.toml", "agent v1\n")
    for d in (".codex/skills", ".agents/skills", ".pi/agent/skills", ".pi/agent/extensions", ".agents/.grove/snapshots/s1"):
        os.makedirs(os.path.join(home, d))
    os.symlink("snapshots/s1", os.path.join(home, ".agents/.grove/current"))
    # The two platform subtrees, an ordinary child beside one, and the native
    # guardrail plugin, enabled from the personal keg's marketplace.
    write(home, SYNCED + "/SKILL.md", "---\nname: docs\n---\nAccount skill.\n")
    write(home, SYSTEM + "/SKILL.md", "---\nname: skill-creator\n---\nSystem skill.\n")
    write(home, ".claude/skills/codebase-memory/SKILL.md", "---\nname: codebase-memory\n---\nTool skill.\n")
    write(home, GUARDRAIL + "/skills/guardrail/SKILL.md", GUARDRAIL_SKILL)
    write(home, GUARDRAIL + "/commands/guardrail.md", "Guardrail command.\n")
    edit_json(home, ".claude/plugins/installed_plugins.json", lambda o: o["plugins"].update({"guardrail@personal": [
        {"scope": "user", "installPath": os.path.join(home, GUARDRAIL), "version": "unknown"}]}))
    edit_json(home, ".claude/settings.json", lambda o: o["enabledPlugins"].update({"guardrail@personal": True}))
    keg = keg_of(home)
    jwrite(home, ".claude/plugins/known_marketplaces.json", {"personal": {"source": {"source": "directory", "path": keg}, "installLocation": keg}})
    # The directory that Claude Code loads, as the personal keg lays it out.
    jwrite(keg, ".claude-plugin/marketplace.json", {"name": "personal", "plugins": [{"name": "guardrail", "source": "./" + KEG_PLUGIN}]})
    write(keg, KEG_PLUGIN + "/.claude-plugin/plugin.json", '{"name": "guardrail"}\n')
    write(keg, KEG_PLUGIN + "/skills/guardrail/SKILL.md", GUARDRAIL_SKILL)
    write(home, ".quint-llm-kit/repo/skills/quint-lang/SKILL.md", "---\nname: quint-lang\n---\nQuint.\n")
    # The two tools under HOME, so that the unchanged run has no anomaly row
    # and the comparison's verdict depends only on the case.
    write(home, ".local/bin/codebase-memory-mcp", "#!/bin/sh\nexit 0\n", 0o755)
    git = ["git", "-C", os.path.join(home, ".quint-llm-kit/repo"), "-c", "user.name=fixture",
           "-c", "user.email=fixture@example.invalid", "-c", "commit.gpgsign=false"]
    for args in (["init", "-q"], ["add", "-A"], ["commit", "-q", "-m", "fixture"]):
        subprocess.run(git + args, check=True, capture_output=True, env={**os.environ, "HOME": home, "GIT_AUTHOR_DATE": "2026-11-08T00:00:00Z", "GIT_COMMITTER_DATE": "2026-11-08T00:00:00Z"})
    pages = ".codex/plugins/cache/openai-curated-remote/pages"
    write(home, pages + "/0.1.0/skills/write-page/SKILL.md", "---\nname: write-page\n---\nFixture page skill.\n")
    jwrite(home, pages + "/.codex-remote-plugin-install.json", {"schema_version": 1, "remote_plugin_id": "fixture"})

def second_hook(s): s["hooks"]["SessionStart"].append(HOOK)
def relink(home):
    p = os.path.join(home, SKILL, "shared"); os.remove(p); os.symlink("assets", p)
CASES = [
    ("repeated run", lambda h: None, ("same",)),
    ("rotated MCP credential", lambda h: edit_json(h, ".claude.json", lambda o: o["mcpServers"]["quint-kb"]["env"].update(KB_TOKEN="credential-2")), ("same",)),
    ("added in-use marker", lambda h: write(h, PLUGIN + "/.in_use/2002", '{"pid":2002,"procStart":"Sat Nov  7 10:00:00 2026"}'), ("same",)),
    ("changed in-use marker", lambda h: write(h, MARKER, MARKER_V1.replace("09:00:00", "09:30:00")), ("same",)),
    ("erased in-use marker", lambda h: os.remove(os.path.join(h, MARKER)), ("same",)),
    ("changed plugin manifest", lambda h: write(h, PLUGIN + "/.claude-plugin/plugin.json", '{"name": "demo", "version": "2"}\n'),
     ("changed", PLUGIN_ROW)),
    ("nested in-use entry in a plugin version directory", lambda h: write(h, PLUGIN + "/.claude-plugin/.in_use/2002", "{}"),
     ("changed", PLUGIN_ROW)),
    ("changed hook command", lambda h: edit_json(h, ".claude/settings.json", lambda o: o["hooks"]["SessionStart"][0]["hooks"][0].update(command="\"$HOME/.claude/hooks/herdr-agent-state.sh\" begin")),
     ("changed", ("claude", "~/.claude/settings.json hooks", "SessionStart:herdr:1"))),
    ("second hook with the same event and owner", lambda h: edit_json(h, ".claude/settings.json", second_hook),
     ("added", ("claude", "~/.claude/settings.json hooks", "SessionStart:herdr:2"))),
    ("changed hook script", lambda h: write(h, ".claude/hooks/herdr-agent-state.sh", "#!/bin/sh\nexit 1\n", 0o755),
     ("changed", ("claude", "~/.claude/settings.json hooks", "SessionStart:herdr:1"))),
    ("changed MCP command with the same name", lambda h: edit_json(h, ".claude.json", lambda o: o["mcpServers"]["quint-kb"].update(command="deno")),
     ("changed", ("claude", "~/.claude.json mcpServers", "quint-kb"))),
    ("changed Codex MCP arguments", lambda h: write(h, ".codex/config.toml", CODEX_TOML.replace('["kb.js"]', '["kb.js", "--verbose"]')),
     ("changed", ("codex", "~/.codex/config.toml mcp_servers", "quint-kb"))),
    ("lost executable bit", lambda h: os.chmod(os.path.join(h, SKILL, "scripts/run.sh"), 0o644),
     ("changed", ("claude", "~/" + PLUGIN + "/skills", "demo:grove"))),
    ("retargeted directory link", relink,
     ("changed", ("claude", "~/" + PLUGIN + "/skills", "demo:grove"))),
    ("changed agent definition", lambda h: write(h, ".claude/agents/demo.md", "agent v2\n"),
     ("changed", ("claude", "~/.claude/agents", "demo.md"))),
    ("changed skill override", lambda h: edit_json(h, ".claude/settings.json", lambda o: o["skillOverrides"].update({"codebase-memory": "on"})),
     ("changed", ("claude", "~/.claude/settings.json skillOverrides", "codebase-memory"))),
    ("changed remote-installed Codex plugin skill", lambda h: write(h, ".codex/plugins/cache/openai-curated-remote/pages/0.1.0/skills/write-page/SKILL.md", "---\nname: write-page\n---\nChanged.\n"),
     ("changed", ("codex", "~/.codex/plugins/cache/openai-curated-remote/pages/0.1.0/skills", "pages@openai-curated-remote:write-page"))),
    ("skill in an unknown Codex marketplace", lambda h: write(h, ".codex/plugins/cache/acme/tool/1/skills/x/SKILL.md", "---\nname: x\n---\n"),
     ("marker", "UNCLASSIFIED")),
    ("skill outside every walked root", lambda h: write(h, ".codex/new-root/x/SKILL.md", "---\nname: x\n---\n"),
     ("marker", "UNCOVERED")),
]

def link(home, target, rel):
    os.symlink(os.path.join(home, target), os.path.join(home, rel))
def synced_link(home):
    shutil.rmtree(os.path.join(home, ".claude/skills/synced"))
    write(home, ".elsewhere/account-1/brainstorming/SKILL.md", SUPERSEDED)
    link(home, ".elsewhere", ".claude/skills/synced")
def other_origin(home):
    edit_json(home, ".claude/plugins/known_marketplaces.json",
              lambda o: o["personal"].update(source={"source": "directory", "path": os.path.join(home, "PersonalSkills")}))
def marketplace_entry(home, **fields):
    edit_json(keg_of(home), ".claude-plugin/marketplace.json", lambda o: o["plugins"][0].update(fields))
def manifest_skills(home):
    write(keg_of(home), KEG_PLUGIN + "/.claude-plugin/plugin.json", '{"name": "guardrail", "skills": ["./extra"]}\n')
    write(keg_of(home), KEG_PLUGIN + "/extra/brainstorming/SKILL.md", SUPERSEDED)
def loaded_outside(home):
    write(home, "../elsewhere/guardrail/skills/guardrail/SKILL.md", GUARDRAIL_SKILL)
    marketplace_entry(home, source="../elsewhere/guardrail")
def guardrail_elsewhere(home):
    edit_json(home, ".claude/plugins/installed_plugins.json", lambda o: o["plugins"].update({"guardrail@acme": [
        {"scope": "user", "installPath": os.path.join(home, ".claude/plugins/cache/acme/guardrail/1"), "version": "1"}]}))
    write(home, ".claude/plugins/cache/acme/guardrail/1/skills/guardrail/SKILL.md", "---\nname: guardrail\n---\nCopy.\n")
    edit_json(home, ".claude/settings.json", lambda o: o["enabledPlugins"].update({"guardrail@acme": True}))
# "Test seams" of docs/specs/owner-skill-delivery.md: each case beside or
# inside a platform subtree, and each case of the native guardrail. "fails"
# and "passes" name the exit of `inventory.py compare` against the unchanged
# run. A pass inside a subtree must also change that subtree's rows, so that
# the exclusion, not a blind walker, passes it.
CASES += [
    ("superseded skill beside synced", lambda h: write(h, ".claude/skills/brainstorming/SKILL.md", SUPERSEDED), ("fails",)),
    ("superseded skill beside .system", lambda h: write(h, ".codex/skills/brainstorming/SKILL.md", SUPERSEDED), ("fails",)),
    ("superseded skill link beside synced", lambda h: link(h, ".quint-llm-kit/repo/skills/quint-lang", ".claude/skills/quint-lang"), ("fails",)),
    ("superseded skill link beside .system", lambda h: link(h, ".quint-llm-kit/repo/skills/quint-lang", ".codex/skills/quint-lang"), ("fails",)),
    ("nested copy in an ordinary child beside synced", lambda h: write(h, ".claude/skills/codebase-memory/brainstorming/SKILL.md", SUPERSEDED), ("fails",)),
    ("hidden child beside .system", lambda h: write(h, ".codex/skills/.brainstorming/SKILL.md", SUPERSEDED), ("fails",)),
    ("synced in the Codex root is an ordinary child", lambda h: write(h, ".codex/skills/synced/account-1/brainstorming/SKILL.md", SUPERSEDED), ("fails",)),
    (".system in the Claude Code root is an ordinary child", lambda h: write(h, ".claude/skills/.system/brainstorming/SKILL.md", SUPERSEDED), ("fails",)),
    ("link in place of synced", synced_link, ("fails",)),
    ("new child of the Codex user root .agents/skills", lambda h: write(h, ".agents/skills/brainstorming/SKILL.md", SUPERSEDED), ("fails",)),
    ("new Pi skill", lambda h: write(h, ".pi/agent/skills/brainstorming/SKILL.md", SUPERSEDED), ("fails",)),
    ("changed skill inside synced", lambda h: write(h, SYNCED + "/SKILL.md", "---\nname: docs\n---\nUpdated by the account.\n"), ("passes",)),
    ("new skill inside .system", lambda h: write(h, ".codex/skills/.system/imagegen/SKILL.md", "---\nname: imagegen\n---\n"), ("passes",)),
    ("native guardrail from its sole-skill personal plugin", lambda h: None, ("residue", "native-keep (native residue)")),
    ("extra skill in the loaded guardrail directory", lambda h: write(keg_of(h), KEG_PLUGIN + "/skills/brainstorming/SKILL.md", SUPERSEDED), ("residue", "UNCLASSIFIED")),
    ("extra skill in the guardrail plugin-cache copy", lambda h: write(h, GUARDRAIL + "/skills/brainstorming/SKILL.md", SUPERSEDED), ("residue", "UNCLASSIFIED")),
    ("skills path in the loaded guardrail manifest", manifest_skills, ("residue", "UNCLASSIFIED")),
    ("skills in the personal marketplace entry", lambda h: marketplace_entry(h, skills=["./skills/guardrail"]), ("residue", "UNCLASSIFIED")),
    ("loaded guardrail directory outside the keg", loaded_outside, ("residue", "UNCLASSIFIED")),
    ("personal marketplace with no guardrail entry", lambda h: os.remove(os.path.join(keg_of(h), ".claude-plugin/marketplace.json")), ("residue", "UNCLASSIFIED")),
    ("guardrail plugin from another origin", other_origin, ("residue", "UNCLASSIFIED")),
    ("guardrail plugin with no personal registration", lambda h: os.remove(os.path.join(h, ".claude/plugins/known_marketplaces.json")), ("residue", "UNCLASSIFIED")),
    ("guardrail skill from another marketplace", guardrail_elsewhere, ("marker", "UNCLASSIFIED")),
]

with open(instrument, encoding="utf-8") as fh: INSTRUMENT_TEXT = fh.read()
if ORIGIN not in INSTRUMENT_TEXT: sys.exit(f"controls: {instrument} never names {ORIGIN}")

def run(home):
    """The instrument's rows for HOME, from a copy whose origin is the fixture keg."""
    copy = os.path.join(home, "..", "inventory.py")
    with open(copy, "w", encoding="utf-8") as fh: fh.write(INSTRUMENT_TEXT.replace(ORIGIN, keg_of(home)))
    env = {k: v for k, v in os.environ.items() if k != "CONTROL_ROOT"}; env["HOME"] = home
    out = subprocess.run([sys.executable, "-I", copy], env=env, capture_output=True, text=True)
    if out.returncode: return None
    with open(os.path.join(home, "..", "inventory.tsv"), "w", encoding="utf-8") as fh: fh.write(out.stdout)
    return {(r[0], r[1], r[2]): r for r in (line.split("\t") for line in out.stdout.splitlines()[1:])}

def compared(tmp, name):
    """The exit status of the instrument's own comparison of the unchanged run with this case."""
    tsv = lambda n: os.path.join(tmp, n.replace(" ", "-"), "inventory.tsv")
    return subprocess.run([sys.executable, "-I", instrument, "compare", tsv("base"), tsv(name)],
                          capture_output=True, text=True).returncode

def fixture_run(tmp, name, mutate):
    home = os.path.join(tmp, name.replace(" ", "-"), "home")
    os.makedirs(home); build(home); mutate(home)
    return run(home)

# Resolved, so that a resolved path under the fixture HOME still starts with HOME.
tmp = os.path.realpath(tempfile.mkdtemp(prefix="owner-inventory-controls."))
failed = 0
try:
    base = fixture_run(tmp, "base", lambda h: None)
    if base is None: sys.exit(f"controls: {instrument} failed on the unchanged fixture")
    anomalies = [k for k, r in base.items() if any(f in ANOMALIES for f in r)]
    if anomalies: sys.exit(f"controls: the unchanged fixture has anomaly rows: {anomalies}")
    for name, mutate, expect in CASES:
        got = fixture_run(tmp, name, mutate)
        if got is None: ok, why = False, "instrument failed"
        elif expect[0] == "same": ok, why = got == base, "output differs from the unchanged run"
        elif expect[0] == "changed":
            k = expect[1]; ok = k in base and k in got and base[k] != got[k]; why = f"entry {k[2]} did not change"
        elif expect[0] == "added":
            k = expect[1]; ok = k not in base and k in got; why = f"entry {k[2]} was not added"
        elif expect[0] == "marker":
            ok = any(r[10] == expect[1] for k, r in got.items() if k not in base); why = f"no new {expect[1]} row"
        elif expect[0] == "residue":
            r = got.get(GUARDRAIL_ROW); ok = r is not None and r[10] == expect[1]
            why = f"guardrail@personal has disposition {r[10] if r else 'none'}, not {expect[1]}"
        elif expect[0] == "passes":
            ok = got != base; why = "the subtree's rows did not change"
        else:
            ok = True
        # Every case also states the comparison's verdict: unchanged and
        # accepted cases pass it, and each other case fails it.
        if got is not None and ok:
            want = 0 if expect[0] in ("same", "passes") or expect[1:] == ("native-keep (native residue)",) else 1
            code = compared(tmp, name)
            if code != want: ok, why = False, f"compare exited {code}, not {want}"
        if not ok and got is not None and expect[0] != "same" and got == base: why += "; whole output unchanged"
        print(("PASS " if ok else "FAIL ") + name + ("" if ok else f": {why}"))
        failed += not ok
finally:
    shutil.rmtree(tmp)
print(f"{len(CASES) - failed} of {len(CASES)} controls passed with {instrument}")
sys.exit(1 if failed else 0)
