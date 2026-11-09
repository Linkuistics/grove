#!/usr/bin/env python3
"""Run the discovery trials of the owner cutover in the retained staging HOME.

"Test seams" and "Cutover contract" of docs/specs/owner-skill-delivery.md own
the trials. The staging HOME, the candidate keg and the accepted Ravel 2.0
build come from owner-staging-setup-k248, and staging_home.py owns the
staging launcher's rules. This runner adds the trials themselves.

    staging_trial.py sources --home HOME --candidate KEG --ravel PREFIX --out DIR [--source ID]
    staging_trial.py case --name NAME --harness claude|codex --home HOME --ravel PREFIX --out DIR
                          [--model M] [--effort E]
    staging_trial.py rejudge --out DIR

`sources` runs the trials that launch no model:

- The off value resolves as `disabled` in a disposable HOME. Its control
  resolves the staging settings in another disposable HOME, and the
  `disabled` check must fail on it.
- One staging launch with a network and one without receive the same skill
  directories. The worker is `/usr/bin/true` behind the route wrapper, which
  records the request view before the worker runs. A network probe shows
  that the profile denies a connection that an unguarded probe makes.
- With one candidate-keg source moved away, a launch refuses as
  `skill_source_missing`, and the wrapper records no launch. The source
  returns, its tree digest is unchanged, and a new launch receives its
  accepted `resourceDigest`. The launches with the source present are the
  control: each records a launch.

`case` runs one model case of `staging-cases/NAME.json` on one harness. The
session runs through `ravel start-session` of the accepted 2.0 build with
`--config`, so no trial writes the retained `config.rhai`. The route wrapper
`scripts/harness-trials/trial-launch.sh` records the view, the environment
and the stream. The adapters of scripts/harness_trial.py extract the events
of the session file. The verdicts are:

- `mandatoryFirst`: the agent reads the mandatory file before it looks for
  another skill and before any task action.
- `requires`: the agent reads each named skill from its source before its
  first edit, and it makes at least one edit. A read is a classified path,
  with globs expanded, or a tool result that holds the skill's `name:` line
  and the first heading of its body. The record keeps each result only as
  its digest and length.
- `kindSkill`: the agent reads the named Grove kind skill from its source
  before the first event that touches the work tree: an edit, a path in the
  work tree or a mention of `.grove`. It also reads a file of that source
  other than a `SKILL.md`, such as `grove/references/bootstrap.md`. The
  record keeps the position of that read. The case's kind is the kind of
  the skill.
- `delegate`: the parent starts at least one delegate, and a delegate reads
  a file of the named skill from its source.
- `compaction`: the session runs three turns in one managed process, through
  the feeder `harness_trial.py turns`. They are the prompt, the harness's
  default compaction and the case's `answer`. Claude Code compacts with a
  bare `/compact`, and Codex with `thread/compact/start`. The agent reads the
  named skill from its source after the compaction boundary and before the
  first edit after it. The record keeps what the context after compaction
  still holds of Ravel's preamble, without a judgement.
- `staleCopy`: the runner copies the named stale native copy from the real
  HOME into each project skill root of the work directory, `.claude/skills`
  for Claude Code and `.codex/skills` for Codex. Its tree digest must equal
  the case's pin. The verdict records which copy the agent reads: the
  planted copy, Ravel's copy from the named source, or both. It fails when
  the agent reads neither, or when the native listing does not hold the
  planted name. The staging HOME holds no copy of that name before or after.
- The native skill listing of the session holds no superseded name. The
  planted name of `staleCopy` is the one exception.
- No real-HOME subject changed, and the real `.claude.json` names no
  staging path. The real plugin cache is compared without the `.in_use`
  reader markers of its version directories. Each marker that the trial
  added or rewrote must name a process that still runs, such as the owner's
  own session.

A case with `tree` runs in a new jj repository whose `.grove/` is a copy of
`staging-cases/trees/TREE`. The runner judges the mandatory order of every
case, and fails it only when the case sets `mandatoryFirst`.

`rejudge` extracts and judges a finished case again from its unchanged
launch records, after a change to the extraction or the verdicts.

A case is a small JSON object. A field that the runner does not know
refuses, so a later leaf adds the field and its verdict together.

Every launch has HOME set to the staging HOME and CODEX_HOME set to its
`.codex` tree, from an allow-listed environment, in a new working directory
under /private/tmp. A real session spends subscription usage, so no gate
runs this script. test_staging_trial.py tests the verdicts.
"""

from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import json
import os
import re
import shutil
import socket
import subprocess
import sys
from pathlib import Path
from typing import Any

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
sys.path.insert(0, str(ROOT / "scripts"))
sys.path.insert(0, str(HERE))
import harness_trial  # the adapters and the event classifier
import staging_home  # the staging launcher's rules

# An agent reads a snapshot through a glob, such as `cat d8ad*/tree/...`,
# because each snapshot directory is a long digest. Codex also reads skills
# inside a Python heredoc, which names no path, so the verdicts read each
# tool result too.
harness_trial.EXPAND_GLOBS = True
harness_trial.KEEP_RESULTS = True
# A Grove session can do its task through commands alone, such as
# `grove-llm leaf-add`, so the kind verdict needs each event's touch of the
# work tree.
harness_trial.KEEP_CLASSES = True

CASES = HERE / "staging-cases"
OFF_SETTINGS = ROOT / "docs" / "owner-cutover" / "settings-off.json"
TREES = CASES / "trees"
CASE_FIELDS = {"schemaVersion", "name", "purpose", "kind", "prompt", "mandatoryFirst", "requires",
               "timeoutMinutes", "tree", "kindSkill", "delegate", "compaction", "staleCopy"}
# The project skill roots that each harness lists, below its working
# directory. Codex 0.167.0 lists `.codex/skills` and `.agents/skills` of the
# working directory or its Git root, and not `.claude/skills`, as `codex debug
# prompt-input` showed in the staging environment
# (staging-compaction-stale-trials-k277). A stale case plants one copy in the
# root of each harness, so each harness lists exactly one.
PROJECT_ROOTS = {"claude": ".claude/skills", "codex": ".codex/skills"}
# The native user roots of the staging HOME. None may hold the planted name.
HOME_ROOTS = (".claude/skills", ".codex/skills", ".agents/skills")
# The key of Codex's default compaction in harness_trial.CODEX_COMPACTION:
# remote compaction, which keeps each user message word for word.
DEFAULT_COMPACTION = {"mode": "path"}
# A line of a SKILL.md this long or longer can tell two copies apart.
DISTINCT_LINE_CHARS = 30
# The classes of an event that looks for a skill or acts on the task. The
# mandatory obligation of Ravel's discovery instructions comes before each.
GATED = {"listing", "skill", "kind-skill", "supporting", "native", "planted", "source-origin", "action",
         "command", "work", "delegate", "native-skill-tool"}
