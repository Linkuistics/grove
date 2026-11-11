#!/usr/bin/env python3
"""Run one real harness trial of the common loading route, and record it.

The acceptance node `common-loading-acceptance-k29` measures actual Claude Code
and Codex sessions on the routes of `docs/specs/codex-skill-provisioning.md`.
This runner owns the mechanics that every trial shares. A case only says what
differs. The trial conventions live in that node's brief and in
`docs/common-loading-acceptance-record.md`.

    harness_trial.py prepare --out DIR --commit COMMIT --ravel FRONT [--keg CANDIDATE]
    harness_trial.py run --prepared DIR --case NAME [--harness H] [--model M] [--effort E]
                         [--working-copy ID=DIR]...
    harness_trial.py check --commit COMMIT [--case NAME]... [--repository REPO] [--keg CANDIDATE]
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
instead, and Grove's run log keeps the record of its launch. Codex runs in
its own sandbox `workspace-write` on every route. A case can add an offline
`ravel skills resolve`. For an
explicit-route request, a case can also copy native skills into the sandbox
`.claude/skills`, write the
sandbox `.claude/settings.json` and plant files under the trial's `guidance/`.
A native copy can come from an older commit of the checkout, as a stale copy.
A Claude Code case with `nativePlugins` adds the marketplace of the prepared
checkout to the sandbox home and installs each named plugin from it, before
the driver. The record names each installed plugin's version, Git commit,
dependency flag and skill files. Grove's marketplace lists no plugin after
`native-marketplace-retirement-k235`, so such a case needs a prepared commit
from before that change. At a later commit, `check` and `run` refuse it.
A source with `external` comes from another repository: the runner makes a
plain Git checkout of its commit under the trial's `sources/`, before any
launch, and the record names the commit, the checkout, the Git tree ID of the
source directory and Ravel's `resourceDigest`. `--working-copy ID=DIR` takes
that source from a jj working copy instead, and the record names its change.
A source with `"keg": "grove"` comes from the candidate keg that `task
owner:candidate-keg` built from the prepared commit, and `prepare --keg`
names its directory. Its `plugin` is its private source in the keg,
`libexec/skill-sources/<id>`, which Ravel names as the `originPath`. The
record names the keg, each path that Ravel captured and Ravel's
`resourceDigest`. A capture from another path, or a
digest other than the keg's accepted digest, fails the run.
A case with `moveAway` and `expectRefusal` moves a clone of one source away
before the driver, and expects Ravel's refusal before any harness launch. An
external source moves its own checkout. With `moveRoot`, only that root of the
source moves, and the checkout stays.
A case with `continuation` resumes the first session outside Ravel after a
source commit and a settings change, then launches a fresh session. Claude
Code resumes with `--resume`, and Codex with `codex exec resume`. A case
with `compaction` runs three turns in one managed process: the prompt, a
compaction, then the case's answer. The feeder `harness_trial.py turns` sends
them, and it sends the answer only when the context after compaction holds no
preamble path. Claude Code compacts with `/compact` and the case's
instructions. Codex runs as `codex app-server`, and the feeder sends
`thread/compact/start`.
Each Agent call of a launch adds a delegation to the record: the prompt that
the parent gave, its comparison with the preamble, the delegate's transcript
and the ending context that the delegate inherits. A case with
`expectDelegate` fails without a delegate transcript. A case can `collect`
files of its trial directory into the record. A case's `systemNote` reaches
Claude Code through `--append-system-prompt` and Codex through `-c
developer_instructions`, the owner's channel of each harness. The prepared
prompt stays unchanged. A Codex delegate is a `spawn_agent` thread of the same
process, with its own session file, which names the parent thread.
Before any launch, `run` checks that the supplier of each required path of
the case holds it: the prepared commit, an older commit of a native copy, an
external checkout or working copy, or a plugin checkout that the marketplace
pins. A native plugin's kind skill can also come from a plugin that its
manifest declares as a dependency. A missing path refuses the run, because
the case measures a build whose supplier held it. A case declares in
`requires` the skills that its task relies on without the case naming them,
each in a source or a native plugin, and in `absent` the skills whose absence
it measures. `check` runs the same check against a commit, without a
build and without a launch.
A case names its harness. `--harness` runs the same case in another harness,
which then takes its own default model and effort. The Codex adapter runs
`codex exec --json` with `CODEX_HOME` set to the sandbox `.codex`, which links
only `auth.json`. It reads its events from the Codex session file.
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
import tempfile
import threading
import time
from pathlib import Path, PurePosixPath
from typing import Any, Callable, Mapping

ROOT = Path(__file__).resolve().parent.parent
TRIALS = ROOT / "scripts" / "harness-trials"
CASES = TRIALS / "cases"
BUILDS = TRIALS / "builds.json"
WRAPPER = TRIALS / "trial-launch.sh"

# The names that the driver receives from the runner's environment. Every
# other name is dropped: API keys, the live session's channel and launch
# directory, CLAUDECODE and every CLAUDE_* name.
PASSED_NAMES = ("USER", "LOGNAME", "LANG", "LC_ALL", "LC_CTYPE")
SYSTEM_PATH = ("/usr/bin", "/bin", "/usr/sbin", "/sbin")

# The live session's authority. A name of this class may reach a trial child
# only with the trial's own value. A trial sets its own RAVEL_EXIT_FILE and
# GROVE_LAUNCH_DIR, so the name alone is no leak. The live session can run in
# either harness, whichever harness the trial runs, so both patterns name the
# names of both.
CONTROL_PATTERN = re.compile(r"^(RAVEL_|GROVE_|HARNESS_DISPATCH_|CLAUDE|CODEX_)")
# Names that no trial child may hold at all, with any value.
HARNESS_PATTERN = re.compile(r"^(CLAUDECODE$|CLAUDE_|ANTHROPIC_|OPENAI_|CODEX_API_KEY$)")

ROUTES = ("native-2.0", "directory-2.0")
# A lifecycle case runs the bare Grove driver in a jj repository. A standalone
# case runs `grove run`.
MODES = ("lifecycle", "standalone")
# The network-denying profile of crates/grove/tests/directory_route.rs.
NO_NETWORK = "(version 1)(allow default)(deny network*)"
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
# The commit of an external source: a full Git hash, so that the checkout
# names exactly one commit.
FULL_COMMIT = re.compile(r"^[0-9a-f]{40}$")


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


def physical(path: str, cwd: str) -> str:
    """The physical form of a path: links resolved, a missing tail kept."""
    return os.path.realpath(os.path.join(cwd, path))


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


def load_case(name: str, harness: str | None = None) -> tuple[dict[str, Any], Path]:
    """The case, with `harness` set to the harness of this run. A case names
    its own harness, and `harness` runs it in another one. Its `model` and
    `effort` then do not apply, because they name models of its own harness."""
    directory = CASES / name
    file = directory / "case.json"
    if not file.is_file():
        known = sorted(path.name for path in CASES.iterdir() if (path / "case.json").is_file())
        raise TrialError(f"no case '{name}'; the cases are {', '.join(known)}")
    case = json.loads(file.read_text())
    if case.get("route") not in ROUTES:
        raise TrialError(f"case {name}: the route '{case.get('route')}' is not one of {', '.join(ROUTES)}")
    adapter, own = run_adapter(name, case.get("harness"), harness)
    if own:
        case["caseHarness"] = own
        case["harness"] = adapter.name
        case.pop("model", None)
        case.pop("effort", None)
    adapter.check_case(name, case)
    mode = case.setdefault("mode", "lifecycle")
    if mode not in MODES:
        raise TrialError(f"case {name}: the mode '{mode}' is not one of {', '.join(MODES)}")
    if mode == "lifecycle" and not (directory / "tree").is_dir():
        raise TrialError(f"case {name}: no tree/ directory")
    if bool(case.get("moveAway")) != bool(case.get("expectRefusal")) or (case.get("moveAway") and mode != "lifecycle"):
        raise TrialError(f"case {name}: a lifecycle case names moveAway and expectRefusal together")
    if any(s.get("id") == case.get("moveAway") and "keg" in s for s in case.get("sources", [])):
        raise TrialError(f"case {name}: moveAway names a source of the candidate keg, which no case moves")
    if case.get("moveRoot") is not None:
        moved = next((s for s in case.get("sources", []) if s.get("id") == case.get("moveAway")), None)
        if moved is None or case["moveRoot"] not in moved.get("roots", []):
            raise TrialError(f"case {name}: moveRoot names one of the roots of the source that moveAway names")
    for item in case.get("collect", []):
        if not isinstance(item, str) or Path(item).is_absolute() or ".." in Path(item).parts:
            raise TrialError(f"case {name}: a collected file is a path inside the trial directory, not {item!r}")
    for source in case.get("sources", []):
        check_external(name, source)
    check_declarations(case)
    plugins = case.get("nativePlugins")
    if plugins is not None and (not isinstance(plugins, list) or not plugins
                                or not all(isinstance(p, str) and PLUGIN_ID.match(p) for p in plugins)):
        raise TrialError(f"case {name}: nativePlugins is a list of <plugin>@<marketplace> names, not {plugins!r}")
    if case.get("continuation") is not None:
        check_continuation(case, directory)
    if case.get("compaction") is not None:
        check_compaction(case, directory)
    if mode == "standalone":
        standalone = case.get("standalone") or {}
        if not (directory / standalone.get("prompt", "prompt.md")).is_file() or not standalone.get("outputs"):
            raise TrialError(f"case {name}: a standalone case names a prompt file and its outputs")
    return case, directory


PLUGIN_ID = re.compile(r"^[A-Za-z0-9._-]+@[A-Za-z0-9._-]+$")
# A source with "keg": "grove" comes from the candidate keg that `prepare
# --keg` names, which `task owner:candidate-keg` builds from the prepared
# commit. Its directory in the keg is the private source
# libexec/skill-sources/<id>, as "The keg layout" of
# docs/specs/codex-skill-provisioning.md states. Ravel canonicalises a
# source, so it names that path, not the public alias in share/ravel/skills.
KEG_FORMULA = "grove"
KEG_SOURCES = "libexec/skill-sources"


def check_external(name: str, source: dict[str, Any]) -> None:
    """A source with `external` names a repository and a full commit hash.
    Its `plugin` is then the source directory inside that repository's
    checkout, as the external pack contract's "Proof shape" states."""
    external = source.get("external")
    plugin = Path(source.get("plugin", ""))
    if plugin.is_absolute() or ".." in plugin.parts or not plugin.parts:
        raise TrialError(f"case {name}: the plugin of the source '{source.get('id')}' is a relative path, "
                         f"not {source.get('plugin')!r}")
    if "keg" in source:
        if external is not None or source["keg"] != KEG_FORMULA:
            raise TrialError(f"case {name}: the source '{source.get('id')}' names \"keg\": \"{KEG_FORMULA}\", "
                             "the candidate keg of the prepared commit, and no external object")
        if source.get("plugin") != f"{KEG_SOURCES}/{source.get('id')}":
            raise TrialError(f"case {name}: the keg source '{source.get('id')}' has the plugin "
                             f"{KEG_SOURCES}/{source.get('id')}, its directory in the keg")
        return
    if external is None:
        return
    if not isinstance(external, dict) or set(external) != {"repository", "commit"}:
        raise TrialError(f"case {name}: the external object of the source '{source.get('id')}' names "
                         "exactly a repository and a commit")
    if not isinstance(external["repository"], str) or not external["repository"]:
        raise TrialError(f"case {name}: the external source '{source.get('id')}' names its repository")
    if not isinstance(external["commit"], str) or not FULL_COMMIT.match(external["commit"]):
        raise TrialError(f"case {name}: the external source '{source.get('id')}' names a commit of 40 "
                         f"lowercase hexadecimal digits, not {external['commit']!r}")


def check_declarations(case: dict[str, Any]) -> None:
    """`requires` names each skill that the task relies on without the case
    naming it, as a source of the case or one of its native plugins, and a
    path inside it. `absent` names each skill that the case measures the
    absence of. The runner reads both only in the supplier check."""
    name = case["name"]
    sources = {source.get("id") for source in case.get("sources", [])}
    listed = case.get("nativePlugins")
    plugins = {p for p in listed if isinstance(p, str)} if isinstance(listed, list) else set()
    requires = case.get("requires", [])
    if not isinstance(requires, list):
        raise TrialError(f"case {name}: requires is a list of {{source, path}} or {{plugin, path}} objects, "
                         f"not {requires!r}")
    for item in requires:
        path = PurePosixPath(item.get("path", "")) if isinstance(item, dict) else None
        holder = ((set(item) == {"source", "path"} and item["source"] in sources)
                  or (set(item) == {"plugin", "path"} and item["plugin"] in plugins)) if path else False
        if not holder or path.is_absolute() or ".." in path.parts or not path.parts:
            raise TrialError(f"case {name}: each entry of requires names one of the sources {sorted(sources)} "
                             f"or one of the native plugins {sorted(plugins)}, and a relative path inside it, "
                             f"not {item!r}")
    absent = case.get("absent", [])
    if not isinstance(absent, list) or not all(isinstance(skill, str) and skill and "/" not in skill
                                               for skill in absent):
        raise TrialError(f"case {name}: absent is a list of skill names, not {absent!r}")
    for item in requires:
        skill = PurePosixPath(item["path"]).parent.name
        if skill in absent:
            raise TrialError(f"case {name}: absent names the skill {skill}, which requires also names")


def check_continuation(case: dict[str, Any], directory: Path) -> None:
    """A continuation case names the answer of the resumed turn, a marker,
    the edits that hold it, the changed mandatory files and the fresh tree."""
    spec = case["continuation"]
    name = case["name"]
    if case.get("mode") != "lifecycle" or case.get("moveAway"):
        raise TrialError(f"case {name}: a continuation case is a lifecycle case without moveAway")
    if any(source.get("external") or "keg" in source for source in case.get("sources", [])):
        # One clone of the prepared commit supplies every source of a
        # continuation, and the runner commits into it between the turns.
        raise TrialError(f"case {name}: a continuation case takes every source from the prepared checkout")
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


def route_for(case: dict[str, Any], records: Path, program: Path, model: str, effort: str) -> dict[str, Any]:
    """The route as data. The configuration script and the fake driver of the
    tests both take it from here. The wrapper of a standalone launch writes
    its record to standard output, `-`, which Grove's run log keeps. The
    adapter gives the program, its arguments and the provider."""
    standalone = case.get("mode") == "standalone"
    return {
        "kinds": case["kinds"],
        "wrapper": str(WRAPPER),
        "records": "-" if standalone else str(records),
        **adapter_of(case).launch(case, records, program, model, effort),
        "model": model,
        "effort": effort,
    }


def config_for(case: dict[str, Any], route: dict[str, Any]) -> str:
    """The configuration script: the case's kinds to the wrapper, any other refuses.
    Each snapshot root also goes to --add-dir, as in the Rhai sample."""
    kinds = ", ".join(f"{rhai_string(kind)}: true" for kind in route["kinds"])
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
{program_args}{add_dirs}
    args += [{', '.join(rhai_string(arg) for arg in fixed)}];
    args.push(request.prompt);
    #{{ status: "selected", program: {rhai_string(route["wrapper"])}, args: args,
       provider: {rhai_string(route.get("provider", "anthropic"))}, model: {rhai_string(model)}, effort: {rhai_string(effort)},
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
        # The search directories: each root of each source, such as `skills`
        # and `craft`, inside its snapshot.
        self.directories = [self._norm(item["path"]) for item in skills.get("directories", [])]
        self.origins = [(source["id"], self._norm(source["originPath"])) for source in skills.get("sources", [])]
        # The roots of every harness, whichever harness ran, so a read of the
        # other harness's configuration still shows as such. The session files:
        # after compaction the summary names the full transcript, which holds
        # the preamble.
        adapters = list(ADAPTERS.values())
        self.transcripts = [self._norm(str(home / adapter.transcript_root)) for adapter in adapters]
        self.residue = [self._norm(str(home / root)) for adapter in adapters for root in adapter.read_residue]
        self.native = [self._norm(str(home / adapter.config_root)) for adapter in adapters]
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
            return "mandatory", self._skill_name(path)
        if self.descriptor and path == self.descriptor:
            return "descriptor", None
        for source, root in self.snapshots:
            if self._inside(path, root):
                skill = self._skill_name(path)
                if os.path.isdir(path):
                    return "listing", skill
                if path.endswith("/SKILL.md"):
                    return ("kind-skill" if skill == self.kind_skill else "skill"), skill
                return "supporting", skill
        for source, root in self.origins:
            if self._inside(path, root):
                return "source-origin", self._skill(path)
        if any(self._inside(path, root) for root in self.transcripts):
            return "transcript", None
        if any(self._inside(path, root) for root in self.residue):
            # A bundled Codex skill lies one level down, in `skills/.system`.
            return "residue", self._skill(path.replace("/skills/.system/", "/skills/"))
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

    def _skill_name(self, path: str) -> str | None:
        """The skill of a path in a search directory is its first segment
        below that directory. A root need not be named `skills`, and Ravel's
        cache path holds a `skills` segment above every snapshot."""
        for directory in self.directories:
            if self._inside(path, directory):
                rest = Path(path).relative_to(directory).parts
                return rest[0] if rest else None
        return self._skill(path)

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
    "planted", "transcript", "residue",
)
# The shell tools whose command names paths: Claude Code's Bash, and each
# nested command of Codex's exec tool. The edit tools write a named file.
SHELL_TOOLS = ("Bash", "exec_command")
EDIT_TOOLS = ("Edit", "Write", "MultiEdit", "NotebookEdit", "apply_patch")
DELEGATE_TOOLS = ("Task", "Agent", "spawn_agent")
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


