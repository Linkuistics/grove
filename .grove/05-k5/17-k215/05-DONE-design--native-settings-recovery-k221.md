# native-settings-recovery-k221

## Goal

Redesign how the native settings inside finish compose with the recovery of
the switch. Finish and the owner's later run must reach each state of the
switch's table through their documented entry points. No outside write to the
Ravel settings may be lost without a report.

## Context

- `homebrew-delivery-design-k219` reviewed this design. Its findings F1 and F2
  are the cases. Read them in the review's committed task file.
  `homebrew-delivery-design-k220` judged both real. It gave them to this leaf,
  because their repair changes the recovery design.
- `ravel-default-skill-directory-k218` added the section "The native settings
  inside finish" of `docs/specs/owner-skill-delivery.md`. Its running log
  holds the reasons. The section claims that the native settings add no
  state, record or inverse, so the recovery model stays unchanged.
- **F1.** The continuation after a stop and the owner's run after an
  unsuccessful finish both run the native settings before the switch. In the
  active and complete states of "Resuming the switch", the `skills` cell holds
  its switched value. Item 5 of the native settings then stops, so neither
  state reaches its row of the table.
- **F2.** Item 4 of the native settings exchanges the settings file before any
  backup exists, and it records nothing. An interruption between the exchange
  and its comparison loses an outside write without a report. A later write
  through a descriptor opened before the exchange is lost the same way.
  "Concurrent writers" requires detection of each write by another program,
  and P1 of `docs/models/owner-switch.qnt` states the same property.
- **An observation of k220, not of the review.** The continuation names the
  native settings but not the hooks installation. A finish that ends between
  the two leaves the plugin absent. Step 3 of the switch then finds the
  changed state, and a classification needs a new plan and a new rehearsal.
- The model's `resumeActive` and `resumeComplete` actions enter recovery
  directly. Its `stage`, `exchange` and `compare` actions model the switch's
  exchanges only, and the model holds no settings file.

## Done when

- The owner design states when each prelude applies: the native settings and
  the hooks installation. Finish's continuation and the owner's run reach each
  state of the table, the active and complete states included.
- Each write of the native settings takes part in detection, or the design
  states an equivalent mechanism. A flagged kept copy from that write is
  reported by each later run and by `mac-check`, also before the first backup.
- `docs/models/owner-switch.qnt` models the changed transitions before the
  design relies on them. `task model:owner-switch` stays green, and a mutant
  instance shows each new mechanism.
- The test seams cover the gap between the exchange and its comparison, a late
  descriptor write and both crash points of F1.
- The views in `docs/design/harness-selection-and-execution/` that show the
  native settings agree with the design.
- The design judges its own review chain. A chain that earns its place is cut
  with `leaf-insert` before `homebrew-delivery-plan-k217`.

## Notes

The review proposes that an open backup goes through its recorded program and
its state table before the native settings run. A simpler design may remove
the separate exchange. One example is a write inside a recorded run of the
switch's program. Compare such options before choosing one.

Two facts constrain the design. Finish's own launch reads the settings through
Ravel 1.1, which refuses a 2.0 key, so nothing writes the off value before
promotion. k218 chose the off value as the native state, so a design that
changes the native state states why.

The owner design runs its switch against the real HOME inside finish, so a
change to its recovery normally earns a review chain.
`homebrew-delivery-plan-k217` cuts implementation from the settled design and
waits for this leaf.

## Decisions (running log)

**The cases (2026-11-06).** The cited text of the owner design is unchanged
since `a7e7be78aef7`. F1, F2 and the observation of k220 hold as stated. A
third case of F1 also exists. An interrupted inverse whose last record is
"inverse begun" can have each cell switched, and item 5 stops there too.

**The baseline (2026-11-06).** A frozen copy of the unchanged model passed
every claim of `run-owner-switch.sh` in 47 seconds. The rarest witnesses were
`wit_deleteFlagged` in 14 and `wit_conflictFlagged` in 18 of 4,000 traces.

**The options (2026-11-06).** Five designs were compared:

- **A. A gate in step 3 and a settings record.** The preludes run inside
  step 3, only while no run is open. Before its exchange, the native settings
  write a durable record with the digests of the read and of the stage file.
  Chosen. It repairs F1 and the observation with one entry point, and it
  gives F2 the switch's own detection rule. The state table does not change.
- **B. The native settings as a recorded run with a backup.** Rejected. Its
  inverse would restore an absent key, which is not a native state, and the
  table would need a second kind of run.