SKILL_READS = {"skill", "kind-skill"}
# The classes of a read of any file in a snapshot.
RESOURCE_READS = {"mandatory", "skill", "kind-skill", "supporting"}
# Real-HOME subjects that the owner's own running session rewrites. The
# check reads them for staging paths instead of comparing digests.
LIVE_SUBJECTS = {".claude.json"}
# Claude Code writes `.in_use/<pid>` into each plugin version directory,
# `cache/<marketplace>/<plugin>/<version>`, that a running session loaded.
# The marker names a reader, not the plugin, and the owner's own session
# writes it during a trial. The plugin cache digest leaves it out, as the
# plugin row of inventory.py does, and the check reads each marker instead.
PLUGIN_CACHE = ".claude/plugins/cache"
READER_MARKER = ".in_use"
NOOP_WORKER = "/usr/bin/true"


class TrialFailure(Exception):
    pass


def now() -> str:
    return dt.datetime.now(dt.UTC).strftime("%Y-%m-%dT%H:%M:%SZ")


# --- Inputs -------------------------------------------------------------------


def load_case(name: str) -> dict[str, Any]:
    path = CASES / f"{name}.json"
    if not path.is_file():
        known = sorted(p.stem for p in CASES.glob("*.json"))
        raise TrialFailure(f"no staging case {name}; the cases are {known}")
    case = json.loads(path.read_text())
    for field in case:
        if field not in CASE_FIELDS:
            raise TrialFailure(f"case {name}: unknown field '{field}'; the fields are {sorted(CASE_FIELDS)}")
    if case.get("name") != name:
        raise TrialFailure(f"case {name}: its name field is {case.get('name')!r}")
    for label, item in [*(("requires", i) for i in case.get("requires", [])),
                        *((key, case[key]) for key in ("kindSkill", "delegate") if key in case)]:
        if not isinstance(item, dict) or set(item) != {"source", "skill"}:
            raise TrialFailure(f"case {name}: {label} is {{source, skill}}, not {item!r}")
    if "kindSkill" in case and case["kindSkill"]["skill"] != f"grove-{case.get('kind')}":
        raise TrialFailure(f"case {name}: kindSkill {case['kindSkill']['skill']} is not the skill of kind "
                           f"{case.get('kind')!r}")
    if "tree" in case and not (TREES / case["tree"] / "_BRIEF.md").is_file():
        raise TrialFailure(f"case {name}: no tree {TREES / case['tree']} with a _BRIEF.md")
    if "compaction" in case:
        spec = case["compaction"]
        if not isinstance(spec, dict) or set(spec) != {"answer", "after"} or not isinstance(spec["answer"], str) \
                or not isinstance(spec["after"], dict) or set(spec["after"]) != {"source", "skill"}:
            raise TrialFailure(f"case {name}: compaction is {{answer, after: {{source, skill}}}}, not {spec!r}")
    if "staleCopy" in case:
        spec = case["staleCopy"]
        if not isinstance(spec, dict) or set(spec) != {"source", "skill", "from", "treeSha256"}:
            raise TrialFailure(f"case {name}: staleCopy is {{source, skill, from, treeSha256}}, not {spec!r}")
        if Path(spec["from"]).is_absolute() or ".." in Path(spec["from"]).parts:
            raise TrialFailure(f"case {name}: staleCopy.from is a path relative to the real HOME, not {spec['from']}")
    return case


def named_skills(case: dict[str, Any]) -> list[dict[str, str]]:
    """Every {source, skill} that a verdict of the case names."""
    return [*case.get("requires", []), *(case[key] for key in ("kindSkill", "delegate") if key in case),
            *([case["compaction"]["after"]] if "compaction" in case else [])]


def plant_stale(work: Path, spec: dict[str, Any]) -> dict[str, Any]:
    """The stale native copy in each project skill root of the work
    directory, copied from the real HOME without its links followed. A copy
    whose tree digest differs from the pin refuses, so each run plants the
    same bytes."""
    origin = staging_home.real_home() / spec["from"]
    digest = staging_home.tree_digest(origin)
    if digest.get("sha256") != spec["treeSha256"]:
        raise TrialFailure(f"the stale copy {origin} has tree digest {digest}, not the pinned {spec['treeSha256']}; "
                           f"a changed or missing copy changes what the case measures")
    plants = {}
    for harness, root in PROJECT_ROOTS.items():
        target = work / root / spec["skill"]
        shutil.copytree(origin, target, symlinks=True)
        plants[harness] = str(target)
    return {"origin": str(origin), "tree": digest, "plants": plants}


def home_copies(home: Path, skill: str) -> list[str]:
    """Each native user root of the staging HOME that holds the skill."""
    return [str(home / root / skill) for root in HOME_ROOTS if os.path.lexists(home / root / skill)]


def distinct_lines(one: Path, other: Path) -> list[str]:
    """The long lines of one file that the other does not hold anywhere. A
    result that holds one of them shows a read of that file's copy. The test
    is a substring test, because the match is one: a line that the other
    file holds inside a longer, differently wrapped line is no mark."""
    theirs = other.read_text()
    return sorted({line.strip() for line in one.read_text().splitlines()
                   if len(line.strip()) >= DISTINCT_LINE_CHARS and line.strip() not in theirs})


def plant_tree(work: Path, tree: str, env: dict[str, str]) -> str:
    """The case's tree as `.grove/` of a new jj repository in the work
    directory, committed. The result is the commit of the tree."""
    jj = shutil.which("jj", path=env["PATH"])
    if not jj:
        raise TrialFailure(f"no jj on the staging PATH {env['PATH']}")
    def jj_run(*args: str) -> str:
        done = subprocess.run([jj, *args], cwd=work, env=env, capture_output=True, text=True, check=False)
        if done.returncode != 0:
            raise TrialFailure(f"jj {' '.join(args)} failed in {work}: {done.stderr.strip()}")
        return done.stdout.strip()
    # Not colocated: Codex's sandbox denies writes below a `.git` component,
    # so a colocated repository refuses every commit. Grove's own launches
    # grant that store through a profile, which a trial route does not pass.
    jj_run("--config", "git.colocate=false", "git", "init", ".")
    shutil.copytree(TREES / tree, work / ".grove")
    jj_run("commit", "--quiet", "--message", "trial tree")
    return jj_run("log", "--no-graph", "-r", "@-", "-T", "commit_id")