def separate_lines(command: str) -> str:
    """The command with each newline outside quotes made a `;`. The shell
    ends a command at such a newline, and shlex reads it as a space, so a
    `cd` on a later line would otherwise lose its command position."""
    out = []
    quote = None
    escaped = False
    for char in command:
        if escaped:
            escaped = False
        elif char == "\\" and quote != "'":
            escaped = True
        elif quote:
            quote = None if char == quote else quote
        elif char in "'\"":
            quote = char
        elif char == "\n":
            char = " ; "
        out.append(char)
    return "".join(out)


# Whether extract_events keeps the full result text of each tool call. A
# read inside a script, such as a Python heredoc, names no path, so only its
# result shows what the agent received. It is off for the same reason as
# EXPAND_GLOBS below, and the staging trial runner turns it on.
KEEP_RESULTS = False
# Whether bash_paths expands a glob token as the shell does. The accepted
# common-loading records were extracted without it, so it is off here and a
# reextraction gives their events unchanged. The staging trial runner
# (docs/evaluations/owner-skill-delivery/staging_trial.py) turns it on.
EXPAND_GLOBS = False
# Whether extract_events keeps the class of every path that a tool call
# names, as `pathClasses`. The event's own class keeps only its first read,
# so without it a command that reads a skill and a task file shows no touch
# of the work tree. It is off for the same reason as EXPAND_GLOBS, and the
# staging trial runner turns it on.
KEEP_CLASSES = False
GLOB_CHARS = re.compile(r"[*?\[]")


def glob_matches(token: str, here: str) -> list[str]:
    """The paths that a glob token matches, in the shell's sorted order. An
    unmatched glob stays literal in the shell, so it names no path."""
    import glob

    return sorted(glob.glob(token if token.startswith("/") else os.path.join(here, token)))


def bash_paths(command: str, cwd: str) -> list[str]:
    """The paths that a Bash command names, in order. A `cd` changes the
    directory against which later relative paths resolve. A plain assignment
    `NAME=value` at the start of a command sets a variable, and a later
    `$NAME` or `${NAME}` takes its value. A relative token counts when it
    exists and is `.`, `..`, holds a slash or ends in `.md`. With
    EXPAND_GLOBS, a glob token counts as each path that it matches. A token
    with any other shell expansion counts not at all."""
    import shlex

    try:
        lexer = shlex.shlex(separate_lines(command), posix=True, punctuation_chars=True)
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
        if EXPAND_GLOBS and GLOB_CHARS.search(token):
            matches = glob_matches(token, here)
            if pending_cd:
                pending_cd = False
                if len(matches) == 1:
                    here = os.path.normpath(matches[0])
                continue
            paths += [os.path.normpath(match) for match in matches]
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
    if name in SHELL_TOOLS:
        command = tool_input.get("command", "")
        return command, bash_paths(command, cwd)
    if name == "apply_patch":
        paths = [os.path.normpath(os.path.join(cwd, p)) for p in tool_input.get("paths", [])]
        return ", ".join(paths), paths
    if name == "Skill":
        return tool_input.get("skill", ""), []
    if name == "spawn_agent":
        return tool_input.get("description", ""), []
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
            cwd = entry.get("cwd") or classifier.repo
            target, paths = tool_target(name, tool_input, cwd)
            classes = [classifier.classify(path) + (path,) for path in paths]
            event: dict[str, Any] = {
                "seq": len(events) + 1,
                "tool": name,
                "target": target,
                "error": errors.get(block.get("id", "")),
                "sidechain": bool(entry.get("isSidechain")),
            }
            if name == "exec_command":
                # A Codex command names its own working directory, and every
                # path that it names, absolute.
                event["cwd"], event["paths"] = cwd, paths
            if classifier.marker:
                event["marker"] = marker_places(classifier.marker, tool_input, results.get(block.get("id", ""), ""))
            if KEEP_RESULTS:
                event["resultFull"] = results.get(block.get("id", ""), "")
            if KEEP_CLASSES:
                event["pathClasses"] = sorted({item[0] for item in classes})
            reads = [item for item in classes if item[0] in READ_CLASSES]
            if reads and name not in EDIT_TOOLS:
                event["class"], event["skill"], event["path"] = reads[0]
                source = classifier.source_of(event["path"])
                if source:
                    event["source"] = source
                if len(reads) > 1:
                    event["more"] = [{"class": c, "skill": s, "path": p, "source": classifier.source_of(p)}
                                     for c, s, p in reads[1:]]
            elif name in EDIT_TOOLS:
                event["class"], event["path"] = "action", target
            elif name == "Skill":
                event["class"] = "native-skill-tool"
                event["resultText"] = results.get(block.get("id", ""), "")[:SKILL_RESULT_CHARS]
            elif name in DELEGATE_TOOLS:
                event["class"] = "delegate"
            elif name in SHELL_TOOLS:
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


def skill_files(home: Path) -> tuple[list[str], list[str], list[str]]:
    """(native copies, platform-managed residue, plugin copies): each
    SKILL.md in the home outside Ravel's cache. claude.ai syncs an account's
    skills into .claude/skills/synced. Codex installs its bundled skills into
    .codex/skills/.system, and a ChatGPT sign-in downloads curated plugins
    into .codex/plugins/cache. Those are residue, not native copies. A plugin
    that a case installs puts its skills under .claude/plugins."""
    native, residue, plugins = [], [], []
    cache = home / ".cache" / "ravel"
    managed = [home / root for adapter in ADAPTERS.values() for root in adapter.managed_skills]
    installed = [home / root for adapter in ADAPTERS.values() for root in adapter.plugin_roots]
    for directory, subdirs, files in os.walk(home):
        here = Path(directory)
        if here == cache or cache in here.parents:
            subdirs[:] = []
            continue
        if "SKILL.md" in files:
            relative = str((here / "SKILL.md").relative_to(home))
            if any(here == root or root in here.parents for root in managed):
                residue.append(relative)
            elif any(root in here.parents for root in installed):
                plugins.append(relative)
            else:
                native.append(relative)
    return sorted(native), sorted(residue), sorted(plugins)


# --- Harness adapters ----------------------------------------------------------
#
# One adapter for each harness owns each difference between the harnesses: the
# refusals of the inherited environment and the sign-in store, the pre-launch
# probes and the harness's own links, the arguments of a launch and of a
# resume, the session-file lookup, the stream fields and the startup. The case,
# the source clone, the settings and the record use only the adapter's
# interface. Only the registry at the end of this section and the adapters
# name a harness.
#
# Skill provision is a layer of its own: the Ravel settings, the native copies
# and the planted files. The case and its route decide them for each
# invocation, and a continuation changes the settings between two invocations
# of one case. The adapter gives that layer only the facts of its harness, such
# as the directory where the harness looks for native skills.


class HarnessAdapter:
    """The interface of a harness adapter. A subclass sets the facts below
    and implements each method that has no shared behaviour."""

    # The program on PATH, and the harness name of a case and of a record.
    name = ""
    # The harness name in a message to the owner.
    title = ""
    # The model and the effort of a case that names neither.
    model = ""
    effort = ""
    # Directories of the sandbox home, relative to it: the configuration
    # directory, the native skills and the session files.
    config_root = ""
    native_root = ""
    transcript_root = ""
    # Platform-managed skills, which are no native copies.
    managed_skills: tuple[str, ...] = ()
    # The managed roots whose reads the classifier names residue.
    read_residue: tuple[str, ...] = ()
    # The directories where the harness installs plugins, with their skills.
    plugin_roots: tuple[str, ...] = ()
    # The harness's own permission layer, as the record names it.
    permission_mode = ""
    # How a continuation resumes the first session.
    resume_mechanism = ""
    # The owner's channel of a case's `systemNote`, as the record names it.
    note_channel = ""
    # The case fields that the adapter has no mechanism for yet, each with the
    # leaf that adds it.
    unsupported: dict[str, str] = {}

    def check_case(self, name: str, case: dict[str, Any]) -> None:
        """Refuse a case field that this adapter cannot run."""
        for field, owner in self.unsupported.items():
            if case.get(field) is not None:
                raise TrialError(f"case {name}: the {self.title} adapter has no mechanism for '{field}' yet ({owner})")

    @property
    def store(self) -> str:
        """The one path of the real home that the sandbox home links."""
        raise NotImplementedError

    def refuse(self, environ: Mapping[str, str], owner_home: Path) -> str | None:
        """The refusals of the inherited environment, before any launch of
        the harness. A returned text reports a name that the run replaces."""
        raise NotImplementedError

    def env(self, home: Path) -> dict[str, str]:
        """The names that the harness needs beyond the allow-list."""
        return {}

    def program(self, environ: Mapping[str, str]) -> Path:
        named = environ.get("HARNESS_TRIAL_PROGRAM") or shutil.which(self.name, path=environ.get("PATH"))
        if not named or not os.access(named, os.X_OK):
            raise TrialError(f"no '{self.name}' program found on PATH; install it, or put it on PATH")
        return Path(named).resolve()

    def prelaunch(self, program: Path, env: dict[str, str], tmp: Path, trial: Path,
                  config_dir: Path) -> tuple[dict[str, Any], str | None, dict[str, Any] | None]:
        """The probes before the launch: (the sign-in facts, the configuration
        directory that the harness reports, its input before the launch)."""
        raise NotImplementedError

    def own_links(self, links: list[tuple[str, str]], program: Path) -> tuple[list[tuple[str, str]],
                                                                            list[tuple[str, str]]]:
        """(other links, the harness's own links) of the sandbox home."""
        return links, []

    def launch(self, case: dict[str, Any], records: Path, program: Path, model: str,
               effort: str) -> dict[str, Any]:
        """The harness's part of the route: its program, the program's own
        arguments, whether snapshot roots go to --add-dir, the fixed
        arguments and the provider."""
        raise NotImplementedError

    def resume_args(self, route: dict[str, Any], session_id: str) -> list[str]:
        """The arguments of a resumed turn before the answer: the fixed
        arguments of the route, then the harness's own resume."""
        raise NotImplementedError

    def record_extras(self, auth: dict[str, Any], replaced: str | None,
                      prelaunch: dict[str, Any] | None) -> dict[str, Any]:
        """The facts that only this harness adds to the record's harness."""
        return {}

    def write_settings(self, case: dict[str, Any], home: Path) -> dict[str, Any] | None:
        """The case's settings file of the harness in the sandbox home."""
        return None

    def install_plugins(self, case: dict[str, Any], program: Path, env: dict[str, str], tmp: Path, home: Path,
                        checkout: Path, commit: str) -> dict[str, Any] | None:
        """The case's native plugins, installed into the sandbox home from the
        marketplace of the prepared checkout. `check_case` refuses the field
        for a harness without this mechanism."""
        return None

    def stream_ids(self, stream: list[dict[str, Any]]) -> tuple[str | None, str | None]:
        """The session ID and the working directory that a stream names."""
        raise NotImplementedError

    def stream_init(self, record: Path) -> tuple[str | None, str | None]:
        """The session ID and the working directory of one launch record."""
        stream = record / "stream.jsonl"
        return self.stream_ids(read_jsonl(stream) if stream.exists() else [])

    def session_files(self, home: Path) -> list[Path]:
        """Every session file of the harness in the sandbox home."""
        raise NotImplementedError

    def session_files_of(self, home: Path, session_id: str) -> list[Path]:
        """The session files of one session."""
        raise NotImplementedError

    def session_file(self, home: Path, session_id: str) -> Path | None:
        return next(iter(self.session_files_of(home, session_id)), None)

    def session_cwd(self, transcript: Path) -> str | None:
        """The working directory that a session file names, when its stream
        names none."""
        return None

    def describe(self, label: str, view: dict[str, Any], child: dict[str, str], stream: list[dict[str, Any]],
                 stream_sha: str | None, home: Path, repo: Path,
                 live: Mapping[str, str], marker: str | None = None, window: tuple[int, int] | None = None,
                 turns: bool = False, prompt: str | None = None) -> dict[str, Any]:
        """One launch: its request view, the environment of its harness and
        its ordered events. A window limits the main session file
        to the lines of this launch, and the digest then covers those lines
        only."""
        raise NotImplementedError

    def render_rows(self, h: dict[str, Any]) -> list[str]:
        """The rows of the facts table that only this harness has."""
        raise NotImplementedError

    def compaction_mechanism(self, mode: str) -> str:
        """How a compaction case of this mode compacts in one managed process."""
        raise NotImplementedError

    def compact_turn(self, spec: dict[str, Any]) -> dict[str, Any]:
        """The feeder's turn that compacts, from the case's compaction."""
        raise NotImplementedError

    def feed(self, argv: list[str], prompt: str, turns: list[dict[str, Any]],
             gate: Callable[[str | None, str], bool], log: Callable[[str, dict[str, Any]], None]) -> int:
        """The feeder's conversation. Run the harness with `argv`, pass its
        output through, send the prompt, then each turn after the end of the
        turn before. Before a turn with a gate, `gate(session, mode)` decides,
        and a failed gate ends the input. Return the harness's status."""
        raise NotImplementedError

    def after_compaction(self, records: list[dict[str, Any]], marks: dict[str, Any]) -> dict[str, Any] | None:
        """The preamble findings in what a session file shows after its last
        compaction and before the next turn, or None without a compaction.
        The feeder's gate reads them."""
        raise NotImplementedError

    def compaction_facts(self, records: list[dict[str, Any]], marks: dict[str, Any],
                         mode: str) -> tuple[dict[str, Any], list[str]]:
        """The record's facts of the last compaction of a session file, and
        each failure that they show. Without a compaction the facts are empty."""
        raise NotImplementedError

    def render_compaction(self, compaction: dict[str, Any]) -> list[str]:
        """The record's lines on this harness's own compaction facts."""
        raise NotImplementedError

    def render_prelaunch(self, h: dict[str, Any]) -> list[str]:
        """The record's lines on the harness's input before the launch."""
        return []


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
DEFAULT_MODEL = "claude-opus-5-5"
DEFAULT_EFFORT = "medium"
# Claude Code's own permission layer. No call prompts or is denied, so the
# transcript shows what the agent chose to read. Ravel confines no launch.
PERMISSION_MODE = "bypassPermissions"
INIT_KEYS = (
    "claude_code_version", "model", "permissionMode", "cwd", "tools", "mcp_servers",
    "slash_commands", "skills", "plugins", "agents", "output_style", "apiKeySource",
)


