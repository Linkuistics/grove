# A review carries its creator reference

A review selected through the supplied review policy learns its original
creator's provider from a reference in its own task file, never from a lookup by
task handle. The session that finishes a producer launched through
`harness-dispatch` takes the run ID from its own environment and writes
`**Creator:** run <run-id>` directly under `**Reviews:**` on its review leaf; a
finishing session with no run removes any such line instead. The
policy reads that run's immutable catalog snapshot. An artifact with no run,
finished before adoption or by a direct harness, carries the owner's
`**Creator:** declared <provider>` instead, and a review with neither refuses.
The [area specification](../specs/harness-selection-and-execution.md#identity-and-creator)
owns the reference forms, the checks and the refusals.

The trade-off is one line in a review body against durable state somewhere
else. Handles restart in every grove and one workspace hosts successive groves,
so a lookup by handle has to tell this grove's handle from an earlier grove's.
[One live driver owns each working tree](one-live-driver-per-working-tree.md)
keeps no grove generation, and reopens that only if handles must be comparable
across separately created groves. A run ID is unique without any namespace.
Carrying it in the tree therefore keeps handles incomparable and Grove
stateless. The reference dies with `.grove/`, while the run and observation
records it names survive teardown. It also fixes which invocation is the
original creator: the session that finished the producer, by retiring its leaf
or by closing its node. A restarted producer's finishing
session replaces the line, or removes it when it ran without dispatch, before
the review runs. A review's retry reads the same line and the same record.

The methodology carries the line as an amendment to three of its rules, which
the human confirmed in `harness-selection-and-execution-k8`:

- `body-carries-no-launch-metadata` — a body still carries nothing that routes
  its own session. A review body may carry one `**Creator:**` line, the only
  record of a past session any body carries.
- `retirement-is-filename-only` — retirement still touches one filename. The
  claim that a waiting review needs no record of how its producer ran is
  rescoped. In its task's commit, a session writes its line for each producer
  it finishes, on the review it cuts and on any live review already naming that
  producer's handle, or removes the line when it has no run. It finishes its own
  leaf and each node its close cascade closes, so the node-close steps and
  their `node-close-four-steps` row carry the same step. The step has a
  conformance row of its own, `finishing-session-names-its-run`, so a second
  file that restates it fails.
- `diversity-is-the-configs` — Grove still records and compares nothing about
  how a producer ran. The producing session names its run, and the dispatcher's
  policy does the comparing.

The statements that no code reads the relationship lines are scoped to Grove's
own code: the glossary, `TASK-FORMAT.md`, `docs/ARCHITECTURE.md` and
`docs/USAGE.md` all make one. The requirements already implied this by giving
the supplied adapter the `**Reviews:**` relationship. The amendments are in the
Grove plugin and in the skills Grove provisions to Codex, and ship in the same
release as the dispatcher, so the methodology never asks a session to name a
run from a tool that is not installed. Conformance rows and
composition-guidance pins hold their wording.

The costs are visible. The reference depends on a session copying its run ID,
and is that session's attestation: the provider is execution-recorded, the
association is not. A missing line or an unknown run refuses. A line naming some
other existing run lends that run's provider, and a session can reach such IDs
in other review bodies, in version history, or inherited by a direct harness
launched inside a dispatched session. An attempt whose execution is unknown
passes on the line's word too. Inspection shows the named run's task identity
beside the reviewed handle; launch does not compare them. The run that closes a
decomposed producer's node stands for the whole producer, whatever kind that
child was. An artifact without a run needs a human declaration once it is
finished, the per-review step the requirements accept for that case. Producer
and review must use the same record store.

## Considered options

- **Look up the reviewed handle's runs under a per-grove dispatch scope.** This
  was the first design. It kept durable grove state in the workspace control
  area, which the one-live-driver record says holds none. Its inode tripwire
  refused a benign recreation of the root, and its only remedy orphaned every
  creator in the grove. Reopen only if Grove gains a durable grove identity for
  some other reason.
- **Mint a durable grove ID under `.grove/` and key runs by it and the handle.**
  Rejected because it reopens the one-live-driver record's rejected generation
  identifier only to make handles comparable. It also keeps a namespace, a run
  lookup and a declaration command. Reopen if handles must become comparable
  across groves for another reason.
- **Look up by workspace path and handle, storing nothing.** Rejected as unsafe.
  Take a direct-harness artifact beside a stale same-handle run from an earlier
  grove in the same workspace. The lookup would read the stale provider and
  could admit a reviewer from the real creator's provider.
- **Have the Grove runner supply the provider.** Grove launches an opaque
  command. It could learn the provider only by persisting launch receipts,
  which its configuration contract and its review-target-diversity rule exclude.
  Reopen only if Grove stops treating commands as opaque.
- **Register creators explicitly with `creator bind` and `creator declare`.**
  Nothing produced the execution evidence that binding required, so every
  unattended review stopped for a human declaration (review
  `harness-selection-and-execution-k5`, finding F1).
- **Write a `**Produced-by:**` line into the producer at retirement.**
  Retirement would stop being a filename-only transition. `complete` runs after
  the task's commit, which is too late to write.
- **Write the provider name instead of the run ID**, or have a finishing session
  without a run declare its own provider. A transcribed name cannot be told
  apart from a declaration, so recorded provenance would lose its meaning, and
  a declaration is the owner's assertion rather than the session's.
- **Require the named run's task identity to equal the reviewed handle.** A
  decomposed producer is finished by a child task with its own handle, so the
  check would refuse correctly named runs. All it would add is protection
  against a transcription error. Reopen if sessions are seen to copy the wrong
  run.
- **Have `grove-llm leaf-add` copy the run ID itself.** Grove's binary would
  then depend on a dispatcher's environment contract. Reopen if sessions are
  seen to omit the line.
