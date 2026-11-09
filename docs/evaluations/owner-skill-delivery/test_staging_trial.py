#!/usr/bin/env python3
"""Tests of the verdicts of staging_trial.py, on synthetic events. Each
verdict passes on a conforming session and fails on a mutated one."""

from __future__ import annotations

import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import staging_trial


def event(seq: int, cls: str, skill: str | None = None, source: str | None = None, target: str = "x") -> dict:
    return {"seq": seq, "class": cls, "skill": skill, "source": source, "target": target, "sidechain": False}


MANDATORY = [{"source": "superpowers", "path": "/c/tree/skills/using-superpowers/SKILL.md"}]
CASE = {"mandatoryFirst": True, "requires": [{"source": "design", "skill": "cli-tool-design"}]}


def launch(events: list[dict], skills: list[str] | None = None) -> dict:
    return {"mandatory": MANDATORY, "control": [],
            "transcripts": [{"main": True, "events": events,
                             "startup": {"init": {"skills": skills or []}}}]}


GOOD = [
    event(1, "text"),
    event(2, "mandatory", "using-superpowers", "superpowers"),
    event(3, "supporting", "using-superpowers", "superpowers"),
    event(4, "listing"),
    event(5, "skill", "cli-tool-design", "design"),
    event(6, "action"),
    event(7, "command"),
]


class Verdicts(unittest.TestCase):
    def test_conforming_session_passes(self) -> None:
        verdict = staging_trial.judge(CASE, [launch(GOOD, ["guardrail"])], {"cli-tool-design", "grove"})
        self.assertEqual(verdict["failures"], [])

    def test_search_before_mandatory_fails(self) -> None:
        events = [event(1, "listing"), *GOOD[1:]]
        verdict = staging_trial.judge(CASE, [launch(events)], set())
        self.assertIn("mandatory first", verdict["failures"][0])

    def test_supporting_of_another_skill_before_mandatory_fails(self) -> None:
        events = [event(1, "supporting", "cli-tool-design", "design"), *GOOD[1:]]
        verdict = staging_trial.judge(CASE, [launch(events)], set())
        self.assertTrue(any("mandatory first" in f for f in verdict["failures"]))

    def test_missing_mandatory_fails(self) -> None:
        events = [e for e in GOOD if e["class"] != "mandatory"]
        verdict = staging_trial.judge(CASE, [launch(events)], set())
        self.assertTrue(any("never read the mandatory" in f for f in verdict["failures"]))

    def test_craft_read_after_edit_fails(self) -> None:
        events = [*GOOD[:4], event(5, "action"), event(6, "skill", "cli-tool-design", "design")]
        verdict = staging_trial.judge(CASE, [launch(events)], set())
        self.assertTrue(any("after the first edit" in f for f in verdict["failures"]))

    def test_craft_from_other_source_fails(self) -> None:
        events = [*GOOD[:4], event(5, "skill", "cli-tool-design", "personal"), event(6, "action")]
        verdict = staging_trial.judge(CASE, [launch(events)], set())
        self.assertTrue(any("no read of cli-tool-design" in f for f in verdict["failures"]))

    def test_no_edit_fails(self) -> None:
        events = [e for e in GOOD if e["class"] != "action"]
        verdict = staging_trial.judge(CASE, [launch(events)], set())
        self.assertTrue(any("made no edit" in f for f in verdict["failures"]))

    def test_craft_read_among_several_reads_of_one_command_passes(self) -> None:
        command = dict(event(5, "listing"), more=[{"class": "skill", "skill": "cli-tool-design",
                                                   "source": "design", "path": "/s"}])
        events = [*GOOD[:4], command, event(6, "action")]
        verdict = staging_trial.judge(CASE, [launch(events)], set())
        self.assertEqual(verdict["failures"], [])

    def test_content_read_needs_name_and_heading(self) -> None:
        import tempfile
        with tempfile.TemporaryDirectory() as scratch:
            skill = Path(scratch) / "craft" / "cli-tool-design"
            skill.mkdir(parents=True)
            (skill / "SKILL.md").write_text("---\nname: cli-tool-design\ndescription: d\n---\n\n# CLI Design\n\nBody.\n")
            listing = dict(event(5, "command"), resultFull="name: cli-tool-design\ndescription: d")
            body = dict(event(6, "command"), resultFull="FILE x\\n---\\nname: cli-tool-design\\n# CLI Design\\n")
            launch_ = launch([*GOOD[:4], listing, body, event(7, "action")])
            launch_["sources"] = [{"id": "design", "snapshotRoot": scratch}]
            staging_trial.mark_content_reads(launch_, CASE["requires"])
            events = launch_["transcripts"][0]["events"]
            self.assertNotIn("contentReads", events[4])
            self.assertEqual(events[5]["contentReads"], [{"source": "design", "skill": "cli-tool-design"}])
            self.assertNotIn("resultFull", events[5])
            self.assertEqual(staging_trial.judge(CASE, [launch_], set())["failures"], [])
            # The control: without the body read, the verdict fails.
            launch_["transcripts"][0]["events"] = [*GOOD[:4], events[4], event(7, "action")]
            self.assertTrue(staging_trial.judge(CASE, [launch_], set())["failures"])

    def test_superseded_listing_fails_with_namespace(self) -> None:
        verdict = staging_trial.judge(CASE, [launch(GOOD, ["grove:grove-impl"])], {"grove-impl"})
        self.assertEqual(verdict["listedSuperseded"], ["grove-impl"])
        self.assertTrue(verdict["failures"])

    def test_two_launches_fail(self) -> None:
        verdict = staging_trial.judge(CASE, [launch(GOOD), launch(GOOD)], set())
        self.assertTrue(verdict["failures"])

    def test_view_comparison_sees_a_change(self) -> None:
        view = {"descriptorPath": "/d", "directories": ["/a", "/b"], "mandatory": [], "sources": []}
        self.assertEqual(staging_trial.view_differences(view, dict(view)), [])
        self.assertEqual(staging_trial.view_differences(view, dict(view, directories=["/a"])), ["directories"])

    def test_refusal_code(self) -> None:
        line = '{"schemaVersion":1,"status":"refused","code":"skill_source_missing","message":"m"}'
        self.assertEqual(staging_trial.refusal_code("ravel: noise\n" + line), "skill_source_missing")
        self.assertIsNone(staging_trial.refusal_code("plain text"))

    def test_unknown_case_refuses(self) -> None:
        with self.assertRaises(staging_trial.TrialFailure):
            staging_trial.load_case("discovery-not-a-case")

    def test_discovery_case_loads(self) -> None:
        case = staging_trial.load_case("discovery")
        self.assertEqual(case["requires"], [{"source": "design", "skill": "cli-tool-design"}])


if __name__ == "__main__":
    unittest.main()
