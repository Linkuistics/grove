# pack-membership-k189

**Reviews:** pack-membership-k171

## Goal

Read `docs/specs/pack-membership.md` adversarially before the moves apply it.
The specification governs every later move under `repository-moves-k5`, so a
wrong placement or a missing proof spreads to eight moves.

## Context

The human approved the first two decisions, because they change the root
table. Review their consequences, not the choice. The other four decisions are
the producer's alone. The views are in `docs/design/external-pack-contract/`,
under the topic "Pack membership".

## Done when

Each doubt below has a finding or a stated reason that it holds. Findings go to
the review's own commit. An `integrate-review-design` leaf follows only if a
finding is worth acting on.

## Notes

The producer could not settle these doubts alone:

1. **The asset rule.** Tests 1 and 3 can both match one of Grove's evidence
   records. Check that the order gives the intended result for each row of the
   table. Check also that Grove's own walkthrough books and their decision
   records stay in Grove while the skill's evaluations move.
2. **The doubt assertions.** They move to EngineeringSkills, and Grove's suite
   then reads no text of the skill. Find a change of Grove that the pack check
   would not see in time.
3. **The trial refusal.** The specification names `design-pack-k175` as the
   first move that removes a path a case names. Check that against each case's
   sources, mandatory files, planted files and launched kinds.
4. **Sources without a kind directory.** `check:packs` runs conformance on each
   pinned `git-subdir` entry. The specification gives the change for a craft-only
   source to `testanyware-skill-move-k172`. Check that the TestAnyware entry is a
   `git-subdir` entry at that point. PersonalSkills has only a `craft` root.
   Check whether that is a boundary that the table omits.
5. **The boundary table.** Check that each trial is the cheapest run that fails
   when its boundary breaks. Check that no move adds a boundary outside the
   table.
6. **The hooks plugins.** Native users keep the full writing plugin, and Ravel
   users enable the skill-free plugin. Check that the static proof of
   `writing-craft-pack-k181` stops a hook from running twice.
7. **The Codex listing.** The specification assumes that a Codex `local` source
   names a path inside the marketplace's own repository. Check that against
   Codex's marketplace rules.
