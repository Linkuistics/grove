#!/usr/bin/env python3
"""Run one real harness trial of the common loading route, and record it.

The acceptance node `common-loading-acceptance-k29` measures actual Claude Code
and Codex sessions on the routes of `docs/specs/codex-skill-provisioning.md`.
This runner owns the mechanics that every trial shares. A case only says what
differs. The trial conventions live in that node's brief and in
`docs/common-loading-acceptance-record.md`.

    harness_trial.py prepare --out DIR --commit COMMIT --ravel FRONT
    harness_trial.py run --prepared DIR --case NAME [--model M] [--effort E]

`prepare` makes a plain Git checkout of this repository at COMMIT, builds
`grove` and `grove-llm` from it into a scratch prefix, and checks the Ravel 2.0
front against `scripts/harness-trials/builds.json`. The checkout also supplies
the skill sources, outside every working directory of a trial.

`run` builds a sandbox home as Ravel's configuration specification states
("Sandbox home for real harness runs"), a disposable jj repository with the
case's Grove tree, and the settings of the case's route. It then runs the
scratch Grove driver with the 2.0 front first on PATH, from an allow-listed
environment, and writes the case record. A real run spends subscription usage,
so no gate runs it. `scripts/test_harness_trial.py` tests the runner against
fake harnesses.

One test seam exists. HARNESS_TRIAL_PROGRAM names the harness program instead
of the one on PATH. The fake-harness tests set it, and an owner never does.
"""

from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import json
import os
import platform
import re
import shutil
import signal
import subprocess
import sys
import time
from pathlib import Path
from typing import Any, Mapping

ROOT = Path(__file__).resolve().parent.parent
TRIALS = ROOT / "scripts" / "harness-trials"
CASES = TRIALS / "cases"
BUILDS = TRIALS / "builds.json"
WRAPPER = TRIALS / "trial-launch.sh"

# The five Claude Code location variables of Ravel's configuration
# specification. Each can expose a real configuration, plugin tree or sign-in
# that no link check sees. The runner refuses each one, also with an empty
# value, before any launch of the harness, and it removes none of them.
LOCATION_NAMES = (
    "CLAUDE_CONFIG_DIR",
    "CLAUDE_SECURESTORAGE_CONFIG_DIR",
    "CLAUDE_CODE_PLUGIN_CACHE_DIR",
    "CLAUDE_CODE_PLUGIN_DIRS",
    "CLAUDE_CODE_PLUGIN_SEED_DIR",
)

# The names that the driver receives from the runner's environment. Every
# other name is dropped: API keys, the live session's channel and launch
# directory, CLAUDECODE and every CLAUDE_* name.
PASSED_NAMES = ("USER", "LOGNAME", "LANG", "LC_ALL", "LC_CTYPE")
SYSTEM_PATH = ("/usr/bin", "/bin", "/usr/sbin", "/sbin")

# The live session's authority. A name of this class may reach a trial child
# only with the trial's own value. A trial sets its own RAVEL_EXIT_FILE and
# GROVE_LAUNCH_DIR, so the name alone is no leak.
CONTROL_PATTERN = re.compile(r"^(RAVEL_|GROVE_|HARNESS_DISPATCH_|CLAUDE)")
# Names that no trial child may hold at all, with any value.
HARNESS_PATTERN = re.compile(r"^(CLAUDECODE$|CLAUDE_|ANTHROPIC_|OPENAI_|CODEX_API_KEY$)")

ROUTES = ("native-2.0", "directory-2.0")
DEFAULT_MODEL = "claude-opus-5-5"
DEFAULT_EFFORT = "medium"
# Claude Code's own permission layer. No call prompts or is denied, so the
# transcript shows what the agent chose to read. Ravel's confinement is an
# operating-system sandbox outside the harness, and this mode leaves it in force.
PERMISSION_MODE = "bypassPermissions"
# The longest Bash command a committed record holds.
COMMAND_CHARS = 160


class TrialError(Exception):
    """A refusal or a failed check. Its text names the remedy."""


def say(text: str) -> None:
    print(f"harness-trial: {text}", flush=True)


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def run_checked(argv: list[str], **kwargs: Any) -> str:
    result = subprocess.run(argv, capture_output=True, text=True, **kwargs)
    if result.returncode != 0:
        raise TrialError(f"{' '.join(argv)} failed with {result.returncode}: {result.stderr.strip()}")
    return result.stdout.strip()


# --- Refusals and the sandbox home ------------------------------------------


def refuse_locations(environ: Mapping[str, str]) -> None:
    """Refuse an inherited Claude Code location variable, before any launch."""
    for name in LOCATION_NAMES:
        if name in environ:
            raise TrialError(
                f"the environment holds {name}, which can expose a real Claude Code "
                f"configuration, plugin tree or sign-in; run the trial again without {name}"
            )


def sign_in_store() -> str:
    """The one path of the real home that the sandbox home links."""
    return "Library/Keychains" if platform.system() == "Darwin" else ".claude/.credentials.json"


def list_links(home: Path) -> list[tuple[str, str]]:
    """Every symbolic link under `home`, as (relative path, target), sorted."""
    links = []
    for directory, subdirs, files in os.walk(home):
        for name in subdirs + files:
            path = Path(directory) / name
            if path.is_symlink():
                links.append((str(path.relative_to(home)), os.readlink(path)))
    return sorted(links)


