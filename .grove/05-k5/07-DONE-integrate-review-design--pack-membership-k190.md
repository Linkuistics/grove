# pack-membership-k190

**Integrates:** pack-membership-k189

## Goal

Triage the design review `pack-membership-k189` and apply the findings that
hold before later repository moves consume the membership specification.

## Context

Read the findings from `pack-membership-k189`'s own commit. The producer is
`pack-membership-k171`. The reviewed artifact is
`docs/specs/pack-membership.md`, with its views under
`docs/design/external-pack-contract/`.

## Done when

Each finding has a reasoned disposition. The accepted changes are in the
owning design artifacts and affected task briefs, with the verification that
the integration procedure requires.

## Notes

The human approved the two changes to the root destination table. Preserve
those decisions while assessing the review. This leaf changes the design.
The later move leaves still own implementation and migration acceptance.

## Decisions (running log)

- **F1 is a real issue. Fix the artifact.** The refusal binds every path to
  the prepared Grove commit. The source contradicts that. `research-discovery`,
  `research-delegate`, `research-root-missing` and `research-source-missing`
  take `plugins/research` and `grove-research-a` from a ResearchSkills
  checkout. `research-native-namespace` takes the kind through the pinned
  `research@linkuistics` plugin. `problem-stale-native` and
  `problem-stale-native-cli` copy their native skill from an older commit
  (`install_native` and `copy_from_commit` in `scripts/harness_trial.py`).
  `missing-kind` launches `impl` and measures the absence of `grove-impl`.
  `continuation` edits four files, which the path list omits. The refusal now
  binds each path to its supplier and revision, runs before a `moveAway`
  fault, and has admission controls. The timing before `design-pack-k175`
  stays, because `continuation` names only engineering and personal files.
- **F2 is a real issue. Fix the artifact.** `plugins/linkuistics` supplies its
  hooks through the default `hooks/hooks.json` of its root. The specification
  gives the ordinary WritingSkills plugin no hooks file, and the hook scripts
  move into the skill-free plugin's root under `craft/`. The specified static
  proof passes for that zero-hook layout. Decision: the ordinary writing
  plugin declares a dependency on the hooks plugin and declares no hook. The
  research plugin's dependency on
  `grove` proves the mechanism. A plugin that declares its own hooks was
  rejected, because both plugins together then register each hook twice. The
  decision is easy to reverse, so it earns no ADR.
- **F3 is a real issue. Fix the artifact.** `plugins/install.sh` refuses
  `--pack` without `skills/`, and `is_ours` recognises a link only by the
  segment `plugins/<plugin>/skills/`. A source with only `craft/` then fails
  installation and reconciliation. The craft-root row now carries a
  craft-only installer fixture in `design-pack-k175`. A new boundary row makes
  `personal-pack-k183` consume it. The pack check selects its branch by
  the absence of kind skills and a rules manifest. Disposition 4 of the review
  forbids a branch on the absence of `skills/`.
- **F4 is a real issue. Fix the artifact.** `using-testanyware` is one
  `SKILL.md`. Its command reference is output of the installed tool. The trial
  now requires implicit activation and a read of `SKILL.md`, without a VM and
  without an invented file. `owner-skill-delivery.md` states the same
  "TestAnyware resource" read for the staging probes. That is the same false
  premise, so its two lines now name the skill and the tool's command
  reference.
- **The seven doubts.** Dispositions 1, 2 and 7 of the review need no change.
  Disposition 2 is the trade-off that the specification already accepts.
  F1 and F3 absorb dispositions 3 and 4. Dispositions 5 and 6 are
  F2, F3 and F4.
- No finding needs a redesign, so no new producer chain is cut. This session
  spent no in-session reviewer.
