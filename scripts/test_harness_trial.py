#!/usr/bin/env python3
"""Test scripts/harness_trial.py against fake harnesses, so that no test makes
a real call or spends subscription usage.

The tests call the runner in process, with a fake real home, a fake prepared
directory and a fake environment. A fake `grove` driver stands in for Grove and
Ravel: it reads the route that the runner wrote, gives the trial its own
channel and launch directory, and starts the real launch wrapper. A fake
`claude` answers the probes and writes a stream and a session transcript.

Each check also runs once against a deliberately broken runner: one runner
function replaced by a broken version. The check must fail there, so a check
that reads clean for every runner fails this file. The real trials are the
opt-in task `trial:claude`.
"""

from __future__ import annotations

import contextlib
import io
import json
import os
import platform
import shutil
import sys
import tempfile
import textwrap
import unittest
from pathlib import Path
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parent))
import harness_trial as trial  # noqa: E402

LIVE = {
    # The live session's authority, as a runner inside a session inherits it.
    "RAVEL_EXIT_FILE": "/live/session/exit",
    "GROVE_LAUNCH_DIR": "/live/session/launch",
    "CLAUDECODE": "1",
    "CLAUDE_CODE_SESSION_ID": "live-session",
    "CLAUDE_CODE_ENTRYPOINT": "cli",
    "ANTHROPIC_API_KEY": "sk-live",
}

# macOS adds __CF_USER_TEXT_ENCODING to the environment of every process, so
# the fake driver holds it although the runner never passes it.
ALLOWED = {"HOME", "TMPDIR", "PATH", "SHELL", "TERM", "__CF_USER_TEXT_ENCODING", *trial.PASSED_NAMES}

FAKE_GROVE = r'''#!{python}
# A fake driver: one launch through the real wrapper, then the refusal of the
# next kind, as the trial configuration ends a real loop.
import json, os, subprocess, sys, hashlib
from pathlib import Path
home = Path(os.environ["HOME"])
trial = home.parent
Path({env_file!r}).write_text(json.dumps(dict(os.environ)))
settings = home / ".config/ravel/settings.json"
for source in (json.loads(settings.read_text())["skills"]["sources"] if settings.exists() else []):
    if not Path(source["path"]).is_dir():
        # Ravel refuses a missing source before any worker, and Grove's loop
        # stops without a relaunch.
        print(f"ravel: refused (skill_source_missing, stage skills): the source {{source['id']!r}} at "
              f"{{source['path']}} does not exist", file=sys.stderr)
        print("  remedy: restore the source directory, or change the settings deliberately", file=sys.stderr)
        print("grove: session kind `impl` for `smoke-file-k1` failed; rerun `grove` to continue", file=sys.stderr)
        sys.exit(0)
route = json.loads((trial / "route.json").read_text())
# The snapshot captures the configured source's craft skill, as Ravel captures
# the current bytes. Changed bytes give a new snapshot and a new descriptor;
# the snapshot of an earlier launch stays as it was.
config = (json.loads(settings.read_text()) if settings.exists() else {{}}).get("skills", {{}})
origin = config["sources"][0]["path"] if config.get("sources") else "/origin/grove"
craft_origin = Path(origin) / "skills/craft/SKILL.md"
craft = craft_origin.read_text() if craft_origin.is_file() else "craft\n"
content = hashlib.sha256(craft.encode()).hexdigest()
unchanged = craft == "craft\n"
snap = home / ".cache/ravel/skills/objects" / ("grove-snapshot" if unchanged else "grove-" + content[:12])
if not snap.exists():
    (snap / "skills/grove-impl").mkdir(parents=True)
    (snap / "skills/grove-impl/SKILL.md").write_text("kind\n")
    (snap / "skills/grove/references").mkdir(parents=True)
    (snap / "skills/grove/SKILL.md").write_text("spine\n")
    (snap / "skills/grove/references/execute.md").write_text("resource\n")
    (snap / "skills/craft").mkdir(parents=True)
    (snap / "skills/craft/SKILL.md").write_text(craft)
mandatory = [{{"source": "grove", "path": str(snap / item["path"])}} for item in config.get("mandatory", [])]
mandatory = mandatory or [{{"source": "grove", "path": str(snap / "skills/grove/SKILL.md")}}]
descriptor_text = (b'{{"schemaVersion":1}}' if unchanged and not config.get("mandatory")
                   else json.dumps({{"content": content, "mandatory": mandatory}}).encode())
digest = hashlib.sha256(descriptor_text).hexdigest()
descriptor = home / ".cache/ravel/descriptors" / digest / "directories.json"
descriptor.parent.mkdir(parents=True, exist_ok=True)
descriptor.write_bytes(descriptor_text)
view = {{"kind": "impl", "originalPrompt": "mandate", "skills": {{
    "descriptorPath": str(descriptor),
    "sources": [{{"id": "grove", "type": "local", "resourceDigest": "ab" * 32 if unchanged else content,
                 "snapshotRoot": str(snap), "originPath": origin}}],
    "directories": [{{"source": "grove", "path": str(snap / "skills")}}],
    "mandatory": mandatory}}}}
if sys.argv[1:2] == ["run"]:
    # A fake `grove run`: the wrapper writes to the run log, as a confined
    # launch does. The fake adds the probes that the configuration script adds.
    probes = [["read", str(descriptor)]]
    probes += [["read", item["path"]] for item in view["skills"]["mandatory"]]
    probes += [["write", str(snap) + "/trial-probe"]]
    probes += route["probes"] + [["write", "trial-probe-inside"]]
    work = trial / "tmp/work"
    work.mkdir()
    logs = home / ".local/state/grove/runs"
    logs.mkdir(parents=True)
    env = dict(os.environ, RAVEL_EXIT_FILE=str(trial / "tmp/exit"), RAVEL_SKILL_DIRECTORIES=str(descriptor))
    args = [route["wrapper"], route["records"], json.dumps(view), route["program"]]
    for op, path in probes:
        args += ["--probe", op, path]
    args += ["--add-dir", str(snap), *route["fixedArgs"], "standalone task"]
    with (logs / "grove-run-fake.log").open("w") as log:
        print("ravel: a line of the run log", file=log, flush=True)
        status = subprocess.run(args, env=env, cwd=work, stdin=subprocess.DEVNULL, stdout=log, stderr=log).returncode
    output = sys.argv[sys.argv.index("--output") + 1]
    Path(output).write_text("#!/usr/bin/env bash\n")
    sys.exit(status)
launch = trial / "tmp/launch"
launch.mkdir(exist_ok=True)
env = dict(os.environ, GROVE_LAUNCH_DIR=str(launch), RAVEL_EXIT_FILE=str(trial / "tmp/exit"),
           RAVEL_SKILL_DIRECTORIES=str(descriptor))
print("grove: launching impl through Ravel — smoke-file-k1", file=sys.stderr, flush=True)
args = [route["wrapper"], route["records"], json.dumps(view), route["program"],
        "--add-dir", str(snap), *route["fixedArgs"], "Grove mandate: the leaf"]
status = subprocess.run(args, env=env, stdin=subprocess.DEVNULL).returncode
print(f"grove: session ended without a supported Grove result — status {{status}}; loop stopped.", file=sys.stderr)
sys.exit(1)
'''