class ClaudeAdapter(HarnessAdapter):
    """Claude Code: `claude -p` with stream-json output. Its session files are
    `<id>.jsonl` under the sandbox `.claude/projects`."""

    name = "claude"
    title = "Claude Code"
    model = DEFAULT_MODEL
    effort = DEFAULT_EFFORT
    config_root = ".claude"
    native_root = ".claude/skills"
    transcript_root = ".claude/projects"
    # claude.ai syncs an account's skills into the sandbox.
    managed_skills = (".claude/skills/synced",)
    # `claude plugin install` copies a plugin into .claude/plugins/cache.
    plugin_roots = (".claude/plugins",)
    permission_mode = PERMISSION_MODE
    # Claude Code keeps the session ID without --fork-session and appends to
    # the session file.
    resume_mechanism = "claude -p --resume <session id>, without --fork-session, outside Ravel's managed process"
    delegate_mechanism = ("Claude Code's Agent tool, in process: the delegate's tool calls run in children of "
                          "the harness process, with its environment")
    note_channel = "`--append-system-prompt`"

    @property
    def store(self) -> str:
        return "Library/Keychains" if platform.system() == "Darwin" else ".claude/.credentials.json"

    def refuse(self, environ: Mapping[str, str], owner_home: Path) -> str | None:
        """Refuse an inherited Claude Code location variable."""
        for name in LOCATION_NAMES:
            if name in environ:
                raise TrialError(
                    f"the environment holds {name}, which can expose a real Claude Code "
                    f"configuration, plugin tree or sign-in; run the trial again without {name}"
                )
        return None

    def prelaunch(self, program: Path, env: dict[str, str], tmp: Path, trial: Path,
                  config_dir: Path) -> tuple[dict[str, Any], str | None, dict[str, Any] | None]:
        """`claude auth status` names the effective configuration directory."""
        auth_text = run_checked([str(program), "auth", "status"], cwd=tmp, env=env)
        try:
            auth = json.loads(auth_text)
        except json.JSONDecodeError as error:
            raise TrialError(f"claude auth status wrote no JSON: {error}") from None
        reported = auth.get("configDirectory")
        if not reported or os.path.realpath(reported) != os.path.realpath(config_dir):
            raise TrialError(f"Claude Code reports the configuration directory '{reported}', not the sandbox {config_dir}")
        return auth, reported, None

    def launch(self, case: dict[str, Any], records: Path, program: Path, model: str,
               effort: str) -> dict[str, Any]:
        """A compaction case selects the feeder as its program, with the
        harness as the feeder's command. It gives no --add-dir: Claude Code
        writes additional directories into the system prompt, which survives
        compaction, so the snapshot paths would survive too."""
        compaction = case.get("compaction") is not None
        # An owner's instruction for one case, in Claude Code's owner channel.
        # The prepared prompt of Ravel stays as it is.
        note = ["--append-system-prompt", case["systemNote"]] if case.get("systemNote") else []
        return {
            **(feeder_route(records, program) if compaction else {"program": str(program), "programArgs": []}),
            "addDirs": not compaction,
            "fixedArgs": [
                "-p", "--output-format", "stream-json", "--verbose",
                "--model", model, "--effort", effort, "--permission-mode", PERMISSION_MODE, *note,
            ],
            "provider": "anthropic",
        }

    def resume_args(self, route: dict[str, Any], session_id: str) -> list[str]:
        return [*route["fixedArgs"], "--resume", session_id]

    def install_plugins(self, case: dict[str, Any], program: Path, env: dict[str, str], tmp: Path, home: Path,
                        checkout: Path, commit: str) -> dict[str, Any] | None:
        """Claude Code's own commands, with HOME the sandbox home: `plugin
        marketplace add` of the checkout, then `plugin install` of each plugin
        at user scope. A dependency that the manifest declares installs with
        its plugin. The checkout must stay clean, because every run of the
        prepared directory reads it."""
        if not case.get("nativePlugins"):
            return None
        added = plugin_command(program, ["marketplace", "add", str(checkout), "--json"], env, tmp)
        installs = [{"id": ident, "result": plugin_command(program, ["install", ident, "--json"], env, tmp)}
                    for ident in case["nativePlugins"]]
        listed = json.loads(run_checked([str(program), "plugin", "list", "--json"], cwd=tmp, env=env))
        status = run_checked(["git", "status", "--porcelain"], cwd=checkout, env=git_env(env))
        if status:
            raise TrialError(f"adding the marketplace changed the checkout {checkout}: {status}")
        return {
            "marketplace": {"path": str(checkout), "commit": commit, "name": added.get("marketplace"),
                            "checkoutClean": True, "result": added},
            "installs": installs,
            "installed": installed_plugins(home, listed),
        }

    def write_settings(self, case: dict[str, Any], home: Path) -> dict[str, Any] | None:
        """The case's `.claude/settings.json`, such as a deny rule that makes
        the native loader refuse a skill it holds."""
        settings = case.get("claudeSettings")
        if settings is None:
            return None
        path = home / self.config_root / "settings.json"
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(json.dumps(settings, indent=2) + "\n")
        return settings

    def compaction_mechanism(self, mode: str) -> str:
        return ("claude -p --input-format stream-json in Ravel's managed process; the feeder sends the "
                "prompt, then /compact with the instructions, then the answer")

    def compact_turn(self, spec: dict[str, Any]) -> dict[str, Any]:
        return {"text": f"/compact {spec['instructions']}"}

    def feed(self, argv: list[str], prompt: str, turns: list[dict[str, Any]],
             gate: Callable[[str | None, str], bool], log: Callable[[str, dict[str, Any]], None]) -> int:
        """Claude Code with stream-json input. A turn ends with the `result`
        event, and the `init` event names the session."""
        child = subprocess.Popen([*argv, "--input-format", "stream-json"], stdin=subprocess.PIPE,
                                 stdout=subprocess.PIPE)
        assert child.stdin is not None and child.stdout is not None

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
            if not turns:
                close()
                continue
            turn = turns.pop(0)
            if turn.get("gate") and not gate(session, turn["gate"]):
                turns.clear()
                close()
                continue
            send(turn["text"], sent)
            sent += 1
        close()
        return child.wait()

    def after_compaction(self, records: list[dict[str, Any]], marks: dict[str, Any]) -> dict[str, Any] | None:
        return after_boundary(records, marks)

    def compaction_facts(self, records: list[dict[str, Any]], marks: dict[str, Any],
                         mode: str) -> tuple[dict[str, Any], list[str]]:
        """The summary, what Claude Code added after the boundary and the
        context of the first model request after it."""
        start = boundary_index(records)
        if start is None:
            return {}, ["the session holds no compaction boundary"]
        failures = []
        summary = next((e for e in records[start + 1:] if e.get("isCompactSummary")), None)
        summary_text = "\n".join(strings((summary or {}).get("message")))
        facts: dict[str, Any] = {
            "summary": {"chars": len(summary_text), "sha256": sha256_bytes(summary_text.encode()),
                        "findings": context_findings(summary_text, summary_text, marks)},
            "afterBoundary": after_boundary(records, marks),
        }
        failures += [f"after the boundary: {problem}"
                     for problem in compaction_problems(facts["afterBoundary"] or {}, mode)] \
            if facts["afterBoundary"] else []
        request_at = next((n for n in range(start + 1, len(records)) if records[n].get("type") == "api-request"),
                          len(records))
        facts["attachments"] = [(e.get("attachment") or {}).get("type") for e in records[start + 1:request_at]
                                if e.get("type") == "attachment"]
        # A file that Claude Code restored after the boundary, with its path.
        facts["restored"] = [(e.get("attachment") or {}).get("filename") for e in records[start + 1:request_at]
                             if e.get("type") == "attachment" and (e.get("attachment") or {}).get("filename")]
        facts["api"] = api = api_context(records, marks)
        if not api["found"]:
            failures.append("the session file holds no model request after the boundary")
        else:
            if api["keep"] != 0 or api["missingBlobs"] or not api["system"]:
                failures.append(f"the request after the boundary is not complete in the session file: "
                                f"keep {api['keep']}, {api['missingBlobs']} blobs missing, "
                                f"system prompt {'found' if api['system'] else 'missing'}")
            failures += [f"model context: {problem}"
                         for problem in compaction_problems(api["findings"], mode, terms=False)]
        return facts, failures

    def render_compaction(self, compaction: dict[str, Any]) -> list[str]:
        api = compaction.get("api") or {}
        summary = compaction.get("summary") or {}
        lines = [
            f"- Summary instructions, which the agent sees after the boundary: `{cell(compaction['instructions'])}`.",
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
        return lines

    def stream_ids(self, stream: list[dict[str, Any]]) -> tuple[str | None, str | None]:
        init = next((e for e in stream if e.get("type") == "system" and e.get("subtype") == "init"), {})
        return init.get("session_id"), init.get("cwd")

    def session_files(self, home: Path) -> list[Path]:
        projects = home / self.transcript_root
        return sorted(projects.rglob("*.jsonl")) if projects.is_dir() else []

    def session_files_of(self, home: Path, session_id: str) -> list[Path]:
        return [path for path in self.session_files(home) if path.stem == session_id]

    def startup(self, stream: list[dict[str, Any]], session: list[dict[str, Any]]) -> dict[str, Any]:
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

    def describe(self, label: str, view: dict[str, Any], child: dict[str, str], stream: list[dict[str, Any]],
                 stream_sha: str | None, home: Path, repo: Path,
                 live: Mapping[str, str], marker: str | None = None, window: tuple[int, int] | None = None,
                 turns: bool = False, prompt: str | None = None) -> dict[str, Any]:
        """The events come from the session file of the launch. When the
        harness wrote none, they come from its stream, which holds the same tool calls. Each Agent call of the main
        transcript adds a delegation, which the prompt of the launch lets the
        record compare with the obligations of the preamble."""
        session_id, cwd = self.stream_ids(stream)
        classifier = Classifier(view, home, Path(cwd or repo), marker, turns)
        transcripts = []
        main_entries: list[dict[str, Any]] = []
        session_dir = None
        for path in self.session_files(home):
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
                "startup": self.startup(stream, entries) if main else None,
            })
        if session_id and not any(t["main"] for t in transcripts):
            transcripts.append({
                "file": "stream", "sha256": stream_sha, "main": True,
                "events": extract_events(stream, classifier), "startup": self.startup(stream, []),
            })
        skills = view.get("skills") or {}
        descriptor = skills.get("descriptorPath")
        mandatory = [os.path.realpath(item["path"]) for item in skills.get("mandatory", [])]
        delegated = self.delegations(main_entries, session_dir, home, prompt, descriptor, child, transcripts,
                                     mandatory) if session_dir else []
        return {
            **launch_facts(label, view, session_id, child, live, stream_sha),
            "startup": self.startup(stream, []) if not transcripts else None,
            "transcripts": transcripts,
            "delegations": delegated,
            # The last result event of the stream, absent when Ravel ended the
            # harness before it wrote one.
            "streamResult": next((e.get("subtype") for e in reversed(stream) if e.get("type") == "result"), None),
        }

    def delegations(self, entries: list[dict[str, Any]], session_dir: Path, home: Path, prompt: str | None,
                    descriptor: str | None, child: Mapping[str, str],
                    transcripts: list[dict[str, Any]], mandatory: list[str]) -> list[dict[str, Any]]:
        """Each Agent call of the parent: what the parent gave the delegate,
        the delegate's transcript and the ending context that it inherited.
        Claude Code links a delegate's transcript to the call through the
        `toolUseId` of `subagents/agent-<id>.meta.json` beside the transcript."""
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
                if not (isinstance(block, dict) and block.get("type") == "tool_use"
                        and block.get("name") in ("Task", "Agent")):
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
                    "mechanism": self.delegate_mechanism,
                    "inherited": [name for name in ENDING_NAMES if name in child],
                })
        return found

    def render_rows(self, h: dict[str, Any]) -> list[str]:
        return ["| Configuration directory | sandbox `.claude`; `claude auth status` reports the same directory |"]


# Codex keeps its configuration in CODEX_HOME, which defaults to `~/.codex`.
CODEX_HOME = "CODEX_HOME"
CODEX_MODEL = "gpt-6.1-sol"
CODEX_EFFORT = "medium"
# Codex's own sandbox, on every route. Reads are unrestricted, and writes
# reach the working directory, TMPDIR and /tmp, with no network. Ravel
# allocates the session channel under TMPDIR, so `end-session` reaches it.
# Ravel confines no launch, so this sandbox is the only one.
CODEX_SANDBOX = "workspace-write"
CODEX_APPROVAL = "never"
# The bundled system skills that Codex installs at startup, and the curated
# plugins that a ChatGPT sign-in downloads. Both are platform-managed residue.
CODEX_RESIDUE = (".codex/skills/.system", ".codex/plugins/cache")
# The case fields that have no Codex mechanism.
CODEX_LATER = {
    "claudeSettings": "no Codex leaf; it is a Claude Code setting",
    "nativePlugins": "no Codex leaf; Codex has no native skill loader in the measured version",
}
SHELLS = ("bash", "zsh", "sh", "dash", "ksh", "fish")
# An encrypted `message` of a direct `spawn_agent` call: a Fernet token.
ENCRYPTED = re.compile(r"^gAAAAA[A-Za-z0-9_=-]+$")
# How Codex 0.166.0 compacts each loss mode, as extra options of `codex
# app-server` and a description. `thread/compact/start` takes only the thread,
# so no summary instructions reach Codex. The OpenAI provider compacts on the
# server (`RemoteCompactionSupport::V2`,
# https://github.com/openai/codex/blob/rust-v0.166.0/codex-rs/model-provider/src/provider.rs).
# That keeps each user message word for word, newest first, up to 64,000
# tokens, then one encrypted compaction item (`build_v2_compacted_history`,
# https://github.com/openai/codex/blob/rust-v0.166.0/codex-rs/core/src/compact_remote_v2.rs).
# The prompt holds the preamble, so no Codex mechanism loses the path alone.
# The development feature `token_budget` makes a manual compaction install a
# fresh context window that keeps no user message (`run_manual_compact_task`,
# https://github.com/openai/codex/blob/rust-v0.166.0/codex-rs/core/src/compact_token_budget.rs),
# which loses the whole preamble.
CODEX_COMPACTION = {
    "path": ([], "remote compaction v2, the default, which keeps each user message word for word"),
    "preamble": (["--enable", "token_budget"],
                 "the development feature token_budget, whose fresh context window keeps no user message"),
}
# The client name of the feeder. Codex records it as the session's originator.
CODEX_CLIENT = "grove-trial-feeder"
# The fields of Codex's session metadata and turn context that a record keeps.
# The metadata also holds the account and user identifiers, which it drops.
CODEX_META_KEYS = ("cli_version", "originator", "source", "thread_source", "model_provider", "cwd", "history_mode")
CODEX_TURN_KEYS = ("model", "effort", "approval_policy", "sandbox_policy", "summary")


