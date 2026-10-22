# The launched child is a job

Grove launches `ravel` as a **job**, as a job-control shell does. One function
serves both callers. In the lifecycle, `ravel` leads a **process group of its
own** in Grove's POSIX session. When Grove owns a controlling terminal and is
its foreground group, Grove hands that terminal to the group of `ravel` with
`tcsetpgrp`. After the reap, Grove takes the terminal back and restores the
attributes that it saved. Across the spawn, Grove resets the dispositions of
`ravel` for the job-control signals to their defaults.

The caller contract states the procedures. These are the handover, the rules of
the reclaim and the handling of signals
([Jobs and authority](../specs/harness-selection-and-execution.md#execution-contract)).
Ravel ends its own job and its managed job
([Ravel supervises the harness](dispatch-supervises-the-harness.md)).

`grove run` runs `ravel` as a **detached job** instead. There `ravel` leads a new
POSIX session with no terminal, and Grove keeps the terminal. That child is
never interactive, so a session of its own costs it nothing.

A lifecycle session is two jobs, one inside the other, as a shell would make
them. Grove's driver makes `ravel start-session` a job. Ravel makes the harness
a job. While the policy selects, the terminal belongs to Ravel's group, which
its policy worker joins. While the harness runs, the terminal belongs to the
harness's group alone.

Each supervisor takes the terminal back from its own child. So a raw-mode
harness that Ravel's escalation killed leaves Ravel a terminal to restore, and
Ravel leaves Grove one.

Two things follow, and each closes a defect that a plain spawn cannot close.
First, the terminal delivers a signal that the human types to the foreground
group, which is the group of `ravel` or of the harness. The driver therefore
survives a Ctrl-C that it never has to catch. The harness receives the signal,
whatever wrapper stands in front of it.

Second, **only an ignored disposition survives `execve`**. The driver ignores
SIGINT for its own reasons. Without the reset, `ravel` would inherit that
ignore. Ravel passes its entry signal state to the harness, so the ignore would
reach the harness and everything that it spawns. It would also reach every login
shell or `ssh` hop in between. The result is an interactive session that nobody
can interrupt and that cannot say why.

The trade-off settled is **which** identity of `ravel` changes. To take the
terminal, `ravel` must lead a group in Grove's session. If that goes wrong,
Grove cannot hand `ravel` the terminal, and `ravel` reads the terminal in
competition with Grove.

## Considered options

- **Give `ravel` its own session (`setsid`) in the lifecycle.** Rejected,
  because a session leader has no controlling terminal and no route back to one.
  `tcsetpgrp` returns ENOTTY from inside the child. It returns EPERM from Grove,
  which cannot name a group in another session. `TIOCSCTTY` returns EPERM in
  both its plain and its stealing form, because the terminal already belongs to
  Grove's session. Reopening the device by name gives a descriptor but no
  controlling terminal, and `/dev/tty` stays ENXIO. Both handover sites ignore
  the return of `tcsetpgrp`, and Grove retries its own at every poll. So the
  failure repeats silently and nothing reports it.

  The child gets no protection from a stop either. The terminal raises SIGTTIN
  only for a background group *of a controlling terminal*. So the child's reads
  succeed, and it takes terminal input that Grove never agreed to give up. A
  typed Ctrl-C goes to Grove's group, which still holds the foreground, and never
  reaches the child. A session of its own suits only a child that is never
  interactive. That is the detached job of `grove run`.

  The evidence is a measurement on a pseudo-terminal that a `setsid` leader made
  a controlling terminal (macOS 26.6, arm64). The child in its own session never
  stopped. It read a queued line while the launcher's group held the foreground.
  It did not receive the Ctrl-C that the launcher received. The control used
  the same reader, the same queued line and the same own group, but *in* the
  launcher's session. SIGTTIN stopped it, so the negative result is a reading
  and not a blind instrument.
- **Run `grove run` as a foreground job too.** Rejected. `grove run` relays the
  transcript to its terminal, and a background group that writes there can stop
  on SIGTTOU. The detached job keeps the terminal with `grove run`, so a typed
  Ctrl-C reaches the process that decides about cancellation and publication.
- **Reclaim the terminal only from the group of `ravel`, always.** Rejected. A
  supervisor that dies, as Ravel can under SIGKILL, leaves its own child's group
  holding the terminal. Grove would then never take the terminal back, and a
  raw-mode orphan would go on reading the human's input.

  Taking the terminal from any group except Grove's own and the session leader's,
  after every launch, was also declined. A job-controlling shell nested inside
  another in one session leads no session. Suppose such a shell took the
  terminal after the driver died. It would lose the terminal when Ravel reaped a
  harness that simply exited.

  The broad take-back is therefore kept for a `ravel` that died of a signal.
  That is the one way in which a supervisor between can leave a delegated
  terminal unreclaimed. Its accepted residue: such a nested shell can still lose
  the terminal when a launch ends in a death by signal.
- **Fix the inherited SIGINT in the driver instead, with an empty handler in
  place of `SIG_IGN`.** Rejected. It works, because `execve` resets caught
  handlers. But it gives the driver EINTR on every call that it makes while a
  signal arrives. The whole purpose of the driver is to survive the interrupt,
  so it keeps the disposition under which no system call sees one.

  The guarantee also belongs to the code that spawns. A function that hands an
  interactive child a terminal cannot know what its caller did to its own
  dispositions.
- **Keep the harness in Ravel's group, or Ravel in the driver's.** The first
  would deliver a typed Ctrl-C to the wrapper as well as to the harness. The
  wrapper would then have to catch what only the harness should receive. The
  second would put the driver in the terminal's foreground group while the
  policy selects. It would also leave the driver no group to hand the terminal
  to. Instead, each supervisor makes its own child a job. Any shell in front of
  Ravel does the same.
- **Take a caller flag for job control.** Rejected, because the gate is already
  exact and needs no configuration. A process with no controlling terminal
  cannot open `/dev/tty`. A process that is not the current foreground group of
  the terminal has no terminal of its own to give away. A flag would only let a
  caller get that wrong.
