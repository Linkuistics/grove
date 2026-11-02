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
route = json.loads((trial / "route.json").read_text())
snap = home / ".cache/ravel/skills/objects/grove-snapshot"
(snap / "skills/grove-impl").mkdir(parents=True)
(snap / "skills/grove-impl/SKILL.md").write_text("kind\n")
(snap / "skills/grove/references").mkdir(parents=True)
(snap / "skills/grove/SKILL.md").write_text("spine\n")
(snap / "skills/grove/references/execute.md").write_text("resource\n")
(snap / "skills/craft").mkdir(parents=True)
(snap / "skills/craft/SKILL.md").write_text("craft\n")
descriptor_text = b'{{"schemaVersion":1}}'
digest = hashlib.sha256(descriptor_text).hexdigest()
descriptor = home / ".cache/ravel/descriptors" / digest / "directories.json"
descriptor.parent.mkdir(parents=True)
descriptor.write_bytes(descriptor_text)
view = {{"kind": "impl", "originalPrompt": "mandate", "skills": {{
    "descriptorPath": str(descriptor),
    "sources": [{{"id": "grove", "type": "local", "resourceDigest": "ab" * 32,
                 "snapshotRoot": str(snap), "originPath": "/origin/grove"}}],
    "directories": [{{"source": "grove", "path": str(snap / "skills")}}],
    "mandatory": [{{"source": "grove", "path": str(snap / "skills/grove/SKILL.md")}}]}}}}
launch = trial / "tmp/launch"
launch.mkdir()
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
snap = sys.argv[sys.argv.index("--add-dir") + 1]
names = {{"SNAP": snap, "HOME": str(home), "DESC": os.environ.get("RAVEL_SKILL_DIRECTORIES", ""),
         "CWD": os.getcwd()}}
session = "11111111-2222-3333-4444-555555555555"
if behaviour.get("noInit"):
    print("Error: Input must be provided", file=sys.stderr)
    sys.exit(1)
print(json.dumps({{"type": "system", "subtype": "init", "session_id": session, "model": "fake",
                  "permissionMode": "bypassPermissions", "tools": ["Read", "Bash"], "skills": [],
                  "claude_code_version": "9.9.9"}}))
entries = [{{"type": "attachment", "attachment": {{"type": "skill_listing", "skillCount": 0, "names": []}}}},
           {{"type": "attachment", "attachment": {{"type": "credential_org", "org": "secret-org"}}}}]
for n, (tool, field, value, error) in enumerate(behaviour["tools"]):
    tid = f"toolu_{{n}}"
    entries.append({{"type": "assistant", "message": {{"content": [
        {{"type": "tool_use", "id": tid, "name": tool, "input": {{field: value.format(**names)}}}}]}}}})
    entries.append({{"type": "user", "message": {{"content": [
        {{"type": "tool_result", "tool_use_id": tid, "is_error": error, "content": "x"}}]}}}})
project = home / ".claude/projects/-fake"
project.mkdir(parents=True, exist_ok=True)
(project / f"{{session}}.jsonl").write_text("".join(json.dumps(e) + "\n" for e in entries))
store = home / {store!r}
if behaviour.get("replaceLink"):
    store.unlink()
    store.mkdir()
if behaviour.get("extraLink"):
    os.symlink("/", home / ".claude/extra")