FAKE_CLAUDE = r'''#!{python}
# A fake Claude Code. Its behaviour file says what it does.
import json, os, sys
from pathlib import Path
behaviour = json.loads(Path({behaviour!r}).read_text())
with open({launched!r}, "a") as log:
    log.write(" ".join(sys.argv[1:2]) + "\n")
home = Path(os.environ["HOME"])
if sys.argv[1:2] == ["--version"]:
    print("9.9.9 (Claude Code)")
    sys.exit(0)
if sys.argv[1:3] == ["auth", "status"]:
    print(json.dumps({{"loggedIn": True, "authMethod": "claude.ai",
                      "configDirectory": behaviour.get("configDirectory") or str(home / ".claude")}}))
    sys.exit(0)
# A resumed turn has no --add-dir, and it reads the first turn's snapshot.
saved = home / ".fake-claude-snap"
if "--add-dir" in sys.argv:
    snap = sys.argv[sys.argv.index("--add-dir") + 1]
    saved.write_text(snap)
else:
    snap = saved.read_text()
names = {{"SNAP": snap, "HOME": str(home), "DESC": os.environ.get("RAVEL_SKILL_DIRECTORIES", ""),
         "CWD": os.getcwd(), "TRIAL": str(home.parent)}}
project = home / ".claude/projects/-fake"
resume = sys.argv[sys.argv.index("--resume") + 1] if "--resume" in sys.argv else None
# A resumed turn keeps the session ID, as Claude Code does without
# --fork-session. Each new session has an ID of its own.
count = len(list(project.glob("*.jsonl"))) if project.is_dir() else 0
session = resume or f"{{count + 1:08d}}-2222-3333-4444-555555555555"
tools = behaviour.get("resumeTools", behaviour["tools"]) if resume else behaviour["tools"]
if behaviour.get("noInit"):
    print("Error: Input must be provided", file=sys.stderr)
    sys.exit(1)
print(json.dumps({{"type": "system", "subtype": "init", "session_id": session, "model": "fake",
                  "permissionMode": "bypassPermissions", "tools": ["Read", "Bash"], "skills": [],
                  "claude_code_version": "9.9.9"}}))
entries = [{{"type": "attachment", "attachment": {{"type": "skill_listing", "skillCount": 0, "names": []}}}},
           {{"type": "attachment", "attachment": {{"type": "credential_org", "org": "secret-org"}}}}]
for n, (tool, field, value, error) in enumerate(tools):
    if tool == "text":
        entries.append({{"type": "assistant", "message": {{"content": [{{"type": "text", "text": value}}]}}}})
        continue
    tid = f"toolu_{{n}}"
    target = value.format(**names)
    entries.append({{"type": "assistant", "message": {{"content": [
        {{"type": "tool_use", "id": tid, "name": tool, "input": {{field: target}}}}]}}}})
    # A read returns the bytes of the file, so a marker in them reaches the result.
    text = Path(target).read_text() if tool == "Read" and Path(target).is_file() else f"{{tool}} result"
    entries.append({{"type": "user", "message": {{"content": [
        {{"type": "tool_result", "tool_use_id": tid, "is_error": error,
          "content": [{{"type": "text", "text": text}}]}}]}}}})
if behaviour.get("noSession"):
    # A confined harness cannot write its session file. Its stream holds the
    # same tool calls.
    for entry in entries:
        print(json.dumps(entry))
else:
    project.mkdir(parents=True, exist_ok=True)
    with (project / f"{{session}}.jsonl").open("a" if resume else "w") as transcript:
        transcript.write("".join(json.dumps(e) + "\n" for e in entries))
store = home / {store!r}
if behaviour.get("replaceLink"):
    store.unlink()
    store.mkdir()
if behaviour.get("extraLink"):
    os.symlink("/", home / ".claude/extra")
if behaviour.get("strayNative"):
    (home / ".claude/skills/stray").mkdir(parents=True)
    (home / ".claude/skills/stray/SKILL.md").write_text("stray\n")
print(json.dumps({{"type": "result", "subtype": "success"}}))
'''

FAKE_FRONT = r'''#!{python}
# A fake 2.0 front. `skills resolve --json` names the descriptor that the fake
# driver published, as a real resolution of the same settings does.
import json, os, sys
from pathlib import Path
if sys.argv[1:] != ["skills", "resolve", "--json"]:
    sys.exit(0)
settings = Path(os.environ["HOME"]) / ".config/ravel/settings.json"
for source in json.loads(settings.read_text())["skills"]["sources"]:
    if not Path(source["path"]).is_dir():
        print(f"ravel: refused (skill_source_missing, stage skills): {{source['path']}} does not exist", file=sys.stderr)
        print("  remedy: restore the source directory, or change the settings deliberately", file=sys.stderr)
        sys.exit(3)
found = sorted(Path(os.environ["HOME"]).glob(".cache/ravel/descriptors/*/directories.json"))
print(json.dumps({{"schemaVersion": 1, "status": "resolved", "descriptorPath": str(found[0]) if found else None,
                  "sources": [{{"id": "grove", "resourceDigest": "ab" * 32}}]}}))
'''

# The transcript of the fake session, in tool-call order, with the class that
# extraction must give each call.
TOOLS = [
    ("Read", "file_path", "{SNAP}/skills/grove/SKILL.md", False),
    ("Read", "file_path", "{SNAP}/skills/grove/references/execute.md", False),
    ("Bash", "command", "cd \"{SNAP}/skills\" && find . -name SKILL.md | while read f; do head -8 \"$f\"; done", False),
    ("Bash", "command", "cd \"{SNAP}/skills\" && cat grove-impl/SKILL.md; ls grove-impl", False),
    ("Read", "file_path", "{SNAP}/skills/craft/SKILL.md", False),
    # A path in a variable that the same command assigns.
    ("Bash", "command", "S=\"{SNAP}/skills/grove\"; cat \"$S/references/execute.md\"; ls -R \"${{S}}\"", False),
    ("Read", "file_path", "{DESC}", False),
    ("Read", "file_path", "{HOME}/.claude/skills/old/SKILL.md", True),
    ("text", "text", "The native loader has no match, so I search the directories.", False),
    ("Skill", "skill", "grove-impl", True),
    # A concrete path that the case planted outside the repository.
    ("Read", "file_path", "{TRIAL}/guidance/craft/SKILL.md", True),
    ("Write", "file_path", "{CWD}/smoke.txt", False),
    ("Bash", "command", "jj commit -m smoke", False),
    ("Bash", "command", "'/x/bin/ravel' end-session grove-continue", False),
]
EXPECTED = [
    ("Read", "mandatory", "grove", False),
    ("Read", "supporting", "grove", False),
    ("Bash", "listing", None, False),
    ("Bash", "kind-skill", "grove-impl", False),
    ("Read", "skill", "craft", False),
    ("Bash", "supporting", "grove", False),
    ("Read", "descriptor", None, False),
    ("Read", "native", "old", True),
    ("text", "text", None, None),
    ("Skill", "native-skill-tool", None, True),
    ("Read", "planted", "craft", True),
    ("Write", "action", None, False),
    ("Bash", "command", None, False),
    ("Bash", "ending", None, False),
]

