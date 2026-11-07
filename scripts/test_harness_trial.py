#!/usr/bin/env python3
"""Test scripts/harness_trial.py against fake harnesses, so that no test makes
a real call or spends subscription usage.

The tests call the runner in process, with a fake real home, a fake prepared
directory and a fake environment. A fake `grove` driver stands in for Grove and
Ravel: it reads the route that the runner wrote, gives the trial its own
channel and launch directory, and starts the real launch wrapper. A fake
`claude` answers the probes and writes a stream and a session transcript. A
fake `codex` writes its session file and its stream in the shapes of Codex
0.166, with the account data of a real session file.

Each check also runs once against a deliberately broken runner: one runner
function replaced by a broken version. The check must fail there, so a check
that reads clean for every runner fails this file. The real trials are the
opt-in tasks `trial:claude` and `trial:codex`.
"""

from __future__ import annotations

import contextlib
import hashlib
import io
import json
import os
import platform
import shutil
import sys
import tempfile
import textwrap
import unittest
from pathlib import Path
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parent))
import harness_trial as trial  # noqa: E402

LIVE = {
    # The live session's authority, as a runner inside a session inherits it.
    "RAVEL_EXIT_FILE": "/live/session/exit",
    "GROVE_LAUNCH_DIR": "/live/session/launch",
    "CLAUDECODE": "1",
    "CLAUDE_CODE_SESSION_ID": "live-session",
    "CLAUDE_CODE_ENTRYPOINT": "cli",
    "ANTHROPIC_API_KEY": "sk-live",
    # A live Codex session's names, as a runner inside Codex inherits them.
    "CODEX_THREAD_ID": "live-thread",
    "CODEX_SANDBOX": "seatbelt",
}

# macOS adds __CF_USER_TEXT_ENCODING to the environment of every process, so
# the fake driver holds it although the runner never passes it.
ALLOWED = {"HOME", "TMPDIR", "PATH", "SHELL", "TERM", "__CF_USER_TEXT_ENCODING", *trial.PASSED_NAMES}

FAKE_GROVE = r'''#!{python}
# A fake driver: one launch through the real wrapper, then the refusal of the
# next kind, as the trial configuration ends a real loop.
import json, os, subprocess, sys, hashlib
from pathlib import Path
home = Path(os.environ["HOME"])
trial = home.parent
Path({env_file!r}).write_text(json.dumps(dict(os.environ)))
Path({env_file!r} + ".argv").write_text(json.dumps(sys.argv))
settings = home / ".config/ravel/settings.json"
for source in (json.loads(settings.read_text())["skills"]["sources"] if settings.exists() else []):
    absent = [path for path in [Path(source["path"])] + [Path(source["path"]) / root for root in source["roots"]]
              if not path.is_dir()]
    if absent:
        # Ravel refuses a missing source or root before any worker, and
        # Grove's loop stops without a relaunch.
        print(f"ravel: refused (skill_source_missing, stage skills): the source {{source['id']!r}} at "
              f"{{absent[0]}} does not exist", file=sys.stderr)
        print("  remedy: restore the source directory, or change the settings deliberately", file=sys.stderr)
        print("grove: session kind `impl` for `smoke-file-k1` failed; rerun `grove` to continue", file=sys.stderr)
        sys.exit(0)
route = json.loads((trial / "route.json").read_text())
# The snapshot captures the configured source's craft skill, as Ravel captures
# the current bytes. Changed bytes give a new snapshot and a new descriptor;
# the snapshot of an earlier launch stays as it was.
config = (json.loads(settings.read_text()) if settings.exists() else {{}}).get("skills", {{}})
origin = config["sources"][0]["path"] if config.get("sources") else "/origin/grove"
craft_origin = Path(origin) / "skills/craft/SKILL.md"
craft = craft_origin.read_text() if craft_origin.is_file() else "craft\n"
content = hashlib.sha256(craft.encode()).hexdigest()
unchanged = craft == "craft\n"
snap = home / ".cache/ravel/skills/objects" / ("grove-snapshot" if unchanged else "grove-" + content[:12])
if not snap.exists():
    (snap / "skills/grove-impl").mkdir(parents=True)
    (snap / "skills/grove-impl/SKILL.md").write_text("kind\n")
    (snap / "skills/grove/references").mkdir(parents=True)
    (snap / "skills/grove/SKILL.md").write_text("spine\n")
    (snap / "skills/grove/references/execute.md").write_text("resource\n")
    (snap / "skills/craft").mkdir(parents=True)
    (snap / "skills/craft/SKILL.md").write_text(craft)
mandatory = [{{"source": "grove", "path": str(snap / item["path"])}} for item in config.get("mandatory", [])]
mandatory = mandatory or [{{"source": "grove", "path": str(snap / "skills/grove/SKILL.md")}}]
descriptor_text = (b'{{"schemaVersion":1}}' if unchanged and not config.get("mandatory")
                   else json.dumps({{"content": content, "mandatory": mandatory}}).encode())
digest = hashlib.sha256(descriptor_text).hexdigest()
descriptor = home / ".cache/ravel/descriptors" / digest / "directories.json"
descriptor.parent.mkdir(parents=True, exist_ok=True)
descriptor.write_bytes(descriptor_text)
def fake_digest(directory):
    # A stand-in for Ravel's resourceDigest: every file name and its bytes.
    digest = hashlib.sha256()
    for path in sorted(p for p in Path(directory).rglob("*") if p.is_file()):
        digest.update(str(path.relative_to(directory)).encode() + b"\0" + path.read_bytes())
    return digest.hexdigest()
others = [{{"id": source["id"], "type": "local", "resourceDigest": fake_digest(source["path"]),
           "snapshotRoot": str(snap) + "-" + source["id"], "originPath": source["path"]}}
          for source in config.get("sources", [])[1:]]
view = {{"kind": "impl", "originalPrompt": "mandate", "skills": {{
    "descriptorPath": str(descriptor),
    "sources": [{{"id": "grove", "type": "local", "resourceDigest": "ab" * 32 if unchanged else content,
                 "snapshotRoot": str(snap), "originPath": origin}}, *others],
    "directories": [{{"source": "grove", "path": str(snap / "skills")}}],
    "mandatory": mandatory}}}}
if sys.argv[1:2] == ["run"]:
    # A fake `grove run`: the wrapper writes to the run log, as a standalone
    # launch does.
    work = trial / "tmp/work"
    work.mkdir()
    logs = home / ".local/state/grove/runs"
    logs.mkdir(parents=True)
    env = dict(os.environ, RAVEL_EXIT_FILE=str(trial / "tmp/exit"), RAVEL_SKILL_DIRECTORIES=str(descriptor))
    args = [route["wrapper"], route["records"], json.dumps(view), route["program"]]
    args += [*(["--add-dir", str(snap)] if route.get("addDirs", True) else []), *route["fixedArgs"], "standalone task"]
    with (logs / "grove-run-fake.log").open("w") as log:
        print("ravel: a line of the run log", file=log, flush=True)
        status = subprocess.run(args, env=env, cwd=work, stdin=subprocess.DEVNULL, stdout=log, stderr=log).returncode
    output = sys.argv[sys.argv.index("--output") + 1]
    Path(output).write_text("#!/usr/bin/env bash\n")
    sys.exit(status)
launch = trial / "tmp/launch"
launch.mkdir(exist_ok=True)
env = dict(os.environ, GROVE_LAUNCH_DIR=str(launch), RAVEL_EXIT_FILE=str(trial / "tmp/exit"),
           RAVEL_SKILL_DIRECTORIES=str(descriptor))
print("grove: launching impl through Ravel — smoke-file-k1", file=sys.stderr, flush=True)
# The prepared prompt: the preamble, then the caller block. A compaction case
# and a delegation read the preamble.
if True:
    prompt = ("Ravel skill directories\n\nSearch directories:\n- " + json.dumps(str(snap / "skills"))
              + "\n\nMandatory files, in order:\n1. " + json.dumps(mandatory[0]["path"])
              + "\n\nDescriptor: " + json.dumps(str(descriptor))
              + "\n\nObligations:\n10. Keep the descriptor path in each continuation or compaction summary, "
              + "and read RAVEL_SKILL_DIRECTORIES when the path is lost.\n"
              + "11. Give each delegate the descriptor path, these obligations and read access to these files.\n"
              + "\nCaller task\n"
              + "BEGIN RAVEL CALLER TASK 0123\nGrove mandate: the leaf\nEND RAVEL CALLER TASK 0123")
args = [route["wrapper"], route["records"], json.dumps(view), route["program"], *route.get("programArgs", []),
        *(["--add-dir", str(snap)] if route.get("addDirs", True) else []), *route["fixedArgs"], prompt]
status = subprocess.run(args, env=env, stdin=subprocess.DEVNULL).returncode
print(f"grove: session ended without a supported Grove result — status {{status}}; loop stopped.", file=sys.stderr)
sys.exit(1)
'''

FAKE_CLAUDE = r'''#!{python}
# A fake Claude Code. Its behaviour file says what it does.
import json, os, sys
from pathlib import Path
behaviour = json.loads(Path({behaviour!r}).read_text())
with open({launched!r}, "a") as log:
    log.write(" ".join(sys.argv[1:2]) + "\n")
home = Path(os.environ["HOME"])
if sys.argv[1:2] == ["--version"]:
    print("9.9.9 (Claude Code)")
    sys.exit(0)
if sys.argv[1:3] == ["auth", "status"]:
    print(json.dumps({{"loggedIn": True, "authMethod": "claude.ai",
                      "configDirectory": behaviour.get("configDirectory") or str(home / ".claude")}}))
    sys.exit(0)
# The plugin commands. A marketplace is a directory with `plugins/<name>`.
# An installation copies the plugin's directory into the cache and installs the
# plugin's dependencies of the behaviour file, as `auto` installations.
if sys.argv[1:2] == ["plugin"]:
    import shutil
    plugins = home / ".claude/plugins"
    plugins.mkdir(parents=True, exist_ok=True)
    known_file, installed_file = plugins / "known_marketplaces.json", plugins / "installed_plugins.json"
    args = sys.argv[2:]
    if args[:2] == ["marketplace", "add"]:
        known_file.write_text(json.dumps({{"fake": {{"installLocation": args[2]}}}}))
        print(json.dumps({{"command": "marketplace-add", "outcome": "ok", "marketplace": "fake"}}))
        sys.exit(0)
    installed = json.loads(installed_file.read_text()) if installed_file.exists() else {{"version": 2, "plugins": {{}}}}
    if args[:1] == ["install"]:
        name, market = args[1].split("@")
        if name in behaviour.get("pluginFails", []):
            print(json.dumps({{"command": "install", "outcome": "error", "message": f"no plugin {{name}}"}}))
            sys.exit(1)
        source = Path(json.loads(known_file.read_text())[market]["installLocation"])
        for each, auto in [(name, False)] + [(d, True) for d in behaviour.get("pluginDependencies", {{}}).get(name, [])]:
            target = plugins / "cache" / market / each / "v1"
            directory = source / "plugins" / each
            if directory.is_dir():
                shutil.copytree(directory, target)
            else:
                (target / "skills").mkdir(parents=True)
            installed["plugins"][f"{{each}}@{{market}}"] = [{{
                "scope": "user", "installPath": str(target), "version": "v1", "gitCommitSha": "a" * 40,
                **({{"auto": True}} if auto else {{}})}}]
        installed_file.write_text(json.dumps(installed))
        print(json.dumps({{"command": "install", "outcome": "ok", "plugin": args[1], "message": "installed"}}))
        sys.exit(0)
    if args[:1] == ["list"]:
        print(json.dumps([{{"id": ident, "version": entries[0]["version"], "scope": "user", "enabled": True,
                            "installPath": entries[0]["installPath"]}}
                           for ident, entries in installed["plugins"].items()]))
        sys.exit(0)
    sys.exit(2)
# A resumed turn has no --add-dir, and it reads the first turn's snapshot.
saved = home / ".fake-claude-snap"
if "--add-dir" in sys.argv:
    snap = sys.argv[sys.argv.index("--add-dir") + 1]
    saved.write_text(snap)
else:
    snap = saved.read_text() if saved.exists() else ""
names = {{"SNAP": snap, "HOME": str(home), "DESC": os.environ.get("RAVEL_SKILL_DIRECTORIES", ""),
         "CWD": os.getcwd(), "TRIAL": str(home.parent)}}
project = home / ".claude/projects/-fake"
resume = sys.argv[sys.argv.index("--resume") + 1] if "--resume" in sys.argv else None
# A resumed turn keeps the session ID, as Claude Code does without
# --fork-session. Each new session has an ID of its own.
count = len(list(project.glob("*.jsonl"))) if project.is_dir() else 0
session = resume or f"{{count + 1:08d}}-2222-3333-4444-555555555555"
tools = behaviour.get("resumeTools", behaviour["tools"]) if resume else behaviour["tools"]
if behaviour.get("noInit"):
    print("Error: Input must be provided", file=sys.stderr)
    sys.exit(1)
print(json.dumps({{"type": "system", "subtype": "init", "session_id": session, "model": "fake",
                  "permissionMode": "bypassPermissions", "tools": ["Read", "Bash"], "skills": [],
                  "claude_code_version": "9.9.9"}}))
entries = [{{"type": "attachment", "attachment": {{"type": "skill_listing", "skillCount": 0, "names": []}}}},
           {{"type": "attachment", "attachment": {{"type": "credential_org", "org": "secret-org"}}}}]
def tool_entries(tools, first):
    found = []
    for n, (tool, field, value, error) in enumerate(tools):
        if tool == "text":
            found.append({{"type": "assistant", "message": {{"content": [{{"type": "text", "text": value}}]}}}})
            continue
        tid = f"toolu_{{first + n}}"
        target = value.format(**names)
        found.append({{"type": "assistant", "message": {{"content": [
            {{"type": "tool_use", "id": tid, "name": tool, "input": {{field: target}}}}]}}}})
        # A read returns the bytes of the file, so a marker in them reaches the result.
        text = Path(target).read_text() if tool == "Read" and Path(target).is_file() else f"{{tool}} result"
        found.append({{"type": "user", "message": {{"content": [
            {{"type": "tool_result", "tool_use_id": tid, "is_error": error,
              "content": [{{"type": "text", "text": text}}]}}]}}}})
    return found
if "--input-format" in sys.argv:
    # Stream-json turns: the prompt, `/compact`, then the answer. Each turn
    # appends to the session file before its result, as Claude Code does.
    import re
    project.mkdir(parents=True, exist_ok=True)
    path = project / f"{{session}}.jsonl"
    def append(found):
        with path.open("a") as transcript:
            transcript.write("".join(json.dumps(e) + "\n" for e in found))
    append(entries)
    summary = None
    for turn, raw in enumerate(sys.stdin):
        text = json.loads(raw)["message"]["content"]
        if turn == 0:
            names["SNAP"] = re.search(r'- "(.+)/skills"', text).group(1)
            first = text
        if text.startswith("/compact "):
            summary = behaviour.get("summary", "The leaf asks for one file.").format(**names)
            append([{{"type": "system", "subtype": "compact_boundary"}},
                    {{"type": "user", "isCompactSummary": True, "message": {{"role": "user", "content": summary}}}},
                    {{"type": "user", "isMeta": True, "message": {{"role": "user", "content": "<local-command-caveat>c</local-command-caveat>"}}}},
                    {{"type": "user", "message": {{"role": "user", "content":
                        "<command-name>/compact</command-name>\n<command-args>" + text[9:] + "</command-args>"}}}},
                    {{"type": "user", "message": {{"role": "user", "content": "<local-command-stdout>Compacted </local-command-stdout>"}}}},
                    *[{{"type": "attachment", "attachment": {{"type": "file", "filename": item.format(**names)}}}}
                      for item in behaviour.get("restored", [])],
                    # Bookkeeping that holds the first prompt and never reaches the model.
                    {{"type": "last-prompt", "lastPrompt": first}}])
            print(json.dumps({{"type": "system", "subtype": "compact_boundary", "session_id": session}}))
        else:
            found = [{{"type": "user", "message": {{"role": "user", "content": text}}}}]
            if summary is not None:
                # The request record of the first model request after compaction.
                found += [
                    {{"type": "api-request-shape", "shapeHash": "s1", "shape": {{"system": [
                        {{"type": "text", "text": behaviour.get("system", "You are a fake.").format(**names)}}]}}}},
                    {{"type": "api-request-blob", "hash": "b1", "message": {{"role": "user", "content": [
                        {{"type": "text", "text": summary}}, {{"type": "text", "text": text}}]}}}},
                    {{"type": "api-request", "keep": behaviour.get("keep", 0), "tail": ["b1"], "shapeHash": "s1",
                      "requestDigest": "d1"}}]
            found += tool_entries(behaviour["tools"] if turn == 0 else behaviour.get("afterTools", []), 100 * turn)
            append(found)
        print(json.dumps({{"type": "result", "subtype": "success"}}), flush=True)
    sys.exit(0)
entries += tool_entries(tools, 0)
delegate = behaviour.get("delegate")
if delegate and not resume:
    # One Agent call. Claude Code writes the delegate's transcript beside the
    # session, with a meta file that names the call.
    text = delegate["prompt"].format(**names)
    entries += [
        {{"type": "assistant", "message": {{"content": [{{"type": "tool_use", "id": "toolu_agent", "name": "Agent",
            "input": {{"description": "Write the check", "prompt": text, "subagent_type": "general-purpose"}}}}]}}}},
        {{"type": "user", "toolUseResult": {{"status": "completed", "agentId": "a1"}}, "message": {{"content": [
            {{"type": "tool_result", "tool_use_id": "toolu_agent", "content": [{{"type": "text", "text": "Check written."}}]}}]}}}},
    ]
    sub = project / session / "subagents"
    sub.mkdir(parents=True, exist_ok=True)
    found = [{{"type": "user", "isSidechain": True, "message": {{"role": "user", "content": text}}}}]
    found += [dict(e, isSidechain=True) for e in tool_entries(delegate["tools"], 500)]
    (sub / "agent-a1.jsonl").write_text("".join(json.dumps(e) + "\n" for e in found))
    (sub / "agent-a1.meta.json").write_text(json.dumps({{"agentType": "general-purpose", "toolUseId": "toolu_agent",
                                                        "spawnDepth": 1, "finished": True}}))
for item in behaviour.get("writeFiles", []):
    Path(item["path"].format(**names)).write_text(item["text"])
if behaviour.get("noSession"):
    # A harness that writes no session file. Its stream holds the same tool
    # calls.
    for entry in entries:
        print(json.dumps(entry))
else:
    project.mkdir(parents=True, exist_ok=True)
    with (project / f"{{session}}.jsonl").open("a" if resume else "w") as transcript:
        transcript.write("".join(json.dumps(e) + "\n" for e in entries))
store = home / {store!r}
if behaviour.get("replaceLink"):
    store.unlink()
    store.mkdir()
if behaviour.get("extraLink"):
    os.symlink("/", home / ".claude/extra")
if behaviour.get("strayNative"):
    (home / ".claude/skills/stray").mkdir(parents=True)
    (home / ".claude/skills/stray/SKILL.md").write_text("stray\n")
if behaviour.get("strayPlugin"):
    (home / ".claude/plugins/cache/other/stray/v1/skills/stray").mkdir(parents=True)
    (home / ".claude/plugins/cache/other/stray/v1/skills/stray/SKILL.md").write_text("stray\n")
print(json.dumps({{"type": "result", "subtype": "success"}}))
'''