class CodexAdapter(HarnessAdapter):
    """Codex: `codex exec --json` with CODEX_HOME set to the sandbox `.codex`,
    which links only `auth.json`. Its session files are
    `rollout-<time>-<thread>.jsonl` under the sandbox `.codex/sessions`, and
    the adapter reads their items as the entries of a Claude Code transcript."""

    name = "codex"
    title = "Codex"
    model = CODEX_MODEL
    effort = CODEX_EFFORT
    config_root = ".codex"
    native_root = ".codex/skills"
    transcript_root = ".codex/sessions"
    managed_skills = CODEX_RESIDUE
    read_residue = CODEX_RESIDUE
    permission_mode = f"sandbox {CODEX_SANDBOX}, approval {CODEX_APPROVAL}"
    # Codex 0.166.0 opens the existing rollout file for append on a resume
    # (`open_rollout_for_append` in
    # https://github.com/openai/codex/blob/rust-v0.166.0/codex-rs/rollout/src/recorder.rs).
    # `resume` takes no --sandbox of its own: the options of `codex exec`
    # before it are shared, and `thread_resume_params_from_config` passes their
    # sandbox and approval policy to the resumed thread
    # (https://github.com/openai/codex/blob/rust-v0.166.0/codex-rs/exec/src/lib.rs).
    resume_mechanism = "codex exec <the route's options> resume <thread id> <answer>, outside Ravel's managed process"
    delegate_mechanism = ("Codex's multi-agent v2 `spawn_agent`, in process: the delegate is a thread of the same "
                          "Codex process, and its commands run with the process environment and the parent's "
                          "sandbox and approval policy")
    note_channel = "`-c developer_instructions`, a developer message that a delegate inherits"
    unsupported = CODEX_LATER

    @property
    def store(self) -> str:
        return ".codex/auth.json"

    def refuse(self, environ: Mapping[str, str], owner_home: Path) -> str | None:
        """Refuse an inherited CODEX_HOME in the real home. The test uses the
        physical path, so an alias of the real home refuses too. An empty
        value selects the default in the real home and refuses as well. A
        value outside the real home is replaced by the sandbox `.codex`, and
        the returned text reports the replacement."""
        if CODEX_HOME not in environ:
            return None
        value = environ[CODEX_HOME]
        cwd = os.getcwd()
        home = os.path.realpath(owner_home)
        inside = not value or physical(value, cwd) == home or physical(value, cwd).startswith(home.rstrip("/") + "/")
        if inside:
            raise TrialError(
                f"the environment holds {CODEX_HOME}={value!r}, which is in the real home {owner_home} and exposes its "
                f"configuration, hooks, plugins and skills; run the trial again without {CODEX_HOME}"
            )
        return f"the inherited {CODEX_HOME} {value} is outside the real home; this run uses the sandbox .codex"

    def env(self, home: Path) -> dict[str, str]:
        return {CODEX_HOME: str(home / self.config_root)}

    def prelaunch(self, program: Path, env: dict[str, str], tmp: Path, trial: Path,
                  config_dir: Path) -> tuple[dict[str, Any], str | None, dict[str, Any] | None]:
        """`codex login status` reads the linked sign-in. `codex debug
        prompt-input` renders the model-visible input without a model call,
        and its skill roots name the effective configuration directory."""
        login = subprocess.run([str(program), "login", "status"], cwd=tmp, env=env, capture_output=True, text=True,
                               check=False)
        auth = {"loggedIn": login.returncode == 0, "status": (login.stdout + login.stderr).strip()}
        if login.returncode != 0:
            raise TrialError(f"codex login status reports no sign-in: {auth['status']}; sign in to Codex as the owner")
        rendered_text = run_checked([str(program), "debug", "prompt-input"], cwd=tmp, env=env)
        (trial / "prompt-input.json").write_text(rendered_text + "\n")
        try:
            rendered = json.loads(rendered_text)
        except json.JSONDecodeError as error:
            raise TrialError(f"codex debug prompt-input wrote no JSON: {error}") from None
        reported = self.config_check(rendered, config_dir)
        return auth, reported, self.input(rendered)

    def config_check(self, rendered: list[dict[str, Any]], config_dir: Path) -> str:
        """The sandbox `.codex` is the effective configuration directory: the
        skill roots of `codex debug prompt-input` name its `skills/.system`."""
        listing = self.input(rendered)["skillListing"] or {}
        roots = list((listing.get("roots") or {}).values())
        expected = os.path.realpath(config_dir / "skills" / ".system")
        if expected not in [os.path.realpath(root) for root in roots]:
            raise TrialError(f"Codex reports the skill roots {roots}, not the sandbox {config_dir}/skills/.system; "
                             f"its configuration directory is not the sandbox {config_dir}")
        return expected

    def own_links(self, links: list[tuple[str, str]], program: Path) -> tuple[list[tuple[str, str]],
                                                                            list[tuple[str, str]]]:
        """Codex links `apply_patch` and `applypatch` to its own executable
        under `.codex/tmp/arg0-*` while it runs, and removes them when it
        exits. Ravel ends the session group before that, so they remain. A
        link there to any other target stays among the other links, and the
        link check fails on it."""
        own = [(path, target) for path, target in links
               if path.startswith(".codex/tmp/arg0") and os.path.realpath(target) == os.path.realpath(program)]
        return [link for link in links if link not in own], own

    def launch(self, case: dict[str, Any], records: Path, program: Path, model: str,
               effort: str) -> dict[str, Any]:
        """Codex reads every path under its sandbox, so it needs no --add-dir.
        A compaction case runs the feeder, which runs `codex app-server` with
        the same model, effort, sandbox and approval policy."""
        compaction = case.get("compaction")
        # An owner's instruction for one case, in Codex's owner channel: the
        # configuration key `developer_instructions` adds a developer message
        # before the skill instructions, and a spawned delegate inherits it
        # (`build_agent_shared_config` in
        # https://github.com/openai/codex/blob/rust-v0.166.0/codex-rs/core/src/agent/child_config.rs).
        # A JSON string is a TOML basic string. The prepared prompt of Ravel
        # stays as it is.
        note = ["-c", f"developer_instructions={json.dumps(case['systemNote'])}"] if case.get("systemNote") else []
        if compaction is not None:
            return {
                **feeder_route(records, program),
                "addDirs": False,
                "fixedArgs": [
                    "app-server", "-c", f'model="{model}"', "-c", f'model_reasoning_effort="{effort}"',
                    "-c", f'sandbox_mode="{CODEX_SANDBOX}"', "-c", f'approval_policy="{CODEX_APPROVAL}"',
                    *note, *CODEX_COMPACTION[compaction["mode"]][0],
                ],
                "provider": "openai",
            }
        return {
            "program": str(program),
            "programArgs": [],
            "addDirs": False,
            "fixedArgs": [
                "exec", "--json", "--skip-git-repo-check", "--color", "never",
                "--model", model, "-c", f'model_reasoning_effort="{effort}"',
                "--sandbox", CODEX_SANDBOX, "-c", f'approval_policy="{CODEX_APPROVAL}"', *note,
            ],
            "provider": "openai",
        }

    def resume_args(self, route: dict[str, Any], session_id: str) -> list[str]:
        return [*route["fixedArgs"], "resume", session_id]

    def record_extras(self, auth: dict[str, Any], replaced: str | None,
                      prelaunch: dict[str, Any] | None) -> dict[str, Any]:
        return {
            "permissionMode": f"sandbox {CODEX_SANDBOX}, approval {CODEX_APPROVAL}",
            "sandboxMode": CODEX_SANDBOX, "approvalPolicy": CODEX_APPROVAL,
            "loginStatus": auth.get("status"), "codexHome": replaced or f"unset; this run sets {CODEX_HOME}",
            "prelaunch": prelaunch,
        }

    def stream_ids(self, stream: list[dict[str, Any]]) -> tuple[str | None, str | None]:
        """Codex names its thread in `thread.started` and no working
        directory, so the caller takes that from the session file."""
        started = next((e for e in stream if e.get("type") == "thread.started"), None)
        if started is not None:
            return started.get("thread_id"), None
        # The app-server stream of a compaction case names it in `thread/started`.
        started = next((e for e in stream if e.get("method") == "thread/started"), {})
        return ((started.get("params") or {}).get("thread") or {}).get("id"), None

    def session_files(self, home: Path) -> list[Path]:
        sessions = home / self.transcript_root
        return sorted(sessions.rglob("rollout-*.jsonl")) if sessions.is_dir() else []

    def session_files_of(self, home: Path, session_id: str) -> list[Path]:
        return [path for path in self.session_files(home) if path.stem.endswith(session_id)]

    def session_cwd(self, transcript: Path) -> str | None:
        """The working directory of a Codex session, from its `session_meta`."""
        meta = next((r.get("payload") or {} for r in read_jsonl(transcript) if r.get("type") == "session_meta"), {})
        return meta.get("cwd")

    def describe(self, label: str, view: dict[str, Any], child: dict[str, str], stream: list[dict[str, Any]],
                 stream_sha: str | None, home: Path, repo: Path,
                 live: Mapping[str, str], marker: str | None = None, window: tuple[int, int] | None = None,
                 turns: bool = False, prompt: str | None = None) -> dict[str, Any]:
        """The stream's `thread.started` names the thread, and its session
        file holds its items. When no session file exists, the items come from
        the stream, which names no working directory. A resumed turn appends
        to the session file of the first turn, so a window limits that file to
        the lines of this launch, as for Claude Code."""
        session_id, _ = self.stream_ids(stream)
        transcripts = []
        for path in self.session_files_of(home, session_id) if session_id else []:
            raw = path.read_bytes()
            if window:
                raw = b"".join(jsonl_lines(raw)[window[0]:window[1]])
            records = read_jsonl_bytes(raw)
            startup = self.startup(records)
            classifier = Classifier(view, home, Path(startup["init"].get("cwd") or repo), marker, turns)
            transcripts.append({
                "file": str(path.relative_to(home)), "sha256": sha256_bytes(raw),
                **({"lines": list(window)} if window else {}), "main": True,
                "events": extract_events(self.entries(records), classifier), "startup": startup,
            })
        if session_id and not transcripts:
            transcripts.append({
                "file": "stream", "sha256": stream_sha, "main": True,
                "events": extract_events(self.entries(stream), Classifier(view, home, repo, marker, turns)),
                "startup": None,
            })
        main_records = []
        for path in self.session_files_of(home, session_id) if session_id else []:
            raw = path.read_bytes()
            main_records = read_jsonl_bytes(b"".join(jsonl_lines(raw)[window[0]:window[1]]) if window else raw)
        children = self.delegate_files(home, session_id) if session_id else {}
        for thread, (path, records) in children.items():
            meta = self.meta(records)
            classifier = Classifier(view, home, Path(meta.get("cwd") or repo), marker, turns)
            transcripts.append({
                "file": str(path.relative_to(home)), "sha256": sha256_file(path), "main": False,
                "thread": thread, "events": extract_events(self.entries(records), classifier),
                "startup": self.startup(records),
            })
        skills = view.get("skills") or {}
        mandatory = [os.path.realpath(item["path"]) for item in skills.get("mandatory", [])]
        delegated = self.delegations(main_records, children, home, prompt, skills.get("descriptorPath"), child,
                                     transcripts, mandatory)
        return {
            **launch_facts(label, view, session_id, child, live, stream_sha),
            "startup": None,
            "transcripts": transcripts,
            "delegations": delegated,
            # The last turn event of the stream, absent when Ravel ended the
            # harness before it wrote one. An app-server stream names it as a
            # method.
            "streamResult": next((e.get("type") or e.get("method") for e in reversed(stream)
                                  if e.get("type") in ("turn.completed", "turn.failed", "error")
                                  or e.get("method") in ("turn/completed", "error")), None),
        }

    @staticmethod
    def meta(records: list[dict[str, Any]]) -> dict[str, Any]:
        """The session's own metadata: the first `session_meta`. A delegate
        that forks the parent's history copies the parent's metadata after
        its own."""
        return next((r.get("payload") or {} for r in records if r.get("type") == "session_meta"), {})

    def delegate_files(self, home: Path, thread: str) -> dict[str, tuple[Path, list[dict[str, Any]]]]:
        """The session file of each delegate below a thread, by its thread
        ID, with its records. A delegate's own `session_meta` names its
        parent thread, and a delegate of a delegate names the delegate."""
        metas = {}
        for path in self.session_files(home):
            records = read_jsonl(path)
            meta = self.meta(records)
            if meta.get("parent_thread_id"):
                metas[meta.get("id")] = (path, records, meta["parent_thread_id"])
        found: dict[str, tuple[Path, list[dict[str, Any]]]] = {}
        parents = {thread}
        while True:
            new = {tid: (path, records) for tid, (path, records, parent) in metas.items()
                   if parent in parents and tid not in found}
            if not new:
                return found
            found.update(new)
            parents |= set(new)

    def delegations(self, records: list[dict[str, Any]], children: dict[str, tuple[Path, list[dict[str, Any]]]],
                    home: Path, prompt: str | None, descriptor: str | None, child: Mapping[str, str],
                    transcripts: list[dict[str, Any]], mandatory: list[str]) -> list[dict[str, Any]]:
        """Each delegate that the parent started. The parent's
        `SubAgentActivity` item of kind `started` names the call ID of its
        `spawn_agent` call and the delegate's thread. The `message` of a
        direct call is encrypted in every transcript, because its parameter
        is declared `with_encrypted()`
        (https://github.com/openai/codex/blob/rust-v0.166.0/codex-rs/core/src/tools/handlers/multi_agents_spec.rs).
        A call from code mode, inside the `exec` tool's JavaScript, is
        plaintext (`agent_message_from_tool` in
        https://github.com/openai/codex/blob/rust-v0.166.0/codex-rs/core/src/tools/handlers/multi_agents_v2.rs),
        and the record then holds the JavaScript of that call."""
        calls: dict[str, dict[str, Any]] = {}
        cells: list[tuple[int, str]] = []
        # The messages of each delegate to the parent, in order, and the
        # targets of the parent's `send_message` calls.
        answers: dict[str, list[str]] = {}
        sent: list[str] = []
        started = []
        for n, record in enumerate(records):
            payload = record.get("payload") if isinstance(record.get("payload"), dict) else {}
            kind = payload.get("type")
            if record.get("type") == "response_item" and kind == "function_call" \
                    and payload.get("name") == "spawn_agent":
                try:
                    arguments = json.loads(payload.get("arguments") or "{}")
                except json.JSONDecodeError:
                    arguments = {}
                calls[payload.get("call_id")] = arguments if isinstance(arguments, dict) else {}
            elif record.get("type") == "response_item" and kind == "function_call" \
                    and payload.get("name") == "send_message":
                try:
                    sent.append(str(json.loads(payload.get("arguments") or "{}").get("target")))
                except (json.JSONDecodeError, AttributeError):
                    sent.append("")
            elif record.get("type") == "response_item" and kind == "custom_tool_call" \
                    and "spawn_agent" in str(payload.get("input")):
                cells.append((n, str(payload.get("input"))))
            elif record.get("type") == "response_item" and kind == "agent_message":
                text = "".join(part.get("text", "") for part in payload.get("content") or [] if isinstance(part, dict))
                answers.setdefault(str(payload.get("author")), []).append(text)
            elif record.get("type") == "event_msg" and kind == "item_completed":
                item = payload.get("item") or {}
                if item.get("type") == "SubAgentActivity" and item.get("kind") == "started":
                    started.append((n, item))
        obligations = obligation_lines(prompt or "")
        found = []
        for n, item in started:
            call_id = item.get("id")
            arguments = calls.get(call_id)
            if arguments is not None:
                message = str(arguments.get("message") or "")
                encrypted = bool(ENCRYPTED.match(message))
                form, text = ("direct, encrypted" if encrypted else "direct"), "" if encrypted else message
            else:
                cell_text = next((text for at, text in reversed(cells) if at < n), "")
                form, text, arguments, encrypted = "code mode", cell_text, {}, False
            thread = item.get("agent_thread_id")
            path, child_records = children.get(thread, (None, []))
            transcript = next((t for t in transcripts if t.get("thread") == thread), None)
            source = ((self.meta(child_records).get("source") or {}).get("subagent") or {}).get("thread_spawn") or {}
            turn = next((r.get("payload") or {} for r in reversed(child_records) if r.get("type") == "turn_context"), {})
            path_name = str(item.get("agent_path"))
            replies = answers.get(path_name, [])
            # The delegate's last report is its `FINAL_ANSWER` message.
            final = next((text for text in reversed(replies) if "Message Type: FINAL_ANSWER" in text),
                         replies[-1] if replies else "")
            names = {path_name, str(arguments.get("task_name")), path_name.rsplit("/", 1)[-1]}
            history = [r.get("payload") or {} for r in child_records if r.get("type") == "response_item"
                       and (r.get("payload") or {}).get("type") == "message"
                       and (r.get("payload") or {}).get("role") == "user"]
            found.append({
                "toolUseId": call_id,
                "description": arguments.get("task_name") or item.get("agent_path"),
                "subagentType": arguments.get("agent_type") or "default",
                "background": None,
                "form": form,
                "forkTurns": arguments.get("fork_turns") or ("all, the default" if arguments is not None
                                                            and form != "code mode" else None),
                "prompt": text,
                "promptSha256": sha256_bytes(text.encode()) if text else None,
                "givesDescriptorPath": None if encrypted else bool(descriptor) and descriptor in text,
                "namesVariable": None if encrypted else DESCRIPTOR_VARIABLE in text,
                "obligations": [line.split(".", 1)[0] for line in obligations if line in text],
                "obligationCount": len(obligations),
                "endingLines": [line.strip()[:COMMAND_CHARS] for line in text.splitlines()
                                if any(term in line for term in ENDING_TERMS)],
                "historyHoldsDescriptor": bool(descriptor) and any(descriptor in json.dumps(m) for m in history),
                "meta": {key: source.get(key) for key in ("depth", "agent_path", "agent_role") if key in source},
                "sandbox": {key: turn.get(key) for key in ("sandbox_policy", "approval_policy", "cwd") if key in turn},
                "status": "started",
                "messages": {"toDelegate": sum(target in names for target in sent), "fromDelegate": len(replies)},
                "result": final[:DELEGATE_RESULT_CHARS],
                "transcript": transcript["file"] if transcript else None,
                "order": delegate_order(transcript["events"], mandatory) if transcript else None,
                "mechanism": self.delegate_mechanism,
                "inherited": [name for name in ENDING_NAMES if name in child],
            })
        return found

    @staticmethod
    def script(command: Any) -> str:
        """The shell text of a Codex command: the script of `SHELL -lc
        SCRIPT`, or else the argument vector as one shell line."""
        import shlex

        if isinstance(command, str):
            try:
                command = shlex.split(command)
            except ValueError:
                return command
        argv = [str(part) for part in command or []]
        if len(argv) == 3 and Path(argv[0]).name in SHELLS and argv[1] in ("-lc", "-c"):
            return argv[2]
        return shlex.join(argv)

    @staticmethod
    def url_path(value: Any) -> str | None:
        """A path from Codex's `file://` URL or plain path."""
        from urllib.parse import unquote, urlparse

        if not isinstance(value, str) or not value:
            return None
        return unquote(urlparse(value).path) if value.startswith("file://") else value

    @staticmethod
    def text(item: dict[str, Any]) -> str:
        if isinstance(item.get("text"), str):
            return item["text"]
        content = item.get("content")
        if isinstance(content, list):
            return "".join(part.get("text", "") for part in content if isinstance(part, dict))
        return content if isinstance(content, str) else ""

    def item(self, item: dict[str, Any], number: int) -> list[dict[str, Any]]:
        """One completed Codex item as the entries of a Claude Code
        transcript: a text block, a user turn, or a tool call with its result.
        The session file names item types in CamelCase and the stream in
        snake_case."""
        kind = re.sub(r"[_\s]", "", str(item.get("type", ""))).lower()
        if kind == "reasoning":
            return []
        if kind == "contextcompaction":
            return [{"type": "system", "subtype": "compact_boundary"}]
        if kind == "usermessage":
            return [{"type": "user", "message": {"role": "user", "content": self.text(item)}}]
        if kind == "agentmessage":
            return [{"type": "assistant", "message": {"content": [{"type": "text", "text": self.text(item)}]}}]
        tid = f"codex_{item.get('id') or number}"
        cwd = None
        result = ""
        error = item.get("status") not in ("completed", None)
        if kind == "commandexecution":
            name, tool_input = "exec_command", {"command": self.script(item.get("command"))}
            cwd = self.url_path(item.get("cwd"))
            error = error or item.get("exit_code") not in (0, None)
            result = item.get("aggregated_output") or ""
        elif kind == "filechange":
            changes = item.get("changes")
            paths = list(changes) if isinstance(changes, dict) else [
                change.get("path") for change in changes or [] if isinstance(change, dict) and change.get("path")]
            name, tool_input = "apply_patch", {"paths": paths}
        elif kind == "mcptoolcall":
            # A call to a tool of an MCP server, named as Claude Code names
            # it. A failed call has another status or a result with
            # `isError`.
            name = f"mcp__{item.get('server')}__{item.get('tool')}"
            tool_input = item.get("arguments") if isinstance(item.get("arguments"), dict) else {}
            outcome = item.get("result") if isinstance(item.get("result"), dict) else {}
            error = error or bool(outcome.get("isError")) or bool(item.get("error"))
            result = result_text(outcome.get("content")) or str(item.get("error") or "")
        elif kind == "subagentactivity" and item.get("kind") == "started":
            # The start of a delegate. Its `id` is the call ID of the
            # parent's `spawn_agent` call.
            name, tool_input = "spawn_agent", {"description": item.get("agent_path") or ""}
        elif kind == "subagentactivity":
            name, tool_input = f"subagent:{item.get('kind')}", {"description": item.get("agent_path") or ""}
        elif kind.startswith("collab"):
            # A collaboration call other than the start, such as `wait`. The
            # start is the `SubAgentActivity` item, so no collaboration item
            # counts as a delegation.
            tool = str(item.get("tool") or "")
            name, tool_input = f"collab:{tool}", {"description": item.get("prompt") or "", "tool": tool}
        else:
            name, tool_input = str(item.get("type")), {key: None for key in item if key not in ("id", "type")}
        call: dict[str, Any] = {"type": "assistant", "message": {"content": [
            {"type": "tool_use", "id": tid, "name": name, "input": tool_input}]}}
        if cwd:
            call["cwd"] = cwd
        return [call, {"type": "user", "message": {"content": [
            {"type": "tool_result", "tool_use_id": tid, "is_error": error, "content": result}]}}]

    def entries(self, records: list[dict[str, Any]]) -> list[dict[str, Any]]:
        """The completed items of a Codex session file or stream, in order,
        as transcript entries. Each nested command of the `exec` tool is an
        item of its own, with its argument vector and its working directory."""
        entries: list[dict[str, Any]] = []
        for record in records:
            item = None
            payload = record.get("payload") if isinstance(record.get("payload"), dict) else {}
            if record.get("type") == "event_msg" and payload.get("type") == "item_completed":
                item = payload.get("item")
            elif record.get("type") == "item.completed":
                item = record.get("item")
            elif record.get("method") == "item/completed":
                item = (record.get("params") or {}).get("item")
            if isinstance(item, dict):
                entries += self.item(item, len(entries))
        return entries

    @staticmethod
    def skills(text: str) -> dict[str, Any]:
        """The skill roots and the listed skills of Codex's skills instructions."""
        roots = dict(re.findall(r"^- `(r\d+)` = `([^`]+)`$", text, re.MULTILINE))
        # A name can hold a plugin namespace, as `grove:grove-planning`. The
        # name ends at the first ": ", because a name holds no space.
        listed = re.findall(r"^- ([^\s:][^\s]*?): .*\(file: ([^)]+)\)$", text, re.MULTILINE)
        return {"roots": roots, "skillCount": len(listed), "names": [name for name, _ in listed],
                "files": [file for _, file in listed]}

    def input(self, items: list[dict[str, Any]]) -> dict[str, Any]:
        """What Codex itself puts before the first prompt: its tools, its
        developer messages, its skill listing and its environment context.
        Each developer message shows by its first line, its length and its
        digest."""
        tools: list[str] = []
        developer = []
        skills = None
        environment = None
        for item in items:
            if item.get("type") == "additional_tools":
                for space in item.get("tools", []):
                    tools += [f"{space.get('name')}.{tool.get('name')}" for tool in space.get("tools", [])]
            if item.get("type") != "message":
                continue
            text = "".join(part.get("text", "") for part in item.get("content", []) if isinstance(part, dict))
            if item.get("role") == "developer":
                # One developer message can join several sections, each a tag.
                developer.append({"firstLine": text.splitlines()[0][:COMMAND_CHARS] if text else "",
                                  "sections": list(dict.fromkeys(re.findall(r"</([a-z_][a-z_ ]*)>", text))),
                                  "chars": len(text), "sha256": sha256_bytes(text.encode())})
                if text.startswith("<skills_instructions>"):
                    skills = self.skills(text)
            elif item.get("role") == "user" and text.startswith("<environment_context>") and environment is None:
                environment = text
        return {"tools": tools, "developerMessages": developer, "skillListing": skills,
                "environmentContext": environment}

    def startup(self, records: list[dict[str, Any]]) -> dict[str, Any]:
        """What Codex itself offered at startup, from its session file,
        without the account and user identifiers of its metadata."""
        meta = next((r.get("payload") or {} for r in records if r.get("type") == "session_meta"), {})
        turn = next((r.get("payload") or {} for r in records if r.get("type") == "turn_context"), {})
        init: dict[str, Any] = {key: meta[key] for key in CODEX_META_KEYS if key in meta}
        if isinstance(meta.get("base_instructions"), (str, dict)):
            base = meta["base_instructions"]
            text = json.dumps(base) if isinstance(base, dict) else base
            init["baseInstructions"] = {"chars": len(text), "sha256": sha256_bytes(text.encode())}
        init.update({key: turn[key] for key in CODEX_TURN_KEYS if key in turn})
        items = [r["payload"] for r in records if r.get("type") == "response_item" and isinstance(r.get("payload"), dict)]
        return {"init": init, **self.input(items)}

    def render_rows(self, h: dict[str, Any]) -> list[str]:
        return [
            f"| Configuration directory | sandbox `.codex`, as `{CODEX_HOME}`; the skill roots of "
            f"`codex debug prompt-input` name its `skills/.system` |",
            f"| Codex sandbox | `{h['sandboxMode']}`, approval `{h['approvalPolicy']}` |",
            f"| `{CODEX_HOME}` | {cell(h['codexHome'])} |",
            f"| Sign-in | `codex login status`: {cell(h['loginStatus'])} |",
        ]

    def compaction_mechanism(self, mode: str) -> str:
        return ("codex app-server in Ravel's managed process; the feeder sends the prompt, then "
                f"thread/compact/start, then the answer. Compaction: {CODEX_COMPACTION[mode][1]}")

    def compact_turn(self, spec: dict[str, Any]) -> dict[str, Any]:
        return {"compact": True}

    def feed(self, argv: list[str], prompt: str, turns: list[dict[str, Any]],
             gate: Callable[[str | None, str], bool], log: Callable[[str, dict[str, Any]], None]) -> int:
        """`codex app-server` over stdio: JSON-RPC, one message to a line.
        The methods are those of `codex app-server generate-json-schema` of
        0.166.0. The feeder starts a thread in its working directory, then
        sends each turn after `turn/completed` of that thread. A compaction is
        `thread/compact/start`, which runs as a turn of its own. The feeder
        refuses each request of the server, because the approval policy is
        `never`. The server exits when its input closes."""
        child = subprocess.Popen(argv, stdin=subprocess.PIPE, stdout=subprocess.PIPE)
        assert child.stdin is not None and child.stdout is not None
        numbers = iter(range(1, 1 << 30))

        def write(message: dict[str, Any]) -> None:
            if not child.stdin.closed:
                child.stdin.write((json.dumps({"jsonrpc": "2.0", **message}) + "\n").encode())
                child.stdin.flush()

        def request(method: str, params: dict[str, Any]) -> int:
            number = next(numbers)
            write({"id": number, "method": method, "params": params})
            return number

        def close() -> None:
            if not child.stdin.closed:
                child.stdin.close()
                # A server that outlives its input is ended, so the feeder ends.
                timer = threading.Timer(60, child.kill)
                timer.daemon = True
                timer.start()

        def send(turn: dict[str, Any], number: int) -> None:
            if turn.get("compact"):
                request("thread/compact/start", {"threadId": thread})
                log("turn", {"turn": number, "request": "thread/compact/start"})
                return
            request("turn/start", {"threadId": thread, "input": [{"type": "text", "text": turn["text"]}]})
            log("turn", {"turn": number, "sha256": sha256_bytes(turn["text"].encode())})

        initialize = request("initialize", {"clientInfo": {"name": CODEX_CLIENT, "version": "1"}})
        started = None
        thread = None
        sent = 0
        for line in child.stdout:
            sys.stdout.buffer.write(line)
            sys.stdout.buffer.flush()
            try:
                message = json.loads(line)
            except json.JSONDecodeError:
                continue
            if not isinstance(message, dict):
                continue
            if "method" in message and "id" in message:
                write({"id": message["id"], "error": {"code": -32601, "message": "the trial feeder answers no request"}})
                log("request", {"method": message["method"]})
                continue
            if "error" in message and "id" in message:
                log("error", {"id": message["id"], "error": message["error"]})
                turns.clear()
                close()
                continue
            if message.get("id") == initialize:
                write({"method": "initialized"})
                started = request("thread/start", {"cwd": os.getcwd()})
                continue
            if message.get("id") == started and started is not None:
                thread = ((message.get("result") or {}).get("thread") or {}).get("id")
                send({"text": prompt}, sent)
                sent += 1
                continue
            if message.get("method") != "turn/completed" or (message.get("params") or {}).get("threadId") != thread:
                continue
            if not turns:
                close()
                continue
            turn = turns.pop(0)
            if turn.get("gate") and not gate(thread, turn["gate"]):
                turns.clear()
                close()
                continue
            send(turn, sent)
            sent += 1
        close()
        return child.wait()

    @staticmethod
    def compaction_index(records: list[dict[str, Any]]) -> int | None:
        """The index of the last `compacted` entry of a session file, or None."""
        found = [n for n, r in enumerate(records) if r.get("type") == "compacted"]
        return found[-1] if found else None

    @staticmethod
    def model_items(records: list[dict[str, Any]], start: int, answer: bool) -> list[dict[str, Any]]:
        """The model-visible items after the compaction at `start`. Codex
        installs the `replacement_history` of the `compacted` entry, and the
        messages that it records later follow. Without `answer`, the items
        stop at the next turn, as the gate sees them. With `answer`, they run
        through the next turn's messages up to the model's first output: the
        input of the first request after compaction."""
        items = list((records[start].get("payload") or {}).get("replacement_history") or [])
        for record in records[start + 1:]:
            payload = record.get("payload") if isinstance(record.get("payload"), dict) else {}
            if record.get("type") == "event_msg" and payload.get("type") == "task_started":
                if not answer:
                    break
                continue
            if record.get("type") != "response_item":
                continue
            if payload.get("type") == "message" and payload.get("role") in ("developer", "user"):
                items.append(payload)
                continue
            break
        return items

    def findings(self, items: list[dict[str, Any]], marks: dict[str, Any]) -> dict[str, Any]:
        """The preamble text that model-visible items hold. The terms are
        searched in user messages only. An encrypted item, such as the item of
        remote compaction, is opaque, and the findings count it."""
        visible = [{key: value for key, value in item.items() if key != "encrypted_content"} for item in items]
        words = "\n".join(self.text(item) for item in items
                          if item.get("type") == "message" and item.get("role") == "user")
        found = context_findings("\n".join(strings(visible)), words, marks)
        found["opaque"] = sum(bool(item.get("encrypted_content")) for item in items)
        return found

    def history_item(self, item: dict[str, Any], marks: dict[str, Any]) -> dict[str, Any]:
        """One model-visible item, by its type, its role, its first line, its
        length and its digest, and whether it holds preamble text."""
        text = self.text(item) if item.get("type") == "message" else ""
        entry: dict[str, Any] = {"type": item.get("type"), **({"role": item["role"]} if item.get("role") else {})}
        if item.get("encrypted_content"):
            entry["encryptedChars"] = len(str(item["encrypted_content"]))
        elif item.get("type") == "additional_tools":
            entry["tools"] = sum(len(space.get("tools", [])) for space in item.get("tools", []))
        else:
            entry.update({"firstLine": text.splitlines()[0][:COMMAND_CHARS] if text else "", "chars": len(text),
                          "sha256": sha256_bytes(text.encode())})
        found = context_findings(text, text, marks)
        entry["preamble"] = bool(found["paths"] or found["digests"] or found["lines"])
        return entry

    def after_compaction(self, records: list[dict[str, Any]], marks: dict[str, Any]) -> dict[str, Any] | None:
        start = self.compaction_index(records)
        return None if start is None else self.findings(self.model_items(records, start, False), marks)

    def compaction_facts(self, records: list[dict[str, Any]], marks: dict[str, Any],
                         mode: str) -> tuple[dict[str, Any], list[str]]:
        """The replacement history of the last compaction, and the input of
        the first model request after it. Codex records no request, so that
        input is the history and the messages that Codex records before the
        model's first output."""
        start = self.compaction_index(records)
        if start is None:
            return {}, ["the session holds no compaction boundary"]
        payload = records[start].get("payload") or {}
        history = payload.get("replacement_history") or []
        context = self.model_items(records, start, True)
        facts = {
            "window": payload.get("window_number"),
            "history": [self.history_item(item, marks) for item in history],
            "afterBoundary": self.after_compaction(records, marks),
            "context": {"items": len(context), "added": [self.history_item(item, marks) for item in context[len(history):]],
                        "findings": self.findings(context, marks)},
        }
        failures = [f"after the boundary: {problem}" for problem in compaction_problems(facts["afterBoundary"], mode)]
        failures += [f"model context: {problem}"
                     for problem in compaction_problems(facts["context"]["findings"], mode)]
        return facts, failures

    @staticmethod
    def render_item(item: dict[str, Any]) -> str:
        head = f"`{item.get('type')}`" + (f" `{item['role']}`" if item.get("role") else "")
        if "encryptedChars" in item:
            body = f", encrypted, {item['encryptedChars']} characters"
        elif "tools" in item:
            body = f", {item['tools']} tools"
        else:
            body = f", {item['chars']} characters, SHA-256 `{item['sha256'][:12]}…`: `{cell(item['firstLine'])}`"
        return head + body + (". Holds preamble text." if item.get("preamble") else ".")

    def render_compaction(self, compaction: dict[str, Any]) -> list[str]:
        lines = [f"- Summary instructions of the case: `{cell(compaction['instructions'])}`. They do not reach "
                 f"Codex, because `thread/compact/start` takes only the thread."]
        if "history" in compaction:
            lines.append(f"- Replacement history of the `compacted` entry, window {compaction.get('window')}: "
                         f"{len(compaction['history'])} items.")
            lines += ["  - " + self.render_item(item) for item in compaction["history"]]
        lines += render_findings("Session lines after the boundary", compaction.get("afterBoundary"))
        context = compaction.get("context")
        if context:
            lines.append(f"- Input of the first model request after the boundary: the replacement history, then "
                         f"{len(context['added'])} items that Codex recorded before the model's first output.")
            lines += ["  - " + self.render_item(item) for item in context["added"]]
            lines += render_findings("Model context of that request", context.get("findings"))
        return lines

    def render_prelaunch(self, h: dict[str, Any]) -> list[str]:
        prelaunch = h.get("prelaunch")
        if not prelaunch:
            return []
        return ["Codex input before the launch, from `codex debug prompt-input`, which makes no model call:", "",
                "```json", json.dumps(prelaunch, indent=2), "```", ""]


