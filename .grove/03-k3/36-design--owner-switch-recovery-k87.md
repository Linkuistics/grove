# owner-switch-recovery-k87

## Goal

Design the recovery model of the owner's skill-delivery switch again, as one
checked whole. Later rehearsals and planning then build on a contract that
review has stopped extending.

## Context

The subject is `docs/specs/owner-skill-delivery.md`. It covers the sections
"The switch inside finish", "Concurrent writers", "Resuming the switch",
"The rehearsed plan" and "Rollback", with the order-6 row and the test seams.
The recovery table of `docs/specs/coordinated-release.md` depends on them.
`docs/adr/finish-promotes-after-the-grove-release.md` limits what finish may
run after its teardown.

owner-pack-checkouts-k79 produced the switch's current shape. Three focused
reviews followed: k81, k83 and k85, integrated by k82, k84 and k86. Each
integration also spent an in-session reviewer. Each round found defects that
the previous repair introduced or exposed. Read the open defects from the
decision log of owner-pack-checkouts-k86, in its commit: R1 to R12 and the P3
items. In summary, they need:

- An exclusion that holds through the whole run and the owner's later
  inverse. A descendant must not pass its exemption by its absence.
- An inverse that restores native delivery when a file is drifted.
- Key-level treatment of the Ravel settings file, which `configure-grove`
  also edits.
- A receipt that the run checks against the layout. A withdrawal that reaches
  the manual, also after a successful finish.
- States bound to the manifest and records of the owner's classifications.
  A way to tell an interruption from an outside write.
- A rehearsal record that can decide reuse and covers the switch's command.
- The file mode and extended attributes through the exchange.

The section "Publication by atomic exchange" of
`docs/owner-skill-delivery-evidence.md` records the measured exchange calls.

The human's three decisions of 2026-11-02 stay closed. mise's
`[bootstrap.repos]` owns the owner's checkouts. Each checkout stays at a
published commit. No tool-neutral contract names mise. The design adds no
Ravel or Grove interface, and the switch stays inside finish after
promotion.

## Done when

- One model covers the switch, the inverse, interruptions, outside writes,
  classifications and receipts. It holds for finish, its continuation and
  the owner's runs after finish.
- Each of R1 to R12 and each P3 item has a recorded disposition.
- A checked model or a stated argument shows four properties. No write by
  another program is lost without a report, outside a stated accepted case.
  Exactly one state holds on each run. No receipt certifies a layout that is
  not switched. The real HOME applies only a rehearsed plan.
- The owner specification, the coordinated recovery row, the evidence, the
  views and the test seams agree.
- The producer cuts its focused review before skill-loading-replan-k80.

## Notes

The recommended route is `linkuistics:model-led-development`. Interruption
points, outside writers, inverses and receipts interleave, so the open
questions concern traces. A Quint model answers them with counterexamples.
Five prose reads have not converged on them. The model is a design artifact.
The executable rehearsal stays with order 6.

A smaller switch that removes a whole class of state is in scope. An example
is a design that stops comparing whole shared files. Record each rejected
alternative in the specification or, if it meets the test, an ADR.

The handoffs of k82 and k84 to skill-loading-replan-k80 still hold outside
the switch's recovery. This leaf replaces their items about resumption,
rollback and rehearsal.
