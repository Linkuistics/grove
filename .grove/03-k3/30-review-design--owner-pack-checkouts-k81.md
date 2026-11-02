# owner-pack-checkouts-k81

**Reviews:** owner-pack-checkouts-k79

## Goal

Read the revised owner-delivery design adversarially before
skill-loading-replan-k80 consumes it. Produce findings and no edits.

## Context

The artifact is the Grove commit of owner-pack-checkouts-k79. Read its diff
against its parent `a20454a854b7`. The main subject is the section "Pack
checkouts" of `docs/specs/owner-skill-delivery.md`. The subject also includes
that file's staging, final-session, switch, rollback and cutover sections.

The evidence is `docs/owner-skill-delivery-evidence.md` and
`docs/evaluations/owner-skill-delivery/checkouts.py`. Run
`task owner:checkouts`, and run the script with `--mutate`. The script writes
only into a temporary directory.

The human fixed three decisions on 2026-11-02, and the review does not reopen
them:

- On the owner's machine, the `[bootstrap.repos]` part of mise owns the pack
  checkouts.
- Each checkout stays at a published commit, the pin, which the owner advances
  by an edit.
- No tool-neutral contract requires mise. Ravel's specification, Grove's
  delivery specification and the external-pack text name no tool.

Compare the design with Ravel's local-source contract `7a894c782037`, in the jj
workspace `Ravel.skill-directory-delivery`. Compare it with mise's own help and
reference, not with memory. Do not run a mise command that changes the real
HOME.

## Doubts to test

An in-session reviewer found 22 findings, and k79 fixed all of them. Three
fixes change the design, and no executable test covers them:

1. **The `grove-core` pin advance inside finish.** Switch step 4 advances the
   pin to the commit of finish's Grove release and runs `apply`. Test the
   order against the coordinated release design and the finish rules in
   `CLAUDE.md`. Test also the rollback and the receipt, and the case where
   `apply` fails inside finish.
2. **The separate checkout HOME for staging trials.** The checkouts now lie
   outside the staging HOME. Test whether the staging settings, the isolation
   trial and Ravel's sandbox recipe still hold. Test also whether the
   credential rule prevents every read of the real keychain.
3. **An advance only while no Ravel launch runs.** Test whether the owner can
   satisfy the rule while a Grove loop runs. Test whether the switch and the
   preparation steps satisfy it.

Also test the handoff text in k79 for external-research-pack-k4 and the root
brief. The text must name no tool. Its order must keep each pinned-Git clause
superseded until k80 removes it.

## Done when

- Each finding names its file and line, its evidence and a severity.
- The review cuts an `integrate-review-design` leaf only if a finding needs
  action.