# The registry: the one place that maps a harness name to its adapter.
CLAUDE = ClaudeAdapter()
CODEX = CodexAdapter()
ADAPTERS: dict[str, HarnessAdapter] = {adapter.name: adapter for adapter in (CLAUDE, CODEX)}
HARNESSES = tuple(ADAPTERS)
# A case or a record without a harness name is of Claude Code, the runner's
# first harness.
DEFAULT_HARNESS = CLAUDE.name


def adapter_named(name: str) -> HarnessAdapter:
    """The adapter of a harness name, such as the name of a record's harness."""
    return ADAPTERS[name]


def adapter_of(case: Mapping[str, Any]) -> HarnessAdapter:
    """The adapter of a case."""
    return adapter_named(case.get("harness", DEFAULT_HARNESS))


def run_adapter(case_name: str, own: Any, requested: str | None) -> tuple[HarnessAdapter, str | None]:
    """The adapter of a run, and the case's own harness when the run uses
    another one. A case names its own harness, and a request runs it in
    another one."""
    if own not in ADAPTERS or (requested is not None and requested not in ADAPTERS):
        raise TrialError(f"case {case_name}: the harness '{requested or own}' is not one of {', '.join(ADAPTERS)}")
    if requested and requested != own:
        return ADAPTERS[requested], own
    return ADAPTERS[own], None


def session_files(home: Path) -> list[Path]:
    """The session files of every harness in the sandbox home."""
    return [path for adapter in ADAPTERS.values() for path in adapter.session_files(home)]


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


def candidate_keg(candidate: Path, commit: str) -> dict[str, Any]:
    """The candidate keg that `task owner:candidate-keg` wrote into the
    directory `candidate`, for the prepared commit: its keg, its skill
    directory and the accepted `resourceDigest` of each source."""
    file = candidate.resolve() / "candidate.json"
    if not file.is_file():
        raise TrialError(f"no {file}; build the candidate keg with `task owner:candidate-keg`")
    record = json.loads(file.read_text())
    if record.get("commit") != commit:
        raise TrialError(f"the candidate keg in {candidate} holds Grove commit {record.get('commit')}, "
                         f"not the prepared commit {commit}; build the keg of the prepared commit")
    keg = Path(record["keg"])
    if not (keg / KEG_SOURCES).is_dir():
        raise TrialError(f"the candidate keg {keg} holds no {KEG_SOURCES}")
    return {"candidate": str(file.parent), "recordSha256": sha256_file(file), "commit": commit,
            "keg": str(keg), "skillDirectory": record["skillDirectory"],
            "digests": {source["id"]: source["resourceDigest"] for source in record["sources"]}}


def prepare(out: Path, commit: str, front: Path, environ: Mapping[str, str], keg: Path | None = None) -> Path:
    out = out.resolve()
    ravel = check_ravel(front)
    full = run_checked(["git", "rev-parse", "--verify", f"{commit}^{{commit}}"], cwd=ROOT)
    kegged = candidate_keg(keg, full) if keg else None
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
        **({"keg": kegged} if kegged else {}),
    }
    (out / "prepared.json").write_text(json.dumps(record, indent=2) + "\n")
    say(f"prepared {out}/prepared.json")
    return out / "prepared.json"


# --- run -----------------------------------------------------------------------


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
    """A plain Git checkout of `commit` at `target`, fetched by its hash. It
    refuses an unknown commit, a `HEAD` other than the commit and a checkout
    that is not clean."""
    run_checked(["git", "init", "--quiet", str(target)], env=env)
    fetched = subprocess.run(["git", "fetch", "--quiet", str(repository), commit], cwd=target, env=env,
                             capture_output=True, text=True)
    if fetched.returncode != 0:
        raise TrialError(f"the repository {repository} gives no commit {commit}; name a published commit "
                         f"of that repository: {fetched.stderr.strip()}")
    run_checked(["git", "checkout", "--quiet", "--detach", "FETCH_HEAD"], cwd=target, env=env)
    problems = checkout_problems(target, commit, env)
    if problems:
        raise TrialError("; ".join(problems))


def checkout_problems(target: Path, commit: str, env: dict[str, str]) -> list[str]:
    """Each way the checkout at `target` is not a clean checkout of `commit`."""
    if not target.is_dir():
        return [f"the checkout {target} does not exist"]
    problems = []
    head = run_checked(["git", "rev-parse", "HEAD"], cwd=target, env=env)
    if head != commit:
        problems.append(f"the checkout {target} has HEAD {head}, not the commit {commit}")
    if run_checked(["git", "status", "--porcelain"], cwd=target, env=env):
        problems.append(f"the checkout {target} is not clean")
    return problems


def external_checkout(source: dict[str, Any], trial: Path, env: dict[str, str]) -> dict[str, Any]:
    """A plain Git checkout of an external source's commit under the trial's
    `sources/`, with the Git tree ID of its source directory. The tree ID
    names the committed files; Ravel's `resourceDigest` later names the
    captured files."""
    external = source["external"]
    target = trial / "sources" / f"{source['id']}-checkout"
    target.parent.mkdir(exist_ok=True)
    fetch_checkout(external["repository"], external["commit"], target, env)
    spec = f"{external['commit']}:{source['plugin']}"
    found = subprocess.run(["git", "rev-parse", "--verify", "--quiet", spec], cwd=target, env=env,
                           capture_output=True, text=True)
    tree = found.stdout.strip()
    if found.returncode != 0 or run_checked(["git", "cat-file", "-t", tree], cwd=target, env=env) != "tree":
        raise TrialError(f"the commit {external['commit']} of {external['repository']} holds no directory "
                         f"{source['plugin']}; name the source directory of the pack in `plugin`")
    return {"id": source["id"], "repository": external["repository"], "commit": external["commit"],
            "plugin": source["plugin"], "checkout": str(target), "treeId": tree}


