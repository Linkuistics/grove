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
            "transcripts": [{"main": True, "file": "main.jsonl", "events": events,
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


KIND_CASE = {"kind": "planning", "kindSkill": {"source": "grove-core", "skill": "grove-planning"}}
KIND_GOOD = [
    event(1, "kind-skill", "grove-planning", "grove-core"),
    event(2, "skill", "grove", "grove-core"),
    event(3, "supporting", "grove", "grove-core"),
    event(4, "action"),
]
DELEGATE_CASE = {"delegate": {"source": "engineering", "skill": "coding-style-bash"}}


def delegated(parent: list[dict], child: list[dict] | None) -> dict:
    launch_ = launch(parent)
    launch_["delegations"] = [{"description": "check", "transcript": "sub.jsonl" if child is not None else None}]
    if child is not None:
        launch_["transcripts"].append({"main": False, "file": "sub.jsonl", "events": child})
    return launch_


class KindAndDelegateVerdicts(unittest.TestCase):
    def test_kind_case_passes(self) -> None:
        verdict = staging_trial.judge(KIND_CASE, [launch(KIND_GOOD)], set())
        self.assertEqual(verdict["failures"], [])
        self.assertEqual(verdict["kindSkill"]["resourceSeq"], 3)

    def test_kind_case_reports_but_does_not_enforce_mandatory_order(self) -> None:
        verdict = staging_trial.judge(KIND_CASE, [launch(KIND_GOOD)], set())
        self.assertFalse(verdict["mandatoryFirst"]["passed"])
        self.assertFalse(verdict["mandatoryFirst"]["enforced"])

    def test_kind_skill_from_other_source_fails(self) -> None:
        events = [event(1, "kind-skill", "grove-planning", "personal"), *KIND_GOOD[1:]]
        verdict = staging_trial.judge(KIND_CASE, [launch(events)], set())
        self.assertTrue(any("no read of the kind skill" in f for f in verdict["failures"]))

    def test_plain_skill_read_is_not_a_kind_read(self) -> None:
        events = [event(1, "skill", "grove-planning", "grove-core"), *KIND_GOOD[1:]]
        verdict = staging_trial.judge(KIND_CASE, [launch(events)], set())
        self.assertTrue(any("no read of the kind skill" in f for f in verdict["failures"]))

    def test_kind_case_without_resource_fails(self) -> None:
        events = [e for e in KIND_GOOD if e["class"] != "supporting"]
        verdict = staging_trial.judge(KIND_CASE, [launch(events)], set())
        self.assertTrue(any("no read of a resource" in f for f in verdict["failures"]))

    def test_resource_of_other_source_fails(self) -> None:
        events = [*KIND_GOOD[:2], event(3, "supporting", "editorial-family", "writing"), KIND_GOOD[3]]
        verdict = staging_trial.judge(KIND_CASE, [launch(events)], set())
        self.assertTrue(any("no read of a resource" in f for f in verdict["failures"]))

    def test_resource_after_task_file_passes_and_is_recorded(self) -> None:
        task = event(3, "command", target="cat .grove/01-planning--word-tools-k1.md")
        events = [*KIND_GOOD[:2], task, event(4, "supporting", "grove", "grove-core"), event(5, "action")]
        verdict = staging_trial.judge(KIND_CASE, [launch(events)], set())
        self.assertEqual(verdict["failures"], [])
        self.assertEqual((verdict["kindSkill"]["taskSeq"], verdict["kindSkill"]["resourceSeq"]), (3, 4))

    def test_kind_skill_after_task_file_fails(self) -> None:
        events = [event(1, "command", target="cat .grove/_BRIEF.md"),
                  event(2, "kind-skill", "grove-planning", "grove-core"),
                  event(3, "supporting", "grove", "grove-core"), event(4, "action")]
        verdict = staging_trial.judge(KIND_CASE, [launch(events)], set())
        self.assertTrue(any("kind skill was read at or after" in f for f in verdict["failures"]))

    def test_commands_alone_give_the_task_event(self) -> None:
        command = dict(event(4, "command", target="grove-llm leaf-add . words --kind impl"), pathClasses=["work"])
        verdict = staging_trial.judge(KIND_CASE, [launch([*KIND_GOOD[:3], command])], set())
        self.assertEqual(verdict["failures"], [])
        self.assertEqual(verdict["kindSkill"]["taskSeq"], 4)
        # The control: without the touch of the work tree, no task event.
        command.pop("pathClasses")
        verdict = staging_trial.judge(KIND_CASE, [launch([*KIND_GOOD[:3], command])], set())
        self.assertTrue(any("never touched the work tree" in f for f in verdict["failures"]))

    def test_task_file_read_before_kind_skill_fails(self) -> None:
        # A read of a renamed task file names no existing path, so only its
        # mention of `.grove` shows it.
        task = event(1, "command", target="cat .grove/01-planning--word-tools-k1.md")
        events = [task, *(dict(e, seq=e["seq"] + 1) for e in KIND_GOOD)]
        verdict = staging_trial.judge(KIND_CASE, [launch(events)], set())
        self.assertEqual(verdict["kindSkill"]["taskSeq"], 1)
        self.assertTrue(verdict["failures"])

    def test_resource_read_with_task_file_in_one_command_passes(self) -> None:
        both = dict(event(3, "supporting", "grove", "grove-core",
                          target="cat /s/tree/skills/grove/references/bootstrap.md .grove/_BRIEF.md"),
                    pathClasses=["supporting", "work"])
        verdict = staging_trial.judge(KIND_CASE, [launch([*KIND_GOOD[:2], both, KIND_GOOD[3]])], set())
        self.assertEqual(verdict["failures"], [])
        self.assertEqual(verdict["kindSkill"]["taskSeq"], 3)

    def test_kind_skill_read_with_task_file_in_one_command_fails(self) -> None:
        both = dict(event(1, "kind-skill", "grove-planning", "grove-core",
                          target="cat /s/tree/skills/grove-planning/SKILL.md .grove/_BRIEF.md"),
                    pathClasses=["kind-skill", "work"])
        verdict = staging_trial.judge(KIND_CASE, [launch([both, *KIND_GOOD[1:]])], set())
        self.assertTrue(any("kind skill was read at or after" in f for f in verdict["failures"]))

    def test_snapshot_path_and_text_are_no_mention(self) -> None:
        events = [event(1, "text", target="I will read .grove next."),
                  event(2, "command", target="ls /s/tree/skills/grove/references"),
                  *(dict(e, seq=e["seq"] + 2) for e in KIND_GOOD)]
        verdict = staging_trial.judge(KIND_CASE, [launch(events)], set())
        self.assertEqual(verdict["failures"], [])
        self.assertEqual(verdict["kindSkill"]["taskSeq"], 6)

    def test_delegate_read_passes(self) -> None:
        child = [event(1, "skill", "coding-style-bash", "engineering"), event(2, "action")]
        verdict = staging_trial.judge(DELEGATE_CASE, [delegated(GOOD, child)], set())
        self.assertEqual(verdict["failures"], [])
        self.assertEqual(verdict["delegate"]["delegates"][0]["readSeq"], 1)

    def test_one_agent_call_listed_twice_is_one_delegate(self) -> None:
        child = [event(1, "skill", "coding-style-bash", "engineering")]
        launch_ = delegated(GOOD, child)
        launch_["delegations"] = [dict(launch_["delegations"][0], toolUseId="t1")] * 2
        self.assertEqual(len(staging_trial.judge(DELEGATE_CASE, [launch_], set())["delegate"]["delegates"]), 1)
        # The control: two calls stay two delegates.
        launch_["delegations"] = [dict(launch_["delegations"][0], toolUseId=i) for i in ("t1", "t2")]
        self.assertEqual(len(staging_trial.judge(DELEGATE_CASE, [launch_], set())["delegate"]["delegates"]), 2)

    def test_parent_read_alone_fails(self) -> None:
        parent = [*GOOD, event(8, "skill", "coding-style-bash", "engineering")]
        verdict = staging_trial.judge(DELEGATE_CASE, [delegated(parent, [event(1, "action")])], set())
        self.assertTrue(any("no delegate read" in f for f in verdict["failures"]))

    def test_no_delegate_fails(self) -> None:
        verdict = staging_trial.judge(DELEGATE_CASE, [launch(GOOD)], set())
        self.assertTrue(any("started no delegate" in f for f in verdict["failures"]))

    def test_delegate_without_transcript_fails(self) -> None:
        verdict = staging_trial.judge(DELEGATE_CASE, [delegated(GOOD, None)], set())
        self.assertTrue(any("no delegate transcript" in f for f in verdict["failures"]))

    def test_delegate_read_of_other_source_fails(self) -> None:
        child = [event(1, "skill", "coding-style-bash", "personal")]
        verdict = staging_trial.judge(DELEGATE_CASE, [delegated(GOOD, child)], set())
        self.assertTrue(any("no delegate read" in f for f in verdict["failures"]))

    def test_delegate_content_read_passes(self) -> None:
        child = [dict(event(1, "command"), contentReads=[{"source": "engineering", "skill": "coding-style-bash"}])]
        verdict = staging_trial.judge(DELEGATE_CASE, [delegated(GOOD, child)], set())
        self.assertEqual(verdict["failures"], [])

    def test_kind_and_delegate_cases_load(self) -> None:
        self.assertEqual(staging_trial.load_case("kind-skill")["kindSkill"]["skill"], "grove-planning")
        self.assertEqual(staging_trial.load_case("delegate")["delegate"]["skill"], "coding-style-bash")

    def test_kind_skill_of_another_kind_refuses(self) -> None:
        import json
        import tempfile
        with tempfile.TemporaryDirectory() as scratch:
            original = staging_trial.CASES
            staging_trial.CASES = Path(scratch)
            try:
                case = {"name": "bad", "kind": "impl", "kindSkill": {"source": "grove-core", "skill": "grove-planning"}}
                (Path(scratch) / "bad.json").write_text(json.dumps(case))
                with self.assertRaises(staging_trial.TrialFailure):
                    staging_trial.load_case("bad")
                case["kind"] = "planning"
                (Path(scratch) / "bad.json").write_text(json.dumps(case))
                staging_trial.load_case("bad")
            finally:
                staging_trial.CASES = original


COMPACTION_CASE = {"mandatoryFirst": True,
                   "compaction": {"answer": "a", "after": {"source": "engineering", "skill": "coding-style-bash"}}}
COMPACTION_GOOD = [
    event(1, "prompt"),
    event(2, "mandatory", "using-superpowers", "superpowers"),
    event(3, "action"),
    event(4, "prompt", target="/compact"),
    event(5, "compaction"),
    event(6, "prompt", target="Next task."),
    event(7, "skill", "coding-style-bash", "engineering"),
    event(8, "action"),
]
STALE_SPEC = {"source": "grove-core", "skill": "grove", "from": ".claude/skills/grove", "treeSha256": "d"}
STALE_CASE = {"staleCopy": STALE_SPEC}


class CompactionVerdicts(unittest.TestCase):
    def test_read_after_compaction_passes(self) -> None:
        verdict = staging_trial.judge(COMPACTION_CASE, [launch(COMPACTION_GOOD)], set())
        self.assertEqual(verdict["failures"], [])
        self.assertEqual((verdict["compaction"]["boundarySeq"], verdict["compaction"]["readSeq"]), (5, 7))

    def test_no_boundary_fails(self) -> None:
        events = [e for e in COMPACTION_GOOD if e["class"] != "compaction"]
        verdict = staging_trial.judge(COMPACTION_CASE, [launch(events)], set())
        self.assertTrue(any("no compaction boundary" in f for f in verdict["failures"]))

    def test_read_only_before_compaction_fails(self) -> None:
        events = [*COMPACTION_GOOD[:2], event(3, "skill", "coding-style-bash", "engineering"),
                  *COMPACTION_GOOD[2:6], event(9, "action")]
        verdict = staging_trial.judge(COMPACTION_CASE, [launch(events)], set())
        self.assertTrue(any("after the compaction" in f for f in verdict["failures"]))
        self.assertEqual(verdict["compaction"]["readBeforeSeq"], 3)

    def test_read_after_the_first_edit_after_compaction_fails(self) -> None:
        events = [*COMPACTION_GOOD[:6], event(7, "action"), event(8, "skill", "coding-style-bash", "engineering")]
        verdict = staging_trial.judge(COMPACTION_CASE, [launch(events)], set())
        self.assertTrue(any("came after the first edit" in f for f in verdict["failures"]))

    def test_read_of_other_source_fails(self) -> None:
        events = [*COMPACTION_GOOD[:6], event(7, "skill", "coding-style-bash", "personal"), event(8, "action")]
        verdict = staging_trial.judge(COMPACTION_CASE, [launch(events)], set())
        self.assertTrue(any("no read of coding-style-bash" in f for f in verdict["failures"]))

    def test_no_edit_after_compaction_fails(self) -> None:
        verdict = staging_trial.judge(COMPACTION_CASE, [launch(COMPACTION_GOOD[:7])], set())
        self.assertTrue(any("no edit after the compaction" in f for f in verdict["failures"]))

    def test_turns_plan_has_a_bare_compaction_and_no_gate(self) -> None:
        import json
        import tempfile
        with tempfile.TemporaryDirectory() as scratch:
            claude = staging_trial.write_turns(Path(scratch), staging_trial.harness_trial.adapter_named("claude"),
                                               COMPACTION_CASE["compaction"])
            self.assertEqual(claude["turns"], [{"text": "/compact"}, {"text": "a"}])
            self.assertEqual(json.loads((Path(scratch) / "turns.json").read_text()), claude)
            codex = staging_trial.write_turns(Path(scratch), staging_trial.harness_trial.adapter_named("codex"),
                                              COMPACTION_CASE["compaction"])
            self.assertEqual(codex["turns"], [{"compact": True}, {"text": "a"}])

    def test_compaction_and_stale_cases_load(self) -> None:
        self.assertEqual(staging_trial.load_case("compaction")["compaction"]["after"]["skill"], "coding-style-bash")
        self.assertEqual(staging_trial.load_case("stale-copy")["staleCopy"]["skill"], "grove")


class StaleCopyVerdicts(unittest.TestCase):
    def test_planted_name_is_the_one_listing_exception(self) -> None:
        events = [event(1, "skill", "grove", "grove-core")]
        verdict = staging_trial.judge(STALE_CASE, [launch(events, ["grove", "guardrail:guardrail"])], {"grove"})
        self.assertEqual(verdict["failures"], [])
        self.assertEqual(verdict["staleCopy"]["copy"], "ravel")
        # The control: another superseded name still fails.
        verdict = staging_trial.judge(STALE_CASE, [launch(events, ["grove", "grove-impl"])], {"grove", "grove-impl"})
        self.assertEqual(verdict["listedSuperseded"], ["grove-impl"])
        # Without the stale case, the planted name fails too.
        self.assertTrue(staging_trial.judge(CASE, [launch(GOOD, ["grove"])], {"grove"})["failures"])

    def test_unlisted_plant_fails(self) -> None:
        verdict = staging_trial.judge(STALE_CASE, [launch([event(1, "skill", "grove", "grove-core")])], {"grove"})
        self.assertTrue(any("does not hold the planted grove" in f for f in verdict["failures"]))

    def test_stale_reads_by_path_native_tool_and_content(self) -> None:
        for stale in (event(1, "work", target="/private/tmp/owner-staging-x/.claude/skills/grove/SKILL.md"),
                      event(1, "command", target="sed -n 1,80p .codex/skills/grove/references/planning.md"),
                      event(1, "native-skill-tool", target="grove"),
                      dict(event(1, "command", target="python3 - <<'PY'"), copyReads=["stale"])):
            verdict = staging_trial.judge(STALE_CASE, [launch([stale], ["grove"])], {"grove"})
            self.assertEqual(verdict["staleCopy"]["copy"], "stale", stale)
            self.assertEqual(verdict["failures"], [])

    def test_near_names_and_failed_loads_are_no_stale_read(self) -> None:
        for other in (event(1, "command", target="cat .claude/skills/grove-planning/SKILL.md"),
                      event(1, "native-skill-tool", target="grove-planning"),
                      dict(event(1, "native-skill-tool", target="grove"), error=True),
                      event(1, "command", target="cat /Users/antony/.claude/skills/grove/SKILL.md")):
            verdict = staging_trial.judge(STALE_CASE, [launch([other], ["grove"])], {"grove"})
            self.assertIsNone(verdict["staleCopy"]["copy"], other)
            self.assertTrue(any("read neither copy" in f for f in verdict["failures"]))

    def test_both_copies_record_the_first(self) -> None:
        events = [event(1, "native-skill-tool", target="grove"), event(2, "supporting", "grove", "grove-core")]
        verdict = staging_trial.judge(STALE_CASE, [launch(events, ["grove"])], {"grove"})
        self.assertEqual((verdict["staleCopy"]["copy"], verdict["staleCopy"]["first"]), ("both", "stale"))
        reordered = [dict(e, seq=n) for n, e in enumerate(reversed(events), 1)]
        verdict = staging_trial.judge(STALE_CASE, [launch(reordered, ["grove"])], {"grove"})
        self.assertEqual(verdict["staleCopy"]["first"], "ravel")

    def test_ravel_read_of_other_source_is_no_ravel_read(self) -> None:
        verdict = staging_trial.judge(STALE_CASE, [launch([event(1, "skill", "grove", "personal")], ["grove"])],
                                      {"grove"})
        self.assertIsNone(verdict["staleCopy"]["copy"])

    def test_copy_lines_tell_the_copies_apart(self) -> None:
        import tempfile
        with tempfile.TemporaryDirectory() as scratch:
            root = Path(scratch)
            shared = "# grove — hierarchical, self-extending workstreams\n"
            (root / "snap" / "skills" / "grove").mkdir(parents=True)
            (root / "snap" / "skills" / "grove" / "SKILL.md").write_text(
                f"---\nname: grove\n---\n\n{shared}\nYour prompt named one kind skill; open it before you act.\n")
            (root / "plant" ).mkdir()
            (root / "plant" / "SKILL.md").write_text(
                f"---\nname: grove\n---\n\n{shared}\n| `planning` | `references/planning.md` |\n")
            launch_ = launch([dict(event(1, "command"), resultFull="x | `planning` | `references/planning.md` | y"),
                              dict(event(2, "command"), resultFull="Your prompt named one kind skill; open it before you act."),
                              dict(event(3, "command"), resultFull=shared)])
            launch_["sources"] = [{"id": "grove-core", "snapshotRoot": str(root / "snap")}]
            copies = staging_trial.copy_lines(launch_, STALE_SPEC, str(root / "plant"))
            staging_trial.mark_content_reads(launch_, [], copies)
            events = launch_["transcripts"][0]["events"]
            self.assertEqual([e.get("copyReads") for e in events], [["stale"], ["ravel"], None])

    def test_a_line_inside_a_longer_line_is_no_mark(self) -> None:
        import tempfile
        with tempfile.TemporaryDirectory() as scratch:
            stale, current = Path(scratch) / "stale.md", Path(scratch) / "current.md"
            stale.write_text("so the reader is not left decoding\ncompressed technical shorthand.\n")
            current.write_text("the reader must not be left decoding compressed technical shorthand.\n")
            self.assertEqual(staging_trial.distinct_lines(stale, current), ["so the reader is not left decoding"])
            # The control: a line that the other file does not hold stays.
            current.write_text("nothing shared here at all, not one line\n")
            self.assertEqual(len(staging_trial.distinct_lines(stale, current)), 2)

    def test_plant_refuses_a_changed_copy(self) -> None:
        import tempfile
        with tempfile.TemporaryDirectory() as scratch:
            original = staging_trial.staging_home.real_home
            staging_trial.staging_home.real_home = lambda: Path(scratch) / "real"
            try:
                origin = Path(scratch) / "real" / ".claude" / "skills" / "grove"
                origin.mkdir(parents=True)
                (origin / "SKILL.md").write_text("old")
                pin = staging_trial.staging_home.tree_digest(origin)["sha256"]
                work = Path(scratch) / "work"
                work.mkdir()
                planted = staging_trial.plant_stale(work, dict(STALE_SPEC, treeSha256=pin))
                self.assertEqual(sorted(planted["plants"]), ["claude", "codex"])
                self.assertEqual((work / ".codex/skills/grove/SKILL.md").read_text(), "old")
                # The control: one changed byte refuses.
                (origin / "SKILL.md").write_text("olD")
                with self.assertRaises(staging_trial.TrialFailure):
                    staging_trial.plant_stale(Path(scratch) / "work2", dict(STALE_SPEC, treeSha256=pin))
            finally:
                staging_trial.staging_home.real_home = original

    def test_home_copies(self) -> None:
        import tempfile
        with tempfile.TemporaryDirectory() as scratch:
            home = Path(scratch)
            self.assertEqual(staging_trial.home_copies(home, "grove"), [])
            (home / ".agents" / "skills").mkdir(parents=True)
            (home / ".agents" / "skills" / "grove").symlink_to("/nowhere")
            self.assertEqual(staging_trial.home_copies(home, "grove"), [str(home / ".agents/skills/grove")])


class RealHomeCheck(unittest.TestCase):
    def tree(self, root: Path) -> Path:
        cache = root / "cache"
        version = cache / "market" / "plugin" / "1.0.0"
        (version / "skills").mkdir(parents=True)
        (version / "skills" / "a.md").write_text("a")
        return cache

    def test_top_level_marker_is_left_out_of_the_digest(self) -> None:
        import os
        import tempfile
        with tempfile.TemporaryDirectory() as scratch:
            cache = self.tree(Path(scratch))
            before = staging_trial.plugin_cache_digest(cache)
            marker = cache / "market" / "plugin" / "1.0.0" / ".in_use"
            marker.mkdir()
            (marker / str(os.getpid())).write_text('{"pid":1}')
            after = staging_trial.plugin_cache_digest(cache)
            self.assertEqual(staging_trial.isolation_problems({staging_trial.PLUGIN_CACHE: before},
                                                              {staging_trial.PLUGIN_CACHE: after}, Path("/none")),
                             [])
            # The control: a nested .in_use stays in the digest.
            nested = cache / "market" / "plugin" / "1.0.0" / "skills" / ".in_use"
            nested.mkdir()
            (nested / "1").write_text("x")
            changed = staging_trial.plugin_cache_digest(cache)
            self.assertEqual(staging_trial.isolation_problems({staging_trial.PLUGIN_CACHE: before},
                                                              {staging_trial.PLUGIN_CACHE: changed}, Path("/none")),
                             [f"the real {staging_trial.PLUGIN_CACHE} changed"])

    def test_marker_of_an_ended_process_fails(self) -> None:
        import os
        import subprocess
        ended = subprocess.Popen(["/usr/bin/true"])
        ended.wait()
        path = "market/plugin/1.0.0/.in_use/"
        self.assertEqual(staging_trial.marker_problems({}, {path + str(os.getpid()): "{}"}), [])
        self.assertEqual(len(staging_trial.marker_problems({}, {path + str(ended.pid): "{}"})), 1)
        # A marker that vanished is no write by the trial.
        self.assertEqual(staging_trial.marker_problems({path + str(ended.pid): "{}"}, {}), [])


if __name__ == "__main__":
    unittest.main()
