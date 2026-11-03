#!/usr/bin/env python3
"""Run one real harness trial of the common loading route, and record it.

The acceptance node `common-loading-acceptance-k29` measures actual Claude Code
and Codex sessions on the routes of `docs/specs/codex-skill-provisioning.md`.
This runner owns the mechanics that every trial shares. A case only says what
differs. The trial conventions live in that node's brief and in
`docs/common-loading-acceptance-record.md`.

    harness_trial.py prepare --out DIR --commit COMMIT --ravel FRONT
    harness_trial.py run --prepared DIR --case NAME [--model M] [--effort E]
    harness_trial.py reextract --trial RUN
    harness_trial.py turns --plan PLAN -- PROGRAM ARG... PROMPT

`prepare` makes a plain Git checkout of this repository at COMMIT, builds
`grove` and `grove-llm` from it into a scratch prefix, and checks the Ravel 2.0
front against `scripts/harness-trials/builds.json`. The checkout also supplies
the skill sources, outside every working directory of a trial.

`run` builds a sandbox home as Ravel's configuration specification states
("Sandbox home for real harness runs"), a disposable jj repository with the
case's Grove tree, and the settings of the case's route. It then runs the
scratch Grove driver with the 2.0 front first on PATH, from an allow-listed
environment, and writes the case record. A standalone case runs `grove run`
instead, which Ravel confines. A case can add access probes inside the launch
and an offline `ravel skills resolve`. For an explicit-route request, a case
can also copy native skills into the sandbox `.claude/skills`, write the
sandbox `.claude/settings.json` and plant files under the trial's `guidance/`.
A native copy can come from an older commit of the checkout, as a stale copy.
A case with `moveAway` and `expectRefusal` moves a clone of one source away
before the driver, and expects Ravel's refusal before any harness launch.
A case with `continuation` resumes the first session outside Ravel after a
source commit and a settings change, then launches a fresh session. A case
with `compaction` runs three turns in one managed process: the prompt,
`/compact` with the case's instructions, then the case's answer. The feeder
`harness_trial.py turns` sends them, and it sends the answer only when the
context after compaction holds no preamble path.
Each Agent call of a launch adds a delegation to the record: the prompt that
the parent gave, its comparison with the preamble, the delegate's transcript
and the ending context that the delegate inherits. A case with
`expectDelegate` fails without a delegate transcript. A case can `collect`
files of its trial directory into the record. A case's `systemNote` reaches
Claude Code through `--append-system-prompt`, the owner's channel, and the
prepared prompt stays unchanged.
`reextract` extracts the events of an earlier run again, from its unchanged
transcripts. A real run spends
subscription usage, so no gate runs it. `scripts/test_harness_trial.py` tests
the runner against fake harnesses.

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
# A lifecycle case runs the bare Grove driver in a jj repository. A standalone
# case runs `grove run`, which Ravel confines.
MODES = ("lifecycle", "standalone")
# The network-denying profile of crates/grove/tests/directory_route.rs.
NO_NETWORK = "(version 1)(allow default)(deny network*)"
DEFAULT_MODEL = "claude-opus-5-5"
DEFAULT_EFFORT = "medium"
# Claude Code's own permission layer. No call prompts or is denied, so the
# transcript shows what the agent chose to read. Ravel's confinement is an
# operating-system sandbox outside the harness, and this mode leaves it in force.
PERMISSION_MODE = "bypassPermissions"
# The longest Bash command or text block a committed record holds.
COMMAND_CHARS = 160
# The longest Skill result a record holds. A successful load returns the
# skill's text, and its start is enough to name the skill.
SKILL_RESULT_CHARS = 300
# The directory of a trial that holds the files a case plants for a
# concrete-path request, outside the repository and every snapshot.
PLANTED = "guidance"
# A tree file's placeholder for the trial directory.
TRIAL_TOKEN = "{{TRIAL}}"
# The two compaction cases: the path is lost and the instructions remain, or
# the whole preamble is lost.
COMPACTION_MODES = ("path", "preamble")
# The descriptor variable of Ravel's skill-discovery specification.
DESCRIPTOR_VARIABLE = "RAVEL_SKILL_DIRECTORIES"
# The line that ends Ravel's preamble and opens the caller block.
CALLER_LABEL = "\nCaller task\nBEGIN RAVEL CALLER TASK "
# Terms of the preamble, in lower case. A summary that lost the whole preamble
# holds none of them, and the instructions of that case name none of them.
PREAMBLE_TERMS = (
    "skill director", "search director", "descriptor", "mandatory file", "obligation",
    "directories.json", ".cache/ravel", "ravel_skill",
)
# A preamble line this long or longer counts as preamble text when a context
# holds it word for word.
PREAMBLE_LINE_CHARS = 40
# The characters on each side of a term that the record shows.
SNIPPET_CHARS = 60
# The session-file entries that reach the model. Every other entry type is
# Claude Code's own record keeping.
MESSAGE_TYPES = ("user", "assistant", "attachment")


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
    mode = case.setdefault("mode", "lifecycle")
    if mode not in MODES:
        raise TrialError(f"case {name}: the mode '{mode}' is not one of {', '.join(MODES)}")
    if mode == "lifecycle" and not (directory / "tree").is_dir():
        raise TrialError(f"case {name}: no tree/ directory")
    if bool(case.get("moveAway")) != bool(case.get("expectRefusal")) or (case.get("moveAway") and mode != "lifecycle"):
        raise TrialError(f"case {name}: a lifecycle case names moveAway and expectRefusal together")
    for item in case.get("collect", []):
        if not isinstance(item, str) or Path(item).is_absolute() or ".." in Path(item).parts:
            raise TrialError(f"case {name}: a collected file is a path inside the trial directory, not {item!r}")
    if case.get("continuation") is not None:
        check_continuation(case, directory)
    if case.get("compaction") is not None:
        check_compaction(case, directory)
    if mode == "standalone":
        standalone = case.get("standalone") or {}
        if not (directory / standalone.get("prompt", "prompt.md")).is_file() or not standalone.get("outputs"):
            raise TrialError(f"case {name}: a standalone case names a prompt file and its outputs")
    return case, directory


def check_continuation(case: dict[str, Any], directory: Path) -> None:
    """A continuation case names the answer of the resumed turn, a marker,
    the edits that hold it, the changed mandatory files and the fresh tree."""
    spec = case["continuation"]
    name = case["name"]
    if case.get("mode") != "lifecycle" or case.get("moveAway"):
        raise TrialError(f"case {name}: a continuation case is a lifecycle case without moveAway")
    marker = spec.get("marker")
    if not marker:
        raise TrialError(f"case {name}: a continuation case names its marker")
    if not (directory / spec.get("answer", "answer.md")).is_file():
        raise TrialError(f"case {name}: no answer file for the resumed turn")
    if not (directory / spec.get("freshTree", "fresh-tree")).is_dir():
        raise TrialError(f"case {name}: no fresh-tree/ directory for the fresh launch")
    edits = spec.get("edits") or []
    if not edits or any(marker not in edit.get("insert", "") for edit in edits):
        raise TrialError(f"case {name}: each edit inserts text that holds the marker {marker}")
    plugins = [source["plugin"] for source in case.get("sources", [])]
    for edit in edits:
        if not any(edit["path"].startswith(plugin + "/") for plugin in plugins):
            raise TrialError(f"case {name}: the edit {edit['path']} lies in no source of the case")
    if "mandatory" not in spec:
        raise TrialError(f"case {name}: a continuation case names the mandatory files of the changed settings")


def check_compaction(case: dict[str, Any], directory: Path) -> None:
    """A compaction case is a lifecycle case on the directory route. It names
    its mode, the summary instructions and the answer of the turn after
    compaction. The agent sees the instructions after the boundary, so the
    instructions of the preamble case name no term of the preamble."""
    spec = case["compaction"]
    name = case["name"]
    if case.get("mode") != "lifecycle" or case.get("moveAway") or case.get("continuation") is not None:
        raise TrialError(f"case {name}: a compaction case is a lifecycle case without moveAway or continuation")
    if case.get("route") != "directory-2.0":
        raise TrialError(f"case {name}: a compaction case uses the route directory-2.0")
    if spec.get("mode") not in COMPACTION_MODES:
        raise TrialError(f"case {name}: the compaction mode is one of {', '.join(COMPACTION_MODES)}")
    instructions = spec.get("instructions", "")
    if not instructions.strip() or "\n" in instructions:
        raise TrialError(f"case {name}: a compaction case names one line of summary instructions")
    if spec["mode"] == "preamble":
        named = [term for term in PREAMBLE_TERMS if term in instructions.lower()]
        if named or DESCRIPTOR_VARIABLE in instructions:
            raise TrialError(f"case {name}: the instructions name {named or [DESCRIPTOR_VARIABLE]}, "
                             "which would remind the agent of the preamble")
    if not (directory / spec.get("answer", "answer.md")).is_file():
        raise TrialError(f"case {name}: no answer file for the turn after compaction")


def fixed_probes(case: dict[str, Any], home: Path, checkout: Path) -> list[list[str]]:
    """The probes that the runner knows before the launch: the protected
    configuration, each mandatory file at its origin, each native copy and a
    write outside the working directory. The configuration script adds the
    probes of the request's own paths, and a write inside the working
    directory. A native copy lies in the sandbox home, outside every grant of
    a confined launch, so its probe shows whether the agent could read it."""
    if not case.get("probes"):
        return []
    config = home / ".config" / "ravel"
    probes = [["read", str(config / "settings.json")], ["read", str(config / "config.rhai")]]
    plugins = {source["id"]: source["plugin"] for source in case.get("sources", [])}
    probes += [["read", str(checkout / plugins[item["source"]] / item["path"])] for item in case.get("mandatory", [])]
    probes += [["read", str(home / ".claude" / "skills" / item["name"] / "SKILL.md")] for item in case.get("native", [])]
    probes.append(["write", str(home / "trial-probe-outside")])
    return probes


def settings_for(case: dict[str, Any], checkout: Path, clones: Mapping[str, Path] | None = None) -> dict[str, Any]:
    """Ravel's owner settings of the case's route. A source in `clones` comes
    from that checkout instead of the prepared one."""
    if case["route"] == "native-2.0":
        return {}
    clones = clones or {}
    sources = [
        {"id": source["id"], "type": "local", "path": str(clones.get(source["id"], checkout) / source["plugin"]),
         "roots": source["roots"]}
        for source in case["sources"]
    ]
    return {"skills": {"schemaVersion": 1, "sources": sources, "mandatory": case.get("mandatory", [])}}


def rhai_string(text: str) -> str:
    return json.dumps(text)


def route_for(case: dict[str, Any], records: Path, program: Path, model: str, effort: str,
              probes: list[list[str]] | None = None) -> dict[str, Any]:
    """The route as data. The configuration script and the fake driver of the
    tests both take it from here. A standalone launch is confined, so its
    wrapper writes its record to standard output, `-`.

    A compaction case selects the feeder as its program, with the harness as
    the feeder's command. It gives no --add-dir: Claude Code writes additional
    directories into the system prompt, which survives compaction, so the
    snapshot paths would survive too."""
    compaction = case.get("compaction") is not None
    # An owner's instruction for one case, in Claude Code's owner channel. The
    # prepared prompt of Ravel stays as it is.
    note = ["--append-system-prompt", case["systemNote"]] if case.get("systemNote") else []
    selected = Path(sys.executable) if compaction else program
    program_args = ["-I", str(Path(__file__).resolve()), "turns", "--plan", str(Path(records).parent / "turns.json"),
                    "--", str(program)] if compaction else []
    return {
        "kinds": case["kinds"],
        "wrapper": str(WRAPPER),
        "records": "-" if case.get("mode") == "standalone" else str(records),
        "probes": probes or [],
        "probeRequest": bool(case.get("probes")),
        "program": str(selected),
        "programArgs": program_args,
        "addDirs": not compaction,
        "fixedArgs": [
            "-p", "--output-format", "stream-json", "--verbose",
            "--model", model, "--effort", effort, "--permission-mode", PERMISSION_MODE, *note,
        ],
        "model": model,
        "effort": effort,
    }


def config_for(case: dict[str, Any], route: dict[str, Any]) -> str:
    """The configuration script: the case's kinds to the wrapper, any other refuses.
    Each snapshot root also goes to --add-dir, as in the Rhai sample."""
    kinds = ", ".join(f"{rhai_string(kind)}: true" for kind in route["kinds"])
    probes = ""
    if route.get("probeRequest"):
        # The probes of the request's own paths come first, then the runner's.
        # The last probe writes inside the working directory of the launch.
        lines = [
            "    if request.skills != () {",
            '        args += ["--probe", "read", request.skills.descriptorPath];',
            '        for item in request.skills.mandatory { args += ["--probe", "read", item.path]; }',
            '        for source in request.skills.sources { args += ["--probe", "write", source.snapshotRoot + "/trial-probe"]; }',
            "    }",
        ]
        lines += [f'    args += ["--probe", {rhai_string(op)}, {rhai_string(path)}];' for op, path in route["probes"]]
        lines.append('    args += ["--probe", "write", "trial-probe-inside"];')
        probes = "\n".join(lines)
    fixed = route["fixedArgs"]
    records, program, model, effort = route["records"], route["program"], route["model"], route["effort"]
    program_args = ""
    if route.get("programArgs"):
        program_args = f"\n    args += [{', '.join(rhai_string(arg) for arg in route['programArgs'])}];"
    add_dirs = ""
    if route.get("addDirs", True):
        add_dirs = "\n".join([
            "",
            "    // --add-dir takes several values, so a fixed flag must follow the last",
            "    // root. Otherwise the prompt would become one more directory.",
            "    if request.skills != () {",
            '        for source in request.skills.sources { args += ["--add-dir", source.snapshotRoot]; }',
            "    }",
        ])
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
{probes}{program_args}{add_dirs}
    args += [{', '.join(rhai_string(arg) for arg in fixed)}];
    args.push(request.prompt);
    #{{ status: "selected", program: {rhai_string(route["wrapper"])}, args: args,
       provider: "anthropic", model: {rhai_string(model)}, effort: {rhai_string(effort)},
       reason: {rhai_string('harness trial ' + case['name'])} }}
}}
"""