def check_inputs(home: Path, ravel: Path) -> dict[str, Any]:
    """The retained staging HOME holds the staging settings, and the Ravel
    build is the pinned 2.0 build."""
    settings = home / ".config" / "ravel" / "settings.json"
    if not settings.is_file():
        raise TrialFailure(f"no staging settings at {settings}; the retained staging HOME is missing, and its "
                           f"absence stops the staging trials")
    if settings.read_bytes() != staging_home.SETTINGS.read_bytes():
        raise TrialFailure(f"{settings} differs from {staging_home.SETTINGS}")
    builds = json.loads(harness_trial.BUILDS.read_text())
    pinned = builds["ravel2"]["binRavelSha256"][harness_trial.platform_key()]
    actual = harness_trial.sha256_file(ravel / "bin" / "ravel")
    if actual != pinned:
        raise TrialFailure(f"{ravel / 'bin' / 'ravel'} has SHA-256 {actual}, not the pinned {pinned}")
    return {"settingsSha256": harness_trial.sha256_file(settings), "ravelSha256": actual,
            "ravelBuild": builds["ravel2"]["buildCommit"]}


def plugin_cache_digest(cache: Path) -> dict[str, Any]:
    """The plugin cache without the top-level `.in_use` of each version
    directory, and the content of each such marker by its path. A nested
    `.in_use` stays in the digest."""
    digest, count, markers = hashlib.sha256(), 0, {}
    for directory, names, files in os.walk(cache):
        here = Path(directory)
        rel_dir = here.relative_to(cache)
        if len(rel_dir.parts) == 3 and READER_MARKER in names:
            names.remove(READER_MARKER)
            for marker in sorted((here / READER_MARKER).iterdir()):
                markers[str(marker.relative_to(cache))] = marker.read_text(errors="replace")[:200]
        names.sort()
        for name in sorted(names + files):
            entry = here / name
            rel = str(entry.relative_to(cache))
            if entry.is_symlink():
                digest.update(f"L {rel} {os.readlink(entry)}\n".encode())
            elif entry.is_file():
                digest.update(f"F {rel} {staging_home.sha256_file(entry)}\n".encode())
            else:
                digest.update(f"D {rel}\n".encode())
            count += 1
    return {"kind": "tree", "sha256": digest.hexdigest(), "entries": count, "readerMarkers": markers}


def real_subjects() -> dict[str, Any]:
    real = staging_home.real_home()
    return {rel: plugin_cache_digest(real / rel) if rel == PLUGIN_CACHE else staging_home.tree_digest(real / rel)
            for rel in staging_home.REAL_SUBJECTS if rel not in LIVE_SUBJECTS}


def alive(pid: int) -> bool:
    try:
        os.kill(pid, 0)
    except ProcessLookupError:
        return False
    except PermissionError:
        return True
    return True


def marker_problems(before: dict[str, str], after: dict[str, str]) -> list[str]:
    """A marker that the trial added or rewrote names a process that still
    runs after the trial. Ravel reaps the trial's processes before the check,
    so a marker of one of them names a process that has ended. A marker that
    vanished belongs to a session that ended, which is not a write by the
    trial."""
    problems = []
    for path in sorted(after):
        if before.get(path) == after[path]:
            continue
        pid = Path(path).name
        if not pid.isdigit() or not alive(int(pid)):
            problems.append(f"the real {PLUGIN_CACHE}/{path} is new and names no running process: {after[path]!r}")
    return problems


def isolation_problems(before: dict[str, Any], after: dict[str, Any], home: Path) -> list[str]:
    def without_markers(digest: Any) -> Any:
        return {k: v for k, v in digest.items() if k != "readerMarkers"} if isinstance(digest, dict) else digest
    problems = [f"the real {rel} changed" for rel in sorted(before)
                if without_markers(before[rel]) != without_markers(after.get(rel))]
    if PLUGIN_CACHE in before:
        problems += marker_problems(before[PLUGIN_CACHE].get("readerMarkers", {}),
                                    (after.get(PLUGIN_CACHE) or {}).get("readerMarkers", {}))
    for rel in sorted(LIVE_SUBJECTS):
        path = staging_home.real_home() / rel
        if path.is_file() and str(home) in path.read_text(errors="replace"):
            problems.append(f"the real {rel} names the staging HOME {home}")
    return problems


# --- Launches -----------------------------------------------------------------


def write_config(out: Path, label: str, kind: str, route: dict[str, Any]) -> Path:
    config = out / f"{label}.rhai"
    config.write_text(harness_trial.config_for({"name": f"staging-{label}"}, {"kinds": [kind], **route}))
    return config


def noop_route(records: Path) -> dict[str, Any]:
    return {"wrapper": str(harness_trial.WRAPPER), "records": str(records), "program": NOOP_WORKER,
            "programArgs": [], "addDirs": False, "fixedArgs": [], "provider": "anthropic",
            "model": "none", "effort": "none"}


def start(ravel: Path, config: Path, kind: str, prompt: str, cwd: Path, env: dict[str, str], timeout: int,
          offline: bool = False) -> dict[str, Any]:
    argv = [str(ravel / "bin" / "ravel"), "start-session", "--kind", kind, "--config", str(config),
            "--prompt", prompt, "--json"]
    if offline:
        argv = ["/usr/bin/sandbox-exec", "-p", harness_trial.NO_NETWORK, *argv]
    started = now()
    done = subprocess.run(argv, env=env, cwd=cwd, capture_output=True, text=True, timeout=timeout, check=False)
    return {"argv": argv, "cwd": str(cwd), "started": started, "ended": now(), "exit": done.returncode,
            "result": done.stdout, "stderr": done.stderr[-6000:]}


def views(records: Path) -> list[dict[str, Any]]:
    return [json.loads((record / "view.json").read_text()) for record in harness_trial.launch_records(records)]


def skill_view(view: dict[str, Any]) -> dict[str, Any]:
    """What a launch received: its directories, its mandatory files and each
    source's identity."""
    skills = view.get("skills") or {}
    return {
        "descriptorPath": skills.get("descriptorPath"),
        "directories": [item.get("path") for item in skills.get("directories", [])],
        "mandatory": [item.get("path") for item in skills.get("mandatory", [])],
        "sources": [{key: source.get(key) for key in ("id", "resourceDigest", "originPath", "snapshotRoot")}
                    for source in skills.get("sources", [])],
    }


def view_differences(one: dict[str, Any], other: dict[str, Any]) -> list[str]:
    return [key for key in ("descriptorPath", "directories", "mandatory", "sources") if one.get(key) != other.get(key)]


def refusal_code(stderr: str) -> str | None:
    """The code of Ravel's JSON refusal document on stderr."""
    for line in stderr.splitlines():
        line = line.strip()
        if line.startswith("{"):
            try:
                document = json.loads(line)
            except json.JSONDecodeError:
                continue
            code = document.get("code") or (document.get("error") or {}).get("code")
            if code:
                return code
    return None


def network_control() -> dict[str, int]:
    """An unguarded probe reaches a local listener, and the same probe under
    the no-network profile cannot."""
    listener = socket.socket()
    listener.bind(("127.0.0.1", 0))
    listener.listen(4)
    port = str(listener.getsockname()[1])
    try:
        probe = ["/usr/bin/nc", "-z", "-w", "2", "127.0.0.1", port]
        unguarded = subprocess.run(probe, capture_output=True, check=False).returncode
        guarded = subprocess.run(["/usr/bin/sandbox-exec", "-p", harness_trial.NO_NETWORK, *probe],
                                 capture_output=True, check=False).returncode
    finally:
        listener.close()
    return {"unguardedExit": unguarded, "guardedExit": guarded}