def check_links(links: list[tuple[str, str]], store: str, owner_home: Path, when: str) -> None:
    """The sandbox home links exactly the sign-in store, to the real store."""
    expected = [(store, str(owner_home / store))]
    if links != expected:
        raise TrialError(
            f"{when} the launch, the sandbox home holds the links {links}, not only {expected}; "
            "a harness that replaced the sign-in link probably refreshed a token, so sign in "
            "again in the real home and run the trial again"
        )


def allowed_env(environ: Mapping[str, str], home: Path, tmp: Path, path_dirs: list[str]) -> dict[str, str]:
    """The whole environment of the driver: the allow-list and nothing else."""
    env = {name: environ[name] for name in PASSED_NAMES if name in environ}
    env.update(
        HOME=str(home),
        TMPDIR=str(tmp),
        PATH=os.pathsep.join(path_dirs),
        SHELL="/bin/sh",
        TERM="dumb",
    )
    return env


def parse_env(text: str) -> dict[str, str]:
    """Parse the output of env(1). A line without `=` continues a value."""
    env: dict[str, str] = {}
    last = None
    for line in text.splitlines():
        name, sep, value = line.partition("=")
        if sep and re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", name):
            env[name] = value
            last = name
        elif last is not None:
            env[last] += "\n" + line
    return env


def child_control(child: Mapping[str, str], live: Mapping[str, str]) -> list[str]:
    """Each way the live session's authority reached a trial child."""
    leaks = []
    for name, value in sorted(child.items()):
        if HARNESS_PATTERN.match(name):
            leaks.append(f"{name} reached the trial child")
        elif CONTROL_PATTERN.match(name) and live.get(name) == value:
            leaks.append(f"{name} reached the trial child with the live session's value")
    return leaks


# --- The case ------------------------------------------------------------------


def load_case(name: str) -> tuple[dict[str, Any], Path]:
    directory = CASES / name
    file = directory / "case.json"
    if not file.is_file():
        known = sorted(path.name for path in CASES.iterdir() if (path / "case.json").is_file())
        raise TrialError(f"no case '{name}'; the cases are {', '.join(known)}")
    case = json.loads(file.read_text())
    if case.get("route") not in ROUTES:
        raise TrialError(f"case {name}: the route '{case.get('route')}' is not one of {', '.join(ROUTES)}")
    if case.get("harness") != "claude":
        raise TrialError(f"case {name}: this runner has an adapter for claude only")
    if not (directory / "tree").is_dir():
        raise TrialError(f"case {name}: no tree/ directory")
    return case, directory


def settings_for(case: dict[str, Any], checkout: Path) -> dict[str, Any]:
    """Ravel's owner settings of the case's route."""
    if case["route"] == "native-2.0":
        return {}
    sources = [
        {"id": source["id"], "type": "local", "path": str(checkout / source["plugin"]), "roots": source["roots"]}
        for source in case["sources"]
    ]
    return {"skills": {"schemaVersion": 1, "sources": sources, "mandatory": case.get("mandatory", [])}}


def rhai_string(text: str) -> str:
    return json.dumps(text)


def route_for(case: dict[str, Any], records: Path, program: Path, model: str, effort: str) -> dict[str, Any]:
    """The route as data. The configuration script and the fake driver of the
    tests both take it from here."""
    return {
        "kinds": case["kinds"],
        "wrapper": str(WRAPPER),
        "records": str(records),
        "program": str(program),
        "fixedArgs": [
            "-p", "--output-format", "stream-json", "--verbose",
            "--model", model, "--effort", effort, "--permission-mode", PERMISSION_MODE,
        ],
        "model": model,
        "effort": effort,
    }


def config_for(case: dict[str, Any], route: dict[str, Any]) -> str:
    """The configuration script: the case's kinds to the wrapper, any other refuses.
    Each snapshot root also goes to --add-dir, as in the Rhai sample."""
    kinds = ", ".join(f"{rhai_string(kind)}: true" for kind in route["kinds"])
    fixed = route["fixedArgs"]
    records, program, model, effort = route["records"], route["program"], route["model"], route["effort"]
    return f"""// Written by scripts/harness_trial.py for the case {case['name']}.
export const SCHEMA_VERSION = 4;
export const VERSION = {rhai_string('harness-trial-' + case['name'])};

const KINDS = #{{ {kinds} }};

fn select(request, context) {{
    if !global::KINDS.contains(request.kind) {{
        return #{{ status: "refused", code: "trial_kind_unrouted",
            message: `the trial routes only its case's kinds, and ${{request.kind}} is not one`,
            remedy: "this refusal ends the trial loop by design; add the kind to the case to launch it" }};
    }}
    let view = #{{ kind: request.kind, originalPrompt: request.originalPrompt, skills: request.skills }};
    let args = [{rhai_string(str(records))}, ravel::json_encode(view), {rhai_string(str(program))}];
    // --add-dir takes several values, so a fixed flag must follow the last
    // root. Otherwise the prompt would become one more directory.
    if request.skills != () {{
        for source in request.skills.sources {{ args += ["--add-dir", source.snapshotRoot]; }}
    }}
    args += [{', '.join(rhai_string(arg) for arg in fixed)}];
    args.push(request.prompt);
    #{{ status: "selected", program: {rhai_string(route["wrapper"])}, args: args,
       provider: "anthropic", model: {rhai_string(model)}, effort: {rhai_string(effort)},
       reason: {rhai_string('harness trial ' + case['name'])} }}
}}
"""


