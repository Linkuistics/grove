# owner-switch-recovery-k88

**Reviews:** owner-switch-recovery-k87

## Goal

Read the rebuilt recovery model of the owner switch adversarially before
skill-loading-replan-k80 consumes it. Produce findings and no edits.

## Context

The artifact is the Grove commit of owner-switch-recovery-k87. Read its diff
against its parent `0eee04ac857d`, the commit of owner-pack-checkouts-k86.

The subject is these parts of `docs/specs/owner-skill-delivery.md`:

- The new terms: cell, part, plan, program and kept copy.
- The sections "The switch inside finish", "Concurrent writers", "Resuming
  the switch", "The rehearsed plan", "Rollback" and "The recovery model".
- The purge conditions, the order-6 row and the switch seams.

The subject also includes the owner recovery row of
`docs/specs/coordinated-release.md`. The model is `docs/models/owner-switch.qnt`
with its runner `docs/models/run-owner-switch.sh`. Run
`task model:owner-switch`. The evidence is
`docs/owner-skill-delivery-evidence.md`: the k87 rows of "Publication by
atomic exchange", "The recovery model" and the disposition table at the end
of "Review corrections". k87's decision log records each design decision and
the verification.

The defects that k87 answers are R1 to R12 and the P3 items in the decision
log of owner-pack-checkouts-k86. Compare the finish order with `CLAUDE.md`
and `docs/adr/finish-promotes-after-the-grove-release.md`.

The human's three decisions of 2026-11-02 stay closed. mise's
`[bootstrap.repos]` owns the owner's checkouts. Each checkout stays at a
published commit. No tool-neutral contract names mise. The design adds no
Ravel or Grove interface, and the switch stays inside finish after promotion.

## Done when

Each finding names its location, its severity and a concrete trace or text
that shows it. Record what the review did not inspect. Cut an integration
leaf only if a finding is worth acting on.

## Notes

The model checks consistency with itself. It cannot show that the right
properties were stated. Attack the statement as well as the result. The
producer could not settle these doubts:

1. The model treats each part as a file. An entry moves by an exclusive
   rename, and the inverse moves back an entry of any identity. Look for an
   entry trace that loses a write or that the state table misreads.
2. The accepted case lets a run replace an outside write of a cell's earlier
   or switched value. mise tracks the shared files and syncs them between
   machines. Decide whether that case can undo a deliberate change on
   another machine without a report that the owner sees.
3. Exclusion now protects readers, captures and progress only. The run's own
   process tree may run, and the staging descendant trial is gone. Look for a
   reader or capture that this admits.
4. A flagged kept copy stops step 8 and step 13 until the owner merges it.
   Check that no flagged copy can keep finish from restoring native
   delivery.
5. Each continuation of a backup runs the program copy in that backup. A
   defect in that copy then has no supported repair route. Decide whether
   the design needs one.
6. The manual says that a receipt applies only while `mac-check` reports
   the switch complete. That rule is prose, and no model holds it.
7. The inverse leaves a Ravel settings file that holds no key. No
   measurement shows how Ravel 1.1 and Ravel 2.0 read it.
8. The results are sampled. Each world ran 4,000 samples of up to 60 steps
   with three parts. Some witnesses landed in fewer than 50 traces.
   Compare the model's `state` and table predicates with the specification's
   table row by row.

Use the review family's procedure. The review makes no change to design,
production or test code.
