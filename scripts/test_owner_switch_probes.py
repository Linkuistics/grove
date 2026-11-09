#!/usr/bin/env python3
"""Fixture controls of scripts/owner_switch_probes.py and of the switch's probe-service cleanup.

"Probe process lifetime" of docs/specs/owner-skill-delivery.md owns the rules.
Fake harnesses first on PATH stand for Claude Code and Codex, and a stub `brew`
names a prefix whose `bin/ravel` is a fake. No control launches a real harness.
Each control ends the fixture processes that it starts.

    python3 -I scripts/test_owner_switch_probes.py [-v] [NAME...]
"""

from __future__ import annotations

import json
import os
import shutil
import signal
import subprocess
import sys
import tempfile
import time
import unittest

HERE = os.path.dirname(os.path.realpath(__file__))
ROOT = os.path.dirname(HERE)
PROBES = os.path.join(HERE, "owner_switch_probes.py")
PLAN = os.path.join(ROOT, "docs", "owner-cutover", "switch-plan-draft.json")
sys.path.insert(0, HERE)
import owner_switch as osw  # noqa: E402
import owner_switch_probes as probes  # noqa: E402

PY = os.path.realpath(sys.executable)
MARK = osw.RESULT_MARK

# A fake harness. FAKE_MODE in its own environment cannot select the mode,
# because the launcher passes an allow-listed environment, so the mode is a
# file beside the fake.
FAKE = r'''#!{py}
import json, os, signal, subprocess, sys, time
here = os.path.dirname(os.path.abspath(__file__))
if "--version" in sys.argv:
    print("0.0.0 (fixture)")
    sys.exit(0)
with open(os.path.join(here, "mode.json")) as f:
    mode = json.load(f)
record = os.path.join(here, "record.jsonl")

def note(**item):
    with open(record, "a") as f:
        f.write(json.dumps(item) + "\n")

note(event="start", pid=os.getpid(), env=dict(os.environ), cwd=os.getcwd(), argv=sys.argv)
kind = mode["kind"]
if kind == "service":
    # A service in a new session through an intermediate that exits, so the
    # service reparents to launchd. DELAY keeps the intermediate alive long
    # enough for the probe to see the parent link.
    pid = os.fork()
    if pid == 0:
        os.setsid()
        child = os.fork()
        if child == 0:
            os.execv("/bin/sleep", ["sleep", "300"])
        note(event="service", pid=child)
        time.sleep(mode.get("delay", 0))
        os._exit(0)
    os.waitpid(pid, 0)
elif kind in ("child", "stubborn-child"):
    # A child in the leader's group, as an MCP server is.
    code = "import signal, time\n"
    if kind == "stubborn-child":
        code += "signal.signal(signal.SIGTERM, signal.SIG_IGN)\nsignal.signal(signal.SIGINT, signal.SIG_IGN)\n"
    code += "time.sleep(300)\n"
    child = subprocess.Popen([sys.executable, "-c", code])
    note(event="service", pid=child.pid)
    time.sleep(mode.get("sleep", 300))
elif kind == "spawning-child":
    # A child that, on SIGTERM, starts a grandchild and exits half a second later.
    code = ("import json, signal, subprocess, sys, time\n"
            "def stop(*_):\n"
            "    g = subprocess.Popen(['/bin/sleep', '300'])\n"
            "    open(sys.argv[1], 'a').write(json.dumps(dict(event='service', pid=g.pid)) + chr(10))\n"
            "    time.sleep(0.5)\n"
            "    sys.exit(0)\n"
            "signal.signal(signal.SIGTERM, stop)\n"
            "time.sleep(300)\n")
    child = subprocess.Popen([sys.executable, "-c", code, record])
    note(event="service", pid=child.pid)
    time.sleep(mode.get("sleep", 300))
elif kind == "sleep":
    time.sleep(mode.get("sleep", 300))
print(json.dumps({{"type": "result", "fixture": kind}}))
'''

FAKE_RAVEL = r'''#!{py}
import json, os, sys
here = os.path.dirname(os.path.abspath(__file__))
with open(os.path.join(here, "resolve.json")) as f:
    doc = json.load(f)
if sys.argv[1:] == ["--version"]:
    print("ravel 2.0.0 (fixture)")
elif sys.argv[1:] == ["skills", "resolve", "--json"]:
    print(json.dumps(doc))
else:
    print("fixture ravel: " + " ".join(sys.argv[1:]), file=sys.stderr)
    sys.exit(2)
'''


