"""Check the owner's drafted Ravel skill settings with the accepted 2.0 build.

The drafts are in docs/owner-cutover. "Owner settings drafts" of
docs/owner-skill-delivery-evidence.md states them, and the sections "Skill
formulae", "Mandatory guidance", "Settings isolation and the staging HOME" and
"The native settings inside finish" of docs/specs/owner-skill-delivery.md own
their content.

The script copies the accepted build into the keg Cellar/ravel@2/2.0.0 of a
fixture Homebrew prefix in OUT and writes its installation record. The record
names the prefix's share/ravel/skills, the default skill directory. That
directory holds one link for each of the ten sources. Each fixture source is
a keg in the projected layout of "The keg layout" in
docs/specs/codex-skill-provisioning.md, under the prefix of its staging path,
moved under OUT/fixture. Its private payload libexec/skill-sources/<id> holds
its roots, one probe skill in each root, and for superpowers the mandatory
file. The keg's share/ravel/skills/<id> is the alias of that payload. The
staging path and the default directory each link the alias, as Homebrew does.
Each fixture keg has its accepted full package version.

Each case runs `ravel skills resolve --json` under `env -i` with a new HOME in
OUT/homes. The command launches no harness, and PATH holds only /usr/bin and
/bin. The cases are these:

- no-settings: without settings, the build resolves the ten default sources.
  This shows that the fixture directory supplies them.
- off: the off value gives `disabled`, and the HOME then holds no cache.
- switched: the switched value resolves the ten default sources and the
  mandatory file. The file in the snapshot has the bytes of the fixture file.
- staging: the staging settings, with each path moved under OUT/fixture,
  resolve exactly the ten sources, in byte order, with the mandatory file.
- staging-as-is: the unchanged staging settings pass the settings checks. The
  result is `resolved`, or `skill_source_missing` for a source path of the
  settings that is absent, while a keg is absent.

In no-settings, switched and staging, each source's originPath must be its
private payload, because Ravel canonicalises the alias.

Three controls must refuse. An off value with a mandatory reference refuses as
settings_invalid. A switched value whose mandatory path names no file refuses
as skill_mandatory_invalid. Staging settings with a repeated source refuse as
settings_invalid. A fourth control replaces the default directory's entry of
personal with a real directory, as the earlier layout had. Its resolution
must report that personal does not come from its private payload.

The script also reads bootstrap-packages.toml. It must declare exactly the five
formulae of "Skill formulae", each at "latest". Homebrew must find each
formula in the local tap, without an update. The record names each tap
formula's full package version, with its revision.

The real HOME's ~/.config/ravel/settings.json must be absent or unchanged after
the run.

Usage:
    python3 -I settings_drafts.py --out DIR

OUT must not exist. The script writes OUT/record.json with the digest of each
draft and of the build, and the result of each case. Ravel publishes each
snapshot read-only, so remove OUT with `chmod -R u+w OUT && rm -rf OUT`. Exit
status 0 means that every case passed.
"""

import argparse, glob, hashlib, json, os, shutil, subprocess, sys, tarfile, tomllib

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", ".."))
DRAFTS = os.path.join(ROOT, "docs", "owner-cutover")
TEN = sorted(["grove-core", "requirements", "design", "engineering", "research", "writing",
              "personal", "testanyware", "superpowers", "quint"])
MANDATORY = {"source": "superpowers", "path": "skills/using-superpowers/SKILL.md"}
FORMULAE = ["grove", "personal", "testanyware", "superpowers", "quint-llm-kit"]
# The keg of each source: its formula and accepted full package version
# ("Packaging-only acceptance" of docs/owner-skill-delivery-evidence.md).
KEG_OF = {**{sid: ("grove", "22.8.0") for sid in ("grove-core", "requirements", "design", "engineering", "research",
                                                    "writing")},
          "personal": ("personal", "2026-11-06_1"), "testanyware": ("testanyware", "2.8.0_1"),
          "superpowers": ("superpowers", "7.0.0_1"), "quint": ("quint-llm-kit", "1.0.0_1")}
ALIASES = "/share/ravel/skills/"


def sha256(path):
    with open(path, "rb") as f:
        return hashlib.sha256(f.read()).hexdigest()


def state(path):
    """The digest of a file, or None when it is absent."""
    return sha256(path) if os.path.lexists(path) else None