# --- Events --------------------------------------------------------------------


def read_jsonl(path: Path) -> list[dict[str, Any]]:
    entries = []
    for line in path.read_text(errors="replace").splitlines():
        line = line.strip()
        if not line:
            continue
        try:
            value = json.loads(line)
        except json.JSONDecodeError:
            continue
        if isinstance(value, dict):
            entries.append(value)
    return entries


class Classifier:
    """Names what a path is, from the request view of the launch."""

    def __init__(self, view: dict[str, Any], home: Path, repo: Path):
        skills = view.get("skills") or {}
        self.kind_skill = f"grove-{view.get('kind', '')}"
        self.mandatory = {self._norm(item["path"]) for item in skills.get("mandatory", [])}
        self.descriptor = self._norm(skills["descriptorPath"]) if skills.get("descriptorPath") else None
        self.snapshots = [(source["id"], self._norm(source["snapshotRoot"])) for source in skills.get("sources", [])]
        self.origins = [(source["id"], self._norm(source["originPath"])) for source in skills.get("sources", [])]
        self.native = [self._norm(str(home / ".claude")), self._norm(str(home / ".codex"))]
        self.repo = self._norm(str(repo))

    @staticmethod
    def _norm(path: str) -> str:
        return os.path.realpath(path)

    @staticmethod
    def _inside(path: str, root: str) -> bool:
        return path == root or path.startswith(root.rstrip("/") + "/")

    def classify(self, raw: str) -> tuple[str, str | None]:
        """(class, skill name) of one absolute path."""
        path = self._norm(raw)
        if path in self.mandatory:
            return "mandatory", self._skill(path)
        if self.descriptor and path == self.descriptor:
            return "descriptor", None
        for source, root in self.snapshots:
            if self._inside(path, root):
                skill = self._skill(path)
                if os.path.isdir(path):
                    return "listing", skill
                if path.endswith("/SKILL.md"):
                    return ("kind-skill" if skill == self.kind_skill else "skill"), skill
                return "supporting", skill
        for source, root in self.origins:
            if self._inside(path, root):
                return "source-origin", self._skill(path)
        for root in self.native:
            if self._inside(path, root):
                return "native", self._skill(path)
        if self._inside(path, self.repo):
            return "work", None
        return "other", None

    @staticmethod
    def _skill(path: str) -> str | None:
        parts = Path(path).parts
        if "skills" in parts:
            index = len(parts) - 1 - parts[::-1].index("skills")
            if index + 1 < len(parts):
                return parts[index + 1]
        return None


PATH_TOKEN = re.compile(r"""(?<![\w.])(/[^\s'"`;|&<>()]+)""")
READ_CLASSES = (
    "mandatory", "descriptor", "kind-skill", "skill", "supporting", "listing", "source-origin", "native",
)
SEPARATORS = {"&&", "||", ";", "|", "&", "\n", "(", ")"}


def bash_paths(command: str, cwd: str) -> list[str]:
    """The paths that a Bash command names, in order. A `cd` changes the
    directory against which later relative paths resolve. A relative token
    counts when it exists and is `.`, `..`, holds a slash or ends in `.md`. A
    token with a shell expansion counts not at all."""
    import shlex

    try:
        lexer = shlex.shlex(command, posix=True, punctuation_chars=True)
        lexer.whitespace_split = True
        tokens = list(lexer)
    except ValueError:
        return PATH_TOKEN.findall(command)
    paths: list[str] = []
    here = cwd
    first = True
    pending_cd = False
    for token in tokens:
        if token in SEPARATORS or set(token) <= set("<>&|;"):
            first, pending_cd = True, False
            continue
        if first:
            first = False
            pending_cd = token == "cd"
            continue
        if "$" in token or "~" in token or token.startswith("-"):
            continue
        if pending_cd:
            pending_cd = False
            here = os.path.normpath(os.path.join(here, token))
            continue
        if token.startswith("/"):
            paths.append(token)
        elif token in (".", "..") or "/" in token or token.endswith(".md"):
            # A relative token that names nothing, such as a find pattern, is
            # no read.
            path = os.path.normpath(os.path.join(here, token))
            if os.path.exists(path):
                paths.append(path)
    return paths


def tool_target(name: str, tool_input: dict[str, Any], cwd: str = "/") -> tuple[str, list[str]]:
    """The shown target of a tool call, and the absolute paths it names."""
    if name in ("Read", "Edit", "Write", "MultiEdit"):
        path = tool_input.get("file_path", "")
        return path, [path] if path else []
    if name == "NotebookEdit":
        path = tool_input.get("notebook_path", "")
        return path, [path] if path else []
    if name in ("Glob", "Grep"):
        path = tool_input.get("path", "")
        shown = f"{tool_input.get('pattern', '')} in {path or '(cwd)'}"
        return shown, [path] if path.startswith("/") else []
    if name == "Bash":
        command = tool_input.get("command", "")
        return command, bash_paths(command, cwd)
    if name == "Skill":
        return tool_input.get("skill", ""), []
    if name in ("Task", "Agent"):
        return f"{tool_input.get('subagent_type', '')}: {tool_input.get('description', '')}", []
    return ",".join(sorted(tool_input)), []