def alive(pid: int) -> bool:
    out = subprocess.run(["/bin/ps", "-o", "stat=", "-p", str(pid)], capture_output=True, text=True).stdout.strip()
    return bool(out) and "Z" not in out


def kill_quietly(pid: int) -> None:
    try:
        os.kill(pid, signal.SIGKILL)
    except ProcessLookupError:
        pass


class Fixture(unittest.TestCase):
    """A fake HOME, a folder of fake programs first on PATH, and the fixture processes to end."""

    def setUp(self):
        self.root = os.path.realpath(tempfile.mkdtemp(prefix="probe-controls-", dir="/private/tmp"))
        self.home = os.path.join(self.root, "home")
        self.bin = os.path.join(self.root, "bin")
        os.makedirs(self.home)
        os.makedirs(self.bin)
        self.started = []

    def tearDown(self):
        for pid in self.started + self.service_pids():
            kill_quietly(pid)
        shutil.rmtree(self.root, ignore_errors=True)

    def fake(self, name: str, mode: dict) -> None:
        path = os.path.join(self.bin, name)
        with open(path, "w") as f:
            f.write(FAKE.format(py=PY))
        os.chmod(path, 0o755)
        with open(os.path.join(self.bin, "mode.json"), "w") as f:
            json.dump(mode, f)

    def events(self) -> list:
        path = os.path.join(self.bin, "record.jsonl")
        if not os.path.exists(path):
            return []
        with open(path) as f:
            return [json.loads(line) for line in f]

    def service_pids(self) -> list:
        return [e["pid"] for e in self.events() if e["event"] == "service"]

    def env(self, **extra) -> dict:
        env = {"PATH": self.bin + ":/usr/bin:/bin", "HOME": self.home, "PYTHONHOME": sys.base_prefix,
               "USER": os.environ.get("USER", "owner")}
        env.update(extra)
        return env

    def probe(self, *args, env=None, signal_after=None) -> tuple:
        """Run the probe program; return its exit status, its result object and its standard error."""
        p = subprocess.Popen([PY, "-I", PROBES, *args], env=env or self.env(), cwd=self.root,
                             stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        if signal_after is not None:
            time.sleep(signal_after)
            p.send_signal(signal.SIGTERM)
        out, err = p.communicate(timeout=300)
        result = None
        for line in reversed(out.splitlines()):
            if line.startswith(MARK):
                result = json.loads(line[len(MARK):])
                break
        self.assertIsNotNone(result, f"no result line: {out[-2000:]} {err[-2000:]}")
        self.assertTrue(out.rstrip().splitlines()[-1].startswith(MARK), "the result is not the last line")
        return p.returncode, result, err

    def launch(self, *extra, **kw) -> tuple:
        return self.probe("launch", "--harness", "claude", "--route", "native", "--prompt", "fixture",
                          "--settle", "1", *extra, **kw)


class LauncherControls(Fixture):

    def test_reparented_service_is_recorded_and_ended(self):
        self.fake("claude", {"kind": "service", "delay": 0.5})
        code, result, err = self.launch()
        service = self.service_pids()[0]
        record = next((r for r in result["owned"] if r["pid"] == service), None)
        self.assertIsNotNone(record, f"the service {service} has no record: {result['owned']}")
        self.assertEqual(record["relation"], "descendant")
        self.assertEqual(record["ppid"], 1, "the service did not reparent to launchd")
        self.assertIn(service, [s["pid"] for s in result["survivors"]])
        self.assertIn(service, [s["pid"] for s in result["cleanup"]["signalled"]])
        self.assertFalse(alive(service), "the reparented service still runs")
        self.assertEqual((code, result["ok"]), (0, True), result["problems"])

    def test_unrecorded_escape_is_never_signalled(self):
        # The intermediate exits at once, so the probe normally never sees the
        # service's parent link. A recorded service may be ended; an unrecorded
        # one must stay alive and fail the probe.
        self.fake("claude", {"kind": "service", "delay": 0})
        code, result, err = self.launch()
        service = self.service_pids()[0]
        if service in [r["pid"] for r in result["owned"]]:
            self.assertFalse(alive(service))
            self.assertTrue(result["ok"], result["problems"])
        else:
            self.assertTrue(alive(service), "the probe signalled a process that it did not record")
            self.assertIn(service, [u["pid"] for u in result["unattributed"]])
            self.assertEqual((code, result["ok"]), (1, False))
            self.assertTrue(any("appeared without a record" in p for p in result["problems"]))

    def test_interrupt_ends_the_group(self):
        self.fake("claude", {"kind": "child"})
        code, result, err = self.launch("--interrupt-after", "1")
        child = self.service_pids()[0]
        self.assertEqual(result["leader"]["endedBy"], "interrupt")
        self.assertEqual(result["events"][0]["group"], "SIGINT")
        self.assertFalse(alive(child))
        self.assertEqual(result["cleanup"]["remaining"], [])
        self.assertTrue(result["ok"], result["problems"])

    def test_stubborn_group_member_gets_sigkill(self):
        self.fake("claude", {"kind": "stubborn-child"})
        code, result, err = self.launch("--crash-after", "1")
        child = self.service_pids()[0]
        self.assertEqual(result["leader"]["endedBy"], "crash")
        self.assertIn(child, [s["pid"] for s in result["survivors"]], "the orphan was not a survivor")
        signals = [s["signal"] for s in result["cleanup"]["signalled"] if s["pid"] == child]
        self.assertEqual(signals, ["SIGTERM", "SIGKILL"])
        self.assertFalse(alive(child))
        self.assertTrue(result["ok"], result["problems"])

    def test_child_created_during_cleanup_is_ended(self):
        self.fake("claude", {"kind": "spawning-child"})
        code, result, err = self.launch("--crash-after", "1")
        child, grandchild = self.service_pids()[:2]
        self.assertIn(grandchild, [r["pid"] for r in result["owned"]], "the grandchild has no record")
        self.assertIn(grandchild, [s["pid"] for s in result["cleanup"]["signalled"]])
        self.assertFalse(alive(child) or alive(grandchild))
        self.assertTrue(result["ok"], result["problems"])

    def test_signal_to_the_probe_ends_the_group(self):
        self.fake("claude", {"kind": "child"})
        code, result, err = self.launch(signal_after=2)
        child = self.service_pids()[0]
        self.assertEqual(code, 1)
        self.assertIn("probe received SIGTERM", result["leader"]["endedBy"])
        self.assertTrue(any("SIGTERM" in p for p in result["problems"]))
        self.assertFalse(alive(child))
        self.assertFalse(alive(result["leader"]["pid"]))

    def test_existing_harness_of_the_home_refuses_without_a_signal(self):
        codex = os.path.join(self.root, "daemon", "codex")
        os.makedirs(os.path.dirname(codex))
        if subprocess.run(["/bin/cp", "-c", PY, codex]).returncode != 0:
            shutil.copy2(PY, codex)
        daemon = subprocess.Popen([codex, "-c", "import time; time.sleep(300)", "app-server", "daemon"],
                                  env={"HOME": self.home, "PYTHONHOME": sys.base_prefix}, start_new_session=True)
        self.started.append(daemon.pid)
        self.addCleanup(daemon.wait)
        time.sleep(0.5)
        self.fake("claude", {"kind": "sleep", "sleep": 0})
        code, result, err = self.launch()
        self.assertEqual(code, 1)
        self.assertIn("runs before the launch", result["problems"][0])
        self.assertIn(str(daemon.pid), result["problems"][0])
        self.assertTrue(alive(daemon.pid), "the probe signalled an existing daemon")
        self.assertEqual(self.events(), [], "the probe launched a harness beside an existing daemon")

    def test_harness_environment_is_allow_listed(self):
        self.fake("claude", {"kind": "sleep", "sleep": 0})
        env = self.env(RAVEL_EXIT_FILE="/x", CODEX_HOME="/x", CLAUDE_CONFIG_DIR="/x", GROVE_LAUNCH_DIR="/x",
                       CLAUDECODE="1", OWNER_SWITCH_PLAN="/x", LANG="en_US.UTF-8")
        code, result, err = self.launch(env=env)
        started = next(e for e in self.events() if e["event"] == "start")
        # macOS can add __CF_USER_TEXT_ENCODING to a child's environment.
        self.assertEqual(set(started["env"]) - {"__CF_USER_TEXT_ENCODING"},
                         {"PATH", "HOME", "USER", "LANG", "DISABLE_AUTOUPDATER"})
        self.assertEqual(started["env"]["DISABLE_AUTOUPDATER"], "1")
        self.assertEqual(started["env"]["HOME"], self.home)
        self.assertTrue(started["cwd"].startswith("/private/tmp/owner-switch-probe-"))
        self.assertFalse(os.path.exists(started["cwd"]), "the probe kept its work directory")
        self.assertIn("RAVEL_EXIT_FILE", result["droppedNames"])
        self.assertTrue(result["ok"], result["problems"])

    def test_journal_names_each_record_as_it_appears(self):
        self.fake("claude", {"kind": "service", "delay": 0.5})
        journal = os.path.join(self.root, "journal.jsonl")
        code, result, err = self.launch(env=self.env(OWNER_SWITCH_PROBE_JOURNAL=journal))
        records = osw.read_journal(journal)
        self.assertEqual(result["journal"], journal)
        self.assertEqual({r["pid"] for r in records}, {r["pid"] for r in result["owned"]})
        self.assertIn(self.service_pids()[0], {r["pid"] for r in records})


    def failing_launch(self, confuse: bool):
        """Run the launcher in-process; its process table fails once after the leader is reaped.

        With CONFUSE, each record's executable differs after the failure, so
        no record authorises a signal and each one stays unresolved.
        """
        self.fake("claude", {"kind": "service", "delay": 0.5})
        work = os.path.join(self.root, "work")
        os.makedirs(work)
        failures = []

        class FailingTracker(probes.Tracker):
            def poll(self):
                if self.leader_proc is not None and self.leader_proc.returncode is not None and not failures:
                    failures.append(time.time())
                    raise probes.Fail("the process listing failed: fixture")
                procs = super().poll()
                if confuse and failures:
                    # poll follows an exec to a new executable, so the change is made again after each poll.
                    for rec in self.records.values():
                        rec["executable"] = "/nonexistent/fixture"
                return procs

        class FixtureLaunch(probes.Launch):
            def argv(self):
                return [os.path.join(self.env["PATH"].split(":")[0], "claude")]

        saved, saved_path = probes.Tracker, os.environ.get("PATH")
        probes.Tracker = FailingTracker
        os.environ["PATH"] = self.bin + ":/usr/bin:/bin"
        try:
            launch = FixtureLaunch("claude", "native", "fixture", self.home, work, settle=1)
            with self.assertRaises(probes.Fail) as raised:
                launch.run()
        finally:
            probes.Tracker = saved
            os.environ["PATH"] = saved_path
        self.assertTrue(failures, "the control never failed the process table")
        self.assertIn("fixture", str(raised.exception), "the original error was lost")
        service = self.service_pids()[0]
        self.assertIn(service, [r["pid"] for r in launch.tracker.owned()], "the control recorded no service")
        return launch, str(raised.exception), service

    def test_a_failure_after_the_leader_is_reaped_still_ends_the_recorded_service(self):
        """L4: a process-table failure in the settle interval keeps the original error and ends the owned service."""
        launch, message, service = self.failing_launch(confuse=False)
        self.assertFalse(alive(service), "the exception path left the recorded service alive")
        self.assertFalse(launch.keep_work, "a resolved cleanup kept the work folder")

    def test_a_failure_that_leaves_a_record_unresolved_reports_it_and_keeps_the_journal(self):
        """L4: an owned process that the cleanup cannot end appears in the failure, and the journal stays."""
        launch, message, service = self.failing_launch(confuse=True)
        self.assertTrue(alive(service), "the cleanup signalled a record whose identity it could not confirm")
        self.assertIn(f"{service} (/nonexistent/fixture, changed)", message)
        self.assertTrue(launch.keep_work, "the failure dropped the work folder that holds the journal")
        self.assertTrue(os.path.exists(os.path.join(launch.work, "journal.jsonl")))

class TrackerControls(unittest.TestCase):

    def test_a_reaped_leaders_pid_is_never_a_root(self):
        """After the leader is reaped, its PID can name another process: the tracker must not adopt it."""
        import types
        root = tempfile.mkdtemp()
        self.addCleanup(shutil.rmtree, root)
        tracker = probes.Tracker(os.path.join(root, "journal.jsonl"))
        other = subprocess.Popen(["/bin/sleep", "300"], start_new_session=True)
        self.addCleanup(lambda: (kill_quietly(other.pid), other.wait()))
        time.sleep(0.3)
        tracker.leader = other.pid
        tracker.leader_proc = types.SimpleNamespace(returncode=0)
        tracker.poll()
        self.assertEqual(tracker.owned(), [], "the tracker adopted a process through a reaped leader's PID")
        tracker.leader_proc = types.SimpleNamespace(returncode=None)
        tracker.poll()
        self.assertEqual([r["pid"] for r in tracker.owned()], [other.pid], "the control cannot see a leader")


class WorkDirectoryControls(unittest.TestCase):

    def test_workspace_and_skill_roots_refuse(self):
        root = os.path.realpath(tempfile.mkdtemp(dir="/private/tmp"))
        try:
            work = os.path.join(root, "a", "b")
            os.makedirs(work)
            home = os.path.join(root, "home")
            self.assertEqual(probes.work_problem(work, "/Users/nobody"), "")
            for mark in (".jj", ".grove", ".claude/skills", ".codex/skills", ".agents/skills"):
                os.makedirs(os.path.join(root, mark))
                self.assertIn(mark, probes.work_problem(work, "/Users/nobody"))
                shutil.rmtree(os.path.join(root, mark.split("/")[0]))
            self.assertIn("overlap", probes.work_problem(work, root))
            self.assertIn("overlap", probes.work_problem(root, work))
            self.assertEqual(probes.work_problem(work, home), "")
        finally:
            shutil.rmtree(root, ignore_errors=True)


class EndOwnedControls(unittest.TestCase):
    """owner_switch.end_owned, which both the probe and the switch use."""

    def start(self, code="import time; time.sleep(300)") -> tuple:
        p = subprocess.Popen([PY, "-I", "-c", code], start_new_session=True)
        self.addCleanup(lambda: (kill_quietly(p.pid), p.wait()))
        time.sleep(0.3)
        rec = {"pid": p.pid, "start": osw.process_start(p.pid), "executable": osw.executable_path(p.pid),
               "home": None}
        return p, rec

    def sent(self):
        calls = []

        def send(pid, sig):
            calls.append((pid, sig))
            os.kill(pid, sig)
        return calls, send

    def test_reused_pid_is_never_signalled(self):
        p, rec = self.start()
        calls, send = self.sent()
        outcome = osw.end_owned([dict(rec, start="1.000001")], term_grace=0.2, kill_grace=0.2, send=send)
        self.assertEqual(calls, [])
        self.assertEqual(len(outcome["ended"]), 1, "a reused PID counts as the recorded process gone")
        self.assertTrue(alive(p.pid))

    def test_changed_executable_is_never_signalled(self):
        p, rec = self.start()
        calls, send = self.sent()
        outcome = osw.end_owned([dict(rec, executable="/nonexistent/python")], term_grace=0.2, kill_grace=0.2,
                                send=send)
        self.assertEqual(calls, [])
        self.assertEqual([r["state"] for r in outcome["remaining"]], ["changed"])
        self.assertTrue(alive(p.pid))

    def test_unknown_executable_is_never_signalled(self):
        p, rec = self.start()
        calls, send = self.sent()
        outcome = osw.end_owned([dict(rec, executable=None)], term_grace=0.2, kill_grace=0.2, send=send)
        self.assertEqual(calls, [])
        self.assertEqual([r["state"] for r in outcome["remaining"]], ["changed"])

    def test_exact_identity_is_ended(self):
        p, rec = self.start()
        calls, send = self.sent()
        outcome = osw.end_owned([rec], term_grace=5, kill_grace=5, send=send)
        p.wait(timeout=10)
        self.assertEqual(calls, [(p.pid, signal.SIGTERM)])
        self.assertEqual((outcome["remaining"], len(outcome["ended"])), ([], 1))

    def test_ignored_sigterm_escalates_to_sigkill(self):
        p, rec = self.start("import signal, time; signal.signal(signal.SIGTERM, signal.SIG_IGN); time.sleep(300)")
        calls, send = self.sent()
        outcome = osw.end_owned([rec], term_grace=0.5, kill_grace=5, send=send)
        p.wait(timeout=10)
        self.assertEqual([s for _, s in calls], [signal.SIGTERM, signal.SIGKILL])
        self.assertEqual(outcome["remaining"], [])

    def test_unreadable_live_pid_is_never_signalled(self):
        # launchd belongs to root: its PID exists, and this account cannot read it.
        calls, send = self.sent()
        rec = {"pid": 1, "start": "1.000001", "executable": "/sbin/launchd", "home": None}
        self.assertEqual(osw.owned_state(rec), "changed")
        outcome = osw.end_owned([rec], term_grace=0.2, kill_grace=0.2, send=send)
        self.assertEqual(calls, [])
        self.assertEqual([r["state"] for r in outcome["remaining"]], ["changed"])

    def test_records_need_a_real_pid_created_after_the_probe_began(self):
        good = {"pid": 4242, "start": "2000.000001", "executable": "/bin/a", "home": None}
        for pid in (True, 0, 1, -1, "7"):
            self.assertFalse(osw.owned_record(dict(good, pid=pid)), pid)
        self.assertTrue(osw.owned_record(good))
        older = dict(good, pid=4343, start="1000.000001")
        owned = osw.probe_owned([{"launched": 1999.5, "result": {"owned": [older]}, "journal": [good]}])
        self.assertEqual(owned, [good])

    def test_journal_and_result_merge_by_identity(self):
        root = tempfile.mkdtemp()
        self.addCleanup(shutil.rmtree, root)
        journal = os.path.join(root, "j.jsonl")
        early = {"pid": 7, "start": "1.000001", "executable": "/bin/a", "home": "/h"}
        later = dict(early, executable="/bin/b")
        other = {"pid": 8, "start": "2.000002", "executable": "/bin/c", "home": None}
        with open(journal, "w") as f:
            f.write(json.dumps(early) + "\n" + json.dumps(later) + "\n" + '{"pid": 9, "sta')
        records = osw.read_journal(journal)
        self.assertEqual(records, [early, later])
        owned = osw.probe_owned([{"result": {"owned": [other]}, "journal": records}, {"result": None}])
        self.assertEqual(sorted(owned, key=lambda r: r["pid"]), [later, other])


    def test_unreadable_executable_keeps_its_cleanup_obligation(self):
        """L1: a record whose executable could not be read reaches the switch, which leaves recovery incomplete."""
        p, rec = self.start()
        rec = dict(rec, executable=None, home="/h")
        root = tempfile.mkdtemp()
        self.addCleanup(shutil.rmtree, root)
        journal = os.path.join(root, "j.jsonl")
        with open(journal, "w") as f:
            f.write(json.dumps(rec) + "\n")
        self.assertEqual(osw.read_journal(journal), [rec])
        owned = osw.probe_owned([{"launched": float(rec["start"]), "result": {"owned": [rec]}, "journal": [rec]}])
        self.assertEqual(owned, [rec])
        with self.assertRaises(osw.Unresolved):
            osw.end_probe_services(None, owned)
        self.assertTrue(alive(p.pid), "the switch signalled a process whose identity it could not confirm")

    def test_switch_records_a_child_created_during_shutdown(self):
        """L2: a journaled service starts a detached child on SIGTERM; the switch records and ends it too."""
        root = os.path.realpath(tempfile.mkdtemp(dir="/private/tmp"))
        self.addCleanup(shutil.rmtree, root)
        pids = os.path.join(root, "pids")
        p = subprocess.Popen([PY, "-I", "-c", SPAWNER, pids], start_new_session=True)
        self.addCleanup(lambda: (kill_quietly(p.pid), p.wait()))
        time.sleep(0.3)
        rec = {"pid": p.pid, "start": osw.process_start(p.pid), "executable": osw.executable_path(p.pid),
               "home": None}
        try:
            osw.end_probe_services(None, [rec])
        finally:
            with open(pids) as f:
                started = [int(line) for line in f.read().split()]
            for pid in started:
                self.addCleanup(kill_quietly, pid)
        grandchild = started[-1]
        self.assertNotEqual(grandchild, p.pid, "the spawner started no child")
        self.assertFalse(alive(grandchild), "the switch left the child that the service created while it ended")

    def test_switch_cleanup_without_a_process_table_is_unresolved(self):
        """L2: when the switch cannot follow descendants, it reports recovery incomplete rather than Stop."""
        p, rec = self.start()
        saved = osw.process_table

        def broken():
            raise osw.Stop("the process listing failed: fixture")
        osw.process_table = broken
        try:
            with self.assertRaises(osw.Unresolved):
                osw.end_probe_services(None, [rec])
        finally:
            osw.process_table = saved


# A service that, on SIGTERM, starts /bin/sleep in a new session, stays half
# a second and exits. It writes its own PID and then the child's to argv[1].
SPAWNER = r"""
import os, signal, subprocess, sys, time
def note(pid):
    with open(sys.argv[1], "a") as f:
        f.write(str(pid) + "\n")
def stop(*_):
    g = subprocess.Popen(["/bin/sleep", "300"], start_new_session=True)
    note(g.pid)
    time.sleep(0.5)
    os._exit(0)
signal.signal(signal.SIGTERM, stop)
note(os.getpid())
time.sleep(300)
"""

# A fake probe for run_probe: it starts a service, journals it, and sleeps
# until SIGTERM, which it answers by ending the service and exiting 1.
FAKE_PROBE = r'''
import json, os, signal, subprocess, sys, time
sys.path.insert(0, sys.argv[1])
import owner_switch as osw
service = subprocess.Popen(["/bin/sleep", "300"], start_new_session=True)
rec = {"pid": service.pid, "start": osw.process_start(service.pid),
       "executable": osw.executable_path(service.pid), "home": os.environ["HOME"]}
with open(os.environ["OWNER_SWITCH_PROBE_JOURNAL"], "a") as f:
    f.write(json.dumps(rec) + "\n")
with open(sys.argv[2], "w") as f:
    f.write(str(service.pid))
if sys.argv[3] == "exits":
    sys.exit(1)
def stop(*_):
    if sys.argv[3] == "cleans":
        service.terminate()
        service.wait()
    sys.exit(1)
signal.signal(signal.SIGTERM, stop)
time.sleep(300)
'''


class RunProbeControls(Fixture):
    """owner_switch.run_probe: an interrupt or the timeout lets the probe clean up, and the journal remains."""

    def command(self, mode: str, name: str = "fake") -> tuple:
        import types
        script = os.path.join(self.root, "fake_probe.py")
        with open(script, "w") as f:
            f.write(FAKE_PROBE)
        pidfile = os.path.join(self.root, f"{name}.pid")
        layout = types.SimpleNamespace(home=self.home, state=os.path.join(self.home, "state"))
        return layout, {"name": name, "argv": [PY, "-I", script, HERE, pidfile, mode]}, pidfile

    def run_probe(self, mode: str):
        layout, command, pidfile = self.command(mode)
        return layout, command, os.path.join(self.root, "journal.jsonl"), pidfile

    def interrupted(self, layout, commands) -> None:
        def interrupt(*_):
            raise KeyboardInterrupt
        previous = signal.signal(signal.SIGALRM, interrupt)
        signal.setitimer(signal.ITIMER_REAL, 2.0)
        try:
            with self.assertRaises(KeyboardInterrupt):
                osw.run_probes(layout, {"_path": "plan", "_digest": "0"}, commands, "7")
        finally:
            signal.setitimer(signal.ITIMER_REAL, 0)
            signal.signal(signal.SIGALRM, previous)

    def service(self, pidfile: str) -> int:
        for _ in range(100):
            if os.path.exists(pidfile) and open(pidfile).read():
                return int(open(pidfile).read())
            time.sleep(0.05)
        self.fail("the fake probe started no service")

    def test_timeout_sends_sigterm_and_keeps_the_journal(self):
        layout, command, journal, pidfile = self.run_probe("cleans")
        saved = osw.PROBE_TIMEOUT
        osw.PROBE_TIMEOUT = 2.0
        try:
            evidence = osw.run_probe(layout, {"_path": "plan", "_digest": "0"}, command, "7", journal)
        finally:
            osw.PROBE_TIMEOUT = saved
        service = self.service(pidfile)
        self.started.append(service)
        self.assertEqual(evidence["exit"], 124)
        self.assertEqual([r["pid"] for r in evidence["journal"]], [service])
        self.assertFalse(alive(service), "the probe did not clean up after SIGTERM")

    def test_interrupt_ends_the_journal_records_and_continues(self):
        layout, command, pidfile = self.command("leaves")
        self.interrupted(layout, [command])
        service = self.service(pidfile)
        self.started.append(service)
        self.assertFalse(alive(service), "the program did not end the journal's service after the interrupt")

    def test_interrupt_of_a_later_probe_ends_the_earlier_probes_records(self):
        """L3: the first probe dies and leaves a journaled service; an interrupt of the second ends both."""
        layout, first, first_pid = self.command("exits", "first")
        _, second, second_pid = self.command("leaves", "second")
        self.interrupted(layout, [first, second])
        services = [self.service(first_pid), self.service(second_pid)]
        self.started.extend(services)
        self.assertEqual([alive(pid) for pid in services], [False, False],
                         "the interrupted sequence left a probe's journaled service")


class ResolveControls(Fixture):

    def setUp(self):
        super().setUp()
        with open(PLAN) as f:
            self.plan = json.load(f)
        self.accepted = self.plan["accepted"]["sources"]
        prefix = os.path.join(self.root, "prefix")
        os.makedirs(os.path.join(prefix, "bin"))
        ravel = os.path.join(prefix, "bin", "ravel")
        with open(ravel, "w") as f:
            f.write(FAKE_RAVEL.format(py=PY))
        os.chmod(ravel, 0o755)
        with open(os.path.join(self.bin, "brew"), "w") as f:
            f.write(f"#!/bin/sh\n[ \"$1\" = --prefix ] && echo '{prefix}'\n")
        os.chmod(os.path.join(self.bin, "brew"), 0o755)
        self.prefix = prefix
        self.snapshots = os.path.join(self.root, "snapshots")

    def document(self, status="resolved", digests=None, drop=(), mandatory=True) -> dict:
        sources = []
        for sid, source in self.accepted.items():
            if sid in drop:
                continue
            root = os.path.join(self.snapshots, sid, "tree")
            os.makedirs(root, exist_ok=True)
            sources.append({"id": sid, "resourceDigest": (digests or {}).get(sid, source["resourceDigest"]),
                            "snapshotRoot": root, "originPath": f"/keg/{sid}", "type": "local"})
        files = []
        if mandatory:
            path = os.path.join(self.snapshots, "superpowers", "tree", "skills", "using-superpowers", "SKILL.md")
            os.makedirs(os.path.dirname(path), exist_ok=True)
            with open(path, "w") as f:
                f.write("---\nname: using-superpowers\n---\n")
            files.append({"path": path, "source": "superpowers"})
        if status != "resolved":
            return {"schemaVersion": 1, "status": status}
        return {"schemaVersion": 1, "status": status, "sources": sources, "mandatory": files}

    def resolve(self, expect: str, doc: dict) -> tuple:
        with open(os.path.join(self.prefix, "bin", "resolve.json"), "w") as f:
            json.dump(doc, f)
        return self.probe("resolve", "--expect", expect, "--plan", PLAN)

    def test_switched_passes_with_the_accepted_layout(self):
        code, result, err = self.resolve("switched", self.document())
        self.assertEqual((code, result["ok"]), (0, True), result["problems"])
        self.assertEqual(len(result["sources"]), 10)
        self.assertEqual(len(result["mandatory"]), 1)
        self.assertEqual(len(result["mandatory"][0]["sha256"]), 64)

    def test_switched_fails_for_a_wrong_layout(self):
        cases = {
            "digest": (self.document(digests={"design": "0" * 64}), "gives resourceDigest"),
            "missing": (self.document(drop=("quint",)), "accepted source quint is missing"),
            "mandatory": (self.document(mandatory=False), "mandatory file superpowers"),
            "disabled": (self.document(status="disabled"), "not 'resolved'"),
        }
        for name, (doc, text) in cases.items():
            with self.subTest(name):
                code, result, err = self.resolve("switched", doc)
                self.assertEqual((code, result["ok"]), (1, False))
                self.assertTrue(any(text in p for p in result["problems"]), result["problems"])

    def test_disabled_passes_and_fails(self):
        code, result, err = self.resolve("disabled", self.document(status="disabled"))
        self.assertEqual((code, result["ok"]), (0, True), result["problems"])
        code, result, err = self.resolve("disabled", self.document())
        self.assertEqual((code, result["ok"]), (1, False))
        self.assertIn("not 'disabled'", result["problems"][0])

    def test_plan_digest_must_match_under_the_switch(self):
        with open(os.path.join(self.prefix, "bin", "resolve.json"), "w") as f:
            json.dump(self.document(status="disabled"), f)
        env = self.env(OWNER_SWITCH_PLAN=PLAN, OWNER_SWITCH_PLAN_DIGEST="0" * 64)
        code, result, err = self.probe("resolve", "--expect", "disabled", env=env)
        self.assertEqual(code, 1)
        self.assertIn("not the plan that the program admitted", result["problems"][0])


if __name__ == "__main__":
    unittest.main()