# --- Events --------------------------------------------------------------------


def read_jsonl(path: Path) -> list[dict[str, Any]]:
    return read_jsonl_bytes(path.read_bytes())


def jsonl_lines(raw: bytes) -> list[bytes]:
    """The lines of a JSONL file, each with its newline. Only a newline ends a
    line: a JSON string can hold a raw U+2028, which str.splitlines splits."""
    return [line for line in re.split(rb"(?<=\n)", raw) if line]


def read_jsonl_bytes(raw: bytes) -> list[dict[str, Any]]:
    entries = []
    for raw_line in jsonl_lines(raw):
        line = raw_line.decode(errors="replace").strip()
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

    def __init__(self, view: dict[str, Any], home: Path, repo: Path, marker: str | None = None,
                 turns: bool = False):
        skills = view.get("skills") or {}
        # The text that only the changed bytes of a continuation case hold.
        self.marker = marker
        # A compaction case shows each user turn and the compaction boundary
        # among the events.
        self.turns = turns
        self.kind_skill = f"grove-{view.get('kind', '')}"
        self.mandatory = {self._norm(item["path"]) for item in skills.get("mandatory", [])}
        self.descriptor = self._norm(skills["descriptorPath"]) if skills.get("descriptorPath") else None
        self.snapshots = [(source["id"], self._norm(source["snapshotRoot"])) for source in skills.get("sources", [])]
        self.origins = [(source["id"], self._norm(source["originPath"])) for source in skills.get("sources", [])]
        # Claude Code's session files. After compaction the summary names the
        # full transcript, which holds the preamble.
        self.transcripts = self._norm(str(home / ".claude" / "projects"))
        self.native = [self._norm(str(home / ".claude")), self._norm(str(home / ".codex"))]
        # The files that a case plants for a concrete-path request.
        self.planted = self._norm(str(home.parent / PLANTED))
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
        if self._inside(path, self.transcripts):
            return "transcript", None
        for root in self.native:
            if self._inside(path, root):
                return "native", self._skill(path)
        if self._inside(path, self.planted):
            # A planted skill file names its skill by its own directory.
            return "planted", Path(path).parent.name if path.endswith("/SKILL.md") else None
        if self._inside(path, self.repo):
            return "work", None
        return "other", None

    def source_of(self, raw: str) -> str | None:
        """The id of the source whose snapshot or origin holds the path. Two
        sources can hold skills of one name, so the class alone cannot show
        which copy a read opened."""
        path = self._norm(raw)
        for source, root in self.snapshots + self.origins:
            if self._inside(path, root):
                return source
        return None

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
    "planted", "transcript",
)
# A Bash command that reads the environment: the descriptor variable by name,
# or `env` or `printenv` as a command word.
ENVIRONMENT_READ = re.compile(r"(?:^|[\s;&|(`$])(?:env|printenv)(?=$|[\s;&|)`])")
SEPARATORS = {"&&", "||", ";", "|", "&", "\n", "(", ")"}


ASSIGNMENT = re.compile(r"^([A-Za-z_][A-Za-z0-9_]*)=(.*)$", re.S)
VARIABLE = re.compile(r"\$(?:\{([A-Za-z_][A-Za-z0-9_]*)\}|([A-Za-z_][A-Za-z0-9_]*))")


def expand(token: str, variables: dict[str, str]) -> str | None:
    """The token with each `$NAME` or `${NAME}` that the command assigned
    replaced, or None when another expansion remains."""
    def value(match: re.Match[str]) -> str:
        name = match.group(1) or match.group(2)
        return variables[name] if name in variables else match.group(0)

    expanded = VARIABLE.sub(value, token)
    return None if "$" in expanded or "~" in expanded else expanded


def bash_paths(command: str, cwd: str) -> list[str]:
    """The paths that a Bash command names, in order. A `cd` changes the
    directory against which later relative paths resolve. A plain assignment
    `NAME=value` at the start of a command sets a variable, and a later
    `$NAME` or `${NAME}` takes its value. A relative token counts when it
    exists and is `.`, `..`, holds a slash or ends in `.md`. A token with any
    other shell expansion counts not at all."""
    import shlex

    try:
        lexer = shlex.shlex(command, posix=True, punctuation_chars=True)
        lexer.whitespace_split = True
        tokens = list(lexer)
    except ValueError:
        return PATH_TOKEN.findall(command)
    paths: list[str] = []
    variables: dict[str, str] = {}
    here = cwd
    first = True
    pending_cd = False
    for raw in tokens:
        if raw in SEPARATORS or set(raw) <= set("<>&|;"):
            first, pending_cd = True, False
            continue
        assignment = ASSIGNMENT.match(raw) if first else None
        if assignment:
            value = expand(assignment.group(2), variables)
            if value is None:
                variables.pop(assignment.group(1), None)
            else:
                variables[assignment.group(1)] = value
            continue
        token = expand(raw, variables)
        if first:
            first = False
            pending_cd = token == "cd"
            continue
        if token is None or token.startswith("-"):
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


def result_text(content: Any) -> str:
    """The text of one tool result, which is a string or a list of blocks."""
    if isinstance(content, str):
        return content
    if isinstance(content, list):
        return "\n".join(b.get("text", "") for b in content if isinstance(b, dict) and b.get("type") == "text")
    return ""


def marker_places(marker: str | None, tool_input: dict[str, Any], result: str) -> list[str]:
    """Where a tool call holds the marker: `input` for what the agent wrote or
    ran, `result` for the bytes that the call returned."""
    if not marker:
        return []
    places = []
    if marker in json.dumps(tool_input, ensure_ascii=False):
        places.append("input")
    if marker in result:
        places.append("result")
    return places


def turn_event(entry: dict[str, Any]) -> dict[str, Any] | None:
    """A user turn or the compaction boundary, as an event, or None. A turn is
    a user message of text that is neither a tool result, a meta message, the
    compaction summary nor the output of a local command. The `/compact`
    command shows as `/compact` and its arguments."""
    if entry.get("type") == "system" and entry.get("subtype") == "compact_boundary":
        return {"tool": "compaction", "class": "compaction", "target": "compact boundary"}
    if entry.get("type") != "user" or entry.get("isMeta") or entry.get("isCompactSummary"):
        return None
    content = (entry.get("message") or {}).get("content")
    if isinstance(content, list):
        if any(isinstance(b, dict) and b.get("type") == "tool_result" for b in content):
            return None
        content = result_text(content)
    if not isinstance(content, str) or not content.strip() or content.startswith("<local-command"):
        return None
    command = re.search(r"<command-name>(/[\w-]+)</command-name>", content)
    if command:
        args = re.search(r"<command-args>(.*?)</command-args>", content, re.S)
        content = command.group(1) + (" " + args.group(1).strip() if args and args.group(1).strip() else "")
    return {"tool": "prompt", "class": "prompt", "target": content.strip()}


def extract_events(entries: list[dict[str, Any]], classifier: Classifier) -> list[dict[str, Any]]:
    """The ordered tool calls and text blocks of one transcript, each with its
    class. A disclosure or a report is text, so its order against the tool
    calls needs the text blocks. A Skill call keeps its result text, which
    says whether the native loader matched. With a marker, each tool call
    names where it holds the marker, so a read shows which bytes it got."""
    errors: dict[str, bool] = {}
    results: dict[str, str] = {}
    for entry in entries:
        content = (entry.get("message") or {}).get("content")
        if isinstance(content, list):
            for block in content:
                if isinstance(block, dict) and block.get("type") == "tool_result":
                    errors[block.get("tool_use_id", "")] = bool(block.get("is_error"))
                    results[block.get("tool_use_id", "")] = result_text(block.get("content"))
    events = []
    for entry in entries:
        turn = turn_event(entry) if classifier.turns else None
        if turn:
            events.append({"seq": len(events) + 1, "error": None, "sidechain": bool(entry.get("isSidechain")), **turn})
            continue
        if entry.get("type") != "assistant":
            continue
        content = (entry.get("message") or {}).get("content")
        if not isinstance(content, list):
            continue
        for block in content:
            if isinstance(block, dict) and block.get("type") == "text" and block.get("text", "").strip():
                events.append({"seq": len(events) + 1, "tool": "text", "target": block["text"].strip(),
                               "error": None, "sidechain": bool(entry.get("isSidechain")), "class": "text"})
                continue
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
            if classifier.marker:
                event["marker"] = marker_places(classifier.marker, tool_input, results.get(block.get("id", ""), ""))
            reads = [item for item in classes if item[0] in READ_CLASSES]
            if reads and name not in ("Edit", "Write", "MultiEdit", "NotebookEdit"):
                event["class"], event["skill"], event["path"] = reads[0]
                source = classifier.source_of(event["path"])
                if source:
                    event["source"] = source
                if len(reads) > 1:
                    event["more"] = [{"class": c, "skill": s, "path": p, "source": classifier.source_of(p)}
                                     for c, s, p in reads[1:]]
            elif name in ("Edit", "Write", "MultiEdit", "NotebookEdit"):
                event["class"], event["path"] = "action", target
            elif name == "Skill":
                event["class"] = "native-skill-tool"
                event["resultText"] = results.get(block.get("id", ""), "")[:SKILL_RESULT_CHARS]
            elif name in ("Task", "Agent"):
                event["class"] = "delegate"
            elif name == "Bash":
                ending = re.search(r"end-session\s+([\w-]+)", target)
                event["class"] = "ending" if ending else "command"
                if ending:
                    event["result"] = ending.group(1)
                    # What the call printed, such as a refusal without context.
                    event["resultText"] = results.get(block.get("id", ""), "")[:SKILL_RESULT_CHARS]
                elif DESCRIPTOR_VARIABLE in target or ENVIRONMENT_READ.search(target):
                    event["class"] = "variable"
            elif classes:
                event["class"], event["skill"], event["path"] = classes[0]
            else:
                event["class"] = "other"
            events.append(event)
    return events


# A delegate prompt names these when it carries ending instructions. A
# prohibition names them too, so the record shows each line for the reader.
ENDING_TERMS = ("end-session", "grove-continue", "grove-force-stop", "record-teardown",
                "RAVEL_EXIT_FILE", "GROVE_LAUNCH_DIR")
# The ending context that a delegate inside the harness process inherits.
ENDING_NAMES = ("RAVEL_EXIT_FILE", "GROVE_LAUNCH_DIR")
DELEGATE_RESULT_CHARS = 600
DELEGATE_MECHANISM = ("Claude Code's Agent tool, in process: the delegate's tool calls run in children of "
                      "the harness process, with its environment")