FAKE_CODEX = r'''#!{python}
# A fake Codex 0.166. Its behaviour file says what it does. It writes its
# session file and its --json stream in the shapes of a real run, with the
# account data that a real session file holds.
import json, os, re, shlex, sys
from pathlib import Path
from urllib.parse import quote
behaviour = json.loads(Path({behaviour!r}).read_text())
with open({launched!r}, "a") as log:
    log.write(" ".join(sys.argv[1:2]) + "\n")
home = Path(os.environ["HOME"])
codex_home = Path(os.environ.get("CODEX_HOME") or home / ".codex")
args = sys.argv[1:]
if args[:1] == ["--version"]:
    print("codex-cli 9.9.9")
    sys.exit(0)
def residue():
    # The bundled system skills, and a curated plugin that a ChatGPT sign-in downloads.
    for name in ("imagegen", "openai-docs"):
        (codex_home / "skills/.system" / name).mkdir(parents=True, exist_ok=True)
        (codex_home / "skills/.system" / name / "SKILL.md").write_text(name + "\n")
    page = codex_home / "plugins/cache/openai-curated-remote/pages/0.1.26/skills/write-page"
    page.mkdir(parents=True, exist_ok=True)
    (page / "SKILL.md").write_text("page\n")
if args[:2] == ["login", "status"]:
    if behaviour.get("loggedOut"):
        print("Not logged in", file=sys.stderr)
        sys.exit(1)
    print("Logged in using ChatGPT")
    sys.exit(0)
root = behaviour.get("skillRoot") or str(codex_home / "skills/.system")
skills = ("<skills_instructions>\n## Skills\n### Skill roots\n- `r0` = `" + root + "`\n### Available skills\n"
          "- imagegen: Make images. (file: r0/imagegen/SKILL.md)\n</skills_instructions>")
items = [
    {{"type": "additional_tools", "role": "developer", "tools": [
        {{"type": "namespace", "name": "functions", "tools": [{{"type": "custom", "name": "exec"}}]}}]}},
    {{"type": "message", "role": "developer", "content": [{{"type": "input_text", "text": "You are a fake Codex."}}]}},
    {{"type": "message", "role": "developer", "content": [{{"type": "input_text", "text": skills}}]}},
    {{"type": "message", "role": "user", "content": [{{"type": "input_text",
        "text": "<environment_context>\n  <cwd>" + os.getcwd() + "</cwd>\n</environment_context>"}}]}},
]
if args[:2] == ["debug", "prompt-input"]:
    residue()
    print(json.dumps(items))
    sys.exit(0)
if args[:1] == ["app-server"]:
    # A compaction case runs the app-server, which the fake of its own serves.
    import runpy
    sys.argv = [{server!r}, {behaviour!r}, *args]
    runpy.run_path({server!r}, run_name="__main__")
    sys.exit(0)
assert args[0] == "exec", args
prompt = args[-1]
# `codex exec ... resume <thread> <prompt>` keeps the thread and appends to
# its session file. The resumed prompt has no preamble, so the resumed turn
# reads the first turn's snapshot.
resume = args[args.index("resume") + 1] if "resume" in args else None
saved = home / ".fake-codex-snap"
snap = re.search(r'- "(.+)/skills"', prompt)
if snap and not resume:
    saved.write_text(snap.group(1))
snap = snap.group(1) if snap else (saved.read_text() if saved.exists() else "")
names = {{"SNAP": snap, "HOME": str(home), "CWD": os.getcwd(), "TRIAL": str(home.parent)}}
residue()
sessions = codex_home / "sessions/2026/11/03"
count = len(list(sessions.glob("*.jsonl"))) if sessions.is_dir() else 0
thread = resume or f"01a19ed6-091b-71d1-8857-{{count + 1:012d}}"
print(json.dumps({{"type": "thread.started", "thread_id": thread}}), flush=True)
print(json.dumps({{"type": "turn.started"}}), flush=True)
context = {{"type": "turn_context", "payload": {{"cwd": os.getcwd(), "model": args[args.index("--model") + 1],
    "effort": "medium", "approval_policy": "never",
    "sandbox_policy": {{"type": args[args.index("--sandbox") + 1], "network_access": False}}}}}}
records = [] if resume else [
    {{"type": "session_meta", "payload": {{"id": thread, "session_id": thread, "cwd": os.getcwd(), "cli_version": "9.9.9",
        "originator": "codex_exec", "source": "exec", "model_provider": "openai", "creator_account_id": "secret-account",
        "creator_user_id": "secret-user", "base_instructions": "You are a fake Codex."}}}}]
records += [
    context,
    *([] if resume else [{{"type": "response_item", "payload": item}} for item in items]),
    {{"type": "event_msg", "payload": {{"type": "token_count", "rate_limits": {{"credits": {{"balance": "secret-balance"}}}}}}}},
]
def output(value):
    # A read returns the bytes of the file, so a marker in them reaches the result.
    found = re.fullmatch(r'cat "([^"]+)"', value)
    if behaviour.get("readFiles") and found and Path(found.group(1)).is_file():
        return Path(found.group(1)).read_text()
    return "out"
stream = []
def build(chosen, records, stream):
    for n, (kind, value, cwd, code) in enumerate(chosen):
        value = value.format(**names)
        where = cwd.format(**names) if cwd else os.getcwd()
        if kind == "text":
            item = {{"type": "AgentMessage", "id": f"m{{n}}", "content": [{{"type": "Text", "text": value}}]}}
            shown = {{"id": f"item_{{n}}", "type": "agent_message", "text": value}}
        elif kind == "patch":
            Path(value).write_text("smoke\n")
            item = {{"type": "FileChange", "id": f"p{{n}}", "status": "completed", "changes": {{value: {{"type": "add"}}}}}}
            shown = {{"id": f"item_{{n}}", "type": "file_change", "status": "completed", "changes": [{{"path": value, "kind": "add"}}]}}
        else:
            item = {{"type": "CommandExecution", "id": f"c{{n}}", "command": ["/bin/zsh", "-lc", value],
                    "cwd": "file://" + quote(where), "status": "completed", "exit_code": code,
                    "aggregated_output": output(value)}}
            shown = {{"id": f"item_{{n}}", "type": "command_execution", "command": "/bin/zsh -lc " + shlex.quote(value),
                     "aggregated_output": output(value), "exit_code": code, "status": "completed"}}
        records.append({{"type": "event_msg", "payload": {{"type": "item_completed", "item": item}}}})
        stream.append({{"type": "item.completed", "item": shown}})
build(behaviour.get("resumeItems", []) if resume else behaviour["codexItems"], records, stream)
delegate = behaviour.get("codexDelegate")
if delegate and not resume:
    # One delegate of multi-agent v2: the parent's spawn call, the start of
    # the delegate with the call's ID, and the delegate's own session file,
    # whose metadata names the parent thread. A direct call holds the message
    # encrypted; a call from code mode holds it in the JavaScript of `exec`.
    child = f"01a19ed6-5e40-7dc2-9400-{{count + 1:012d}}"
    text = delegate["prompt"].format(**names, DESC=os.environ.get("RAVEL_SKILL_DIRECTORIES", ""))
    if delegate.get("codeMode"):
        call = "cell_1"
        records.append({{"type": "response_item", "payload": {{"type": "custom_tool_call", "name": "exec",
            "call_id": "call_exec", "input": "await tools.spawn_agent(" + json.dumps({{"task_name": "check", "message": text}}) + ")"}}}})
    else:
        call = "call_spawn"
        arguments = {{"task_name": "check", "message": "gAAAAABqFake_token-0" if delegate.get("encrypted") else text}}
        if "fork" in delegate:
            arguments["fork_turns"] = delegate["fork"]
        records.append({{"type": "response_item", "payload": {{"type": "function_call", "name": "spawn_agent",
            "namespace": "collaboration", "call_id": call, "arguments": json.dumps(arguments)}}}})
    records += [
        {{"type": "event_msg", "payload": {{"type": "item_completed", "item": {{"type": "SubAgentActivity", "id": call,
            "kind": "started", "agent_thread_id": child, "agent_path": "/root/check"}}}}}},
        {{"type": "response_item", "payload": {{"type": "function_call", "name": "send_message",
            "namespace": "collaboration", "call_id": "call_send", "arguments": json.dumps({{"target": "check", "message": "gAAAAABqFake"}})}}}},
        {{"type": "response_item", "payload": {{"type": "agent_message", "author": "/root/check", "recipient": "/root",
            "content": [{{"type": "input_text", "text": "Message Type: MESSAGE\nPayload:\n"}},
                        {{"type": "encrypted_content", "encrypted_content": "gAAAAABqFake"}}]}}}},
        {{"type": "response_item", "payload": {{"type": "agent_message", "author": "/root/check", "recipient": "/root",
            "content": [{{"type": "input_text", "text": "Message Type: FINAL_ANSWER\nPayload:\nCheck written."}}]}}}},
        {{"type": "event_msg", "payload": {{"type": "item_completed", "item": {{"type": "SubAgentActivity",
            "id": "subagent-completed", "kind": "completed", "agent_thread_id": child, "agent_path": "/root/check"}}}}}},
    ]
    own = {{"type": "session_meta", "payload": {{"id": child, "session_id": thread, "parent_thread_id": thread,
        "cwd": os.getcwd(), "cli_version": "9.9.9", "creator_account_id": "secret-account",
        "source": {{"subagent": {{"thread_spawn": {{"parent_thread_id": thread, "depth": 1, "agent_path": "/root/check",
                                                 "agent_nickname": "Bohr", "agent_role": None}}}}}}}}}}
    # A forked delegate copies the parent's history, with its metadata and
    # the prompt, but none of the parent's completed items.
    forked = [{{"type": "session_meta", "payload": {{"id": thread, "cwd": os.getcwd()}}}},
              {{"type": "response_item", "payload": {{"type": "message", "role": "user",
                  "content": [{{"type": "input_text", "text": prompt}}]}}}}] if delegate.get("fork", "all") == "all" else []
    child_records = [own, *forked, context]
    build(delegate["items"], child_records, [])
    sessions.mkdir(parents=True, exist_ok=True)
    (sessions / f"rollout-2026-11-03T00-00-9{{count}}-{{child}}.jsonl").write_text(
        "".join(json.dumps(r) + "\n" for r in child_records))
if not behaviour.get("noSession"):
    sessions.mkdir(parents=True, exist_ok=True)
    found = sorted(sessions.glob(f"rollout-*-{{thread}}.jsonl"))
    path = found[0] if resume and found else sessions / f"rollout-2026-11-03T00-00-0{{count}}-{{thread}}.jsonl"
    with path.open("a" if resume else "w") as transcript:
        transcript.write("".join(json.dumps(r) + "\n" for r in records))
for event in stream:
    print(json.dumps(event))
store = home / ".codex/auth.json"
if behaviour.get("replaceLink"):
    store.unlink()
    store.write_text("{{}}\n")
if behaviour.get("extraLink"):
    os.symlink("/", codex_home / "extra")
# Codex's own helper links to its executable, which a group stop leaves behind.
helpers = codex_home / "tmp/arg0-macos/codex-arg0Fake"
if behaviour.get("arg0Links") or behaviour.get("arg0Foreign"):
    helpers.mkdir(parents=True)
    for name in ("apply_patch", "applypatch"):
        os.symlink(os.path.realpath(sys.argv[0]), helpers / name)
if behaviour.get("arg0Foreign"):
    os.symlink("/", helpers / "other")
if behaviour.get("strayNative"):
    (codex_home / "skills/stray").mkdir(parents=True)
    (codex_home / "skills/stray/SKILL.md").write_text("stray\n")
print(json.dumps({{"type": "turn.completed", "usage": {{"input_tokens": 1}}}}))
'''

# The items of the fake Codex session: (kind, value, working directory,
# exit code). Each command names its working directory, and a relative read
# resolves against it.
CODEX_ITEMS = [
    ("command", "cat \"{SNAP}/skills/grove/SKILL.md\"", None, 0),
    ("command", "sed -n 1,80p skills/grove/references/execute.md", "{SNAP}", 0),
    ("command", "cd skills && cat grove-impl/SKILL.md", "{SNAP}", 0),
    ("command", "cat \"{SNAP}/skills/craft/SKILL.md\"", None, 0),
    ("command", "cat \"$RAVEL_SKILL_DIRECTORIES\"", None, 0),
    ("command", "cat {HOME}/.codex/skills/.system/imagegen/SKILL.md", None, 0),
    ("command", "cat {HOME}/.codex/skills/old/SKILL.md", None, 1),
    ("text", "I read the craft skill.", None, None),
    # A script of two lines: the cd on the second line moves the third read.
    ("command", "echo start\ncd skills\ncat craft/SKILL.md", "{SNAP}", 0),
    ("patch", "{CWD}/smoke.txt", None, None),
    ("command", "jj commit -m smoke", None, 0),
    ("command", "'/x/bin/ravel' end-session grove-continue", None, 0),
]
CODEX_EXPECTED = [
    ("exec_command", "mandatory", "grove", False),
    ("exec_command", "supporting", "grove", False),
    ("exec_command", "kind-skill", "grove-impl", False),
    ("exec_command", "skill", "craft", False),
    ("exec_command", "variable", None, False),
    ("exec_command", "residue", "imagegen", False),
    ("exec_command", "native", "old", True),
    ("text", "text", None, None),
    ("exec_command", "skill", "craft", False),
    ("apply_patch", "action", None, False),
    ("exec_command", "command", None, False),
    ("exec_command", "ending", None, False),
]

FAKE_CODEX_SERVER = r'''
# A fake `codex app-server` of 0.166 over stdio: one JSON-RPC message to a line.
# It writes its session file as a real app-server does. A manual compaction
# writes a `compacted` entry: with `--enable token_budget`, a fresh window that
# keeps no user message; without it, the remote form, which keeps the prompt
# word for word and adds one encrypted item.
import json, os, re, shlex, sys
from pathlib import Path
from urllib.parse import quote
behaviour = json.loads(Path(sys.argv[1]).read_text())
args = sys.argv[2:]
home = Path(os.environ["HOME"])
codex_home = Path(os.environ.get("CODEX_HOME") or home / ".codex")
budget = "--enable" in args and args[args.index("--enable") + 1] == "token_budget"
thread = "01a19fa3-a888-73f0-82ab-000000000001"
sessions = codex_home / "sessions/2026/11/03"
sessions.mkdir(parents=True, exist_ok=True)
path = sessions / f"rollout-2026-11-03T00-00-00-{thread}.jsonl"
model = next(arg.split("=", 1)[1].strip('"') for arg in args if arg.startswith("model="))
skills = ("<skills_instructions>\n## Skills\n### Skill roots\n- `r0` = `" + str(codex_home / "skills/.system")
          + "`\n### Available skills\n- imagegen: Make images. (file: r0/imagegen/SKILL.md)\n</skills_instructions>")
developer = [
    {"type": "additional_tools", "role": "developer", "tools": [
        {"type": "namespace", "name": "functions", "tools": [{"type": "custom", "name": "exec"}]}]},
    {"type": "message", "role": "developer", "content": [{"type": "input_text", "text": "You are a fake Codex."}]},
    {"type": "message", "role": "developer", "content": [{"type": "input_text", "text": skills}]},
]
state = {"turn": 0, "prompt": None, "snap": "", "cwd": os.getcwd()}


def out(message):
    print(json.dumps({"jsonrpc": "2.0", **message}), flush=True)


def record(*entries):
    with path.open("a") as transcript:
        transcript.write("".join(json.dumps(entry) + "\n" for entry in entries))


def message(role, text):
    return {"type": "message", "role": role, "content": [{"type": "input_text", "text": text}]}


def environment():
    return message("user", "<environment_context>\n  <cwd>" + state["cwd"] + "</cwd>\n</environment_context>")


def complete(item, shown):
    record({"type": "event_msg", "payload": {"type": "item_completed", "item": item}})
    out({"method": "item/completed", "params": {"threadId": thread, "item": shown}})


def run_items(items):
    names = {"SNAP": state["snap"], "HOME": str(home), "CWD": state["cwd"], "TRIAL": str(home.parent)}
    for n, (kind, value, cwd, code) in enumerate(items):
        value = value.format(**names)
        if kind == "text":
            complete({"type": "AgentMessage", "id": f"m{n}", "content": [{"type": "Text", "text": value}]},
                     {"type": "agentMessage", "id": f"m{n}", "text": value})
            record({"type": "response_item", "payload": {"type": "message", "role": "assistant",
                                                         "content": [{"type": "output_text", "text": value}]}})
        else:
            where = cwd.format(**names) if cwd else state["cwd"]
            complete({"type": "CommandExecution", "id": f"c{n}", "command": ["/bin/zsh", "-lc", value],
                      "cwd": "file://" + quote(where), "status": "completed", "exit_code": code,
                      "aggregated_output": "out"},
                     {"type": "commandExecution", "id": f"c{n}", "command": "/bin/zsh -lc " + shlex.quote(value)})


for line in sys.stdin:
    request = json.loads(line)
    method, number = request.get("method"), request.get("id")
    params = request.get("params") or {}
    if method == "initialize":
        out({"id": number, "result": {"userAgent": "fake"}})
    elif method == "initialized":
        continue
    elif method == "thread/start":
        state["cwd"] = params.get("cwd") or state["cwd"]
        record({"type": "session_meta", "payload": {"id": thread, "session_id": thread, "cwd": state["cwd"],
                                                   "cli_version": "9.9.9", "originator": "grove-trial-feeder",
                                                   "source": "vscode", "model_provider": "openai",
                                                   "creator_account_id": "secret-account"}},
               {"type": "turn_context", "payload": {"cwd": state["cwd"], "model": model, "approval_policy": "never",
                                                    "sandbox_policy": {"type": "workspace-write"}}},
               *[{"type": "response_item", "payload": item} for item in [*developer, environment()]])
        out({"id": number, "result": {"thread": {"id": thread}}})
        out({"method": "thread/started", "params": {"thread": {"id": thread}}})
    elif method == "turn/start":
        text = params["input"][0]["text"]
        out({"id": number, "result": {"turn": {"id": f"t{state['turn']}"}}})
        if state["prompt"] is None:
            state["prompt"] = text
            found = re.search(r'- "(.+)/skills"', text)
            state["snap"] = found.group(1) if found else ""
        record({"type": "event_msg", "payload": {"type": "task_started"}},
               {"type": "response_item", "payload": message("developer", skills)},
               {"type": "response_item", "payload": message("user", text)})
        complete({"type": "UserMessage", "id": f"u{state['turn']}", "content": [{"type": "text", "text": text}]},
                 {"type": "userMessage", "id": f"u{state['turn']}"})
        run_items(behaviour["codexFirst"] if state["turn"] == 0 else behaviour["codexAfter"])
        record({"type": "event_msg", "payload": {"type": "task_complete"}})
        state["turn"] += 1
        out({"method": "turn/completed", "params": {"threadId": thread, "turn": {"status": "completed"}}})
    elif method == "thread/compact/start":
        out({"id": number, "result": {}})
        history = [*developer, environment()]
        if not budget or behaviour.get("keepPrompt"):
            history.append(message("user", state["prompt"]))
        if not budget:
            history.append({"type": "compaction", "encrypted_content": "gAAAA" + "x" * 40})
        record({"type": "event_msg", "payload": {"type": "task_started"}},
               {"type": "compacted", "payload": {"message": "", "replacement_history": history, "window_number": 1}})
        complete({"type": "ContextCompaction", "id": "cc"}, {"type": "contextCompaction", "id": "cc"})
        record({"type": "event_msg", "payload": {"type": "task_complete"}})
        out({"method": "turn/completed", "params": {"threadId": thread, "turn": {"status": "completed"}}})
    elif number is not None:
        out({"id": number, "error": {"code": -32601, "message": f"no method {method}"}})
'''

FAKE_FRONT = r'''#!{python}
# A fake 2.0 front. `skills resolve --json` names the descriptor that the fake
# driver published, as a real resolution of the same settings does.
import json, os, sys
from pathlib import Path
if sys.argv[1:] != ["skills", "resolve", "--json"]:
    sys.exit(0)
import hashlib
settings = Path(os.environ["HOME"]) / ".config/ravel/settings.json"
sources = json.loads(settings.read_text())["skills"]["sources"]
for source in sources:
    absent = [path for path in [Path(source["path"])] + [Path(source["path"]) / root for root in source["roots"]]
              if not path.is_dir()]
    if absent:
        print(f"ravel: refused (skill_source_missing, stage skills): {{absent[0]}} does not exist", file=sys.stderr)
        print("  remedy: restore the source directory, or change the settings deliberately", file=sys.stderr)
        sys.exit(3)
def fake_digest(directory):
    # The fake driver's stand-in for Ravel's resourceDigest.
    digest = hashlib.sha256()
    for path in sorted(p for p in Path(directory).rglob("*") if p.is_file()):
        digest.update(str(path.relative_to(directory)).encode() + b"\0" + path.read_bytes())
    return digest.hexdigest()
found = sorted(Path(os.environ["HOME"]).glob(".cache/ravel/descriptors/*/directories.json"))
print(json.dumps({{"schemaVersion": 1, "status": "resolved", "descriptorPath": str(found[0]) if found else None,
                  "sources": [{{"id": "grove", "resourceDigest": "ab" * 32}},
                              *[{{"id": s["id"], "resourceDigest": fake_digest(s["path"])}} for s in sources[1:]]]}}))
'''

# The transcript of the fake session, in tool-call order, with the class that
# extraction must give each call.
TOOLS = [
    ("Read", "file_path", "{SNAP}/skills/grove/SKILL.md", False),
    ("Read", "file_path", "{SNAP}/skills/grove/references/execute.md", False),
    ("Bash", "command", "cd \"{SNAP}/skills\" && find . -name SKILL.md | while read f; do head -8 \"$f\"; done", False),
    ("Bash", "command", "cd \"{SNAP}/skills\" && cat grove-impl/SKILL.md; ls grove-impl", False),
    ("Read", "file_path", "{SNAP}/skills/craft/SKILL.md", False),
    # A path in a variable that the same command assigns.
    ("Bash", "command", "S=\"{SNAP}/skills/grove\"; cat \"$S/references/execute.md\"; ls -R \"${{S}}\"", False),
    ("Read", "file_path", "{DESC}", False),
    ("Read", "file_path", "{HOME}/.claude/skills/old/SKILL.md", True),
    ("text", "text", "The native loader has no match, so I search the directories.", False),
    ("Skill", "skill", "grove-impl", True),
    # A concrete path that the case planted outside the repository.
    ("Read", "file_path", "{TRIAL}/guidance/craft/SKILL.md", True),
    ("Write", "file_path", "{CWD}/smoke.txt", False),
    ("Bash", "command", "jj commit -m smoke", False),
    ("Bash", "command", "'/x/bin/ravel' end-session grove-continue", False),
]
EXPECTED = [
    ("Read", "mandatory", "grove", False),
    ("Read", "supporting", "grove", False),
    ("Bash", "listing", None, False),
    ("Bash", "kind-skill", "grove-impl", False),
    ("Read", "skill", "craft", False),
    ("Bash", "supporting", "grove", False),
    ("Read", "descriptor", None, False),
    ("Read", "native", "old", True),
    ("text", "text", None, None),
    ("Skill", "native-skill-tool", None, True),
    ("Read", "planted", "craft", True),
    ("Write", "action", None, False),
    ("Bash", "command", None, False),
    ("Bash", "ending", None, False),
]

# The resumed turn of the continuation case: a mandatory read and a craft read
# in the first turn's snapshot, then a read of the changed source itself.
RESUME_TOOLS = [
    ("Read", "file_path", "{SNAP}/skills/grove/SKILL.md", False),
    ("Read", "file_path", "{SNAP}/skills/craft/SKILL.md", False),
    ("Read", "file_path", "{TRIAL}/sources/checkout/plugins/grove/skills/craft/SKILL.md", False),
]
MARKER = "k111-test-marker"
# The resumed turn of the continuation case in Codex: the same three reads, as
# `cat` commands of its `exec` tool.
CODEX_RESUME_ITEMS = [
    ("command", "cat \"{SNAP}/skills/grove/SKILL.md\"", None, 0),
    ("command", "cat \"{SNAP}/skills/craft/SKILL.md\"", None, 0),
    ("command", "cat \"{TRIAL}/sources/checkout/plugins/grove/skills/craft/SKILL.md\"", None, 0),
]