def accepted_ravel2(out):
    with open(os.path.join(ROOT, "scripts/harness-trials/builds.json")) as f:
        want = json.load(f)["ravel2"]["binRavelSha256"]["Darwin-arm64"]
    archives = glob.glob(os.path.join(ROOT, "target/ravel-archives-2.0/ravel-v*-aarch64-apple-darwin.tar.xz"))
    if len(archives) != 1:
        sys.exit(f"expected one darwin archive in target/ravel-archives-2.0, found {archives}")
    dest = os.path.join(out, "ravel-archive")
    with tarfile.open(archives[0]) as tar:
        tar.extractall(dest, filter="tar")
    (binary,) = glob.glob(os.path.join(dest, "*", "bin", "ravel"))
    if sha256(binary) != want:
        sys.exit(f"{binary} is not the accepted 2.0 build: {sha256(binary)} != {want}")
    return binary


def load(name):
    with open(os.path.join(DRAFTS, name), encoding="utf-8") as f:
        return json.load(f)


def fixture(out, ravel, staging):
    """A Homebrew prefix whose default skill directory links the ten fixture sources."""
    prefix = os.path.join(out, "brew")
    keg = os.path.join(prefix, "Cellar", "ravel@2", "2.0.0")
    skill_dir = os.path.join(prefix, "share", "ravel", "skills")
    os.makedirs(os.path.join(keg, "bin"))
    os.makedirs(os.path.join(keg, "share", "ravel"))
    os.makedirs(skill_dir)
    shutil.copy2(ravel, os.path.join(keg, "bin", "ravel"))
    with open(os.path.join(keg, "share", "ravel", "installation.json"), "w") as f:
        json.dump({"schemaVersion": 1, "skillDirectory": skill_dir}, f)
    mandatory_text = None
    for source in staging["skills"]["sources"]:
        path, alias = fixture_keg(out, source)
        for root in source["roots"]:
            probe = os.path.join(path, root, f"{source['id']}-probe")
            os.makedirs(probe)
            with open(os.path.join(probe, "SKILL.md"), "w") as f:
                f.write(f"---\nname: {source['id']}-probe\ndescription: A fixture skill.\n---\n")
        if source["id"] == MANDATORY["source"]:
            mandatory = os.path.join(path, MANDATORY["path"])
            os.makedirs(os.path.dirname(mandatory), exist_ok=True)
            mandatory_text = f"The mandatory fixture file of {out}.\n"
            with open(mandatory, "w") as f:
                f.write(mandatory_text)
        os.symlink(alias, os.path.join(skill_dir, source["id"]))
    return os.path.join(keg, "bin", "ravel"), mandatory_text


def fixture_keg(out, source):
    """The private payload and the keg alias of SOURCE, with its staging path linked to the alias."""
    head, sep, sid = source["path"].rpartition(ALIASES)
    if not sep or sid != source["id"]:
        sys.exit(f"the staging path {source['path']} is not the alias of {source['id']} in a prefix")
    prefix = os.path.join(out, "fixture") + head
    name, version = KEG_OF[sid]
    keg = os.path.join(prefix, "Cellar", name, version)
    payload = os.path.join(keg, "libexec", "skill-sources", sid)
    alias = os.path.join(keg, "share", "ravel", "skills", sid)
    os.makedirs(payload)
    os.makedirs(os.path.dirname(alias), exist_ok=True)
    os.symlink(os.path.join("..", "..", "..", "libexec", "skill-sources", sid), alias)
    os.makedirs(os.path.dirname(prefix + ALIASES + sid), exist_ok=True)
    os.symlink(os.path.join("..", "..", "..", "Cellar", name, version, "share", "ravel", "skills", sid),
               prefix + ALIASES + sid)
    return payload, alias


def resolve(out, ravel, case, settings):
    """Run `ravel skills resolve --json` in a new HOME with SETTINGS, or none."""
    home = os.path.join(out, "homes", case)
    os.makedirs(os.path.join(home, ".config", "ravel"))
    os.makedirs(os.path.join(out, "tmp"), exist_ok=True)
    if settings is not None:
        with open(os.path.join(home, ".config", "ravel", "settings.json"), "w") as f:
            json.dump(settings, f, indent=2)
    env = {"HOME": home, "TMPDIR": os.path.join(out, "tmp"), "PATH": "/usr/bin:/bin"}
    run = subprocess.run([ravel, "skills", "resolve", "--json"], env=env, capture_output=True, text=True)
    document = None
    for text in (run.stdout, run.stderr):
        try:
            document = json.loads(text)
            break
        except ValueError:
            continue
    return home, run.returncode, document, run.stderr


def code_of(document):
    if not isinstance(document, dict):
        return None
    return document.get("code") or (document.get("error") or {}).get("code")