def obligation_lines(prompt: str) -> list[str]:
    """The numbered obligation lines of a prepared prompt's preamble."""
    end = prompt.find(CALLER_LABEL)
    preamble = prompt[:end] if end >= 0 else ""
    start = preamble.find("\nObligations:\n")
    if start < 0:
        return []
    return [line.strip() for line in preamble[start:].splitlines() if re.match(r"\s*\d+\. ", line)]


def delegate_order(events: list[dict[str, Any]], mandatory: list[str]) -> dict[str, Any]:
    """The order of a delegate's reads: the mandatory files by first read, the
    class of its first guidance read, its first action, and whether it read
    every mandatory file, in the configured order, before that action."""
    first_seen: dict[str, int] = {}
    first_read = first_action = None
    for event in events:
        reads = [(event.get("class"), event.get("path"))] + [(m["class"], m["path"]) for m in event.get("more", [])]
        for cls, path in reads:
            if cls in READ_CLASSES and first_read is None:
                first_read = {"seq": event["seq"], "class": cls}
            if cls == "mandatory" and path and os.path.realpath(path) in mandatory:
                first_seen.setdefault(os.path.realpath(path), event["seq"])
        if event.get("class") == "action" and first_action is None:
            first_action = event["seq"]
    order = sorted(first_seen, key=lambda path: first_seen[path])
    return {
        "mandatoryOrder": order,
        "firstRead": first_read,
        "firstAction": first_action,
        "mandatoryBeforeAction": order == mandatory and (
            first_action is None or all(seq < first_action for seq in first_seen.values())),
    }


def delegations(entries: list[dict[str, Any]], session_dir: Path, home: Path, prompt: str | None,
                descriptor: str | None, child: Mapping[str, str],
                transcripts: list[dict[str, Any]], mandatory: list[str]) -> list[dict[str, Any]]:
    """Each Agent call of the parent: what the parent gave the delegate, the
    delegate's transcript and the ending context that it inherited. Claude
    Code links a delegate's transcript to the call through the `toolUseId` of
    `subagents/agent-<id>.meta.json` beside the transcript."""
    metas = {}
    for meta in sorted((session_dir / "subagents").glob("agent-*.meta.json")) if session_dir.is_dir() else []:
        data = json.loads(meta.read_text())
        metas[data.get("toolUseId")] = (meta, data)
    results = {}
    for entry in entries:
        content = (entry.get("message") or {}).get("content")
        for block in content if isinstance(content, list) else []:
            if isinstance(block, dict) and block.get("type") == "tool_result":
                results[block.get("tool_use_id")] = (result_text(block.get("content")), entry.get("toolUseResult"))
    obligations = obligation_lines(prompt or "")
    found = []
    for entry in entries:
        content = (entry.get("message") or {}).get("content")
        for block in content if isinstance(content, list) else []:
            if not (isinstance(block, dict) and block.get("type") == "tool_use" and block.get("name") in ("Task", "Agent")):
                continue
            tool_input = block.get("input") or {}
            text = tool_input.get("prompt", "")
            meta, data = metas.get(block.get("id"), (None, {}))
            transcript = None
            if meta:
                file = str(meta.with_name(meta.name.replace(".meta.json", ".jsonl")).relative_to(home))
                transcript = next((t for t in transcripts if t["file"] == file), None)
            result, tool_result = results.get(block.get("id"), ("", None))
            found.append({
                "toolUseId": block.get("id"),
                "description": tool_input.get("description"),
                "subagentType": tool_input.get("subagent_type"),
                "background": tool_input.get("run_in_background"),
                "prompt": text,
                "promptSha256": sha256_bytes(text.encode()),
                "givesDescriptorPath": bool(descriptor) and descriptor in text,
                "namesVariable": DESCRIPTOR_VARIABLE in text,
                "obligations": [line.split(".", 1)[0] for line in obligations if line in text],
                "obligationCount": len(obligations),
                "endingLines": [line.strip()[:COMMAND_CHARS] for line in text.splitlines()
                                if any(term in line for term in ENDING_TERMS)],
                "meta": {key: data.get(key) for key in ("agentType", "requestShape", "spawnDepth", "finished")
                         if key in data},
                "status": (tool_result or {}).get("status") if isinstance(tool_result, dict) else None,
                "result": result[:DELEGATE_RESULT_CHARS],
                "transcript": transcript["file"] if transcript else None,
                "order": delegate_order(transcript["events"], mandatory) if transcript else None,
                "mechanism": DELEGATE_MECHANISM,
                "inherited": [name for name in ENDING_NAMES if name in child],
            })
    return found


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
    fetch_checkout(common, full, checkout, git_env(environ))

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
    """The case's tree as `.grove/` of a new jj repository. A task can name a
    concrete path of the trial, so `{{TRIAL}}` in a tree file becomes the
    trial directory."""
    repo.mkdir()
    run_checked(["jj", "--config", "git.colocate=false", "git", "init", "."], cwd=repo, env=env)
    shutil.copytree(tree, repo / ".grove")
    for path in (repo / ".grove").rglob("*.md"):
        text = path.read_text()
        if TRIAL_TOKEN in text:
            path.write_text(text.replace(TRIAL_TOKEN, str(repo.parent)))
    run_checked(["jj", "commit", "--quiet", "--message", "trial tree"], cwd=repo, env=env)
    return run_checked(["jj", "log", "--no-graph", "-r", "@-", "-T", "commit_id"], cwd=repo, env=env)


def git_env(environ: Mapping[str, str]) -> dict[str, str]:
    """Git without the owner's global or system configuration."""
    return {**environ, "GIT_CONFIG_GLOBAL": "/dev/null", "GIT_CONFIG_NOSYSTEM": "1"}


def fetch_checkout(repository: str | Path, commit: str, target: Path, env: dict[str, str]) -> None:
    """A plain Git checkout of `commit` at `target`, fetched by its hash."""
    run_checked(["git", "init", "--quiet", str(target)], env=env)
    run_checked(["git", "fetch", "--quiet", str(repository), commit], cwd=target, env=env)
    run_checked(["git", "checkout", "--quiet", "--detach", "FETCH_HEAD"], cwd=target, env=env)
    if run_checked(["git", "status", "--porcelain"], cwd=target, env=env):
        raise TrialError(f"the checkout {target} is not clean")


def copy_from_commit(checkout: Path, commit: str, directory: str, target: Path) -> None:
    """The files of `directory` as the commit holds them, written to `target`."""
    env = git_env(os.environ)
    files = run_checked(["git", "ls-tree", "-r", "--name-only", commit, "--", directory], cwd=checkout, env=env)
    if not files:
        raise TrialError(f"the commit {commit} of {checkout} holds no files under {directory}")
    for name in files.splitlines():
        data = subprocess.run(["git", "show", f"{commit}:{name}"], cwd=checkout, env=env,
                              capture_output=True, check=True).stdout
        path = target / Path(name).relative_to(directory)
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(data)


def install_native(case: dict[str, Any], home: Path, checkout: Path) -> list[dict[str, str]]:
    """The case's native copies, each a skill directory of the checkout copied
    into the sandbox `.claude/skills`, never into the owner's home. With a
    `commit`, the copy holds the directory as that older commit held it, such
    as a stale native copy beside the current directory copy."""
    installed = []
    for item in case.get("native", []):
        target = home / ".claude" / "skills" / item["name"]
        entry = {"name": item["name"], "from": item["from"], "path": str(target.relative_to(home))}
        if item.get("commit"):
            commit = run_checked(["git", "rev-parse", "--verify", f"{item['commit']}^{{commit}}"],
                                 cwd=checkout, env=git_env(os.environ))
            copy_from_commit(checkout, commit, item["from"], target)
            current = checkout / item["from"] / "SKILL.md"
            entry.update(commit=commit, currentSha256=sha256_file(current) if current.is_file() else None)
        else:
            shutil.copytree(checkout / item["from"], target)
        entry["skillSha256"] = sha256_file(target / "SKILL.md")
        installed.append(entry)
    return installed


def expected_native(case: dict[str, Any]) -> list[str]:
    """The native copies that the case declares, as `skill_files` names them."""
    return sorted(f".claude/skills/{item['name']}/SKILL.md" for item in case.get("native", []))


def native_failures(native: list[str], expected: list[str]) -> list[str]:
    if native == expected:
        return []
    return [f"the sandbox home holds the native copies {native}, and the case declares {expected}"]


def write_claude_settings(case: dict[str, Any], home: Path) -> dict[str, Any] | None:
    """The case's `.claude/settings.json` in the sandbox home, such as a deny
    rule that makes the native loader refuse a skill it holds."""
    settings = case.get("claudeSettings")
    if settings is None:
        return None
    path = home / ".claude" / "settings.json"
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(settings, indent=2) + "\n")
    return settings


def plant_files(case: dict[str, Any], trial: Path, checkout: Path) -> list[dict[str, str]]:
    """The case's planted files: checkout files copied under the trial's
    `guidance/`, for a concrete-path request."""
    planted = []
    for item in case.get("plant", []):
        target = trial / item["to"]
        if Path(item["to"]).parts[:1] != (PLANTED,) or ".." in Path(item["to"]).parts:
            raise TrialError(f"case {case['name']}: a planted file must lie under {PLANTED}/, not {item['to']}")
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(checkout / item["from"], target)
        planted.append({"from": item["from"], "path": item["to"], "sha256": sha256_file(target)})
    return planted


def tree_state(repo: Path) -> list[str]:
    grove = repo / ".grove"
    if not grove.is_dir():
        return []
    return sorted(str(path.relative_to(grove)) for path in grove.rglob("*.md"))


def run_driver(argv: list[str], cwd: Path, env: dict[str, str], out: Path, timeout_s: float,
               name: str = "driver") -> dict[str, Any]:
    """Run the driver in a session of its own, and stop it at the deadline.
    Its output goes to `NAME.stdout` and `NAME.stderr` in `out`."""
    started = time.monotonic()
    with (out / f"{name}.stdout").open("wb") as stdout, (out / f"{name}.stderr").open("wb") as stderr:
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


def parse_probes(text: str) -> list[dict[str, str]]:
    """The `trial-probe OP STATUS PATH` lines of the wrapper, in order."""
    probes = []
    for line in text.splitlines():
        parts = line.split(" ", 3)
        if len(parts) == 4 and parts[0] == "trial-probe":
            probes.append({"op": parts[1], "status": parts[2], "path": parts[3]})
    return probes


def read_window(record: Path) -> tuple[int, int] | None:
    """The lines of the session file that belong to this launch. A resumed
    turn appends to the session file of the first turn, so a continuation
    case writes `lines` into each turn's record."""
    file = record / "lines"
    if not file.exists():
        return None
    start, end = (int(part) for part in file.read_text().split())
    return start, end


def describe_launch(record: Path, home: Path, repo: Path, live: Mapping[str, str],
                    marker: str | None = None, turns: bool = False) -> dict[str, Any]:
    """One launch of a lifecycle case, from its record directory."""
    stream_file = record / "stream.jsonl"
    probes = record / "probes"
    prompt = record / "prompt"
    return describe(
        str(record), json.loads((record / "view.json").read_text()), parse_env((record / "env").read_text()),
        read_jsonl(stream_file) if stream_file.exists() else [],
        sha256_file(stream_file) if stream_file.exists() else None,
        parse_probes(probes.read_text()) if probes.exists() else [],
        home, repo, live, marker, read_window(record), turns,
        prompt.read_text() if prompt.exists() else None,
    )


def split_log(text: str) -> list[dict[str, Any]]:
    """The launches of a confined standalone run, from Grove's run log. The
    wrapper begins each launch with its `trial-view-b64` line. The JSON lines
    after it are the stream of the harness, and other lines stay as text."""
    import base64

    launches: list[dict[str, Any]] = []
    for line in text.splitlines():
        head, _, rest = line.partition(" ")
        if head == "trial-view-b64":
            launches.append({"view": json.loads(base64.b64decode(rest)), "env": {}, "probes": [],
                             "stream": [], "streamLines": []})
        elif not launches:
            continue
        elif head == "trial-env-b64":
            launches[-1]["env"] = parse_env(base64.b64decode(rest).decode(errors="replace"))
        elif head == "trial-probe":
            launches[-1]["probes"] += parse_probes(line)
        elif head == "trial-started":
            continue
        else:
            stripped = line.strip()
            try:
                value = json.loads(stripped) if stripped.startswith("{") else None
            except json.JSONDecodeError:
                value = None
            if isinstance(value, dict):
                launches[-1]["stream"].append(value)
                launches[-1]["streamLines"].append(stripped)
    return launches