# The compaction cases: the first turn reads the spine and asks the human. The
# turn after compaction reads the variable, the spine again, the transcript and
# the environment.
FIRST_TOOLS = [
    ("Read", "file_path", "{SNAP}/skills/grove/SKILL.md", False),
    ("text", "text", "Which label do you want?", False),
]
AFTER_TOOLS = [
    ("Bash", "command", "cat \"$RAVEL_SKILL_DIRECTORIES\"", False),
    ("Read", "file_path", "{SNAP}/skills/grove/SKILL.md", False),
    ("Read", "file_path", "{HOME}/.claude/projects/-fake/00000001-2222-3333-4444-555555555555.jsonl", False),
    ("Bash", "command", "env | sort", False),
]
AFTER_EXPECTED = [
    ("prompt", None), ("mandatory", "grove"), ("text", None), ("compaction", None), ("prompt", None),
    ("prompt", None), ("variable", None), ("mandatory", "grove"), ("transcript", None), ("variable", None),
]
# The Codex compaction cases: the first turn reads the spine and asks the
# human. The turn after compaction reads the variable and the spine again, then
# ends the session.
CODEX_FIRST = [
    ("command", "cat \"{SNAP}/skills/grove/SKILL.md\"", None, 0),
    ("text", "Which label do you want?", None, None),
]
CODEX_AFTER = [
    ("command", "cat \"$RAVEL_SKILL_DIRECTORIES\"", None, 0),
    ("command", "cat \"{SNAP}/skills/grove/SKILL.md\"", None, 0),
    ("command", "'/x/bin/ravel' end-session grove-continue", None, 0),
]
CODEX_COMPACTION_EXPECTED = [
    ("prompt", None), ("mandatory", "grove"), ("text", None), ("compaction", None), ("prompt", None),
    ("variable", None), ("mandatory", "grove"), ("ending", None),
]
SUMMARIES = {
    "path": "Obligation 10: read RAVEL_SKILL_DIRECTORIES when the path is lost. The leaf asks for one file.",
    "preamble": "The leaf asks for one file, and the human has a question to answer.",
}
CRAFT = "plugins/grove/skills/craft/SKILL.md"

# A delegation: the parent gives the descriptor path, obligation 11 and a
# prohibition. The delegate reads the mandatory file, then a craft skill, then
# writes, then tries an ending of its own.
DELEGATE = {
    "prompt": ("Descriptor: {DESC}\n"
               "11. Give each delegate the descriptor path, these obligations and read access to these files.\n"
               "Do not run end-session.\nWrite check.sh."),
    "tools": [
        ("Read", "file_path", "{SNAP}/skills/grove/SKILL.md", False),
        ("Read", "file_path", "{SNAP}/skills/craft/SKILL.md", False),
        ("Write", "file_path", "{CWD}/check.sh", False),
        ("Bash", "command", "ravel end-session trial-delegate-early", False),
    ],
}
AUTHORITY_LOG = "first exit 0\nsecond exit 0\n"
# A Codex delegation. The delegate reads the mandatory file, then a craft
# skill, then writes, then tries an ending of its own.
CODEX_DELEGATE = {
    "prompt": DELEGATE["prompt"],
    "items": [
        ("command", "cat \"{SNAP}/skills/grove/SKILL.md\"", None, 0),
        ("command", "cat \"{SNAP}/skills/craft/SKILL.md\"", None, 0),
        ("patch", "{CWD}/check.sh", None, None),
        ("command", "ravel end-session trial-delegate-early", None, 0),
    ],
}


class Fixture:
    """A fake real home, a fake prepared directory and the fake programs."""

    def __init__(self, root: Path, external: dict[str, str] | None = None):
        self.root = root
        self.store = trial.CLAUDE.store
        self.owner = root / "owner"
        (self.owner / self.store).parent.mkdir(parents=True)
        if self.store.endswith("Keychains"):
            (self.owner / self.store).mkdir()
            (self.owner / self.store / "login.keychain-db").write_text("keychain\n")
        else:
            (self.owner / self.store).write_text("{}\n")
        (self.owner / ".claude/skills/real").mkdir(parents=True)
        (self.owner / ".claude/skills/real/SKILL.md").write_text("real\n")
        # The owner's real Codex home: the sign-in file and a real skill.
        (self.owner / ".codex/skills/real").mkdir(parents=True)
        (self.owner / ".codex/skills/real/SKILL.md").write_text("real\n")
        (self.owner / trial.CODEX.store).write_text("{}\n")

        self.prepared = root / "prepared"
        grove_bin = self.prepared / "grove/bin"
        grove_bin.mkdir(parents=True)
        self.grove_env = root / "grove.env"
        self._exec(grove_bin / "grove", FAKE_GROVE.format(python=sys.executable, env_file=str(self.grove_env)))
        self._exec(grove_bin / "grove-llm", "#!/bin/sh\nexit 0\n")
        front = root / "ravel/bin/ravel"
        front.parent.mkdir(parents=True)
        self._exec(front, FAKE_FRONT.format(python=sys.executable))
        # The checkout is a Git repository with two commits, so that a case can
        # copy the older revision of a skill as a stale native copy. It holds
        # the kind skill of `impl` and a marketplace, as a Grove commit does.
        # The marketplace pins the external pack's commit for `research`.
        checkout = root / "source"
        self.checkout = checkout
        (checkout / "plugins/grove/skills/craft").mkdir(parents=True)
        (checkout / KIND_SKILL).parent.mkdir(parents=True)
        (checkout / KIND_SKILL).write_text("kind\n")
        (checkout / SPINE_SKILL).parent.mkdir(parents=True)
        (checkout / SPINE_SKILL).write_text("spine\n")
        marketplace = [{"name": "grove", "source": "./plugins/grove"}]
        if external:
            marketplace.append({"name": "research", "source": {
                "source": "git-subdir", "url": external["repository"], "path": RESEARCH_PLUGIN,
                "sha": external["commit"]}})
        (checkout / ".claude-plugin").mkdir()
        (checkout / ".claude-plugin/marketplace.json").write_text(
            json.dumps({"name": "fake", "plugins": marketplace}, indent=2) + "\n")
        self.old_commit = self._commit(checkout, "craft old\n", init=True)
        head = self._commit(checkout, "craft\n")
        (self.prepared / "prepared.json").write_text(json.dumps({
            "grove": {"commit": "c" * 40, "prefix": str(grove_bin.parent),
                      "sha256": {name: trial.sha256_file(grove_bin / name) for name in ("grove", "grove-llm")}},
            "source": {"checkout": str(checkout), "commit": head},
            "ravel": {"front": str(front), "binRavelSha256": trial.sha256_file(front),
                      "buildIdentity": "i" * 64, "buildCommit": "b" * 12, "outputCommit": "o" * 12,
                      "line": "skill-directory-delivery"},
        }))

        self.behaviour = root / "claude.behaviour"
        self.launched = root / "claude.launched"
        self.claude = root / "claude-bin/claude"
        self.claude.parent.mkdir()
        self._exec(self.claude, FAKE_CLAUDE.format(
            python=sys.executable, behaviour=str(self.behaviour), launched=str(self.launched), store=self.store))
        self.codex_launched = root / "codex.launched"
        self.codex = root / "codex-bin/codex"
        self.codex.parent.mkdir()
        server = root / "codex-bin/app_server.py"
        server.write_text(FAKE_CODEX_SERVER)
        self._exec(self.codex, FAKE_CODEX.format(
            python=sys.executable, behaviour=str(self.behaviour), launched=str(self.codex_launched),
            server=str(server)))
        self.behave()

    @staticmethod
    def _commit(checkout: Path, text: str, init: bool = False) -> str:
        env = {**trial.git_env(os.environ), "GIT_AUTHOR_NAME": "t", "GIT_AUTHOR_EMAIL": "t@example.com",
               "GIT_COMMITTER_NAME": "t", "GIT_COMMITTER_EMAIL": "t@example.com"}
        if init:
            trial.run_checked(["git", "init", "--quiet", str(checkout)], env=env)
        (checkout / "plugins/grove/skills/craft/SKILL.md").write_text(text)
        trial.run_checked(["git", "add", "--all"], cwd=checkout, env=env)
        trial.run_checked(["git", "commit", "--quiet", "-m", text.strip()], cwd=checkout, env=env)
        return trial.run_checked(["git", "rev-parse", "HEAD"], cwd=checkout, env=env)

    def remove(self, *paths: str) -> str:
        """A commit that removes `paths` from the checkout, which the prepared
        directory then names as its commit. It returns the commit."""
        env = {**trial.git_env(os.environ), **GIT_IDENTITY}
        trial.run_checked(["git", "rm", "--quiet", "-r", *paths], cwd=self.checkout, env=env)
        trial.run_checked(["git", "commit", "--quiet", "-m", f"remove {' '.join(paths)}"], cwd=self.checkout, env=env)
        head = trial.run_checked(["git", "rev-parse", "HEAD"], cwd=self.checkout, env=env)
        prepared = json.loads((self.prepared / "prepared.json").read_text())
        prepared["source"]["commit"] = head
        (self.prepared / "prepared.json").write_text(json.dumps(prepared))
        return head

    def add(self, files: dict[str, str]) -> str:
        """A commit that adds `files` to the checkout, which the prepared
        directory then names as its commit. It returns the commit."""
        env = {**trial.git_env(os.environ), **GIT_IDENTITY}
        for path, text in files.items():
            (self.checkout / path).parent.mkdir(parents=True, exist_ok=True)
            (self.checkout / path).write_text(text)
        trial.run_checked(["git", "add", "--all"], cwd=self.checkout, env=env)
        trial.run_checked(["git", "commit", "--quiet", "-m", "add files"], cwd=self.checkout, env=env)
        head = trial.run_checked(["git", "rev-parse", "HEAD"], cwd=self.checkout, env=env)
        prepared = json.loads((self.prepared / "prepared.json").read_text())
        prepared["source"]["commit"] = head
        (self.prepared / "prepared.json").write_text(json.dumps(prepared))
        return head

    def keg(self, accepted: str | None = None) -> Path:
        """A candidate keg of the prepared commit, as `task owner:candidate-keg`
        writes it, which the prepared directory then names. Its source `grove`
        is a copy of the checkout's plugins/grove, and its accepted digest is
        the fake front's digest of an unchanged first source. `accepted`
        replaces that digest. It returns the candidate directory."""
        candidate = self.root / "candidate"
        shutil.rmtree(candidate, ignore_errors=True)
        source = candidate / "keg" / "share/ravel/skills/grove"
        shutil.copytree(self.checkout / "plugins/grove", source)
        prepared = json.loads((self.prepared / "prepared.json").read_text())
        commit = prepared["source"]["commit"]
        (candidate / "candidate.json").write_text(json.dumps({
            "schemaVersion": 1, "commit": commit, "keg": str(candidate / "keg"),
            "skillDirectory": str(candidate / "brew/share/ravel/skills"),
            "sources": [{"id": "grove", "resourceDigest": accepted or "ab" * 32,
                         "originPath": str(source), "roots": ["skills"]}],
        }))
        prepared["keg"] = trial.candidate_keg(candidate, commit)
        (self.prepared / "prepared.json").write_text(json.dumps(prepared))
        return candidate

    def check(self, *cases: str, commit: str | None = None, keg: Path | None = None) -> tuple[int, str]:
        """Check cases against a commit of the checkout without a launch;
        return the status and everything it printed."""
        commit = commit or json.loads((self.prepared / "prepared.json").read_text())["source"]["commit"]
        argv = ["check", "--repository", str(self.checkout), "--commit", commit]
        if keg:
            argv += ["--keg", str(keg)]
        for case in cases:
            argv += ["--case", case]
        output = io.StringIO()
        with contextlib.redirect_stdout(output), contextlib.redirect_stderr(output):
            status = trial.main(argv, self.environ())
        return status, output.getvalue()

    @staticmethod
    def _exec(path: Path, text: str) -> None:
        path.write_text(text)
        path.chmod(0o755)

    def behave(self, **changes) -> None:
        self.behaviour.write_text(json.dumps({"tools": TOOLS, "codexItems": CODEX_ITEMS, **changes}))

    def environ(self, **extra) -> dict[str, str]:
        return {"HOME": str(self.owner), "PATH": os.environ["PATH"], "USER": "owner",
                "HARNESS_TRIAL_PROGRAM": str(self.claude), **LIVE, **extra}

    def run(self, case: str = "smoke", args: tuple[str, ...] = (), **extra) -> tuple[int, str]:
        """Run a case; return the status and everything it printed."""
        output = io.StringIO()
        with contextlib.redirect_stdout(output), contextlib.redirect_stderr(output):
            status = trial.main(["run", "--prepared", str(self.prepared), "--case", case, *args],
                                self.environ(**extra))
        return status, output.getvalue()

    def run_codex(self, case: str = "smoke", **extra) -> tuple[int, str]:
        """Run a case in the fake Codex; return the status and everything it printed."""
        output = io.StringIO()
        environ = self.environ(HARNESS_TRIAL_PROGRAM=str(self.codex), **extra)
        with contextlib.redirect_stdout(output), contextlib.redirect_stderr(output):
            status = trial.main(["run", "--prepared", str(self.prepared), "--case", case, "--harness", "codex"],
                                environ)
        return status, output.getvalue()

    def launched_any(self) -> bool:
        return self.launched.exists() or self.codex_launched.exists() or self.grove_env.exists()

    def record(self) -> dict:
        runs = sorted((self.prepared / "runs").iterdir())
        return json.loads((runs[-1] / "record.json").read_text())


KIND_SKILL = "plugins/grove/skills/grove-impl/SKILL.md"
SPINE_SKILL = "plugins/grove/skills/grove/SKILL.md"
RESEARCH_PLUGIN = "plugins/research"
RESEARCH_SKILL = f"{RESEARCH_PLUGIN}/skills/grove-research-a/SKILL.md"
GIT_IDENTITY = {"GIT_AUTHOR_NAME": "t", "GIT_AUTHOR_EMAIL": "t@example.com",
                "GIT_COMMITTER_NAME": "t", "GIT_COMMITTER_EMAIL": "t@example.com"}


def make_external(root: Path) -> dict[str, str]:
    """An external pack repository with one commit and an annotated tag of
    it. The tag's object ID is a full hash whose checkout has another HEAD."""
    repo = root / "research-repo"
    env = {**trial.git_env(os.environ), **GIT_IDENTITY}
    trial.run_checked(["git", "init", "--quiet", str(repo)], env=env)
    (repo / RESEARCH_SKILL).parent.mkdir(parents=True)
    (repo / RESEARCH_SKILL).write_text("research\n")
    # The plugin depends on `grove`, as a pack plugin does, so that a native
    # case can take a core kind skill from the dependency.
    (repo / RESEARCH_PLUGIN / ".claude-plugin").mkdir()
    (repo / RESEARCH_PLUGIN / ".claude-plugin/plugin.json").write_text(
        json.dumps({"name": "research", "dependencies": ["grove"]}) + "\n")
    (repo / "README.md").write_text("pack\n")
    trial.run_checked(["git", "add", "--all"], cwd=repo, env=env)
    trial.run_checked(["git", "commit", "--quiet", "-m", "pack"], cwd=repo, env=env)
    trial.run_checked(["git", "tag", "--annotate", "-m", "release", "v1"], cwd=repo, env=env)
    return {
        "repository": str(repo),
        "commit": trial.run_checked(["git", "rev-parse", "HEAD"], cwd=repo, env=env),
        "tag": trial.run_checked(["git", "rev-parse", "v1"], cwd=repo, env=env),
        "tree": trial.run_checked(["git", "rev-parse", f"HEAD:{RESEARCH_PLUGIN}"], cwd=repo, env=env),
    }


def fake_digest(directory: Path) -> str:
    """The fake front's resourceDigest of a source directory."""
    digest = hashlib.sha256()
    for path in sorted(p for p in directory.rglob("*") if p.is_file()):
        digest.update(str(path.relative_to(directory)).encode() + b"\0" + path.read_bytes())
    return digest.hexdigest()


def plant_cases(root: Path, external: dict[str, str]) -> Path:
    """The smoke case, and two cases of the same shape for the offline
    resolution and the standalone mode."""
    cases = root / "cases"
    shutil.copytree(trial.CASES / "smoke", cases / "smoke")
    smoke = json.loads((cases / "smoke" / "case.json").read_text())
    # A case with an external source, and the cases that refuse it: a moved
    # checkout, an unknown commit and a commit whose checkout has another HEAD.
    for name, commit, extra in (
        ("external", external["commit"], {}),
        ("external-missing", external["commit"], {"moveAway": "research", "expectRefusal": "skill_source_missing"}),
        ("external-root-missing", external["commit"],
         {"moveAway": "research", "moveRoot": "skills", "expectRefusal": "skill_source_missing"}),
        ("external-unknown", "0123456789abcdef0123456789abcdef01234567", {}),
        ("external-tag", external["tag"], {}),
    ):
        shutil.copytree(cases / "smoke", cases / name)
        research = {"id": "research", "plugin": RESEARCH_PLUGIN, "roots": ["skills"],
                    "external": {"repository": external["repository"], "commit": commit}}
        (cases / name / "case.json").write_text(json.dumps({
            **smoke, "name": name, "sources": [*smoke["sources"], research], **extra,
        }))
    resolve = cases / "resolve"
    shutil.copytree(cases / "smoke", resolve)
    (resolve / "case.json").write_text(json.dumps({**smoke, "name": "resolve", "resolveOffline": True}))
    standalone = cases / "standalone"
    standalone.mkdir()
    (standalone / "prompt.md").write_text("Write stamp.sh.\n")
    (standalone / "case.json").write_text(json.dumps({
        **smoke, "name": "standalone", "mode": "standalone",
        "standalone": {"kind": "impl", "prompt": "prompt.md", "outputs": ["stamp.sh"]},
        "mandatory": [{"source": "grove", "path": "skills/grove/SKILL.md"}],
    }))
    # A case with a native copy, a sandbox deny rule, a planted file and a task
    # that names the planted path.
    explicit = cases / "explicit"
    shutil.copytree(cases / "smoke", explicit)
    (explicit / "tree" / "02-impl--path-k2.md").write_text("Read `{{TRIAL}}/guidance/craft/SKILL.md`.\n")
    (explicit / "case.json").write_text(json.dumps({
        **smoke, "name": "explicit",
        "native": [{"name": "craft", "from": "plugins/grove/skills/craft"}],
        "claudeSettings": {"permissions": {"deny": ["Skill(craft)"]}},
        "plant": [{"from": "plugins/grove/skills/craft/SKILL.md", "to": "guidance/craft/SKILL.md"}],
    }))
    # A native plugin route: the marketplace of the prepared checkout installs
    # the plugin `grove`, and no Ravel settings exist.
    plugins = cases / "plugins"
    shutil.copytree(cases / "smoke", plugins)
    (plugins / "case.json").write_text(json.dumps({
        **{k: v for k, v in smoke.items() if k not in ("sources", "mandatory")},
        "name": "plugins", "route": "native-2.0", "nativePlugins": ["grove@fake"],
    }))
    # The supplier check: a declared skill, a kind declared absent, a kind of
    # an external source and a kind of a plugin that the marketplace pins.
    for name, extra in (
        ("declared", {"requires": [{"source": "grove", "path": "skills/craft/SKILL.md"}]}),
        ("kind-absent", {"absent": ["grove-impl"]}),
        ("external-kind", {"kinds": ["research-a"], "sources": [*smoke["sources"], {
            "id": "research", "plugin": RESEARCH_PLUGIN, "roots": ["skills"],
            "external": {"repository": external["repository"], "commit": external["commit"]}}]}),
    ):
        shutil.copytree(cases / "smoke", cases / name)
        (cases / name / "case.json").write_text(json.dumps({**smoke, "name": name, **extra}))
    shutil.copytree(cases / "smoke", cases / "plugin-kind")
    (cases / "plugin-kind" / "case.json").write_text(json.dumps({
        **{k: v for k, v in smoke.items() if k not in ("sources", "mandatory")},
        "name": "plugin-kind", "route": "native-2.0", "kinds": ["research-a"], "nativePlugins": ["research@fake"],
    }))
    # A native plugin whose dependency supplies the core kind, and a skill of
    # that plugin that the task relies on, present or absent.
    for name, path in (("plugin-dependency", "skills/grove-research-a/SKILL.md"),
                       ("plugin-requires-missing", "craft/absent/SKILL.md")):
        shutil.copytree(cases / "smoke", cases / name)
        (cases / name / "case.json").write_text(json.dumps({
            **{k: v for k, v in smoke.items() if k not in ("sources", "mandatory")},
            "name": name, "route": "native-2.0", "nativePlugins": ["research@fake"],
            "requires": [{"plugin": "research@fake", "path": path}],
        }))
    # A stale native copy: the skill as the commit before the checkout's head held it.
    stale = cases / "stale"
    shutil.copytree(cases / "smoke", stale)
    (stale / "case.json").write_text(json.dumps({
        **smoke, "name": "stale",
        "native": [{"name": "craft", "from": "plugins/grove/skills/craft", "commit": "HEAD~1"}],
    }))
    # A source moved away before the driver, which expects Ravel's refusal.
    missing = cases / "missing"
    shutil.copytree(cases / "smoke", missing)
    (missing / "case.json").write_text(json.dumps({
        **smoke, "name": "missing", "moveAway": "grove", "expectRefusal": "skill_source_missing",
    }))
    # A continuation: a source commit and a settings change between a first
    # turn and a resumed turn, then a fresh launch on a new leaf.
    continuation = cases / "continuation"
    shutil.copytree(cases / "smoke", continuation)
    (continuation / "answer.md").write_text("Yes, add the label.\n")
    (continuation / "fresh-tree").mkdir()
    (continuation / "fresh-tree" / "02-impl--fresh-k2.md").write_text("# fresh-k2\n")
    (continuation / "case.json").write_text(json.dumps({
        **smoke, "name": "continuation",
        "mandatory": [{"source": "grove", "path": "skills/grove/SKILL.md"}],
        "continuation": {
            "marker": MARKER, "answer": "answer.md", "freshTree": "fresh-tree",
            "edits": [{"path": CRAFT, "insert": f"Rule {MARKER}."}],
            "mandatory": [{"source": "grove", "path": "skills/grove-impl/SKILL.md"},
                          {"source": "grove", "path": "skills/grove/SKILL.md"}],
        },
    }))
    # A case whose only source comes from the candidate keg of the prepared
    # commit, at its directory in the keg.
    keg = cases / "keg"
    shutil.copytree(cases / "smoke", keg)
    (keg / "case.json").write_text(json.dumps({
        **smoke, "name": "keg",
        "sources": [{"id": "grove", "keg": "grove", "plugin": "share/ravel/skills/grove", "roots": ["skills"]}],
    }))
    delegate = cases / "delegate"
    shutil.copytree(cases / "smoke", delegate)
    (delegate / "case.json").write_text(json.dumps({
        **smoke, "name": "delegate", "expectDelegate": True, "collect": ["authority.log"],
        "mandatory": [{"source": "grove", "path": "skills/grove/SKILL.md"}],
    }))
    for mode, instructions in (("path", "Keep the obligations word for word, and write no absolute path."),
                               ("preamble", "Summarise only the Grove mandate and what followed it.")):
        compaction = cases / f"compaction-{mode}"
        shutil.copytree(cases / "smoke", compaction)
        (compaction / "answer.md").write_text("Yes, add the label.\n")
        (compaction / "case.json").write_text(json.dumps({
            **smoke, "name": f"compaction-{mode}",
            "mandatory": [{"source": "grove", "path": "skills/grove/SKILL.md"}],
            "compaction": {"mode": mode, "instructions": instructions, "answer": "answer.md"},
        }))
    return cases