def resolve_in(home: Path, ravel: Path, settings: Path, cwd: Path) -> dict[str, Any]:
    """`ravel skills resolve --json` in a disposable HOME that holds only the
    given settings."""
    (home / ".config" / "ravel").mkdir(parents=True)
    shutil.copyfile(settings, home / ".config" / "ravel" / "settings.json")
    env = {"HOME": str(home), "PATH": f"{ravel / 'bin'}:/usr/bin:/bin", "TMPDIR": os.environ.get("TMPDIR", "/tmp")}
    argv = [str(ravel / "bin" / "ravel"), "skills", "resolve", "--json"]
    done = subprocess.run(argv, env=env, cwd=cwd, capture_output=True, text=True, check=False)
    try:
        document = json.loads(done.stdout)
    except json.JSONDecodeError:
        document = {}
    return {"argv": argv, "home": str(home), "settings": str(settings), "exit": done.returncode,
            "status": document.get("status"), "sourceCount": len(document.get("sources", [])),
            "stdout": done.stdout[:400], "stderr": done.stderr[-2000:]}


# --- Source trials ------------------------------------------------------------


def candidate_source(candidate: Path, source_id: str) -> dict[str, Any]:
    """The candidate keg's record of one source: its origin and its accepted
    resourceDigest."""
    record = json.loads((candidate / "candidate.json").read_text())
    for source in record["sources"]:
        if source["id"] == source_id:
            return source
    raise TrialFailure(f"the candidate keg has no source {source_id}")


def digest_of(view: dict[str, Any] | None, source_id: str) -> str | None:
    """The resourceDigest of one source in a skill view."""
    return next((s["resourceDigest"] for s in (view or {}).get("sources", []) if s["id"] == source_id), None)


def sources(home: Path, candidate: Path, ravel: Path, out: Path, source_id: str,
            environ: dict[str, str]) -> dict[str, Any]:
    staging_home.refuse_inherited(environ, home)
    inputs = check_inputs(home, ravel)
    if out.exists():
        raise TrialFailure(f"{out} exists; name a new directory")
    out.mkdir(parents=True)
    work = staging_home.work_directory(home)
    env = staging_home.staged_env(home, ravel)
    record: dict[str, Any] = {"started": now(), "home": str(home), "candidate": str(candidate),
                              "inputs": inputs, "work": str(work)}
    before = real_subjects()
    failures: list[str] = []
    kind = "staging-source-trial"
    prompt = "A source trial launches no model."

    # The off value and its control.
    off = resolve_in(out / "off-home", ravel, OFF_SETTINGS, work)
    control = resolve_in(out / "control-home", ravel, staging_home.SETTINGS, work)
    record["offValue"] = {"off": off, "control": control,
                          "offDisabled": off["status"] == "disabled",
                          "controlDisabled": control["status"] == "disabled"}
    if not (off["exit"] == 0 and off["status"] == "disabled"):
        failures.append(f"the off value resolved as {off['status']!r} with exit {off['exit']}, not 'disabled'")
    if control["status"] == "disabled" or control["sourceCount"] != 10:
        failures.append(f"the control resolved as {control['status']!r} with {control['sourceCount']} sources; "
                        f"the disabled check is not seen to fail")

    def launch(label: str, offline: bool = False) -> dict[str, Any]:
        records = out / label
        records.mkdir()
        config = write_config(out, label, kind, noop_route(records))
        result = start(ravel, config, kind, prompt, work, env, 300, offline)
        found = views(records)
        result["records"] = len(found)
        result["view"] = skill_view(found[0]) if found else None
        result["refusalCode"] = refusal_code(result["stderr"])
        return result

    # The same directories with and without a network.
    online = launch("online")
    network = network_control()
    offline = launch("offline", offline=True)
    differences = view_differences(online["view"] or {}, offline["view"] or {})
    # The comparison's own control: a view without its last directory differs.
    mutated = dict(online["view"] or {}, directories=(online["view"] or {}).get("directories", [])[:-1])
    record["offline"] = {"online": online, "offline": offline, "networkControl": network,
                         "differences": differences,
                         "comparisonControl": view_differences(online["view"] or {}, mutated)}
    if network["unguardedExit"] != 0 or network["guardedExit"] == 0:
        failures.append(f"the network control did not separate: {network}")
    if online["records"] != 1 or offline["records"] != 1:
        failures.append(f"the online and offline launches recorded {online['records']} and {offline['records']} "
                        f"launches, not one each")
    elif differences:
        failures.append(f"the offline launch received different {differences}")
    if not record["offline"]["comparisonControl"]:
        failures.append("the comparison of views is not seen to fail on a changed view")

    # A moved candidate-keg source refuses before any worker.
    keg_source = candidate_source(candidate, source_id)
    origin = Path(keg_source["originPath"])
    moved = origin.with_name(origin.name + ".moved-away")
    if moved.exists():
        raise TrialFailure(f"{moved} exists; move it back to {origin} first")
    tree_before = staging_home.tree_digest(origin)
    accepted = keg_source["resourceDigest"]
    os.rename(origin, moved)
    try:
        missing = launch("missing")
    finally:
        os.rename(moved, origin)
    tree_after = staging_home.tree_digest(origin)
    restored = launch("restored")
    record["missingSource"] = {
        "source": source_id, "origin": str(origin), "movedTo": str(moved), "missing": missing,
        "restored": restored, "treeBefore": tree_before, "treeAfter": tree_after,
        "acceptedDigest": accepted, "onlineDigest": digest_of(online["view"], source_id),
        "restoredDigest": digest_of(restored["view"], source_id),
    }
    if missing["refusalCode"] != "skill_source_missing" or missing["records"] != 0:
        failures.append(f"the moved source gave code {missing['refusalCode']!r} and {missing['records']} launch "
                        f"records, not skill_source_missing before any worker")
    if online["records"] != 1 or restored["records"] != 1:
        failures.append("a launch with the source present recorded no launch; the no-worker check is not seen "
                        "to fail")
    if tree_before != tree_after:
        failures.append(f"the tree of {origin} changed across the move")
    for label in ("onlineDigest", "restoredDigest"):
        if record["missingSource"][label] != accepted:
            failures.append(f"{label} of {source_id} is {record['missingSource'][label]}, not the accepted {accepted}")

    record["isolation"] = isolation_problems(before, real_subjects(), home)
    failures += record["isolation"]
    record["failures"] = failures
    record["ended"] = now()
    (out / "record.json").write_text(json.dumps(record, indent=2) + "\n")
    if failures:
        raise TrialFailure("; ".join(failures))
    return record