def check_resolved(document, ids, mandatory_text, origins=False):
    """Problems with a resolved document that must hold IDS.

    With MANDATORY_TEXT, the document names one mandatory file with that text.
    Without it, the document names no mandatory file. With ORIGINS, each
    source's originPath must be its private payload libexec/skill-sources/<id>.
    """
    problems = []
    if not document or document.get("status") != "resolved":
        return [f"status {document and document.get('status')}, not resolved"]
    got = [s["id"] for s in document["sources"]]
    if got != ids:
        problems.append(f"sources {got}, not {ids}")
    if origins:
        problems += [f"{s['id']} comes from {s.get('originPath')}, not from its private payload"
                     for s in document["sources"]
                     if not str(s.get("originPath")).endswith(f"/libexec/skill-sources/{s['id']}")]
    entries = document.get("mandatory") or []
    want = 0 if mandatory_text is None else 1
    if len(entries) != want:
        problems.append(f"{len(entries)} mandatory entries, not {want}")
    elif want:
        entry = entries[0]
        path = entry.get("path") if isinstance(entry, dict) else entry
        if not (isinstance(path, str) and os.path.isabs(path)):
            problems.append(f"the mandatory entry {entry} names no absolute path")
        else:
            snapshot = next((s["snapshotRoot"] for s in document["sources"] if s["id"] == MANDATORY["source"]), "")
            if not path.startswith(snapshot + os.sep):
                problems.append(f"the mandatory path {path} lies outside the superpowers snapshot {snapshot}")
            with open(path, encoding="utf-8") as f:
                if f.read() != mandatory_text:
                    problems.append(f"the mandatory file {path} differs from the fixture file")
    return problems


