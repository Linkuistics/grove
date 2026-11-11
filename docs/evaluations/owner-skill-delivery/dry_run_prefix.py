#!/usr/bin/env python3
"""Build the disposable Homebrew prefix of the owner switch dry run before finish.

docs/specs/owner-skill-delivery.md, "The rehearsed plan", owns the dry run.
"Owner switch rehearsal" of docs/owner-skill-delivery-evidence.md records
why this prefix exists and the limits of a result that uses it.

    python3 -I dry_run_prefix.py build --out PFX --grove-keg DIR --ravel DIR
                                       [--ravel-version V] [--real PREFIX]

Before finish, the real prefix cannot pass gates 4 and 6 of the switch: its
`bin/ravel` is Ravel 1.1.0, and its Grove has no keg layout. The dry run
therefore puts PFX/bin first on PATH. Real Homebrew then answers for PFX.

PFX is a new or empty folder outside Homebrew's temporary folder, which
Homebrew refuses as a prefix. The build writes only into PFX:
    Homebrew/bin/brew   a link to REAL/bin/brew
    Homebrew/Library    a link to REAL/Library, so the taps are the real taps
    bin/brew            a link to ../Homebrew/bin/brew. bin/brew finds its
                        prefix from its own folder and its repository from
                        that one link. The repository holds no Cellar, so
                        Homebrew's Cellar is PFX/Cellar.
    Cellar/<f>/<v>      an APFS clone (`cp -c -pPR`) of each real keg that a
                        link of REAL/share/ravel/skills names. Homebrew needs
                        each linked keg inside its own Cellar, so a link out
                        of PFX is no keg.
    Cellar/grove/<v>    a clone of the --grove-keg folder; <v> is its name
    Cellar/ravel@2/<V>  a clone of the --ravel installation, with
                        share/ravel/installation.json naming
                        PFX/share/ravel/skills. Ravel reads that record
                        beside the canonical path of its executable.
    bin/ravel           a link to ../Cellar/ravel@2/<V>/bin/ravel, which
                        stands for the promoted Ravel
    opt/<f>, var/homebrew/linked/<f>
                        links to each keg, as `brew link` makes them; opt/ravel
                        and opt/ravel@2 name the ravel@2 keg, which is not linked
    share/ravel/skills/<id>
                        each source alias, with the link text that Homebrew
                        gives it
The build then asks PFX/bin/brew for --prefix, --cellar and
`info --json=v2` of each formula, and stops unless each answer names PFX
and the linked keg. It writes PFX/prefix.json with the clones, their tree
digests and those answers.

`mise bootstrap packages status` reads the real prefix and no PATH folder.
Gate 5 therefore compares mise's real versions with PFX's linked kegs, so each
clone keeps the version of its real keg. The Grove keg is the exception: its
version must equal the installed Grove's.

Exit status: 0 when the prefix was built and verified; 1 when a check failed;
2 for a usage error; 3 when the build could not start.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
import stat
import subprocess
import sys

SCHEMA = "owner-switch-dry-run-prefix/1"
ALIASES = "share/ravel/skills"
RECORD = "share/ravel/installation.json"
TAP = "linkuistics/taps"
# Homebrew's temporary folder on macOS, which it refuses as a prefix.
HOMEBREW_TEMP = ("/private/tmp", "/tmp")


class Refused(Exception):
    """The build cannot start or a check failed."""


def tree_digest(path: str) -> str:
    """The SHA-256 of each relative path, type, executable bit, content and link text, in name order."""
    h = hashlib.sha256()

    def frame(tag: bytes, data: bytes) -> None:
        h.update(b"%d:%s %d:" % (len(tag), tag, len(data)))
        h.update(data)

    def walk(p: str, rel: str) -> None:
        st = os.lstat(p)
        if stat.S_ISLNK(st.st_mode):
            frame(b"link " + rel.encode(), os.readlink(p).encode())
        elif stat.S_ISDIR(st.st_mode):
            frame(b"dir " + rel.encode(), b"")
            for name in sorted(os.listdir(p)):
                walk(os.path.join(p, name), f"{rel}/{name}" if rel else name)
        else:
            with open(p, "rb") as f:
                frame(b"file%s %s" % (b"x" if st.st_mode & 0o111 else b"", rel.encode()), f.read())
    walk(path, "")
    return h.hexdigest()


def clone(src: str, dst: str) -> None:
    os.makedirs(os.path.dirname(dst), exist_ok=True)
    r = subprocess.run(["/bin/cp", "-c", "-pPR", src, dst], capture_output=True, text=True)
    if r.returncode != 0:
        raise Refused(f"cp -c -pPR {src} {dst} failed: {r.stderr.strip()}")


def link(target: str, path: str) -> None:
    os.makedirs(os.path.dirname(path), exist_ok=True)
    os.symlink(target, path)


def real_sources(real: str) -> list:
    """(id, formula, version) of each alias of REAL/share/ravel/skills: ../../../Cellar/<f>/<v>/share/ravel/skills/<id>."""
    folder = os.path.join(real, ALIASES)
    out = []
    for sid in sorted(os.listdir(folder)) if os.path.isdir(folder) else []:
        text = os.readlink(os.path.join(folder, sid))
        parts = text.split("/")
        if parts[:4] != ["..", "..", "..", "Cellar"] or parts[6:] != ALIASES.split("/") + [sid]:
            raise Refused(f"{folder}/{sid} links to {text}, not to a keg's {ALIASES}/{sid}")
        out.append((sid, parts[4], parts[5]))
    return out


def brew_answers(pfx: str, formulae: list) -> dict:
    brew = os.path.join(pfx, "bin", "brew")
    env = dict(os.environ, HOMEBREW_NO_AUTO_UPDATE="1", HOMEBREW_NO_ANALYTICS="1")

    def run(*args: str) -> str:
        r = subprocess.run([brew, *args], cwd=pfx, env=env, capture_output=True, text=True, timeout=300)
        if r.returncode != 0:
            raise Refused(f"{brew} {' '.join(args)} failed: {(r.stdout + r.stderr).strip()[-1500:]}")
        return r.stdout
    out = {"prefix": run("--prefix").strip(), "cellar": run("--cellar").strip(),
           "version": run("--version").splitlines()[0], "info": {}}
    for name in formulae:
        f = json.loads(run("info", "--json=v2", f"{TAP}/{name}"))["formulae"][0]
        out["info"][name] = {"stable": f["versions"]["stable"], "revision": f.get("revision") or 0,
                             "linked_keg": f.get("linked_keg"),
                             "installed": [i["version"] for i in f.get("installed") or []]}
    return out


def build(args) -> int:
    pfx = os.path.realpath(args.out)
    real = os.path.realpath(args.real)
    grove_keg = os.path.realpath(args.grove_keg)
    ravel = os.path.realpath(args.ravel)
    if any(pfx == t or pfx.startswith(t + "/") for t in HOMEBREW_TEMP):
        raise Refused(f"PFX {pfx} lies in Homebrew's temporary folder, which Homebrew refuses as a prefix")
    if os.path.exists(pfx) and os.listdir(pfx):
        raise Refused(f"PFX {pfx} must be a new or empty folder")
    for what, path in (("--grove-keg", os.path.join(grove_keg, "bin", "grove")),
                       ("--ravel", os.path.join(ravel, "bin", "ravel")),
                       ("--real", os.path.join(real, "bin", "brew"))):
        if not os.path.isfile(path):
            raise Refused(f"{what} has no {path}")
    os.makedirs(pfx, exist_ok=True)

    link(os.path.join(real, "bin", "brew"), os.path.join(pfx, "Homebrew/bin/brew"))
    link(os.path.join(real, "Library"), os.path.join(pfx, "Homebrew/Library"))
    link("../Homebrew/bin/brew", os.path.join(pfx, "bin/brew"))

    kegs = []
    others = real_sources(real)
    for formula, version in sorted({(f, v) for _, f, v in others}):
        if formula == "grove":
            raise Refused(f"{real}/{ALIASES} already holds Grove's sources; the dry run needs no stand-in prefix")
        kegs.append({"formula": formula, "version": version, "source": os.path.join(real, "Cellar", formula, version)})
    grove_version = os.path.basename(grove_keg)
    kegs.append({"formula": "grove", "version": grove_version, "source": grove_keg})
    for keg in kegs:
        dst = os.path.join(pfx, "Cellar", keg["formula"], keg["version"])
        clone(keg["source"], dst)
        keg["path"] = dst
        link(f"../Cellar/{keg['formula']}/{keg['version']}", os.path.join(pfx, "opt", keg["formula"]))
        link(f"../../../Cellar/{keg['formula']}/{keg['version']}",
             os.path.join(pfx, "var/homebrew/linked", keg["formula"]))
    aliases = [(sid, f, v) for sid, f, v in others]
    grove_aliases = os.path.join(grove_keg, ALIASES)
    aliases += [(sid, "grove", grove_version) for sid in sorted(os.listdir(grove_aliases))]
    for sid, formula, version in aliases:
        link(f"../../../Cellar/{formula}/{version}/{ALIASES}/{sid}", os.path.join(pfx, ALIASES, sid))

    ravel_keg = os.path.join(pfx, "Cellar", "ravel@2", args.ravel_version)
    clone(ravel, ravel_keg)
    record = os.path.join(ravel_keg, RECORD)
    os.makedirs(os.path.dirname(record), exist_ok=True)
    if os.path.lexists(record):
        os.remove(record)
    with open(record, "w") as f:
        json.dump({"schemaVersion": 1, "skillDirectory": os.path.join(pfx, ALIASES)}, f, separators=(",", ":"))
    for name in ("ravel", "ravel@2"):
        link(f"../Cellar/ravel@2/{args.ravel_version}", os.path.join(pfx, "opt", name))
    link(f"../Cellar/ravel@2/{args.ravel_version}/bin/ravel", os.path.join(pfx, "bin/ravel"))

    answers = brew_answers(pfx, [k["formula"] for k in kegs])
    problems = []
    if answers["prefix"] != pfx:
        problems.append(f"brew --prefix printed {answers['prefix']}, not {pfx}")
    if answers["cellar"] != os.path.join(pfx, "Cellar"):
        problems.append(f"brew --cellar printed {answers['cellar']}, not {pfx}/Cellar")
    for keg in kegs:
        info = answers["info"][keg["formula"]]
        if info["linked_keg"] != keg["version"] or keg["version"] not in info["installed"]:
            problems.append(f"brew info {keg['formula']} links {info['linked_keg']} and installs {info['installed']}, "
                            f"not {keg['version']}")
    grove_info = answers["info"]["grove"]
    tap_version = grove_info["stable"] + (f"_{grove_info['revision']}" if grove_info["revision"] else "")
    if tap_version != grove_version:
        problems.append(f"the tap's Grove is {tap_version}, and the Grove keg is {grove_version}")

    manifest = {
        "schema": SCHEMA, "prefix": pfx, "real": real,
        "kegs": [dict(k, digest=tree_digest(k["path"])) for k in kegs],
        "ravel": {"source": ravel, "keg": ravel_keg, "record": os.path.join(pfx, ALIASES),
                  "bin": hashlib.sha256(open(os.path.join(ravel_keg, "bin/ravel"), "rb").read()).hexdigest()},
        "aliases": {sid: f"../../../Cellar/{f}/{v}/{ALIASES}/{sid}" for sid, f, v in aliases},
        "brew": answers, "problems": problems,
    }
    with open(os.path.join(pfx, "prefix.json"), "w") as f:
        json.dump(manifest, f, indent=2)
        f.write("\n")
    for keg in manifest["kegs"]:
        print(f"keg {keg['formula']} {keg['version']}: {keg['digest']}")
    print(f"ravel@2 {args.ravel_version}: bin/ravel {manifest['ravel']['bin']}")
    print(f"aliases: {', '.join(sorted(manifest['aliases']))}")
    for p in problems:
        print(f"problem: {p}")
    print(f"prefix: {pfx} ({'verified' if not problems else 'NOT VERIFIED'})")
    return 0 if not problems else 1


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n", 1)[0])
    sub = parser.add_subparsers(dest="command", required=True)
    p = sub.add_parser("build")
    p.add_argument("--out", required=True, help="the new prefix folder PFX")
    p.add_argument("--grove-keg", required=True, help="the candidate Grove keg, Cellar/grove/<version>")
    p.add_argument("--ravel", required=True, help="the accepted Ravel 2.0 installation, with bin/ravel")
    p.add_argument("--ravel-version", default="2.0.0")
    p.add_argument("--real", default="/opt/homebrew", help="the real Homebrew prefix")
    args = parser.parse_args(argv)
    try:
        return build(args)
    except Refused as e:
        print(f"stop: {e}", file=sys.stderr)
        return 3


if __name__ == "__main__":
    sys.exit(main())