# --- Model cases --------------------------------------------------------------


def main_events(launch: dict[str, Any]) -> list[dict[str, Any]]:
    transcript = next((t for t in launch["transcripts"] if t["main"]), None)
    if not transcript:
        return []
    return [event for event in transcript["events"] if not event.get("sidechain")]


def native_listing(launch: dict[str, Any]) -> list[str]:
    """The skill names that the harness itself listed at startup."""
    names: list[str] = []
    for transcript in launch["transcripts"]:
        if not transcript["main"]:
            continue
        startup = transcript.get("startup") or {}
        names += list((startup.get("init") or {}).get("skills") or [])
        names += list((startup.get("skillListing") or {}).get("names") or [])
    startup = launch.get("startup") or {}
    names += list((startup.get("init") or {}).get("skills") or [])
    return sorted({name.strip() for name in names if isinstance(name, str) and name.strip()})


def bare(name: str) -> str:
    return name.split(":")[-1]


def mandatory_verdict(events: list[dict[str, Any]], mandatory_skills: set[str]) -> dict[str, Any]:
    """The mandatory file comes before any skill search and any task action.
    A supporting file of a mandatory skill belongs to that read."""
    first = next((i for i, e in enumerate(events) if e.get("class") == "mandatory"), None)
    if first is None:
        return {"passed": False, "reason": "the agent never read the mandatory file"}
    early = [e for e in events[:first] if e.get("class") in GATED
             and not (e.get("class") == "supporting" and e.get("skill") in mandatory_skills)]
    if early:
        shown = [f"#{e['seq']} {e.get('class')} {str(e.get('target'))[:80]}" for e in early]
        return {"passed": False, "reason": f"events before the mandatory read: {shown}", "mandatorySeq": events[first]["seq"]}
    return {"passed": True, "mandatorySeq": events[first]["seq"]}


def reads_of(event: dict[str, Any]) -> list[dict[str, Any]]:
    """Each classified read of one event. One shell command can read several
    files, and the classifier keeps the first as the event's class and the
    rest under `more`. A content read counts as a skill read."""
    first = {key: event.get(key) for key in ("class", "skill", "source", "path")}
    content = [{"class": "skill", **item} for item in event.get("contentReads", [])]
    return [first, *event.get("more", []), *content]


def skill_marks(snapshot: Path, skill: str) -> tuple[str, str] | None:
    """The two lines that show a read of a skill's body: its `name:` line and
    the first heading after its front matter. A listing of front matter
    holds the first line but not the second."""
    files = sorted(snapshot.glob(f"*/{skill}/SKILL.md"))
    if not files:
        return None
    text = files[0].read_text()
    body = text.split("---", 2)[2] if text.startswith("---") else text
    heading = next((line.strip() for line in body.splitlines() if line.startswith("# ")), None)
    return (f"name: {skill}", heading) if heading else None


def mark_content_reads(launch: dict[str, Any], requires: list[dict[str, str]],
                       copies: dict[str, list[str]] | None = None) -> None:
    """Add `contentReads` to each event whose result holds both marks of a
    named skill's snapshot copy. With `copies`, add `copyReads` with each
    label whose distinct lines the result holds. Then replace each full
    result with its digest and length, so the record keeps no copy of a
    skill."""
    roots = {source["id"]: Path(source["snapshotRoot"]) for source in launch.get("sources", [])
             if source.get("snapshotRoot")}
    marks = {(item["source"], item["skill"]): skill_marks(roots[item["source"]], item["skill"])
             for item in requires if item["source"] in roots}
    for transcript in launch["transcripts"]:
        for event in transcript["events"]:
            result = event.pop("resultFull", None)
            if result is None:
                continue
            found = [{"source": source, "skill": skill} for (source, skill), pair in marks.items()
                     if pair and all(mark in result for mark in pair)]
            if found:
                event["contentReads"] = found
            labels = sorted(label for label, lines in (copies or {}).items() if any(line in result for line in lines))
            if labels:
                event["copyReads"] = labels
            event["result"] = {"sha256": harness_trial.sha256_bytes(result.encode()), "chars": len(result)}


def copy_lines(launch: dict[str, Any], spec: dict[str, Any], plant: str) -> dict[str, list[str]]:
    """The distinct lines of the planted stale `SKILL.md` and of Ravel's
    snapshot copy of the same skill."""
    root = next((Path(s["snapshotRoot"]) for s in launch.get("sources", [])
                 if s.get("id") == spec["source"] and s.get("snapshotRoot")), None)
    current = sorted(root.glob(f"*/{spec['skill']}/SKILL.md")) if root else []
    stale = Path(plant) / "SKILL.md"
    if not current or not stale.is_file():
        return {}
    return {"stale": distinct_lines(stale, current[0]), "ravel": distinct_lines(current[0], stale)}


def requires_verdict(events: list[dict[str, Any]], requires: list[dict[str, str]]) -> list[dict[str, Any]]:
    """Each named skill is read from its source before the first edit."""
    action = first_action(events)
    verdicts = []
    for item in requires:
        read = next((e for e in events if any(
            r.get("class") in SKILL_READS and r.get("skill") == item["skill"] and r.get("source") == item["source"]
            for r in reads_of(e))), None)
        verdict: dict[str, Any] = {**item, "readSeq": read["seq"] if read else None,
                                   "actionSeq": action["seq"] if action else None}
        if not read:
            verdict.update(passed=False, reason=f"no read of {item['skill']} from {item['source']}")
        elif not action:
            verdict.update(passed=False, reason="the agent made no edit, so the read precedes no action")
        else:
            verdict.update(passed=read["seq"] < action["seq"],
                           reason=None if read["seq"] < action["seq"] else "the read came after the first edit")
        verdicts.append(verdict)
    return verdicts


def first_action(events: list[dict[str, Any]]) -> dict[str, Any] | None:
    return next((e for e in events if e.get("class") == "action"), None)


# A mention of the planted tree. The extractor counts a relative path only
# when it still exists, and retirement renames the task file.
GROVE_MENTION = re.compile(r"(?<![\w.-])\.grove(?![\w-])")


def first_task_event(events: list[dict[str, Any]]) -> dict[str, Any] | None:
    """The first event that edits a file, names a path in the work tree or
    mentions `.grove`, such as a read of the task file or
    `grove-llm leaf-add .`. A text event is no task event."""
    return next((e for e in events if e.get("class") in ("action", "work")
                 or "work" in e.get("pathClasses", [])
                 or (e.get("class") != "text" and GROVE_MENTION.search(str(e.get("target", ""))))), None)


