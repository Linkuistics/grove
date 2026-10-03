# The launched child is a job

`keyed-launch` spawns its child into a **process group of its own** and, when
the launcher owns a controlling terminal and is the foreground group of it,
hands that terminal to the child's group with `tcsetpgrp` — reclaiming it once
the child is reaped, from the child's group or, when the child died of a signal,
from whichever group then holds it unless that is the launcher's own or the
session leader's, with the terminal attributes it saved at the handover put
back. Across the same spawn the child's dispositions for the terminal-generated
signals are reset to their defaults. The kill escalation then signals `-pgid` as
well as the pid, and whenever the child exits, whatever ended it, the launcher
kills what remains of its group before reaping it.

A lifecycle session is two such jobs, one inside the other, as a shell would
make them. Grove's driver makes `harness-dispatch run` a job, and dispatch, which
[supervises the harness](dispatch-supervises-the-harness.md), makes the harness
one. While the policy is selecting, the terminal is dispatch's group's, which its
policy worker joins; while the harness runs it is the harness's group's alone.
Each launcher takes it back from its own child, so a raw-mode harness that the
escalation killed leaves dispatch a terminal to restore and dispatch leaves
Grove one, and a launcher whose child died of a signal takes it from whichever
group its launch left holding it, so a dispatch killed while its orphaned
harness held the terminal leaves Grove one too.

Three things follow, and each closes a defect that a per-process launch cannot.
A terminal signal the human types is delivered to *the child's* group, so a
launcher survives a Ctrl-C it never has to catch, and the child receives one
whatever wrapper stands in front of it. **Only an ignored disposition survives
`execve`**, so a launcher that ignores SIGINT for its own reasons would
otherwise hand that ignore to the child, to everything the child spawns, and to
every login shell or `ssh` hop in between — each of which keeps ignoring it and
forces it onward, leaving an interactive session that cannot be interrupted at
all and cannot say why. And the end of a launch reaps descendants: a tool
subprocess, a language server, or an agent's own in-flight command dies with the
session rather than surviving the SIGKILL, or the child's own exit, holding the
terminal, and holding whatever locks it had taken — so long as it stays in the
group. A harness that starts its commands in sessions of their own, as Claude
Code does its shell commands, puts them beyond any group signal. The group is
killed while the exited child is still unreaped, so the group's ID cannot have
been reused when the signal arrives, and the launch reports its end only once
the group is gone.

The trade-off settled is **which** of the child's identities changes. Signalling
a group at all requires the child to lead one, and the cost of getting that wrong
is a child that cannot be handed the terminal at all, reading it in competition
with the launcher that was supposed to hand it over.

## Considered options

- **Give the child its own session (`setsid`).** Rejected: a session leader has
  no controlling terminal, and there is no route back to one. `tcsetpgrp`
  returns ENOTTY from inside the child and EPERM from the launcher, which cannot
  name a group in another session; `TIOCSCTTY` is EPERM in both its plain and
  its stealing form, because the terminal is already the launcher's session's;
  and reopening the device by name gets a descriptor but no controlling terminal
  (`/dev/tty` stays ENXIO). Both handover sites ignore `tcsetpgrp`'s return, and
  the launcher retries its own every poll tick, so the failure is silent and
  repeated rather than reported. Nor is the child protected by being stopped:
  SIGTTIN is raised only for a background group *of a controlling terminal*, so
  its reads succeed and it takes terminal input the launcher was never asked to
  give up — while a typed Ctrl-C goes to the launcher's group, which still holds
  the foreground, and never reaches the child at all. Its own group is all the
  escalation needs, and it keeps the terminal. Reopen only for a launcher whose
  children are never interactive, where a session buys detachment worth having.

  Measured on a pseudo-terminal made a controlling terminal by a `setsid` leader
  (macOS 26.6, arm64): the child in its own session was never stopped, read a
  queued line while the launcher's group held the foreground, and did not
  receive the Ctrl-C the launcher did. The control — the same reader, same
  queued line, same own group, but *in* the launcher's session — was stopped by
  SIGTTIN, so the negative result is a reading rather than a blind instrument.
- **Leave the escalation on the pid alone.** Rejected: the compound failure is
  expensive rather than untidy — a surviving `grove-llm` grandchild holds shared
  epoch admission, so the driver's post-reap invalidation waits out its full 30s
  bound and then turns a session that finished correctly into a fatal error with
  its ending unread (*[one live driver owns each working
  tree](./one-live-driver-per-working-tree.md)*). That bound still binds, but for
  a process that was never in the group.
- **Leave the rest of the group when the child exits, as a shell does.**
  Rejected: a TERM-ignoring tool outlives a child that exits on the TERM, and a
  running one outlives a child that exits within the grace or on its own, so a
  caller that relaunches or publishes on the ending would act beside it, and a
  surviving `grove-llm` would hold shared epoch admission. A second grace for
  the survivors after the child exits was declined too: the child's own
  shutdown is where a descendant gets its time, and a child that wants one
  finished waits for it. Reaping first and signalling the group afterwards was
  declined because an emptied group's ID can be reused before the signal.
- **Reclaim the terminal only from the direct child's group, always.**
  Rejected: a supervisor that dies, as dispatch can under SIGKILL, leaves its
  own child's group holding the terminal, so its launcher would never take it
  back and a raw-mode orphan would go on reading the human's input. Taking it
  from any group but the launcher's own and the session leader's, after every
  launch, was declined too: a job-controlling shell nested inside another in one
  session leads no session, and one that took the terminal after the driver died
  would lose it when dispatch reaped a harness that had simply exited. The broad
  take-back is therefore kept for a child that died of a signal, the one way a
  supervisor between can leave a delegated terminal unreclaimed. Its residue,
  accepted: such a nested shell can still lose the terminal when a launch ends
  in a death by signal.
- **Fix the inherited SIGINT in the driver instead, by installing an empty
  handler rather than `SIG_IGN`.** Rejected: it works — `execve` resets caught
  handlers — but it buys the driver EINTR on every call it makes while a signal
  arrives, and a driver whose whole purpose is to survive the interrupt should
  keep the disposition under which a syscall never sees one. The guarantee also
  belongs to the thing that spawns: a runner handing an interactive child a
  terminal cannot know what its caller did to its own dispositions.
- **Keep the harness in dispatch's group, or dispatch in the driver's.** The
  first would deliver a typed Ctrl-C to the wrapper as well as the harness, so
  the wrapper would have to catch what the harness alone should receive. The
  second would put the driver in the terminal's foreground group while the
  policy selects, and leave it no group to address if dispatch died. Each
  supervisor makes its own child a job instead, which is also what any shell in
  front of dispatch does.
- **Take a caller flag for job control.** Rejected: the gate is already exact and
  needs no configuration — a launcher with no controlling terminal cannot open
  `/dev/tty`, and one that is not the terminal's current foreground group has no
  terminal of its own to give away. A flag would only let a caller get that
  wrong.