JJ_FACTS = 'change_id ++ "\\n" ++ commit_id ++ "\\n" ++ if(empty, "true", "false") ++ "\\n"'


def working_copy_facts(directory: Path, environ: Mapping[str, str]) -> dict[str, Any]:
    """The working-copy change of a Jujutsu workspace: its change ID, its
    commit ID and whether it is empty. jj snapshots the working copy first,
    so the commit names the files as they are now."""
    text = run_checked(["jj", "log", "--no-graph", "--color", "never", "-R", str(directory), "-r", "@",
                        "-T", JJ_FACTS], env=dict(environ))
    change, commit, empty = text.splitlines()
    return {"path": str(directory), "changeId": change, "commitId": commit, "empty": empty == "true"}


def external_sources(case: dict[str, Any], trial: Path, working_copies: Mapping[str, Path],
                     environ: Mapping[str, str], keg: Mapping[str, Any] | None = None) -> dict[str, dict[str, Any]]:
    """The external sources of the case, by ID. Each source comes from a
    plain checkout of its commit, or from the working copy that the run
    option `--working-copy` names for it. A keg source comes from the
    candidate keg of the preparation."""
    external_ids = [source["id"] for source in case.get("sources", []) if source.get("external")]
    for source_id in working_copies:
        if source_id not in external_ids:
            raise TrialError(f"--working-copy names '{source_id}', and the external sources of the case "
                             f"{case['name']} are {external_ids or 'none'}")
    if case.get("moveAway") in working_copies:
        raise TrialError(f"case {case['name']}: moveAway would move the working copy of "
                         f"'{case['moveAway']}'; run the case on its checkout")
    externals = {}
    for source in case.get("sources", []):
        if "keg" in source:
            if keg is None:
                raise TrialError(f"case {case['name']}: the source '{source['id']}' comes from the candidate "
                                 "keg; prepare with --keg, or check with --keg")
            if source["id"] not in keg["digests"]:
                raise TrialError(f"case {case['name']}: the candidate keg {keg['keg']} holds no source "
                                 f"'{source['id']}'")
            externals[source["id"]] = {
                "id": source["id"], "keg": keg["keg"], "commit": keg["commit"], "plugin": source["plugin"],
                "candidate": keg["candidate"], "acceptedDigest": keg["digests"][source["id"]],
            }
            continue
        if not source.get("external"):
            continue
        if source["id"] in working_copies:
            directory = working_copies[source["id"]]
            if not (directory / source["plugin"]).is_dir():
                raise TrialError(f"the working copy {directory} holds no directory {source['plugin']}")
            externals[source["id"]] = {
                "id": source["id"], "repository": source["external"]["repository"],
                "commit": source["external"]["commit"], "plugin": source["plugin"],
                "workingCopy": working_copy_facts(directory, environ),
            }
        else:
            externals[source["id"]] = external_checkout(source, trial, git_env(environ))
    return externals


def external_root(external: dict[str, Any]) -> Path:
    """The directory that holds the source directory of an external source."""
    if "keg" in external:
        return Path(external["keg"])
    return Path(external["workingCopy"]["path"] if "workingCopy" in external else external["checkout"])


def external_failures(externals: dict[str, dict[str, Any]], launches: list[dict[str, Any]],
                      refusal: dict[str, Any] | None, environ: Mapping[str, str]) -> list[str]:
    """Add each external source's `resourceDigest` from Ravel, and check that
    no source changed during the run. Ravel names the digest in the view of
    each launch and in the restored resolution of a refusal case."""
    failures = []
    resolved = [] if not refusal else refusal["restored"].get("sources", [])
    for source_id, external in externals.items():
        found = {source.get("resourceDigest") for launch in launches for source in launch.get("sources", [])
                 if source.get("id") == source_id}
        found |= {source.get("resourceDigest") for source in resolved if source.get("id") == source_id}
        digests = sorted(digest for digest in found if digest)
        external["resourceDigest"] = digests[0] if len(digests) == 1 else None
        if len(digests) > 1:
            external["resourceDigests"] = digests
            failures.append(f"Ravel gave the external source '{source_id}' the digests {digests} in one run")
        elif not digests:
            failures.append(f"Ravel named no resourceDigest of the external source '{source_id}'")
        if "keg" in external:
            # Ravel canonicalises a source path at capture, so each launch
            # names the keg directory itself.
            origins = sorted({source.get("originPath") for launch in launches
                              for source in launch.get("sources", []) if source.get("id") == source_id})
            external["originPaths"] = origins
            inside = str(Path(external["keg"]) / external["plugin"])
            failures += [f"Ravel captured the keg source '{source_id}' from {origin}, not from {inside}"
                         for origin in origins if origin != inside]
            if digests and digests != [external["acceptedDigest"]]:
                failures.append(f"Ravel gave the keg source '{source_id}' the digest {digests[0]}, and the "
                                f"candidate keg accepted {external['acceptedDigest']}")
        elif "workingCopy" in external:
            after = working_copy_facts(Path(external["workingCopy"]["path"]), environ)
            external["workingCopy"]["commitAfter"] = after["commitId"]
            if after["commitId"] != external["workingCopy"]["commitId"]:
                failures.append(f"the working copy {after['path']} changed during the run, from commit "
                                f"{external['workingCopy']['commitId']} to {after['commitId']}")
        else:
            problems = checkout_problems(Path(external["checkout"]), external["commit"], git_env(environ))
            external["cleanAfter"] = not problems
            failures += [f"after the run, {problem}" for problem in problems]
    return failures


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
    into the sandbox `.claude/skills` or `.codex/skills`, never into the
    owner's home. With a `commit`, the copy holds the directory as that older
    commit held it, such as a stale native copy beside the current directory
    copy."""
    installed = []
    for item in case.get("native", []):
        target = home / adapter_of(case).native_root / item["name"]
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
    root = adapter_of(case).native_root
    return sorted(f"{root}/{item['name']}/SKILL.md" for item in case.get("native", []))


def plugin_command(program: Path, args: list[str], env: dict[str, str], tmp: Path) -> dict[str, Any]:
    """One `claude plugin` command with `--json`. Its last line of standard
    output is the result, and an outcome other than "ok" refuses the run
    before the driver."""
    done = subprocess.run([str(program), "plugin", *args], cwd=tmp, env=env, capture_output=True, text=True)
    lines = done.stdout.strip().splitlines()
    try:
        result = json.loads(lines[-1]) if lines else {}
    except json.JSONDecodeError:
        result = {}
    if done.returncode != 0 or result.get("outcome") != "ok":
        shown = " ".join(a for a in args if a != "--json")
        raise TrialError(f"plugin {shown} failed with exit {done.returncode}: "
                         f"{result.get('message') or done.stderr.strip() or done.stdout.strip()}")
    return result


def installed_plugins(home: Path, listed: list[dict[str, Any]]) -> list[dict[str, Any]]:
    """Each installed plugin, as `claude plugin list --json` lists it, with
    the Git commit and the dependency flag of Claude Code's own
    `installed_plugins.json`. `auto` marks a plugin that installed as a
    dependency of another. The skills are the plugin's SKILL.md files, in
    `skills/` and in each directory that the manifest's `skills` field
    adds, such as `craft/`. Each skill names its root."""
    path = home / ".claude" / "plugins" / "installed_plugins.json"
    own = json.loads(path.read_text()).get("plugins", {}) if path.is_file() else {}
    found = []
    for item in sorted(listed, key=lambda item: item["id"]):
        entry = (own.get(item["id"]) or [{}])[0]
        install = Path(os.path.realpath(item["installPath"]))
        skills = sorted({skill for root in plugin_skill_roots(install)
                         for skill in install.glob(f"{root}/*/SKILL.md")})
        found.append({
            "id": item["id"], "version": item.get("version"), "enabled": item.get("enabled"),
            "gitCommitSha": entry.get("gitCommitSha"), "dependency": bool(entry.get("auto")),
            "installPath": str(install.relative_to(os.path.realpath(home))),
            "skills": [{"root": str(skill.parent.parent.relative_to(install)), "name": skill.parent.name,
                        "sha256": sha256_file(skill)} for skill in skills],
        })
    return found


def plugin_skill_roots(install: Path) -> list[str]:
    """The skill directories of an installed plugin: `skills`, then each
    relative directory that the manifest lists in `skills`. Claude Code adds
    a listed directory to the default scan of `skills/`."""
    roots = ["skills"]
    manifest = install / ".claude-plugin" / "plugin.json"
    listed = json.loads(manifest.read_text()).get("skills", []) if manifest.is_file() else []
    for item in [listed] if isinstance(listed, str) else listed:
        root = PurePosixPath(item) if isinstance(item, str) else None
        if root is not None and not root.is_absolute() and ".." not in root.parts and str(root) not in roots:
            roots.append(str(root))
    return roots


def plugin_failures(found: list[str], plugins: dict[str, Any] | None) -> list[str]:
    """The plugin copies of the home must be the skills of the installed
    plugins. Without installed plugins, the home holds no plugin copy."""
    expected = sorted(f"{plugin['installPath']}/{skill.get('root', 'skills')}/{skill['name']}/SKILL.md"
                      for plugin in (plugins or {}).get("installed", []) for skill in plugin["skills"])
    if found == expected:
        return []
    return [f"the sandbox home holds the plugin copies {found}, and the installed plugins hold {expected}"]


def native_failures(native: list[str], expected: list[str]) -> list[str]:
    if native == expected:
        return []
    return [f"the sandbox home holds the native copies {native}, and the case declares {expected}"]


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


# --- the supplier check ---------------------------------------------------------
#
# The acceptance record cites each case by the build that it measured, and a
# later Grove commit can remove a path that a case requires. Before any
# launch, the check binds each required path to its one supplier at its
# revision, as "The trial cases that name moving skills" of
# docs/specs/pack-membership.md states, and refuses a path that the supplier
# does not hold.

SUPPLY_NOTE = ("The case measures a build whose supplier held this path; run it against the Grove commit "
               "that its record names")


class Supplier:
    """One holder of required paths: a commit of a Git repository, or the
    files of a working copy as they are now. A refusal names `name` and
    `revision`."""

    def __init__(self, name: str, revision: str, repository: Path | None = None, directory: Path | None = None):
        self.name, self.revision, self.repository, self.directory = name, revision, repository, directory

    def holds(self, path: str, directory: bool = False) -> bool:
        """Whether the supplier holds `path` as a directory, or as a file."""
        if self.directory is not None:
            target = self.directory / path
            return target.is_dir() if directory else target.is_file()
        found = subprocess.run(["git", "cat-file", "-t", f"{self.revision}:{path}"], cwd=self.repository,
                               env=git_env(os.environ), capture_output=True, text=True)
        return found.returncode == 0 and found.stdout.strip() == ("tree" if directory else "blob")

    def read(self, path: str) -> str | None:
        """The text of the file `path`, or None when the supplier holds no
        such file."""
        if not self.holds(path):
            return None
        if self.directory is not None:
            return (self.directory / path).read_text()
        return run_checked(["git", "show", f"{self.revision}:{path}"], cwd=self.repository, env=git_env(os.environ))


def unsupplied(case: dict[str, Any], supplier: Supplier, path: str, role: str) -> str:
    return f"case {case['name']}: {supplier.name} {supplier.revision} holds no {path}, {role}. {SUPPLY_NOTE}"


def source_suppliers(case: dict[str, Any], prepared: Supplier,
                     externals: Mapping[str, dict[str, Any]]) -> dict[str, Supplier]:
    """The supplier of each source, by ID. An external source comes from its
    checkout at its commit, or from the working copy of `--working-copy`.
    Every other source comes from the prepared commit."""
    suppliers = {}
    for source in case.get("sources", []):
        external = externals.get(source["id"])
        if external is None:
            suppliers[source["id"]] = prepared
        elif "keg" in external:
            suppliers[source["id"]] = Supplier(f"the candidate keg {external['keg']} of Grove commit",
                                               external["commit"], directory=Path(external["keg"]))
        elif "workingCopy" in external:
            copy = external["workingCopy"]
            suppliers[source["id"]] = Supplier(f"the working copy of the external source '{source['id']}' at",
                                               copy["commitId"], directory=Path(copy["path"]))
        else:
            suppliers[source["id"]] = Supplier(
                f"the external source '{source['id']}' of {external['repository']} at commit", external["commit"],
                repository=Path(external["checkout"]))
    return suppliers


def native_supplier(item: dict[str, Any], prepared: Supplier) -> Supplier | str:
    """The supplier of a native copy. A copy with a `commit` comes from that
    commit, which must lie in the history of the prepared commit; the current
    directory copy beside it is a path of its own source. A refusal is
    returned as its text."""
    if not item.get("commit"):
        return prepared
    env = git_env(os.environ)
    resolved = subprocess.run(["git", "rev-parse", "--verify", "--quiet", f"{item['commit']}^{{commit}}"],
                              cwd=prepared.repository, env=env, capture_output=True, text=True).stdout.strip()
    inside = bool(resolved) and subprocess.run(
        ["git", "merge-base", "--is-ancestor", resolved, prepared.revision], cwd=prepared.repository, env=env,
        capture_output=True).returncode == 0
    if not inside:
        return (f"the commit {item['commit']} of the native copy '{item['name']}' is not in the history of "
                f"the prepared Grove commit {prepared.revision}")
    return Supplier("the prepared checkout's commit", resolved, repository=prepared.repository)


def plugin_supplier(plugin: str, prepared: Supplier, scratch: Path) -> tuple[Supplier, str] | str:
    """The supplier of a native plugin and the plugin's directory in it, as
    the marketplace of the prepared commit names it. A relative entry is a
    directory of the prepared commit. A `git-subdir` entry is its repository
    at its `sha`, fetched under `scratch`. A refusal is returned as its
    text."""
    name, market = plugin.split("@")
    manifest = ".claude-plugin/marketplace.json"
    shown = subprocess.run(["git", "show", f"{prepared.revision}:{manifest}"], cwd=prepared.repository,
                           env=git_env(os.environ), capture_output=True, text=True)
    if shown.returncode != 0:
        return f"{prepared.name} {prepared.revision} holds no {manifest}, which names the plugin {plugin}"
    marketplace = json.loads(shown.stdout)
    entry = next((item for item in marketplace.get("plugins", []) if item.get("name") == name), None)
    if marketplace.get("name") != market or entry is None:
        return (f"the marketplace '{marketplace.get('name')}' of {prepared.name} {prepared.revision} has no "
                f"entry for the plugin {plugin}")
    source = entry.get("source")
    if isinstance(source, str):
        return prepared, str(PurePosixPath(source))
    if (isinstance(source, dict) and source.get("source") == "git-subdir"
            and FULL_COMMIT.match(str(source.get("sha", ""))) and source.get("url") and source.get("path")):
        target = scratch / f"plugin-{name}-checkout"
        if not target.exists():
            target.parent.mkdir(parents=True, exist_ok=True)
            try:
                fetch_checkout(source["url"], source["sha"], target, git_env(os.environ))
            except TrialError as error:
                return f"the marketplace entry of the plugin {plugin} pins no fetchable commit: {error}"
        supplier = Supplier(f"the checkout of the plugin '{name}' that the marketplace pins, {source['url']} at commit",
                            source["sha"], repository=target)
        return supplier, str(PurePosixPath(source["path"]))
    return f"the marketplace entry of the plugin {plugin} has a source that the check cannot read: {source!r}"


def native_plugins(plugins: list[str], prepared: Supplier, scratch: Path):
    """Each native plugin and the plugin's supplier and directory, or the
    text of a refusal. After each plugin come the plugins that its manifest
    declares in `dependencies`, from the same marketplace, as Claude Code
    installs them with it."""
    seen: set[str] = set()
    queue = list(plugins)
    while queue:
        ident = queue.pop(0)
        if ident in seen:
            continue
        seen.add(ident)
        found = plugin_supplier(ident, prepared, scratch)
        yield ident, found
        if isinstance(found, str):
            continue
        manifest = found[0].read(str(PurePosixPath(found[1], ".claude-plugin/plugin.json")))
        market = ident.split("@")[1]
        for dependency in (json.loads(manifest).get("dependencies", []) if manifest else []):
            if isinstance(dependency, str):
                queue.append(dependency if "@" in dependency else f"{dependency}@{market}")


def kind_candidates(case: dict[str, Any], skill: str, suppliers: Mapping[str, Supplier],
                    natives: list[tuple[dict[str, Any], Supplier]], prepared: Supplier, scratch: Path):
    """Each place that can supply the kind skill `skill`, in order: each root
    of each source on the directory route, each native copy of that name and
    each native plugin with its dependencies. Each item is a supplier and a
    path, or the text of a plugin that names no supplier."""
    if case["route"] == "directory-2.0":
        for source in case.get("sources", []):
            for root in source["roots"]:
                yield suppliers[source["id"]], str(PurePosixPath(source["plugin"], root, skill, "SKILL.md"))
    for item, supplier in natives:
        if item["name"] == skill:
            yield supplier, str(PurePosixPath(item["from"], "SKILL.md"))
    for _, found in native_plugins(case.get("nativePlugins", []), prepared, scratch):
        if isinstance(found, str):
            yield found
        else:
            yield found[0], str(PurePosixPath(found[1], "skills", skill, "SKILL.md"))


def supply_problems(case: dict[str, Any], checkout: Path, commit: str,
                    externals: Mapping[str, dict[str, Any]], scratch: Path) -> tuple[int, list[str]]:
    """The supplier check: the number of required paths that their suppliers
    hold, and a refusal for each one that its supplier does not hold. It
    reads Git objects and files only, and launches nothing."""
    prepared = Supplier("the prepared Grove commit", commit, repository=checkout)
    suppliers = source_suppliers(case, prepared, externals)
    plugins = {source["id"]: source["plugin"] for source in case.get("sources", [])}
    required: list[tuple[Supplier, str, bool, str]] = []
    problems = []

    def in_source(item: dict[str, Any], role: str) -> None:
        if item["source"] not in suppliers:
            problems.append(f"case {case['name']}: {role} {item['path']} names the source '{item['source']}', "
                            "which is not one of the case's sources")
            return
        required.append((suppliers[item["source"]], str(PurePosixPath(plugins[item["source"]], item["path"])),
                         False, role))

    for source in case.get("sources", []):
        supplier = suppliers[source["id"]]
        required.append((supplier, source["plugin"], True, f"the directory of the source '{source['id']}'"))
        for root in source["roots"]:
            required.append((supplier, str(PurePosixPath(source["plugin"], root)), True,
                             f"the root '{root}' of the source '{source['id']}'"))
    continuation = case.get("continuation") or {}
    for item in [*case.get("mandatory", []), *continuation.get("mandatory", [])]:
        in_source(item, "a mandatory file")
    for item in case.get("requires", []):
        if "plugin" not in item:
            in_source(item, "a skill that the task relies on")
            continue
        found = plugin_supplier(item["plugin"], prepared, scratch)
        if isinstance(found, str):
            problems.append(f"case {case['name']}: {found}. {SUPPLY_NOTE}")
        else:
            required.append((found[0], str(PurePosixPath(found[1], item["path"])), False,
                             f"a skill of the native plugin {item['plugin']} that the task relies on"))
    for item in case.get("plant", []):
        required.append((prepared, item["from"], False, "a planted file"))
    for edit in continuation.get("edits", []):
        required.append((prepared, edit["path"], False, "a file that the continuation edits"))
    natives = []
    for item in case.get("native", []):
        supplier = native_supplier(item, prepared)
        if isinstance(supplier, str):
            problems.append(f"case {case['name']}: {supplier}. {SUPPLY_NOTE}")
            continue
        natives.append((item, supplier))
        required.append((supplier, str(PurePosixPath(item["from"], "SKILL.md")), False,
                         f"the native copy '{item['name']}'"))

    held = 0
    # A path that two fields name, such as a mandatory file of a continuation,
    # is one requirement.
    required = list({(supplier.name, supplier.revision, path, directory): (supplier, path, directory, role)
                     for supplier, path, directory, role in required}.values())
    for supplier, path, directory, role in required:
        if supplier.holds(path, directory):
            held += 1
        else:
            problems.append(unsupplied(case, supplier, path, role))
    absent = set(case.get("absent", []))
    for kind in case.get("kinds", []):
        skill = f"grove-{kind}"
        if skill in absent:
            continue
        tried = []
        for candidate in kind_candidates(case, skill, suppliers, natives, prepared, scratch):
            if isinstance(candidate, str):
                tried.append(candidate)
            elif candidate[0].holds(candidate[1]):
                held += 1
                break
            else:
                tried.append(unsupplied(case, candidate[0], candidate[1], f"the kind skill of {kind}"))
        else:
            if not tried:
                tried.append(f"case {case['name']}: no source, native copy or native plugin can supply {skill}, "
                             f"the kind skill of {kind}. {SUPPLY_NOTE}")
            problems += [line if line.startswith("case ") else f"case {case['name']}: {line}. {SUPPLY_NOTE}"
                         for line in tried]
    # A plugin that names no supplier refuses once, also when a declared
    # skill and a kind skill both need it.
    return held, list(dict.fromkeys(problems))


def refuse_unsupplied(case: dict[str, Any], checkout: Path, commit: str,
                      externals: Mapping[str, dict[str, Any]], scratch: Path) -> int:
    """Refuse the run before any launch when a supplier lacks a required
    path. It returns the number of held paths."""
    held, problems = supply_problems(case, checkout, commit, externals, scratch)
    if problems:
        raise TrialError("\n".join(problems))
    return held


def check_cases(names: list[str], commit: str, repository: Path | None, environ: Mapping[str, str],
                candidate: Path | None = None) -> int:
    """The supplier check of each named case, or of every case, against a
    commit of this repository or of `repository`. It fetches the commit into
    a plain Git checkout, as `prepare` does, and each external source as a
    run does. A keg source needs the candidate keg of the same commit. It
    builds nothing, and it launches no harness and no driver."""
    origin = str(repository) if repository else run_checked(
        ["git", "rev-parse", "--path-format=absolute", "--git-common-dir"], cwd=ROOT)
    full = run_checked(["git", "rev-parse", "--verify", f"{commit}^{{commit}}"], cwd=origin)
    keg = candidate_keg(candidate, full) if candidate else None
    names = names or sorted(path.name for path in CASES.iterdir() if (path / "case.json").is_file())
    failed = 0
    with tempfile.TemporaryDirectory(prefix="grove-trial-check-") as directory:
        scratch = Path(directory)
        checkout = scratch / "grove-source"
        fetch_checkout(origin, full, checkout, git_env(environ))
        for name in names:
            try:
                case, _ = load_case(name)
                trial = scratch / "cases" / name
                trial.mkdir(parents=True)
                externals = external_sources(case, trial, {}, environ, keg)
                held, problems = supply_problems(case, checkout, full, externals, trial / "suppliers")
            except TrialError as error:
                held, problems = 0, [f"case {name}: {error}"]
            if problems:
                failed += 1
                for problem in problems:
                    print(f"harness-trial: {problem}", file=sys.stderr)
            else:
                say(f"case {name}: each required path is held ({held} paths) at Grove commit {full}")
    say(f"checked {len(names)} cases against Grove commit {full}: {len(names) - failed} held, {failed} refused")
    return 1 if failed else 0


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


def read_window(record: Path) -> tuple[int, int] | None:
    """The lines of the session file that belong to this launch. A resumed
    turn appends to the session file of the first turn, so a continuation
    case writes `lines` into each turn's record."""
    file = record / "lines"
    if not file.exists():
        return None
    start, end = (int(part) for part in file.read_text().split())
    return start, end


