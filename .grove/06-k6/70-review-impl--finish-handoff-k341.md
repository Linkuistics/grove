# finish-handoff-k341

**Reviews:** finish-handoff-k324

## Goal

Read the finish handoff adversarially before the finish session uses it.
Find each step whose command, order or pass condition is wrong. A wrong step
fails, harms the owner's machine or reports a success without evidence.
Produce findings, not changes.

## Context

- The subject is `docs/finish-handoff.md` and the section "The finish
  extension" of the root brief `.grove/_BRIEF.md`.
- The producer's task file `finish-handoff-k324` holds its Done when and its
  running decision log.
- The owners of the rules: `docs/specs/coordinated-release.md`, steps 6 and
  7 and "Recovery boundaries", and `docs/specs/owner-skill-delivery.md`,
  from "The final session and new launches" to "Purge".
- `docs/adr/finish-promotes-after-the-grove-release.md`, `CLAUDE.md`
  "Finish, integrate, and release", and `docs/RELEASING.md`.
- `docs/owner-cutover/owner-checklist.md` and
  `docs/owner-cutover/prepared-instructions.md`.
- Ravel's `docs/specs/release-channels.md`, "Promotion interface" and
  "Consumer readiness receipt", in
  `~/Development/AgenticOrchestration/Ravel`.

## Done when

- Each finding names the step, a concrete state that shows the defect, and
  the rule, command or source line that it breaks.
- The findings cover each doubt in Notes. Each doubt needs a reading of the
  command's source or help, not only of the document's prose.

## Notes

Every step after `finish-commit` runs with no `.grove/`, so no leaf can
carry a correction there. A defect found now costs one integration. The same
defect found inside finish costs a force-stop and a manual recovery.

The doubts to test:

- **The cleanup suppression.** The producer added
  `HOMEBREW_NO_INSTALL_CLEANUP=1` to `task release:minor`, because the
  task's `brew upgrade grove` would otherwise remove the running driver's
  Grove 22.8.0 keg. Verify that reading against
  `/opt/homebrew/Library/Homebrew/cleanup.rb`. Find any other Homebrew
  mutation in the sequence that runs without suppression. Find any step of
  the release task that the variable changes.
- **The receipt's routes.** Step 6 maps `native-1.1`,
  `native-2.0-delivery-disabled` and `directory-2.0` to the runs of steps 3
  and 5. Read `scripts/grove-installed-smoke.sh` and the release task. Find
  each route that the named run does not in fact exercise. The 2.0 lane of
  the installed smoke reads the real installation record, so decide which
  route its lifecycle runs.
- **The dry run against the real prefix.** Step 14 drops `--path` and gives
  `--prefix` the canonical kegs. Read `scripts/owner_switch_dry_run.py`.
  The comparison of step 15 names the `opt` links as its prefixes. Decide
  whether that run and that comparison agree row by row. Decide whether the run can pass gates 4
  to 6 against the real prefix, and whether its copied layout is complete.
- **The default-directory resolution.** Step 10 resolves in a new empty HOME
  under `env -i`. Decide whether that resolution reads the same shared
  directory that a default launch reads. Decide whether the jq output lets
  the operator verify each `originPath` against its owning keg.
- **The order.** Compare the order of steps 6 to 19 with step 7 of the
  coordinated release design. Compare it also with the switch's nine steps.
  The owner's checklist establishes the quiet interval at step 12, after the
  Homebrew mutations. Decide whether `native-off` at step 8 needs any part of the
  checklist first.
- **A differing digest.** Step 10 stops in native delivery when a digest
  differs. Gate 4 of `scripts/owner_switch.py` reruns the staging trials of
  a differing Grove source by itself. Decide which behaviour the design
  requires and whether the document states it.
- **Recovery.** Compare the recovery table with "Recovery boundaries" row by
  row. Find a failure in finish that maps to no row, and a row whose action
  the document changed.
- **The ending.** Verify that the document keeps the one confirmation and
  the teardown helper. Verify that `grove-llm record-teardown` comes before
  the original ending command. Verify the force-stop command for an
  incomplete finish. Verify that no step removes the Ravel 1.1.0 keg or the
  Grove 22.8.0 keg before the final signal.
