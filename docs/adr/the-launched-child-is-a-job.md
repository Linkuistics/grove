# The launched child is a job

`keyed-launch` spawns its child into a **process group of its own** and, when the
launcher owns a controlling terminal and is the foreground group of it, hands
that terminal to the child's group with `tcsetpgrp` — reclaiming it once the
child is reaped. Across the same spawn the child's dispositions for the
terminal-generated signals are reset to their defaults. The kill escalation then
signals `-pgid` as well as the pid.

Three things follow, and each closes a defect that a per-process launch cannot.
A terminal signal the human types is delivered to *the child's* group, so a
launcher survives a Ctrl-C it never has to catch, and the child receives one
whatever wrapper the command template names. **Only an ignored disposition
survives `execve`**, so a launcher that ignores SIGINT for its own reasons would
otherwise hand that ignore to the child, to everything the child spawns, and to
every login shell or `ssh` hop in between — each of which keeps ignoring it and
forces it onward, leaving an interactive session that cannot be interrupted at
all and cannot say why. And the escalation reaps descendants: a tool subprocess,
a language server, or an agent's own in-flight command dies with the session
rather than surviving the SIGKILL, holding the terminal, and holding whatever
locks it had taken.

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
  its token discarded (*[one live driver owns each working
  tree](./one-live-driver-per-working-tree.md)*). That bound still binds, but for
  a process that was never in the group.
- **Fix the inherited SIGINT in the driver instead, by installing an empty
  handler rather than `SIG_IGN`.** Rejected: it works — `execve` resets caught
  handlers — but it buys the driver EINTR on every call it makes while a signal
  arrives, and a driver whose whole purpose is to survive the interrupt should
  keep the disposition under which a syscall never sees one. The guarantee also
  belongs to the thing that spawns: a runner handing an interactive child a
  terminal cannot know what its caller did to its own dispositions.
- **Take a caller flag for job control.** Rejected: the gate is already exact and
  needs no configuration — a launcher with no controlling terminal cannot open
  `/dev/tty`, and one that is not the terminal's current foreground group has no
  terminal of its own to give away. A flag would only let a caller get that
  wrong.