def describe_launch(adapter: HarnessAdapter, record: Path, home: Path, repo: Path, live: Mapping[str, str],
                    marker: str | None = None, turns: bool = False) -> dict[str, Any]:
    """One launch of a lifecycle case, from its record directory."""
    stream_file = record / "stream.jsonl"
    prompt = record / "prompt"
    return adapter.describe(
        str(record), json.loads((record / "view.json").read_text()), parse_env((record / "env").read_text()),
        read_jsonl(stream_file) if stream_file.exists() else [],
        sha256_file(stream_file) if stream_file.exists() else None,
        home, repo, live, marker, read_window(record), turns,
        prompt.read_text() if prompt.exists() else None,
    )


def split_log(text: str) -> list[dict[str, Any]]:
    """The launches of a standalone run, from Grove's run log. The
    wrapper begins each launch with its `trial-view-b64` line. The JSON lines
    after it are the stream of the harness, and other lines stay as text."""
    import base64

    launches: list[dict[str, Any]] = []
    for line in text.splitlines():
        head, _, rest = line.partition(" ")
        if head == "trial-view-b64":
            launches.append({"view": json.loads(base64.b64decode(rest)), "env": {},
                             "stream": [], "streamLines": []})
        elif not launches:
            continue
        elif head == "trial-env-b64":
            launches[-1]["env"] = parse_env(base64.b64decode(rest).decode(errors="replace"))
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


def launch_facts(label: str, view: dict[str, Any], session_id: str | None, child: dict[str, str],
                 live: Mapping[str, str], stream_sha: str | None) -> dict[str, Any]:
    """The facts of one launch that do not depend on the harness."""
    skills = view.get("skills") or {}
    descriptor = skills.get("descriptorPath")
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
        "streamSha256": stream_sha,
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


def clone_source(case: dict[str, Any], trial: Path, checkout: Path, commit: str,
                 externals: Mapping[str, dict[str, Any]] | None = None) -> dict[str, Any] | None:
    """For a case with `moveAway`, a plain Git checkout of the prepared commit
    in the trial directory. The settings name it for that source, so that the
    move changes no checkout that another case reads. An external source
    already has a checkout of its own in the trial, so the move takes that
    checkout. With `moveRoot`, the move takes only that root of the source
    directory, and the checkout stays in place."""
    source = case.get("moveAway")
    if not source:
        return None
    record = next((item for item in case.get("sources", []) if item["id"] == source), None)
    if record is None:
        raise TrialError(f"case {case['name']}: moveAway names '{source}', which is not one of its sources")
    root = case.get("moveRoot")
    external = (externals or {}).get(source)
    if external:
        moved = {"source": source, "checkout": external["checkout"], "commit": external["commit"],
                 "external": True}
    else:
        clone = trial / "sources" / f"{source}-checkout"
        clone.parent.mkdir(exist_ok=True)
        fetch_checkout(checkout, commit, clone, git_env(os.environ))
        head = run_checked(["git", "rev-parse", "HEAD"], cwd=clone, env=git_env(os.environ))
        moved = {"source": source, "checkout": str(clone), "commit": head}
    if root:
        moved["root"] = root
        moved["path"] = str(Path(moved["checkout"]) / record["plugin"] / root)
        moved["movedTo"] = str(trial / "sources" / f"{source}-{Path(root).name}-moved")
    else:
        moved["path"] = moved["checkout"]
        moved["movedTo"] = str(trial / "sources" / f"{source}-moved")
    return moved


def move_away(moved: dict[str, Any]) -> None:
    os.rename(moved["path"], moved["movedTo"])


def restore(moved: dict[str, Any]) -> None:
    if os.path.lexists(moved["movedTo"]):
        os.rename(moved["movedTo"], moved["path"])


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
            "sources": [{key: source.get(key) for key in ("id", "resourceDigest")}
                        for source in document.get("sources", [])],
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


def write_window(record: Path, start: int, end: int) -> None:
    (record / "lines").write_text(f"{start} {end}\n")


def resume_turn(adapter: HarnessAdapter, records: Path, view: dict[str, Any], program: Path, route: dict[str, Any],
                session_id: str, answer: str, cwd: Path, env: dict[str, str], out: Path,
                timeout_s: float) -> dict[str, Any]:
    """The resumed turn: the harness alone, outside Ravel, through the launch
    wrapper, which records its view, its prompt and its environment. The
    arguments are the fixed arguments of the route, the harness's resume and
    the answer, without `--add-dir`, as an owner's manual resume gives them."""
    args = adapter.resume_args(route, session_id)
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
    adapter = adapter_of(case)
    result: dict[str, Any] = {"mechanism": adapter.resume_mechanism, "marker": marker, "failures": []}
    first = launch_records(records)
    if len(first) != 1:
        result["failures"].append(f"the first driver run made {len(first)} launches, and a continuation resumes one")
        return result
    session_id, cwd = adapter.stream_init(first[0])
    transcript = adapter.session_file(home, session_id) if session_id else None
    if not transcript:
        result["failures"].append("the first turn wrote no session file to resume")
        return result
    cwd = cwd or adapter.session_cwd(transcript)
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
    result["resume"] = resume_turn(adapter, records, view, program, route, session_id, answer,
                                   Path(cwd or repo), env, trial, timeout_s)
    resumed = [record for record in launch_records(records) if record != first[0]]
    if len(resumed) != 1:
        result["failures"].append(f"the resumed turn left {len(resumed)} launch records, not one")
        return result
    resumed_id, _ = adapter.stream_init(resumed[0])
    if resumed_id != session_id:
        result["failures"].append(f"the resumed turn reports the session {resumed_id}, not {session_id}")
    elif adapter.session_files_of(home, session_id) != [transcript]:
        result["failures"].append("the resumed turn wrote a session file of its own, not the first one")
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
    if findings.get("opaque"):
        problems.append(f"the context holds {findings['opaque']} encrypted items, whose text the trial cannot "
                        f"read, so it cannot show the loss")
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


def gate_session(adapter: HarnessAdapter, home: Path, session: str | None, marks: dict[str, Any],
                 mode: str) -> dict[str, Any]:
    """The feeder's check before the answer turn: the session lines after the
    compaction show the case's loss. The adapter finds the session file and
    reads it in its harness's format."""
    path = adapter.session_file(home, session) if session else None
    if not path:
        return {"ok": False, "problems": ["no session file to check"]}
    findings = adapter.after_compaction(read_jsonl(path), marks)
    if findings is None:
        return {"ok": False, "problems": ["the session file holds no compaction boundary"]}
    problems = compaction_problems(findings, mode)
    return {"ok": not problems, "problems": problems, "findings": findings}


def run_turns(plan: Path, command: list[str]) -> int:
    """The feeder of a compaction case. The plan names the harness, whose
    adapter runs the conversation. The prompt is the last argument. Before a
    gated turn the feeder checks the session file, and a failed check ends the
    input with no further turn. Each step is a `trial-*` line on stderr."""
    spec = json.loads(plan.read_text())
    if not command:
        raise TrialError("the feeder needs a harness command")
    adapter = ADAPTERS[spec["harness"]]
    *argv, prompt = command
    marks = preamble_marks(prompt)
    home = Path(os.environ["HOME"])

    def log(kind: str, data: dict[str, Any]) -> None:
        stamp = dt.datetime.now(dt.UTC).strftime("%Y-%m-%dT%H:%M:%SZ")
        sys.stderr.write(f"trial-{kind} {json.dumps({'at': stamp, **data})}\n")
        sys.stderr.flush()

    def gate(session: str | None, mode: str) -> bool:
        verdict = gate_session(adapter, home, session, marks, mode)
        log("gate", {"mode": mode, "ok": verdict["ok"], "problems": verdict["problems"]})
        return verdict["ok"]

    return adapter.feed(argv, prompt, list(spec["turns"]), gate, log)


def feeder_route(records: Path, program: Path) -> dict[str, Any]:
    """The program of a compaction case: the feeder, with the harness program
    as its command."""
    return {"program": sys.executable,
            "programArgs": ["-I", str(Path(__file__).resolve()), "turns", "--plan",
                            str(Path(records).parent / "turns.json"), "--", str(program)]}