def kind_verdict(events: list[dict[str, Any]], spec: dict[str, str]) -> dict[str, Any]:
    """The kind skill is read before the first event that touches the work
    tree, and a file of its source other than a `SKILL.md` is read. A Grove
    mandate asks for the kind skill before the task file. It sets no order
    between the task file and the spine's references: the spine's bootstrap
    names the task file, and its references name it too. So the record keeps
    the position of the resource read, and the verdict does not judge it. A
    session can do its task through commands alone."""
    def first(match) -> dict[str, Any] | None:
        return next((e for e in events if any(match(r) for r in reads_of(e))), None)
    skill = first(lambda r: r.get("class") == "kind-skill" and r.get("skill") == spec["skill"]
                  and r.get("source") == spec["source"])
    resource = first(lambda r: r.get("class") == "supporting" and r.get("source") == spec["source"])
    action = first_task_event(events)
    verdict: dict[str, Any] = {**spec, "skillSeq": skill["seq"] if skill else None,
                               "resourceSeq": resource["seq"] if resource else None,
                               "resource": next((r.get("path") for r in reads_of(resource)
                                                 if r.get("class") == "supporting"), None) if resource else None,
                               "taskSeq": action["seq"] if action else None}
    if not skill:
        reason = f"no read of the kind skill {spec['skill']} from {spec['source']}"
    elif not resource:
        reason = f"no read of a resource of {spec['source']} other than a SKILL.md"
    elif not action:
        reason = "the agent never touched the work tree, so the reads precede no task event"
    elif skill["seq"] >= action["seq"]:
        reason = "the kind skill was read at or after the first touch of the work tree"
    else:
        reason = None
    verdict.update(passed=reason is None, reason=reason)
    return verdict


def delegations(launch: dict[str, Any]) -> list[dict[str, Any]]:
    """Each delegate once. A Claude Code session file can hold one assistant
    entry more than once, and the adapter then lists its Agent call twice
    with one `toolUseId`."""
    seen: set[str] = set()
    unique = []
    for delegation in launch.get("delegations", []):
        key = delegation.get("toolUseId")
        if key and key in seen:
            continue
        if key:
            seen.add(key)
        unique.append(delegation)
    return unique


def delegate_verdict(launch: dict[str, Any], spec: dict[str, str]) -> dict[str, Any]:
    """At least one delegate reads a file of the named skill from its
    source."""
    by_file = {t["file"]: t for t in launch.get("transcripts", [])}
    delegates = []
    for delegation in delegations(launch):
        transcript = by_file.get(delegation.get("transcript") or "")
        events = transcript["events"] if transcript else []
        read = next((e for e in events if any(
            r.get("class") in RESOURCE_READS and r.get("skill") == spec["skill"] and r.get("source") == spec["source"]
            for r in reads_of(e))), None)
        delegates.append({"description": delegation.get("description"), "transcript": delegation.get("transcript"),
                          "events": len(events), "readSeq": read["seq"] if read else None,
                          "readPath": (read.get("path") or read.get("target")) if read else None})
    verdict: dict[str, Any] = {**spec, "delegates": delegates}
    if not delegates:
        reason = "the parent started no delegate"
    elif not any(d["transcript"] for d in delegates):
        reason = "no delegate transcript was found"
    elif not any(d["readSeq"] for d in delegates):
        reason = f"no delegate read a file of {spec['skill']} from {spec['source']}"
    else:
        reason = None
    verdict.update(passed=reason is None, reason=reason)
    return verdict


def compaction_verdict(events: list[dict[str, Any]], spec: dict[str, Any]) -> dict[str, Any]:
    """A compaction boundary exists, and after it the agent reads the named
    skill from its source before its first edit after the boundary."""
    after = spec["after"]
    boundary = next((i for i, e in enumerate(events) if e.get("class") == "compaction"), None)
    verdict: dict[str, Any] = {**after, "boundarySeq": events[boundary]["seq"] if boundary is not None else None}
    if boundary is None:
        verdict.update(passed=False, reason="the session holds no compaction boundary")
        return verdict
    def reads(event: dict[str, Any]) -> bool:
        return any(r.get("class") in SKILL_READS and r.get("skill") == after["skill"]
                   and r.get("source") == after["source"] for r in reads_of(event))
    before = next((e for e in events[:boundary] if reads(e)), None)
    rest = events[boundary + 1:]
    read = next((e for e in rest if reads(e)), None)
    action = first_action(rest)
    verdict.update(readBeforeSeq=before["seq"] if before else None, readSeq=read["seq"] if read else None,
                   actionSeq=action["seq"] if action else None,
                   mandatoryAfterSeq=next((e["seq"] for e in rest if e.get("class") == "mandatory"), None))
    if not read:
        reason = f"no read of {after['skill']} from {after['source']} after the compaction"
    elif not action:
        reason = "the agent made no edit after the compaction, so the read precedes no action"
    elif read["seq"] >= action["seq"]:
        reason = "the read after the compaction came after the first edit after it"
    else:
        reason = None
    verdict.update(passed=reason is None, reason=reason)
    return verdict


# A path of a planted stale copy, relative or absolute.
def stale_pattern(skill: str) -> re.Pattern[str]:
    roots = "|".join(re.escape(root) for root in PROJECT_ROOTS.values())
    return re.compile(rf"(?:{roots})/{re.escape(skill)}(?![\w-])")


def stale_verdict(events: list[dict[str, Any]], spec: dict[str, Any], listed: list[str]) -> dict[str, Any]:
    """Which copy the agent reads. A stale read names a path of a planted
    copy, loads the name through the native Skill tool, or returns a line
    that only the stale `SKILL.md` holds. A Ravel read is a classified read
    from the named source, or returns a line that only Ravel's copy holds."""
    skill, pattern = spec["skill"], stale_pattern(spec["skill"])
    real = str(staging_home.real_home())
    def stale(event: dict[str, Any]) -> bool:
        target = str(event.get("target", ""))
        by_path = bool(pattern.search(target)) and not any(f"{real}/{root}" in target for root in HOME_ROOTS)
        native = event.get("class") == "native-skill-tool" and bare(target) == skill and not event.get("error")
        return by_path or native or "stale" in event.get("copyReads", [])
    def ravel(event: dict[str, Any]) -> bool:
        return "ravel" in event.get("copyReads", []) or any(
            r.get("class") in RESOURCE_READS and r.get("skill") == skill and r.get("source") == spec["source"]
            for r in reads_of(event))
    stale_seqs = [e["seq"] for e in events if stale(e)]
    ravel_seqs = [e["seq"] for e in events if ravel(e)]
    copy = {(True, True): "both", (True, False): "stale", (False, True): "ravel"}.get(
        (bool(stale_seqs), bool(ravel_seqs)))
    verdict: dict[str, Any] = {"source": spec["source"], "skill": skill, "staleSeqs": stale_seqs,
                               "ravelSeqs": ravel_seqs, "copy": copy,
                               "first": None if not copy else ("stale" if not ravel_seqs or (
                                   stale_seqs and stale_seqs[0] < ravel_seqs[0]) else "ravel"),
                               "listed": skill in {bare(name) for name in listed}}
    if not verdict["listed"]:
        reason = f"the native listing does not hold the planted {skill}, so no copy coexisted"
    elif not copy:
        reason = f"the agent read neither copy of {skill}"
    else:
        reason = None
    verdict.update(passed=reason is None, reason=reason)
    return verdict


