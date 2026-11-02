# owner-switch-program-k91

## Goal

Build the program that runs the owner's skill-delivery switch and its
automatic inverse. Build the rehearsal task that proves it in disposable
layouts. The real HOME stays unchanged.

## Context

- The contract is `docs/specs/owner-skill-delivery.md`: "The switch inside
  finish", "Concurrent writers", "Resuming the switch", "The rehearsed plan",
  "Rollback", "The recovery model" and "Test seams".
- owner-switch-recovery-k87 settled the recovery model, and
  owner-switch-recovery-k89 integrated its review. Read the handoff of k87
  and its amendments in k89, in their committed task records. They replace
  the earlier handoff items of k82, k84 and k86 about resumption, rollback
  and rehearsal.
- `docs/models/owner-switch.qnt` is the checked model. `task
  model:owner-switch` runs it.
- The evidence section "Publication by atomic exchange" records the measured
  `renamex_np` calls on macOS and APFS.

## Done when

- The program reads the state of the switch set from the layout, the run
  record and the plan, as "Resuming the switch" states. It reports each of
  the six states and continues as the table states.
- The program applies each part by stage file and exchange, or by an
  exclusive rename, and then compares the displaced content. It keeps each
  displaced content as a kept copy with a recorded digest. The digest covers
  the mode, the ACL and the extended attributes.
- The program writes each backup and each run record by an exclusive rename.
  Each backup holds a copy of the program. A conflict applies the inverse to
  the parts that the run changed.
- Before each write interval, the program lists processes against the kept
  processes only. A delegate harness session, a background command and a
  Ravel or Grove command of the session count as other processes.
- Step 14 compares the entries outside the switch set before it copies the
  receipt. A difference counts as a failed probe. The complete continuation
  runs step 14 when the manual lacks the receipt.
- A dry run reports the parts of a plan and changes nothing.
- A Taskfile task rehearses the program on a fixture plan in disposable
  layouts. It runs each switch control of "Test seams" that this leaf's
  behaviour covers, and it records the plan, the program and each result.
  `task model:owner-switch` stays green with its ten invariants and eight
  mutants.

## Notes

The probes are commands that the plan names. The rehearsal can use fixture
probes. The real probes are rehearsed on the real plan in cutover order 6.

The program runs outside every Grove workspace when it acts on the real
HOME. Its probes and core reruns drop Grove's loop control variables. A
program in one file makes the copy in each backup exact. Choose the location
and the language in this leaf, and record the reason.

A change to a recovery rule changes the model first. The owner's inverse, the
merge entry and the `mac-check` report belong to owner-switch-entries-k92.
This leaf can decompose if the program and its controls do not fit one
session. The recovery mechanism has no executable test yet, so earn a
`review-impl` at this leaf's boundary.