def check_declarations():
    with open(os.path.join(DRAFTS, "bootstrap-packages.toml"), "rb") as f:
        packages = tomllib.load(f)["bootstrap"]["packages"]
    want = {f"brew:linkuistics/taps/{name}": "latest" for name in FORMULAE}
    problems = [] if packages == want else [f"the declarations are {packages}, not {want}"]
    names = [key.removeprefix("brew:") for key in packages]
    found = {}
    for name in names:
        run = subprocess.run(["brew", "info", "--json=v2", name], capture_output=True, text=True,
                             env={**os.environ, "HOMEBREW_NO_AUTO_UPDATE": "1"})
        if run.returncode != 0:
            problems.append(f"the local tap holds no formula {name}: {run.stderr.strip()}")
            continue
        for formula in json.loads(run.stdout)["formulae"]:
            revision = formula.get("revision") or 0
            stable = formula["versions"]["stable"]
            found[formula["full_name"]] = f"{stable}_{revision}" if revision else stable
    return problems, found


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    parser.add_argument("--out", required=True)
    args = parser.parse_args()
    out = os.path.abspath(args.out)
    if os.path.lexists(out):
        sys.exit(f"{out} exists; name a new directory")
    os.makedirs(out)
    real_settings = os.path.expanduser("~/.config/ravel/settings.json")
    real_before = state(real_settings)

    names = ["settings-off.json", "settings-switched.json", "settings-staging.json", "bootstrap-packages.toml"]
    digests = {name: sha256(os.path.join(DRAFTS, name)) for name in names}
    off, switched, staging = (load(n) for n in names[:3])
    accepted = accepted_ravel2(out)
    ravel, mandatory_text = fixture(out, accepted, staging)

    moved = json.loads(json.dumps(staging))
    for source in moved["skills"]["sources"]:
        source["path"] = os.path.join(out, "fixture") + source["path"]
    off_mandatory = json.loads(json.dumps(off))
    off_mandatory["skills"]["mandatory"] = [MANDATORY]
    switched_wrong = json.loads(json.dumps(switched))
    switched_wrong["skills"]["mandatory"][0]["path"] = "skills/using-superpowers/ABSENT.md"
    repeated = json.loads(json.dumps(staging))
    repeated["skills"]["sources"].append(repeated["skills"]["sources"][0])

    results = []

    def case(name, settings, check):
        home, status, document, stderr = resolve(out, ravel, name, settings)
        problems = check(home, status, document)
        error = document.get("error") if isinstance(document, dict) else None
        results.append({"case": name, "status": status, "code": code_of(document),
                        "location": error and error.get("location"), "message": error and error.get("message"),
                        "result": document.get("status") if isinstance(document, dict) else None,
                        "passed": not problems, "problems": problems,
                        "stderr": stderr.strip() if problems else ""})
        print(f"{'pass' if not problems else 'FAIL'}  {name}: exit {status}"
              + (f", {code_of(document)}" if code_of(document) else "")
              + "".join(f"\n      {p}" for p in problems))

    def refuses(status_want, code_want):
        def check(_home, status, document):
            if status == status_want and code_of(document) == code_want:
                return []
            return [f"exit {status} and {code_of(document)}, not exit {status_want} and {code_want}"]
        return check

    def disabled(home, status, document):
        problems = [] if status == 0 and document == {"schemaVersion": 1, "status": "disabled"} else [
            f"exit {status} and status {isinstance(document, dict) and document.get('status')}, "
            "not exit 0 and the disabled object"]
        extra = sorted(os.path.relpath(os.path.join(d, x), home)
                       for d, dirs, files in os.walk(home) for x in dirs + files)
        if extra != [".config", ".config/ravel", ".config/ravel/settings.json"]:
            problems.append(f"the HOME holds {extra[:3]} and more after a disabled resolution")
        return problems

    def as_is(_home, status, document):
        if status == 0 and isinstance(document, dict) and document.get("status") == "resolved":
            return check_resolved(document, TEN, open(
                os.path.join(next(s["path"] for s in staging["skills"]["sources"]
                                  if s["id"] == MANDATORY["source"]), MANDATORY["path"])).read())
        if status == 3 and code_of(document) == "skill_source_missing":
            missing = document["error"].get("source")
            named = [source["path"] for source in staging["skills"]["sources"]]
            if missing in named and not os.path.lexists(missing):
                return []
            return [f"the refusal names {missing}, not an absent source path of the staging settings"]
        return [f"exit {status} and {code_of(document)}, not resolved or skill_source_missing"]

    case("no-settings", None, lambda _h, s, d: check_resolved(d, TEN, None, origins=True))
    case("off", off, disabled)
    case("switched", switched, lambda _h, s, d: check_resolved(d, TEN, mandatory_text, origins=True))
    case("staging", moved, lambda _h, s, d: check_resolved(d, TEN, mandatory_text, origins=True))
    case("staging-as-is", staging, as_is)
    case("control-off-with-mandatory", off_mandatory, refuses(2, "settings_invalid"))
    case("control-switched-absent-file", switched_wrong, refuses(3, "skill_mandatory_invalid"))
    case("control-staging-repeated-source", repeated, refuses(2, "settings_invalid"))
    # The earlier layout: a real directory with the same bytes in the default directory.
    entry = os.path.join(out, "brew", "share", "ravel", "skills", "personal")
    payload = os.path.realpath(entry)
    os.remove(entry)
    shutil.copytree(payload, entry, symlinks=True)

    def earlier_layout(_home, _status, document):
        got = check_resolved(document, TEN, None, origins=True)
        want = [f"personal comes from {os.path.realpath(entry)}, not from its private payload"]
        return [] if got == want else [f"the problems are {got}, not {want}"]
    case("control-default-earlier-layout", None, earlier_layout)

    declaration_problems, versions = check_declarations()
    print(f"{'pass' if not declaration_problems else 'FAIL'}  declarations: "
          + ", ".join(f"{k} {v}" for k, v in versions.items())
          + "".join(f"\n      {p}" for p in declaration_problems))

    real_after = state(real_settings)
    unchanged = real_before == real_after
    print(f"{'pass' if unchanged else 'FAIL'}  real settings: "
          + ("absent" if real_after is None else real_after) + (" (unchanged)" if unchanged else " (CHANGED)"))
    drafts_unchanged = all(sha256(os.path.join(DRAFTS, n)) == d for n, d in digests.items())

    record = {
        "schemaVersion": 1,
        "drafts": {n: {"path": os.path.relpath(os.path.join(DRAFTS, n), ROOT), "sha256": d}
                   for n, d in digests.items()},
        "draftsUnchanged": drafts_unchanged,
        "ravel2.0": {"path": accepted, "sha256": sha256(accepted)},
        "cases": results,
        "declarations": {"passed": not declaration_problems, "problems": declaration_problems,
                         "tapVersions": versions},
        "realSettings": {"path": real_settings, "before": real_before, "after": real_after},
    }
    with open(os.path.join(out, "record.json"), "w") as f:
        f.write(json.dumps(record, indent=2) + "\n")
    passed = all(r["passed"] for r in results) and not declaration_problems and unchanged and drafts_unchanged
    print(f"settings-drafts: {'every case passed' if passed else 'a case failed'}; wrote {out}/record.json")
    sys.exit(0 if passed else 1)


if __name__ == "__main__":
    main()