def extract_events(entries: list[dict[str, Any]], classifier: Classifier) -> list[dict[str, Any]]:
    """The ordered tool calls of one transcript, each with its class."""
    errors: dict[str, bool] = {}
    for entry in entries:
        content = (entry.get("message") or {}).get("content")
        if isinstance(content, list):
            for block in content:
                if isinstance(block, dict) and block.get("type") == "tool_result":
                    errors[block.get("tool_use_id", "")] = bool(block.get("is_error"))
    events = []
    for entry in entries:
        if entry.get("type") != "assistant":
            continue
        content = (entry.get("message") or {}).get("content")
        if not isinstance(content, list):
            continue
        for block in content:
            if not (isinstance(block, dict) and block.get("type") == "tool_use"):
                continue
            name = block.get("name", "")
            tool_input = block.get("input") or {}
            target, paths = tool_target(name, tool_input, entry.get("cwd") or classifier.repo)
            classes = [classifier.classify(path) + (path,) for path in paths]
            event: dict[str, Any] = {
                "seq": len(events) + 1,
                "tool": name,
                "target": target,
                "error": errors.get(block.get("id", "")),
                "sidechain": bool(entry.get("isSidechain")),
            }
            reads = [item for item in classes if item[0] in READ_CLASSES]
            if reads and name not in ("Edit", "Write", "MultiEdit", "NotebookEdit"):
                event["class"], event["skill"], event["path"] = reads[0]
                if len(reads) > 1:
                    event["more"] = [{"class": c, "skill": s, "path": p} for c, s, p in reads[1:]]
            elif name in ("Edit", "Write", "MultiEdit", "NotebookEdit"):
                event["class"], event["path"] = "action", target
            elif name == "Skill":
                event["class"] = "native-skill-tool"
            elif name in ("Task", "Agent"):
                event["class"] = "delegate"
            elif name == "Bash":
                ending = re.search(r"end-session\s+([\w-]+)", target)
                event["class"] = "ending" if ending else "command"
                if ending:
                    event["result"] = ending.group(1)
            elif classes:
                event["class"], event["skill"], event["path"] = classes[0]
            else:
                event["class"] = "other"
            events.append(event)
    return events


INIT_KEYS = (
    "claude_code_version", "model", "permissionMode", "cwd", "tools", "mcp_servers",
    "slash_commands", "skills", "plugins", "agents", "output_style", "apiKeySource",
)


def native_startup(stream: list[dict[str, Any]], session: list[dict[str, Any]]) -> dict[str, Any]:
    """What Claude Code itself offered at startup, without account data."""
    init = next((e for e in stream if e.get("type") == "system" and e.get("subtype") == "init"), None)
    startup: dict[str, Any] = {"init": {key: init[key] for key in INIT_KEYS if init and key in init}}
    listing = next(
        (e["attachment"] for e in session if e.get("type") == "attachment"
         and (e.get("attachment") or {}).get("type") == "skill_listing"),
        None,
    )
    if listing:
        startup["skillListing"] = {"skillCount": listing.get("skillCount"), "names": listing.get("names")}
    startup["attachmentTypes"] = sorted({
        (e.get("attachment") or {}).get("type", "") for e in session if e.get("type") == "attachment"
    })
    return startup


def skill_files(home: Path) -> tuple[list[str], list[str]]:
    """(native copies, platform-managed residue): each SKILL.md in the home
    outside Ravel's cache. claude.ai syncs an account's skills into
    .claude/skills/synced, and those are residue, not native copies."""
    native, residue = [], []
    cache = home / ".cache" / "ravel"
    synced = home / ".claude" / "skills" / "synced"
    for directory, subdirs, files in os.walk(home):
        here = Path(directory)
        if here == cache or cache in here.parents:
            subdirs[:] = []
            continue
        if "SKILL.md" in files:
            relative = str((here / "SKILL.md").relative_to(home))
            (residue if here == synced or synced in here.parents else native).append(relative)
    return sorted(native), sorted(residue)


# --- prepare -------------------------------------------------------------------


def platform_key() -> str:
    return f"{platform.system()}-{platform.machine()}"


def check_ravel(front: Path) -> dict[str, Any]:
    """The 2.0 front matches the pins of builds.json."""
    pins = json.loads(BUILDS.read_text())["ravel2"]
    if not front.is_absolute() or not front.is_file():
        raise TrialError(f"no Ravel front at {front}; install the 2.0 build by the stage record's recipe")
    digest = sha256_file(front)
    expected = pins["binRavelSha256"].get(platform_key())
    if digest != expected:
        raise TrialError(
            f"{front} has SHA-256 {digest}, and builds.json pins {expected} for {platform_key()}; "
            "if the bookmark skill-directory-delivery moved, record the new build in builds.json "
            "and in the acceptance record before any result"
        )
    if (front.parent.parent / "libexec").exists():
        raise TrialError(f"{front.parent.parent}/libexec exists; the 2.0 prefix holds bin/ravel only")
    identity = pins["buildIdentity"]
    count = front.read_bytes().count(identity.encode())
    if count == 0:
        raise TrialError(f"{front} does not hold the build identity {identity}")
    return {
        "front": str(front), "binRavelSha256": digest, "buildIdentity": identity,
        "identityOccurrences": count, "buildCommit": pins["buildCommit"],
        "outputCommit": pins["outputCommit"], "line": pins["line"],
    }


