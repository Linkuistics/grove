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

The fixture stands in for the owner's HOME, so the script writes only inside
its temporary directory. The instrument still reads the absolute tool paths
under /opt/homebrew and /Applications, which are the same for every case.

Usage:
    python3 -I controls.py [--instrument PATH]

Exit status 0 means that every control passed. --instrument runs the same
cases against another revision of the instrument, such as the reviewed one.
A revision that cannot see a class of change must fail its case.
"""
import json, os, shutil, subprocess, sys, tempfile

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
CODEX_TOML = '[mcp_servers.quint-kb]\ncommand = "node"\nargs = ["kb.js"]\n\n[plugins."visualize@openai-bundled"]\nenabled = false\n'

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

def run(home):
    env = {k: v for k, v in os.environ.items() if k != "CONTROL_ROOT"}; env["HOME"] = home
    out = subprocess.run([sys.executable, "-I", instrument], env=env, capture_output=True, text=True)
    if out.returncode: return None
    return {(r[0], r[1], r[2]): r for r in (line.split("\t") for line in out.stdout.splitlines()[1:])}

def fixture_run(tmp, name, mutate):
    home = os.path.join(tmp, name.replace(" ", "-"), "home")
    os.makedirs(home); build(home); mutate(home)
    return run(home)

tmp = tempfile.mkdtemp(prefix="owner-inventory-controls.")
failed = 0
try:
    base = fixture_run(tmp, "base", lambda h: None)
    if base is None: sys.exit(f"controls: {instrument} failed on the unchanged fixture")
    for name, mutate, expect in CASES:
        got = fixture_run(tmp, name, mutate)
        if got is None: ok, why = False, "instrument failed"
        elif expect[0] == "same": ok, why = got == base, "output differs from the unchanged run"
        elif expect[0] == "changed":
            k = expect[1]; ok = k in base and k in got and base[k] != got[k]; why = f"entry {k[2]} did not change"
        elif expect[0] == "added":
            k = expect[1]; ok = k not in base and k in got; why = f"entry {k[2]} was not added"
        else:
            ok = any(r[10] == expect[1] for k, r in got.items() if k not in base); why = f"no new {expect[1]} row"
        if not ok and got is not None and expect[0] != "same" and got == base: why += "; whole output unchanged"
        print(("PASS " if ok else "FAIL ") + name + ("" if ok else f": {why}"))
        failed += not ok
finally:
    shutil.rmtree(tmp)
print(f"{len(CASES) - failed} of {len(CASES)} controls passed with {instrument}")
sys.exit(1 if failed else 0)