# The resumed turn of the continuation case: a mandatory read and a craft read
# in the first turn's snapshot, then a read of the changed source itself.
RESUME_TOOLS = [
    ("Read", "file_path", "{SNAP}/skills/grove/SKILL.md", False),
    ("Read", "file_path", "{SNAP}/skills/craft/SKILL.md", False),
    ("Read", "file_path", "{TRIAL}/sources/checkout/plugins/grove/skills/craft/SKILL.md", False),
]
MARKER = "k111-test-marker"
CRAFT = "plugins/grove/skills/craft/SKILL.md"


class Fixture:
    """A fake real home, a fake prepared directory and the fake programs."""

    def __init__(self, root: Path):
        self.root = root
        self.store = trial.sign_in_store()
        self.owner = root / "owner"
        (self.owner / self.store).parent.mkdir(parents=True)
        if self.store.endswith("Keychains"):
            (self.owner / self.store).mkdir()
            (self.owner / self.store / "login.keychain-db").write_text("keychain\n")
        else:
            (self.owner / self.store).write_text("{}\n")
        (self.owner / ".claude/skills/real").mkdir(parents=True)
        (self.owner / ".claude/skills/real/SKILL.md").write_text("real\n")

        self.prepared = root / "prepared"
        grove_bin = self.prepared / "grove/bin"
        grove_bin.mkdir(parents=True)
        self.grove_env = root / "grove.env"
        self._exec(grove_bin / "grove", FAKE_GROVE.format(python=sys.executable, env_file=str(self.grove_env)))
        self._exec(grove_bin / "grove-llm", "#!/bin/sh\nexit 0\n")
        front = root / "ravel/bin/ravel"
        front.parent.mkdir(parents=True)
        self._exec(front, FAKE_FRONT.format(python=sys.executable))
        # The checkout is a Git repository with two commits, so that a case can
        # copy the older revision of a skill as a stale native copy.
        checkout = root / "source"
        (checkout / "plugins/grove/skills/craft").mkdir(parents=True)
        self.old_commit = self._commit(checkout, "craft old\n", init=True)
        head = self._commit(checkout, "craft\n")
        (self.prepared / "prepared.json").write_text(json.dumps({
            "grove": {"commit": "c" * 40, "prefix": str(grove_bin.parent),
                      "sha256": {name: trial.sha256_file(grove_bin / name) for name in ("grove", "grove-llm")}},
            "source": {"checkout": str(checkout), "commit": head},
            "ravel": {"front": str(front), "binRavelSha256": trial.sha256_file(front),
                      "buildIdentity": "i" * 64, "buildCommit": "b" * 12, "outputCommit": "o" * 12,
                      "line": "skill-directory-delivery"},
        }))

        self.behaviour = root / "claude.behaviour"
        self.launched = root / "claude.launched"
        self.claude = root / "claude-bin/claude"
        self.claude.parent.mkdir()
        self._exec(self.claude, FAKE_CLAUDE.format(
            python=sys.executable, behaviour=str(self.behaviour), launched=str(self.launched), store=self.store))
        self.behave()

    @staticmethod
    def _commit(checkout: Path, text: str, init: bool = False) -> str:
        env = {**trial.git_env(os.environ), "GIT_AUTHOR_NAME": "t", "GIT_AUTHOR_EMAIL": "t@example.com",
               "GIT_COMMITTER_NAME": "t", "GIT_COMMITTER_EMAIL": "t@example.com"}
        if init:
            trial.run_checked(["git", "init", "--quiet", str(checkout)], env=env)
        (checkout / "plugins/grove/skills/craft/SKILL.md").write_text(text)
        trial.run_checked(["git", "add", "--all"], cwd=checkout, env=env)
        trial.run_checked(["git", "commit", "--quiet", "-m", text.strip()], cwd=checkout, env=env)
        return trial.run_checked(["git", "rev-parse", "HEAD"], cwd=checkout, env=env)

    @staticmethod
    def _exec(path: Path, text: str) -> None:
        path.write_text(text)
        path.chmod(0o755)

    def behave(self, **changes) -> None:
        self.behaviour.write_text(json.dumps({"tools": TOOLS, **changes}))

    def environ(self, **extra) -> dict[str, str]:
        return {"HOME": str(self.owner), "PATH": os.environ["PATH"], "USER": "owner",
                "HARNESS_TRIAL_PROGRAM": str(self.claude), **LIVE, **extra}

    def run(self, case: str = "smoke", **extra) -> tuple[int, str]:
        """Run a case; return the status and everything it printed."""
        output = io.StringIO()
        with contextlib.redirect_stdout(output), contextlib.redirect_stderr(output):
            status = trial.main(["run", "--prepared", str(self.prepared), "--case", case], self.environ(**extra))
        return status, output.getvalue()

    def launched_any(self) -> bool:
        return self.launched.exists() or self.grove_env.exists()

    def record(self) -> dict:
        runs = sorted((self.prepared / "runs").iterdir())
        return json.loads((runs[-1] / "record.json").read_text())