def judge(case: dict[str, Any], launches: list[dict[str, Any]], superseded: set[str]) -> dict[str, Any]:
    failures: list[str] = []
    if len(launches) != 1:
        return {"failures": [f"the case recorded {len(launches)} launches, not one"]}
    launch = launches[0]
    events = main_events(launch)
    mandatory_skills = {Path(item["path"]).parent.name for item in launch.get("mandatory", [])}
    verdict: dict[str, Any] = {}
    verdict["mandatoryFirst"] = mandatory_verdict(events, mandatory_skills)
    verdict["mandatoryFirst"]["enforced"] = bool(case.get("mandatoryFirst"))
    if case.get("mandatoryFirst") and not verdict["mandatoryFirst"]["passed"]:
        failures.append(f"mandatory first: {verdict['mandatoryFirst']['reason']}")
    verdict["requires"] = requires_verdict(events, case.get("requires", []))
    failures += [f"requires {v['skill']}: {v['reason']}" for v in verdict["requires"] if not v["passed"]]
    if "kindSkill" in case:
        verdict["kindSkill"] = kind_verdict(events, case["kindSkill"])
        if not verdict["kindSkill"]["passed"]:
            failures.append(f"kind skill: {verdict['kindSkill']['reason']}")
    if "delegate" in case:
        verdict["delegate"] = delegate_verdict(launch, case["delegate"])
        if not verdict["delegate"]["passed"]:
            failures.append(f"delegate: {verdict['delegate']['reason']}")
    if "compaction" in case:
        verdict["compaction"] = compaction_verdict(events, case["compaction"])
        if not verdict["compaction"]["passed"]:
            failures.append(f"compaction: {verdict['compaction']['reason']}")
    listed = native_listing(launch)
    verdict["nativeListing"] = listed
    planted = {case["staleCopy"]["skill"]} if "staleCopy" in case else set()
    verdict["listedSuperseded"] = sorted({bare(name) for name in listed} & superseded - planted)
    if verdict["listedSuperseded"]:
        failures.append(f"native discovery listed superseded skills {verdict['listedSuperseded']}")
    if "staleCopy" in case:
        verdict["staleCopy"] = stale_verdict(events, case["staleCopy"], listed)
        if not verdict["staleCopy"]["passed"]:
            failures.append(f"stale copy: {verdict['staleCopy']['reason']}")
    failures += launch.get("control", [])
    verdict["failures"] = failures
    return verdict


def render(record: dict[str, Any]) -> str:
    lines = [f"# Staging case {record['case']} on {record['harness']['name']}", "",
             (f"- Harness: {record['harness']['version']}, model {record['harness']['model']}, "
              f"effort {record['harness']['effort']}"),
             (f"- Started {record['started']}, ended {record['ended']}, Ravel exit {record['launch']['exit']}, "
              f"result {record['launch']['result']!r}"),
             f"- Failures: {record['verdict'].get('failures') or 'none'}"]
    for key in ("compaction", "staleCopy"):
        if key in record["verdict"]:
            shown = {k: v for k, v in record["verdict"][key].items() if k not in ("source", "skill")}
            lines.append(f"- {key}: {json.dumps(shown)}")
    for context in record.get("compactionContext") or []:
        after = context.get("afterBoundary") or {}
        lines.append(f"- Context after compaction: {len(after.get('paths', []))} preamble paths, "
                     f"{len(after.get('lines', []))} preamble lines, variable named: {after.get('variable')}")
    lines += ["", "| Seq | Tool | Class | Skill | Source | Target |", "| --- | --- | --- | --- | --- | --- |"]
    def rows(events: list[dict[str, Any]]) -> None:
        for event in events:
            target = str(event.get("target", "")).replace("|", "\\|").replace("\n", " ")[:110]
            skill = event.get("skill") or ""
            source = event.get("source") or ""
            for item in event.get("contentReads", []):
                skill, source = f"{skill} {item['skill']} (content)".strip(), f"{source} {item['source']}".strip()
            for label in event.get("copyReads", []):
                skill = f"{skill} {label} copy (content)".strip()
            lines.append(f"| {event['seq']} | {event.get('tool')} | {event.get('class')} | {skill} "
                         f"| {source} | `{target}` |")
    for launch in record["launches"]:
        rows(main_events(launch))
        by_file = {t["file"]: t for t in launch.get("transcripts", [])}
        for delegation in delegations(launch):
            transcript = by_file.get(delegation.get("transcript") or "")
            lines += ["", f"Delegate {delegation.get('description')!r}, transcript `{delegation.get('transcript')}`",
                      "", "| Seq | Tool | Class | Skill | Source | Target |", "| --- | --- | --- | --- | --- | --- |"]
            rows(transcript["events"] if transcript else [])
    return "\n".join(lines) + "\n"


def write_turns(out: Path, adapter: Any, spec: dict[str, Any]) -> dict[str, Any]:
    """The feeder's plan, beside the records directory, where
    `harness_trial.feeder_route` names it: the harness's default compaction,
    then the answer. No turn has a gate, because the case measures the
    harness's own compaction, not a chosen loss."""
    compact = adapter.compact_turn({"instructions": ""})
    if "text" in compact:
        compact = {"text": compact["text"].strip()}
    plan = {"harness": adapter.name, "turns": [compact, {"text": spec["answer"]}]}
    (out / "turns.json").write_text(json.dumps(plan, indent=2) + "\n")
    return plan


def compaction_context(adapter: Any, record: Path, launch: dict[str, Any], home: Path) -> dict[str, Any]:
    """What the context after the last compaction still holds of Ravel's
    preamble: its paths, digests, long lines and the descriptor variable.
    The feeder's turns come from its `trial-turn` lines."""
    prompt = (record / "prompt").read_text() if (record / "prompt").exists() else ""
    stderr = (record / "stderr").read_text(errors="replace") if (record / "stderr").exists() else ""
    marks = harness_trial.preamble_marks(prompt)
    path = adapter.session_file(home, launch["sessionId"]) if launch.get("sessionId") else None
    facts, _ = adapter.compaction_facts(harness_trial.read_jsonl(path) if path else [], marks,
                                        DEFAULT_COMPACTION["mode"])
    return {"turns": harness_trial.trial_lines(stderr, "turn"),
            "preamble": {"paths": len(marks["paths"]), "digests": len(marks["digests"]), "lines": len(marks["lines"])},
            **facts}


