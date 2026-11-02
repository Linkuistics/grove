# owner-pack-checkouts-k83

**Reviews:** owner-pack-checkouts-k82

## Goal

Read the repaired owner-delivery design adversarially before
skill-loading-replan-k80 consumes it. Produce findings and no edits.

## Context

The artifact is the Grove commit of owner-pack-checkouts-k82. Read its diff
against its parent `03794cdd5645`. k82 integrated the review
owner-pack-checkouts-k81 of the design owner-pack-checkouts-k79. One
in-session reviewer then found nine more defects, and k82 repaired them too.

The subject is these parts of `docs/specs/owner-skill-delivery.md`:

- The section "Pack checkouts": the `grove-core` candidate and the advance
  rule.
- The section "Settings isolation and the staging HOME": the clone
  environment and the credential control.
- The section "The switch inside finish": the steps, the continuation after a
  stop or an inverse, the probes and the receipt.
- The order table and the staging trials of "Cutover contract".

The subject also includes step 5 and the recovery table of
`docs/specs/coordinated-release.md`. The evidence is
`docs/owner-skill-delivery-evidence.md`. k82's decision log records each
disposition and the verification.

The human's three decisions of 2026-11-02 stay closed. mise's
`[bootstrap.repos]` owns the owner's checkouts. Each checkout stays at a
published commit. No tool-neutral contract names mise.

Compare the finish order with `CLAUDE.md` and
`docs/adr/finish-promotes-after-the-grove-release.md`. Compare capture with
Ravel's local-source contract `7a894c782037` in the jj workspace
`Ravel.skill-directory-delivery`. Compare Grove's loop variables with the
section "Environment" of `docs/specs/harness-selection-and-execution.md`.

## Doubts to test

1. **The continuation after a stop or an inverse.** Test whether finish can
   always end without the success signal, and whether the owner can always
   complete activation later. Test the case where the inverse itself fails.
2. **The Grove patch release after a failed core trial.** Test it against the
   recovery rules of the coordinated release design and `docs/RELEASING.md`.
   Test whether the tree ID of the final line can still differ at the released
   commit.
3. **The advance rule in daily use.** Test whether the process check is
   enough while harness sessions run. Test `mac-bootstrap` and a pin that
   `mise dot sync` brings from another machine.
4. **The probe environment.** A probe runs outside the workspace without
   Grove's loop variables. Test whether any route remains by which a probe
   could end or finish this driver's loop.
5. **The credential control.** Test whether the routes are complete under Git's
   documented configuration and credential rules, and whether the control can
   fail.

## Done when

- Each finding names its file and line, its evidence and a severity.
- The review cuts an `integrate-review-design` leaf only if a finding needs
  action.

## Notes

k82 did not change `docs/evaluations/owner-skill-delivery/checkouts.py`. Its
recorded run passed 33 cases, and `--mutate` failed the one expected case.