def log_lines(text: str) -> list[str]:
    """The lines of a run log that are neither the wrapper's record nor the
    stream: Grove's and Ravel's own diagnostics, also of a refused launch."""
    lines = []
    for line in text.splitlines():
        stripped = line.strip()
        if not stripped or stripped.startswith(("trial-", "{")):
            continue
        lines.append(stripped)
    return lines


def describe(label: str, view: dict[str, Any], child: dict[str, str], stream: list[dict[str, Any]],
             stream_sha: str | None, probes: list[dict[str, str]], home: Path, repo: Path,
             live: Mapping[str, str], marker: str | None = None,
             window: tuple[int, int] | None = None, turns: bool = False,
             prompt: str | None = None) -> dict[str, Any]:
    """One launch: its request view, the environment of its harness, its
    probes and its ordered events. The events come from the session file of
    the harness. When the harness wrote none, as a confined harness cannot,
    they come from its stream, which holds the same tool calls. A window
    limits the main session file to the lines of this launch, and the digest
    then covers those lines only. Each Agent call of the main transcript adds
    a delegation, which the prompt of the launch lets the record compare with
    the obligations of the preamble."""
    init = next((e for e in stream if e.get("type") == "system" and e.get("subtype") == "init"), {})
    session_id = init.get("session_id")
    classifier = Classifier(view, home, Path(init.get("cwd") or repo), marker, turns)
    transcripts = []
    main_entries: list[dict[str, Any]] = []
    session_dir = None
    for path in session_files(home):
        if session_id and session_id not in str(path):
            continue
        main = path.stem == session_id
        raw = path.read_bytes()
        if main and window:
            raw = b"".join(jsonl_lines(raw)[window[0]:window[1]])
        entries = read_jsonl_bytes(raw)
        if main:
            main_entries, session_dir = entries, path.parent / path.stem
        transcripts.append({
            "file": str(path.relative_to(home)),
            "sha256": sha256_bytes(raw),
            **({"lines": list(window)} if main and window else {}),
            "main": main,
            "events": extract_events(entries, classifier),
            "startup": native_startup(stream, entries) if main else None,
        })
    if session_id and not any(t["main"] for t in transcripts):
        transcripts.append({
            "file": "stream", "sha256": stream_sha, "main": True,
            "events": extract_events(stream, classifier), "startup": native_startup(stream, []),
        })
    skills = view.get("skills") or {}
    descriptor = skills.get("descriptorPath")
    mandatory = [os.path.realpath(item["path"]) for item in skills.get("mandatory", [])]
    delegated = delegations(main_entries, session_dir, home, prompt, descriptor, child, transcripts,
                            mandatory) if session_dir else []
    descriptor_identity = None
    if descriptor and Path(descriptor).is_file():
        descriptor_identity = {
            "directory": Path(descriptor).parent.name,
            "sha256": sha256_file(Path(descriptor)),
        }
    return {
        "record": label,
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
        "probes": probes,
        "streamSha256": stream_sha,
        "startup": native_startup(stream, []) if not transcripts else None,
        "transcripts": transcripts,
        "delegations": delegated,
        # The last result event of the stream, absent when Ravel ended the
        # harness before it wrote one.
        "streamResult": next((e.get("subtype") for e in reversed(stream) if e.get("type") == "result"), None),
    }


def resolution_path(front: Path) -> str:
    """The PATH of the offline resolution: the 2.0 front alone, with no `git`."""
    return str(front.parent)


def resolve_offline(front: Path, env: dict[str, str], out: Path) -> dict[str, Any]:
    """`ravel skills resolve --json` with the settings of the launch, under a
    profile that denies the network and with no `git` on PATH. Two controls
    run first, and each must be seen to differ: an unguarded probe reaches a
    local listener that the guarded probe cannot reach, and `git` is on the
    usual PATH but not on the PATH of the resolution."""
    import socket

    listener = socket.socket()
    listener.bind(("127.0.0.1", 0))
    listener.listen(4)
    port = str(listener.getsockname()[1])
    try:
        probe = ["/usr/bin/nc", "-z", "-w", "2", "127.0.0.1", port]
        unguarded = subprocess.run(probe, capture_output=True).returncode
        guarded = subprocess.run(["/usr/bin/sandbox-exec", "-p", NO_NETWORK, *probe], capture_output=True).returncode
    finally:
        listener.close()
    path = resolution_path(front)
    git_usual = shutil.which("git", path=env["PATH"])
    git_resolution = shutil.which("git", path=path)
    result = subprocess.run(
        ["/usr/bin/sandbox-exec", "-p", NO_NETWORK, str(front), "skills", "resolve", "--json"],
        env={**env, "PATH": path}, cwd=out, capture_output=True, text=True,
    )
    (out / "resolve.json").write_text(result.stdout)
    (out / "resolve.stderr").write_text(result.stderr)
    document: dict[str, Any] = {}
    try:
        document = json.loads(result.stdout)
    except json.JSONDecodeError:
        pass
    descriptor = document.get("descriptorPath")
    identity = None
    if descriptor and Path(descriptor).is_file():
        identity = {"directory": Path(descriptor).parent.name, "sha256": sha256_file(Path(descriptor))}
    return {
        "profile": NO_NETWORK,
        "path": path,
        "networkControl": {"unguardedExit": unguarded, "guardedExit": guarded},
        "gitControl": {"usualPath": git_usual, "resolutionPath": git_resolution},
        "exit": result.returncode,
        "status": document.get("status"),
        "descriptorPath": descriptor,
        "descriptorIdentity": identity,
        "sources": [
            {key: source.get(key) for key in ("id", "resourceDigest")} for source in document.get("sources", [])
        ],
        "stderr": result.stderr.strip()[-2000:],
    }


def resolve_failures(resolve: dict[str, Any], launches: list[dict[str, Any]]) -> list[str]:
    """Each way the offline resolution does not prove its claim."""
    failures = []
    network = resolve["networkControl"]
    if network["unguardedExit"] != 0 or network["guardedExit"] == 0:
        failures.append(f"the network control did not differ: {network}")
    if not resolve["gitControl"]["usualPath"] or resolve["gitControl"]["resolutionPath"]:
        failures.append(f"the git control did not differ: {resolve['gitControl']}")
    if resolve["exit"] != 0 or resolve["status"] != "resolved":
        failures.append(f"the offline resolution failed with {resolve['exit']}: {resolve['stderr']}")
    identities = {json.dumps(launch["descriptorIdentity"], sort_keys=True) for launch in launches}
    if identities != {json.dumps(resolve["descriptorIdentity"], sort_keys=True)}:
        failures.append(
            f"the offline resolution gave the descriptor {resolve['descriptorIdentity']}, "
            f"and the launches received {sorted(identities)}"
        )
    return failures


def clone_source(case: dict[str, Any], trial: Path, checkout: Path, commit: str) -> dict[str, Any] | None:
    """For a case with `moveAway`, a plain Git checkout of the prepared commit
    in the trial directory. The settings name it for that source, so that the
    move changes no checkout that another case reads."""
    source = case.get("moveAway")
    if not source:
        return None
    if source not in {item["id"] for item in case.get("sources", [])}:
        raise TrialError(f"case {case['name']}: moveAway names '{source}', which is not one of its sources")
    clone = trial / "sources" / f"{source}-checkout"
    clone.parent.mkdir()
    fetch_checkout(checkout, commit, clone, git_env(os.environ))
    head = run_checked(["git", "rev-parse", "HEAD"], cwd=clone, env=git_env(os.environ))
    return {"source": source, "checkout": str(clone), "commit": head,
            "movedTo": str(trial / "sources" / f"{source}-moved")}


def move_away(moved: dict[str, Any]) -> None:
    os.rename(moved["checkout"], moved["movedTo"])


def restore(moved: dict[str, Any]) -> None:
    if os.path.lexists(moved["movedTo"]):
        os.rename(moved["movedTo"], moved["checkout"])


def resolve_direct(front: Path, env: dict[str, str], cwd: Path) -> dict[str, Any]:
    """`ravel skills resolve --json` with the settings of the launch. It
    resolves and publishes sources, and it launches no worker or harness."""
    result = subprocess.run([str(front), "skills", "resolve", "--json"], env=env, cwd=cwd,
                            capture_output=True, text=True, check=False)
    document: dict[str, Any] = {}
    try:
        document = json.loads(result.stdout)
    except json.JSONDecodeError:
        pass
    lines = result.stderr.strip().splitlines()
    return {"exit": result.returncode, "status": document.get("status"),
            "firstLine": lines[0] if lines else "",
            "remedy": next((line.strip() for line in lines if line.strip().startswith("remedy:")), None)}


# --- Continuation ------------------------------------------------------------


def insert_after_front_matter(text: str, addition: str) -> str:
    """`addition` as its own paragraph directly after the front matter, so a
    read of the first lines of the file also returns it."""
    lines = text.splitlines(keepends=True)
    if lines and lines[0].strip() == "---":
        for index in range(1, len(lines)):
            if lines[index].strip() == "---":
                return "".join(lines[:index + 1]) + "\n" + addition.rstrip("\n") + "\n" + "".join(lines[index + 1:])
    return addition.rstrip("\n") + "\n\n" + text


def commit_edits(clone: Path, edits: list[dict[str, str]]) -> dict[str, Any]:
    """One new commit in the source clone that inserts each edit's text. The
    snapshots of earlier launches keep the bytes they captured."""
    env = {**git_env(os.environ), "GIT_AUTHOR_NAME": "Grove Trial", "GIT_AUTHOR_EMAIL": "grove-trial@example.com",
           "GIT_COMMITTER_NAME": "Grove Trial", "GIT_COMMITTER_EMAIL": "grove-trial@example.com"}
    before = run_checked(["git", "rev-parse", "HEAD"], cwd=clone, env=env)
    files = []
    for edit in edits:
        path = clone / edit["path"]
        old = path.read_bytes()
        path.write_text(insert_after_front_matter(old.decode(), edit["insert"]))
        files.append({"path": edit["path"], "sha256Before": sha256_bytes(old), "sha256After": sha256_file(path)})
    run_checked(["git", "add", "--all"], cwd=clone, env=env)
    run_checked(["git", "commit", "--quiet", "-m", "trial: change the read files between the turns"], cwd=clone, env=env)
    after = run_checked(["git", "rev-parse", "HEAD"], cwd=clone, env=env)
    return {"checkout": str(clone), "commitBefore": before, "commitAfter": after, "files": files}


def stream_init(record: Path) -> tuple[str | None, str | None]:
    """The session ID and the working directory of one launch, from its stream."""
    stream = record / "stream.jsonl"
    entries = read_jsonl(stream) if stream.exists() else []
    init = next((e for e in entries if e.get("type") == "system" and e.get("subtype") == "init"), {})
    return init.get("session_id"), init.get("cwd")


def session_file(home: Path, session_id: str) -> Path | None:
    return next((path for path in session_files(home) if path.stem == session_id), None)


def write_window(record: Path, start: int, end: int) -> None:
    (record / "lines").write_text(f"{start} {end}\n")


def resume_turn(records: Path, view: dict[str, Any], program: Path, route: dict[str, Any], session_id: str,
                answer: str, cwd: Path, env: dict[str, str], out: Path, timeout_s: float) -> dict[str, Any]:
    """The resumed turn: the harness alone, outside Ravel, through the launch
    wrapper, which records its view, its prompt and its environment. The
    arguments are the fixed arguments of the route, `--resume` and the answer,
    without `--add-dir`, as an owner's manual resume gives them."""
    args = [*route["fixedArgs"], "--resume", session_id]
    argv = [str(WRAPPER), str(records), json.dumps(view), str(program), *args, answer]
    driver = run_driver(argv, cwd, env, out, timeout_s, name="resume")
    return {"args": args, "cwd": str(cwd), "answerSha256": sha256_bytes(answer.encode()), **driver}


def add_fresh_tree(repo: Path, tree: Path, env: dict[str, str]) -> dict[str, Any]:
    """The fresh-launch leaf, committed into the trial tree in a change of its
    own, as the owner adds work. An existing file is never replaced."""
    run_checked(["jj", "new", "--quiet"], cwd=repo, env=env)
    added = []
    for path in sorted(tree.rglob("*.md")):
        target = repo / ".grove" / path.relative_to(tree)
        if target.exists():
            raise TrialError(f"the fresh tree would replace {target}")
        target.parent.mkdir(parents=True, exist_ok=True)
        text = path.read_text()
        target.write_text(text.replace(TRIAL_TOKEN, str(repo.parent)))
        added.append(str(target.relative_to(repo / ".grove")))
    run_checked(["jj", "commit", "--quiet", "--message", "trial: add the fresh-launch leaf"], cwd=repo, env=env)
    commit = run_checked(["jj", "log", "--no-graph", "-r", "@-", "-T", "commit_id"], cwd=repo, env=env)
    return {"added": added, "commit": commit}


