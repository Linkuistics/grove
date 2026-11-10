#!/usr/bin/env python3
"""Measure the service lifecycle of the probe launcher with real harnesses.

"Probe process lifetime" of docs/specs/owner-skill-delivery.md requires a
measured lifecycle for each probe launcher: a fresh HOME without an existing
daemon, normal exit, failure and interruption. scripts/owner_switch_probes.py
owns the launcher. This tool runs its `launch` subcommand in each scenario,
one at a time, and records each result.

    probe_lifecycle.py run --out OUT [--staging HOME] [--ravel PREFIX] [--only REGEX]
    probe_lifecycle.py resolve --out OUT [--staging HOME] [--ravel PREFIX]

The homes, each new under /private/tmp unless stated:
    bare       Sign-in links only: Library/Keychains and .codex/auth.json to
               the account's own, and a .claude.json that completes
               onboarding. No daemon package and no configuration.
    package    bare, plus a clone of the newest release of the account's
               Codex daemon package under .codex/packages/app-server-daemon,
               with its own `current` link. It copies no pidfile, lock or
               socket. This is the real HOME's state after the owner stops
               the daemon.
    staging    The retained staging HOME of owner-staging-setup-k248, with
               the owner's MCP servers and residue plugins. It holds no
               daemon package.
The scenarios:
    normal     A one-word answer.
    failure    A model that does not exist, so the harness exits non-zero.
    interrupt  A shell command `sleep 120`, and SIGINT to the leader's group
               after 40 seconds.
    crash      The same command, and SIGKILL to the leader alone after 40
               seconds, which orphans its children.
The native route runs each harness directly. The ravel route runs
`ravel start-session` of the accepted 2.0 build (--ravel) in the staging HOME,
through a stub `brew` first on PATH whose prefix holds that build.

`resolve` runs the probe's `resolve` once in each mode where it must pass,
and once against a wrong layout where it must fail, with the plan draft
docs/owner-cutover/switch-plan-draft.json and the stub `brew`:
    switched        the staging HOME, whose settings name the ten accepted
                    sources and the mandatory file: passes.
    switched-quint  a disposable HOME with the staging settings without the
                    source quint: fails.
    switched-off    a disposable HOME with the off value: fails.
    disabled        a disposable HOME with the off value: passes.
    disabled-staging  the staging HOME: fails.
It writes OUT/resolve.json.

After each launch, the tool lists each process whose argument vector or
executable names the home, at once and again after 10 seconds. That listing
is independent of the launcher's records. Each launch's result, the listings
and the versions go to OUT/record.json, and a table to OUT/summary.md. A real
launch spends subscription usage, so no gate runs this tool.
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

HERE = os.path.dirname(os.path.realpath(__file__))
ROOT = os.path.realpath(os.path.join(HERE, "..", "..", ".."))
PROBES = os.path.join(ROOT, "scripts", "owner_switch_probes.py")
sys.path.insert(0, os.path.join(ROOT, "scripts"))
import owner_switch as osw  # noqa: E402  (the environment rule of the program's commands)
sys.path.insert(0, HERE)
import staging_home  # noqa: E402  (the staging environment)
import staging_trial  # noqa: E402  (the staging HOME clone and the plugin registry readers)

REAL = os.path.realpath(os.path.expanduser("~"))
STAGING = os.path.join(REAL, ".local/state/owner-staging/home")
RAVEL_2_0 = os.path.join(REAL, ".local/state/owner-staging/ravel-2.0")
DAEMON_PACKAGE = os.path.join(REAL, ".codex/packages/app-server-daemon")
INTERRUPT_AT = 40.0
SLEEP_PROMPT = ("Run exactly this shell command and wait for it to finish: sleep 120. "
                "Then reply with the single word done.")
SCENARIOS = {
    "normal": {"prompt": "Reply with the single word ok. Do not run any tool.", "args": []},
    "failure": {"prompt": "Reply with the single word ok.", "args": ["--model", "no-such-model-owner-probe"]},
    "interrupt": {"prompt": SLEEP_PROMPT, "args": ["--interrupt-after", str(INTERRUPT_AT)]},
    "crash": {"prompt": SLEEP_PROMPT, "args": ["--crash-after", str(INTERRUPT_AT)]},
}
# (label, home, harness, route, scenario)
PLAN = [
    ("bare-claude-normal", "bare", "claude", "native", "normal"),
    ("bare-claude-failure", "bare", "claude", "native", "failure"),
    ("bare-claude-interrupt", "bare", "claude", "native", "interrupt"),
    ("bare-claude-crash", "bare", "claude", "native", "crash"),
    ("bare-codex-normal", "bare", "codex", "native", "normal"),
    ("bare-codex-failure", "bare", "codex", "native", "failure"),
    ("bare-codex-interrupt", "bare", "codex", "native", "interrupt"),
    ("bare-codex-crash", "bare", "codex", "native", "crash"),
    ("package-codex-normal", "package", "codex", "native", "normal"),
    ("package-codex-interrupt", "package", "codex", "native", "interrupt"),
    ("staging-claude-normal", "staging", "claude", "native", "normal"),
    ("staging-claude-interrupt", "staging", "claude", "native", "interrupt"),
    ("staging-claude-crash", "staging", "claude", "native", "crash"),
    ("staging-codex-normal", "staging", "codex", "native", "normal"),
    ("staging-codex-interrupt", "staging", "codex", "native", "interrupt"),
    ("staging-codex-crash", "staging", "codex", "native", "crash"),
    ("staging-claude-ravel-normal", "staging", "claude", "ravel", "normal"),
    ("staging-claude-ravel-interrupt", "staging", "claude", "ravel", "interrupt"),
    ("staging-codex-ravel-normal", "staging", "codex", "ravel", "normal"),
    ("staging-codex-ravel-interrupt", "staging", "codex", "ravel", "interrupt"),
]


def now() -> str:
    return dt.datetime.now(dt.timezone.utc).isoformat(timespec="seconds")


def bare_home() -> str:
    home = os.path.realpath(tempfile.mkdtemp(prefix="owner-probe-home-", dir="/private/tmp"))
    os.makedirs(os.path.join(home, "Library"))
    os.symlink(os.path.join(REAL, "Library", "Keychains"), os.path.join(home, "Library", "Keychains"))
    os.makedirs(os.path.join(home, ".codex"))
    os.symlink(os.path.join(REAL, ".codex", "auth.json"), os.path.join(home, ".codex", "auth.json"))
    os.makedirs(os.path.join(home, ".claude"))
    with open(os.path.join(home, ".claude.json"), "w") as f:
        json.dump({"hasCompletedOnboarding": True, "autoUpdates": False}, f)
    return home


def package_home() -> str:
    home = bare_home()
    releases = os.path.join(DAEMON_PACKAGE, "releases")
    newest = sorted(os.listdir(releases))[-1]
    target = os.path.join(home, ".codex", "packages", "app-server-daemon")
    os.makedirs(os.path.join(target, "releases"))
    subprocess.run(["/bin/cp", "-cR", os.path.join(releases, newest), os.path.join(target, "releases", newest)],
                   check=True)
    os.symlink(os.path.join(target, "releases", newest), os.path.join(target, "current"))
    for name in ("auto-update-version", "selection-history"):
        if os.path.isfile(os.path.join(DAEMON_PACKAGE, name)):
            shutil.copyfile(os.path.join(DAEMON_PACKAGE, name), os.path.join(target, name))
    return home


def stub_brew(out: str, ravel: str) -> str:
    """A folder with a stub `brew` whose prefix holds the accepted 2.0 build as bin/ravel."""
    prefix = os.path.join(out, "prefix")
    os.makedirs(os.path.join(prefix, "bin"), exist_ok=True)
    link = os.path.join(prefix, "bin", "ravel")
    if not os.path.lexists(link):
        os.symlink(os.path.join(ravel, "bin", "ravel"), link)
    folder = os.path.join(out, "stub-bin")
    os.makedirs(folder, exist_ok=True)
    with open(os.path.join(folder, "brew"), "w") as f:
        f.write(f"#!/bin/sh\n[ \"$1\" = --prefix ] && {{ echo '{prefix}'; exit 0; }}\n"
                "echo \"stub brew: unsupported $*\" >&2; exit 2\n")
    os.chmod(os.path.join(folder, "brew"), 0o755)
    return folder


def home_processes(home: str) -> list:
    """Each process whose executable or arguments name HOME, from ps."""
    listing = subprocess.run(["/bin/ps", "-axww", "-o", "pid=,ppid=,lstart=,command="], capture_output=True,
                             text=True).stdout
    return [line.strip() for line in listing.splitlines() if home in line and "ps -axww" not in line]


def probe_env(home: str, path_first=None) -> dict:
    env = osw.subject_env(os.environ)
    env = {k: v for k, v in env.items() if not k.startswith("OWNER_SWITCH_")}
    env["HOME"] = home
    if path_first:
        env["PATH"] = path_first + ":" + env.get("PATH", "")
    return env


def run_one(out: str, label: str, home: str, harness: str, route: str, scenario: str, path_first) -> dict:
    spec = SCENARIOS[scenario]
    argv = [sys.executable, "-I", PROBES, "launch", "--harness", harness, "--route", route,
            "--prompt", spec["prompt"], *spec["args"]]
    env = probe_env(home, path_first)
    before = home_processes(home)
    started = now()
    done = subprocess.run(argv, env=env, cwd="/private/tmp", capture_output=True, text=True, timeout=3600)
    result = None
    for line in reversed(done.stdout.splitlines()):
        if line.startswith(osw.RESULT_MARK):
            result = json.loads(line[len(osw.RESULT_MARK):])
            break
    at_end = home_processes(home)
    time.sleep(10)
    later = home_processes(home)
    with open(os.path.join(out, f"{label}.log"), "w") as f:
        f.write(done.stdout + "\n--- stderr ---\n" + done.stderr)
    return {"label": label, "home": home, "harness": harness, "route": route, "scenario": scenario,
            "argv": argv, "started": started, "ended": now(), "exit": done.returncode, "result": result,
            "homeProcessesBefore": before, "homeProcessesAtEnd": at_end, "homeProcessesAfter10s": later}


PLAN_DRAFT = os.path.join(ROOT, "docs", "owner-cutover", "switch-plan-draft.json")
OFF = os.path.join(ROOT, "docs", "owner-cutover", "settings-off.json")


def settings_home(settings: dict) -> str:
    home = os.path.realpath(tempfile.mkdtemp(prefix="owner-probe-resolve-", dir="/private/tmp"))
    os.makedirs(os.path.join(home, ".config", "ravel"))
    with open(os.path.join(home, ".config", "ravel", "settings.json"), "w") as f:
        json.dump(settings, f, indent=2)
    return home


def resolve_cases(out: str, staging: str, ravel: str) -> int:
    os.makedirs(out, exist_ok=False)
    path_first = stub_brew(out, ravel)
    with open(os.path.join(staging, ".config", "ravel", "settings.json")) as f:
        staged = json.load(f)
    without = json.loads(json.dumps(staged))
    without["skills"]["sources"] = [s for s in without["skills"]["sources"] if s["id"] != "quint"]
    with open(OFF) as f:
        off = json.load(f)
    cases = [("switched", staging, "switched", 0, None),
             ("switched-quint", settings_home(without), "switched", 1, "accepted source quint is missing"),
             ("switched-off", settings_home(off), "switched", 1, "not 'resolved'"),
             ("disabled", settings_home(off), "disabled", 0, None),
             ("disabled-staging", staging, "disabled", 1, "not 'disabled'")]
    record = {"tool": os.path.relpath(os.path.realpath(__file__), ROOT), "plan": PLAN_DRAFT, "ravel": ravel,
              "started": now(), "cases": []}
    failures = 0
    for name, home, expect, want, text in cases:
        argv = [sys.executable, "-I", PROBES, "resolve", "--expect", expect, "--plan", PLAN_DRAFT]
        done = subprocess.run(argv, env=probe_env(home, path_first), cwd="/private/tmp", capture_output=True,
                              text=True, timeout=900)
        result = next((json.loads(line[len(osw.RESULT_MARK):]) for line in reversed(done.stdout.splitlines())
                       if line.startswith(osw.RESULT_MARK)), None)
        problems = (result or {}).get("problems") or []
        ok = done.returncode == want and (text is None or any(text in p for p in problems))
        failures += not ok
        print(f"{'PASS' if ok else 'FAIL'} {name}: exit {done.returncode}, wanted {want}; {problems[:2]}", flush=True)
        record["cases"].append({"name": name, "home": home, "expect": expect, "wantedExit": want,
                                "wantedText": text, "exit": done.returncode, "result": result, "ok": ok})
    record["ended"] = now()
    with open(os.path.join(out, "resolve.json"), "w") as f:
        json.dump(record, f, indent=2)
    return 1 if failures else 0


# The session cases: (label, harness, route, layout, wanted exit, the observation that must fail).
SESSION_CASES = [
    ("claude-ravel", "claude", "ravel", "switched", 0, None),
    ("claude-ravel-stale", "claude", "ravel", "stale", 1, "listing"),
    ("claude-native", "claude", "native", "native", 0, None),
    ("claude-native-bare", "claude", "native", "switched", 1, "kindSkill"),
    ("codex-ravel", "codex", "ravel", "switched", 0, None),
    ("codex-ravel-stale", "codex", "ravel", "stale", 1, "listing"),
    ("codex-native", "codex", "native", "native", 0, None),
    ("codex-native-bare", "codex", "native", "switched", 1, "kindSkill"),
]
HOOKS_PLUGIN = "technical-communication-hooks@writing-hooks"
CANDIDATE = os.path.join(REAL, ".local/state/owner-staging/candidate")
STALE_SKILL = "grove-planning"


def superseded_entries(plan: dict) -> list:
    """The real HOME's Codex entries that the plan's switched assertions name absent, as relative paths."""
    return [a["path"] for a in plan["assertions"] if a.get("absent") and os.path.dirname(a["path"]) == ".codex/skills"
            and os.path.exists(os.path.join(REAL, a["path"]))]


def plant(home: str, names: list) -> list:
    """Copy each named real `.codex/skills` entry, links followed, into both native roots of HOME."""
    planted = []
    for name in names:
        source = os.path.join(REAL, ".codex", "skills", name)
        for root in (".claude/skills", ".codex/skills"):
            target = os.path.join(home, root, name)
            os.makedirs(os.path.dirname(target), exist_ok=True)
            done = subprocess.run(["/bin/cp", "-cRL", source, target], capture_output=True, text=True)
            if done.returncode != 0:
                raise SystemExit(f"cp -cRL {source} {target} failed: {done.stderr.strip()}")
            planted.append(f"{root}/{name}")
    return planted


def install_hooks(home: str, ravel: str) -> dict:
    """technical-communication-hooks@writing-hooks from the candidate keg, through Claude Code's own commands.

    The marketplace is registered from the candidate prefix's opt/grove, as
    the owner registers it from $(brew --prefix)/opt/grove: Claude Code stores
    that path and runs the plugin in place through it ("The hook adapters on
    the projected keg" of docs/homebrew-skill-delivery-evidence.md).
    """
    opt = os.path.join(CANDIDATE, "brew", "opt", "grove")
    keg = os.path.realpath(opt)
    if not os.path.isdir(os.path.join(keg, ".claude-plugin")):
        raise SystemExit(f"{opt} names no keg with a Claude Code marketplace file")
    env = staging_home.staged_env(staging_trial.Path(home), staging_trial.Path(ravel))
    cwd = staging_home.work_directory(staging_trial.Path(home))
    log = []
    staging_home.run(["claude", "plugin", "marketplace", "add", opt], env, cwd, log)
    staging_home.run(["claude", "plugin", "install", HOOKS_PLUGIN], env, cwd, log)
    enabled = staging_trial.enabled_value(staging_trial.Path(home), HOOKS_PLUGIN)
    if enabled is not True:
        raise SystemExit(f"after the install, enabledPlugins[{HOOKS_PLUGIN}] is {enabled!r}, not true")
    return {"keg": keg, "marketplace": opt, "entry": staging_trial.installed_entry(staging_trial.Path(home), HOOKS_PLUGIN),
            "log": [{k: item[k] for k in ("argv", "exit")} for item in log]}


def remove_bytecode() -> list:
    """Remove and name each `__pycache__` in the candidate keg's skill sources (tc-hook-bytecode-k284).

    The technical-communication hooks run in place from the `writing` source
    and write bytecode there, which changes its resourceDigest. Removing it
    before each case lets the case measure the accepted digests.
    """
    removed = []
    for folder, dirs, files in os.walk(os.path.join(CANDIDATE, "brew", "Cellar")):
        if "__pycache__" in dirs:
            path = os.path.join(folder, "__pycache__")
            removed.append({"path": path, "files": sorted(os.listdir(path))})
            shutil.rmtree(path)
            dirs.remove("__pycache__")
    return removed


def session_home(out: str, label: str, staging: str, harness: str, layout: str, plan: dict, ravel: str) -> dict:
    """A `cp -c` clone of the staging HOME with the case's layout."""
    home = os.path.join(out, "homes", label)
    os.makedirs(os.path.dirname(home), exist_ok=True)
    facts = {"home": home, "layout": layout,
             "clone": staging_trial.clone_home(staging_trial.Path(staging), staging_trial.Path(home))}
    if harness == "claude" and layout in ("switched", "stale"):
        facts["hooks"] = install_hooks(home, ravel)
    if layout == "stale":
        facts["planted"] = plant(home, [STALE_SKILL])
    elif layout == "native":
        facts["planted"] = plant(home, [os.path.basename(p) for p in superseded_entries(plan)])
    return facts


def session_cases(out: str, staging: str, ravel: str, only=None) -> int:
    os.makedirs(out, exist_ok=False)
    path_first = stub_brew(out, ravel)
    with open(PLAN_DRAFT) as f:
        plan = json.load(f)
    with open(PROBES, "rb") as f:
        program = hashlib.sha256(f.read()).hexdigest()
    record = {"tool": os.path.relpath(os.path.realpath(__file__), ROOT), "plan": PLAN_DRAFT, "ravel": ravel,
              "probes": PROBES, "programSha256": program, "started": now(), "cases": []}
    failures = 0
    for label, harness, route, layout, want, failing in SESSION_CASES:
        if only and not re.search(only, label):
            continue
        print(f"{now()} {label}: preparing the {layout} layout", flush=True)
        removed = remove_bytecode()
        if removed:
            print(f"  removed stray bytecode from the candidate keg: {removed}", flush=True)
        facts = session_home(out, label, staging, harness, layout, plan, ravel)
        facts["bytecodeRemoved"] = removed
        argv = [sys.executable, "-I", PROBES, "session", "--harness", harness, "--route", route, "--plan", PLAN_DRAFT]
        started = now()
        done = subprocess.run(argv, env=probe_env(facts["home"], path_first), cwd="/private/tmp",
                              capture_output=True, text=True, timeout=3600)
        with open(os.path.join(out, f"{label}.log"), "w") as f:
            f.write(done.stdout + "\n--- stderr ---\n" + done.stderr)
        result = next((json.loads(line[len(osw.RESULT_MARK):]) for line in reversed(done.stdout.splitlines())
                       if line.startswith(osw.RESULT_MARK)), None) or {}
        observations = result.get("observations") or {}
        failed = sorted(name for name, v in observations.items() if not v.get("passed"))
        ok = done.returncode == want and (failing is None or failing in failed)
        failures += not ok
        print(f"{'PASS' if ok else 'FAIL'} {label}: exit {done.returncode}, wanted {want}; failed {failed}; "
              f"{(result.get('problems') or [])[:3]}", flush=True)
        record["cases"].append({"label": label, "harness": harness, "route": route, "layout": layout,
                                "wantedExit": want, "wantedFailure": failing, "started": started, "ended": now(),
                                "exit": done.returncode, "failed": failed, "ok": ok, "facts": facts,
                                "result": result})
        with open(os.path.join(out, "sessions.json"), "w") as f:
            json.dump(record, f, indent=2)
    record["ended"] = now()
    with open(os.path.join(out, "sessions.json"), "w") as f:
        json.dump(record, f, indent=2)
    return 1 if failures else 0


def summary(records: list) -> str:
    lines = ["| Launch | Exit | Leader | Owned | Survivors | Signalled | Unattributed | Home processes after 10 s |",
             "| --- | --- | --- | --- | --- | --- | --- | --- |"]
    for r in records:
        res = r["result"] or {}
        leader = res.get("leader") or {}
        lines.append("| {} | {} | {} {} | {} | {} | {} | {} | {} |".format(
            r["label"], r["exit"], leader.get("exit"), leader.get("endedBy") or "", len(res.get("owned") or []),
            len(res.get("survivors") or []), len((res.get("cleanup") or {}).get("signalled") or []),
            len(res.get("unattributed") or []), len(r["homeProcessesAfter10s"])))
    return "\n".join(lines) + "\n"


def main(argv=None) -> int:
    p = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    sub = p.add_subparsers(dest="command", required=True)
    r = sub.add_parser("run")
    r.add_argument("--out", required=True)
    r.add_argument("--staging", default=STAGING)
    r.add_argument("--ravel", default=RAVEL_2_0)
    r.add_argument("--only")
    v = sub.add_parser("resolve")
    v.add_argument("--out", required=True)
    v.add_argument("--staging", default=STAGING)
    v.add_argument("--ravel", default=RAVEL_2_0)
    n = sub.add_parser("sessions")
    n.add_argument("--out", required=True)
    n.add_argument("--staging", default=STAGING)
    n.add_argument("--ravel", default=RAVEL_2_0)
    n.add_argument("--only")
    args = p.parse_args(argv)
    if args.command == "sessions":
        return session_cases(os.path.realpath(args.out), args.staging, args.ravel, args.only)
    if args.command == "resolve":
        return resolve_cases(os.path.realpath(args.out), args.staging, args.ravel)
    out = os.path.realpath(args.out)
    os.makedirs(out, exist_ok=False)
    homes = {}
    path_first = stub_brew(out, args.ravel)
    versions = {name: subprocess.run([name, "--version"], capture_output=True, text=True).stdout.strip()
                for name in ("claude", "codex")}
    record = {"tool": os.path.relpath(os.path.realpath(__file__), ROOT), "probes": PROBES, "started": now(),
              "versions": versions, "ravel": args.ravel, "launches": []}
    for label, home_kind, harness, route, scenario in PLAN:
        if args.only and not re.search(args.only, label):
            continue
        if home_kind not in homes:
            homes[home_kind] = {"bare": bare_home, "package": package_home}.get(home_kind, lambda: args.staging)()
        print(f"{now()} {label} in {homes[home_kind]}", flush=True)
        item = run_one(out, label, homes[home_kind], harness, route, scenario, path_first)
        record["launches"].append(item)
        res = item["result"] or {}
        print(f"  exit {item['exit']}: {res.get('problems')} owned {len(res.get('owned') or [])}, "
              f"survivors {len(res.get('survivors') or [])}, home processes after 10 s "
              f"{len(item['homeProcessesAfter10s'])}", flush=True)
        with open(os.path.join(out, "record.json"), "w") as f:
            json.dump(dict(record, homes=homes), f, indent=2)
    record["ended"] = now()
    record["homes"] = homes
    with open(os.path.join(out, "record.json"), "w") as f:
        json.dump(record, f, indent=2)
    with open(os.path.join(out, "summary.md"), "w") as f:
        f.write(summary(record["launches"]))
    print(summary(record["launches"]))
    return 0


if __name__ == "__main__":
    sys.exit(main())
