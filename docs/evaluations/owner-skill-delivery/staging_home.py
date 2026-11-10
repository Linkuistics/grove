#!/usr/bin/env python3
"""Build the retained staging HOME of the owner cutover, launch staging
sessions in it, and run its isolation trial.

"Settings isolation and the staging HOME" of docs/specs/owner-skill-delivery.md
states the contract. The staging HOME holds its own Ravel configuration, the
staging settings with the ten sources, and harness configuration that
prototypes the native residue. It reuses only the sign-in store of each
harness: a link to `Library/Keychains` for Claude Code and a link to
`.codex/auth.json` for Codex.

    staging_home.py build --home HOME --candidate KEG --ravel PREFIX
    staging_home.py links --home HOME
    staging_home.py launch --home HOME --ravel PREFIX --harness claude|codex --out DIR --prompt TEXT
    staging_home.py isolation --home HOME --candidate KEG --ravel PREFIX --out DIR
    staging_home.py mcp-entries --home HOME

`build` refuses an existing HOME. It copies the owner's `config.rhai`, writes
docs/owner-cutover/settings-staging.json, derives each harness configuration
from the real one, and registers the skill-free plugins and `guardrail` with
Claude Code's own commands. It installs no superseded skill and does not
register `writing-hooks`. It then runs the step of `mcp-entries`.

`mcp-entries` makes two changes to the MCP entries, each in the staging HOME
only. First, it sets the cache folder of each staging entry of
`codebase-memory-mcp` to the account daemon's folder, `~/.cache/codebase-memory-mcp`
of the real HOME. The server is one daemon per account, and a client whose
cache folder differs from the daemon's refuses to start. A client in the
staging HOME derives another folder from HOME. Claude Code's entry gets
`env.CBM_CACHE_DIR`. Codex's entry gets `env = { CBM_CACHE_DIR = ... }` in
place of its `env_vars` line, because the staging launcher passes neither
forwarded name. Second, it adds `default_tools_approval_mode = "approve"`
to Codex's `quint-kb` entry. A staging Codex session runs with the approval
policy `never`, and Codex 0.167.0 denies each MCP call that needs approval
under that policy. The owner's file approves each `codebase-memory-mcp` tool
and no `quint-kb` tool, so this second change departs from the owner's file.
A second run changes nothing.

`launch` is the staging launcher. It refuses each of Claude Code's five
location variables and any inherited `CODEX_HOME` other than the staging
`.codex` before anything runs. Each session runs in a new directory under
/private/tmp. Claude Code reads `.claude/skills` in every parent folder of its
working directory, so a working directory in the real HOME, the staging HOME
included, would reach the real native skills. The launcher refuses one.
It sets HOME and CODEX_HOME to the staging HOME, passes an allow-listed
environment, and runs `ravel start-session` of the accepted 2.0 build with a
headless trial script. The record names the environment, the argv and the
result.

`isolation` runs the five steps of the isolation trial and writes
OUT/record.json. Step 4 digests the real configuration before `build` and
after the sessions, so `build` itself runs inside the measured interval. A
Codex session, an app-server's `account/read` with `refreshToken`, refreshes
the token in step 5.

The real HOME is HOME of the calling environment. No subcommand writes it,
except the token refresh of step 5, which Codex writes through the link.
"""

from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
import time
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "scripts"))
import harness_trial  # noqa: E402  (the shared adapters of the real trials)

SETTINGS = ROOT / "docs" / "owner-cutover" / "settings-staging.json"
LOCATION_NAMES = harness_trial.LOCATION_NAMES
SIGN_IN_LINKS = ("Library/Keychains", ".codex/auth.json")
SYSTEM_PATH = ("/usr/bin", "/bin", "/usr/sbin", "/sbin")
PASSED_NAMES = ("USER", "LOGNAME", "LANG", "LC_ALL", "LC_CTYPE", "TERM")
CLAUDE_MODEL, CLAUDE_EFFORT = "claude-sonnet-5-5", "low"
CODEX_MODEL, CODEX_EFFORT = "gpt-6.1-sol", "low"
# The skill-free plugins of the native residue ("Native residue" of the owner
# design), from the official marketplace.
RESIDUE_PLUGINS = ("commit-commands", "context7", "explanatory-output-style", "feature-dev", "playwright",
                   "swift-lsp")
OFFICIAL = ("claude-plugins-official", "anthropics/claude-plugins-official")
# Real Claude Code settings keys that the staging copy replaces.
REPLACED_KEYS = ("enabledPlugins", "extraKnownMarketplaces")
# Codex configuration sections that name real paths, real trust or platform
# plugins. The staging copy drops them; Codex installs its platform plugins.
DROPPED_CODEX_SECTIONS = re.compile(r'^\[(projects\.|hooks\.state|marketplaces\.|plugins\.|desktop)')
DROPPED_CODEX_KEYS = re.compile(r"^notify\s*=")