def marker_files(repo: Path, marker: str) -> list[str]:
    """The files of the trial repository, outside `.grove/` and the VCS
    stores, that hold the marker: what the agent wrote under the new bytes."""
    found = []
    for directory, subdirs, files in os.walk(repo):
        subdirs[:] = [d for d in subdirs if not (Path(directory) == repo and d in (".jj", ".git", ".grove"))]
        for name in files:
            path = Path(directory) / name
            try:
                if marker in path.read_text(errors="replace"):
                    found.append(str(path.relative_to(repo)))
            except OSError:
                continue
    return sorted(found)


def snapshot_markers(case: dict[str, Any], launch: dict[str, Any]) -> dict[str, bool | None]:
    """For each edited file, whether this launch's snapshot copy holds the
    marker. None when the launch has no copy of the file."""
    spec = case.get("continuation") or {}
    plugins = {source["id"]: source["plugin"] for source in case.get("sources", [])}
    roots = {source["id"]: source.get("snapshotRoot") for source in launch.get("sources", [])}
    found: dict[str, bool | None] = {}
    for edit in spec.get("edits", []):
        found[edit["path"]] = None
        for source, plugin in plugins.items():
            if edit["path"].startswith(plugin + "/") and roots.get(source):
                copy = Path(roots[source]) / edit["path"][len(plugin) + 1:]
                if copy.is_file():
                    found[edit["path"]] = spec["marker"] in copy.read_text(errors="replace")
    return found


def continue_case(case: dict[str, Any], case_dir: Path, trial: Path, home: Path, repo: Path, records: Path,
                  env: dict[str, str], route: dict[str, Any], program: Path, settings: dict[str, Any],
                  source_clone: Path, driver_argv: list[str], timeout_s: float) -> dict[str, Any]:
    """After the first turn: one commit in the source clone and a settings
    change, then the resumed turn, then a fresh launch on a new leaf."""
    spec = case["continuation"]
    marker = spec["marker"]
    result: dict[str, Any] = {
        "mechanism": "claude -p --resume <session id>, without --fork-session, outside Ravel's managed process",
        "marker": marker, "failures": [],
    }
    first = launch_records(records)
    if len(first) != 1:
        result["failures"].append(f"the first driver run made {len(first)} launches, and a continuation resumes one")
        return result
    session_id, cwd = stream_init(first[0])
    transcript = session_file(home, session_id) if session_id else None
    if not transcript:
        result["failures"].append("the first turn wrote no session file to resume")
        return result
    first_end = len(jsonl_lines(transcript.read_bytes()))
    write_window(first[0], 0, first_end)
    result["sessionId"] = session_id

    result["edit"] = commit_edits(source_clone, spec["edits"])
    changed = json.loads(json.dumps(settings))
    changed["skills"]["mandatory"] = spec["mandatory"]
    (home / ".config" / "ravel" / "settings.json").write_text(json.dumps(changed, indent=2) + "\n")
    result["settings"] = {"mandatoryBefore": settings["skills"]["mandatory"], "mandatoryAfter": spec["mandatory"]}
    say(f"{case['name']}: committed {result['edit']['commitAfter'][:12]} in the source clone and changed the settings")

    view = json.loads((first[0] / "view.json").read_text())
    view["continuation"] = {"resumes": session_id,
                            "note": "no configuration script ran; the skills fields are those of the first turn"}
    answer = (case_dir / spec.get("answer", "answer.md")).read_text()
    say(f"{case['name']}: resuming session {session_id} outside Ravel")
    result["resume"] = resume_turn(records, view, program, route, session_id, answer,
                                   Path(cwd or repo), env, trial, timeout_s)
    resumed = [record for record in launch_records(records) if record != first[0]]
    if len(resumed) != 1:
        result["failures"].append(f"the resumed turn left {len(resumed)} launch records, not one")
        return result
    resumed_id, _ = stream_init(resumed[0])
    if resumed_id != session_id:
        result["failures"].append(f"the resumed turn reports the session {resumed_id}, not {session_id}")
    else:
        write_window(resumed[0], first_end, len(jsonl_lines(transcript.read_bytes())))
    result["markerFilesAfterResume"] = marker_files(repo, marker)

    result["freshTree"] = add_fresh_tree(repo, case_dir / spec.get("freshTree", "fresh-tree"), env)
    say(f"{case['name']}: fresh launch on {', '.join(result['freshTree']['added'])}")
    fresh = run_driver(driver_argv, repo, env, trial, timeout_s, name="driver-fresh")
    stderr = (trial / "driver-fresh.stderr").read_text(errors="replace")
    result["freshDriver"] = {**fresh, "lines": [line for line in stderr.splitlines()
                                                 if line.startswith(("grove", "ravel:", "Error", "  remedy:"))]}
    result["markerFilesAfterFresh"] = marker_files(repo, marker)
    if len(launch_records(records)) < 3:
        result["failures"].append("the fresh driver run launched no harness; read driver-fresh.stderr of the trial")
    return result


# --- Compaction ----------------------------------------------------------------


def preamble_marks(prompt: str) -> dict[str, Any]:
    """What identifies the preamble of a prepared prompt: its absolute paths,
    the 64-digit digests in them, its long lines and the descriptor variable.
    The preamble is the text before Ravel's caller label."""
    end = prompt.find(CALLER_LABEL)
    if end < 0:
        raise TrialError("the prompt has no Ravel caller block, so it has no preamble to lose")
    preamble = prompt[:end]
    paths = sorted(set(re.findall(r'"(/[^"]+)"', preamble)))
    return {
        "paths": paths,
        "digests": sorted(set(re.findall(r"[0-9a-f]{64}", preamble))),
        "lines": sorted({line.strip() for line in preamble.splitlines() if len(line.strip()) >= PREAMBLE_LINE_CHARS}),
        "sha256": sha256_bytes(preamble.encode()),
    }


def strings(value: Any) -> list[str]:
    """Every string in a JSON value, in order."""
    if isinstance(value, str):
        return [value]
    if isinstance(value, dict):
        return [text for item in value.values() for text in strings(item)]
    if isinstance(value, list):
        return [text for item in value for text in strings(item)]
    return []


def context_findings(text: str, words: str, marks: dict[str, Any]) -> dict[str, Any]:
    """The preamble text that a context holds. `text` is all of it. `words`
    is the part that Claude Code did not add itself, such as a skill listing,
    and only that part is searched for the terms of the preamble."""
    lowered = words.lower()
    snippets = []
    for term in PREAMBLE_TERMS:
        for match in re.finditer(re.escape(term), lowered):
            start, end = max(0, match.start() - SNIPPET_CHARS), match.end() + SNIPPET_CHARS
            snippets.append(words[start:end].replace("\n", " "))
    return {
        "paths": [path for path in marks["paths"] if path in text],
        "digests": [digest[:12] for digest in marks["digests"] if digest in text],
        "lines": [line[:PREAMBLE_LINE_CHARS] for line in marks["lines"] if line in text],
        "terms": [term for term in PREAMBLE_TERMS if term in lowered],
        "variable": DESCRIPTOR_VARIABLE in text,
        "snippets": snippets[:8],
    }


def compaction_problems(findings: dict[str, Any], mode: str, terms: bool = True) -> list[str]:
    """Each way a context after compaction does not show the case's loss.
    Path loss: no path or digest of the preamble remains, and the instruction
    to read the variable does. Preamble loss: no text of the preamble remains."""
    problems = []
    if findings["paths"] or findings["digests"]:
        problems.append(f"the context still holds the preamble paths {findings['paths']} "
                        f"or digests {findings['digests']}")
    if mode == "path" and not findings["variable"]:
        problems.append(f"the context no longer names {DESCRIPTOR_VARIABLE}, so the instructions did not remain")
    if mode == "preamble":
        if findings["lines"]:
            problems.append(f"the context holds preamble lines {findings['lines']}")
        if findings["variable"]:
            problems.append(f"the context names {DESCRIPTOR_VARIABLE}")
        if terms and findings["terms"]:
            problems.append(f"the context holds preamble terms {findings['terms']}")
    return problems


def boundary_index(entries: list[dict[str, Any]]) -> int | None:
    """The index of the last compaction boundary of a session, or None."""
    found = [n for n, e in enumerate(entries) if e.get("type") == "system" and e.get("subtype") == "compact_boundary"]
    return found[-1] if found else None


def after_boundary(entries: list[dict[str, Any]], marks: dict[str, Any]) -> dict[str, Any] | None:
    """The findings in the session lines after the last boundary and before
    the next turn: the summary, the command, its output and the attachments.
    That is what the feeder's gate reads. The terms are searched in the user
    messages only, because a native attachment such as the skill listing is no
    text of the preamble."""
    start = boundary_index(entries)
    if start is None:
        return None
    rest = []
    for entry in entries[start + 1:]:
        turn = turn_event(entry)
        if turn and turn["class"] == "prompt" and not turn["target"].startswith("/compact"):
            break
        # Claude Code's own bookkeeping, such as `last-prompt`, which holds the
        # first prompt, never reaches the model.
        if entry.get("type") in MESSAGE_TYPES:
            rest.append(entry)
    text = "\n".join(s for e in rest for s in strings(e))
    words = "\n".join(s for e in rest if e.get("type") == "user" for s in strings(e.get("message")))
    return context_findings(text, words, marks)


def api_context(entries: list[dict[str, Any]], marks: dict[str, Any]) -> dict[str, Any]:
    """The context of the first model request after the last boundary, from
    the request records of the session file. The request names its message
    blobs by hash, `keep` messages of the request before it, and the shape that
    holds the system prompt. With `keep` 0 the blobs and the system prompt are
    the whole context that the model received."""
    start = boundary_index(entries)
    request = next((e for e in entries[(start or 0) + 1:] if e.get("type") == "api-request"), None) \
        if start is not None else None
    if request is None:
        return {"found": False}
    blobs = {e.get("hash"): e for e in entries if e.get("type") == "api-request-blob"}
    shapes = {e.get("shapeHash"): e for e in entries if e.get("type") == "api-request-shape"}
    tail = request.get("tail") or []
    texts = [s for h in tail if h in blobs for s in strings(blobs[h].get("message"))]
    shape = shapes.get(request.get("shapeHash"))
    system = strings((shape or {}).get("shape", {}).get("system")) if shape else []
    text = "\n".join(texts + system)
    return {
        "found": True,
        "keep": request.get("keep"),
        "blobs": len(tail),
        "missingBlobs": sum(h not in blobs for h in tail),
        "system": shape is not None,
        "requestDigest": request.get("requestDigest"),
        "findings": context_findings(text, text, marks),
    }


def gate_session(home: Path, session: str | None, marks: dict[str, Any], mode: str) -> dict[str, Any]:
    """The feeder's check before the answer turn: the session lines after the
    boundary show the case's loss."""
    path = session_file(home, session) if session else None
    if not path:
        return {"ok": False, "problems": ["no session file to check"]}
    findings = after_boundary(read_jsonl(path), marks)
    if findings is None:
        return {"ok": False, "problems": ["the session file holds no compaction boundary"]}
    problems = compaction_problems(findings, mode)
    return {"ok": not problems, "problems": problems, "findings": findings}