def plant_cases(root: Path) -> Path:
    """The smoke case, and two cases of the same shape for the offline
    resolution and the confined standalone mode."""
    cases = root / "cases"
    shutil.copytree(trial.CASES / "smoke", cases / "smoke")
    smoke = json.loads((cases / "smoke" / "case.json").read_text())
    resolve = cases / "resolve"
    shutil.copytree(cases / "smoke", resolve)
    (resolve / "case.json").write_text(json.dumps({**smoke, "name": "resolve", "resolveOffline": True}))
    confined = cases / "confined"
    confined.mkdir()
    (confined / "prompt.md").write_text("Write stamp.sh.\n")
    (confined / "case.json").write_text(json.dumps({
        **smoke, "name": "confined", "mode": "standalone", "probes": True,
        "standalone": {"kind": "impl", "prompt": "prompt.md", "outputs": ["stamp.sh"]},
        "mandatory": [{"source": "grove", "path": "skills/grove/SKILL.md"}],
    }))
    # A case with a native copy, a sandbox deny rule, a planted file and a task
    # that names the planted path.
    explicit = cases / "explicit"
    shutil.copytree(cases / "smoke", explicit)
    (explicit / "tree" / "02-impl--path-k2.md").write_text("Read `{{TRIAL}}/guidance/craft/SKILL.md`.\n")
    (explicit / "case.json").write_text(json.dumps({
        **smoke, "name": "explicit",
        "native": [{"name": "craft", "from": "plugins/grove/skills/craft"}],
        "claudeSettings": {"permissions": {"deny": ["Skill(craft)"]}},
        "plant": [{"from": "plugins/grove/skills/craft/SKILL.md", "to": "guidance/craft/SKILL.md"}],
    }))
    # A stale native copy: the skill as the commit before the checkout's head held it.
    stale = cases / "stale"
    shutil.copytree(cases / "smoke", stale)
    (stale / "case.json").write_text(json.dumps({
        **smoke, "name": "stale",
        "native": [{"name": "craft", "from": "plugins/grove/skills/craft", "commit": "HEAD~1"}],
    }))
    # A source moved away before the driver, which expects Ravel's refusal.
    missing = cases / "missing"
    shutil.copytree(cases / "smoke", missing)
    (missing / "case.json").write_text(json.dumps({
        **smoke, "name": "missing", "moveAway": "grove", "expectRefusal": "skill_source_missing",
    }))
    # A continuation: a source commit and a settings change between a first
    # turn and a resumed turn, then a fresh launch on a new leaf.
    continuation = cases / "continuation"
    shutil.copytree(cases / "smoke", continuation)
    (continuation / "answer.md").write_text("Yes, add the label.\n")
    (continuation / "fresh-tree").mkdir()
    (continuation / "fresh-tree" / "02-impl--fresh-k2.md").write_text("# fresh-k2\n")
    (continuation / "case.json").write_text(json.dumps({
        **smoke, "name": "continuation",
        "mandatory": [{"source": "grove", "path": "skills/grove/SKILL.md"}],
        "continuation": {
            "marker": MARKER, "answer": "answer.md", "freshTree": "fresh-tree",
            "edits": [{"path": CRAFT, "insert": f"Rule {MARKER}."}],
            "mandatory": [{"source": "grove", "path": "skills/grove-impl/SKILL.md"},
                          {"source": "grove", "path": "skills/grove/SKILL.md"}],
        },
    }))
    return cases


