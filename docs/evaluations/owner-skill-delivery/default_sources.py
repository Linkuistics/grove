"""Verify the installed skill formulae through Ravel's shared default directory.

"The keg layout" of docs/specs/codex-skill-provisioning.md and "Skill
formulae" of docs/specs/owner-skill-delivery.md own the rules. This tool
reads only. It changes no keg, link, tap or HOME.

Ravel 2.0 resolves the default skill directory of the real Homebrew prefix
in a disposable HOME whose settings name no sources. For each source of each
checked formula, the tool compares:

- the resolved source ID with the plan's accepted IDs;
- the `resourceDigest` with the accepted digest;
- the canonical `originPath` with `libexec/skill-sources/<id>` in the keg
  that Homebrew links for the formula;
- for a non-Grove formula, the linked keg with the plan's accepted full
  package version.

RAVEL_PREFIX is an installation prefix of Ravel 2.0. Ravel finds the default
skill directory through the installation record `share/ravel/installation.json`
beside its canonical executable ("Default skill directory" of Ravel's skill
discovery specification). A Homebrew keg of Ravel has that record. A prefix of
recorded archives, as `scripts/ravel_target_inputs.py` prepares, has none. For
such a prefix, the tool copies `bin/ravel` into a temporary prefix with a
record that names `$(brew --prefix)/share/ravel/skills`.

Exit 0: each checked source matches. Exit 1: a difference, printed on stderr.
Exit 2: a usage or input error.
"""

import argparse
import json
import os
import shutil
import subprocess
import sys
import tempfile

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))
PLAN = os.path.join(ROOT, "docs", "owner-cutover", "switch-plan-draft.json")
GROVE = "linkuistics/taps/grove"


class InputError(Exception):
    pass


def run(argv, env=None, cwd=None) -> str:
    result = subprocess.run(argv, capture_output=True, text=True, env=env, cwd=cwd, check=False)
    if result.returncode != 0:
        raise InputError(f"{' '.join(argv)} exited {result.returncode}: {result.stderr.strip()[:2000]}")
    return result.stdout


def load_plan(path: str):
    with open(path, encoding="utf-8") as f:
        plan = json.load(f)
    accepted = plan.get("accepted") if isinstance(plan, dict) else None
    sources = accepted.get("sources") if isinstance(accepted, dict) else None
    versions = accepted.get("versions") if isinstance(accepted, dict) else None
    if not isinstance(sources, dict) or not isinstance(versions, dict):
        raise InputError(f"{path} has no accepted.sources and accepted.versions")
    for sid, source in sources.items():
        if not (isinstance(source, dict) and isinstance(source.get("formula"), str)
                and isinstance(source.get("resourceDigest"), str)):
            raise InputError(f"{path}: the accepted source {sid} needs a formula and a resourceDigest")
    return sources, versions


def linked_keg(cellar: str, formula: str) -> str:
    """The keg that Homebrew links for FORMULA, from `brew info --json=v2`."""
    info = json.loads(run(["brew", "info", "--json=v2", formula]))
    try:
        entry = info["formulae"][0]
    except (KeyError, IndexError, TypeError):
        raise InputError(f"brew info --json=v2 {formula} has no formula")
    linked = entry.get("linked_keg")
    if not isinstance(linked, str) or not linked:
        return ""
    return os.path.join(cellar, entry["name"], linked)


def ravel_executable(prefix: str, skills: str, scratch: str) -> str:
    """bin/ravel of PREFIX, or a copy of it in a prefix whose record names SKILLS."""
    ravel = os.path.join(prefix, "bin", "ravel")
    if not os.access(ravel, os.X_OK):
        raise InputError(f"{ravel} is not an executable")
    record = os.path.join(os.path.dirname(os.path.dirname(os.path.realpath(ravel))),
                          "share", "ravel", "installation.json")
    if os.path.isfile(record):
        with open(record, encoding="utf-8") as f:
            named = json.load(f).get("skillDirectory")
        if named != skills:
            raise InputError(f"{record} names {named}, not {skills}")
        return ravel
    own = os.path.join(scratch, "ravel-prefix")
    os.makedirs(os.path.join(own, "bin"))
    os.makedirs(os.path.join(own, "share", "ravel"))
    shutil.copy2(ravel, os.path.join(own, "bin", "ravel"))
    with open(os.path.join(own, "share", "ravel", "installation.json"), "w", encoding="utf-8") as f:
        json.dump({"schemaVersion": 1, "skillDirectory": skills}, f)
    return os.path.join(own, "bin", "ravel")