class HarnessTrialTest(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        root = Path(os.path.realpath(self.temp.name))
        self.external = make_external(root)
        patcher = mock.patch.object(trial, "CASES", plant_cases(root, self.external))
        patcher.start()
        self.addCleanup(patcher.stop)
        self.fixture = Fixture(root / "main", self.external)

    def tearDown(self):
        self.temp.cleanup()

    def fresh(self) -> Fixture:
        return Fixture(Path(tempfile.mkdtemp(dir=self.temp.name)), self.external)

    def broken(self, check, name, replacement):
        """The check fails once against a runner whose `name` is broken. A
        dotted name, such as `CODEX.resume_args`, breaks one method of one
        harness adapter."""
        *owners, attribute = name.split(".")
        owner = trial
        for part in owners:
            owner = getattr(owner, part)
        with mock.patch.object(owner, attribute, replacement):
            with self.assertRaises(AssertionError, msg=f"the check passed with a broken {name}"):
                check(self.fresh())

    # --- The refusals ---------------------------------------------------------

    def check_refusals(self, fixture: Fixture):
        for name in trial.LOCATION_NAMES:
            for value in (str(fixture.owner / ".claude"), ""):
                status, output = fixture.run(**{name: value})
                self.assertEqual(status, 1, f"{name}={value!r} did not refuse: {output}")
                self.assertIn(f"the environment holds {name}", output)
                self.assertFalse(fixture.launched_any(), f"{name}: a harness or driver launched before the refusal")

    def test_each_location_variable_refuses_before_any_launch(self):
        self.check_refusals(self.fixture)
        self.broken(self.check_refusals, "CLAUDE.refuse", lambda environ, owner_home: None)

    def test_a_foreign_configuration_directory_refuses(self):
        self.fixture.behave(configDirectory=str(self.fixture.owner / ".claude"))
        status, output = self.fixture.run()
        self.assertEqual(status, 1)
        self.assertIn("reports the configuration directory", output)

    # --- The allow-list and the authority control ----------------------------

    def check_allow_list(self, fixture: Fixture):
        status, output = fixture.run(CLAUDE_CODE_PLANTED="planted")
        self.assertEqual(status, 0, output)
        driver = json.loads(fixture.grove_env.read_text())
        self.assertLessEqual(set(driver), ALLOWED, "the driver received a name outside the allow-list")
        prepared = json.loads((fixture.prepared / "prepared.json").read_text())
        self.assertEqual(driver["PATH"].split(":")[0], str(Path(prepared["ravel"]["front"]).parent),
                         "the 2.0 front is not first on PATH")
        record = fixture.record()
        self.assertEqual(record["launches"][0]["control"], [])
        self.assertNotIn("CLAUDE_CODE_PLANTED", record["launches"][0]["envNames"])

    def test_the_driver_receives_only_the_allow_list(self):
        self.check_allow_list(self.fixture)

        def leaky(environ, home, tmp, path_dirs):
            return {**environ, "HOME": str(home), "TMPDIR": str(tmp), "PATH": ":".join(path_dirs)}

        self.broken(self.check_allow_list, "allowed_env", leaky)

    def check_control_fires(self, fixture: Fixture):
        """A runner that plants a live name in the driver's environment: the
        name reaches the child, and the control fails the run."""
        real = trial.allowed_env

        def planting(environ, home, tmp, path_dirs):
            return {**real(environ, home, tmp, path_dirs), "CLAUDE_CODE_PLANTED": "planted"}

        with mock.patch.object(trial, "allowed_env", planting):
            status, output = fixture.run(CLAUDE_CODE_PLANTED="planted")
        self.assertEqual(status, 1, output)
        self.assertIn("CLAUDE_CODE_PLANTED reached the trial child", output)

    def test_a_planted_name_fails_the_control_once(self):
        self.check_control_fires(self.fixture)
        self.broken(self.check_control_fires, "child_control", lambda child, live: [])

    def test_the_control_compares_values_not_only_names(self):
        live = {"RAVEL_EXIT_FILE": "/live/exit", "GROVE_LAUNCH_DIR": "/live/launch"}
        self.assertEqual(trial.child_control({"RAVEL_EXIT_FILE": "/trial/exit", "GROVE_LAUNCH_DIR": "/t"}, live), [])
        self.assertEqual(trial.child_control({"RAVEL_EXIT_FILE": "/live/exit"}, live),
                         ["RAVEL_EXIT_FILE reached the trial child with the live session's value"])
        self.assertEqual(trial.child_control({"CLAUDECODE": "1"}, {}), ["CLAUDECODE reached the trial child"])

    # --- The link list --------------------------------------------------------

    def check_links(self, fixture: Fixture):
        status, output = fixture.run()
        self.assertEqual(status, 0, output)
        links = [line for line in output.splitlines()
                 if line.startswith("harness-trial: link ") and not line.startswith("harness-trial: link after")]
        self.assertEqual(links, [f"harness-trial: link {fixture.store} -> {fixture.owner / fixture.store}"])
        self.assertEqual(len([line for line in output.splitlines() if "link after" in line]), 1)
        for change in ({"replaceLink": True}, {"extraLink": True}):
            other = self.fresh()
            other.behave(**change)
            status, output = other.run()
            self.assertEqual(status, 1, f"{change}: the run passed: {output}")
            self.assertIn("the sandbox home holds the links", output)
            self.assertTrue((other.owner / other.store).exists(), "the real store changed")

    def test_the_link_list_holds_only_the_sign_in_store_before_and_after(self):
        self.check_links(self.fixture)
        self.broken(self.check_links, "check_links", lambda links, store, owner, when: None)

    def test_no_run_changes_the_real_home_and_the_record_keeps_no_link(self):
        before = sorted(str(p) for p in self.fixture.owner.rglob("*"))
        status, output = self.fixture.run()
        self.assertEqual(status, 0, output)
        self.assertEqual(sorted(str(p) for p in self.fixture.owner.rglob("*")), before)
        trial_home = Path(self.fixture.record()["trial"]) / "home"
        self.assertEqual(trial.list_links(trial_home), [], "the trial directory keeps the sign-in link")

    # --- Event extraction -----------------------------------------------------

    def check_events(self, fixture: Fixture):
        status, output = fixture.run()
        self.assertEqual(status, 0, output)
        launch = fixture.record()["launches"][0]
        main = [t for t in launch["transcripts"] if t["main"]]
        self.assertEqual(len(main), 1)
        events = [(e["tool"], e["class"], e.get("skill"), e["error"]) for e in main[0]["events"]]
        self.assertEqual(events, EXPECTED)
        self.assertEqual([e["seq"] for e in main[0]["events"]], list(range(1, len(EXPECTED) + 1)))
        self.assertEqual(fixture.record()["ending"]["endingCommands"], ["grove-continue"])
        by_tool = {e["tool"]: e for e in main[0]["events"]}
        self.assertEqual(by_tool["text"]["target"], "The native loader has no match, so I search the directories.")
        self.assertEqual(by_tool["Skill"]["resultText"], "Skill result")
        startup = main[0]["startup"]
        self.assertEqual(startup["init"]["permissionMode"], "bypassPermissions")
        self.assertNotIn("session_id", startup["init"])
        self.assertNotIn("secret-org", json.dumps(launch), "account data reached the record")
        identity = launch["descriptorIdentity"]
        self.assertEqual(identity["directory"], identity["sha256"])
        self.assertEqual(launch["sources"][0]["resourceDigest"], "ab" * 32)
        # Each snapshot read names its source, and a read outside every source names none.
        sources = {(e["class"], e.get("skill")): e.get("source") for e in main[0]["events"]}
        self.assertEqual(sources[("skill", "craft")], "grove")
        self.assertEqual(sources[("kind-skill", "grove-impl")], "grove")
        self.assertIsNone(sources[("native", "old")])
        markdown = (Path(fixture.record()["trial"]) / "record.md").read_text()
        self.assertIn("| 5 | Read | skill | craft | grove |", markdown)

    def test_extraction_orders_and_classifies_every_tool_call(self):
        self.check_events(self.fixture)
        # A runner that names no source loses which copy a read opened.
        with (mock.patch.object(trial.Classifier, "source_of", lambda self, raw: None),
              self.assertRaises(AssertionError, msg="the check passed with a broken source_of")):
            self.check_events(self.fresh())
        real = trial.extract_events
        self.broken(self.check_events, "extract_events", lambda entries, c: list(reversed(real(entries, c))))
        self.broken(self.check_events, "extract_events",
                    lambda entries, c: [e for e in real(entries, c) if e["tool"] != "Bash"])
        # A runner that ignores `cd` resolves relative reads against the wrong directory.
        self.broken(self.check_events, "bash_paths", lambda command, cwd: trial.PATH_TOKEN.findall(command))
        # A runner that expands no assigned variable misses the read in it.
        self.broken(self.check_events, "expand", lambda token, variables: None if "$" in token else token)
        # A runner that drops the result text loses what the native loader said.
        self.broken(self.check_events, "result_text", lambda content: "")

    def check_reextract(self, fixture: Fixture):
        status, output = fixture.run()
        self.assertEqual(status, 0, output)
        trial_dir = Path(fixture.record()["trial"])
        record = json.loads((trial_dir / "record.json").read_text())
        record["launches"][0]["transcripts"][0]["events"] = []
        (trial_dir / "record.json").write_text(json.dumps(record))
        with contextlib.redirect_stdout(io.StringIO()):
            self.assertEqual(trial.main(["reextract", "--trial", str(trial_dir)], fixture.environ()), 0)
        again = json.loads((trial_dir / "record.json").read_text())
        events = [(e["tool"], e["class"], e.get("skill"), e["error"]) for e in again["launches"][0]["transcripts"][0]["events"]]
        self.assertEqual(events, EXPECTED)
        self.assertEqual(again["launches"][0]["control"], record["launches"][0]["control"])
        self.assertIn("| Events extracted again |", (trial_dir / "record.md").read_text())

    def test_a_run_can_be_extracted_again_from_its_transcripts(self):
        self.check_reextract(self.fixture)
        # A command that rewrites nothing leaves the emptied events empty.
        self.broken(self.check_reextract, "reextract", lambda trial_dir: {})

    def test_the_record_names_builds_harness_and_ending(self):
        status, output = self.fixture.run()
        self.assertEqual(status, 0, output)
        record = self.fixture.record()
        self.assertEqual(record["harness"]["version"], "9.9.9 (Claude Code)")
        self.assertEqual(record["harness"]["model"], "claude-opus-5-5")
        self.assertEqual(record["ending"]["launches"], 1)
        self.assertFalse(record["ending"]["teardownRecorded"])
        self.assertEqual(record["ending"]["treeBefore"], record["ending"]["treeAfter"])
        self.assertEqual(record["skills"], {"nativeCopies": [], "residue": [], "installed": [],
                                            "claudeSettings": None, "planted": [], "pluginCopies": [],
                                        "plugins": None})
        markdown = (Path(record["trial"]) / "record.md").read_text()
        for needle in ("| Route | directory-2.0 |", "Native startup text:", "| 4 | Bash | kind-skill | grove-impl |",
                       "Harness launches: 1."):
            self.assertIn(needle, markdown)

    # --- Native copies, sandbox settings and planted files ---------------------

    def check_explicit(self, fixture: Fixture):
        owner_before = sorted(str(p) for p in fixture.owner.rglob("*"))
        status, output = fixture.run("explicit")
        self.assertEqual(status, 0, output)
        record = fixture.record()
        trial_dir = Path(record["trial"])
        home = trial_dir / "home"
        self.assertEqual(record["skills"]["nativeCopies"], [".claude/skills/craft/SKILL.md"])
        self.assertEqual(record["skills"]["installed"][0]["skillSha256"],
                         trial.sha256_bytes(b"craft\n"))
        self.assertTrue((home / ".claude/settings.json").is_file(), "the case's settings were not written")
        self.assertEqual(json.loads((home / ".claude/settings.json").read_text()),
                         {"permissions": {"deny": ["Skill(craft)"]}})
        self.assertTrue((trial_dir / "guidance/craft/SKILL.md").is_file(), "the case's file was not planted")
        self.assertEqual((trial_dir / "guidance/craft/SKILL.md").read_text(), "craft\n")
        task = (trial_dir / "repo/.grove/02-impl--path-k2.md").read_text()
        self.assertEqual(task, f"Read `{trial_dir}/guidance/craft/SKILL.md`.\n")
        self.assertEqual(sorted(str(p) for p in fixture.owner.rglob("*")), owner_before,
                         "a native copy or a setting reached the owner's home")
        markdown = (trial_dir / "record.md").read_text()
        for needle in ("| Installed native copy | `.claude/skills/craft` from", "| Planted file | `$TRIAL/guidance/craft/SKILL.md`",
                       "| Sandbox `.claude/settings.json` |"):
            self.assertIn(needle, markdown)

    def test_a_case_installs_native_copies_settings_and_planted_files(self):
        self.check_explicit(self.fixture)
        self.broken(self.check_explicit, "install_native", lambda case, home, checkout: [])
        self.broken(self.check_explicit, "CLAUDE.write_settings", lambda case, home: None)
        self.broken(self.check_explicit, "plant_files", lambda case, trial_dir, checkout: [])
        real_plant = trial.plant_repo

        def unsubstituted(repo, tree, env):
            with mock.patch.object(trial, "TRIAL_TOKEN", "{{NOT-A-TOKEN}}"):
                return real_plant(repo, tree, env)

        self.broken(self.check_explicit, "plant_repo", unsubstituted)

    def check_stale(self, fixture: Fixture):
        status, output = fixture.run("stale")
        self.assertEqual(status, 0, output)
        record = fixture.record()
        installed = record["skills"]["installed"][0]
        self.assertEqual(installed.get("commit"), fixture.old_commit)
        self.assertEqual(installed["skillSha256"], trial.sha256_bytes(b"craft old\n"))
        self.assertEqual(installed["currentSha256"], trial.sha256_bytes(b"craft\n"))
        home = Path(record["trial"]) / "home"
        self.assertEqual((home / ".claude/skills/craft/SKILL.md").read_text(), "craft old\n")
        self.assertIn(f"at commit `{fixture.old_commit[:12]}`", (Path(record["trial"]) / "record.md").read_text())

    def test_a_native_copy_can_come_from_an_older_commit(self):
        self.check_stale(self.fixture)
        real = trial.install_native

        def current_only(case, home, checkout):
            plain = {**case, "native": [{k: v for k, v in item.items() if k != "commit"} for item in case["native"]]}
            return real(plain, home, checkout)

        self.broken(self.check_stale, "install_native", current_only)

    def test_a_planted_file_outside_the_guidance_directory_refuses(self):
        case = {"name": "x", "plant": [{"from": "plugins/grove/skills/craft/SKILL.md", "to": "../escape"}]}
        with self.assertRaises(trial.TrialError):
            trial.plant_files(case, self.fixture.root, self.fixture.root / "source")

    def check_stray_native(self, fixture: Fixture):
        fixture.behave(strayNative=True)
        status, output = fixture.run()
        self.assertEqual(status, 1, output)
        self.assertIn("holds the native copies ['.claude/skills/stray/SKILL.md']", output)

    def test_an_undeclared_native_copy_fails_the_run(self):
        self.check_stray_native(self.fixture)
        self.broken(self.check_stray_native, "native_failures", lambda native, expected: [])

    def check_plugins(self, fixture: Fixture):
        fixture.behave(pluginDependencies={"grove": ["extra"]})
        status, output = fixture.run("plugins")
        self.assertEqual(status, 0, output)
        record = fixture.record()
        plugins = record["skills"]["plugins"]
        self.assertIsNotNone(plugins, "the record names no installed plugin")
        source = record["sources"]["checkout"]
        self.assertEqual(plugins["marketplace"]["path"], source)
        self.assertEqual(plugins["marketplace"]["commit"], record["sources"]["commit"])
        self.assertEqual(plugins["marketplace"]["name"], "fake")
        self.assertTrue(plugins["marketplace"]["checkoutClean"])
        self.assertEqual([i["id"] for i in plugins["installs"]], ["grove@fake"])
        installed = {p["id"]: p for p in plugins["installed"]}
        self.assertEqual(sorted(installed), ["extra@fake", "grove@fake"])
        self.assertFalse(installed["grove@fake"]["dependency"])
        self.assertTrue(installed["extra@fake"]["dependency"])
        self.assertEqual(installed["grove@fake"]["gitCommitSha"], "a" * 40)
        self.assertEqual(installed["grove@fake"]["skills"],
                         [{"root": "skills", "name": "craft", "sha256": trial.sha256_bytes(b"craft\n")},
                          {"root": "skills", "name": "grove", "sha256": trial.sha256_bytes(b"spine\n")},
                          {"root": "skills", "name": "grove-impl", "sha256": trial.sha256_bytes(b"kind\n")}])
        self.assertEqual(record["skills"]["pluginCopies"],
                         [".claude/plugins/cache/fake/grove/v1/skills/craft/SKILL.md",
                          ".claude/plugins/cache/fake/grove/v1/skills/grove-impl/SKILL.md",
                          ".claude/plugins/cache/fake/grove/v1/skills/grove/SKILL.md"])
        self.assertEqual(record["skills"]["nativeCopies"], [])
        self.assertEqual(record["sources"]["settings"], {})
        self.assertIn("plugin", fixture.launched.read_text().split())
        text = (Path(record["trial"]) / "record.md").read_text()
        self.assertIn("| Installed plugin `extra@fake` | version `v1`", text)
        self.assertIn("installed as a dependency", text)

    def test_a_case_installs_native_plugins_from_the_checkout_marketplace(self):
        self.check_plugins(self.fixture)
        self.broken(self.check_plugins, "CLAUDE.install_plugins",
                    lambda case, program, env, tmp, home, checkout, commit: None)

        def no_dependencies(home, installs):
            return [{**plugin, "dependency": False} for plugin in real_installed(home, installs)]

        real_installed = trial.installed_plugins
        self.broken(self.check_plugins, "installed_plugins", no_dependencies)

    def check_craft_plugin(self, fixture: Fixture):
        """A plugin whose manifest adds `craft/` to its skills accounts for
        each copy in `skills/` and in `craft/`."""
        fixture.add({"plugins/crafty/.claude-plugin/plugin.json": json.dumps({"name": "crafty",
                                                                             "skills": ["./craft/"]}),
                     "plugins/crafty/skills/kindish/SKILL.md": "kindish\n",
                     "plugins/crafty/craft/note/SKILL.md": "note\n"})
        fixture.behave(pluginDependencies={"grove": ["crafty"]})
        status, output = fixture.run("plugins")
        self.assertEqual(status, 0, output)
        record = fixture.record()
        installed = {p["id"]: p for p in record["skills"]["plugins"]["installed"]}
        self.assertEqual(installed["crafty@fake"]["skills"],
                         [{"root": "craft", "name": "note", "sha256": trial.sha256_bytes(b"note\n")},
                          {"root": "skills", "name": "kindish", "sha256": trial.sha256_bytes(b"kindish\n")}])
        self.assertIn(".claude/plugins/cache/fake/crafty/v1/craft/note/SKILL.md", record["skills"]["pluginCopies"])
        text = (Path(record["trial"]) / "record.md").read_text()
        self.assertIn("skills `craft/note`", text)

    def test_a_plugin_with_a_craft_root_accounts_for_its_copies(self):
        self.check_craft_plugin(self.fixture)
        self.broken(self.check_craft_plugin, "plugin_skill_roots", lambda install: ["skills"])

    def check_plugin_refusal(self, fixture: Fixture):
        fixture.behave(pluginFails=["grove"])
        status, output = fixture.run("plugins")
        self.assertEqual(status, 1, output)
        self.assertIn("plugin install grove@fake", output)
        self.assertFalse(fixture.grove_env.exists(), "the driver ran after a failed installation")

    def test_a_failed_plugin_installation_refuses_before_the_driver(self):
        self.check_plugin_refusal(self.fixture)

    def test_a_native_plugin_names_its_marketplace(self):
        case = json.loads((trial.CASES / "plugins" / "case.json").read_text())
        for value in (["grove"], "grove@fake", [""], ["grove@fake@x"]):
            (trial.CASES / "plugins" / "case.json").write_text(json.dumps({**case, "nativePlugins": value}))
            with self.assertRaises(trial.TrialError, msg=f"{value!r} loaded") as caught:
                trial.load_case("plugins")
            self.assertIn("<plugin>@<marketplace>", str(caught.exception))

    def check_stray_plugin(self, fixture: Fixture):
        fixture.behave(strayPlugin=True)
        status, output = fixture.run("plugins")
        self.assertEqual(status, 1, output)
        self.assertIn("holds the plugin copies", output)
        self.assertIn(".claude/plugins/cache/other/stray/v1/skills/stray/SKILL.md", output)

    def test_a_plugin_copy_outside_the_installed_plugins_fails_the_run(self):
        self.check_stray_plugin(self.fixture)
        self.check_stray_plugin_without_plugins(self.fixture)
        self.broken(self.check_stray_plugin, "plugin_failures", lambda found, plugins: [])

    def check_stray_plugin_without_plugins(self, fixture: Fixture):
        fixture.behave(strayPlugin=True)
        status, output = fixture.run()
        self.assertEqual(status, 1, output)
        self.assertIn("holds the plugin copies", output)

    def check_unstarted(self, fixture: Fixture):
        fixture.behave(noInit=True)
        status, output = fixture.run()
        self.assertEqual(status, 1, output)
        self.assertIn("launch 0: the harness wrote no startup event", output)

    def test_a_harness_that_starts_no_session_fails_the_run(self):
        self.check_unstarted(self.fixture)
        self.broken(self.check_unstarted, "started", lambda launch: True)

    # --- A source moved away -----------------------------------------------------

    def check_source_missing(self, fixture: Fixture):
        status, output = fixture.run("missing")
        self.assertEqual(status, 0, output)
        record = fixture.record()
        self.assertEqual(record["launches"], [])
        refusal = record["refusal"]
        self.assertEqual(refusal["moved"]["exit"], 3)
        self.assertIn("skill_source_missing", refusal["moved"]["firstLine"])
        self.assertEqual(refusal["restored"]["status"], "resolved")
        self.assertTrue(Path(refusal["clone"]["checkout"]).is_dir(), "the clone was not restored")
        self.assertEqual(refusal["clone"]["commit"], json.loads(
            (fixture.prepared / "prepared.json").read_text())["source"]["commit"])
        self.assertEqual(record["sources"]["settings"]["skills"]["sources"][0]["path"],
                         str(Path(refusal["clone"]["checkout"]) / "plugins/grove"))
        # The harness answered only the version and sign-in questions.
        self.assertEqual(fixture.launched.read_text().split(), ["--version", "auth"])
        self.assertTrue(any("ravel: refused (skill_source_missing" in line for line in record["ending"]["driverLines"]))
        self.assertIn("Expected refusal:", (Path(record["trial"]) / "record.md").read_text())

    def test_a_moved_source_refuses_before_any_launch(self):
        self.check_source_missing(self.fixture)
        self.broken(self.check_source_missing, "move_away", lambda moved: None)
        self.broken(self.check_source_missing, "restore", lambda moved: None)
        # A runner whose settings ignore the clone moves a directory that no launch reads.
        real_settings = trial.settings_for
        self.broken(self.check_source_missing, "settings_for",
                    lambda case, checkout, clones=None: real_settings(case, checkout))

    def check_refusal_needs_no_launch(self, fixture: Fixture):
        with mock.patch.object(trial, "move_away", lambda moved: None):
            status, output = fixture.run("missing")
        self.assertEqual(status, 1, output)
        self.assertIn("the driver launched a harness", output)
        self.assertIn("the driver did not report Ravel's refusal", output)

    def test_a_refusal_case_fails_when_the_harness_launches(self):
        self.check_refusal_needs_no_launch(self.fixture)
        self.broken(self.check_refusal_needs_no_launch, "refusal_failures", lambda *args: [])

    def test_move_away_needs_an_expected_refusal(self):
        case = json.loads((trial.CASES / "missing" / "case.json").read_text())
        (trial.CASES / "half").mkdir()
        shutil.copytree(trial.CASES / "smoke" / "tree", trial.CASES / "half" / "tree")
        (trial.CASES / "half" / "case.json").write_text(json.dumps({**case, "expectRefusal": None}))
        with self.assertRaises(trial.TrialError):
            trial.load_case("half")

    # --- The supplier check -----------------------------------------------------

    def assert_refused(self, fixture: Fixture, output: str, case: str, path: str, supplier: str, revision: str):
        """The refusal names the case, the path, the supplier and the
        revision, and says that the case measures another build."""
        self.assertIn(f"case {case}: {supplier} {revision} holds no {path}", output)
        self.assertIn("The case measures a build whose supplier held this path", output)
        self.assertFalse(fixture.launched_any(), f"{case}: a harness or driver launched before the refusal")

    def check_supplied(self, fixture: Fixture):
        """A case against a commit that holds its local paths passes the
        check, in a run and without a launch."""
        status, output = fixture.check("smoke", "explicit", "declared", "stale", "missing", "continuation")
        self.assertEqual(status, 0, output)
        self.assertFalse(fixture.launched_any(), "the check launched a harness or the driver")
        for case in ("smoke", "explicit", "declared", "stale", "missing", "continuation"):
            self.assertIn(f"case {case}: each required path is held", output)
        status, output = fixture.run("declared")
        self.assertEqual(status, 0, output)

    def check_unsupplied(self, fixture: Fixture):
        """The same cases against a commit without one of their paths refuse
        before any launch, also before a moved source."""
        head = fixture.remove("plugins/grove/skills/craft")
        for case in ("declared", "explicit"):
            status, output = fixture.run(case)
            self.assertEqual(status, 1, output)
            self.assert_refused(fixture, output, case, "plugins/grove/skills/craft/SKILL.md",
                                "the prepared Grove commit", head)
        status, output = fixture.check("declared")
        self.assertEqual(status, 1, output)
        self.assert_refused(fixture, output, "declared", "plugins/grove/skills/craft/SKILL.md",
                            "the prepared Grove commit", head)
        head = fixture.remove(KIND_SKILL)
        for case in ("smoke", "missing"):
            status, output = fixture.run(case)
            self.assertEqual(status, 1, output)
            self.assert_refused(fixture, output, case, KIND_SKILL, "the prepared Grove commit", head)
            self.assertIn("the kind skill of impl", output)
            self.assertNotIn("moved the checkout", output)
        status, output = fixture.run("plugins")
        self.assertEqual(status, 1, output)
        self.assert_refused(fixture, output, "plugins", KIND_SKILL, "the prepared Grove commit", head)

    def test_a_case_against_a_commit_with_its_paths_passes_the_check(self):
        self.check_supplied(self.fixture)
        self.broken(self.check_supplied, "Supplier.holds", lambda supplier, path, directory=False: False)

    def test_a_case_against_a_commit_without_a_path_refuses_before_any_launch(self):
        self.check_unsupplied(self.fixture)
        self.broken(self.check_unsupplied, "supply_problems", lambda *args: (0, []))

    def check_absent_kind(self, fixture: Fixture):
        """A kind skill that the case declares absent is not required."""
        fixture.remove(KIND_SKILL)
        status, output = fixture.check("kind-absent")
        self.assertEqual(status, 0, output)
        status, output = fixture.check("smoke")
        self.assertEqual(status, 1, output)

    def test_a_kind_declared_absent_is_not_required(self):
        self.check_absent_kind(self.fixture)
        real = trial.supply_problems
        self.broken(self.check_absent_kind, "supply_problems",
                    lambda case, *args: real({**case, "absent": []}, *args))

    def check_external_kind(self, fixture: Fixture):
        """A case with an external source passes the check, although the
        prepared commit holds neither the source directory nor its kind
        skill. A plugin that the marketplace pins supplies its kind too."""
        prepared = json.loads((fixture.prepared / "prepared.json").read_text())["source"]["commit"]
        for path in (RESEARCH_PLUGIN, RESEARCH_SKILL):
            found = trial.subprocess.run(["git", "cat-file", "-e", f"{prepared}:{path}"], cwd=fixture.checkout,
                                         capture_output=True)
            self.assertNotEqual(found.returncode, 0, f"the prepared commit holds {path}")
        status, output = fixture.check("external-kind", "plugin-kind")
        self.assertEqual(status, 0, output)
        status, output = fixture.run("external-kind")
        self.assertEqual(status, 0, output)

    def test_an_external_source_supplies_its_own_paths(self):
        self.check_external_kind(self.fixture)
        self.broken(self.check_external_kind, "source_suppliers",
                    lambda case, prepared, externals: {s["id"]: prepared for s in case.get("sources", [])})
        self.broken(self.check_external_kind, "plugin_supplier",
                    lambda plugin, prepared, scratch: (prepared, RESEARCH_PLUGIN))

    def check_historical_native(self, fixture: Fixture):
        """A case with a native copy from an older commit passes the check,
        although the prepared commit no longer holds the copied directory."""
        fixture.remove("plugins/grove/skills/craft")
        status, output = fixture.check("stale")
        self.assertEqual(status, 0, output)
        status, output = fixture.run("stale")
        self.assertEqual(status, 0, output)
        self.assertIsNone(fixture.record()["skills"]["installed"][0]["currentSha256"])

    def test_a_native_copy_from_an_older_commit_passes_the_check(self):
        self.check_historical_native(self.fixture)
        self.broken(self.check_historical_native, "native_supplier", lambda item, prepared: prepared)

    def check_foreign_native_commit(self, fixture: Fixture):
        """A native copy's commit outside the history of the prepared commit
        refuses."""
        case = json.loads((trial.CASES / "stale" / "case.json").read_text())
        (trial.CASES / "foreign").mkdir(exist_ok=True)
        shutil.copytree(trial.CASES / "stale" / "tree", trial.CASES / "foreign" / "tree", dirs_exist_ok=True)
        foreign = self.external["commit"]
        trial.run_checked(["git", "fetch", "--quiet", self.external["repository"], foreign], cwd=fixture.checkout,
                          env=trial.git_env(os.environ))
        (trial.CASES / "foreign" / "case.json").write_text(json.dumps({
            **case, "name": "foreign",
            "native": [{"name": "research", "from": "plugins/research/skills/grove-research-a", "commit": foreign}]}))
        status, output = fixture.check("foreign")
        self.assertEqual(status, 1, output)
        self.assertIn(f"is not in the history of the prepared Grove commit", output)
        self.assertIn("The case measures a build whose supplier held this path", output)

    def check_plugin_dependency(self, fixture: Fixture):
        """The dependency `grove` of a native plugin supplies the kind skill
        of `impl`, and the plugin's checkout supplies a skill that the task
        relies on. A missing path of either supplier refuses."""
        status, output = fixture.check("plugin-dependency")
        self.assertEqual(status, 0, output)
        self.assertIn("case plugin-dependency: each required path is held", output)
        status, output = fixture.check("plugin-requires-missing")
        self.assertEqual(status, 1, output)
        self.assert_refused(fixture, output, "plugin-requires-missing", f"{RESEARCH_PLUGIN}/craft/absent/SKILL.md",
                            f"the checkout of the plugin 'research' that the marketplace pins, "
                            f"{self.external['repository']} at commit", self.external["commit"])
        self.assertIn("a skill of the native plugin research@fake that the task relies on", output)
        head = fixture.remove(KIND_SKILL)
        status, output = fixture.check("plugin-dependency")
        self.assertEqual(status, 1, output)
        self.assert_refused(fixture, output, "plugin-dependency", KIND_SKILL, "the prepared Grove commit", head)
        self.assertIn(f"{RESEARCH_PLUGIN}/skills/grove-impl/SKILL.md", output)

    def test_a_native_plugin_dependency_supplies_its_kind(self):
        self.check_plugin_dependency(self.fixture)
        self.broken(self.check_plugin_dependency, "native_plugins",
                    lambda plugins, prepared, scratch: ((p, trial.plugin_supplier(p, prepared, scratch))
                                                        for p in plugins))
        self.broken(self.check_plugin_dependency, "plugin_supplier",
                    lambda plugin, prepared, scratch: (prepared, RESEARCH_PLUGIN))

    def test_a_native_commit_outside_the_prepared_history_refuses(self):
        self.check_foreign_native_commit(self.fixture)
        self.broken(self.check_foreign_native_commit, "native_supplier",
                    lambda item, prepared: trial.Supplier("the commit", item["commit"], repository=prepared.repository))

    def test_a_case_declares_its_skills_by_source_and_path(self):
        case = json.loads((trial.CASES / "smoke" / "case.json").read_text())
        for extra, needle in (
            ({"requires": [{"source": "other", "path": "skills/craft/SKILL.md"}]}, "requires"),
            ({"requires": [{"source": "grove", "path": "../craft/SKILL.md"}]}, "requires"),
            ({"requires": [{"source": "grove"}]}, "requires"),
            ({"requires": [{"plugin": "grove@fake", "path": "skills/craft/SKILL.md"}]}, "native plugins"),
            ({"requires": [{"plugin": "grove@fake", "source": "grove", "path": "skills/craft/SKILL.md"}]},
             "requires"),
            ({"absent": "grove-impl"}, "absent"),
            ({"absent": ["skills/grove-impl"]}, "absent"),
            ({"requires": [{"source": "grove", "path": "skills/craft/SKILL.md"}], "absent": ["craft"]}, "absent"),
        ):
            (trial.CASES / "smoke" / "case.json").write_text(json.dumps({**case, **extra}))
            with self.assertRaises(trial.TrialError, msg=f"{extra!r} loaded") as caught:
                trial.load_case("smoke")
            self.assertIn(needle, str(caught.exception))

    # --- External sources ------------------------------------------------------

    def external_record(self, record: dict) -> dict:
        externals = record["sources"]["external"]
        self.assertEqual([external["id"] for external in externals], ["research"])
        return externals[0]

    def jj(self, *args: str) -> str:
        """jj with an empty home, as the runner reads a working copy with
        the fake owner's environment."""
        home = Path(self.temp.name) / "jj-home"
        home.mkdir(exist_ok=True)
        env = {name: value for name, value in os.environ.items() if not name.startswith("JJ_")}
        return trial.run_checked(["jj", *args], env={**env, "HOME": str(home)})

    def check_external(self, fixture: Fixture):
        status, output = fixture.run("external")
        self.assertEqual(status, 0, output)
        record = fixture.record()
        external = self.external_record(record)
        checkout = Path(record["trial"]) / "sources" / "research-checkout"
        self.assertEqual(external["repository"], self.external["repository"])
        self.assertEqual(external["commit"], self.external["commit"])
        self.assertEqual(external["checkout"], str(checkout))
        self.assertEqual(external["treeId"], self.external["tree"])
        self.assertEqual(external.get("resourceDigest"), fake_digest(checkout / RESEARCH_PLUGIN))
        self.assertTrue(external["cleanAfter"])
        self.assertEqual(record["sources"]["settings"]["skills"]["sources"][1]["path"],
                         str(checkout / RESEARCH_PLUGIN))
        markdown = (Path(record["trial"]) / "record.md").read_text()
        self.assertIn(f"| External source `research` | `{self.external['repository']}` at commit "
                      f"`{self.external['commit']}`, a plain Git checkout at `$TRIAL/sources/research-checkout`",
                      markdown)

    def test_an_external_source_comes_from_a_checkout_of_its_commit(self):
        self.check_external(self.fixture)
        self.broken(self.check_external, "external_sources", lambda *args: {})
        self.broken(self.check_external, "external_failures", lambda *args: [])
        real_settings = trial.settings_for
        self.broken(self.check_external, "settings_for",
                    lambda case, checkout, clones=None: real_settings(case, checkout))

    def lax_fetch(self, repository, commit, target, env):
        """A broken fetch: it ignores every failure and checks nothing."""
        for argv in (["git", "init", "--quiet", str(target)], ["git", "fetch", "--quiet", str(repository), commit],
                     ["git", "checkout", "--quiet", "--detach", "FETCH_HEAD"]):
            trial.subprocess.run(argv, cwd=target if argv[1] != "init" else None, env=env, capture_output=True)

    def check_external_refusals(self, fixture: Fixture):
        for case, needle in (("external-unknown", "gives no commit 0123456789abcdef"),
                             ("external-tag", f"has HEAD {self.external['commit']}, not the commit")):
            status, output = fixture.run(case)
            self.assertEqual(status, 1, output)
            self.assertIn(needle, output)
            self.assertFalse(fixture.launched_any(), f"{case}: a harness or driver launched before the refusal")

    def test_an_unknown_commit_or_another_head_refuses_before_any_launch(self):
        self.check_external_refusals(self.fixture)
        self.broken(self.check_external_refusals, "fetch_checkout", self.lax_fetch)
        self.broken(self.check_external_refusals, "checkout_problems", lambda *args: [])

    def test_a_checkout_that_is_not_clean_or_moved_is_a_problem(self):
        env = trial.git_env(os.environ)
        target = Path(self.temp.name) / "probe"
        trial.fetch_checkout(self.external["repository"], self.external["commit"], target, env)
        self.assertEqual(trial.checkout_problems(target, self.external["commit"], env), [])
        (target / "stray").write_text("stray\n")
        self.assertEqual(trial.checkout_problems(target, self.external["commit"], env),
                         [f"the checkout {target} is not clean"])
        other = "f" * 40
        self.assertIn(f"has HEAD {self.external['commit']}, not the commit {other}",
                      trial.checkout_problems(target, other, env)[0])
        self.assertEqual(trial.checkout_problems(target / "gone", other, env),
                         [f"the checkout {target / 'gone'} does not exist"])

    def test_differing_or_absent_digests_fail_the_run(self):
        external = {"id": "research", "checkout": "/nowhere", "commit": "f" * 40}
        launches = [{"sources": [{"id": "research", "resourceDigest": "a" * 64}]},
                    {"sources": [{"id": "research", "resourceDigest": "b" * 64}]}]
        failures = trial.external_failures({"research": external}, launches, None, os.environ)
        self.assertIn("the digests", failures[0])
        self.assertIsNone(external["resourceDigest"])
        self.assertIn("does not exist", failures[1])
        failures = trial.external_failures({"research": dict(external)}, [], None, os.environ)
        self.assertIn("named no resourceDigest", failures[0])

    def check_external_missing(self, fixture: Fixture):
        status, output = fixture.run("external-missing")
        self.assertEqual(status, 0, output)
        record = fixture.record()
        external = self.external_record(record)
        refusal = record["refusal"]
        self.assertEqual(record["launches"], [])
        self.assertEqual(refusal["clone"]["checkout"], external["checkout"])
        self.assertEqual(refusal["clone"]["commit"], self.external["commit"])
        self.assertIn("skill_source_missing", refusal["moved"]["firstLine"])
        self.assertIn(str(Path(external["checkout"]) / RESEARCH_PLUGIN), refusal["moved"]["firstLine"])
        self.assertEqual(refusal["restored"]["status"], "resolved")
        self.assertEqual(external.get("resourceDigest"), fake_digest(Path(external["checkout"]) / RESEARCH_PLUGIN))
        self.assertTrue(external["cleanAfter"])

    def check_external_root_missing(self, fixture: Fixture):
        status, output = fixture.run("external-root-missing")
        self.assertEqual(status, 0, output)
        record = fixture.record()
        external = self.external_record(record)
        refusal = record["refusal"]
        root = Path(external["checkout"]) / RESEARCH_PLUGIN / "skills"
        self.assertEqual(record["launches"], [])
        self.assertEqual(refusal["clone"]["checkout"], external["checkout"])
        self.assertEqual(refusal["clone"]["path"], str(root))
        # The refusal names the root, so the source directory itself was present.
        self.assertIn(f"{root} does not exist", refusal["moved"]["firstLine"])
        self.assertEqual(refusal["restored"]["status"], "resolved")
        self.assertTrue(root.is_dir())
        self.assertTrue(external["cleanAfter"])
        self.assertIn("Its root `skills` was moved from", (Path(record["trial"]) / "record.md").read_text())

    def test_move_root_moves_one_root_of_a_source(self):
        self.check_external_root_missing(self.fixture)
        real_clone = trial.clone_source
        self.broken(self.check_external_root_missing, "clone_source",
                    lambda case, trial_dir, checkout, commit, externals=None:
                        real_clone({key: value for key, value in case.items() if key != "moveRoot"},
                                   trial_dir, checkout, commit, externals))
        self.broken(self.check_external_root_missing, "restore", lambda moved: None)

    def test_move_root_names_a_root_of_the_moved_source(self):
        case = json.loads((trial.CASES / "missing" / "case.json").read_text())
        for name, extra in (("no-move", {"moveAway": None, "expectRefusal": None, "moveRoot": "skills"}),
                            ("other-root", {"moveRoot": "absent"})):
            with self.subTest(name=name):
                (trial.CASES / name).mkdir()
                shutil.copytree(trial.CASES / "smoke" / "tree", trial.CASES / name / "tree")
                (trial.CASES / name / "case.json").write_text(json.dumps({**case, **extra}))
                with self.assertRaises(trial.TrialError) as raised:
                    trial.load_case(name)
                self.assertIn("moveRoot names one of the roots", str(raised.exception))

    def test_move_away_moves_the_checkout_of_an_external_source(self):
        self.check_external_missing(self.fixture)
        real_clone = trial.clone_source
        self.broken(self.check_external_missing, "clone_source",
                    lambda case, trial_dir, checkout, commit, externals=None:
                        real_clone(case, trial_dir, checkout, commit))
        self.broken(self.check_external_missing, "restore", lambda moved: None)

    def working_copy(self) -> Path:
        """A jj working copy of the pack, with its files in the working-copy change."""
        directory = Path(os.path.realpath(tempfile.mkdtemp(dir=self.temp.name))) / "research-wc"
        (directory / RESEARCH_SKILL).parent.mkdir(parents=True)
        (directory / RESEARCH_SKILL).write_text("research in progress\n")
        self.jj("git", "init", "--quiet", str(directory))
        return directory

    def check_working_copy(self, fixture: Fixture):
        directory = self.working_copy()
        for empty in (False, True):
            if empty:
                self.jj("commit", "--quiet", "-m", "pack", "-R", str(directory))
            status, output = fixture.run("external", args=("--working-copy", f"research={directory}"))
            self.assertEqual(status, 0, output)
            record = fixture.record()
            external = self.external_record(record)
            self.assertIn("workingCopy", external)
            copy = external["workingCopy"]
            facts = self.jj("log", "--no-graph", "-R", str(directory), "-r", "@", "-T", 'change_id ++ " " ++ commit_id')
            self.assertEqual(f"{copy['changeId']} {copy['commitId']}", facts)
            self.assertEqual(copy["path"], str(directory))
            self.assertIs(copy["empty"], empty)
            self.assertEqual(copy["commitAfter"], copy["commitId"])
            self.assertEqual(external["commit"], self.external["commit"])
            self.assertNotIn("checkout", external)
            self.assertFalse((Path(record["trial"]) / "sources" / "research-checkout").exists())
            self.assertEqual(record["sources"]["settings"]["skills"]["sources"][1]["path"],
                             str(directory / RESEARCH_PLUGIN))
            self.assertEqual(external.get("resourceDigest"), fake_digest(directory / RESEARCH_PLUGIN))
            self.assertIn(f"the working copy `{directory}` in place of", (Path(record["trial"]) / "record.md").read_text())

    def test_a_working_copy_replaces_the_checkout_of_one_source(self):
        self.check_working_copy(self.fixture)
        self.broken(self.check_working_copy, "parse_working_copies", lambda values: {})
        self.broken(self.check_working_copy, "working_copy_facts",
                    lambda directory, environ: {"path": str(directory), "changeId": "", "commitId": "",
                                                "empty": False})

    def check_working_copy_refusals(self, fixture: Fixture):
        directory = self.working_copy()
        for case, value, needle in (
            ("external", f"grove={directory}", "the external sources of the case external are ['research']"),
            ("smoke", f"research={directory}", "the external sources of the case smoke are none"),
            ("external-missing", f"research={directory}", "moveAway would move the working copy"),
            ("external", "research", "--working-copy takes ID=DIR"),
            ("external", f"research={directory / 'absent'}", "which is not a directory"),
        ):
            status, output = fixture.run(case, args=("--working-copy", value))
            self.assertEqual(status, 1, output)
            self.assertIn(needle, output)
            self.assertFalse(fixture.launched_any(), f"{case} {value}: a launch before the refusal")
        shutil.rmtree(directory / "plugins")
        status, output = fixture.run("external", args=("--working-copy", f"research={directory}"))
        self.assertEqual(status, 1, output)
        self.assertIn(f"holds no directory {RESEARCH_PLUGIN}", output)
        self.assertFalse(fixture.launched_any())

    def test_a_working_copy_option_refuses_what_it_cannot_replace(self):
        self.check_working_copy_refusals(self.fixture)
        real_sources = trial.external_sources
        self.broken(self.check_working_copy_refusals, "external_sources",
                    lambda case, trial_dir, copies, environ, keg=None: real_sources(case, trial_dir, {}, environ, keg))

    # --- The candidate keg -------------------------------------------------------

    def check_keg(self, fixture: Fixture):
        """A keg source comes from the candidate keg, and Ravel captures it
        from its directory in the keg with the accepted digest."""
        candidate = fixture.keg()
        status, output = fixture.check("keg", keg=candidate)
        self.assertEqual(status, 0, output)
        status, output = fixture.run("keg")
        self.assertEqual(status, 0, output)
        record = fixture.record()
        source = candidate / "keg" / "share/ravel/skills/grove"
        external = record["sources"]["external"][0]
        self.assertEqual((external["id"], external["keg"]), ("grove", str(candidate / "keg")))
        self.assertEqual(external["resourceDigest"], external["acceptedDigest"])
        self.assertEqual(external["originPaths"], [str(source)])
        self.assertEqual(record["sources"]["settings"]["skills"]["sources"][0]["path"], str(source))
        self.assertEqual(record["builds"]["keg"]["commit"], record["sources"]["commit"])
        markdown = (Path(record["trial"]) / "record.md").read_text()
        self.assertIn(f"| Keg source `grove` | the candidate keg `{candidate / 'keg'}`", markdown)

    def test_a_keg_source_comes_from_the_candidate_keg(self):
        self.check_keg(self.fixture)
        self.broken(self.check_keg, "external_sources", lambda *args: {})
        self.broken(self.check_keg, "external_root", lambda external: Path(external.get("checkout", "/absent")))

    def check_keg_digest(self, fixture: Fixture):
        """A digest other than the accepted one fails the run."""
        fixture.keg(accepted="0" * 64)
        status, output = fixture.run("keg")
        self.assertEqual(status, 1, output)
        self.assertIn(f"and the candidate keg accepted {'0' * 64}", output)

    def test_a_keg_source_with_another_digest_fails(self):
        self.check_keg_digest(self.fixture)
        self.broken(self.check_keg_digest, "external_failures", lambda *args: [])

    def check_keg_refusals(self, fixture: Fixture):
        """Without a candidate keg, or with one of another commit, a keg case
        refuses before any launch."""
        status, output = fixture.run("keg")
        self.assertEqual(status, 1, output)
        self.assertIn("comes from the candidate keg; prepare with --keg", output)
        self.assertFalse(fixture.launched_any(), "a harness or driver launched before the refusal")
        status, output = fixture.check("keg")
        self.assertEqual(status, 1, output)
        candidate = fixture.keg()
        fixture.add({"plugins/grove/skills/extra/SKILL.md": "extra\n"})
        status, output = fixture.check("keg", keg=candidate)
        self.assertEqual(status, 1, output)
        self.assertIn("not the prepared commit", output)

    def test_a_keg_case_without_its_keg_refuses_before_any_launch(self):
        self.check_keg_refusals(self.fixture)
        real = trial.external_sources
        self.broken(self.check_keg_refusals, "external_sources",
                    lambda case, trial_dir, copies, environ, keg=None: real(
                        {**case, "sources": [s for s in case["sources"] if "keg" not in s]},
                        trial_dir, copies, environ, keg))

    def test_a_keg_source_names_the_keg_and_its_directory(self):
        keg = json.loads((trial.CASES / "keg" / "case.json").read_text())
        source = keg["sources"][0]
        missing = json.loads((trial.CASES / "missing" / "case.json").read_text())
        continuation = json.loads((trial.CASES / "continuation" / "case.json").read_text())
        for name, case in (
            ("other-formula", {**keg, "sources": [{**source, "keg": "personal"}]}),
            ("other-directory", {**keg, "sources": [{**source, "plugin": "plugins/grove"}]}),
            ("keg-external", {**keg, "sources": [{**source, "external": {
                "repository": self.external["repository"], "commit": self.external["commit"]}}]}),
            ("keg-moved", {**missing, "sources": [source]}),
            ("keg-resumed", {**continuation, "sources": [*continuation["sources"],
                                                         {**source, "id": "kegged",
                                                          "plugin": "share/ravel/skills/kegged"}]}),
        ):
            shutil.copytree(trial.CASES / "continuation", trial.CASES / name)
            (trial.CASES / name / "case.json").write_text(json.dumps({**case, "name": name}))
            with self.assertRaises(trial.TrialError, msg=name):
                trial.load_case(name)
        trial.load_case("keg")

    def test_an_external_source_needs_a_full_commit_and_no_continuation(self):
        smoke = json.loads((trial.CASES / "external" / "case.json").read_text())
        research = smoke["sources"][1]
        continuation = json.loads((trial.CASES / "continuation" / "case.json").read_text())
        for name, case in (
            ("short", {**smoke, "sources": [smoke["sources"][0],
                                            {**research, "external": {**research["external"], "commit": "abc123"}}]}),
            ("extra", {**smoke, "sources": [smoke["sources"][0],
                                            {**research, "external": {**research["external"], "ref": "main"}}]}),
            ("upward", {**smoke, "sources": [smoke["sources"][0], {**research, "plugin": "../research"}]}),
            ("resumed", {**continuation, "sources": [*continuation["sources"], research]}),
        ):
            shutil.copytree(trial.CASES / "continuation", trial.CASES / name)
            (trial.CASES / name / "case.json").write_text(json.dumps({**case, "name": name}))
            with self.assertRaises(trial.TrialError, msg=name):
                trial.load_case(name)

    # --- The offline resolution ------------------------------------------------

    def check_resolve(self, fixture: Fixture):
        status, output = fixture.run("resolve")
        self.assertEqual(status, 0, output)
        record = fixture.record()
        resolve = record["resolve"]
        self.assertEqual(resolve["descriptorIdentity"], record["launches"][0]["descriptorIdentity"])
        self.assertIsNotNone(resolve["descriptorIdentity"])
        self.assertEqual(resolve["networkControl"]["unguardedExit"], 0)
        self.assertNotEqual(resolve["networkControl"]["guardedExit"], 0)
        self.assertIsNone(resolve["gitControl"]["resolutionPath"])
        self.assertIn("Offline resolution:", (Path(record["trial"]) / "record.md").read_text())

    @unittest.skipUnless(platform.system() == "Darwin", "the profile is a Seatbelt profile")
    def test_the_offline_resolution_denies_the_network_and_git(self):
        self.check_resolve(self.fixture)
        # A profile that allows the network: the guarded probe reaches the listener.
        with mock.patch.object(trial, "NO_NETWORK", "(version 1)(allow default)"):
            self.broken(self.check_resolve, "resolve_failures", trial.resolve_failures)
        git = os.path.dirname(shutil.which("git"))
        self.broken(self.check_resolve, "resolution_path", lambda front: f"{front.parent}:{git}")

    def test_a_different_descriptor_fails_the_offline_resolution(self):
        launch = {"descriptorIdentity": {"directory": "a", "sha256": "a"}}
        resolve = {"networkControl": {"unguardedExit": 0, "guardedExit": 1},
                   "gitControl": {"usualPath": "/usr/bin/git", "resolutionPath": None},
                   "exit": 0, "status": "resolved", "stderr": "",
                   "descriptorIdentity": {"directory": "b", "sha256": "b"}}
        failures = trial.resolve_failures(resolve, [launch])
        self.assertEqual(len(failures), 1)
        self.assertIn("the offline resolution gave the descriptor", failures[0])
        self.assertEqual(trial.resolve_failures({**resolve, "descriptorIdentity": launch["descriptorIdentity"]},
                                                [launch]), [])

    # --- The standalone mode --------------------------------------------------

    def runtime_reads(self, fixture: Fixture) -> list[str]:
        argv = json.loads(Path(str(fixture.grove_env) + ".argv").read_text())
        return [argv[n + 1] for n, arg in enumerate(argv) if arg == "--runtime-read"]

    def check_standalone(self, fixture: Fixture):
        """A standalone run sends no `--runtime-read`, and its record comes
        from Grove's run log."""
        fixture.behave(noSession=True)
        status, output = fixture.run("standalone")
        self.assertEqual(status, 0, output)
        record = fixture.record()
        self.assertEqual(record["mode"], "standalone")
        self.assertEqual(self.runtime_reads(fixture), [])
        self.assertEqual(len(record["launches"]), 1)
        launch = record["launches"][0]
        self.assertEqual(launch["control"], [])
        main = [t for t in launch["transcripts"] if t["main"]]
        self.assertEqual([t["file"] for t in main], ["stream"])
        self.assertEqual([(e["tool"], e["class"], e.get("skill"), e["error"]) for e in main[0]["events"]], EXPECTED)
        self.assertEqual([line.split()[0] for line in record["ending"]["published"]], ["stamp.sh"])
        self.assertIn("ravel: a line of the run log", record["ending"]["runLog"])
        markdown = (Path(record["trial"]) / "record.md").read_text()
        self.assertIn("| Mode | standalone |", markdown)

    def test_a_standalone_run_records_its_stream_from_the_run_log(self):
        self.check_standalone(self.fixture)
        self.broken(self.check_standalone, "split_log", lambda text: [])
        real = trial.run_driver
        self.broken(self.check_standalone, "run_driver",
                    lambda argv, *rest: real([*argv, "--runtime-read", "/granted"], *rest))

    def check_standalone_codex(self, fixture: Fixture):
        """A standalone Codex launch runs in the Codex sandbox
        `workspace-write`, with the sandbox `.codex` of a lifecycle launch."""
        status, output = fixture.run_codex("standalone")
        self.assertEqual(status, 0, output)
        record = fixture.record()
        self.assertEqual(self.runtime_reads(fixture), [])
        launch = record["launches"][0]
        self.assertEqual(launch["control"], [])
        self.assertNotEqual(launch["transcripts"][0]["file"], "stream")
        route = json.loads((Path(record["trial"]) / "route.json").read_text())
        self.assertEqual(route["fixedArgs"][route["fixedArgs"].index("--sandbox") + 1], "workspace-write")
        self.assertEqual(record["harness"]["sandboxMode"], "workspace-write")
        self.assertIn("| Codex sandbox | `workspace-write`, approval `never` |",
                      (Path(record["trial"]) / "record.md").read_text())

    def test_a_standalone_codex_launch_keeps_its_own_sandbox(self):
        self.check_standalone_codex(self.fixture)
        self.broken(self.check_standalone_codex, "CODEX_SANDBOX", "danger-full-access")

    def test_every_codex_route_keeps_its_own_sandbox(self):
        for mode in trial.MODES:
            case = {"kinds": ["impl"], "name": "x", "harness": "codex", "mode": mode}
            route = trial.route_for(case, Path("/r"), Path("/c"), "m", "e")
            self.assertEqual(route["fixedArgs"][route["fixedArgs"].index("--sandbox") + 1], trial.CODEX_SANDBOX)

    # --- Continuation ----------------------------------------------------------

    def check_continuation(self, fixture: Fixture):
        fixture.behave(resumeTools=RESUME_TOOLS)
        status, output = fixture.run("continuation")
        self.assertEqual(status, 0, output)
        record = fixture.record()
        launches = record["launches"]
        self.assertEqual([launch.get("role") for launch in launches], ["first turn", "resumed turn", "fresh launch"])
        first, resumed, fresh = launches

        def main(launch):
            return next(t for t in launch["transcripts"] if t["main"])

        # The resumed turn appends to the first turn's session file, and each
        # turn holds only its own lines.
        self.assertEqual(first["sessionId"], resumed["sessionId"])
        self.assertNotEqual(fresh["sessionId"], first["sessionId"])
        self.assertIn("lines", main(first), "the first turn has no window")
        self.assertIn("lines", main(resumed), "the resumed turn has no window")
        self.assertEqual(main(first)["lines"][1], main(resumed)["lines"][0])
        self.assertEqual([(e["tool"], e["class"], e.get("skill"), e["error"]) for e in main(first)["events"]], EXPECTED)
        # The first turn's snapshot gives the original bytes, and the source
        # origin gives the changed bytes.
        self.assertEqual([(e["class"], e.get("skill"), e["marker"]) for e in main(resumed)["events"]],
                         [("mandatory", "grove", []), ("skill", "craft", []), ("source-origin", "craft", ["result"])])
        crafts = {name: [e["marker"] for e in main(launch)["events"] if e["class"] == "skill"]
                  for name, launch in (("first", first), ("fresh", fresh))}
        self.assertEqual(crafts, {"first": [[]], "fresh": [["result"]]})
        self.assertEqual(resumed["descriptorIdentity"], first["descriptorIdentity"])
        self.assertNotEqual(fresh["descriptorIdentity"], first["descriptorIdentity"])
        self.assertEqual(first["snapshotMarkers"], {CRAFT: False})
        self.assertEqual(fresh["snapshotMarkers"], {CRAFT: True})
        # The resumed turn runs outside Ravel, so it has no descriptor variable.
        self.assertIn("RAVEL_SKILL_DIRECTORIES", first["envNames"])
        self.assertNotIn("RAVEL_SKILL_DIRECTORIES", resumed["envNames"])
        self.assertEqual(resumed["control"], [])
        continuation = record["continuation"]
        self.assertEqual(continuation["failures"], [])
        self.assertEqual(continuation["resume"]["args"][-2:], ["--resume", first["sessionId"]])
        self.assertNotIn("--add-dir", continuation["resume"]["args"])
        self.assertNotEqual(continuation["edit"]["commitBefore"], continuation["edit"]["commitAfter"])
        trial_dir = Path(record["trial"])
        settings = json.loads((trial_dir / "home/.config/ravel/settings.json").read_text())["skills"]
        self.assertEqual(settings["mandatory"], continuation["settings"]["mandatoryAfter"])
        self.assertEqual(settings["sources"][0]["path"], str(trial_dir / "sources/checkout/plugins/grove"))
        self.assertIn("02-impl--fresh-k2.md", record["ending"]["treeAfter"])
        self.assertTrue((fixture.root / "source" / CRAFT).read_text() == "craft\n", "the prepared checkout changed")
        markdown = (trial_dir / "record.md").read_text()
        for needle in ("Continuation:", "#### Launch 1: kind `impl`, resumed turn", " Marker |",
                       f"- Snapshot copy of `{CRAFT}`: holds the marker."):
            self.assertIn(needle, markdown)
        # A second extraction keeps each turn's window.
        with contextlib.redirect_stdout(io.StringIO()):
            self.assertEqual(trial.main(["reextract", "--trial", str(trial_dir)], fixture.environ()), 0)
        again = json.loads((trial_dir / "record.json").read_text())
        self.assertEqual([main(launch)["events"] for launch in again["launches"]],
                         [main(launch)["events"] for launch in launches])

    def test_a_continuation_resumes_after_a_source_commit_and_launches_fresh(self):
        self.check_continuation(self.fixture)
        self.broken(self.check_continuation, "resume_turn", lambda *args, **kwargs: {
            "args": [], "cwd": "", "answerSha256": "", "exit": 0, "timedOut": False, "elapsedS": 0})
        self.broken(self.check_continuation, "commit_edits", lambda clone, edits: {
            "checkout": str(clone), "commitBefore": "x", "commitAfter": "x", "files": []})
        self.broken(self.check_continuation, "write_window", lambda record, start, end: None)
        self.broken(self.check_continuation, "read_window", lambda record: None)
        self.broken(self.check_continuation, "marker_places", lambda marker, tool_input, result: [])
        self.broken(self.check_continuation, "snapshot_markers", lambda case, launch: {})
        self.broken(self.check_continuation, "add_fresh_tree", lambda repo, tree, env: {"added": [], "commit": "x" * 12})

    def check_codex_continuation(self, fixture: Fixture):
        """The continuation in Codex: `codex exec ... resume <thread>` keeps
        the thread and appends to the first rollout file, and each turn holds
        only its own lines."""
        fixture.behave(resumeItems=CODEX_RESUME_ITEMS, readFiles=True)
        status, output = fixture.run_codex("continuation")
        self.assertEqual(status, 0, output)
        record = fixture.record()
        launches = record["launches"]
        self.assertEqual([launch.get("role") for launch in launches], ["first turn", "resumed turn", "fresh launch"])
        first, resumed, fresh = launches

        def main(launch):
            return next(t for t in launch["transcripts"] if t["main"])

        self.assertEqual(first["sessionId"], resumed["sessionId"])
        self.assertNotEqual(fresh["sessionId"], first["sessionId"])
        self.assertEqual(main(first)["file"], main(resumed)["file"])
        self.assertIn("lines", main(first), "the first turn has no window")
        self.assertIn("lines", main(resumed), "the resumed turn has no window")
        self.assertEqual(main(first)["lines"][1], main(resumed)["lines"][0])
        self.assertEqual([(e["tool"], e["class"], e.get("skill"), e["error"]) for e in main(first)["events"]],
                         CODEX_EXPECTED)
        self.assertEqual([(e["class"], e.get("skill"), e["marker"]) for e in main(resumed)["events"]],
                         [("mandatory", "grove", []), ("skill", "craft", []), ("source-origin", "craft", ["result"])])
        crafts = {name: [e["marker"] for e in main(launch)["events"] if e["class"] == "skill"]
                  for name, launch in (("first", first), ("fresh", fresh))}
        self.assertEqual(crafts, {"first": [[], []], "fresh": [["result"], []]})
        self.assertEqual(resumed["descriptorIdentity"], first["descriptorIdentity"])
        self.assertNotEqual(fresh["descriptorIdentity"], first["descriptorIdentity"])
        self.assertEqual(first["snapshotMarkers"], {CRAFT: False})
        self.assertEqual(fresh["snapshotMarkers"], {CRAFT: True})
        # The resumed turn keeps the sandbox of the route, and its window
        # holds a turn context but no session metadata of its own.
        self.assertEqual(main(resumed)["startup"]["init"]["sandbox_policy"]["type"], "workspace-write")
        self.assertNotIn("cli_version", main(resumed)["startup"]["init"])
        self.assertIn("RAVEL_SKILL_DIRECTORIES", first["envNames"])
        self.assertNotIn("RAVEL_SKILL_DIRECTORIES", resumed["envNames"])
        self.assertIn("CODEX_HOME", resumed["envNames"])
        self.assertEqual(resumed["control"], [])
        continuation = record["continuation"]
        self.assertEqual(continuation["failures"], [])
        self.assertTrue(continuation["mechanism"].startswith("codex exec"), continuation["mechanism"])
        args = continuation["resume"]["args"]
        self.assertEqual(args[0], "exec")
        self.assertEqual(args[-2:], ["resume", first["sessionId"]])
        self.assertLess(args.index("--sandbox"), args.index("resume"))
        self.assertEqual(continuation["resume"]["cwd"], str(Path(record["trial"]) / "repo"))
        markdown = (Path(record["trial"]) / "record.md").read_text()
        for needle in ("Continuation:", "#### Launch 1: kind `impl`, resumed turn", " Marker |"):
            self.assertIn(needle, markdown)

    def test_a_codex_continuation_resumes_its_thread_and_launches_fresh(self):
        self.check_codex_continuation(self.fixture)
        self.broken(self.check_codex_continuation, "CODEX.resume_args", trial.CLAUDE.resume_args)
        self.broken(self.check_codex_continuation, "CODEX.stream_init", lambda record: (None, None))
        self.broken(self.check_codex_continuation, "CODEX.session_cwd", lambda transcript: "/")
        self.broken(self.check_codex_continuation, "read_window", lambda record: None)
        real_describe = trial.CODEX.describe
        self.broken(self.check_codex_continuation, "CODEX.describe",
                    lambda *args, **kwargs: real_describe(*args[:9]))

    def test_a_continuation_case_needs_its_parts(self):
        case = json.loads((trial.CASES / "continuation" / "case.json").read_text())
        for change in ({"marker": ""}, {"edits": []}, {"edits": [{"path": CRAFT, "insert": "no marker"}]},
                       {"edits": [{"path": "elsewhere/SKILL.md", "insert": MARKER}]}, {"freshTree": "absent"}):
            name = "broken-" + "-".join(change)
            shutil.copytree(trial.CASES / "continuation", trial.CASES / name, dirs_exist_ok=True)
            (trial.CASES / name / "case.json").write_text(json.dumps(
                {**case, "name": name, "continuation": {**case["continuation"], **change}}))
            with self.assertRaises(trial.TrialError, msg=f"{change} loaded"):
                trial.load_case(name)
        trial.load_case("continuation")

    def test_an_insertion_follows_the_front_matter(self):
        self.assertEqual(trial.insert_after_front_matter("---\nname: x\n---\n\n# X\n", "M."),
                         "---\nname: x\n---\n\nM.\n\n# X\n")
        self.assertEqual(trial.insert_after_front_matter("# X\n", "M."), "M.\n\n# X\n")

    def test_marker_files_skip_the_tree_and_the_stores(self):
        repo = Path(tempfile.mkdtemp(dir=self.temp.name))
        for name in ("stamp.sh", ".grove/leaf.md", ".jj/x", "tools/count.sh", "plain.txt"):
            (repo / name).parent.mkdir(parents=True, exist_ok=True)
            (repo / name).write_text(f"# {MARKER}\n" if name != "plain.txt" else "plain\n")
        self.assertEqual(trial.marker_files(repo, MARKER), ["stamp.sh", "tools/count.sh"])

    # --- Compaction --------------------------------------------------------------

    def compact(self, fixture: Fixture, mode: str, **behaviour) -> tuple[int, str, dict]:
        fixture.behave(**{"tools": FIRST_TOOLS, "afterTools": AFTER_TOOLS, "summary": SUMMARIES[mode], **behaviour})
        status, output = fixture.run(f"compaction-{mode}")
        return status, output, fixture.record()

    def check_compaction(self, fixture: Fixture):
        for mode in ("path", "preamble"):
            status, output, record = self.compact(fixture, mode)
            self.assertEqual(status, 0, output)
            compaction = record["compaction"]
            self.assertEqual(compaction["failures"], [])
            self.assertTrue(compaction["gate"]["ok"], compaction["gate"])
            self.assertEqual([turn["turn"] for turn in compaction["turns"]], [0, 1, 2])
            self.assertEqual((compaction["api"]["keep"], compaction["api"]["missingBlobs"]), (0, 0))
            self.assertEqual((compaction["afterBoundary"]["paths"], compaction["afterBoundary"]["lines"]), ([], []))
            self.assertEqual(compaction["restored"], [])
            self.assertEqual(compaction["afterBoundary"]["variable"], mode == "path")
            self.assertEqual(compaction["variable"], {"set": True, "descriptor": True})
            self.assertGreater(compaction["preamble"]["paths"], 0)
            launch = record["launches"][0]
            events = next(t for t in launch["transcripts"] if t["main"])["events"]
            self.assertEqual([(e["class"], e.get("skill")) for e in events], AFTER_EXPECTED)
            self.assertTrue(events[4]["target"].startswith("/compact "), events[4])
            # The feeder ran the harness in one launch, with no --add-dir.
            self.assertEqual(len(record["launches"]), 1)
            trial_dir = Path(record["trial"])
            self.assertFalse((trial_dir / "home/.fake-claude-snap").exists(), "the launch had --add-dir")
            route = json.loads((trial_dir / "route.json").read_text())
            self.assertEqual((route["addDirs"], route["programArgs"][2]), (False, "turns"))
            markdown = (trial_dir / "record.md").read_text()
            for needle in ("Compaction:", "Gate before the answer: passed", "`keep` 0"):
                self.assertIn(needle, markdown)
            with contextlib.redirect_stdout(io.StringIO()):
                self.assertEqual(trial.main(["reextract", "--trial", str(trial_dir)], fixture.environ()), 0)
            again = json.loads((trial_dir / "record.json").read_text())
            self.assertEqual(next(t for t in again["launches"][0]["transcripts"] if t["main"])["events"], events)

    def test_a_compaction_case_compacts_in_one_launch_and_reads_the_context(self):
        self.check_compaction(self.fixture)
        self.broken(self.check_compaction, "turn_event", lambda entry: None)
        self.broken(self.check_compaction, "MESSAGE_TYPES", ("user", "assistant", "attachment", "last-prompt"))
        self.broken(self.check_compaction, "api_context", lambda entries, marks: {"found": False})
        self.broken(self.check_compaction, "write_turns", lambda case, case_dir, trial: (trial / "turns.json")
                    .write_text(json.dumps({"turns": [{"text": "/compact x"}]})))

    def check_gate(self, fixture: Fixture):
        # A summary that keeps a snapshot path ends the input before the answer.
        status, output, record = self.compact(fixture, "preamble", summary="Read {SNAP}/skills/grove/SKILL.md.")
        self.assertEqual(status, 1, output)
        compaction = record["compaction"]
        self.assertFalse(compaction["gate"]["ok"])
        self.assertTrue(compaction["afterBoundary"]["paths"], "the in-process scan missed the path")
        self.assertEqual([turn["turn"] for turn in compaction["turns"]], [0, 1])
        self.assertIn("the gate before the answer did not pass", output)
        # A summary of the path case that lost the variable's name fails too.
        status, output, record = self.compact(fixture, "path", summary="The leaf asks for one file.")
        self.assertEqual(status, 1, output)
        self.assertIn("no longer names RAVEL_SKILL_DIRECTORIES", output)
        # A restored file attachment that names a preamble path fails the gate.
        status, output, record = self.compact(fixture, "preamble", restored=["{SNAP}/skills/grove/SKILL.md"])
        self.assertEqual(status, 1, output)
        self.assertTrue(record["compaction"]["afterBoundary"]["paths"])
        self.assertTrue(record["compaction"]["restored"][0].endswith("/skills/grove/SKILL.md"))

    def test_the_gate_withholds_the_answer_when_the_preamble_remains(self):
        self.check_gate(self.fixture)
        self.broken(self.check_gate, "after_boundary", lambda entries, marks: trial.context_findings("", "", {
            "paths": [], "digests": [], "lines": []}))
        self.broken(self.check_gate, "preamble_marks", lambda prompt: {
            "paths": [], "digests": [], "lines": [], "sha256": ""})

    def check_model_context(self, fixture: Fixture):
        # The system prompt of the first request after compaction keeps a path,
        # which the session lines after the boundary do not show.
        status, output, record = self.compact(fixture, "preamble", system="Additional directory: {SNAP}/skills")
        self.assertEqual(status, 1, output)
        self.assertTrue(record["compaction"]["gate"]["ok"])
        self.assertIn("model context: the context still holds the preamble paths", output)
        # A request that keeps earlier messages is not the whole context.
        status, output, record = self.compact(fixture, "preamble", keep=2)
        self.assertEqual(status, 1, output)
        self.assertIn("keep 2", output)

    def test_the_model_context_of_the_first_request_must_lack_the_preamble(self):
        self.check_model_context(self.fixture)
        self.broken(self.check_model_context, "context_findings", lambda text, words, marks: {
            "paths": [], "digests": [], "lines": [], "terms": [], "variable": False, "snippets": []})

    def test_a_compaction_case_needs_its_parts(self):
        case = json.loads((trial.CASES / "compaction-preamble" / "case.json").read_text())
        for change, top in (({"mode": "other"}, {}), ({"instructions": ""}, {}),
                            ({"instructions": "Drop the descriptor."}, {}),
                            ({"instructions": "Drop RAVEL_SKILL_DIRECTORIES."}, {}),
                            ({"answer": "absent.md"}, {}), ({}, {"route": "native-2.0"})):
            name = "broken-compaction-" + "-".join([*change, *top])
            shutil.copytree(trial.CASES / "compaction-preamble", trial.CASES / name, dirs_exist_ok=True)
            (trial.CASES / name / "case.json").write_text(json.dumps(
                {**case, **top, "name": name, "compaction": {**case["compaction"], **change}}))
            with self.assertRaises(trial.TrialError, msg=f"{change} {top} loaded"):
                trial.load_case(name)
        trial.load_case("compaction-preamble")

    def test_the_extraction_names_a_read_of_the_variable(self):
        for command, expected in (("cat \"$RAVEL_SKILL_DIRECTORIES\"", "variable"), ("env", "variable"),
                                  ("printenv HOME", "variable"), ("echo envoy", "command"),
                                  ("ls environment", "command")):
            entry = {"type": "assistant", "message": {"content": [
                {"type": "tool_use", "id": "t", "name": "Bash", "input": {"command": command}}]}}
            classifier = trial.Classifier({}, Path("/h"), Path("/r"))
            self.assertEqual(trial.extract_events([entry], classifier)[0]["class"], expected, command)

    def check_craft_root(self):
        # Ravel's cache lies under `.cache/ravel/skills/v1`, so a read in a
        # `craft` root has a `skills` segment only above the snapshot.
        temp = Path(tempfile.mkdtemp(dir=self.temp.name))
        snap = temp / "home/.cache/ravel/skills/v1/snapshots/d8/tree"
        for relative in ("skills/grove-design/SKILL.md", "craft/cli-tool-design/SKILL.md"):
            (snap / relative).parent.mkdir(parents=True, exist_ok=True)
            (snap / relative).write_text("---\nname: x\n---\n")
        view = {"kind": "impl", "skills": {
            "sources": [{"id": "design", "snapshotRoot": str(snap), "originPath": str(temp / "origin")}],
            "directories": [{"source": "design", "path": str(snap / "skills")},
                            {"source": "design", "path": str(snap / "craft")}],
            "mandatory": []}}
        classifier = trial.Classifier(view, temp / "home", temp / "repo")
        self.assertEqual(classifier.classify(str(snap / "craft/cli-tool-design/SKILL.md")),
                         ("skill", "cli-tool-design"))
        self.assertEqual(classifier.classify(str(snap / "craft/cli-tool-design")),
                         ("listing", "cli-tool-design"))
        self.assertEqual(classifier.classify(str(snap / "skills/grove-design/SKILL.md")),
                         ("skill", "grove-design"))
        self.assertEqual(classifier.classify(str(snap / "craft")), ("listing", None))

    def test_a_read_in_a_craft_root_names_its_skill(self):
        self.check_craft_root()
        # A runner that names a skill by the last `skills` segment alone gives
        # the cache's `v1` for a craft read.
        with (mock.patch.object(trial.Classifier, "_skill_name", lambda self, path: trial.Classifier._skill(path)),
              self.assertRaises(AssertionError, msg="the check passed with a broken _skill_name")):
            self.check_craft_root()

    # --- Delegation -------------------------------------------------------------

    def check_delegation(self, fixture: Fixture):
        fixture.behave(delegate=DELEGATE, writeFiles=[{"path": "{TRIAL}/authority.log", "text": AUTHORITY_LOG}])
        status, output = fixture.run("delegate")
        self.assertEqual(status, 0, output)
        record = fixture.record()
        found = record["launches"][0]["delegations"]
        self.assertEqual(len(found), 1)
        item = found[0]
        self.assertTrue(item["givesDescriptorPath"])
        self.assertFalse(item["namesVariable"])
        self.assertEqual((item["obligations"], item["obligationCount"]), (["11"], 2))
        self.assertEqual(item["endingLines"], ["Do not run end-session."])
        self.assertTrue(item["transcript"].endswith("/subagents/agent-a1.jsonl"))
        self.assertEqual(item["status"], "completed")
        self.assertEqual(item["result"], "Check written.")
        self.assertEqual(item["inherited"], ["RAVEL_EXIT_FILE", "GROVE_LAUNCH_DIR"])
        order = item["order"]
        self.assertEqual(order["firstRead"], {"seq": 1, "class": "mandatory"})
        self.assertEqual(order["firstAction"], 3)
        self.assertTrue(order["mandatoryBeforeAction"])
        self.assertEqual(len(order["mandatoryOrder"]), 1)
        # The delegate's ending shows apart from the parent's.
        calls = {(c["main"], c["result"]) for c in record["ending"]["endingCalls"]}
        self.assertEqual(calls, {(True, "grove-continue"), (False, "trial-delegate-early")})
        self.assertEqual([c["text"] for c in record["collected"]], [AUTHORITY_LOG])
        markdown = (Path(record["trial"]) / "record.md").read_text()
        for needle in ("Delegation 0: `general-purpose`", "Obligations word for word: 1 of 2 (11).",
                       "`trial-delegate-early` at call 4 of `agent-a1.jsonl`, a delegate",
                       "Collected file `$TRIAL/authority.log`", "first exit 0"):
            self.assertIn(needle, markdown)
        # A delegate that acts before it reads the mandatory file shows so.
        late = dict(DELEGATE, tools=[DELEGATE["tools"][2], *DELEGATE["tools"][:2]])
        fixture = self.fresh()
        fixture.behave(delegate=late)
        status, output = fixture.run("delegate")
        self.assertEqual(status, 0, output)
        self.assertFalse(fixture.record()["launches"][0]["delegations"][0]["order"]["mandatoryBeforeAction"])
        self.assertEqual([c["sha256"] for c in fixture.record()["collected"]], [None])
        # A run without a delegate fails a case that expects one.
        fixture = self.fresh()
        status, output = fixture.run("delegate")
        self.assertEqual(status, 1, output)
        self.assertIn("expects a delegate", output)

    def test_a_delegation_records_the_prompt_the_reads_and_the_ending(self):
        self.check_delegation(self.fixture)
        self.broken(self.check_delegation, "CLAUDE.delegations", lambda *args: [])
        self.broken(self.check_delegation, "obligation_lines", lambda prompt: [])
        self.broken(self.check_delegation, "collect_files", lambda case, trial_dir: [])
        self.broken(self.check_delegation, "delegate_failures", lambda case, launches: [])
        self.broken(self.check_delegation, "ending_calls", lambda launches: [])
        real = trial.delegate_order
        self.broken(self.check_delegation, "delegate_order",
                    lambda events, mandatory: {**real(events, mandatory), "mandatoryBeforeAction": True})

    def check_system_note(self, fixture: Fixture):
        case = json.loads((trial.CASES / "delegate" / "case.json").read_text())
        (trial.CASES / "delegate" / "case.json").write_text(json.dumps({**case, "systemNote": "Owner note."}))
        try:
            fixture.behave(delegate=DELEGATE)
            status, output = fixture.run("delegate")
        finally:
            (trial.CASES / "delegate" / "case.json").write_text(json.dumps(case))
        self.assertEqual(status, 0, output)
        record = fixture.record()
        route = json.loads((Path(record["trial"]) / "route.json").read_text())
        self.assertEqual(route["fixedArgs"][-2:], ["--append-system-prompt", "Owner note."])
        config = (Path(record["trial"]) / "home/.config/ravel/config.rhai").read_text()
        self.assertIn('"--append-system-prompt", "Owner note."', config)
        self.assertEqual(record["systemNote"]["text"], "Owner note.")
        self.assertIn("Owner note through `--append-system-prompt`",
                      (Path(record["trial"]) / "record.md").read_text())

    def test_a_system_note_reaches_the_harness_and_the_record(self):
        self.check_system_note(self.fixture)
        real = trial.CLAUDE.launch
        self.broken(self.check_system_note, "CLAUDE.launch",
                    lambda case, *args, **kw: real({k: v for k, v in case.items() if k != "systemNote"}, *args, **kw))

    def check_codex_delegation(self, fixture: Fixture):
        fixture.behave(codexDelegate={**CODEX_DELEGATE, "fork": "none"})
        status, output = fixture.run_codex("delegate")
        self.assertEqual(status, 0, output)
        record = fixture.record()
        launch = record["launches"][0]
        found = launch["delegations"]
        self.assertEqual(len(found), 1)
        item = found[0]
        self.assertEqual((item["form"], item["forkTurns"], item["description"]), ("direct", "none", "check"))
        self.assertTrue(item["givesDescriptorPath"])
        self.assertEqual((item["obligations"], item["obligationCount"]), (["11"], 2))
        self.assertEqual(item["endingLines"], ["Do not run end-session."])
        self.assertFalse(item["historyHoldsDescriptor"])
        self.assertRegex(item["transcript"], r"^\.codex/sessions/2026/11/03/rollout-.*-01a19ed6-5e40-.*\.jsonl$")
        self.assertEqual(item["result"], "Message Type: FINAL_ANSWER\nPayload:\nCheck written.")
        self.assertEqual(item["messages"], {"toDelegate": 1, "fromDelegate": 2})
        self.assertEqual(item["inherited"], ["RAVEL_EXIT_FILE", "GROVE_LAUNCH_DIR"])
        self.assertEqual(item["meta"], {"depth": 1, "agent_path": "/root/check", "agent_role": None})
        self.assertEqual(item["sandbox"]["sandbox_policy"]["type"], "workspace-write")
        order = item["order"]
        self.assertEqual(order["firstRead"], {"seq": 1, "class": "mandatory"})
        self.assertEqual(order["firstAction"], 3)
        self.assertTrue(order["mandatoryBeforeAction"])
        # The parent's events show the start of the delegate as a delegation.
        main = next(t for t in launch["transcripts"] if t["main"])
        self.assertIn(("spawn_agent", "delegate"), [(e["tool"], e["class"]) for e in main["events"]])
        calls = {(c["main"], c["result"]) for c in record["ending"]["endingCalls"]}
        self.assertEqual(calls, {(True, "grove-continue"), (False, "trial-delegate-early")})
        markdown = (Path(record["trial"]) / "record.md").read_text()
        for needle in ("Delegation 0: `default`, \"check\"", "Call: direct. Fork: none.",
                       "Obligations word for word: 1 of 2 (11).", "`trial-delegate-early` at call 4 of `rollout-"):
            self.assertIn(needle, markdown)
        # A direct call hides its message, and the default fork gives the
        # delegate the parent's history, preamble included.
        fixture = self.fresh()
        fixture.behave(codexDelegate={**CODEX_DELEGATE, "encrypted": True})
        status, output = fixture.run_codex("delegate")
        self.assertEqual(status, 0, output)
        item = fixture.record()["launches"][0]["delegations"][0]
        self.assertEqual((item["form"], item["forkTurns"]), ("direct, encrypted", "all, the default"))
        self.assertEqual((item["prompt"], item["promptSha256"], item["givesDescriptorPath"]), ("", None, None))
        self.assertTrue(item["historyHoldsDescriptor"])
        # The forked history holds none of the parent's calls.
        self.assertEqual(item["order"]["firstRead"], {"seq": 1, "class": "mandatory"})
        self.assertIn("encrypted in every transcript", (Path(fixture.record()["trial"]) / "record.md").read_text())
        # A call from code mode is plaintext, in the JavaScript of `exec`.
        fixture = self.fresh()
        fixture.behave(codexDelegate={**CODEX_DELEGATE, "codeMode": True})
        status, output = fixture.run_codex("delegate")
        self.assertEqual(status, 0, output)
        item = fixture.record()["launches"][0]["delegations"][0]
        self.assertEqual(item["form"], "code mode")
        self.assertIn("tools.spawn_agent(", item["prompt"])
        self.assertTrue(item["givesDescriptorPath"])
        # A run without a delegate fails a case that expects one.
        fixture = self.fresh()
        status, output = fixture.run_codex("delegate")
        self.assertEqual(status, 1, output)
        self.assertIn("expects a delegate", output)

    def test_a_codex_delegation_records_the_call_the_reads_and_the_ending(self):
        self.check_codex_delegation(self.fixture)
        self.broken(self.check_codex_delegation, "CODEX.delegations", lambda *args: [])
        self.broken(self.check_codex_delegation, "CODEX.delegate_files", lambda home, thread: {})
        self.broken(self.check_codex_delegation, "ENCRYPTED", trial.re.compile(r"^$"))
        # The last metadata of a forked delegate is the parent's copy.
        self.broken(self.check_codex_delegation, "CODEX.meta",
                    lambda records: next((r.get("payload") or {} for r in reversed(records)
                                          if r.get("type") == "session_meta"), {}))

    def check_codex_system_note(self, fixture: Fixture):
        case = json.loads((trial.CASES / "delegate" / "case.json").read_text())
        (trial.CASES / "delegate" / "case.json").write_text(json.dumps({**case, "systemNote": "Owner \"note\"."}))
        try:
            fixture.behave(codexDelegate=CODEX_DELEGATE)
            status, output = fixture.run_codex("delegate")
        finally:
            (trial.CASES / "delegate" / "case.json").write_text(json.dumps(case))
        self.assertEqual(status, 0, output)
        record = fixture.record()
        route = json.loads((Path(record["trial"]) / "route.json").read_text())
        self.assertEqual(route["fixedArgs"][-2:], ["-c", 'developer_instructions="Owner \\"note\\"."'])
        self.assertEqual(record["systemNote"]["text"], 'Owner "note".')
        self.assertIn("Owner note through `-c developer_instructions`",
                      (Path(record["trial"]) / "record.md").read_text())

    def test_a_codex_system_note_reaches_developer_instructions(self):
        self.check_codex_system_note(self.fixture)
        real = trial.CODEX.launch
        self.broken(self.check_codex_system_note, "CODEX.launch",
                    lambda case, *args, **kw: real({k: v for k, v in case.items() if k != "systemNote"}, *args, **kw))

    def test_a_collected_file_stays_inside_the_trial(self):
        case = json.loads((trial.CASES / "delegate" / "case.json").read_text())
        for item in ("/etc/passwd", "../outside.log", 3):
            name = f"broken-collect-{abs(hash(str(item)))}"
            shutil.copytree(trial.CASES / "delegate", trial.CASES / name)
            (trial.CASES / name / "case.json").write_text(json.dumps({**case, "name": name, "collect": [item]}))
            with self.assertRaises(trial.TrialError, msg=f"{item!r} loaded"):
                trial.load_case(name)
        trial.load_case("delegate")

    def test_the_configuration_routes_only_the_case_kinds(self):
        route = trial.route_for({"kinds": ["impl"], "name": "x"}, Path("/r"), Path("/c"), "m", "e")
        config = trial.config_for({"kinds": ["impl"], "name": "x"}, route)
        self.assertIn('const KINDS = #{ "impl": true };', config)
        self.assertIn('"trial_kind_unrouted"', config)
        self.assertIn("args.push(request.prompt);", config)
        self.assertIn('"--permission-mode", "bypassPermissions"', config)
        # The variadic --add-dir comes before the fixed flags, never before the prompt.
        self.assertLess(config.index('"--add-dir"'), config.index('"-p"'))
        # A compaction case runs the feeder, with the harness as its command, and
        # gives no --add-dir.
        case = {"kinds": ["impl"], "name": "x", "compaction": {"mode": "path"}}
        route = trial.route_for(case, Path("/t/records"), Path("/c"), "m", "e")
        config = trial.config_for(case, route)
        self.assertNotIn('"--add-dir"', config)
        self.assertEqual(route["program"], sys.executable)
        self.assertIn('"turns", "--plan", "/t/turns.json", "--", "/c"', config)
        self.assertLess(config.index('"turns"'), config.index('"-p"'))

    # --- The Codex adapter ----------------------------------------------------

    def check_codex_home_refusals(self, fixture: Fixture):
        """An inherited CODEX_HOME in the real home refuses before any launch:
        the default directory, the home itself, an empty value and an alias
        of the real home."""
        alias = fixture.root / "alias"
        if not alias.exists():
            alias.symlink_to(fixture.owner)
        for value in (str(fixture.owner / ".codex"), str(fixture.owner), "", str(alias / ".codex")):
            status, output = fixture.run_codex(CODEX_HOME=value)
            self.assertEqual(status, 1, f"CODEX_HOME={value!r} did not refuse: {output}")
            self.assertIn("the environment holds CODEX_HOME", output)
            self.assertFalse(fixture.launched_any(), f"CODEX_HOME={value!r}: a harness or driver launched")

    def test_codex_home_in_the_real_home_refuses_before_any_launch(self):
        self.check_codex_home_refusals(self.fixture)
        self.broken(self.check_codex_home_refusals, "CODEX.refuse", lambda environ, owner_home: None)

    def check_codex_home_replaced(self, fixture: Fixture):
        elsewhere = fixture.root / "elsewhere" / ".codex"
        status, output = fixture.run_codex(CODEX_HOME=str(elsewhere))
        self.assertEqual(status, 0, output)
        self.assertIn(f"the inherited CODEX_HOME {elsewhere} is outside the real home; this run uses the sandbox .codex",
                      output)
        record = fixture.record()
        sandbox = str(Path(record["trial"]) / "home/.codex")
        self.assertEqual(json.loads(fixture.grove_env.read_text()).get("CODEX_HOME"), sandbox)
        self.assertIn("outside the real home", record["harness"]["codexHome"])
        self.assertFalse(elsewhere.exists(), "Codex wrote into the inherited CODEX_HOME")
        self.assertTrue(record["launches"][0]["transcripts"][0]["file"].startswith(".codex/sessions/"))

    def test_codex_home_outside_the_real_home_is_replaced_and_reported(self):
        self.check_codex_home_replaced(self.fixture)
        self.broken(self.check_codex_home_replaced, "CODEX.env", lambda home: {})

    def check_codex_allow_list(self, fixture: Fixture):
        status, output = fixture.run_codex(CODEX_PLANTED="planted")
        self.assertEqual(status, 0, output)
        driver = json.loads(fixture.grove_env.read_text())
        self.assertLessEqual(set(driver), ALLOWED | {"CODEX_HOME"}, "the driver received a name outside the allow-list")
        launch = fixture.record()["launches"][0]
        self.assertEqual(launch["control"], [])
        self.assertNotIn("CODEX_PLANTED", launch["envNames"])
        self.assertNotIn("CODEX_THREAD_ID", launch["envNames"])

    def test_the_codex_driver_receives_only_the_allow_list(self):
        self.check_codex_allow_list(self.fixture)

        def leaky(environ, home, tmp, path_dirs):
            return {**environ, "HOME": str(home), "TMPDIR": str(tmp), "PATH": ":".join(path_dirs)}

        self.broken(self.check_codex_allow_list, "allowed_env", leaky)

    def check_codex_control_fires(self, fixture: Fixture):
        """A runner that passes the live session's CODEX_THREAD_ID: the name
        reaches the child with the live value, and the control fails the run."""
        real = trial.allowed_env

        def planting(environ, home, tmp, path_dirs):
            return {**real(environ, home, tmp, path_dirs), "CODEX_THREAD_ID": LIVE["CODEX_THREAD_ID"]}

        with mock.patch.object(trial, "allowed_env", planting):
            status, output = fixture.run_codex()
        self.assertEqual(status, 1, output)
        self.assertIn("CODEX_THREAD_ID reached the trial child with the live session's value", output)

    def test_a_planted_codex_name_fails_the_control_once(self):
        self.check_codex_control_fires(self.fixture)
        self.broken(self.check_codex_control_fires, "child_control", lambda child, live: [])

    def check_codex_links(self, fixture: Fixture):
        status, output = fixture.run_codex()
        self.assertEqual(status, 0, output)
        store = trial.CODEX.store
        links = [line for line in output.splitlines()
                 if line.startswith("harness-trial: link ") and not line.startswith("harness-trial: link after")]
        self.assertEqual(links, [f"harness-trial: link {store} -> {fixture.owner / store}"])
        self.assertEqual([line for line in output.splitlines() if "link after" in line],
                         [f"harness-trial: link after {store} -> {fixture.owner / store}"])
        for change in ({"replaceLink": True}, {"extraLink": True}):
            other = self.fresh()
            other.behave(**change)
            status, output = other.run_codex()
            self.assertEqual(status, 1, f"{change}: the run passed: {output}")
            self.assertIn("the sandbox home holds the links", output)
            self.assertEqual((other.owner / store).read_text(), "{}\n", "the real sign-in changed")

    def test_the_codex_link_list_holds_only_auth_json_before_and_after(self):
        self.check_codex_links(self.fixture)
        self.broken(self.check_codex_links, "check_links", lambda links, store, owner, when: None)

    def check_codex_events(self, fixture: Fixture):
        status, output = fixture.run_codex()
        self.assertEqual(status, 0, output)
        record = fixture.record()
        launch = record["launches"][0]
        self.assertEqual(launch["sessionId"][:8], "01a19ed6")
        self.assertEqual(len(launch["transcripts"]), 1)
        transcript = launch["transcripts"][0]
        self.assertRegex(transcript["file"], r"^\.codex/sessions/2026/11/03/rollout-.*-01a19ed6-.*\.jsonl$")
        events = transcript["events"]
        self.assertEqual([(e["tool"], e["class"], e.get("skill"), e["error"]) for e in events], CODEX_EXPECTED)
        snap = launch["sources"][0]["snapshotRoot"]
        # Each read names its absolute path, also a relative read in the
        # command's own working directory, and each command names both.
        self.assertEqual(events[1]["path"], os.path.join(snap, "skills/grove/references/execute.md"))
        self.assertEqual(events[1]["cwd"], snap)
        self.assertEqual(events[2]["path"], os.path.join(snap, "skills/grove-impl/SKILL.md"))
        for event in events:
            if event["tool"] == "exec_command":
                self.assertTrue(event["cwd"].startswith("/"), event)
                self.assertTrue(all(path.startswith("/") for path in event["paths"]), event)
        self.assertEqual(events[8]["path"], os.path.join(snap, "skills/craft/SKILL.md"))
        self.assertEqual(events[9]["path"], str(Path(record["trial"]) / "repo/smoke.txt"))
        self.assertEqual(record["ending"]["endingCommands"], ["grove-continue"])
        self.assertEqual(launch["streamResult"], "turn.completed")
        startup = transcript["startup"]
        self.assertEqual(startup["init"]["cli_version"], "9.9.9")
        self.assertEqual(startup["init"]["sandbox_policy"]["type"], "workspace-write")
        self.assertEqual(startup["tools"], ["functions.exec"])
        self.assertEqual(startup["skillListing"]["names"], ["imagegen"])
        self.assertEqual([m["sections"] for m in startup["developerMessages"]], [[], ["skills_instructions"]])
        self.assertTrue((Path(record["trial"]) / "prompt-input.json").is_file())
        markdown = (Path(record["trial"]) / "record.md").read_text()
        self.assertIn(f"in `{snap}`", markdown)

    def test_codex_extraction_orders_every_command_with_absolute_paths(self):
        self.check_codex_events(self.fixture)
        real = trial.CODEX.item
        self.broken(self.check_codex_events, "CODEX.item",
                    lambda item, n: [{k: v for k, v in e.items() if k != "cwd"} for e in real(item, n)])
        self.broken(self.check_codex_events, "separate_lines", lambda command: command)

    def check_codex_own_links(self, fixture: Fixture):
        fixture.behave(arg0Links=True)
        status, output = fixture.run_codex()
        self.assertEqual(status, 0, output)
        own = fixture.record()["links"]["harnessOwn"]
        self.assertEqual(sorted(Path(link).name for link, _ in own), ["apply_patch", "applypatch"])
        self.assertEqual({target for _, target in own}, {os.path.realpath(fixture.codex)})
        foreign = self.fresh()
        foreign.behave(arg0Foreign=True)
        status, output = foreign.run_codex()
        self.assertEqual(status, 1, output)
        self.assertIn("codex-arg0Fake/other", output)

    def test_codex_helper_links_to_its_own_program_pass_and_others_fail(self):
        self.check_codex_own_links(self.fixture)
        self.broken(self.check_codex_own_links, "CODEX.own_links", lambda links, program: (links, []))

    def check_codex_stream(self, fixture: Fixture):
        """Without a session file, the events come from the stream."""
        fixture.behave(noSession=True)
        status, output = fixture.run_codex()
        self.assertEqual(status, 0, output)
        transcript = fixture.record()["launches"][0]["transcripts"][0]
        self.assertEqual(transcript["file"], "stream")
        classes = [e["class"] for e in transcript["events"]]
        self.assertEqual(classes[:1], ["mandatory"])
        self.assertIn("ending", classes)

    def test_codex_extraction_falls_back_to_the_stream(self):
        self.check_codex_stream(self.fixture)
        self.broken(self.check_codex_stream, "CODEX.entries", lambda records: [])

    def check_codex_config_directory(self, fixture: Fixture):
        fixture.behave(skillRoot=str(fixture.owner / ".codex/skills/.system"))
        status, output = fixture.run_codex()
        self.assertEqual(status, 1, output)
        self.assertIn("Codex reports the skill roots", output)
        self.assertFalse(fixture.grove_env.exists(), "the driver launched after the failed check")

    def test_a_codex_configuration_directory_elsewhere_refuses(self):
        self.check_codex_config_directory(self.fixture)
        self.broken(self.check_codex_config_directory, "CODEX.config_check", lambda rendered, config: "unchecked")

    def check_codex_residue(self, fixture: Fixture):
        status, output = fixture.run_codex()
        self.assertEqual(status, 0, output)
        skills = fixture.record()["skills"]
        self.assertEqual(skills["nativeCopies"], [])
        self.assertIn(".codex/skills/.system/imagegen/SKILL.md", skills["residue"])
        self.assertIn(".codex/plugins/cache/openai-curated-remote/pages/0.1.26/skills/write-page/SKILL.md",
                      skills["residue"])
        stray = self.fresh()
        stray.behave(strayNative=True)
        status, output = stray.run_codex()
        self.assertEqual(status, 1, output)
        self.assertIn(".codex/skills/stray/SKILL.md", output)
        # A declared native copy lies in the sandbox .codex/skills.
        stale = self.fresh()
        status, output = stale.run_codex("stale")
        self.assertEqual(status, 0, output)
        self.assertEqual(stale.record()["skills"]["nativeCopies"], [".codex/skills/craft/SKILL.md"])

    def test_codex_residue_is_no_native_copy(self):
        self.check_codex_residue(self.fixture)

        def claude_only(home):
            native, residue = [], []
            for path in sorted(home.rglob("SKILL.md")):
                relative = str(path.relative_to(home))
                if ".cache/ravel" not in relative:
                    (residue if "skills/synced" in relative else native).append(relative)
            return native, residue, []

        self.broken(self.check_codex_residue, "skill_files", claude_only)

    def check_codex_account_data(self, fixture: Fixture):
        status, output = fixture.run_codex()
        self.assertEqual(status, 0, output)
        trial_dir = Path(fixture.record()["trial"])
        for name in ("record.json", "record.md"):
            text = (trial_dir / name).read_text()
            for secret in ("secret-account", "secret-user", "secret-balance"):
                self.assertNotIn(secret, text, f"{secret} reached {name}")
        harness = fixture.record()["harness"]
        self.assertEqual(harness["name"], "codex")
        self.assertEqual(harness["caseHarness"], "claude")
        self.assertEqual(harness["model"], trial.CODEX_MODEL)
        self.assertEqual(harness["sandboxMode"], "workspace-write")
        self.assertEqual(harness["loginStatus"], "Logged in using ChatGPT")
        self.assertEqual(harness["authConfigDirectory"], str(trial_dir / "home/.codex/skills/.system"))
        self.assertEqual(harness["prelaunch"]["skillListing"]["roots"], {"r0": harness["authConfigDirectory"]})
        self.assertIn("Codex input before the launch", (trial_dir / "record.md").read_text())

    def test_the_codex_record_keeps_no_account_data(self):
        self.check_codex_account_data(self.fixture)
        self.broken(self.check_codex_account_data, "CODEX.startup",
                    lambda records: {"init": next(r["payload"] for r in records if r["type"] == "session_meta")})

    def check_codex_later_fields(self, fixture: Fixture):
        for case, field in (("explicit", "claudeSettings"), ("plugins", "nativePlugins")):
            with self.assertRaises(trial.TrialError, msg=f"{case} loaded for Codex") as caught:
                trial.load_case(case, "codex")
            self.assertIn(f"'{field}'", str(caught.exception))
        trial.load_case("delegate", "codex")
        trial.load_case("compaction-path", "claude")
        trial.load_case("compaction-path", "codex")
        trial.load_case("continuation", "codex")

    def test_codex_refuses_the_fields_of_later_leaves(self):
        self.check_codex_later_fields(self.fixture)
        self.broken(self.check_codex_later_fields, "CODEX.unsupported", {})

    def test_the_codex_configuration_selects_codex_exec(self):
        case = {"kinds": ["impl"], "name": "x", "harness": "codex"}
        route = trial.route_for(case, Path("/r"), Path("/c"), "gpt-x", "low")
        config = trial.config_for(case, route)
        self.assertIn('"exec", "--json", "--skip-git-repo-check"', config)
        self.assertIn('"--sandbox", "workspace-write", "-c", "approval_policy=\\"never\\""', config)
        self.assertIn('"-c", "model_reasoning_effort=\\"low\\""', config)
        self.assertIn('provider: "openai"', config)
        self.assertNotIn('"--add-dir"', config)
        self.assertIn("args.push(request.prompt);", config)
        with self.assertRaises(trial.TrialError):
            trial.load_case("smoke", "gemini")
        # A compaction case runs the feeder, with `codex app-server` as its
        # command. Only the preamble case enables token_budget.
        for mode in ("path", "preamble"):
            case = {"kinds": ["impl"], "name": "x", "harness": "codex", "compaction": {"mode": mode}}
            route = trial.route_for(case, Path("/t/records"), Path("/c"), "gpt-x", "low")
            config = trial.config_for(case, route)
            self.assertEqual(route["program"], sys.executable)
            self.assertIn('"turns", "--plan", "/t/turns.json", "--", "/c"', config)
            self.assertIn('"app-server", "-c", "model=\\"gpt-x\\""', config)
            self.assertIn('"-c", "sandbox_mode=\\"workspace-write\\""', config)
            self.assertNotIn('"--add-dir"', config)
            self.assertEqual('"--enable", "token_budget"' in config, mode == "preamble")

    # --- Codex compaction ---------------------------------------------------------

    def codex_compact(self, fixture: Fixture, mode: str, **behaviour) -> tuple[int, str, dict]:
        fixture.behave(codexFirst=CODEX_FIRST, codexAfter=CODEX_AFTER, **behaviour)
        status, output = fixture.run_codex(f"compaction-{mode}")
        return status, output, fixture.record()

    def check_codex_compaction(self, fixture: Fixture):
        # The path case: the default mechanism keeps the prompt and adds an
        # encrypted item, so the gate withholds the answer. It runs first,
        # because the fixture reads the record of the last run by name.
        status, output, record = self.codex_compact(fixture, "path")
        self.assertEqual(status, 1, output)
        compaction = record["compaction"]
        self.assertFalse(compaction["gate"]["ok"])
        self.assertEqual([turn["turn"] for turn in compaction["turns"]], [0, 1])
        self.assertTrue([item for item in compaction.get("history", []) if item.get("role") == "user" and item["preamble"]])
        self.assertTrue([item for item in compaction.get("history", []) if "encryptedChars" in item])
        self.assertEqual((compaction.get("afterBoundary") or {}).get("opaque"), 1)
        self.assertIn("encrypted items", output)
        # The preamble case: token_budget keeps no user message, so the gate
        # passes, and the answer follows in the same launch.
        status, output, record = self.codex_compact(fixture, "preamble")
        self.assertEqual(status, 0, output)
        compaction = record["compaction"]
        self.assertEqual(compaction["failures"], [])
        self.assertTrue(compaction["gate"]["ok"], compaction["gate"])
        self.assertEqual([turn["turn"] for turn in compaction["turns"]], [0, 1, 2])
        self.assertEqual(compaction["turns"][1]["request"], "thread/compact/start")
        self.assertIn("token_budget", compaction["mechanism"])
        self.assertFalse(any(item["preamble"] for item in compaction.get("history", [None])))
        context = compaction.get("context") or {"findings": {}, "added": [{}]}
        self.assertEqual(context["findings"].get("opaque"), 0)
        self.assertEqual((context["added"] or [{}])[-1].get("firstLine"), "Yes, add the label.")
        self.assertEqual(compaction["variable"], {"set": True, "descriptor": True})
        self.assertEqual(len(record["launches"]), 1)
        events = next(t for t in record["launches"][0]["transcripts"] if t["main"])["events"]
        self.assertEqual([(e["class"], e.get("skill")) for e in events], CODEX_COMPACTION_EXPECTED)
        trial_dir = Path(record["trial"])
        route = json.loads((trial_dir / "route.json").read_text())
        self.assertEqual((route["fixedArgs"][0], route["fixedArgs"][-2:]), ("app-server", ["--enable", "token_budget"]))
        self.assertEqual(json.loads((trial_dir / "turns.json").read_text())["harness"], "codex")
        markdown = (trial_dir / "record.md").read_text()
        for needle in ("Compaction:", "Gate before the answer: passed", "Replacement history", "do not reach Codex"):
            self.assertIn(needle, markdown)
        # A retained prompt fails the gate, and no answer follows.
        status, output, record = self.codex_compact(fixture, "preamble", keepPrompt=True)
        self.assertEqual(status, 1, output)
        self.assertFalse(record["compaction"]["gate"]["ok"])
        self.assertEqual([turn["turn"] for turn in record["compaction"]["turns"]], [0, 1])
        self.assertTrue((record["compaction"].get("afterBoundary") or {}).get("paths"), "the in-process scan missed the path")

    def test_a_codex_compaction_runs_the_app_server_and_reads_the_replacement_history(self):
        self.check_codex_compaction(self.fixture)
        self.broken(self.check_codex_compaction, "CODEX.compact_turn", lambda spec: {"text": "/compact x"})
        real_launch = trial.CODEX.launch
        self.broken(self.check_codex_compaction, "CODEX.launch",
                    lambda case, records, program, model, effort: real_launch(
                        {**case, "compaction": {**case["compaction"], "mode": "path"}}, records, program, model, effort))
        self.broken(self.check_codex_compaction, "CODEX.after_compaction",
                    lambda records, marks: trial.context_findings("", "", {"paths": [], "digests": [], "lines": []}))
        self.broken(self.check_codex_compaction, "CODEX.model_items", lambda records, start, answer: [])
        self.broken(self.check_codex_compaction, "CODEX.stream_ids", lambda stream: (None, None))


if __name__ == "__main__":
    unittest.main(verbosity=2)