print(json.dumps({{"type": "result", "subtype": "success"}}))
'''

# The transcript of the fake session, in tool-call order, with the class that
# extraction must give each call.
TOOLS = [
    ("Read", "file_path", "{SNAP}/skills/grove/SKILL.md", False),
    ("Read", "file_path", "{SNAP}/skills/grove/references/execute.md", False),
    ("Bash", "command", "cd \"{SNAP}/skills\" && find . -name SKILL.md | while read f; do head -8 \"$f\"; done", False),
    ("Bash", "command", "cd \"{SNAP}/skills\" && cat grove-impl/SKILL.md; ls grove-impl", False),
    ("Read", "file_path", "{SNAP}/skills/craft/SKILL.md", False),
    ("Read", "file_path", "{DESC}", False),
    ("Read", "file_path", "{HOME}/.claude/skills/old/SKILL.md", True),
    ("Skill", "skill", "grove-impl", True),
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
    ("Read", "descriptor", None, False),
    ("Read", "native", "old", True),
    ("Skill", "native-skill-tool", None, True),
    ("Write", "action", None, False),
    ("Bash", "command", None, False),
    ("Bash", "ending", None, False),
]


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
        self._exec(front, "#!/bin/sh\nexit 0\n")
        checkout = root / "source"
        (checkout / "plugins/grove/skills").mkdir(parents=True)
        (self.prepared / "prepared.json").write_text(json.dumps({
            "grove": {"commit": "c" * 40, "prefix": str(grove_bin.parent),
                      "sha256": {name: trial.sha256_file(grove_bin / name) for name in ("grove", "grove-llm")}},
            "source": {"checkout": str(checkout), "commit": "c" * 40},
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
    def _exec(path: Path, text: str) -> None:
        path.write_text(text)
        path.chmod(0o755)

    def behave(self, **changes) -> None:
        self.behaviour.write_text(json.dumps({"tools": TOOLS, **changes}))

    def environ(self, **extra) -> dict[str, str]:
        return {"HOME": str(self.owner), "PATH": os.environ["PATH"], "USER": "owner",
                "HARNESS_TRIAL_PROGRAM": str(self.claude), **LIVE, **extra}

    def run(self, **extra) -> tuple[int, str]:
        """Run the smoke case; return the status and everything it printed."""
        output = io.StringIO()
        with contextlib.redirect_stdout(output), contextlib.redirect_stderr(output):
            status = trial.main(["run", "--prepared", str(self.prepared), "--case", "smoke"], self.environ(**extra))
        return status, output.getvalue()

    def launched_any(self) -> bool:
        return self.launched.exists() or self.grove_env.exists()

    def record(self) -> dict:
        runs = sorted((self.prepared / "runs").iterdir())
        return json.loads((runs[-1] / "record.json").read_text())


class HarnessTrialTest(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.fixture = Fixture(Path(os.path.realpath(self.temp.name)))

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
        startup = main[0]["startup"]
        self.assertEqual(startup["init"]["permissionMode"], "bypassPermissions")
        self.assertNotIn("session_id", startup["init"])
        self.assertNotIn("secret-org", json.dumps(launch), "account data reached the record")
        identity = launch["descriptorIdentity"]
        self.assertEqual(identity["directory"], identity["sha256"])
        self.assertEqual(launch["sources"][0]["resourceDigest"], "ab" * 32)

    def test_extraction_orders_and_classifies_every_tool_call(self):
        self.check_events(self.fixture)
        real = trial.extract_events
        self.broken(self.check_events, "extract_events", lambda entries, c: list(reversed(real(entries, c))))
        self.broken(self.check_events, "extract_events",
                    lambda entries, c: [e for e in real(entries, c) if e["tool"] != "Bash"])
        # A runner that ignores `cd` resolves relative reads against the wrong directory.
        self.broken(self.check_events, "bash_paths", lambda command, cwd: trial.PATH_TOKEN.findall(command))

    def test_the_record_names_builds_harness_and_ending(self):
        status, output = self.fixture.run()
        self.assertEqual(status, 0, output)
        record = self.fixture.record()
        self.assertEqual(record["harness"]["version"], "9.9.9 (Claude Code)")
        self.assertEqual(record["harness"]["model"], "claude-opus-5-5")
        self.assertEqual(record["ending"]["launches"], 1)
        self.assertFalse(record["ending"]["teardownRecorded"])
        self.assertEqual(record["ending"]["treeBefore"], record["ending"]["treeAfter"])
        self.assertEqual(record["skills"], {"nativeCopies": [], "residue": []})
        markdown = (Path(record["trial"]) / "record.md").read_text()
        for needle in ("| Route | directory-2.0 |", "Native startup text:", "| 4 | Bash | kind-skill | grove-impl |",
                       "Harness launches: 1."):
            self.assertIn(needle, markdown)

    def check_unstarted(self, fixture: Fixture):
        fixture.behave(noInit=True)
        status, output = fixture.run()
        self.assertEqual(status, 1, output)
        self.assertIn("launch 0: the harness wrote no startup event", output)

    def test_a_harness_that_starts_no_session_fails_the_run(self):
        self.check_unstarted(self.fixture)
        self.broken(self.check_unstarted, "started", lambda launch: True)

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