def resolve(ravel: str, scratch: str) -> dict:
    """`ravel skills resolve --json` in a disposable HOME without settings."""
    home = os.path.join(scratch, "home")
    os.makedirs(home)
    # Ravel reads its settings and cache from HOME. No RAVEL_* variable may
    # steer the resolution.
    env = {k: v for k, v in os.environ.items() if not k.startswith("RAVEL_")}
    env["HOME"] = home
    version = run([ravel, "--version"], env, home).strip()
    result = subprocess.run([ravel, "skills", "resolve", "--json"], capture_output=True, text=True,
                            env=env, cwd=home, check=False)
    try:
        doc = json.loads(result.stdout)
    except json.JSONDecodeError:
        raise InputError(f"ravel skills resolve --json exited {result.returncode} without JSON: "
                         f"{result.stderr.strip()[:2000]}")
    return {"ravel": ravel, "version": version, "exit": result.returncode, "doc": doc}


def check(sources: dict, versions: dict, formulae: list, resolution: dict, cellar: str) -> list:
    """Each difference between the resolution and the accepted values, as a line."""
    doc = resolution["doc"]
    if not isinstance(doc, dict) or doc.get("status") != "resolved":
        return [f"the default directory did not resolve: {json.dumps(doc)[:2000]}"]
    got = {s.get("id"): s for s in doc.get("sources") or [] if isinstance(s, dict)}
    problems = []
    for sid in sorted(set(got) - set(sources)):
        problems.append(f"{sid} resolves, and the plan accepts no such source")
    for formula in formulae:
        keg = linked_keg(cellar, formula)
        if not keg:
            problems.append(f"Homebrew links no keg of {formula}")
            continue
        wanted = versions.get(formula)
        if formula != GROVE and os.path.basename(keg) != wanted:
            problems.append(f"{formula} links {os.path.basename(keg)}, not its accepted version {wanted}")
        for sid in sorted(s for s, v in sources.items() if v["formula"] == formula):
            source = got.get(sid)
            if source is None:
                problems.append(f"{sid} of {formula} does not resolve")
                continue
            accepted = sources[sid]["resourceDigest"]
            if source.get("resourceDigest") != accepted:
                problems.append(f"{sid} gives {source.get('resourceDigest')}, not its accepted {accepted}")
            payload = os.path.join(keg, "libexec", "skill-sources", sid)
            origin = source.get("originPath")
            if not isinstance(origin, str) or os.path.realpath(origin) != os.path.realpath(payload):
                problems.append(f"{sid} comes from {origin}, not from the private payload {payload}")
    return problems


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("ravel_prefix", metavar="RAVEL_PREFIX", help="an installation prefix of Ravel 2.0")
    parser.add_argument("--plan", default=PLAN, help="the plan whose accepted values to compare")
    parser.add_argument("--formula", action="append", default=[],
                        help="a formula to check, such as linkuistics/taps/personal; repeatable. "
                             "The default is every formula of the plan")
    parser.add_argument("--json", action="store_true", help="print the resolved sources as JSON")
    args = parser.parse_args()
    try:
        sources, versions = load_plan(args.plan)
        known = sorted({s["formula"] for s in sources.values()})
        formulae = args.formula or known
        unknown = sorted(set(formulae) - set(known))
        if unknown:
            raise InputError(f"the plan accepts no source of {', '.join(unknown)}")
        prefix = run(["brew", "--prefix"]).strip()
        cellar = run(["brew", "--cellar"]).strip()
        skills = os.path.join(prefix, "share", "ravel", "skills")
        with tempfile.TemporaryDirectory(prefix="default-sources-") as scratch:
            ravel = ravel_executable(args.ravel_prefix, skills, scratch)
            resolution = resolve(ravel, scratch)
        problems = check(sources, versions, formulae, resolution, cellar)
    except (InputError, OSError, json.JSONDecodeError) as error:
        print(f"default-sources: {error}", file=sys.stderr)
        return 2
    doc = resolution["doc"]
    resolved = [s for s in (doc.get("sources") or []) if isinstance(s, dict)] if isinstance(doc, dict) else []
    if args.json:
        print(json.dumps({"ravel": resolution["version"], "directory": skills, "formulae": formulae,
                          "sources": [{k: s.get(k) for k in ("id", "resourceDigest", "originPath")}
                                      for s in resolved]}, indent=1))
    else:
        print(f"{resolution['version']} resolved {skills}")
        for s in resolved:
            print(f"{s.get('id')}\t{s.get('resourceDigest')}\t{s.get('originPath')}")
    for line in problems:
        print(f"default-sources: {line}", file=sys.stderr)
    if problems:
        return 1
    print(f"default-sources: each source of {', '.join(formulae)} matches its accepted ID, digest, "
          "private payload and version", file=sys.stderr)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