def write_turns(case: dict[str, Any], case_dir: Path, trial: Path) -> dict[str, Any]:
    """The feeder's plan: the harness, its compaction turn, then the answer,
    gated by the case's mode."""
    spec = case["compaction"]
    adapter = adapter_of(case)
    answer = (case_dir / spec.get("answer", "answer.md")).read_text()
    plan = {"harness": adapter.name,
            "turns": [adapter.compact_turn(spec), {"text": answer, "gate": spec["mode"]}]}
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
    the harness's own facts of the context after compaction, and the variable
    in the harness environment."""
    spec = case["compaction"]
    adapter = adapter_of(case)
    prompt = (record / "prompt").read_text()
    marks = preamble_marks(prompt)
    stderr = (record / "stderr").read_text(errors="replace") if (record / "stderr").exists() else ""
    env = parse_env((record / "env").read_text())
    report: dict[str, Any] = {
        "mode": spec["mode"],
        "mechanism": adapter.compaction_mechanism(spec["mode"]),
        "instructions": spec["instructions"],
        "preamble": {"sha256": marks["sha256"], "paths": len(marks["paths"]), "digests": len(marks["digests"]),
                     "lines": len(marks["lines"])},
        "turns": trial_lines(stderr, "turn"),
        "gate": next(iter(trial_lines(stderr, "gate")), None),
        "variable": {"set": DESCRIPTOR_VARIABLE in env,
                     "descriptor": env.get(DESCRIPTOR_VARIABLE) == launch.get("descriptorPath")},
        "failures": [],
    }
    path = adapter.session_file(home, launch["sessionId"]) if launch.get("sessionId") else None
    facts, failures = adapter.compaction_facts(read_jsonl(path) if path else [], marks, spec["mode"])
    report.update(facts)
    report["failures"] += failures
    if not facts:
        return report
    gate = report["gate"]
    if not gate or not gate.get("ok"):
        report["failures"].append(f"the gate before the answer did not pass: {gate}")
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
             model: str | None = None, effort: str | None = None, harness: str | None = None,
             working_copies: Mapping[str, Path] | None = None) -> dict[str, Any]:
    # Loading a case launches nothing. Nothing launches before the refusals
    # of the harness, including its version probe.
    case, case_dir = load_case(name, harness)
    adapter = adapter_of(case)
    owner_home = Path(environ["HOME"])
    replaced = adapter.refuse(environ, owner_home)
    if replaced:
        say(replaced)
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
    keg = prepared.get("keg")
    if keg and sha256_file(Path(keg["candidate"]) / "candidate.json") != keg["recordSha256"]:
        raise TrialError(f"{keg['candidate']}/candidate.json changed since preparation; prepare again")
    store = adapter.store
    if not (owner_home / store).exists():
        raise TrialError(f"no sign-in store at {owner_home / store}; sign in to {adapter.name} as the owner")
    program = adapter.program(environ)
    model = model or case.get("model", adapter.model)
    effort = effort or case.get("effort", adapter.effort)

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
    # An external source comes from its own checkout or working copy. Each
    # refusal of it comes before the harness's version probe.
    externals = external_sources(case, trial, working_copies or {}, environ, keg)
    # The supplier check comes before the moved-source clone, every copy and
    # the harness's version probe, so a fault sees each source intact.
    held = refuse_unsupplied(case, checkout, prepared["source"]["commit"], externals, trial / "suppliers")
    say(f"{name}: each required path is held ({held} paths)")
    clones = {source_id: external_root(external) for source_id, external in externals.items()}
    clone = clone_source(case, trial, checkout, prepared["source"]["commit"], externals)
    if clone:
        clones[clone["source"]] = Path(clone["checkout"])
    source_clone = None
    if case.get("continuation"):
        # A continuation commits into its sources between the turns, so one
        # clone of its own supplies every source and no other case sees it.
        source_clone = trial / "sources" / "checkout"
        source_clone.parent.mkdir(exist_ok=True)
        fetch_checkout(checkout, prepared["source"]["commit"], source_clone, git_env(os.environ))
        clones = {source["id"]: source_clone for source in case["sources"]}
    settings = settings_for(case, checkout, clones)
    if settings:
        (home / ".config" / "ravel" / "settings.json").write_text(json.dumps(settings, indent=2) + "\n")
    installed = install_native(case, home, checkout)
    claude_settings = adapter.write_settings(case, home)
    planted = plant_files(case, trial, checkout)
    route = route_for(case, records, program, model, effort)
    (trial / "route.json").write_text(json.dumps(route, indent=2) + "\n")
    if case.get("compaction") is not None:
        write_turns(case, case_dir, trial)
    (home / ".config" / "ravel" / "config.rhai").write_text(config_for(case, route))

    links_before = list_links(home)
    for link, target in links_before:
        say(f"link {link} -> {target}")
    check_links(links_before, store, owner_home, "before")
    config_dir = home / adapter.config_root
    say(f"configuration directory {config_dir}")

    tools = [shutil.which(tool, path=environ.get("PATH")) for tool in ("jj", "git")]
    if not all(tools):
        raise TrialError("jj and git must be on PATH")
    path_dirs = [str(front.parent), str(grove_bin), str(program.parent)]
    path_dirs += [str(Path(tool).parent) for tool in tools if tool] + list(SYSTEM_PATH)
    path_dirs = list(dict.fromkeys(path_dirs))
    env = {**allowed_env(environ, home, tmp, path_dirs), **adapter.env(home)}

    version = run_checked([str(program), "--version"], cwd=tmp, env=env).splitlines()[0]
    auth, reported, prelaunch = adapter.prelaunch(program, env, tmp, trial, config_dir)
    check_links(list_links(home), store, owner_home, "after the probes and before")
    plugins = adapter.install_plugins(case, program, env, tmp, home, checkout, prepared["source"]["commit"])
    if plugins:
        for plugin in plugins["installed"]:
            say(f"installed plugin {plugin['id']} version {plugin['version']}"
                + (" as a dependency" if plugin["dependency"] else ""))

    standalone = case["mode"] == "standalone"
    compaction = None
    deadline = case.get("timeoutMinutes", 30) * 60
    if standalone:
        # `grove run` stages the prompt, launches the case's kind through
        # Ravel and publishes the declared outputs after a successful end.
        spec = case["standalone"]
        outputs = trial / "out"
        outputs.mkdir()
        prompt = trial / "prompt.md"
        shutil.copyfile(case_dir / spec.get("prompt", "prompt.md"), prompt)
        argv = [str(grove_bin / "grove"), "run", spec.get("kind", case["kinds"][0]), "--prompt-file", str(prompt),
                *[arg for output in spec["outputs"] for arg in ("--output", str(outputs / output))],
                "--ui", "inline"]
        say(f"{name}: {version}, model {model}, effort {effort}, route {case['route']}, standalone, trial {trial}")
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
    links_checked, links_own = adapter.own_links(links_after, program)
    link_failure = None
    try:
        check_links(links_checked, store, owner_home, "after")
    except TrialError as error:
        link_failure = str(error)

    stderr = (trial / "driver.stderr").read_text(errors="replace")
    native, residue, plugin_copies = skill_files(home)
    if standalone:
        logs = sorted((home / ".local" / "state" / "grove" / "runs").glob("*.log"))
        launches = []
        run_log: list[str] = []
        for log in logs:
            for n, part in enumerate(split_log(log.read_text(errors="replace"))):
                stream_text = "".join(line + "\n" for line in part["streamLines"]).encode()
                launch = adapter.describe(f"{log.relative_to(home)}#{n}", part["view"], part["env"], part["stream"],
                                          sha256_bytes(stream_text), home, trial / "unknown", environ)
                launches.append(launch)
            run_log += log_lines(log.read_text(errors="replace"))
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
            launch = describe_launch(adapter, path, home, repo, environ, marker, turns)
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
    external_problems = external_failures(externals, launches, refusal, environ)
    record = {
        "schemaVersion": 1,
        "case": name,
        "purpose": case.get("purpose"),
        "credits": case.get("credits", []),
        "route": case["route"],
        "mode": case["mode"],
        "trial": str(trial),
        "builds": {"grove": prepared["grove"], "ravel": prepared["ravel"],
                   **({"keg": {k: v for k, v in keg.items() if k != "digests"}} if keg else {})},
        "harness": {
            "name": adapter.name, "program": str(program), "version": version, "model": model,
            "effort": effort, "permissionMode": adapter.permission_mode,
            "configDirectory": str(config_dir), "authConfigDirectory": reported,
            "loggedIn": auth.get("loggedIn"),
            **({"caseHarness": case["caseHarness"]} if case.get("caseHarness") else {}),
            **adapter.record_extras(auth, replaced, prelaunch),
        },
        "sources": {"checkout": prepared["source"]["checkout"], "commit": prepared["source"]["commit"],
                    "settings": settings, "external": list(externals.values())},
        "environment": sorted(env),
        "links": {"before": links_before, "after": links_after, "harnessOwn": links_own, "failure": link_failure},
        "skills": {"nativeCopies": native, "residue": residue, "installed": installed,
                   "claudeSettings": claude_settings, "planted": planted,
                   "pluginCopies": plugin_copies, "plugins": plugins},
        "launches": launches,
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
    failures += plugin_failures(plugin_copies, plugins)
    failures += delegate_failures(case, launches)
    failures += external_problems
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
    adapter = adapter_named(record["harness"].get("name", DEFAULT_HARNESS))
    fresh: list[dict[str, Any]] = []
    if record.get("mode", "lifecycle") == "standalone":
        for log in sorted((home / ".local" / "state" / "grove" / "runs").glob("*.log")):
            for n, part in enumerate(split_log(log.read_text(errors="replace"))):
                stream_text = "".join(line + "\n" for line in part["streamLines"]).encode()
                fresh.append(adapter.describe(f"{log.relative_to(home)}#{n}", part["view"], part["env"],
                                              part["stream"], sha256_bytes(stream_text), home,
                                              trial / "unknown", {}))
    else:
        marker = (record.get("continuation") or {}).get("marker")
        turns = record.get("compaction") is not None
        fresh = [describe_launch(adapter, path, home, repo, {}, marker, turns)
                 for path in launch_records(trial / "records")]
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
    if event["tool"] in ("Bash", "exec_command", "text", "prompt") and len(target) > COMMAND_CHARS:
        target = target[:COMMAND_CHARS] + "…"
    if event["tool"] in ("text", "prompt", "compaction"):
        return cell(target)
    if event["tool"] == "exec_command" and event.get("cwd"):
        return f"`{cell(target)}` in `{cell(event['cwd'])}`"
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
        f"`{DESCRIPTOR_VARIABLE}` {'named' if findings['variable'] else 'not named'}"
        + (f", {findings['opaque']} encrypted items." if findings.get("opaque") else ".")
    ]
    lines += [f"  - Term in context: `…{cell(snippet)}…`" for snippet in findings.get("snippets", [])]
    return lines


def render_compaction(adapter: HarnessAdapter, compaction: dict[str, Any]) -> list[str]:
    preamble = compaction["preamble"]
    gate = compaction.get("gate") or {}
    lines = [
        "Compaction:",
        "",
        f"- Mode: {compaction['mode']} loss. Mechanism: {compaction['mechanism']}.",
        f"- Preamble: SHA-256 `{preamble['sha256']}`, {preamble['paths']} paths, {preamble['digests']} digests, "
        f"{preamble['lines']} lines of at least {PREAMBLE_LINE_CHARS} characters.",
        *[f"- Turn {t['turn']} sent at {t['at']}, "
          + (f"SHA-256 `{t['sha256']}`." if t.get("sha256") else f"request `{t.get('request')}`.")
          for t in compaction.get("turns", [])],
        (f"- Gate before the answer: {'passed' if gate.get('ok') else 'failed'}"
         + (f": {cell('; '.join(gate.get('problems', [])))}" if gate.get("problems") else "") + "."
         if gate else "- Gate before the answer: no gate ran."),
        *adapter.render_compaction(compaction),
    ]
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
            *([f"- Call: {item['form']}. Fork: {cell(item.get('forkTurns') or 'not named')}. "
               f"The delegate's history holds the descriptor path: {'yes' if item.get('historyHoldsDescriptor') else 'no'}. "
               f"Delegate's turn: `{cell(json.dumps(item.get('sandbox') or {}))}`. Messages: "
               f"{(item.get('messages') or {}).get('toDelegate')} from the parent by `send_message`, "
               f"{(item.get('messages') or {}).get('fromDelegate')} from the delegate."] if "form" in item else []),
            (f"- Prompt: encrypted in every transcript, so what the parent gave is unknown." if item["promptSha256"] is None
             and item.get("form") == "direct, encrypted" else
             f"- Prompt SHA-256 `{item['promptSha256']}`. Descriptor path: {'given' if item['givesDescriptorPath'] else 'not given'}. "
             f"Variable named: {'yes' if item['namesVariable'] else 'no'}. Obligations word for word: "
             f"{len(item['obligations'])} of {item['obligationCount']}"
             + (f" ({', '.join(item['obligations'])})" if item['obligations'] else "") + "."),
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


def external_row(external: dict[str, Any], trial: str) -> str:
    """The table row of one external source: its checkout, or the working
    copy that replaced the checkout."""
    digest = f"`resourceDigest` `{external.get('resourceDigest')}`"
    if "keg" in external:
        return (f"| Keg source `{external['id']}` | the candidate keg `{cell(external['keg'])}` of Grove commit "
                f"`{external['commit']}`; source directory `{external['plugin']}`, captured from "
                f"{', '.join(f'`{cell(o)}`' for o in external.get('originPaths', [])) or 'no path'}; {digest}, "
                f"accepted `{external['acceptedDigest']}` |")
    wanted = f"`{cell(external['repository'])}` at commit `{external['commit']}`"
    if "workingCopy" in external:
        copy = external["workingCopy"]
        state = "empty" if copy["empty"] else "not empty"
        return (f"| External source `{external['id']}` | the working copy `{cell(copy['path'])}` in place of "
                f"{wanted}: jj change `{copy['changeId'][:12]}`, commit `{copy['commitId'][:12]}`, the change "
                f"is {state}; source directory `{external['plugin']}`, {digest} |")
    checkout = Path(external["checkout"]).relative_to(trial)
    return (f"| External source `{external['id']}` | {wanted}, a plain Git checkout at `$TRIAL/{checkout}`; "
            f"source directory `{external['plugin']}` with Git tree `{external['treeId']}`, {digest} |")


def plugin_rows(plugins: dict[str, Any] | None) -> list[str]:
    """The table rows of a native plugin route: the marketplace, then each
    installed plugin with its skills."""
    if not plugins:
        return []
    market = plugins["marketplace"]
    rows = [f"| Plugin marketplace | `{cell(market['name'])}`, added from the checkout `{cell(market['path'])}` "
            f"at commit `{market['commit'][:12]}`; the checkout stayed clean |",
            "| Plugin installations | " + cell(", ".join(f"`{i['id']}`" for i in plugins["installs"])) + " |"]
    for plugin in plugins["installed"]:
        skills = ", ".join(f"`{'' if s.get('root', 'skills') == 'skills' else s['root'] + '/'}{s['name']}` "
                           f"`{s['sha256'][:12]}…`" for s in plugin["skills"]) or "none"
        how = "installed as a dependency" if plugin["dependency"] else "installed by name"
        commit = f"Git commit `{plugin['gitCommitSha'][:12]}`" if plugin.get("gitCommitSha") else "no Git commit"
        rows.append(f"| Installed plugin `{cell(plugin['id'])}` | version `{cell(plugin['version'])}`, {commit}, "
                    f"{how}, at `{cell(plugin['installPath'])}`; skills {cell(skills)} |")
    return rows


def render(record: dict[str, Any]) -> str:
    """The Markdown facts of one case. The result and its limit are the
    reader's judgement, so the acceptance record adds them by hand."""
    h = record["harness"]
    adapter = adapter_named(h.get("name", DEFAULT_HARNESS))
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
        *adapter.render_rows(h),
        *([f"| Case harness | the case names `{h['caseHarness']}`; this run uses `{h['name']}` with its own defaults |"]
          if h.get("caseHarness") else []),
        f"| Source checkout | commit `{record['sources']['commit'][:12]}` |",
        *[external_row(external, record["trial"]) for external in record["sources"].get("external", [])],
        f"| Links before and after | {cell(record['links']['before'] and record['links']['before'][0][0])} only; after: {cell(', '.join(link for link, _ in record['links']['after']) or 'none')} |",
        *([f"| Harness's own links after | {cell(', '.join(f'{link} -> {target}' for link, target in record['links']['harnessOwn']))} |"]
          if record['links'].get('harnessOwn') else []),
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
        *plugin_rows(record['skills'].get('plugins')),
        f"| Platform-managed residue | {cell(', '.join(record['skills']['residue']) or 'none')} |",
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
    lines += adapter.render_prelaunch(h)
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
            (f"- Source `{clone['source']}`: a plain Git checkout at commit `{clone['commit'][:12]}`. "
             + (f"Its root `{clone['root']}` was " if clone.get("root") else "It was ")
             + f"moved from `$TRIAL/{Path(clone.get('path', clone['checkout'])).relative_to(record['trial'])}` "
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
        lines += render_compaction(adapter, compaction)
    note = record.get("systemNote")
    if note:
        lines += [f"Owner note through {adapter.note_channel}, SHA-256 `{note['sha256']}`:", "",
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


def parse_working_copies(values: list[str]) -> dict[str, Path]:
    """The `--working-copy ID=DIR` options, by source ID."""
    copies: dict[str, Path] = {}
    for value in values:
        source_id, separator, directory = value.partition("=")
        if not separator or not source_id or not directory:
            raise TrialError(f"--working-copy takes ID=DIR, not {value!r}")
        if source_id in copies:
            raise TrialError(f"--working-copy names the source '{source_id}' twice")
        path = Path(directory).expanduser().resolve()
        if not path.is_dir():
            raise TrialError(f"--working-copy names {path}, which is not a directory")
        copies[source_id] = path
    return copies


def main(argv: list[str] | None = None, environ: Mapping[str, str] | None = None) -> int:
    environ = os.environ if environ is None else environ
    parser = argparse.ArgumentParser(prog="harness_trial.py", description=__doc__.split("\n\n")[0])
    commands = parser.add_subparsers(dest="command", required=True)
    p = commands.add_parser("prepare", help="build Grove at a commit and check the Ravel 2.0 front")
    p.add_argument("--out", required=True, type=Path)
    p.add_argument("--commit", required=True)
    p.add_argument("--ravel", required=True, type=Path, help="the absolute 2.0 front, PREFIX/bin/ravel")
    p.add_argument("--keg", type=Path, help="the OUT of `task owner:candidate-keg` for the same commit, "
                   "which supplies each keg source")
    r = commands.add_parser("run", help="run one real trial case (spends subscription usage)")
    r.add_argument("--prepared", required=True, type=Path)
    r.add_argument("--case", required=True)
    r.add_argument("--harness", choices=HARNESSES, help="run the case in this harness, with its own defaults")
    r.add_argument("--model")
    r.add_argument("--effort")
    r.add_argument("--working-copy", action="append", default=[], metavar="ID=DIR",
                   help="take the external source ID from the working copy DIR instead of its checkout")
    c = commands.add_parser("check", help="check the required paths of cases against a Grove commit, "
                            "without a launch")
    c.add_argument("--commit", required=True, help="the Grove commit that the record of the case names")
    c.add_argument("--case", action="append", default=[], help="a case to check; every case when omitted")
    c.add_argument("--repository", type=Path, help="the repository that holds the commit; this one by default")
    c.add_argument("--keg", type=Path, help="the OUT of `task owner:candidate-keg` for the same commit")
    x = commands.add_parser("reextract", help="extract the events of an earlier run again, from its transcripts")
    x.add_argument("--trial", required=True, type=Path)
    t = commands.add_parser("turns", help="the feeder of a compaction case, which the trial configuration runs")
    t.add_argument("--plan", required=True, type=Path)
    t.add_argument("program_argv", nargs=argparse.REMAINDER, help="-- PROGRAM ARG... PROMPT")
    args = parser.parse_args(argv)
    try:
        if args.command == "prepare":
            prepare(args.out, args.commit, args.ravel, environ, args.keg)
        elif args.command == "check":
            return check_cases(args.case, args.commit, args.repository, environ, args.keg)
        elif args.command == "reextract":
            reextract(args.trial)
        elif args.command == "turns":
            command = args.program_argv[1:] if args.program_argv[:1] == ["--"] else args.program_argv
            return run_turns(args.plan, command)
        else:
            run_case(args.prepared, args.case, environ, args.model, args.effort, args.harness,
                     parse_working_copies(args.working_copy))
    except TrialError as error:
        print(f"harness-trial: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
