# owner-pack-checkouts-k85

**Reviews:** owner-pack-checkouts-k84

## Goal

Read the repaired switch recovery and the source identity adversarially
before skill-loading-replan-k80 consumes them. Produce findings and no edits.

## Context

The artifact is the Grove commit of owner-pack-checkouts-k84. Read its diff
against its parent `e3070a5cc489`. k84 integrated the review
owner-pack-checkouts-k83. One in-session reviewer then found eleven more
defects, and k84 repaired them too.

The subject is these parts of `docs/specs/owner-skill-delivery.md`:

- The term "switch set" and the identity paragraphs of "Pack checkouts": the
  tree ID, the `resourceDigest` and the accepted digests.
- The section "The switch inside finish": steps 1 to 14, the continuation
  after a stop and the owner's run.
- The sections "Resuming the switch" and "Rollback", with the native probe.
- The order table and the test seams.

The subject also includes step 5 and the recovery table of
`docs/specs/coordinated-release.md`. The evidence is
`docs/owner-skill-delivery-evidence.md`, with its section "Checkout
configuration and the tree ID". k84's decision log records each disposition
and the verification.

The human's three decisions of 2026-11-02 stay closed. mise's
`[bootstrap.repos]` owns the owner's checkouts. Each checkout stays at a
published commit. No tool-neutral contract names mise.

Compare capture and resolution with Ravel's local-source contract
`7a894c782037` in the jj workspace `Ravel.skill-directory-delivery`. Compare
the finish order with `CLAUDE.md` and
`docs/adr/finish-promotes-after-the-grove-release.md`.

## Doubts to test

1. **Comparison by key.** Test whether the earlier and switched values of each
   plugin key are well defined in Claude Code's `settings.json` and Codex's
   `config.toml`. Test whether a comparison before each rename can still lose
   a concurrent write. Test whether step 1 can name every writer.
2. **The drifted route.** After the owner classifies a drifted item, the
   manifest and the dry run are prepared again without a new rehearsal. Test
   whether that route can lose the original recovery material, or reach a
   switch that nobody rehearsed.
3. **The state table.** Test whether each layout that an interruption or an
   outside change can produce has exactly one state. Test whether each state's
   next steps use only evidence that exists at that moment.
4. **The accepted digests.** Test where each accepted digest comes from: the
   staging trials, rollout's published archives, or a step-4 rerun. Test
   whether the disposable-HOME resolution of step 6 measures what the real
   HOME's launches capture.
5. **The native probe.** Test whether its four observations prove a working
   native route on both harnesses. Test what `ravel skills resolve` reports
   when no settings file exists.

## Done when

- Each finding names its file and line, its evidence and a severity.
- The review cuts an `integrate-review-design` leaf only if a finding needs
  action.

## Notes

k84 did not change `docs/evaluations/owner-skill-delivery/checkouts.py`. No
executable test covers the repaired switch. Its rehearsals and controls are
obligations of later leaves.
