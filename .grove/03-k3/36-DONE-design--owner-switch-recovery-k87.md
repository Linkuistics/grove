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

## Decisions (running log)

**Base.** On 2026-11-02 this checkout was at `0eee04ac857d`, the commit of
owner-pack-checkouts-k86. The subject is the switch text of that commit.

**A measurement.** A scratch test on macOS 27.0.1, APFS, extended the k86
measurement. A descriptor that a writer opened before a `RENAME_SWAP` wrote
into the displaced name. The exchange gave the path the prepared file's mode
and extended attributes, and the displaced name kept the old ones.
`RENAME_EXCL` moved a directory and a dangling link, and it refused an
existing target with `EEXIST`. `RENAME_SWAP` also exchanged two directories.

**Design.** The recovery model is rebuilt on five decisions:

1. **Cells.** The switch changes cells: one key of a shared file or one
   file-system entry. Each cell has an earlier and a switched value in the
   plan. A part is one atomic step. A file part is one exchange, which
   changes all its cells together. An entry part is one rename. Nothing compares a
   whole shared file. This closes R4 and R5.
2. **Detection, not exclusion, protects writes.** Before each exchange, the
   run records the content that the displaced name must hold. A kept copy
   that holds other content is reported by every later check until the
   owner merges it. No exemption by evidence remains, and the run's own
   process tree may write. The exclusion stays for readers, captures and
   progress, and the program checks it at the start of each write interval.
   This closes R1, R2 and R3, and it removes the staging descendant trial.
3. **A run record.** The backup holds an append-only record: switch begun,
   receipt, inverse begun, inverse ended. The state comes from the layout,
   the record and the plan. Only an open run with a prefix pattern of
   switched parts is an interruption. This closes R9 with one exception.
   An outside write of a switched value during an open run is reset by the
   resumed inverse, which reports it.
4. **The plan is the record of classifications.** States use the plan of
   the latest backup while its run is open, and the prepared plan
   otherwise. A classification makes a new plan. A plan runs only with the
   program that a passing rehearsal names for it, and each backup keeps
   that program. No kind reuse remains. This closes R7, R8 and R10.
5. **A receipt describes, the layout decides.** A receipt records the
   switched layout that the probes saw. "Complete" also needs that layout
   now, so no withdrawal record exists. `mac-check` reports the state, and
   the manual says that the receipt applies only while `mac-check` reports
   it complete. This closes R6 and R12.

A conflict no longer makes a cell drifted. The run stops, inverts its own
parts, and the owner merges the reported copy. The swap-back alternative was
rejected: a crash during it can leave a write in a copy that looks unused.

**The checked model.** `docs/models/owner-switch.qnt` models parts, cells,
kept copies, the run record, plans, rehearsals, receipts, crashes and outside
writes at every step, with no exclusion. `docs/models/run-owner-switch.sh`
ran it with Quint 0.33.1: 4,000 samples of up to 60 steps for each world, and
three parts need 15 steps to a receipt.

All nine invariants held in the seven design worlds. Every witness was
reached, and every mutant witness stayed unreached there. Each of six mutants
reached its violation. Model digest `793ef9ef86a4`, runner digest
`5b75f97aa9bb`.

Writing the model found one rule the design lacked. A merged copy that a
descriptor writes into later was invisible. The owner's merge now records the
content that the copy held, and a later change flags it again.

The first run found two instrument defects. Outside writes spent their budget
before a receipt, so drift after a receipt was never reached. A phase dial
now has focused worlds, which hunt witnesses only. The late-write witness
also fired on every conflict, and a ghost set of late writes now separates
the two.

**No ADR.** The redesign fails the first condition of the ADR test. Nothing
is implemented, and the switch is a one-off design on one machine, so it is
easy to reverse. The specification records the three rejected alternatives:
the exemption by staging evidence, the swap back and whole-file comparison.
The finish decision still holds unchanged.

**Artifacts.** The owner specification replaces "The switch inside finish",
"Concurrent writers", "Resuming the switch", "The rehearsed plan" and
"Rollback". It adds "The recovery model", five terms, a purge condition, the
order-6 row and the switch seams. The staging descendant trial is removed.

The coordinated recovery row names the new states and the kept copies. The
evidence records the k87 exchange measurement, the model run and a
disposition for each of R1 to R12 and the P3 items. The views gain the state
view `owner-switch-states`, the cutover view follows the new states, and the
README and the discussion panel name the change. `Taskfile.yml` gains
`model:owner-switch`.

**Verification.** The prose checker reports no hard finding in the four
changed Markdown documents. A scratch resolver checked 66 relative links in
them. Three of them name explicit `<a id>` anchors, which exist. The rest
resolved. A control file with a missing heading made the resolver report one
failure.

`shellcheck` passes on the runner. `diagrams.json` parses.

A browser rendered the state view with its six states and 15 transitions. It
rendered the cutover view with each new label. Neither showed a syntax error.
The only console error was a missing `favicon.ico`. This session removed the
browser tool's three files from `.playwright-mcp/`.

**Handoff to skill-loading-replan-k80.** These obligations replace the items
of k82, k84 and k86 about resumption, rollback and rehearsal:

1. A switch program. It reads the six states and applies each part by
   stage, exchange or exclusive rename, and comparison. It writes kept copies
   with their recorded digests, which cover mode, ACL and extended
   attributes. It writes the run records by exclusive rename. It lists
   processes before each write interval. It keeps a copy of itself in each
   backup.
2. Entries of that program for the owner's inverse and for recording a merge.
3. A `mac-check` report of the switch state and of flagged kept copies.
4. A rehearsal task. It runs the program on a plan with every switch control
   of the test seams. It records the plan, the program and each result.
5. `task model:owner-switch` stays green. A change to a recovery rule
   changes the model first.

**Review.** A focused `review-design` leaf goes before skill-loading-replan-k80,
as the brief requires. This producer has a scheduled review, so it spends no
in-session reviewer.
