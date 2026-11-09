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
- The native skill listing of the session holds no superseded name.
- No real-HOME subject changed, and the real `.claude.json` names no
  staging path.

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
import json
import os
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

CASES = HERE / "staging-cases"
OFF_SETTINGS = ROOT / "docs" / "owner-cutover" / "settings-off.json"
CASE_FIELDS = {"schemaVersion", "name", "purpose", "kind", "prompt", "mandatoryFirst", "requires",
               "timeoutMinutes"}
# The fields that later leaves of owner-staging-discovery-k249 add, each with
# its leaf.
LATER_FIELDS = {
    "delegate": "staging-kind-delegate-trials-k276",
    "kindSkill": "staging-kind-delegate-trials-k276",
    "compaction": "staging-compaction-stale-trials-k277",
    "staleCopy": "staging-compaction-stale-trials-k277",
}
# The classes of an event that looks for a skill or acts on the task. The
# mandatory obligation of Ravel's discovery instructions comes before each.
GATED = {"listing", "skill", "kind-skill", "supporting", "native", "planted", "source-origin", "action",
         "command", "work", "delegate", "native-skill-tool"}
SKILL_READS = {"skill", "kind-skill"}
# Real-HOME subjects that the owner's own running session rewrites. The
# check reads them for staging paths instead of comparing digests.
LIVE_SUBJECTS = {".claude.json"}
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
        if field in LATER_FIELDS:
            raise TrialFailure(f"case {name}: the runner has no verdict for '{field}' yet ({LATER_FIELDS[field]})")
        if field not in CASE_FIELDS:
            raise TrialFailure(f"case {name}: unknown field '{field}'; the fields are {sorted(CASE_FIELDS)}")
    if case.get("name") != name:
        raise TrialFailure(f"case {name}: its name field is {case.get('name')!r}")
    for item in case.get("requires", []):
        if set(item) != {"source", "skill"}:
            raise TrialFailure(f"case {name}: each entry of requires is {{source, skill}}, not {item!r}")
    return case


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


def real_subjects() -> dict[str, Any]:
    real = staging_home.real_home()
    return {rel: staging_home.tree_digest(real / rel) for rel in staging_home.REAL_SUBJECTS
            if rel not in LIVE_SUBJECTS}


def isolation_problems(before: dict[str, Any], after: dict[str, Any], home: Path) -> list[str]:
    problems = [f"the real {rel} changed" for rel in sorted(before) if before[rel] != after.get(rel)]
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


def mark_content_reads(launch: dict[str, Any], requires: list[dict[str, str]]) -> None:
    """Add `contentReads` to each event whose result holds both marks of a
    required skill's snapshot copy. Then replace each full result with its
    digest and length, so the record keeps no copy of a skill."""
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
            event["result"] = {"sha256": harness_trial.sha256_bytes(result.encode()), "chars": len(result)}


def requires_verdict(events: list[dict[str, Any]], requires: list[dict[str, str]]) -> list[dict[str, Any]]:
    """Each named skill is read from its source before the first edit."""
    action = next((e for e in events if e.get("class") == "action"), None)
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


def judge(case: dict[str, Any], launches: list[dict[str, Any]], superseded: set[str]) -> dict[str, Any]:
    failures: list[str] = []
    if len(launches) != 1:
        return {"failures": [f"the case recorded {len(launches)} launches, not one"]}
    launch = launches[0]
    events = main_events(launch)
    mandatory_skills = {Path(item["path"]).parent.name for item in launch.get("mandatory", [])}
    verdict: dict[str, Any] = {}
    if case.get("mandatoryFirst"):
        verdict["mandatoryFirst"] = mandatory_verdict(events, mandatory_skills)
        if not verdict["mandatoryFirst"]["passed"]:
            failures.append(f"mandatory first: {verdict['mandatoryFirst']['reason']}")
    verdict["requires"] = requires_verdict(events, case.get("requires", []))
    failures += [f"requires {v['skill']}: {v['reason']}" for v in verdict["requires"] if not v["passed"]]
    listed = native_listing(launch)
    verdict["nativeListing"] = listed
    verdict["listedSuperseded"] = sorted({bare(name) for name in listed} & superseded)
    if verdict["listedSuperseded"]:
        failures.append(f"native discovery listed superseded skills {verdict['listedSuperseded']}")
    failures += launch.get("control", [])
    verdict["failures"] = failures
    return verdict


def render(record: dict[str, Any]) -> str:
    lines = [f"# Staging case {record['case']} on {record['harness']['name']}", "",
             (f"- Harness: {record['harness']['version']}, model {record['harness']['model']}, "
              f"effort {record['harness']['effort']}"),
             (f"- Started {record['started']}, ended {record['ended']}, Ravel exit {record['launch']['exit']}, "
              f"result {record['launch']['result']!r}"),
             f"- Failures: {record['verdict'].get('failures') or 'none'}", "",
             "| Seq | Tool | Class | Skill | Source | Target |", "| --- | --- | --- | --- | --- | --- |"]
    for launch in record["launches"]:
        for event in main_events(launch):
            target = str(event.get("target", "")).replace("|", "\\|").replace("\n", " ")[:110]
            skill = event.get("skill") or ""
            source = event.get("source") or ""
            for item in event.get("contentReads", []):
                skill, source = f"{skill} {item['skill']} (content)".strip(), f"{source} {item['source']}".strip()
            lines.append(f"| {event['seq']} | {event.get('tool')} | {event.get('class')} | {skill} "
                         f"| {source} | `{target}` |")
    return "\n".join(lines) + "\n"


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
    route = {"wrapper": str(harness_trial.WRAPPER), "records": str(records),
             **adapter.launch({}, records, program, model, effort), "model": model, "effort": effort}
    config = write_config(out, "trial", case["kind"], route)
    work = staging_home.work_directory(home)
    version = subprocess.run([str(program), "--version"], env=env, cwd=work, capture_output=True, text=True,
                             check=False).stdout.strip().splitlines()[0]
    before = real_subjects()
    launch = start(ravel, config, case["kind"], case["prompt"], work, env, case.get("timeoutMinutes", 20) * 60)
    (out / "ravel.stderr").write_text(launch["stderr"])
    isolation = isolation_problems(before, real_subjects(), home)
    launches = [harness_trial.describe_launch(adapter, record, home, work, environ)
                for record in harness_trial.launch_records(records)]
    for described in launches:
        mark_content_reads(described, case.get("requires", []))
    verdict = judge(case, launches, staging_home.superseded_names())
    verdict["isolation"] = isolation
    verdict["failures"] = verdict.get("failures", []) + isolation
    record = {
        "schemaVersion": 1, "case": name, "purpose": case.get("purpose"), "started": launch["started"],
        "ended": launch["ended"], "home": str(home), "work": str(work), "inputs": inputs,
        "harness": {"name": harness, "program": str(program), "version": version, "model": model, "effort": effort},
        "config": config.read_text(), "environment": sorted(env), "launch": launch, "launches": launches,
        "verdict": verdict,
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
    launches = [harness_trial.describe_launch(adapter, path, home, work, environ)
                for path in harness_trial.launch_records(trial / "records")]
    for described in launches:
        mark_content_reads(described, case.get("requires", []))
    verdict = judge(case, launches, staging_home.superseded_names())
    verdict["isolation"] = record["verdict"].get("isolation", [])
    verdict["failures"] = verdict.get("failures", []) + verdict["isolation"]
    record.update(launches=launches, verdict=verdict, rejudged=now())
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