def run_turns(plan: Path, command: list[str]) -> int:
    """The feeder of a compaction case. It runs the harness with stream-json
    input and passes the stream through. It sends the prompt, which is the last
    argument, then each turn of the plan after the result of the turn before.
    Before a gated turn it checks the session file, and a failed check ends
    the input with no further turn. Each step is a `trial-*` line on stderr."""
    spec = json.loads(plan.read_text())
    if not command:
        raise TrialError("the feeder needs a harness command")
    *argv, prompt = command
    marks = preamble_marks(prompt)
    home = Path(os.environ["HOME"])
    child = subprocess.Popen([*argv, "--input-format", "stream-json"], stdin=subprocess.PIPE, stdout=subprocess.PIPE)
    assert child.stdin is not None and child.stdout is not None
    pending = list(spec["turns"])

    def log(kind: str, data: dict[str, Any]) -> None:
        stamp = dt.datetime.now(dt.UTC).strftime("%Y-%m-%dT%H:%M:%SZ")
        sys.stderr.write(f"trial-{kind} {json.dumps({'at': stamp, **data})}\n")
        sys.stderr.flush()

    def send(text: str, number: int) -> None:
        message = {"type": "user", "message": {"role": "user", "content": text}}
        child.stdin.write((json.dumps(message) + "\n").encode())
        child.stdin.flush()
        log("turn", {"turn": number, "sha256": sha256_bytes(text.encode())})

    def close() -> None:
        if not child.stdin.closed:
            child.stdin.close()

    send(prompt, 0)
    session = None
    sent = 1
    for line in child.stdout:
        sys.stdout.buffer.write(line)
        sys.stdout.buffer.flush()
        try:
            event = json.loads(line)
        except json.JSONDecodeError:
            continue
        if not isinstance(event, dict):
            continue
        if event.get("type") == "system" and event.get("subtype") == "init":
            session = session or event.get("session_id")
        if event.get("type") != "result":
            continue
        if not pending:
            close()
            continue
        turn = pending.pop(0)
        if turn.get("gate"):
            verdict = gate_session(home, session, marks, turn["gate"])
            log("gate", {"mode": turn["gate"], "ok": verdict["ok"], "problems": verdict["problems"]})
            if not verdict["ok"]:
                pending.clear()
                close()
                continue
        send(turn["text"], sent)
        sent += 1
    close()
    return child.wait()


def write_turns(case: dict[str, Any], case_dir: Path, trial: Path) -> dict[str, Any]:
    """The feeder's plan: `/compact` with the case's instructions, then the
    answer, gated by the case's mode."""
    spec = case["compaction"]
    answer = (case_dir / spec.get("answer", "answer.md")).read_text()
    plan = {"turns": [{"text": f"/compact {spec['instructions']}"}, {"text": answer, "gate": spec["mode"]}]}
    (trial / "turns.json").write_text(json.dumps(plan, indent=2) + "\n")
    return plan


def trial_lines(text: str, kind: str) -> list[dict[str, Any]]:
    """The feeder's `trial-KIND` lines of a launch's stderr."""
    found = []
    for line in text.splitlines():
        head, _, rest = line.partition(" ")
        if head == f"trial-{kind}":
            try:
                found.append(json.loads(rest))
            except json.JSONDecodeError:
                continue
    return found


def compaction_report(case: dict[str, Any], record: Path, launch: dict[str, Any], home: Path) -> dict[str, Any]:
    """The compaction of one launch: the turns that the feeder sent, its gate,
    the summary, what Claude Code added after the boundary, the context of the
    first request after it, and the variable in the harness environment."""
    spec = case["compaction"]
    prompt = (record / "prompt").read_text()
    marks = preamble_marks(prompt)
    stderr = (record / "stderr").read_text(errors="replace") if (record / "stderr").exists() else ""
    env = parse_env((record / "env").read_text())
    report: dict[str, Any] = {
        "mode": spec["mode"],
        "mechanism": "claude -p --input-format stream-json in Ravel's managed process; the feeder sends the "
                     "prompt, then /compact with the instructions, then the answer",
        "instructions": spec["instructions"],
        "preamble": {"sha256": marks["sha256"], "paths": len(marks["paths"]), "digests": len(marks["digests"]),
                     "lines": len(marks["lines"])},
        "turns": trial_lines(stderr, "turn"),
        "gate": next(iter(trial_lines(stderr, "gate")), None),
        "variable": {"set": DESCRIPTOR_VARIABLE in env,
                     "descriptor": env.get(DESCRIPTOR_VARIABLE) == launch.get("descriptorPath")},
        "failures": [],
    }
    path = session_file(home, launch["sessionId"]) if launch.get("sessionId") else None
    entries = read_jsonl(path) if path else []
    start = boundary_index(entries)
    if start is None:
        report["failures"].append("the session holds no compaction boundary")
        return report
    summary = next((e for e in entries[start + 1:] if e.get("isCompactSummary")), None)
    summary_text = "\n".join(strings((summary or {}).get("message")))
    report["summary"] = {"chars": len(summary_text), "sha256": sha256_bytes(summary_text.encode()),
                         "findings": context_findings(summary_text, summary_text, marks)}
    report["afterBoundary"] = after_boundary(entries, marks)
    report["failures"] += [f"after the boundary: {problem}"
                           for problem in compaction_problems(report["afterBoundary"] or {}, spec["mode"])] \
        if report["afterBoundary"] else []
    request_at = next((n for n in range(start + 1, len(entries)) if entries[n].get("type") == "api-request"),
                      len(entries))
    report["attachments"] = [(e.get("attachment") or {}).get("type") for e in entries[start + 1:request_at]
                             if e.get("type") == "attachment"]
    # A file that Claude Code restored after the boundary, with its path.
    report["restored"] = [(e.get("attachment") or {}).get("filename") for e in entries[start + 1:request_at]
                          if e.get("type") == "attachment" and (e.get("attachment") or {}).get("filename")]
    report["api"] = api_context(entries, marks)
    gate = report["gate"]
    if not gate or not gate.get("ok"):
        report["failures"].append(f"the gate before the answer did not pass: {gate}")
    api = report["api"]
    if not api["found"]:
        report["failures"].append("the session file holds no model request after the boundary")
    else:
        if api["keep"] != 0 or api["missingBlobs"] or not api["system"]:
            report["failures"].append(f"the request after the boundary is not complete in the session file: "
                                      f"keep {api['keep']}, {api['missingBlobs']} blobs missing, "
                                      f"system prompt {'found' if api['system'] else 'missing'}")
        report["failures"] += [f"model context: {problem}"
                               for problem in compaction_problems(api["findings"], spec["mode"], terms=False)]
    if not (report["variable"]["set"] and report["variable"]["descriptor"]):
        report["failures"].append(f"the harness environment does not hold {DESCRIPTOR_VARIABLE} with the descriptor")
    return report


def launch_role(case: dict[str, Any], index: int, launch: dict[str, Any], view: dict[str, Any]) -> str | None:
    if not case.get("continuation"):
        return None
    if view.get("continuation"):
        return "resumed turn"
    return "first turn" if index == 0 else "fresh launch"


def refusal_failures(case: dict[str, Any], refusal: dict[str, Any], launches: list[dict[str, Any]],
                     stderr: str, sessions: list[Path], tree_before: list[str], tree_after: list[str]) -> list[str]:
    """Each way a case that expects a refusal before any launch did not show
    it. A restored source must resolve, so that the move is the cause."""
    code = case["expectRefusal"]
    failures = []
    if launches or sessions:
        failures.append(f"the driver launched a harness, and the case expects {code} before any launch")
    if f"ravel: refused ({code}" not in stderr:
        failures.append(f"the driver did not report Ravel's refusal {code}; read driver.stderr of the trial")
    moved = refusal["moved"]
    if moved["exit"] == 0 or code not in moved["firstLine"]:
        failures.append(f"the direct resolution with the source moved away gave {moved}, not the refusal {code}")
    if refusal["restored"]["exit"] != 0 or refusal["restored"]["status"] != "resolved":
        failures.append(f"the direct resolution with the source restored gave {refusal['restored']}; "
                        "the refusal may have another cause")
    if tree_before != tree_after:
        failures.append(f"the tree changed from {tree_before} to {tree_after}")
    return failures


def contrast_probes(probes: list[dict[str, str]], cwd: Path) -> list[dict[str, str]]:
    """The same probes, run unconfined by the same wrapper. A confined failure
    belongs to the sandbox only where this run succeeds."""
    argv = [str(WRAPPER), "--probes-only"]
    for item in probes:
        argv += ["--probe", item["op"], item["path"]]
    result = subprocess.run(argv, cwd=cwd, capture_output=True, text=True, check=True)
    return parse_probes(result.stdout)


def started(launch: dict[str, Any]) -> bool:
    """The harness began a session: its stream holds the init event."""
    return launch["sessionId"] is not None


COLLECT_CHARS = 4000


def ending_calls(launches: list[dict[str, Any]]) -> list[dict[str, Any]]:
    """Each ending call of every transcript, with the transcript that made it,
    so a delegate's ending shows apart from the parent's."""
    return [{"file": t["file"], "main": t["main"], "seq": e["seq"], "result": e.get("result"),
             "resultText": e.get("resultText"), "error": e.get("error")}
            for l in launches for t in l["transcripts"] for e in t["events"] if e["class"] == "ending"]


def collect_files(case: dict[str, Any], trial: Path) -> list[dict[str, Any]]:
    """The trial files that a case names in `collect`, such as a log that a
    delegate writes. Ravel can end the group before a tool result reaches the
    session file, and a file outlives that."""
    found = []
    for item in case.get("collect", []):
        path = trial / item
        if path.is_file():
            data = path.read_bytes()
            found.append({"path": item, "sha256": sha256_bytes(data),
                          "text": data.decode(errors="replace")[:COLLECT_CHARS]})
        else:
            found.append({"path": item, "sha256": None, "text": None})
    return found