class HarnessTrialTest(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        root = Path(os.path.realpath(self.temp.name))
        patcher = mock.patch.object(trial, "CASES", plant_cases(root))
        patcher.start()
        self.addCleanup(patcher.stop)
        self.fixture = Fixture(root / "main")

    def tearDown(self):
        self.temp.cleanup()

    def fresh(self) -> Fixture:
        return Fixture(Path(tempfile.mkdtemp(dir=self.temp.name)))

    def broken(self, check, name, replacement):
        """The check fails once against a runner whose `name` is broken."""
        with mock.patch.object(trial, name, replacement):
            with self.assertRaises(AssertionError, msg=f"the check passed with a broken {name}"):
                check(self.fresh())

    # --- The refusals ---------------------------------------------------------

    def check_refusals(self, fixture: Fixture):
        for name in trial.LOCATION_NAMES:
            for value in (str(fixture.owner / ".claude"), ""):
                status, output = fixture.run(**{name: value})
                self.assertEqual(status, 1, f"{name}={value!r} did not refuse: {output}")
                self.assertIn(f"the environment holds {name}", output)
                self.assertFalse(fixture.launched_any(), f"{name}: a harness or driver launched before the refusal")

    def test_each_location_variable_refuses_before_any_launch(self):
        self.check_refusals(self.fixture)
        self.broken(self.check_refusals, "refuse_locations", lambda environ: None)

    def test_a_foreign_configuration_directory_refuses(self):
        self.fixture.behave(configDirectory=str(self.fixture.owner / ".claude"))
        status, output = self.fixture.run()
        self.assertEqual(status, 1)
        self.assertIn("reports the configuration directory", output)

    # --- The allow-list and the authority control ----------------------------

    def check_allow_list(self, fixture: Fixture):
        status, output = fixture.run(CLAUDE_CODE_PLANTED="planted")
        self.assertEqual(status, 0, output)
        driver = json.loads(fixture.grove_env.read_text())
        self.assertLessEqual(set(driver), ALLOWED, "the driver received a name outside the allow-list")
        prepared = json.loads((fixture.prepared / "prepared.json").read_text())
        self.assertEqual(driver["PATH"].split(":")[0], str(Path(prepared["ravel"]["front"]).parent),
                         "the 2.0 front is not first on PATH")
        record = fixture.record()
        self.assertEqual(record["launches"][0]["control"], [])
        self.assertNotIn("CLAUDE_CODE_PLANTED", record["launches"][0]["envNames"])

    def test_the_driver_receives_only_the_allow_list(self):
        self.check_allow_list(self.fixture)

        def leaky(environ, home, tmp, path_dirs):
            return {**environ, "HOME": str(home), "TMPDIR": str(tmp), "PATH": ":".join(path_dirs)}

        self.broken(self.check_allow_list, "allowed_env", leaky)

    def check_control_fires(self, fixture: Fixture):
        """A runner that plants a live name in the driver's environment: the
        name reaches the child, and the control fails the run."""
        real = trial.allowed_env

        def planting(environ, home, tmp, path_dirs):
            return {**real(environ, home, tmp, path_dirs), "CLAUDE_CODE_PLANTED": "planted"}

        with mock.patch.object(trial, "allowed_env", planting):
            status, output = fixture.run(CLAUDE_CODE_PLANTED="planted")
        self.assertEqual(status, 1, output)
        self.assertIn("CLAUDE_CODE_PLANTED reached the trial child", output)

    def test_a_planted_name_fails_the_control_once(self):
        self.check_control_fires(self.fixture)
        self.broken(self.check_control_fires, "child_control", lambda child, live: [])

    def test_the_control_compares_values_not_only_names(self):
        live = {"RAVEL_EXIT_FILE": "/live/exit", "GROVE_LAUNCH_DIR": "/live/launch"}
        self.assertEqual(trial.child_control({"RAVEL_EXIT_FILE": "/trial/exit", "GROVE_LAUNCH_DIR": "/t"}, live), [])
        self.assertEqual(trial.child_control({"RAVEL_EXIT_FILE": "/live/exit"}, live),
                         ["RAVEL_EXIT_FILE reached the trial child with the live session's value"])
        self.assertEqual(trial.child_control({"CLAUDECODE": "1"}, {}), ["CLAUDECODE reached the trial child"])

    # --- The link list --------------------------------------------------------

    def check_links(self, fixture: Fixture):
        status, output = fixture.run()
        self.assertEqual(status, 0, output)
        links = [line for line in output.splitlines()
                 if line.startswith("harness-trial: link ") and not line.startswith("harness-trial: link after")]
        self.assertEqual(links, [f"harness-trial: link {fixture.store} -> {fixture.owner / fixture.store}"])
        self.assertEqual(len([line for line in output.splitlines() if "link after" in line]), 1)
        for change in ({"replaceLink": True}, {"extraLink": True}):
            other = self.fresh()
            other.behave(**change)
            status, output = other.run()
            self.assertEqual(status, 1, f"{change}: the run passed: {output}")
            self.assertIn("the sandbox home holds the links", output)
            self.assertTrue((other.owner / other.store).exists(), "the real store changed")

    def test_the_link_list_holds_only_the_sign_in_store_before_and_after(self):
        self.check_links(self.fixture)
        self.broken(self.check_links, "check_links", lambda links, store, owner, when: None)

    def test_no_run_changes_the_real_home_and_the_record_keeps_no_link(self):
        before = sorted(str(p) for p in self.fixture.owner.rglob("*"))
        status, output = self.fixture.run()
        self.assertEqual(status, 0, output)
        self.assertEqual(sorted(str(p) for p in self.fixture.owner.rglob("*")), before)
        trial_home = Path(self.fixture.record()["trial"]) / "home"
        self.assertEqual(trial.list_links(trial_home), [], "the trial directory keeps the sign-in link")

    # --- Event extraction -----------------------------------------------------

    def check_events(self, fixture: Fixture):
        status, output = fixture.run()
        self.assertEqual(status, 0, output)
        launch = fixture.record()["launches"][0]
        main = [t for t in launch["transcripts"] if t["main"]]
        self.assertEqual(len(main), 1)
        events = [(e["tool"], e["class"], e.get("skill"), e["error"]) for e in main[0]["events"]]
        self.assertEqual(events, EXPECTED)
        self.assertEqual([e["seq"] for e in main[0]["events"]], list(range(1, len(EXPECTED) + 1)))
        self.assertEqual(fixture.record()["ending"]["endingCommands"], ["grove-continue"])
        by_tool = {e["tool"]: e for e in main[0]["events"]}
        self.assertEqual(by_tool["text"]["target"], "The native loader has no match, so I search the directories.")
        self.assertEqual(by_tool["Skill"]["resultText"], "Skill result")
        startup = main[0]["startup"]
        self.assertEqual(startup["init"]["permissionMode"], "bypassPermissions")
        self.assertNotIn("session_id", startup["init"])
        self.assertNotIn("secret-org", json.dumps(launch), "account data reached the record")
        identity = launch["descriptorIdentity"]
        self.assertEqual(identity["directory"], identity["sha256"])
        self.assertEqual(launch["sources"][0]["resourceDigest"], "ab" * 32)
        # Each snapshot read names its source, and a read outside every source names none.
        sources = {(e["class"], e.get("skill")): e.get("source") for e in main[0]["events"]}
        self.assertEqual(sources[("skill", "craft")], "grove")
        self.assertEqual(sources[("kind-skill", "grove-impl")], "grove")
        self.assertIsNone(sources[("native", "old")])
        markdown = (Path(fixture.record()["trial"]) / "record.md").read_text()
        self.assertIn("| 5 | Read | skill | craft | grove |", markdown)

    def test_extraction_orders_and_classifies_every_tool_call(self):
        self.check_events(self.fixture)
        # A runner that names no source loses which copy a read opened.
        with (mock.patch.object(trial.Classifier, "source_of", lambda self, raw: None),
              self.assertRaises(AssertionError, msg="the check passed with a broken source_of")):
            self.check_events(self.fresh())
        real = trial.extract_events
        self.broken(self.check_events, "extract_events", lambda entries, c: list(reversed(real(entries, c))))
        self.broken(self.check_events, "extract_events",
                    lambda entries, c: [e for e in real(entries, c) if e["tool"] != "Bash"])
        # A runner that ignores `cd` resolves relative reads against the wrong directory.
        self.broken(self.check_events, "bash_paths", lambda command, cwd: trial.PATH_TOKEN.findall(command))
        # A runner that expands no assigned variable misses the read in it.
        self.broken(self.check_events, "expand", lambda token, variables: None if "$" in token else token)
        # A runner that drops the result text loses what the native loader said.
        self.broken(self.check_events, "result_text", lambda content: "")

    def check_reextract(self, fixture: Fixture):
        status, output = fixture.run()
        self.assertEqual(status, 0, output)
        trial_dir = Path(fixture.record()["trial"])
        record = json.loads((trial_dir / "record.json").read_text())
        record["launches"][0]["transcripts"][0]["events"] = []
        (trial_dir / "record.json").write_text(json.dumps(record))
        with contextlib.redirect_stdout(io.StringIO()):
            self.assertEqual(trial.main(["reextract", "--trial", str(trial_dir)], fixture.environ()), 0)
        again = json.loads((trial_dir / "record.json").read_text())
        events = [(e["tool"], e["class"], e.get("skill"), e["error"]) for e in again["launches"][0]["transcripts"][0]["events"]]
        self.assertEqual(events, EXPECTED)
        self.assertEqual(again["launches"][0]["control"], record["launches"][0]["control"])
        self.assertIn("| Events extracted again |", (trial_dir / "record.md").read_text())

    def test_a_run_can_be_extracted_again_from_its_transcripts(self):
        self.check_reextract(self.fixture)
        # A command that rewrites nothing leaves the emptied events empty.
        self.broken(self.check_reextract, "reextract", lambda trial_dir: {})

    def test_the_record_names_builds_harness_and_ending(self):
        status, output = self.fixture.run()
        self.assertEqual(status, 0, output)
        record = self.fixture.record()
        self.assertEqual(record["harness"]["version"], "9.9.9 (Claude Code)")
        self.assertEqual(record["harness"]["model"], "claude-opus-5-5")
        self.assertEqual(record["ending"]["launches"], 1)
        self.assertFalse(record["ending"]["teardownRecorded"])
        self.assertEqual(record["ending"]["treeBefore"], record["ending"]["treeAfter"])
        self.assertEqual(record["skills"], {"nativeCopies": [], "residue": [], "installed": [],
                                            "claudeSettings": None, "planted": []})
        markdown = (Path(record["trial"]) / "record.md").read_text()
        for needle in ("| Route | directory-2.0 |", "Native startup text:", "| 4 | Bash | kind-skill | grove-impl |",
                       "Harness launches: 1."):
            self.assertIn(needle, markdown)

    # --- Native copies, sandbox settings and planted files ---------------------

    def check_explicit(self, fixture: Fixture):
        owner_before = sorted(str(p) for p in fixture.owner.rglob("*"))
        status, output = fixture.run("explicit")
        self.assertEqual(status, 0, output)
        record = fixture.record()
        trial_dir = Path(record["trial"])
        home = trial_dir / "home"
        self.assertEqual(record["skills"]["nativeCopies"], [".claude/skills/craft/SKILL.md"])
        self.assertEqual(record["skills"]["installed"][0]["skillSha256"],
                         trial.sha256_bytes(b"craft\n"))
        self.assertTrue((home / ".claude/settings.json").is_file(), "the case's settings were not written")
        self.assertEqual(json.loads((home / ".claude/settings.json").read_text()),
                         {"permissions": {"deny": ["Skill(craft)"]}})
        self.assertTrue((trial_dir / "guidance/craft/SKILL.md").is_file(), "the case's file was not planted")
        self.assertEqual((trial_dir / "guidance/craft/SKILL.md").read_text(), "craft\n")
        task = (trial_dir / "repo/.grove/02-impl--path-k2.md").read_text()
        self.assertEqual(task, f"Read `{trial_dir}/guidance/craft/SKILL.md`.\n")
        self.assertEqual(sorted(str(p) for p in fixture.owner.rglob("*")), owner_before,
                         "a native copy or a setting reached the owner's home")
        markdown = (trial_dir / "record.md").read_text()
        for needle in ("| Installed native copy | `.claude/skills/craft` from", "| Planted file | `$TRIAL/guidance/craft/SKILL.md`",
                       "| Sandbox `.claude/settings.json` |"):
            self.assertIn(needle, markdown)

    def test_a_case_installs_native_copies_settings_and_planted_files(self):
        self.check_explicit(self.fixture)
        self.broken(self.check_explicit, "install_native", lambda case, home, checkout: [])
        self.broken(self.check_explicit, "write_claude_settings", lambda case, home: None)
        self.broken(self.check_explicit, "plant_files", lambda case, trial_dir, checkout: [])
        real_plant = trial.plant_repo

        def unsubstituted(repo, tree, env):
            with mock.patch.object(trial, "TRIAL_TOKEN", "{{NOT-A-TOKEN}}"):
                return real_plant(repo, tree, env)

        self.broken(self.check_explicit, "plant_repo", unsubstituted)

    def check_stale(self, fixture: Fixture):
        status, output = fixture.run("stale")
        self.assertEqual(status, 0, output)
        record = fixture.record()
        installed = record["skills"]["installed"][0]
        self.assertEqual(installed.get("commit"), fixture.old_commit)
        self.assertEqual(installed["skillSha256"], trial.sha256_bytes(b"craft old\n"))
        self.assertEqual(installed["currentSha256"], trial.sha256_bytes(b"craft\n"))
        home = Path(record["trial"]) / "home"
        self.assertEqual((home / ".claude/skills/craft/SKILL.md").read_text(), "craft old\n")
        self.assertIn(f"at commit `{fixture.old_commit[:12]}`", (Path(record["trial"]) / "record.md").read_text())

    def test_a_native_copy_can_come_from_an_older_commit(self):
        self.check_stale(self.fixture)
        real = trial.install_native

        def current_only(case, home, checkout):
            plain = {**case, "native": [{k: v for k, v in item.items() if k != "commit"} for item in case["native"]]}
            return real(plain, home, checkout)

        self.broken(self.check_stale, "install_native", current_only)

    def test_a_planted_file_outside_the_guidance_directory_refuses(self):
        case = {"name": "x", "plant": [{"from": "plugins/grove/skills/craft/SKILL.md", "to": "../escape"}]}
        with self.assertRaises(trial.TrialError):
            trial.plant_files(case, self.fixture.root, self.fixture.root / "source")

    def check_stray_native(self, fixture: Fixture):
        fixture.behave(strayNative=True)
        status, output = fixture.run()
        self.assertEqual(status, 1, output)
        self.assertIn("holds the native copies ['.claude/skills/stray/SKILL.md']", output)

    def test_an_undeclared_native_copy_fails_the_run(self):
        self.check_stray_native(self.fixture)
        self.broken(self.check_stray_native, "native_failures", lambda native, expected: [])

    def check_unstarted(self, fixture: Fixture):
        fixture.behave(noInit=True)
        status, output = fixture.run()
        self.assertEqual(status, 1, output)
        self.assertIn("launch 0: the harness wrote no startup event", output)

    def test_a_harness_that_starts_no_session_fails_the_run(self):
        self.check_unstarted(self.fixture)
        self.broken(self.check_unstarted, "started", lambda launch: True)

    # --- A source moved away -----------------------------------------------------

    def check_source_missing(self, fixture: Fixture):
        status, output = fixture.run("missing")
        self.assertEqual(status, 0, output)
        record = fixture.record()
        self.assertEqual(record["launches"], [])
        refusal = record["refusal"]
        self.assertEqual(refusal["moved"]["exit"], 3)
        self.assertIn("skill_source_missing", refusal["moved"]["firstLine"])
        self.assertEqual(refusal["restored"]["status"], "resolved")
        self.assertTrue(Path(refusal["clone"]["checkout"]).is_dir(), "the clone was not restored")
        self.assertEqual(refusal["clone"]["commit"], json.loads(
            (fixture.prepared / "prepared.json").read_text())["source"]["commit"])
        self.assertEqual(record["sources"]["settings"]["skills"]["sources"][0]["path"],
                         str(Path(refusal["clone"]["checkout"]) / "plugins/grove"))
        # The harness answered only the version and sign-in questions.
        self.assertEqual(fixture.launched.read_text().split(), ["--version", "auth"])
        self.assertTrue(any("ravel: refused (skill_source_missing" in line for line in record["ending"]["driverLines"]))
        self.assertIn("Expected refusal:", (Path(record["trial"]) / "record.md").read_text())

    def test_a_moved_source_refuses_before_any_launch(self):
        self.check_source_missing(self.fixture)
        self.broken(self.check_source_missing, "move_away", lambda moved: None)
        self.broken(self.check_source_missing, "restore", lambda moved: None)
        # A runner whose settings ignore the clone moves a directory that no launch reads.
        real_settings = trial.settings_for
        self.broken(self.check_source_missing, "settings_for",
                    lambda case, checkout, clones=None: real_settings(case, checkout))

    def check_refusal_needs_no_launch(self, fixture: Fixture):
        with mock.patch.object(trial, "move_away", lambda moved: None):
            status, output = fixture.run("missing")
        self.assertEqual(status, 1, output)
        self.assertIn("the driver launched a harness", output)
        self.assertIn("the driver did not report Ravel's refusal", output)

    def test_a_refusal_case_fails_when_the_harness_launches(self):
        self.check_refusal_needs_no_launch(self.fixture)
        self.broken(self.check_refusal_needs_no_launch, "refusal_failures", lambda *args: [])

    def test_move_away_needs_an_expected_refusal(self):
        case = json.loads((trial.CASES / "missing" / "case.json").read_text())
        (trial.CASES / "half").mkdir()
        shutil.copytree(trial.CASES / "smoke" / "tree", trial.CASES / "half" / "tree")
        (trial.CASES / "half" / "case.json").write_text(json.dumps({**case, "expectRefusal": None}))
        with self.assertRaises(trial.TrialError):
            trial.load_case("half")

    # --- The offline resolution ------------------------------------------------

    def check_resolve(self, fixture: Fixture):
        status, output = fixture.run("resolve")
        self.assertEqual(status, 0, output)
        record = fixture.record()
        resolve = record["resolve"]
        self.assertEqual(resolve["descriptorIdentity"], record["launches"][0]["descriptorIdentity"])
        self.assertIsNotNone(resolve["descriptorIdentity"])
        self.assertEqual(resolve["networkControl"]["unguardedExit"], 0)
        self.assertNotEqual(resolve["networkControl"]["guardedExit"], 0)
        self.assertIsNone(resolve["gitControl"]["resolutionPath"])
        self.assertIn("Offline resolution:", (Path(record["trial"]) / "record.md").read_text())

    @unittest.skipUnless(platform.system() == "Darwin", "the profile is a Seatbelt profile")
    def test_the_offline_resolution_denies_the_network_and_git(self):
        self.check_resolve(self.fixture)
        # A profile that allows the network: the guarded probe reaches the listener.
        with mock.patch.object(trial, "NO_NETWORK", "(version 1)(allow default)"):
            self.broken(self.check_resolve, "resolve_failures", trial.resolve_failures)
        git = os.path.dirname(shutil.which("git"))
        self.broken(self.check_resolve, "resolution_path", lambda front: f"{front.parent}:{git}")

    def test_a_different_descriptor_fails_the_offline_resolution(self):
        launch = {"descriptorIdentity": {"directory": "a", "sha256": "a"}}
        resolve = {"networkControl": {"unguardedExit": 0, "guardedExit": 1},
                   "gitControl": {"usualPath": "/usr/bin/git", "resolutionPath": None},
                   "exit": 0, "status": "resolved", "stderr": "",
                   "descriptorIdentity": {"directory": "b", "sha256": "b"}}
        failures = trial.resolve_failures(resolve, [launch])
        self.assertEqual(len(failures), 1)
        self.assertIn("the offline resolution gave the descriptor", failures[0])
        self.assertEqual(trial.resolve_failures({**resolve, "descriptorIdentity": launch["descriptorIdentity"]},
                                                [launch]), [])

    # --- The confined standalone mode ----------------------------------------

    def check_confined(self, fixture: Fixture):
        fixture.behave(noSession=True)
        status, output = fixture.run("confined")
        self.assertEqual(status, 0, output)
        record = fixture.record()
        self.assertEqual(record["mode"], "standalone")
        self.assertEqual(len(record["launches"]), 1)
        launch = record["launches"][0]
        self.assertEqual(launch["control"], [])
        main = [t for t in launch["transcripts"] if t["main"]]
        self.assertEqual([t["file"] for t in main], ["stream"])
        self.assertEqual([(e["tool"], e["class"], e.get("skill"), e["error"]) for e in main[0]["events"]], EXPECTED)
        trial_dir = Path(record["trial"])
        home = trial_dir / "home"
        checkout = fixture.root / "source"
        expected = [
            ("read", "descriptor"), ("read", "mandatory"), ("write", "snapshot"),
            ("read", str(home / ".config/ravel/settings.json")), ("read", str(home / ".config/ravel/config.rhai")),
            ("read", str(checkout / "plugins/grove/skills/grove/SKILL.md")),
            ("write", str(home / "trial-probe-outside")), ("write", "trial-probe-inside"),
        ]
        probes = launch["probes"]
        self.assertEqual([p["op"] for p in probes], [op for op, _ in expected])
        for probe, (_, path) in list(zip(probes, expected))[3:]:
            self.assertEqual(probe["path"], path)
        self.assertEqual([(p["op"], p["path"]) for p in record["contrast"]], [(p["op"], p["path"]) for p in probes])
        self.assertFalse((home / "trial-probe-outside").exists(), "a write probe left its file")
        self.assertEqual([line.split()[0] for line in record["ending"]["published"]], ["stamp.sh"])
        self.assertIn("ravel: a line of the run log", record["ending"]["runLog"])
        markdown = (trial_dir / "record.md").read_text()
        self.assertIn("| Mode | standalone |", markdown)
        self.assertIn("| write | `trial-probe-inside` | ok | ok |", markdown)

    def test_a_confined_run_records_its_probes_and_its_stream(self):
        self.check_confined(self.fixture)
        self.broken(self.check_confined, "split_log", lambda text: [])
        self.broken(self.check_confined, "contrast_probes", lambda probes, cwd: [])
        self.broken(self.check_confined, "fixed_probes", lambda case, home, checkout: [])

    def test_the_wrapper_reports_a_failed_access(self):
        directory = Path(tempfile.mkdtemp(dir=self.temp.name))
        readable = directory / "readable"
        readable.write_text("x\n")
        locked = directory / "locked"
        locked.mkdir(mode=0o500)
        result = trial.contrast_probes([
            {"op": "read", "path": str(readable)}, {"op": "read", "path": str(directory / "absent")},
            {"op": "write", "path": str(locked / "probe")}, {"op": "write", "path": str(directory / "probe")},
        ], directory)
        self.assertEqual([p["status"] for p in result], ["ok", "failed", "failed", "ok"])
        self.assertFalse((directory / "probe").exists())

    # --- Continuation ----------------------------------------------------------

    def check_continuation(self, fixture: Fixture):
        fixture.behave(resumeTools=RESUME_TOOLS)
        status, output = fixture.run("continuation")
        self.assertEqual(status, 0, output)
        record = fixture.record()
        launches = record["launches"]
        self.assertEqual([launch.get("role") for launch in launches], ["first turn", "resumed turn", "fresh launch"])
        first, resumed, fresh = launches

        def main(launch):
            return next(t for t in launch["transcripts"] if t["main"])

        # The resumed turn appends to the first turn's session file, and each
        # turn holds only its own lines.
        self.assertEqual(first["sessionId"], resumed["sessionId"])
        self.assertNotEqual(fresh["sessionId"], first["sessionId"])
        self.assertIn("lines", main(first), "the first turn has no window")
        self.assertIn("lines", main(resumed), "the resumed turn has no window")
        self.assertEqual(main(first)["lines"][1], main(resumed)["lines"][0])
        self.assertEqual([(e["tool"], e["class"], e.get("skill"), e["error"]) for e in main(first)["events"]], EXPECTED)
        # The first turn's snapshot gives the original bytes, and the source
        # origin gives the changed bytes.
        self.assertEqual([(e["class"], e.get("skill"), e["marker"]) for e in main(resumed)["events"]],
                         [("mandatory", "grove", []), ("skill", "craft", []), ("source-origin", "craft", ["result"])])
        crafts = {name: [e["marker"] for e in main(launch)["events"] if e["class"] == "skill"]
                  for name, launch in (("first", first), ("fresh", fresh))}
        self.assertEqual(crafts, {"first": [[]], "fresh": [["result"]]})
        self.assertEqual(resumed["descriptorIdentity"], first["descriptorIdentity"])
        self.assertNotEqual(fresh["descriptorIdentity"], first["descriptorIdentity"])
        self.assertEqual(first["snapshotMarkers"], {CRAFT: False})
        self.assertEqual(fresh["snapshotMarkers"], {CRAFT: True})
        # The resumed turn runs outside Ravel, so it has no descriptor variable.
        self.assertIn("RAVEL_SKILL_DIRECTORIES", first["envNames"])
        self.assertNotIn("RAVEL_SKILL_DIRECTORIES", resumed["envNames"])
        self.assertEqual(resumed["control"], [])
        continuation = record["continuation"]
        self.assertEqual(continuation["failures"], [])
        self.assertEqual(continuation["resume"]["args"][-2:], ["--resume", first["sessionId"]])
        self.assertNotIn("--add-dir", continuation["resume"]["args"])
        self.assertNotEqual(continuation["edit"]["commitBefore"], continuation["edit"]["commitAfter"])
        trial_dir = Path(record["trial"])
        settings = json.loads((trial_dir / "home/.config/ravel/settings.json").read_text())["skills"]
        self.assertEqual(settings["mandatory"], continuation["settings"]["mandatoryAfter"])
        self.assertEqual(settings["sources"][0]["path"], str(trial_dir / "sources/checkout/plugins/grove"))
        self.assertIn("02-impl--fresh-k2.md", record["ending"]["treeAfter"])
        self.assertTrue((fixture.root / "source" / CRAFT).read_text() == "craft\n", "the prepared checkout changed")
        markdown = (trial_dir / "record.md").read_text()
        for needle in ("Continuation:", "#### Launch 1: kind `impl`, resumed turn", " Marker |",
                       f"- Snapshot copy of `{CRAFT}`: holds the marker."):
            self.assertIn(needle, markdown)
        # A second extraction keeps each turn's window.
        with contextlib.redirect_stdout(io.StringIO()):
            self.assertEqual(trial.main(["reextract", "--trial", str(trial_dir)], fixture.environ()), 0)
        again = json.loads((trial_dir / "record.json").read_text())
        self.assertEqual([main(launch)["events"] for launch in again["launches"]],
                         [main(launch)["events"] for launch in launches])

    def test_a_continuation_resumes_after_a_source_commit_and_launches_fresh(self):
        self.check_continuation(self.fixture)
        self.broken(self.check_continuation, "resume_turn", lambda *args, **kwargs: {
            "args": [], "cwd": "", "answerSha256": "", "exit": 0, "timedOut": False, "elapsedS": 0})
        self.broken(self.check_continuation, "commit_edits", lambda clone, edits: {
            "checkout": str(clone), "commitBefore": "x", "commitAfter": "x", "files": []})
        self.broken(self.check_continuation, "write_window", lambda record, start, end: None)
        self.broken(self.check_continuation, "read_window", lambda record: None)
        self.broken(self.check_continuation, "marker_places", lambda marker, tool_input, result: [])
        self.broken(self.check_continuation, "snapshot_markers", lambda case, launch: {})
        self.broken(self.check_continuation, "add_fresh_tree", lambda repo, tree, env: {"added": [], "commit": "x" * 12})

    def test_a_continuation_case_needs_its_parts(self):
        case = json.loads((trial.CASES / "continuation" / "case.json").read_text())
        for change in ({"marker": ""}, {"edits": []}, {"edits": [{"path": CRAFT, "insert": "no marker"}]},
                       {"edits": [{"path": "elsewhere/SKILL.md", "insert": MARKER}]}, {"freshTree": "absent"}):
            name = "broken-" + "-".join(change)
            shutil.copytree(trial.CASES / "continuation", trial.CASES / name, dirs_exist_ok=True)
            (trial.CASES / name / "case.json").write_text(json.dumps(
                {**case, "name": name, "continuation": {**case["continuation"], **change}}))
            with self.assertRaises(trial.TrialError, msg=f"{change} loaded"):
                trial.load_case(name)
        trial.load_case("continuation")

    def test_an_insertion_follows_the_front_matter(self):
        self.assertEqual(trial.insert_after_front_matter("---\nname: x\n---\n\n# X\n", "M."),
                         "---\nname: x\n---\n\nM.\n\n# X\n")
        self.assertEqual(trial.insert_after_front_matter("# X\n", "M."), "M.\n\n# X\n")

    def test_marker_files_skip_the_tree_and_the_stores(self):
        repo = Path(tempfile.mkdtemp(dir=self.temp.name))
        for name in ("stamp.sh", ".grove/leaf.md", ".jj/x", "tools/count.sh", "plain.txt"):
            (repo / name).parent.mkdir(parents=True, exist_ok=True)
            (repo / name).write_text(f"# {MARKER}\n" if name != "plain.txt" else "plain\n")
        self.assertEqual(trial.marker_files(repo, MARKER), ["stamp.sh", "tools/count.sh"])

    def test_the_configuration_routes_only_the_case_kinds(self):
        route = trial.route_for({"kinds": ["impl"], "name": "x"}, Path("/r"), Path("/c"), "m", "e")
        config = trial.config_for({"kinds": ["impl"], "name": "x"}, route)
        self.assertIn('const KINDS = #{ "impl": true };', config)
        self.assertIn('"trial_kind_unrouted"', config)
        self.assertIn("args.push(request.prompt);", config)
        self.assertIn('"--permission-mode", "bypassPermissions"', config)
        # The variadic --add-dir comes before the fixed flags, never before the prompt.
        self.assertLess(config.index('"--add-dir"'), config.index('"-p"'))


if __name__ == "__main__":
    unittest.main(verbosity=2)