def prepare(out: Path, commit: str, front: Path, environ: Mapping[str, str]) -> Path:
    out = out.resolve()
    ravel = check_ravel(front)
    full = run_checked(["git", "rev-parse", "--verify", f"{commit}^{{commit}}"], cwd=ROOT)
    common = run_checked(["git", "rev-parse", "--path-format=absolute", "--git-common-dir"], cwd=ROOT)
    checkout = out / "grove-source"
    if checkout.exists():
        raise TrialError(f"{checkout} exists; name an empty OUT for a new preparation")
    out.mkdir(parents=True, exist_ok=True)
    git_env = {**environ, "GIT_CONFIG_GLOBAL": "/dev/null", "GIT_CONFIG_NOSYSTEM": "1"}
    run_checked(["git", "init", "--quiet", str(checkout)], env=git_env)
    run_checked(["git", "fetch", "--quiet", common, full], cwd=checkout, env=git_env)
    run_checked(["git", "checkout", "--quiet", "--detach", "FETCH_HEAD"], cwd=checkout, env=git_env)
    if run_checked(["git", "status", "--porcelain"], cwd=checkout, env=git_env):
        raise TrialError(f"the checkout {checkout} is not clean")

    say(f"building grove and grove-llm at {full[:12]} into {out}")
    build_env = {name: value for name, value in environ.items() if not CONTROL_PATTERN.match(name)}
    build_env["CARGO_TARGET_DIR"] = str(out / "target")
    subprocess.run(
        ["cargo", "build", "--locked", "--release", "--bin", "grove", "--bin", "grove-llm"],
        cwd=checkout, env=build_env, check=True,
    )
    prefix = out / "grove" / "bin"
    prefix.mkdir(parents=True, exist_ok=True)
    binaries = {}
    for name in ("grove", "grove-llm"):
        shutil.copy2(out / "target" / "release" / name, prefix / name)
        binaries[name] = sha256_file(prefix / name)
    record = {
        "schemaVersion": 1,
        "prepared": dt.datetime.now(dt.UTC).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "platform": platform_key(),
        "grove": {"commit": full, "prefix": str(prefix.parent), "sha256": binaries},
        "source": {"checkout": str(checkout), "commit": full},
        "ravel": ravel,
    }
    (out / "prepared.json").write_text(json.dumps(record, indent=2) + "\n")
    say(f"prepared {out}/prepared.json")
    return out / "prepared.json"


# --- run -----------------------------------------------------------------------


def harness_program(environ: Mapping[str, str]) -> Path:
    named = environ.get("HARNESS_TRIAL_PROGRAM") or shutil.which("claude", path=environ.get("PATH"))
    if not named or not os.access(named, os.X_OK):
        raise TrialError("no 'claude' program found on PATH; install Claude Code, or put it on PATH")
    return Path(named).resolve()


def plant_repo(repo: Path, tree: Path, env: dict[str, str]) -> str:
    repo.mkdir()
    run_checked(["jj", "--config", "git.colocate=false", "git", "init", "."], cwd=repo, env=env)
    shutil.copytree(tree, repo / ".grove")
    run_checked(["jj", "commit", "--quiet", "--message", "trial tree"], cwd=repo, env=env)
    return run_checked(["jj", "log", "--no-graph", "-r", "@-", "-T", "commit_id"], cwd=repo, env=env)


def tree_state(repo: Path) -> list[str]:
    grove = repo / ".grove"
    if not grove.is_dir():
        return []
    return sorted(str(path.relative_to(grove)) for path in grove.rglob("*.md"))


def run_driver(argv: list[str], cwd: Path, env: dict[str, str], out: Path, timeout_s: float) -> dict[str, Any]:
    """Run the driver in a session of its own, and stop it at the deadline."""
    started = time.monotonic()
    with (out / "driver.stdout").open("wb") as stdout, (out / "driver.stderr").open("wb") as stderr:
        child = subprocess.Popen(
            argv, cwd=cwd, env=env, stdin=subprocess.DEVNULL, stdout=stdout, stderr=stderr,
            start_new_session=True,
        )
        timed_out = False
        try:
            status = child.wait(timeout=timeout_s)
        except subprocess.TimeoutExpired:
            timed_out = True
            # Grove forwards TERM to Ravel, which reaps its managed group.
            child.send_signal(signal.SIGTERM)
            try:
                status = child.wait(timeout=60)
            except subprocess.TimeoutExpired:
                child.kill()
                status = child.wait()
    return {"exit": status, "timedOut": timed_out, "elapsedS": round(time.monotonic() - started, 1)}


def launch_records(records: Path) -> list[Path]:
    return sorted((p for p in records.iterdir() if p.is_dir()), key=lambda p: int(p.name))