def delegate_failures(case: dict[str, Any], launches: list[dict[str, Any]]) -> list[str]:
    """A case that expects a delegate fails when no Agent call of any launch
    has a delegate transcript."""
    if not case.get("expectDelegate"):
        return []
    if any(d["transcript"] for launch in launches for d in launch.get("delegations", [])):
        return []
    return ["the case expects a delegate, and no Agent call has a delegate transcript"]


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
    # Two runs of one case in one second take distinct directories.
    for n in range(2, 100):
        if not trial.exists():
            break
        trial = prepared_dir / "runs" / f"{name}-{stamp}-{n}"
    home, repo, records, tmp = trial / "home", trial / "repo", trial / "records", trial / "tmp"
    for directory in (home / ".config" / "ravel", home / ".config" / "jj", records, tmp):
        directory.mkdir(parents=True)
    (home / store).parent.mkdir(parents=True, exist_ok=True)
    os.symlink(owner_home / store, home / store)
    (home / ".config" / "jj" / "config.toml").write_text(
        'user.name = "Grove Trial"\nuser.email = "grove-trial@example.com"\n'
    )
    clone = clone_source(case, trial, checkout, prepared["source"]["commit"])
    clones = {clone["source"]: Path(clone["checkout"])} if clone else {}
    source_clone = None
    if case.get("continuation"):
        # A continuation commits into its sources between the turns, so one
        # clone of its own supplies every source and no other case sees it.
        source_clone = trial / "sources" / "checkout"
        source_clone.parent.mkdir()
        fetch_checkout(checkout, prepared["source"]["commit"], source_clone, git_env(os.environ))
        clones = {source["id"]: source_clone for source in case["sources"]}
    settings = settings_for(case, checkout, clones)
    if settings:
        (home / ".config" / "ravel" / "settings.json").write_text(json.dumps(settings, indent=2) + "\n")
    installed = install_native(case, home, checkout)
    claude_settings = write_claude_settings(case, home)
    planted = plant_files(case, trial, checkout)
    route = route_for(case, records, program, model, effort, fixed_probes(case, home, checkout))
    (trial / "route.json").write_text(json.dumps(route, indent=2) + "\n")
    if case.get("compaction") is not None:
        write_turns(case, case_dir, trial)
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

    standalone = case["mode"] == "standalone"
    compaction = None
    deadline = case.get("timeoutMinutes", 30) * 60
    if standalone:
        # `grove run` confines the harness. Its program is a regular file
        # outside every grant, so the runner grants it read-only, as an owner
        # grants a credential. The wrapper itself is the selected program,
        # which Ravel admits.
        spec = case["standalone"]
        outputs = trial / "out"
        outputs.mkdir()
        prompt = trial / "prompt.md"
        shutil.copyfile(case_dir / spec.get("prompt", "prompt.md"), prompt)
        argv = [str(grove_bin / "grove"), "run", spec.get("kind", case["kinds"][0]), "--prompt-file", str(prompt),
                *[arg for output in spec["outputs"] for arg in ("--output", str(outputs / output))],
                "--runtime-read", str(program), "--ui", "inline"]
        say(f"{name}: {version}, model {model}, effort {effort}, route {case['route']}, confined, trial {trial}")
        driver = run_driver(argv, tmp, env, trial, deadline)
    else:
        base = plant_repo(repo, case_dir / "tree", env)
        tree_before = tree_state(repo)
        refusal = None
        if clone:
            move_away(clone)
            say(f"{name}: moved the checkout of {clone['source']} to {clone['movedTo']}")
            refusal = {"expected": case["expectRefusal"], "clone": clone,
                       "moved": resolve_direct(front, env, tmp)}
        say(f"{name}: {version}, model {model}, effort {effort}, route {case['route']}, trial {trial}")
        driver = run_driver([str(grove_bin / "grove")], repo, env, trial, deadline)
        if clone and refusal:
            restore(clone)
            refusal["restored"] = resolve_direct(front, env, tmp)
        continuation = None
        if source_clone:
            continuation = continue_case(case, case_dir, trial, home, repo, records, env, route, program,
                                         settings, source_clone, [str(grove_bin / "grove")], deadline)

    links_after = list_links(home)
    for link, target in links_after:
        say(f"link after {link} -> {target}")
    link_failure = None
    try:
        check_links(links_after, store, owner_home, "after")
    except TrialError as error:
        link_failure = str(error)

    stderr = (trial / "driver.stderr").read_text(errors="replace")
    native, residue = skill_files(home)
    contrast: list[dict[str, str]] = []
    if standalone:
        logs = sorted((home / ".local" / "state" / "grove" / "runs").glob("*.log"))
        launches = []
        run_log: list[str] = []
        for log in logs:
            for n, part in enumerate(split_log(log.read_text(errors="replace"))):
                stream_text = "".join(line + "\n" for line in part["streamLines"]).encode()
                launch = describe(f"{log.relative_to(home)}#{n}", part["view"], part["env"], part["stream"],
                                  sha256_bytes(stream_text), part["probes"], home, trial / "unknown", environ)
                launches.append(launch)
            run_log += log_lines(log.read_text(errors="replace"))
        probes = [probe for launch in launches for probe in launch["probes"]]
        if probes:
            (trial / "contrast").mkdir()
            contrast = contrast_probes(probes, trial / "contrast")
        tree_before = tree_after = []
        refusal = None
        continuation = None
        published = "\n".join(
            f"{path.name} {sha256_file(path)}" for path in sorted(outputs.iterdir()) if path.is_file()
        )
        changed = ""
    else:
        run_log = []
        marker = (case.get("continuation") or {}).get("marker")
        turns = case.get("compaction") is not None
        launches = []
        for index, path in enumerate(launch_records(records)):
            launch = describe_launch(path, home, repo, environ, marker, turns)
            role = launch_role(case, index, launch, json.loads((path / "view.json").read_text()))
            if role:
                launch["role"] = role
                launch["snapshotMarkers"] = snapshot_markers(case, launch)
            launches.append(launch)
        tree_after = tree_state(repo)
        if turns and launches:
            compaction = compaction_report(case, launch_records(records)[0], launches[0], home)
        published = run_checked(
            ["jj", "log", "--no-graph", "-r", f"{base}..@ ~ empty()", "-T", 'commit_id.short(12) ++ " " ++ if(description, description.first_line(), "(no description)") ++ "\\n"'],
            cwd=repo, env=env,
        )
        changed = run_checked(["jj", "diff", "--summary", "--from", base, "--to", "@"], cwd=repo, env=env)
    resolve = resolve_offline(front, env, trial) if case.get("resolveOffline") else None
    record = {
        "schemaVersion": 1,
        "case": name,
        "purpose": case.get("purpose"),
        "credits": case.get("credits", []),
        "route": case["route"],
        "mode": case["mode"],
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
        "skills": {"nativeCopies": native, "residue": residue, "installed": installed,
                   "claudeSettings": claude_settings, "planted": planted},
        "launches": launches,
        "contrast": contrast,
        "resolve": resolve,
        "refusal": refusal,
        **({"continuation": continuation} if continuation is not None else {}),
        **({"compaction": compaction} if compaction is not None else {}),
        **({"collected": collect_files(case, trial)} if case.get("collect") else {}),
        **({"systemNote": {"text": case["systemNote"], "sha256": sha256_bytes(case["systemNote"].encode())}}
           if case.get("systemNote") else {}),
        "ending": {
            "driverExit": driver["exit"],
            "timedOut": driver["timedOut"],
            "elapsedS": driver["elapsedS"],
            "driverLines": [line for line in stderr.splitlines()
                            if line.startswith(("grove", "ravel:", "Error", "  remedy:"))],
            "runLog": run_log[-60:],
            "endingCommands": [e.get("result") for l in launches for t in l["transcripts"]
                               for e in t["events"] if e["class"] == "ending"],
            "endingCalls": ending_calls(launches),
            "teardownRecorded": "grove finished — loop complete" in stderr,
            "treeBefore": tree_before,
            "treeAfter": tree_after,
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
    failures += [f"launch {n}: the harness wrote no startup event; read {launch['record']} and its stderr"
                 for n, launch in enumerate(launches) if not started(launch)]
    if refusal:
        failures += refusal_failures(case, refusal, launches, stderr, session_files(home), tree_before, tree_after)
    elif not launches:
        failures.append("the driver launched no harness; read driver.stderr of the trial")
    if link_failure:
        failures.append(link_failure)
    if continuation:
        failures += continuation["failures"]
    if compaction:
        failures += compaction["failures"]
    elif case.get("compaction") is not None:
        failures.append("the case compacted no launch; read driver.stderr of the trial")
    failures += native_failures(native, expected_native(case))
    failures += delegate_failures(case, launches)
    if resolve:
        failures += resolve_failures(resolve, launches)
    say(f"record {trial}/record.md")
    if failures:
        raise TrialError("; ".join(failures))
    return record


def reextract(trial: Path) -> dict[str, Any]:
    """Extract the events of an earlier run again, from its unchanged
    transcripts, with the current classifier. Every other fact of the record
    stays as the run wrote it. The record names the time of the extraction."""
    trial = trial.resolve()
    record = json.loads((trial / "record.json").read_text())
    home, repo = trial / "home", trial / "repo"
    fresh: list[dict[str, Any]] = []
    if record.get("mode", "lifecycle") == "standalone":
        for log in sorted((home / ".local" / "state" / "grove" / "runs").glob("*.log")):
            for n, part in enumerate(split_log(log.read_text(errors="replace"))):
                stream_text = "".join(line + "\n" for line in part["streamLines"]).encode()
                fresh.append(describe(f"{log.relative_to(home)}#{n}", part["view"], part["env"], part["stream"],
                                      sha256_bytes(stream_text), part["probes"], home, trial / "unknown", {}))
    else:
        marker = (record.get("continuation") or {}).get("marker")
        turns = record.get("compaction") is not None
        fresh = [describe_launch(path, home, repo, {}, marker, turns) for path in launch_records(trial / "records")]
    if len(fresh) != len(record["launches"]):
        raise TrialError(f"{trial} now shows {len(fresh)} launches, and its record holds {len(record['launches'])}")
    for launch, again in zip(record["launches"], fresh):
        for old, new in zip(launch["transcripts"], again["transcripts"]):
            if old["sha256"] != new["sha256"]:
                raise TrialError(f"the transcript {old['file']} of {trial} changed since the run")
        launch["transcripts"] = again["transcripts"]
        launch["delegations"] = again["delegations"]
    record["ending"]["endingCommands"] = [e.get("result") for l in record["launches"] for t in l["transcripts"]
                                          for e in t["events"] if e["class"] == "ending"]
    record["ending"]["endingCalls"] = ending_calls(record["launches"])
    record["reextracted"] = dt.datetime.now(dt.UTC).strftime("%Y-%m-%dT%H:%M:%SZ")
    (trial / "record.json").write_text(json.dumps(record, indent=2) + "\n")
    (trial / "record.md").write_text(render(record))
    say(f"record {trial}/record.md")
    return record


# --- The record ----------------------------------------------------------------


def cell(text: Any) -> str:
    return str(text).replace("|", "\\|").replace("\n", " ")


def shown(event: dict[str, Any]) -> str:
    target = event["target"]
    if event["tool"] in ("Bash", "text", "prompt") and len(target) > COMMAND_CHARS:
        target = target[:COMMAND_CHARS] + "…"
    if event["tool"] in ("text", "prompt", "compaction"):
        return cell(target)
    if event["tool"] == "Skill" and event.get("resultText"):
        result = event["resultText"]
        result = result[:COMMAND_CHARS] + "…" if len(result) > COMMAND_CHARS else result
        return f"`{cell(target)}`: {cell(result)}"
    return f"`{cell(target)}`" if target else ""


def render_findings(label: str, findings: dict[str, Any] | None) -> list[str]:
    if findings is None:
        return [f"- {label}: not read."]
    lines = [
        f"- {label}: {len(findings['paths'])} preamble paths, {len(findings['digests'])} digests, "
        f"{len(findings['lines'])} preamble lines, terms {cell(', '.join(findings['terms']) or 'none')}, "
        f"`{DESCRIPTOR_VARIABLE}` {'named' if findings['variable'] else 'not named'}."
    ]
    lines += [f"  - Term in context: `…{cell(snippet)}…`" for snippet in findings.get("snippets", [])]
    return lines


def render_compaction(compaction: dict[str, Any]) -> list[str]:
    preamble = compaction["preamble"]
    gate = compaction.get("gate") or {}
    api = compaction.get("api") or {}
    summary = compaction.get("summary") or {}
    lines = [
        "Compaction:",
        "",
        f"- Mode: {compaction['mode']} loss. Mechanism: {compaction['mechanism']}.",
        f"- Summary instructions, which the agent sees after the boundary: `{cell(compaction['instructions'])}`.",
        f"- Preamble: SHA-256 `{preamble['sha256']}`, {preamble['paths']} paths, {preamble['digests']} digests, "
        f"{preamble['lines']} lines of at least {PREAMBLE_LINE_CHARS} characters.",
        *[f"- Turn {t['turn']} sent at {t['at']}, SHA-256 `{t['sha256']}`." for t in compaction.get("turns", [])],
        (f"- Gate before the answer: {'passed' if gate.get('ok') else 'failed'}"
         + (f": {cell('; '.join(gate.get('problems', [])))}" if gate.get("problems") else "") + "."
         if gate else "- Gate before the answer: no gate ran."),
        *([f"- Summary: {summary['chars']} characters, SHA-256 `{summary['sha256']}`."] if summary else []),
        *(render_findings("Summary", summary.get("findings")) if summary else []),
        *render_findings("Session lines after the boundary", compaction.get("afterBoundary")),
        *([f"- Attachments after the boundary, before the first request: "
           f"{cell(', '.join(str(a) for a in compaction.get('attachments', [])) or 'none')}."]
          if "attachments" in compaction else []),
        *[f"- File that Claude Code restored after the boundary: `{cell(path)}`."
          for path in compaction.get("restored", [])],
    ]
    if api.get("found"):
        lines += [
            f"- First model request after the boundary: `keep` {api['keep']}, {api['blobs']} blobs, "
            f"{api['missingBlobs']} missing, system prompt {'found' if api['system'] else 'missing'}, "
            f"request digest `{api['requestDigest']}`.",
            *render_findings("Model context of that request", api["findings"]),
        ]
    elif api:
        lines.append("- First model request after the boundary: none in the session file.")
    variable = compaction["variable"]
    lines += [
        f"- Harness environment: `{DESCRIPTOR_VARIABLE}` {'set' if variable['set'] else 'unset'}"
        f"{', equal to the descriptor path' if variable['descriptor'] else ''}.",
        *[f"- Failure: {cell(failure)}." for failure in compaction.get("failures", [])],
        "",
    ]
    return lines


def render_delegations(found: list[dict[str, Any]]) -> list[str]:
    """What the parent gave each delegate, and how the delegate read."""
    lines = []
    for number, item in enumerate(found):
        order = item.get("order") or {}
        first = order.get("firstRead") or {}
        lines += [
            f"Delegation {number}: `{cell(item['subagentType'])}`, \"{cell(item['description'])}\"",
            "",
            f"- Mechanism: {item['mechanism']}. Status `{item['status']}`, meta `{cell(json.dumps(item['meta']))}`.",
            f"- Inherited ending context: {cell(', '.join(item['inherited']) or 'none')}.",
            f"- Prompt SHA-256 `{item['promptSha256']}`. Descriptor path: {'given' if item['givesDescriptorPath'] else 'not given'}. "
            f"Variable named: {'yes' if item['namesVariable'] else 'no'}. Obligations word for word: "
            f"{len(item['obligations'])} of {item['obligationCount']}"
            + (f" ({', '.join(item['obligations'])})" if item['obligations'] else "") + ".",
            f"- Lines with an ending term: {cell('; '.join(item['endingLines']) or 'none')}.",
            f"- Transcript: {cell(item['transcript'] or 'none')}. Mandatory files by first read: "
            f"{len(order.get('mandatoryOrder', []))}; first guidance read: {first.get('class')} at call {first.get('seq')}; "
            f"first action at call {order.get('firstAction')}; all mandatory files in order before it: "
            f"{'yes' if order.get('mandatoryBeforeAction') else 'no'}.",
            "",
            "Prompt that the parent gave:",
            "",
            "```text",
            item["prompt"].rstrip("\n"),
            "```",
            "",
            "Result that the parent received:",
            "",
            "```text",
            item["result"].rstrip("\n"),
            "```",
            "",
        ]
    return lines


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
        f"| Mode | {record.get('mode', 'lifecycle')} |",
        f"| Credits | {', '.join(record['credits']) or 'none'} |",
        *([f"| Events extracted again | {record['reextracted']}, from the same transcripts |"]
          if record.get("reextracted") else []),
        f"| Grove build | commit `{g['commit'][:12]}`, `grove` SHA-256 `{g['sha256']['grove']}`, `grove-llm` SHA-256 `{g['sha256']['grove-llm']}` |",
        f"| Ravel build | commit `{r['buildCommit']}` (line `{r['line']}` at `{r['outputCommit']}`), identity `{r['buildIdentity']}`, `bin/ravel` SHA-256 `{r['binRavelSha256']}` |",
        f"| Harness | {cell(h['version'])}, model `{h['model']}`, effort `{h['effort']}`, permission mode `{h['permissionMode']}` |",
        f"| Configuration directory | sandbox `.claude`; `claude auth status` reports the same directory |",
        f"| Source checkout | commit `{record['sources']['commit'][:12]}` |",
        f"| Links before and after | {cell(record['links']['before'] and record['links']['before'][0][0])} only; after: {cell(', '.join(link for link, _ in record['links']['after']) or 'none')} |",
        f"| Driver environment | {cell(', '.join(record['environment']))} |",
        f"| Native copies | {cell(', '.join(record['skills']['nativeCopies']) or 'none')} |",
        *[f"| Installed native copy | `{i['path']}` from `{i['from']}`"
          + (f" at commit `{i['commit'][:12]}`" if i.get("commit") else "")
          + f", `SKILL.md` SHA-256 `{i['skillSha256']}`"
          + (f"; the checkout's current `SKILL.md` has SHA-256 `{i['currentSha256']}`" if i.get("commit") else "")
          + " |"
          for i in record['skills'].get('installed', [])],
        *([f"| Sandbox `.claude/settings.json` | `{cell(json.dumps(record['skills']['claudeSettings']))}` |"]
          if record['skills'].get('claudeSettings') is not None else []),
        *[f"| Planted file | `$TRIAL/{p['path']}` from `{p['from']}`, SHA-256 `{p['sha256']}` |"
          for p in record['skills'].get('planted', [])],
        f"| Synced claude.ai skills (residue) | {cell(', '.join(record['skills']['residue']) or 'none')} |",
        "",
    ]
    marked = bool(record.get("continuation"))
    for number, launch in enumerate(record["launches"]):
        role = f", {launch['role']}" if launch.get("role") else ""
        lines += [f"#### Launch {number}: kind `{launch['kind']}`{role}", ""]
        identity = launch["descriptorIdentity"] or {}
        lines += [
            f"- Descriptor identity: `{identity.get('directory')}` (file SHA-256 `{identity.get('sha256')}`).",
            *[f"- Source `{s['id']}`: `resourceDigest` `{s['resourceDigest']}`." for s in launch["sources"]],
            f"- Authority control: {'no live control name reached the child' if not launch['control'] else '; '.join(launch['control'])}.",
            f"- Child environment names: {cell(', '.join(launch['envNames']))}.",
        ]
        for transcript in launch["transcripts"]:
            window = (f", lines {transcript['lines'][0] + 1} to {transcript['lines'][1]} of the session file"
                      if transcript.get("lines") else "")
            lines += [f"- Transcript `{transcript['file']}`{window}: SHA-256 `{transcript['sha256']}`, {sum(e['tool'] != 'text' for e in transcript['events'])} tool calls, "
                      f"{sum(e['tool'] == 'text' for e in transcript['events'])} text blocks."]
        for path, holds in (launch.get("snapshotMarkers") or {}).items():
            state = "no copy" if holds is None else ("holds the marker" if holds else "does not hold the marker")
            lines.append(f"- Snapshot copy of `{path}`: {state}.")
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
                "| # | Tool | Class | Skill | Source | Target | Error |" + (" Marker |" if marked else ""),
                "| --- | --- | --- | --- | --- | --- | --- |" + (" --- |" if marked else ""),
            ]
            for event in transcript["events"]:
                lines.append(
                    f"| {event['seq']} | {event['tool']} | {event['class']} | {event.get('skill') or ''} "
                    f"| {event.get('source') or ''} | {shown(event)} | {'yes' if event['error'] else ''} |"
                    + (f" {', '.join(event.get('marker') or [])} |" if marked else "")
                )
            lines.append("")
        lines += render_delegations(launch.get("delegations") or [])
    contrast = {(item["op"], item["path"]): item["status"] for item in record.get("contrast") or []}
    probes = [probe for launch in record["launches"] for probe in launch.get("probes", [])]
    if probes:
        lines += [
            "Probes inside the launch, and the same probes unconfined:",
            "",
            "| Access | Path | Inside | Unconfined |",
            "| --- | --- | --- | --- |",
        ]
        for probe in probes:
            lines.append(f"| {probe['op']} | `{cell(probe['path'])}` | {probe['status']} "
                         f"| {contrast.get((probe['op'], probe['path']), '')} |")
        lines.append("")
    if record["ending"].get("runLog"):
        lines += ["Grove's run log, without the stream and the wrapper's lines:", "", "```text",
                  *record["ending"]["runLog"], "```", ""]
    resolve = record.get("resolve")
    if resolve:
        identity = resolve["descriptorIdentity"] or {}
        lines += [
            "Offline resolution:",
            "",
            f"- Command: `ravel skills resolve --json` under `sandbox-exec -p '{resolve['profile']}'`, PATH `{resolve['path']}`.",
            f"- Network control: unguarded `nc` exit {resolve['networkControl']['unguardedExit']}, "
            f"guarded exit {resolve['networkControl']['guardedExit']}.",
            f"- Git control: usual PATH `{resolve['gitControl']['usualPath']}`, resolution PATH "
            f"`{resolve['gitControl']['resolutionPath']}`.",
            f"- Exit {resolve['exit']}, status `{resolve['status']}`.",
            f"- Descriptor identity: `{identity.get('directory')}` (file SHA-256 `{identity.get('sha256')}`).",
            *[f"- Source `{s['id']}`: `resourceDigest` `{s['resourceDigest']}`." for s in resolve["sources"]],
            "",
        ]
    refusal = record.get("refusal")
    if refusal:
        clone = refusal["clone"]
        lines += [
            "Expected refusal:",
            "",
            f"- Expected code: `{refusal['expected']}`, before any harness launch.",
            (f"- Source `{clone['source']}`: a plain Git checkout at commit `{clone['commit'][:12]}`, "
             f"moved from `$TRIAL/{Path(clone['checkout']).relative_to(record['trial'])}` "
             f"to `$TRIAL/{Path(clone['movedTo']).relative_to(record['trial'])}` before the driver, "
             "and restored after it."),
            (f"- Direct `ravel skills resolve --json`, source moved away: exit {refusal['moved']['exit']}, "
             f"`{cell(refusal['moved']['firstLine'])}`."),
            *([f"  - `{cell(refusal['moved']['remedy'])}`"] if refusal['moved'].get('remedy') else []),
            (f"- The same resolution, source restored: exit {refusal['restored']['exit']}, "
             f"status `{refusal['restored']['status']}`."),
            "",
        ]
    continuation = record.get("continuation")
    if continuation:
        edit = continuation.get("edit") or {}
        resume = continuation.get("resume") or {}
        fresh = continuation.get("freshDriver") or {}
        settings = continuation.get("settings") or {}
        lines += [
            "Continuation:",
            "",
            f"- Mechanism: {continuation['mechanism']}. Marker: `{continuation['marker']}`.",
            *([f"- Session: `{continuation['sessionId']}`."] if continuation.get("sessionId") else []),
            *([f"- Source commit: `{edit['commitBefore'][:12]}` to `{edit['commitAfter'][:12]}` in "
               f"`$TRIAL/{Path(edit['checkout']).relative_to(record['trial'])}`."] if edit else []),
            *[f"  - `{f['path']}`: SHA-256 `{f['sha256Before']}` to `{f['sha256After']}`." for f in edit.get("files", [])],
            *([f"- Mandatory files before: {cell(', '.join(m['source'] + ':' + m['path'] for m in settings['mandatoryBefore']))}; "
               f"after: {cell(', '.join(m['source'] + ':' + m['path'] for m in settings['mandatoryAfter']))}."]
              if settings else []),
            *([f"- Resumed turn: arguments `{cell(' '.join(resume['args']))}` and the answer "
               f"(SHA-256 `{resume['answerSha256']}`), working directory `{cell(resume['cwd'])}`; "
               f"exit {resume['exit']}{', timed out' if resume['timedOut'] else ''}, {resume['elapsedS']} s."]
              if resume else []),
            *([f"- Repository files with the marker after the resumed turn: "
               f"{cell(', '.join(continuation['markerFilesAfterResume']) or 'none')}."]
              if "markerFilesAfterResume" in continuation else []),
            *([f"- Fresh-launch leaf: {cell(', '.join(continuation['freshTree']['added']))}, "
               f"commit `{continuation['freshTree']['commit'][:12]}`."] if continuation.get("freshTree") else []),
            *([f"- Fresh driver: exit {fresh['exit']}{', timed out' if fresh['timedOut'] else ''}, {fresh['elapsedS']} s."]
              if fresh else []),
            *[f"  - `{cell(line)}`" for line in fresh.get("lines", [])],
            *([f"- Repository files with the marker after the fresh launch: "
               f"{cell(', '.join(continuation['markerFilesAfterFresh']) or 'none')}."]
              if "markerFilesAfterFresh" in continuation else []),
            *[f"- Failure: {cell(failure)}." for failure in continuation.get("failures", [])],
            "",
        ]
    compaction = record.get("compaction")
    if compaction:
        lines += render_compaction(compaction)
    note = record.get("systemNote")
    if note:
        lines += [f"Owner note through `--append-system-prompt`, SHA-256 `{note['sha256']}`:", "",
                  "```text", note["text"].rstrip("\n"), "```", ""]
    for item in record.get("collected") or []:
        if item["sha256"] is None:
            lines += [f"Collected file `$TRIAL/{item['path']}`: absent.", ""]
        else:
            lines += [f"Collected file `$TRIAL/{item['path']}`, SHA-256 `{item['sha256']}`:", "",
                      "```text", item["text"].rstrip("\n"), "```", ""]
    lines += [
        "Ending outcome:",
        "",
        f"- Ending commands that the agent ran: {cell(', '.join(str(x) for x in e['endingCommands']) or 'none')}.",
        *[f"  - `{c['result']}` at call {c['seq']} of `{Path(c['file']).name}`"
          f"{'' if c['main'] else ', a delegate'}{', error' if c['error'] else ''}: "
          f"`{cell((c['resultText'] or '').strip()[:160]) or 'no result text'}`."
          for c in e.get("endingCalls", [])],
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
    x = commands.add_parser("reextract", help="extract the events of an earlier run again, from its transcripts")
    x.add_argument("--trial", required=True, type=Path)
    t = commands.add_parser("turns", help="the feeder of a compaction case, which the trial configuration runs")
    t.add_argument("--plan", required=True, type=Path)
    t.add_argument("harness", nargs=argparse.REMAINDER, help="-- PROGRAM ARG... PROMPT")
    args = parser.parse_args(argv)
    try:
        if args.command == "prepare":
            prepare(args.out, args.commit, args.ravel, environ)
        elif args.command == "reextract":
            reextract(args.trial)
        elif args.command == "turns":
            harness = args.harness[1:] if args.harness[:1] == ["--"] else args.harness
            return run_turns(args.plan, harness)
        else:
            run_case(args.prepared, args.case, environ, args.model, args.effort)
    except TrialError as error:
        print(f"harness-trial: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
