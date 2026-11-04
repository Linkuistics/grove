# pack-membership-k171

## Goal

Settle each open membership question of the remaining moves before the move
that needs the answer. Keep the agreed domain boundaries of the root brief.
Where an answer would change the root's destination table, ask the human.

## Context

`docs/specs/external-pack-contract.md` owns the pack layout, the family rule,
the reference rules and conformance. `docs/specs/owner-skill-delivery.md`
owns the ten Ravel sources and the native residue. Its section "Native
residue" gives the repository moves three packaging changes. The running log
of repository-moves-k11 records the inventory behind each question.

## Done when

Each question below has a decision in Grove's owning document, with the
rejected options and the reason. The external-pack contract gains a section
"Pack membership", or a sibling specification holds it and the contract
links to it. Each decision names the move that applies it.

1. **The ADR and spec formats.** The root table gives "ADR/spec formats" to
   DesignSkills. The spine's `SKILL.md`, `references/retire.md`,
   `references/grove.md`, `BRIEF-FORMAT.md`, the cross-pack family file
   `references/integrate-review.md` and the requirements kind all name
   `ADR-FORMAT.md` or `SPEC-FORMAT.md`. Every kind reaches them through the
   spine. Decide among these options, or a better one:
   - Keep both files in the spine, as the contract keeps cross-pack family
     files there. This departs from the wording of the root table.
   - Move both to DesignSkills as a skill that core loads by name. Core then
     depends on DesignSkills, and a core-only installation loses its ADR rules.
   - Split them. The spine keeps what every kind needs, such as the
     when-to-write test and the placement. DesignSkills keeps the authoring
     detail that only design and its review read.
   Ask the human before you adopt an option that changes the root table.
2. **The always-on hooks.** The root table gives "always-on hooks" to
   PersonalSkills. The only always-on hooks in the owner packs are those of
   `technical-communication`, which the root gives to WritingSkills as craft.
   The owner design ships them as a skill-free Claude Code plugin from
   WritingSkills. Decide which repository owns the hooks, the Claude Code
   hooks plugin and the Codex hooks plugin of `.agents/plugins/marketplace.json`.
   Correct the root wording through the human if the answer differs from it.
3. **The Grove assets of moving craft skills.** Grove holds these assets of
   craft skills that leave it. Each one can move to the skill's pack with its
   checks. It can stay in Grove as a consumer with a check against a pinned
   pack. It can also stay in Grove as history. Decide for each class:
   - The `technical-communication` specification, guide, design views,
     publication review, research notes and the off-file decision record.
   - The `writing-code-walkthroughs` evaluations under `docs/evaluations/`.
   - The formal-methods trial records of `model-led-development`.
   - `scripts/pi-extension-load.test.sh`, and the checker self-test and the
     `tc-hook` lint in `scripts/check.sh`.
   - The Rust tests that embed `doubt-driven-development`. They check how it
     composes with the review allowance.
   - `plugins/linkuistics/PROVENANCE.md`, which serves skills of four packs.
4. **The Grove tests that name moving kinds.** Some Grove tests and scripts
   use `grove-impl` or `grove-design` as fixture names. Others read a kind
   file from the checkout. Decide the rule for each class. A fixture name can
   stay. A read of a moved file needs a core kind or a fixture.
5. **The trial cases that name moving skills.** Many cases under
   `scripts/harness-trials/cases/` use `plugins/linkuistics` or a moving kind
   as a source. They are the evidence of the common-loading acceptance.
   A move can retarget them to external sources. They can also stay as a
   record of the build that they measured. Decide which. A case that cannot
   run must say so. No test may skip it without a report.
6. **The acceptance of each move.** The research move proved the pattern with
   pack checks, Grove's aggregate, two directory trials, a native trial and
   both refusals. The staging trials of cutover order 5 later run every
   source on both harnesses. Decide which proofs each later move owes. A
   move that adds a new boundary owes a trial of that boundary. These are
   the new boundaries:
   - An external member of a cross-pack family.
   - The first `craft/` root.
   - A second plugin in one repository.
   - A family skill outside research.
   - A source that is not a Linkuistics pack.

## Notes

This leaf decides placement only. It moves no file. The craft skills keep
their own checks, as the contract's "Out of scope" states.

The design may earn a `review-design` leaf by the rule in
`references/decompose.md`. Place it before requirements-pack-k173.

## Decisions (running log)

- Question 1, settled by the human: `ADR-FORMAT.md` and `SPEC-FORMAT.md` stay
  in the spine. Core's manifest has thirteen rows for the two files, and the
  spine's `SKILL.md` triggers them for every kind. They are also the fallback
  text for `decision-records` and `codebase-design`. The root table changes:
  core holds the tree and artifact formats, and DesignSkills holds the craft
  skills that deepen the formats.
- Question 2, settled by the human: WritingSkills owns the always-on hooks of
  `technical-communication`, its Claude Code hooks plugin and its Codex hooks
  plugin. Each hook runs that skill's scripts and injects its own text.
  PersonalSkills keeps `guardrail` and its native-only plugin, whose hook is
  user-invoked. The root table changes to say so.
- The root brief was not in controlled English. Its conversion landed as its
  own change before the table edit, as the writing procedure requires.
- Questions 3 to 6 settled in `docs/specs/pack-membership.md`, in the sections
  of the same names. The external pack contract links to it from its section
  "Pack membership".
- Question 3 found one more obligation. `check:packs` runs conformance on each
  pinned `git-subdir` entry, and TestAnyware and PersonalSkills ship no kind
  skills. `testanyware-skill-move-k172` makes the check accept such a source.
- The specification governs eight later moves, so it earns a design review.
  `pack-membership-k189` reviews it before `testanyware-skill-move-k172`. The
  review is scheduled, so this leaf spent no in-session reviewer.
