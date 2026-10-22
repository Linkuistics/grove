# The launched child is a job

`keyed-launch` spawns its child into a **process group of its own**. When the
launcher owns a controlling terminal and is its foreground group, the launcher
hands that terminal to the child's group with `tcsetpgrp`. It reclaims the
terminal once it reaps the child. It takes the terminal from the child's group.

When the child died of a signal, it takes the terminal from whichever group then
holds it. The two exceptions are the launcher's own group and the session
leader's group. It puts back the terminal attributes that it saved at the handover.

Across the same spawn, the launcher resets the child's dispositions for the
terminal-generated signals to their defaults. The kill escalation then signals
`-pgid` as well as the pid. Whenever the child exits, whatever ended it, the
launcher kills what remains of the child's group before it reaps the child.

A lifecycle session is two such jobs, one inside the other, as a shell would
make them. Grove's driver makes `ravel start-session` a job. Ravel
[supervises the harness](dispatch-supervises-the-harness.md) and makes the
harness a job. While the policy selects, the terminal belongs to Ravel's group,
which its policy worker joins. While the harness runs, the terminal belongs to
the harness's group alone.

Each launcher takes the terminal back from its own child. So a raw-mode harness
that the escalation killed leaves Ravel a terminal to restore, and Ravel leaves
Grove one. A launcher whose child died of a signal takes the terminal from
whichever group its launch left holding it. So a Ravel killed while its orphaned
harness held the terminal also leaves Grove a terminal.

Three things follow, and each closes a defect that a launch per process cannot
close. First, the terminal delivers a signal that the human types to *the
child's* group. A launcher therefore survives a Ctrl-C that it never has to
catch, and the child receives the signal whatever wrapper stands in front of it.

Second, **only an ignored disposition survives `execve`**. A launcher that
ignores SIGINT for its own reasons would otherwise hand that ignore to the
child and to everything the child spawns. It would also reach every login shell
or `ssh` hop in between. Each of these keeps the ignore and forces it onward. The
result is an interactive session that nobody can interrupt and that cannot say
why.

Third, the end of a launch reaps descendants. A tool subprocess, a language
server or an agent's own command in flight dies with the session. It does not
survive the SIGKILL, or the child's own exit, while it holds the terminal and
any locks it took. This holds only while the descendant stays in the group. A
harness that runs its commands in sessions of their own, as Claude Code does
for its shell commands, puts them beyond any group signal.

The launcher kills the group while the exited child is still unreaped. So no
other process can reuse the group's ID before the signal arrives. The launch
reports its end only once the group is gone.

The trade-off settled is **which** of the child's identities changes. To signal
a group at all, the child must lead one. If that goes wrong, the launcher cannot
hand the child the terminal at all. The child then reads the terminal in
competition with the launcher that had to hand it over.

## Considered options

- **Give the child its own session (`setsid`).** Rejected, because a session
  leader has no controlling terminal and no route back to one. `tcsetpgrp`
  returns ENOTTY from inside the child. It returns EPERM from the launcher,
  which cannot name a group in another session. `TIOCSCTTY` returns EPERM in
  both its plain and its stealing form, because the terminal already belongs to
  the launcher's session. Reopening the device by name gives a descriptor but no
  controlling terminal, and `/dev/tty` stays ENXIO. Both handover sites ignore
  the return of `tcsetpgrp`, and the launcher retries its own at every poll
  tick. So the failure repeats silently and nothing reports it.

  The child gets no protection from a stop either. The terminal raises SIGTTIN
  only for a background group *of a controlling terminal*. So the child's reads
  succeed, and it takes terminal input that the launcher never agreed to give
  up. A typed Ctrl-C goes to the launcher's group, which still holds the
  foreground, and never reaches the child. The escalation needs only the
  child's own group, and that group keeps the terminal. Reopen only for a
  launcher whose children are never interactive, where a session buys a useful
  detachment.

  The evidence is a measurement on a pseudo-terminal that a `setsid` leader made
  a controlling terminal (macOS 26.6, arm64). The child in its own session never
  stopped. It read a queued line while the launcher's group held the foreground.
  It did not receive the Ctrl-C that the launcher received. The control used
  the same reader, the same queued line and the same own group, but *in* the
  launcher's session. SIGTTIN stopped it, so the negative result is a reading
  and not a blind instrument.
- **Leave the escalation on the pid alone.** Rejected, because the compound
  failure is expensive and not only untidy. A surviving `grove-llm` grandchild
  holds shared epoch admission. The driver's post-reap invalidation then waits
  out its full 30-second bound. It then turns a session that finished correctly
  into a fatal error, with its ending unread
  (*[one live driver owns each working tree](./one-live-driver-per-working-tree.md)*).
  That bound still applies, but to a process that was never in the group.
- **Leave the rest of the group when the child exits, as a shell does.**
  Rejected. A tool that ignores TERM outlives a child that exits on the TERM. A
  running tool outlives a child that exits within the grace or on its own. A
  caller that relaunches or publishes on the ending would then act beside the
  tool, and a surviving `grove-llm` would hold shared epoch admission.

  A second grace for the survivors after the child exits was also declined. A
  descendant gets its time from the child's own shutdown. A child that wants a
  descendant finished waits for it. Reaping first and signalling the group
  afterwards was also declined, because another process can reuse the ID of an
  emptied group before the signal.
- **Reclaim the terminal only from the direct child's group, always.**
  Rejected. A supervisor that dies, as Ravel can under SIGKILL, leaves its own
  child's group holding the terminal. Its launcher would then never take the
  terminal back, and a raw-mode orphan would go on reading the human's input.

  Taking the terminal from any group except the launcher's own and the session
  leader's, after every launch, was also declined. A job-controlling shell nested
  inside another in one session leads no session. Suppose such a shell took the
  terminal after the driver died. It would lose the terminal when Ravel reaped a
  harness that simply exited.

  The broad take-back is therefore kept for a child that died of a signal. That is the one way in which a supervisor between can
  leave a delegated terminal unreclaimed. Its accepted residue: such a nested
  shell can still lose the terminal when a launch ends in a death by signal.
- **Fix the inherited SIGINT in the driver instead, with an empty handler in
  place of `SIG_IGN`.** Rejected. It works, because `execve` resets caught
  handlers. But it gives the driver EINTR on every call that it makes while a
  signal arrives. The whole purpose of the driver is to survive the interrupt,
  so it keeps the disposition under which no system call sees one.

  The guarantee also belongs to the code that spawns. A runner that hands an
  interactive child a terminal cannot know what its caller did to its own
  dispositions.
- **Keep the harness in Ravel's group, or Ravel in the driver's.** The first
  would deliver a typed Ctrl-C to the wrapper as well as to the harness. The
  wrapper would then have to catch what only the harness should receive. The
  second would put the driver in the terminal's foreground group while the
  policy selects. It would also leave the driver no group to address if Ravel
  died. Instead, each supervisor makes its own child a job. Any shell in front
  of Ravel does the same.
- **Take a caller flag for job control.** Rejected, because the gate is already
  exact and needs no configuration. A launcher with no controlling terminal
  cannot open `/dev/tty`. A launcher that is not the current foreground group of
  the terminal has no terminal of its own to give away. A flag would only let a
  caller get that wrong.