def session_files(home: Path) -> list[Path]:
    projects = home / ".claude" / "projects"
    return sorted(projects.rglob("*.jsonl")) if projects.is_dir() else []


def describe_launch(record: Path, home: Path, repo: Path, live: Mapping[str, str]) -> dict[str, Any]:
    view = json.loads((record / "view.json").read_text())
    child = parse_env((record / "env").read_text())
    stream = read_jsonl(record / "stream.jsonl") if (record / "stream.jsonl").exists() else []
    init = next((e for e in stream if e.get("type") == "system" and e.get("subtype") == "init"), {})
    session_id = init.get("session_id")
    classifier = Classifier(view, home, repo)
    transcripts = []
    for path in session_files(home):
        if session_id and session_id not in str(path):
            continue
        raw = path.read_bytes()
        entries = read_jsonl(path)
        transcripts.append({
            "file": str(path.relative_to(home)),
            "sha256": sha256_bytes(raw),
            "main": path.stem == session_id,
            "events": extract_events(entries, classifier),
            "startup": native_startup(stream, entries) if path.stem == session_id else None,
        })
    skills = view.get("skills") or {}
    descriptor = skills.get("descriptorPath")
    descriptor_identity = None
    if descriptor and Path(descriptor).is_file():
        descriptor_identity = {
            "directory": Path(descriptor).parent.name,
            "sha256": sha256_file(Path(descriptor)),
        }
    return {
        "record": str(record),
        "kind": view.get("kind"),
        "sessionId": session_id,
        "descriptorPath": descriptor,
        "descriptorIdentity": descriptor_identity,
        "sources": [
            {key: source.get(key) for key in ("id", "resourceDigest", "originPath", "snapshotRoot")}
            for source in skills.get("sources", [])
        ],
        "mandatory": skills.get("mandatory", []),
        "envNames": sorted(child),
        "control": child_control(child, live),
        "streamSha256": sha256_file(record / "stream.jsonl") if (record / "stream.jsonl").exists() else None,
        "startup": native_startup(stream, []) if not transcripts else None,
        "transcripts": transcripts,
        # The last result event of the stream, absent when Ravel ended the
        # harness before it wrote one.
        "streamResult": next((e.get("subtype") for e in reversed(stream) if e.get("type") == "result"), None),
    }


def started(launch: dict[str, Any]) -> bool:
    """The harness began a session: its stream holds the init event."""
    return launch["sessionId"] is not None