- **C. An absent key as the native state.** Rejected. Ravel 2.0 would then
  deliver the keg sources beside the native installations in the native
  state. k218's reason for the off value still holds.
- **D. A stop for an existing file without `skills`.** Rejected. The owner's
  editor would then make the same undetected write by hand.
- **E. The off value as a part of the switch's plan.** Rejected. A cell has
  one earlier and one switched value. Two parts on one cell break the prefix
  rule and the inverse.

**The gate (2026-11-06).** Step 3 reads the run record first. While the
latest complete backup's run is open, no prelude runs. The layout then holds
each prelude cell at its earlier, switched or third value, and the table's row
decides.

Otherwise step 3 runs the native settings, then the hooks installation, then
reads the state. The continuation and the owner's run both run the switch from
step 1. The observation of k220 therefore closes too.

**The settings record (2026-11-06).** It lies beside the backups and is no
backup: it holds no plan and no run record. Step 3, step 13, the final reading
of step 9 and `mac-check` report its flagged kept copy, and step 8 stops while
it is flagged. Purge covers it. The creation of an absent file displaces
nothing, so it writes no record.

**An interrupted hooks installation (2026-11-06).** The installation now
disables an enabled `writing-hooks` plugin while no run is open. The enabled
value is the cell's switched value, which a classification never accepts. The
earlier stop therefore led only to the same disabling by hand.

**Claude Code's own writes (2026-11-06).** `marketplace add`,
`plugin install` and `plugin disable` rewrite `~/.claude/settings.json`, a file
of the switch set. The Homebrew evidence measured the first two. No exchange
of the program publishes those writes, so detection cannot cover them, and the
exclusion keeps the kept processes.

This is the class of F2 for another file, outside this leaf's goal. It goes to
a new design leaf before the planning leaf. The model treats each command as
one atomic write and says so.

**The native state (2026-11-06).** Unchanged. The native state holds the off
value, as k218 chose.

**The model (2026-11-06).** `owner-switch.qnt` now has an entry, the gate and
both preludes. The parts HOOKS and SETTINGS are absent at first. A sixth property
says that each entry reaches its row of the table. Four mutants remove the
gate, the settings record, the hooks installation from later entries and the
resumption of an interrupted installation. Two focused worlds reach the
native settings' gap and F1's first crash point.

**The first run (2026-11-06).** It failed, and each failure was in the
model. The trace of the prelude invariant showed another program disabling
the plugin between the installation and the disabling. The disabling step
then refused a native layout. The design gains the matching rule: the program
reads the plugin again before the disabling command and runs no command for a
disabled plugin.

The other failures were instruments. `wit_m_thirdValueLost` did not exclude
the new accepted case. The crash budget ran out in the preludes, so F1's first
crash point needs the world `active_entry`. The phase `run` included the
preludes, and a world with two parts had no ordinary part to classify. Each
of the four new mutants reached only its own violation.

**The second run (2026-11-06).** Every claim held, but several witnesses fell
to one or two of 4,000 traces. The first entry's preludes used up the outside
and crash budgets, and the stop at the table often ended a run that could
continue. Two instrument changes followed:

- The first state of each old world is the layout that the first entry's
  preludes leave. In four worlds, the first state comes before them. Two of
  those worlds, `prelude_crashes` and `both_preludes`, are new.
- The stop at the table acts only where no row continues. A stop of a row
  that could continue changes no cell, so it only delays the run.

The thinnest mutant world, `m_inverse_resets_third`, runs three times the
samples.

**The third run (2026-11-06).** It failed one claim. `both` never reached an
exchange conflict, and the conflict witnesses of `writers` fell below 15. A
scratch comparison isolated the cause. The later first state, not the
narrower stop, removed those conflicts.

Neither first state served every witness of `writers` or `both`. A larger
outside budget did not help either, because a cell write before the first
preludes ends most traces early. Two witnesses therefore moved to other
worlds, and none left the suite:

- `writers` has its first state before the preludes again, and keeps its
  conflicts.
- `both` keeps its later first state, and `both_preludes` takes its exchange
  conflict.
- `wit_driftedDuringRun` stays only in `during_run`, where it reached 2,683
  traces.

**The run of record (2026-11-06).** `task model:owner-switch` passed every
claim in 4 minutes 50 seconds, with the same digests before and after. The
evidence document records the digests and the counts. The two moved witnesses
reached 34 and 2,681 traces. The rarest witness, `wit_deleteFlagged`, reached
12 of 4,000, beside 14 in the baseline.
