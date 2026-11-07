# owner-switch-core-k239

## Goal

Build the core of the switch program and its rehearsal task. The core reads
the state of the switch set and runs the switch and its automatic inverse.
The rehearsal proves them on a fixture plan in disposable layouts.

## Context

- The node brief holds the contract, the language and the location.
- `docs/specs/owner-skill-delivery.md`: "The switch inside finish",
  "Concurrent writers", "Resuming the switch", "The rehearsed plan",
  "Rollback", "The recovery model" and "Test seams".

## Done when

- The program reads the state from the layout, the run record and the plan,
  and it reports each of the six states with its continuation.
- The program applies each file part by stage file, exchange and comparison,
  and each entry part by an exclusive rename. Each kept copy has recorded
  digests over the content, the mode, the ACL and the extended attributes.
- Backups and run records are written by an exclusive rename. Each backup
  holds the plan and a copy of the program. A conflict inverts the parts that
  the run changed.
- Before each write interval, the program lists processes against the kept
  processes only.
- Step 14 compares the entries outside the switch set before it copies the
  receipt. The complete continuation runs step 14 when the manual lacks the
  receipt.
- A dry run reports the parts of a plan and changes nothing.
- A Taskfile task rehearses the program on a fixture plan. It runs each
  control of "Test seams" that this slice covers, and it records the plan,
  the program and each result. `task model:owner-switch` stays green.

## Notes

Steps 4 to 7, the probes of step 10 and the native probe run commands that
the plan names, with Grove's loop control variables dropped. The preludes
and the concrete gates are the later slices.

## Decisions (running log)

**Plan format.** The plan is JSON with each path relative to HOME. So one
plan runs in a disposable HOME and in the real HOME, and its digest is the
same in each. The program's docstring states the format. A file part holds
cells of one JSON file, and a value `{"$absent": true}` is an absent key.
The plan names the job listing, the gates, the probes, the native probe and
`mac-check` as commands.

**Processes.** The kept processes are the program, its descendants, its
ancestors and the MCP servers of the nearest ancestor that the plan's
`session.harness` pattern names. Each other process that the plan watches,
and each other descendant of that harness process, keeps an interval closed.
Without a harness ancestor, the session is the program's parent.

**Rehearsal seam.** `OWNER_SWITCH_REHEARSAL_HOOK` names a command that the
program runs at each named point. Exit 99 is a crash. The program refuses the
hook when HOME is the account's home directory, so a real run cannot honour
it.

**Continuation.** A run that finds an open run executes the program copy of
that backup in place of itself when the digests differ.

**A finding.** An ACL entry that denies `delete` on a file of the switch set
also denies the exchange. The run then stops at a conflict and changes
nothing. The control `acl-denies-exchange` covers it.

**Verification.** `task owner:switch:rehearse` passed 93 of 93 controls with
Python 3.15.1. The digests of the program `c1d1cdd2f7ef` and of the
rehearsal script `b5bc773fc816` were the same before and after the run.
Eight controls also passed with the system Python 3.9.6.

**Model.** `mise exec -- task model:owner-switch` exited 0 with "all claims
hold". The model digest `c90bbf9ab658` and the runner digest `0fe7ef316e63`
were the same before and after the run. This slice changed no recovery rule.