def run_case(prepared_dir: Path, name: str, environ: Mapping[str, str],
             model: str | None = None, effort: str | None = None) -> dict[str, Any]:
    # Nothing launches before these refusals, including `claude --version`.
    refuse_locations(environ)
    case, case_dir = load_case(name)
    prepared_dir = prepared_dir.resolve()
    prepared = json.loads((prepared_dir / "prepared.json").read_text())
    front = Path(prepared["ravel"]["front"])
    if sha256_file(front) != prepared["ravel"]["binRavelSha256"]:
        raise TrialError(f"{front} changed since preparation; prepare again")
    grove_bin = Path(prepared["grove"]["prefix"]) / "bin"
    for binary, digest in prepared["grove"]["sha256"].items():
        if sha256_file(grove_bin / binary) != digest:
            raise TrialError(f"{grove_bin / binary} changed since preparation; prepare again")
    checkout = Path(prepared["source"]["checkout"])
    owner_home = Path(environ["HOME"])
    store = sign_in_store()
    if not (owner_home / store).exists():
        raise TrialError(f"no sign-in store at {owner_home / store}; sign in to Claude Code as the owner")
    program = harness_program(environ)
    model = model or case.get("model", DEFAULT_MODEL)
    effort = effort or case.get("effort", DEFAULT_EFFORT)

    stamp = dt.datetime.now(dt.UTC).strftime("%Y%m%dT%H%M%SZ")
    trial = prepared_dir / "runs" / f"{name}-{stamp}"
    home, repo, records, tmp = trial / "home", trial / "repo", trial / "records", trial / "tmp"
    for directory in (home / ".config" / "ravel", home / ".config" / "jj", records, tmp):
        directory.mkdir(parents=True)
    (home / store).parent.mkdir(parents=True, exist_ok=True)
    os.symlink(owner_home / store, home / store)
    (home / ".config" / "jj" / "config.toml").write_text(
        'user.name = "Grove Trial"\nuser.email = "grove-trial@example.com"\n'
    )
    settings = settings_for(case, checkout)
    if settings:
        (home / ".config" / "ravel" / "settings.json").write_text(json.dumps(settings, indent=2) + "\n")
    route = route_for(case, records, program, model, effort)
    (trial / "route.json").write_text(json.dumps(route, indent=2) + "\n")
    (home / ".config" / "ravel" / "config.rhai").write_text(config_for(case, route))

    links_before = list_links(home)
    for link, target in links_before:
        say(f"link {link} -> {target}")
    check_links(links_before, store, owner_home, "before")
    config_dir = home / ".claude"
    say(f"configuration directory {config_dir}")

    tools = [shutil.which(tool, path=environ.get("PATH")) for tool in ("jj", "git")]
    if not all(tools):
        raise TrialError("jj and git must be on PATH")
    path_dirs = [str(front.parent), str(grove_bin), str(program.parent)]
    path_dirs += [str(Path(tool).parent) for tool in tools if tool] + list(SYSTEM_PATH)
    path_dirs = list(dict.fromkeys(path_dirs))
    env = allowed_env(environ, home, tmp, path_dirs)

    version = run_checked([str(program), "--version"], cwd=tmp, env=env).splitlines()[0]
    auth_text = run_checked([str(program), "auth", "status"], cwd=tmp, env=env)
    try:
        auth = json.loads(auth_text)
    except json.JSONDecodeError as error:
        raise TrialError(f"claude auth status wrote no JSON: {error}") from None
    reported = auth.get("configDirectory")
    if not reported or os.path.realpath(reported) != os.path.realpath(config_dir):
        raise TrialError(f"Claude Code reports the configuration directory '{reported}', not the sandbox {config_dir}")
    check_links(list_links(home), store, owner_home, "after the probes and before")

    base = plant_repo(repo, case_dir / "tree", env)
    tree_before = tree_state(repo)
    say(f"{name}: {version}, model {model}, effort {effort}, route {case['route']}, trial {trial}")
    driver = run_driver([str(grove_bin / "grove")], repo, env, trial, case.get("timeoutMinutes", 30) * 60)

    links_after = list_links(home)
    for link, target in links_after:
        say(f"link after {link} -> {target}")
    link_failure = None
    try:
        check_links(links_after, store, owner_home, "after")
    except TrialError as error:
        link_failure = str(error)

    stderr = (trial / "driver.stderr").read_text(errors="replace")
    launches = [describe_launch(record, home, repo, environ) for record in launch_records(records)]
    native, residue = skill_files(home)
    published = run_checked(
        ["jj", "log", "--no-graph", "-r", f"{base}..@ ~ empty()", "-T", 'commit_id.short(12) ++ " " ++ if(description, description.first_line(), "(no description)") ++ "\\n"'],
        cwd=repo, env=env,
    )
    changed = run_checked(["jj", "diff", "--summary", "--from", base, "--to", "@"], cwd=repo, env=env)
    record = {
        "schemaVersion": 1,
        "case": name,
        "purpose": case.get("purpose"),
        "credits": case.get("credits", []),
        "route": case["route"],
        "trial": str(trial),
        "builds": {"grove": prepared["grove"], "ravel": prepared["ravel"]},
        "harness": {
            "name": "claude", "program": str(program), "version": version, "model": model,
            "effort": effort, "permissionMode": PERMISSION_MODE,
            "configDirectory": str(config_dir), "authConfigDirectory": reported,
            "loggedIn": auth.get("loggedIn"),
        },
        "sources": {"checkout": prepared["source"]["checkout"], "commit": prepared["source"]["commit"],
                    "settings": settings},
        "environment": sorted(env),
        "links": {"before": links_before, "after": links_after, "failure": link_failure},
        "skills": {"nativeCopies": native, "residue": residue},
        "launches": launches,
        "ending": {
            "driverExit": driver["exit"],
            "timedOut": driver["timedOut"],
            "elapsedS": driver["elapsedS"],
            "driverLines": [line for line in stderr.splitlines() if line.startswith(("grove:", "ravel:", "Error"))],
            "endingCommands": [e.get("result") for l in launches for t in l["transcripts"]
                               for e in t["events"] if e["class"] == "ending"],
            "teardownRecorded": "grove finished — loop complete" in stderr,
            "treeBefore": tree_before,
            "treeAfter": tree_state(repo),
            "published": published.splitlines(),
            "changed": changed.splitlines(),
            "launches": len(launches),
        },
    }
    (trial / "record.json").write_text(json.dumps(record, indent=2) + "\n")
    (trial / "record.md").write_text(render(record))
    # The sign-in link is removed so that no trial directory keeps the real store.
    if (home / store).is_symlink():
        (home / store).unlink()
    failures = [leak for launch in launches for leak in launch["control"]]
    failures += [f"launch {n}: the harness wrote no startup event; read {launch['record']}/stderr"
                 for n, launch in enumerate(launches) if not started(launch)]
    if not launches:
        failures.append("the driver launched no harness; read driver.stderr of the trial")
    if link_failure:
        failures.append(link_failure)
    if native:
        failures.append(f"native copies exist in the sandbox home: {native}")
    say(f"record {trial}/record.md")
    if failures:
        raise TrialError("; ".join(failures))
    return record


# --- The record ----------------------------------------------------------------


def cell(text: Any) -> str:
    return str(text).replace("|", "\\|").replace("\n", " ")


def shown(event: dict[str, Any]) -> str:
    target = event["target"]
    if event["tool"] == "Bash" and len(target) > COMMAND_CHARS:
        target = target[:COMMAND_CHARS] + "…"
    return f"`{cell(target)}`" if target else ""


