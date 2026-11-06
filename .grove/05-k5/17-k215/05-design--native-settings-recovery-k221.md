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