class StagingError(Exception):
    pass


def now() -> str:
    return dt.datetime.now(dt.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def real_home() -> Path:
    return Path(os.environ["HOME"]).resolve()


def inside(path: Path, root: Path) -> bool:
    path, root = Path(os.path.realpath(path)), Path(os.path.realpath(root))
    return path == root or root in path.parents


def staging_sources() -> list[dict[str, Any]]:
    return json.loads(SETTINGS.read_text())["skills"]["sources"]


# --- Build -------------------------------------------------------------------


def check_placement(home: Path) -> None:
    """The staging HOME contains no source, and no source contains it."""
    for source in staging_sources():
        path = Path(source["path"])
        if inside(path, home) or inside(home, path):
            raise StagingError(f"the staging HOME {home} and the source {source['id']} at {path} overlap")
    if inside(home, real_home() / ".claude") or inside(home, real_home() / ".codex"):
        raise StagingError(f"the staging HOME {home} lies inside a real harness configuration")


def staged_env(home: Path, ravel: Path | None = None) -> dict[str, str]:
    """The allow-listed environment of every staging command."""
    path = [str(ravel / "bin")] if ravel else []
    path += ["/opt/homebrew/bin", str(real_home() / ".local" / "bin"), *SYSTEM_PATH]
    env = {name: os.environ[name] for name in PASSED_NAMES if name in os.environ}
    env.update({"HOME": str(home), "CODEX_HOME": str(home / ".codex"), "PATH": ":".join(path),
                "TMPDIR": os.environ.get("TMPDIR", "/tmp"), "DISABLE_AUTOUPDATER": "1"})
    return env


def run(argv: list[str], env: dict[str, str], cwd: Path, log: list[dict[str, Any]], check: bool = True,
        stdin: str | None = None, timeout: int = 900) -> subprocess.CompletedProcess[str]:
    started = now()
    done = subprocess.run(argv, env=env, cwd=cwd, capture_output=True, text=True, input=stdin, timeout=timeout,
                          check=False)
    log.append({"argv": argv, "cwd": str(cwd), "started": started, "exit": done.returncode,
                "stdout": done.stdout[-4000:], "stderr": done.stderr[-4000:]})
    if check and done.returncode != 0:
        raise StagingError(f"{' '.join(argv)} exited {done.returncode}: {done.stderr.strip()[-800:]}")
    return done


def claude_settings(real: Path, home: Path) -> dict[str, Any]:
    """The real user settings with the delivery keys replaced and every real
    HOME path in a command moved to the staging HOME."""
    settings = json.loads((real / ".claude" / "settings.json").read_text())
    for key in REPLACED_KEYS:
        settings.pop(key, None)
    settings["enabledPlugins"] = {f"{name}@{OFFICIAL[0]}": True for name in RESIDUE_PLUGINS}
    settings["enabledPlugins"]["guardrail@personal"] = True
    text = json.dumps(settings).replace(str(real / ".claude") + "/", str(home / ".claude") + "/")
    return json.loads(text)


def codex_config(real: Path, home: Path) -> str:
    """The real config.toml without real trust, hook state, platform plugins
    and notify, with real `.codex` paths moved to the staging tree."""
    kept, dropping = [], False
    for line in (real / ".codex" / "config.toml").read_text().splitlines():
        if line.startswith("["):
            dropping = bool(DROPPED_CODEX_SECTIONS.match(line))
        if dropping or DROPPED_CODEX_KEYS.match(line):
            continue
        kept.append(line)
    return "\n".join(kept).replace(str(real / ".codex") + "/", str(home / ".codex") + "/") + "\n"


def build(home: Path, candidate: Path, log: list[dict[str, Any]]) -> dict[str, Any]:
    real = real_home()
    if home.exists() or home.is_symlink():
        raise StagingError(f"{home} exists; name a new directory")
    if not (candidate / "brew" / "share" / "ravel" / "skills").is_dir():
        raise StagingError(f"{candidate} holds no candidate keg")
    check_placement(home)
    home.mkdir(parents=True)
    (home / "Library").mkdir()
    os.symlink(real / "Library" / "Keychains", home / "Library" / "Keychains")
    claude, codex = home / ".claude", home / ".codex"
    claude.mkdir()
    codex.mkdir()
    os.symlink(real / ".codex" / "auth.json", codex / "auth.json")

    ravel_dir = home / ".config" / "ravel"
    ravel_dir.mkdir(parents=True)
    shutil.copy2(real / ".config" / "ravel" / "config.rhai", ravel_dir / "config.rhai")
    shutil.copyfile(SETTINGS, ravel_dir / "settings.json")

    # Claude Code: settings, tool-managed hooks, agents and skill copy, the
    # status line, and the owner's MCP entries in .claude.json.
    (claude / "settings.json").write_text(json.dumps(claude_settings(real, home), indent=2) + "\n")
    for name in ("hooks", "agents"):
        shutil.copytree(real / ".claude" / name, claude / name, symlinks=True)
    (claude / "skills").mkdir()
    shutil.copytree(real / ".claude" / "skills" / "codebase-memory", claude / "skills" / "codebase-memory",
                    symlinks=True)
    shutil.copy2(real / ".claude" / "statusline-command.sh", claude / "statusline-command.sh")
    owner = json.loads((real / ".claude.json").read_text())
    (home / ".claude.json").write_text(json.dumps({
        "hasCompletedOnboarding": True, "autoUpdates": False, "theme": owner.get("theme", "dark"),
        "mcpServers": owner.get("mcpServers", {}),
    }, indent=2) + "\n")

    # Codex: configuration, hooks with their helper, and global instructions.
    (codex / "config.toml").write_text(codex_config(real, home))
    (codex / "config.toml").chmod(0o600)
    hooks = (real / ".codex" / "hooks.json").read_text()
    (codex / "hooks.json").write_text(hooks.replace(str(real / ".codex") + "/", str(codex) + "/"))
    for name in ("herdr-agent-state.sh", "AGENTS.md", "instructions.md"):
        if (real / ".codex" / name).exists():
            shutil.copy2(real / ".codex" / name, codex / name)

    stage_mcp_entries(home, real)

    # Claude Code's own commands register the residue plugins and guardrail.
    env = staged_env(home)
    claude_bin = shutil.which("claude", path=env["PATH"])
    if not claude_bin:
        raise StagingError("no claude on the staging PATH")
    known = run([claude_bin, "plugin", "marketplace", "list", "--json"], env, home, log, check=False)
    if OFFICIAL[0] not in known.stdout:
        run([claude_bin, "plugin", "marketplace", "add", OFFICIAL[1]], env, home, log)
    for name in RESIDUE_PLUGINS:
        run([claude_bin, "plugin", "install", f"{name}@{OFFICIAL[0]}", "-s", "user"], env, home, log)
    personal = Path("/opt/homebrew/opt/personal")
    run([claude_bin, "plugin", "marketplace", "add", str(personal)], env, home, log)
    run([claude_bin, "plugin", "install", "guardrail@personal", "-s", "user"], env, home, log)
    plugins = run([claude_bin, "plugin", "list", "--json"], env, home, log, check=False)
    return {"home": str(home), "candidate": str(candidate), "pluginList": plugins.stdout,
            "configRhaiSha256": sha256_file(ravel_dir / "config.rhai"),
            "settingsSha256": sha256_file(ravel_dir / "settings.json")}


CBM_SERVER = "codebase-memory-mcp"
CBM_HEADER = f"[mcp_servers.{CBM_SERVER}]"
# The server-wide approval key of an MCP entry in Codex 0.167.0:
# https://github.com/openai/codex/blob/rust-v0.167.0/codex-rs/config/src/mcp_types.rs#L287-L289
# Under the policy `never`, a call that needs approval is denied:
# https://github.com/openai/codex/blob/rust-v0.167.0/codex-rs/core/src/mcp_tool_call.rs#L1691-L1699
QUINT_SERVER = "quint-kb"
QUINT_HEADER = f"[mcp_servers.{QUINT_SERVER}]"
QUINT_APPROVAL = 'default_tools_approval_mode = "approve"'


def cbm_cache(real: Path) -> str:
    return str(real / ".cache" / CBM_SERVER)


def replace_file(path: Path, text: str) -> None:
    """Replace PATH with TEXT through a rename, and keep its mode."""
    tmp = path.with_name(path.name + ".mcp-entries")
    tmp.write_text(text)
    shutil.copymode(path, tmp)
    os.rename(tmp, path)


def join_cbm(home: Path, real: Path) -> list[str]:
    """Name the account daemon's cache folder in each staging entry of
    codebase-memory-mcp. Returns the files that changed, relative to HOME.
    Refuses an entry that it cannot read or a result that does not parse."""
    import tomllib

    cache, changed = cbm_cache(real), []
    claude_json = home / ".claude.json"
    doc = json.loads(claude_json.read_text())
    entry = (doc.get("mcpServers") or {}).get(CBM_SERVER)
    if entry is not None and (entry.get("env") or {}).get("CBM_CACHE_DIR") != cache:
        entry.setdefault("env", {})["CBM_CACHE_DIR"] = cache
        replace_file(claude_json, json.dumps(doc, indent=2) + "\n")
        changed.append(".claude.json")
    config = home / ".codex" / "config.toml"
    text = config.read_text()
    servers = tomllib.loads(text).get("mcp_servers") or {}
    if CBM_SERVER in servers:
        if (servers[CBM_SERVER].get("env") or {}).get("CBM_CACHE_DIR") != cache \
                or "env_vars" in servers[CBM_SERVER]:
            lines, inside, done = text.splitlines(), False, False
            out = []
            for line in lines:
                if line.startswith("["):
                    inside = line.strip() == CBM_HEADER
                    out.append(line)
                    if inside and not done:
                        out.append(f"env = {{ CBM_CACHE_DIR = {json.dumps(cache)} }}")
                        done = True
                    continue
                if inside and re.match(r"\s*(env_vars|env)\s*=", line):
                    continue
                out.append(line)
            joined = "\n".join(out) + "\n"
            entry = (tomllib.loads(joined).get("mcp_servers") or {}).get(CBM_SERVER) or {}
            if not done or entry.get("env") != {"CBM_CACHE_DIR": cache} or "env_vars" in entry:
                raise StagingError(f"{config}: the {CBM_SERVER} entry did not take the cache folder")
            replace_file(config, joined)
            changed.append(".codex/config.toml")
    return changed


def approve_quint(home: Path) -> list[str]:
    """Add the server-wide approval to Codex's quint-kb entry of the staging
    HOME. Returns the files that changed, relative to HOME."""
    import tomllib

    config = home / ".codex" / "config.toml"
    text = config.read_text()
    entry = (tomllib.loads(text).get("mcp_servers") or {}).get(QUINT_SERVER)
    if entry is None or entry.get("default_tools_approval_mode") == "approve":
        return []
    out, inside = [], False
    for line in text.splitlines():
        if line.startswith("["):
            inside = line.strip() == QUINT_HEADER
            out.append(line)
            if inside:
                out.append(QUINT_APPROVAL)
            continue
        if inside and re.match(r"\s*default_tools_approval_mode\s*=", line):
            continue
        out.append(line)
    joined = "\n".join(out) + "\n"
    if ((tomllib.loads(joined).get("mcp_servers") or {}).get(QUINT_SERVER) or {}).get(
            "default_tools_approval_mode") != "approve":
        raise StagingError(f"{config}: the {QUINT_SERVER} entry did not take the approval")
    replace_file(config, joined)
    return [".codex/config.toml"]


def stage_mcp_entries(home: Path, real: Path) -> dict[str, list[str]]:
    """The two staging changes of the MCP entries, as `mcp-entries` states."""
    return {"cbmCache": join_cbm(home, real), "quintApproval": approve_quint(home)}


# --- Observation -------------------------------------------------------------


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def links(home: Path) -> list[dict[str, str]]:
    """Every link under HOME, without following one, with its text and
    whether it resolves inside HOME."""
    found = []
    for directory, names, files in os.walk(home):
        for name in sorted(names + files):
            path = Path(directory) / name
            if path.is_symlink():
                target = os.readlink(path)
                found.append({"path": str(path.relative_to(home)), "target": target,
                              "resolvesInside": inside(path.resolve(), home)})
    return sorted(found, key=lambda link: link["path"])


def classify_links(home: Path) -> dict[str, Any]:
    """Every link under HOME. Only the two sign-in links may lead out of it.
    A link that a tool created and that resolves inside HOME joins nothing,
    so it is listed with its owner instead."""
    found = links(home)
    sign_in = [link for link in found if link["path"] in SIGN_IN_LINKS]
    leaving = [link for link in found if link not in sign_in and not link["resolvesInside"]]
    internal = []
    for link in found:
        if link in sign_in or link in leaving:
            continue
        owner = "unknown"
        if "/node_modules/.bin/" in link["path"] and link["path"].startswith(".npm/"):
            owner = "npm (.bin link of an npx package)"
        elif link["path"].startswith(".codex/"):
            owner = "Codex"
        internal.append({**link, "owner": owner})
    return {"all": found, "signIn": sign_in, "leaving": leaving, "internal": internal,
            "signInComplete": sorted(link["path"] for link in sign_in) == sorted(SIGN_IN_LINKS),
            "onlySignIn": not leaving and not internal, "onlySignInLeaves": not leaving,
            "unknownInternal": [link for link in internal if link["owner"] == "unknown"]}


def check_cwd(cwd: Path, home: Path) -> None:
    """A staging session's working directory has no harness folder above it
    outside the staging HOME. Claude Code reads `.claude/skills` in every
    parent folder of its working directory, so a working directory inside
    the real HOME reaches the real native skills."""
    for parent in [cwd, *cwd.parents]:
        if inside(parent, home):
            continue
        for name in (".claude", ".agents", ".codex"):
            if (parent / name).exists():
                raise StagingError(f"the working directory {cwd} lies below {parent / name}, which a harness "
                                   f"reads as project configuration; use a directory outside the real HOME")


def cwd_refusals(home: Path) -> dict[str, str]:
    """The control of check_cwd: a directory in the real HOME and one in the
    staging HOME must refuse, and a new one under /private/tmp must not."""
    results = {}
    for label, cwd in (("real HOME", real_home() / "Development"), ("staging HOME", home / ".cache"),
                       ("/private/tmp", Path("/private/tmp"))):
        try:
            check_cwd(cwd, home)
            results[label] = "admitted"
        except StagingError as error:
            results[label] = f"refused: {error}"
    return results


def work_directory(home: Path) -> Path:
    work = Path(tempfile.mkdtemp(prefix="owner-staging-", dir="/private/tmp"))
    check_cwd(work, home)
    return work


def tree_digest(path: Path) -> dict[str, Any]:
    """A digest of a file, a link or a whole tree, without following links."""
    if path.is_symlink():
        return {"kind": "link", "target": os.readlink(path)}
    if not path.exists():
        return {"kind": "absent"}
    if path.is_file():
        return {"kind": "file", "sha256": sha256_file(path)}
    digest, count = hashlib.sha256(), 0
    for directory, names, files in os.walk(path):
        names.sort()
        for name in sorted(names + files):
            entry = Path(directory) / name
            rel = str(entry.relative_to(path))
            if entry.is_symlink():
                digest.update(f"L {rel} {os.readlink(entry)}\n".encode())
            elif entry.is_file():
                digest.update(f"F {rel} {sha256_file(entry)}\n".encode())
            elif entry.is_dir():
                digest.update(f"D {rel}\n".encode())
            else:
                digest.update(f"O {rel}\n".encode())
            count += 1
    return {"kind": "tree", "sha256": digest.hexdigest(), "entries": count}


# The real configuration files, hook files, plugin registries and skill
# roots of step 4. auth.json is the sign-in store, which step 5 refreshes.
REAL_SUBJECTS = (
    ".claude/settings.json", ".claude/settings.local.json", ".claude.json", ".claude/hooks", ".claude/agents",
    ".claude/plugins/installed_plugins.json", ".claude/plugins/known_marketplaces.json",
    ".claude/plugins/marketplaces", ".claude/plugins/cache", ".claude/skills",
    ".codex/config.toml", ".codex/hooks.json", ".codex/herdr-agent-state.sh", ".codex/AGENTS.md",
    ".codex/plugins", ".codex/skills", ".agents/skills", ".config/ravel",
)


def real_digests() -> dict[str, Any]:
    real = real_home()
    return {rel: tree_digest(real / rel) for rel in REAL_SUBJECTS}


def json_key_changes(before: dict[str, Any], after: dict[str, Any]) -> list[str]:
    return sorted(key for key in set(before) | set(after) if before.get(key) != after.get(key))


def auth_facts() -> dict[str, Any]:
    path = real_home() / ".codex" / "auth.json"
    data = json.loads(path.read_text())
    return {"lastRefresh": data.get("last_refresh"), "sha256": sha256_file(path), "isLink": path.is_symlink(),
            "mode": oct(path.stat().st_mode & 0o777)}


# --- Launch ------------------------------------------------------------------


def refuse_inherited(environ: dict[str, str], home: Path) -> None:
    """The staging launcher's refusals, before anything runs. An inherited
    CODEX_HOME other than the staging `.codex` refuses, because the staging
    HOME itself lies inside the real HOME."""
    for name in LOCATION_NAMES:
        if name in environ:
            raise StagingError(f"the environment holds {name}, which can expose a real Claude Code configuration, "
                               f"plugin tree or sign-in; launch again without {name}")
    if "CODEX_HOME" in environ:
        value = environ["CODEX_HOME"]
        if not value or os.path.realpath(value) != os.path.realpath(home / ".codex"):
            raise StagingError(f"the environment holds CODEX_HOME={value!r}, which is not the staging "
                               f"{home / '.codex'}; launch again without it")


def trial_script(harness: str, model: str, effort: str) -> str:
    """A headless selection script for one harness, as the real trials use."""
    if harness == "claude":
        program, fixed = "claude", ["-p", "--output-format", "stream-json", "--verbose", "--model", model,
                                    "--effort", effort, "--permission-mode", "bypassPermissions"]
        add_dirs = ('    if request.skills != () { for source in request.skills.sources '
                    '{ args += ["--add-dir", source.snapshotRoot]; } }\n')
    else:
        program, fixed = "codex", ["exec", "--json", "--skip-git-repo-check", "--color", "never", "--model", model,
                                   "-c", f'model_reasoning_effort="{effort}"', "--sandbox",
                                   harness_trial.CODEX_SANDBOX, "-c",
                                   f'approval_policy="{harness_trial.CODEX_APPROVAL}"']
        add_dirs = ""
    provider = "anthropic" if harness == "claude" else "openai"
    quoted = ", ".join(json.dumps(arg) for arg in fixed)
    return (f"// Written by staging_home.py for one staging {harness} launch.\n"
            "export const SCHEMA_VERSION = 4;\n"
            f'export const VERSION = "staging-{harness}";\n\n'
            "fn select(request, context) {\n"
            "    let args = [];\n" + add_dirs +
            f"    args += [{quoted}];\n"
            "    args.push(request.prompt);\n"
            f'    #{{ status: "selected", program: {json.dumps(program)}, args: args, provider: "{provider}", '
            f'model: {json.dumps(model)}, effort: {json.dumps(effort)}, reason: "staging launch" }}\n'
            "}\n")


def launch(home: Path, ravel: Path, harness: str, out: Path, prompt: str, environ: dict[str, str],
           model: str | None = None, effort: str | None = None) -> dict[str, Any]:
    refuse_inherited(environ, home)
    out.mkdir(parents=True, exist_ok=False)
    model = model or (CLAUDE_MODEL if harness == "claude" else CODEX_MODEL)
    effort = effort or (CLAUDE_EFFORT if harness == "claude" else CODEX_EFFORT)
    (out / "trial.rhai").write_text(trial_script(harness, model, effort))
    work = work_directory(home)
    env = staged_env(home, ravel)
    argv = [str(ravel / "bin" / "ravel"), "start-session", "--kind", f"staging-{harness}", "--config",
            str(out / "trial.rhai"), "--prompt", prompt, "--json"]
    started = now()
    done = subprocess.run(argv, env=env, cwd=work, capture_output=True, text=True, timeout=1800, check=False)
    (out / "stream.jsonl").write_text(done.stderr)
    record = {"harness": harness, "model": model, "effort": effort, "argv": argv, "cwd": str(work),
              "env": env, "started": started, "ended": now(), "exit": done.returncode, "result": done.stdout,
              "configScript": (out / "trial.rhai").read_text()}
    (out / "launch.json").write_text(json.dumps(record, indent=2) + "\n")
    return record


def stream_events(text: str) -> list[dict[str, Any]]:
    events = []
    for line in text.splitlines():
        line = line.strip()
        if line.startswith("{"):
            try:
                events.append(json.loads(line))
            except json.JSONDecodeError:
                pass
    return events


# --- Isolation trial ---------------------------------------------------------


def superseded_names() -> set[str]:
    """Every skill name of the ten staging sources, and the stale native names."""
    names = {"grove", "terminal-browser"}
    for source in staging_sources():
        for root in source["roots"]:
            base = Path(source["path"]) / root
            if base.is_dir():
                names.update(entry.name for entry in base.iterdir() if (entry / "SKILL.md").exists())
    return names


def codex_listing(home: Path, log: list[dict[str, Any]]) -> dict[str, Any]:
    env = staged_env(home)
    codex_bin = shutil.which("codex", path=env["PATH"]) or "codex"
    work = work_directory(home)
    login = run([codex_bin, "login", "status"], env, work, log, check=False)
    rendered = run([codex_bin, "debug", "prompt-input"], env, work, log)
    listing = harness_trial.CodexAdapter().input(json.loads(rendered.stdout))["skillListing"] or {}
    return {"loginStatus": (login.stdout + login.stderr).strip(), "loginExit": login.returncode,
            "listing": listing}


def listed_names(listing: dict[str, Any]) -> set[str]:
    return {name.strip() for name in listing.get("names") or [] if name.strip()}


def refresh_token(home: Path, out: Path) -> dict[str, Any]:
    """One Codex app-server session in the staging CODEX_HOME that asks for a
    proactive token refresh (`account/read` with `refreshToken`, the managed
    refresh flow of codex-rs/app-server-protocol/src/protocol/v2/account.rs at
    rust-v0.167.0)."""
    env = staged_env(home)
    codex_bin = shutil.which("codex", path=env["PATH"]) or "codex"
    child = subprocess.Popen([codex_bin, "app-server"], env=env, cwd=work_directory(home), stdin=subprocess.PIPE,
                             stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    assert child.stdin and child.stdout
    transcript: list[Any] = []

    def request(ident: int, method: str, params: dict[str, Any]) -> dict[str, Any]:
        message = {"jsonrpc": "2.0", "id": ident, "method": method, "params": params}
        child.stdin.write(json.dumps(message) + "\n")
        child.stdin.flush()
        transcript.append({"sent": {"id": ident, "method": method, "params": params}})
        deadline = time.time() + 120
        while time.time() < deadline:
            line = child.stdout.readline()
            if not line:
                break
            try:
                value = json.loads(line)
            except json.JSONDecodeError:
                continue
            if value.get("id") == ident:
                transcript.append({"received": redact(value)})
                return value
            transcript.append({"notification": value.get("method")})
        raise StagingError(f"codex app-server gave no response to {method}")

    try:
        request(1, "initialize", {"clientInfo": {"name": "staging_home", "version": "1"}})
        child.stdin.write(json.dumps({"jsonrpc": "2.0", "method": "initialized"}) + "\n")
        child.stdin.flush()
        response = request(2, "account/read", {"refreshToken": True})
    finally:
        child.stdin.close()
        try:
            child.wait(timeout=30)
        except subprocess.TimeoutExpired:
            child.terminate()
            child.wait(timeout=10)
    stderr = child.stderr.read() if child.stderr else ""
    (out / "refresh-transcript.json").write_text(json.dumps(transcript, indent=2) + "\n")
    return {"response": redact(response), "exit": child.returncode, "stderrTail": stderr[-2000:]}


def redact(value: Any) -> Any:
    if isinstance(value, dict):
        return {key: ("<redacted>" if re.search(r"token|key|secret", key, re.I) else redact(item))
                for key, item in value.items()}
    if isinstance(value, list):
        return [redact(item) for item in value]
    return value


def codex_daemons(home: Path) -> list[str]:
    listing = subprocess.run(["ps", "-axo", "pid=,ppid=,lstart=,command="], capture_output=True, text=True,
                             check=False).stdout
    marker = str(home / ".codex")
    return [line.strip() for line in listing.splitlines() if marker in line and "app-server" in line]


def isolation(home: Path, candidate: Path, ravel: Path, out: Path) -> dict[str, Any]:
    if out.exists():
        raise StagingError(f"{out} exists; name a new directory")
    out.mkdir(parents=True)
    log: list[dict[str, Any]] = []
    record: dict[str, Any] = {"started": now(), "home": str(home), "candidate": str(candidate),
                              "ravel": str(ravel), "ravelSha256": sha256_file(ravel / "bin" / "ravel"),
                              "realHome": str(real_home()), "commands": log}
    versions = {}
    for tool in ("claude", "codex"):
        versions[tool] = subprocess.run([tool, "--version"], capture_output=True, text=True,
                                        check=False).stdout.strip()
    record["versions"] = versions
    real_claude_json = json.loads((real_home() / ".claude.json").read_text())
    record["authBefore"] = auth_facts()
    record["digestsBefore"] = real_digests()

    # Step 4 begins: write the staging harness configuration.
    record["build"] = build(home, candidate, log)

    # Step 1, before any session.
    record["step1Before"] = classify_links(home)

    # Step 3: configuration directories, overrides and refusals.
    env = staged_env(home)
    auth = run(["claude", "auth", "status"], env, home, log, check=False)
    claude_auth = json.loads(auth.stdout) if auth.stdout.strip().startswith("{") else {"raw": auth.stdout}
    codex = codex_listing(home, log)
    roots = list((codex["listing"].get("roots") or {}).values())
    refusals = {}
    for name in LOCATION_NAMES:
        try:
            refuse_inherited({**os.environ, name: str(real_home() / ".claude")}, home)
            refusals[name] = "not refused"
        except StagingError as error:
            refusals[name] = f"refused: {error}"
    try:
        refuse_inherited({**os.environ, "CODEX_HOME": str(real_home() / ".codex")}, home)
        refusals["CODEX_HOME"] = "not refused"
    except StagingError as error:
        refusals["CODEX_HOME"] = f"refused: {error}"
    record["step3"] = {
        "claudeConfigDirectory": claude_auth.get("configDirectory"),
        "claudeLoggedIn": claude_auth.get("loggedIn"), "claudeAuthMethod": claude_auth.get("authMethod"),
        "codexSkillRoots": roots, "codexLoginStatus": codex["loginStatus"],
        "codexOverrides": ["-c model_reasoning_effort=...", "--sandbox " + harness_trial.CODEX_SANDBOX,
                           f'-c approval_policy="{harness_trial.CODEX_APPROVAL}"'],
        "inheritedCodexHome": os.environ.get("CODEX_HOME"), "refusals": refusals,
        "workingDirectoryRefusals": cwd_refusals(home),
    }

    # Step 4 sessions, one on each harness. Step 2 reads their listings.
    prompt = ("This is an isolation check of a staging HOME. Do not read or change any file. "
              "Run exactly this shell command and nothing else: ravel end-session isolation-ok")
    sessions = {}
    for harness in ("claude", "codex"):
        sessions[harness] = launch(home, ravel, harness, out / f"session-{harness}", prompt, dict(os.environ))
    record["sessions"] = {h: {k: v for k, v in s.items() if k != "env"} | {"envNames": sorted(s["env"])}
                          for h, s in sessions.items()}
    events = stream_events((out / "session-claude" / "stream.jsonl").read_text())
    init = next((e for e in events if e.get("type") == "system" and e.get("subtype") == "init"), {})
    claude_skills = sorted(init.get("skills") or [])
    claude_plugins = init.get("plugins") or []
    after_codex = codex_listing(home, log)
    codex_names = sorted(listed_names(after_codex["listing"]))
    superseded = superseded_names()
    entries = {root: sorted(p.name for p in (home / root).iterdir()) if (home / root).is_dir() else []
               for root in (".codex/skills", ".agents/skills", ".claude/skills")}
    record["step2"] = {
        "nativeEntries": entries,
        "supersededEntries": {root: sorted(set(names) & superseded) for root, names in entries.items()},
        "claudeListedSkills": claude_skills, "claudePlugins": claude_plugins,
        "claudeListedSuperseded": sorted(set(claude_skills) & superseded),
        "codexListedSkills": codex_names, "codexListedSuperseded": sorted(set(codex_names) & superseded),
        "claudeInit": {key: init.get(key) for key in harness_trial.INIT_KEYS if key in init},
    }

    # Step 5: refresh the token in one Codex app-server session.
    record["step5"] = {"refresh": refresh_token(home, out), "authAfterRefresh": auth_facts()}
    real_env = {k: v for k, v in os.environ.items() if k != "CODEX_HOME"}
    status = subprocess.run(["codex", "login", "status"], env=real_env, capture_output=True, text=True,
                            check=False)
    record["step5"]["realLoginStatus"] = {"exit": status.returncode,
                                          "output": (status.stdout + status.stderr).strip()}
    record["step5"]["stagingLinkAfter"] = os.readlink(home / ".codex" / "auth.json") \
        if (home / ".codex" / "auth.json").is_symlink() else "replaced by a file"

    # Step 4 ends.
    record["digestsAfter"] = real_digests()
    changed = sorted(rel for rel in REAL_SUBJECTS if record["digestsBefore"][rel] != record["digestsAfter"][rel])
    record["step4"] = {"changed": changed}
    if ".claude.json" in changed:
        after_claude_json = json.loads((real_home() / ".claude.json").read_text())
        record["step4"]["claudeJsonKeysChanged"] = json_key_changes(real_claude_json, after_claude_json)
        record["step4"]["claudeJsonNamesStaging"] = str(home) in (real_home() / ".claude.json").read_text()
    record["step1After"] = classify_links(home)
    record["codexDaemons"] = codex_daemons(home)
    if record["codexDaemons"]:
        # A daemon that a staging session started: stop it through its own
        # interface, then list again.
        stop = run(["codex", "app-server", "daemon", "stop"], staged_env(home), work_directory(home), log,
                   check=False)
        record["codexDaemonStop"] = {"exit": stop.returncode, "stdout": stop.stdout.strip()}
        record["codexDaemonsAfterStop"] = codex_daemons(home)
    record["ended"] = now()
    (out / "record.json").write_text(json.dumps(record, indent=2) + "\n")
    return record


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = parser.add_subparsers(dest="command", required=True)
    b = sub.add_parser("build")
    b.add_argument("--home", type=Path, required=True)
    b.add_argument("--candidate", type=Path, required=True)
    b.add_argument("--ravel", type=Path, required=True)
    lk = sub.add_parser("links")
    lk.add_argument("--home", type=Path, required=True)
    la = sub.add_parser("launch")
    la.add_argument("--home", type=Path, required=True)
    la.add_argument("--ravel", type=Path, required=True)
    la.add_argument("--harness", choices=("claude", "codex"), required=True)
    la.add_argument("--out", type=Path, required=True)
    la.add_argument("--prompt", required=True)
    la.add_argument("--model")
    la.add_argument("--effort")
    iso = sub.add_parser("isolation")
    iso.add_argument("--home", type=Path, required=True)
    iso.add_argument("--candidate", type=Path, required=True)
    iso.add_argument("--ravel", type=Path, required=True)
    iso.add_argument("--out", type=Path, required=True)
    jc = sub.add_parser("mcp-entries")
    jc.add_argument("--home", type=Path, required=True)
    args = parser.parse_args()
    try:
        if args.command == "build":
            log: list[dict[str, Any]] = []
            print(json.dumps(build(args.home.absolute(), args.candidate.absolute(), log), indent=2))
        elif args.command == "mcp-entries":
            home = args.home.absolute()
            if home.resolve() == real_home().resolve():
                raise StagingError(f"{home} is the real HOME; mcp-entries writes only a staging HOME")
            check_placement(home)
            print(json.dumps(stage_mcp_entries(home, real_home())))
        elif args.command == "links":
            print(json.dumps(classify_links(args.home.absolute()), indent=2))
        elif args.command == "launch":
            result = launch(args.home.absolute(), args.ravel.absolute(), args.harness, args.out.absolute(),
                            args.prompt, dict(os.environ), args.model, args.effort)
            print(json.dumps({k: v for k, v in result.items() if k != "env"}, indent=2))
            return 0 if result["exit"] == 0 else 1
        else:
            result = isolation(args.home.absolute(), args.candidate.absolute(), args.ravel.absolute(),
                               args.out.absolute())
            print(f"staging_home: wrote {args.out.absolute() / 'record.json'}")
            print(json.dumps({"step4": result["step4"], "step1After": {
                k: result["step1After"][k] for k in ("signInComplete", "onlySignInLeaves", "internal")}},
                indent=2))
    except StagingError as error:
        print(f"staging_home: {error}", file=sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    sys.exit(main())