def render(record: dict[str, Any]) -> str:
    """The Markdown facts of one case. The result and its limit are the
    reader's judgement, so the acceptance record adds them by hand."""
    h = record["harness"]
    g = record["builds"]["grove"]
    r = record["builds"]["ravel"]
    e = record["ending"]
    lines = [
        f"### Facts of `{record['case']}`",
        "",
        "| Subject | Value |",
        "| --- | --- |",
        f"| Route | {record['route']} |",
        f"| Credits | {', '.join(record['credits']) or 'none'} |",
        f"| Grove build | commit `{g['commit'][:12]}`, `grove` SHA-256 `{g['sha256']['grove']}`, `grove-llm` SHA-256 `{g['sha256']['grove-llm']}` |",
        f"| Ravel build | commit `{r['buildCommit']}` (line `{r['line']}` at `{r['outputCommit']}`), identity `{r['buildIdentity']}`, `bin/ravel` SHA-256 `{r['binRavelSha256']}` |",
        f"| Harness | {cell(h['version'])}, model `{h['model']}`, effort `{h['effort']}`, permission mode `{h['permissionMode']}` |",
        f"| Configuration directory | sandbox `.claude`; `claude auth status` reports the same directory |",
        f"| Source checkout | commit `{record['sources']['commit'][:12]}` |",
        f"| Links before and after | {cell(record['links']['before'] and record['links']['before'][0][0])} only; after: {cell(', '.join(link for link, _ in record['links']['after']) or 'none')} |",
        f"| Driver environment | {cell(', '.join(record['environment']))} |",
        f"| Native copies | {cell(', '.join(record['skills']['nativeCopies']) or 'none')} |",
        f"| Synced claude.ai skills (residue) | {cell(', '.join(record['skills']['residue']) or 'none')} |",
        "",
    ]
    for number, launch in enumerate(record["launches"]):
        lines += [f"#### Launch {number}: kind `{launch['kind']}`", ""]
        identity = launch["descriptorIdentity"] or {}
        lines += [
            f"- Descriptor identity: `{identity.get('directory')}` (file SHA-256 `{identity.get('sha256')}`).",
            *[f"- Source `{s['id']}`: `resourceDigest` `{s['resourceDigest']}`." for s in launch["sources"]],
            f"- Authority control: {'no live control name reached the child' if not launch['control'] else '; '.join(launch['control'])}.",
            f"- Child environment names: {cell(', '.join(launch['envNames']))}.",
        ]
        for transcript in launch["transcripts"]:
            lines += [f"- Transcript `{transcript['file']}`: SHA-256 `{transcript['sha256']}`, {len(transcript['events'])} tool calls."]
        startup = next((t["startup"] for t in launch["transcripts"] if t["startup"]), launch["startup"]) or {}
        init = startup.get("init", {})
        lines += [
            "",
            "Native startup text:",
            "",
            "```json",
            json.dumps({**init, **{k: v for k, v in startup.items() if k != "init"}}, indent=2),
            "```",
            "",
        ]
        for transcript in launch["transcripts"]:
            lines += [
                f"Ordered events of `{Path(transcript['file']).name}`:",
                "",
                "| # | Tool | Class | Skill | Target | Error |",
                "| --- | --- | --- | --- | --- | --- |",
            ]
            for event in transcript["events"]:
                lines.append(
                    f"| {event['seq']} | {event['tool']} | {event['class']} | {event.get('skill') or ''} "
                    f"| {shown(event)} | {'yes' if event['error'] else ''} |"
                )
            lines.append("")
    lines += [
        "Ending outcome:",
        "",
        f"- Ending commands that the agent ran: {cell(', '.join(str(x) for x in e['endingCommands']) or 'none')}.",
        f"- Driver: exit {e['driverExit']}{', timed out' if e['timedOut'] else ''}, {e['elapsedS']} s.",
        *[f"  - `{cell(line)}`" for line in e["driverLines"]],
        f"- Teardown recorded: {'yes' if e['teardownRecorded'] else 'no'}.",
        f"- Tree before: {cell(', '.join(e['treeBefore']))}.",
        f"- Tree after: {cell(', '.join(e['treeAfter']))}.",
        f"- Published commits: {cell('; '.join(e['published']) or 'none')}.",
        f"- Changed files: {cell('; '.join(e['changed']) or 'none')}.",
        f"- Harness launches: {e['launches']}.",
        "",
    ]
    return "\n".join(lines)


def main(argv: list[str] | None = None, environ: Mapping[str, str] | None = None) -> int:
    environ = os.environ if environ is None else environ
    parser = argparse.ArgumentParser(prog="harness_trial.py", description=__doc__.split("\n\n")[0])
    commands = parser.add_subparsers(dest="command", required=True)
    p = commands.add_parser("prepare", help="build Grove at a commit and check the Ravel 2.0 front")
    p.add_argument("--out", required=True, type=Path)
    p.add_argument("--commit", required=True)
    p.add_argument("--ravel", required=True, type=Path, help="the absolute 2.0 front, PREFIX/bin/ravel")
    r = commands.add_parser("run", help="run one real trial case (spends subscription usage)")
    r.add_argument("--prepared", required=True, type=Path)
    r.add_argument("--case", required=True)
    r.add_argument("--model")
    r.add_argument("--effort")
    args = parser.parse_args(argv)
    try:
        if args.command == "prepare":
            prepare(args.out, args.commit, args.ravel, environ)
        else:
            run_case(args.prepared, args.case, environ, args.model, args.effort)
    except TrialError as error:
        print(f"harness-trial: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