def extract_and_judge(case: dict[str, Any], adapter: Any, records: Path, home: Path, work: Path,
                      environ: dict[str, str], stale: dict[str, Any] | None
                      ) -> tuple[list[dict[str, Any]], dict[str, Any], list[dict[str, Any]] | None]:
    """The launches of a case from its records, its verdict and, for a
    compaction case, the context after each compaction."""
    turns = "compaction" in case
    paths = harness_trial.launch_records(records)
    launches = [harness_trial.describe_launch(adapter, path, home, work, environ, turns=turns) for path in paths]
    for described in launches:
        copies = copy_lines(described, case["staleCopy"], stale["plants"][adapter.name]) if stale else None
        mark_content_reads(described, named_skills(case), copies)
    context = [compaction_context(adapter, path, described, home)
               for path, described in zip(paths, launches)] if turns else None
    return launches, judge(case, launches, staging_home.superseded_names()), context


def run_case(name: str, harness: str, home: Path, ravel: Path, out: Path, environ: dict[str, str],
             model: str | None = None, effort: str | None = None) -> dict[str, Any]:
    case = load_case(name)
    staging_home.refuse_inherited(environ, home)
    inputs = check_inputs(home, ravel)
    if out.exists():
        raise TrialFailure(f"{out} exists; name a new directory")
    out.mkdir(parents=True)
    adapter = harness_trial.adapter_named(harness)
    env = staging_home.staged_env(home, ravel)
    found = shutil.which(harness, path=env["PATH"])
    if not found:
        raise TrialFailure(f"no {harness} on the staging PATH {env['PATH']}")
    program = Path(found)
    defaults = {"claude": (staging_home.CLAUDE_MODEL, staging_home.CLAUDE_EFFORT),
                "codex": (staging_home.CODEX_MODEL, staging_home.CODEX_EFFORT)}[harness]
    model, effort = model or defaults[0], effort or defaults[1]
    records = out / "records"
    records.mkdir()
    compaction = case.get("compaction")
    route = {"wrapper": str(harness_trial.WRAPPER), "records": str(records),
             **adapter.launch({"compaction": DEFAULT_COMPACTION} if compaction else {}, records, program, model,
                              effort),
             "model": model, "effort": effort}
    if compaction:
        write_turns(out, adapter, compaction)
    config = write_config(out, "trial", case["kind"], route)
    work = staging_home.work_directory(home)
    stale = None
    if "staleCopy" in case:
        stale = plant_stale(work, case["staleCopy"])
        stale["homeBefore"] = home_copies(home, case["staleCopy"]["skill"])
    tree = plant_tree(work, case["tree"], env) if "tree" in case else None
    version = subprocess.run([str(program), "--version"], env=env, cwd=work, capture_output=True, text=True,
                             check=False).stdout.strip().splitlines()[0]
    before = real_subjects()
    launch = start(ravel, config, case["kind"], case["prompt"], work, env, case.get("timeoutMinutes", 20) * 60)
    (out / "ravel.stderr").write_text(launch["stderr"])
    isolation = isolation_problems(before, real_subjects(), home)
    if stale is not None:
        stale["homeAfter"] = home_copies(home, case["staleCopy"]["skill"])
        isolation += [f"the staging HOME holds a copy of {case['staleCopy']['skill']} at {path} {when}"
                      for when in ("before", "after") for path in stale[f"home{when.title()}"]]
    launches, verdict, context = extract_and_judge(case, adapter, records, home, work, environ, stale)
    verdict["isolation"] = isolation
    verdict["failures"] = verdict.get("failures", []) + isolation
    record = {
        "schemaVersion": 1, "case": name, "purpose": case.get("purpose"), "started": launch["started"],
        "ended": launch["ended"], "home": str(home), "work": str(work), "tree": tree, "inputs": inputs,
        "harness": {"name": harness, "program": str(program), "version": version, "model": model, "effort": effort},
        "config": config.read_text(), "environment": sorted(env), "launch": launch, "launches": launches,
        "stale": stale, "compactionContext": context, "verdict": verdict,
    }
    (out / "record.json").write_text(json.dumps(record, indent=2) + "\n")
    (out / "record.md").write_text(render(record))
    if verdict["failures"]:
        raise TrialFailure("; ".join(verdict["failures"]))
    return record


def rejudge(trial: Path, environ: dict[str, str]) -> dict[str, Any]:
    """Extract and judge a finished case again, from its unchanged launch
    records and the session files in the staging HOME. The isolation result
    of the original run stays, because a later reading cannot repeat it."""
    record = json.loads((trial / "record.json").read_text())
    case = load_case(record["case"])
    home, work = Path(record["home"]), Path(record["work"])
    adapter = harness_trial.adapter_named(record["harness"]["name"])
    launches, verdict, context = extract_and_judge(case, adapter, trial / "records", home, work, environ,
                                                   record.get("stale"))
    verdict["isolation"] = record["verdict"].get("isolation", [])
    verdict["failures"] = verdict.get("failures", []) + verdict["isolation"]
    record.update(launches=launches, verdict=verdict, compactionContext=context, rejudged=now())
    (trial / "record.json").write_text(json.dumps(record, indent=2) + "\n")
    (trial / "record.md").write_text(render(record))
    if verdict["failures"]:
        raise TrialFailure("; ".join(verdict["failures"]))
    return record


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    sub = parser.add_subparsers(dest="command", required=True)
    s = sub.add_parser("sources", help="run the trials that launch no model")
    s.add_argument("--home", type=Path, required=True)
    s.add_argument("--candidate", type=Path, required=True)
    s.add_argument("--ravel", type=Path, required=True)
    s.add_argument("--out", type=Path, required=True)
    s.add_argument("--source", default="research", help="the candidate-keg source to move away")
    c = sub.add_parser("case", help="run one model case on one harness")
    c.add_argument("--name", required=True)
    c.add_argument("--harness", choices=("claude", "codex"), required=True)
    c.add_argument("--home", type=Path, required=True)
    c.add_argument("--ravel", type=Path, required=True)
    c.add_argument("--out", type=Path, required=True)
    c.add_argument("--model")
    c.add_argument("--effort")
    r = sub.add_parser("rejudge", help="extract and judge a finished case again, without a launch")
    r.add_argument("--out", type=Path, required=True, help="the trial directory of the case")
    args = parser.parse_args(argv)
    environ = dict(os.environ)
    try:
        if args.command == "sources":
            sources(args.home.resolve(), args.candidate.resolve(), args.ravel.resolve(), args.out.resolve(),
                    args.source, environ)
        elif args.command == "rejudge":
            rejudge(args.out.resolve(), environ)
        else:
            run_case(args.name, args.harness, args.home.resolve(), args.ravel.resolve(), args.out.resolve(), environ,
                     args.model, args.effort)
    except (TrialFailure, staging_home.StagingError, harness_trial.TrialError) as error:
        print(f"staging trial failed: {error}", file=sys.stderr)
        print(f"  record: {args.out}/record.json", file=sys.stderr)
        return 1
    print(f"staging trial passed: {args.out}/record.json")
    return 0


if __name__ == "__main__":
    sys.exit(main())
