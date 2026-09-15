# item-status-k3

## Goal

Adversarially review the item-status design produced by `item-status-k2` against
the human-agreed requirements in the root brief. The artifacts are
`docs/specs/item-status.md` and the observation extension in
`docs/adr/one-live-driver-per-working-tree.md`, with their glossary and module
ownership citations. Produce findings; implementation is not this leaf's work.

## Context

Read the producer's committed artifact and the root requirements as the contract.
The human's final refinements require ✓ for DONE and ✗ for ABANDONED at the
start of each label and status colors for terminal items, while explicit words
remain. The last refinement reserves the active highlight for RUNNING; ordinary
LIVE text stays neutral and cursor selection remains independent.

Aim the read at concrete ways the design could fail:

- Can a dead predecessor or another viewer manufacture a current RUNNING
  observation during replacement's old-epoch handoff? Check witness probe mode,
  independent descriptors, lease-owned lifetime and release order.
- Does the generic runner's Started/Reaped contract cover failed spawn,
  immediate exit, supervision errors, escalation and publication failure
  without changing admission or completion authority?
- Can tree replacement, including a first viewer opened after replacement,
  attach an old mandate to a reused key? Check when selection pins the root and
  when that pin can be dropped, including decomposition and disappearance, and
  the observer's final probe after pinning its own tree identity.
- Does `try_observe` actually admit a read-only, nonblocking implementation
  with separate tree/runtime errors, safe guard order and bounded consistency
  checks? Attack stale current pairs and multi-viewer contention.
- Does exclusion happen before finish eligibility without hiding malformed
  input, excluding descendants of a running branch or inventing a sentinel?
- Can the 60 × 10 layout, colors, cursor and glyphs satisfy the requirements
  while preserving both viewports, file source anchors and activity summaries?

Source anchors for feasibility: `DriverLease`, epoch record parsing and
`probe_live_lease_with_post_unlock_hook` in
`crates/grove-loop/src/driver_lease.rs`; `launch_configured_session` and `drive`
in `crates/grove-loop/src/loop_driver.rs`; `selected` in
`crates/grove-loop/src/task_tree.rs`; `run`, `supervise` and `watch` in
`crates/keyed-launch/src/run.rs`; `Workspace` in
`crates/jj-workspace/src/lib.rs`; the Viewer application and observation adapter
in `crates/grove-tui/src/lib.rs` and `observation.rs`.

The producer used graph Tier 2, project
`Users-antony-Development-grove.make-item-status-obvious-in-tui`, generation
`2026-09-15T11:44:22Z`. Those code paths and the browser/driver-lease tests had
matching metadata and no recorded coverage gaps; docs are excluded and were
read directly. Search/trace pages used for the bounded source anchors completed.
Some inferred call edges pointed to unrelated helpers, so exact source governed
material claims. Reconfirm freshness; this is not repository-completeness evidence.

## Done when

The spec, runtime decision and acceptance seams have been checked against the
agreed contract and source feasibility, with actionable findings recorded in
the review's own artifact. If findings need integration, place its leaf ahead
of the waiting `item-status-k4` planning step. A clean review needs no integration
leaf. Keep the spec and ADR set coherent; do not turn preferences into findings.

## Findings

Reviewed at commit `d13209e1d39a` (`item-status-k2`): `docs/specs/item-status.md`,
the observation extension in `docs/adr/one-live-driver-per-working-tree.md`, the
four new glossary entries, and the pointer edits in `CONTEXT-MAP.md`,
`docs/ARCHITECTURE.md`, `docs/specs/module-decomposition.md` and
`docs/specs/user-guide-coverage.md`, against the root brief and the source
anchors named in Context. `path:line` citations are that commit's. Each finding
names its anchor, the evidence, and the classification this reviewer proposes;
the integration grades every one itself and may reject any of them.

### What held

The six attack axes in Context were checked against the source and the design
answers them as written; none produced a finding on its own.

- A dead predecessor cannot be reported RUNNING during replacement's old-epoch
  wait: the replacement takes the lease before the epoch, preserves the old
  lease bytes, and never locks the old witness; the dead driver's witness lock is
  gone, a shared probe succeeds, and success means Idle. Concurrent viewers take
  only shared probes, which are mutually compatible, so one viewer cannot make
  another see contention.
- The runner's Started/Reaped events map onto `run`/`watch` in
  `crates/keyed-launch/src/run.rs`: a spawn error returns before supervision
  (no Started); every ordinary exit and every escalation path passes a confirmed
  `wait`; the `try_wait` error path is the one place reap is unconfirmed, and the
  ADR's rule that no Reaped is invented there is implementable. Admission and
  completion authority are untouched.
- Pin-before-probe closes the inode-reuse join: the driver's open task-root
  descriptor keeps the old inode allocated while its mandate is observable, so a
  first viewer after replacement compares a different identity. The viewer's own
  retained root descriptor already gives the same guarantee for its comparisons.
- Guard order is epoch then tree everywhere a process holds both (agent
  admission, viewer); the driver never nests them, and every viewer attempt is
  nonblocking, so there is no lock-order deadlock.
- Exclusion after whole-tree validation and before finish eligibility matches
  `selected` in `crates/grove-loop/src/task_tree.rs`; a decomposed running item
  is a node and was never a candidate, so only its identity is excluded and its
  children stay eligible. Nothing on the viewer's side writes a sentinel.
- The 60 × 10 arithmetic is right: 58 inner cells, 22 fixed, 36 for the item;
  five chrome lines leave three bordered content rows. Tab already maps to one
  action in the terminal fixture.

### item-status-k3 F1 — A stalled or suspended viewer holding the shared epoch guard can stop the loop (medium)

Anchor: `docs/adr/one-live-driver-per-working-tree.md:244` (step 2, the shared
epoch guard), `:259` (step 5, the tree guard taken under it), `:272` (release
after the caller's capture), `:284` ("Multiple viewers may share a short
epoch/tree read"); `docs/specs/item-status.md:36` ("The guard owns whatever
short read guards the operation acquired. Callers copy rows and selected bytes,
then drop it").

Evidence: every driver epoch write goes through `acquire_epoch_file_with`
(`crates/grove-loop/src/driver_lease.rs:360`), which retries nonblocking for a
fixed 30 s and then fails. In `drive` (`crates/grove-loop/src/loop_driver.rs:280`)
a failed pre-spawn activation aborts the launch and a failed post-reap
invalidation stops the loop with "completion signal left unconsumed". The ADR's
own *Orphaned* paragraph (`:86`–`:91`) names the residual holder class as a
process "started from another session, another terminal, or by hand". `grove
view` is exactly that process: interactive, long-lived, outside the session's
process group so the escalation never reaps it, suspendable with Ctrl-Z, and by
design it holds the shared epoch guard across the whole tree walk and the
selected-file read, twice per 500 ms sample for the consistency pair. A suspend
or stall that lands inside that window stops the driver's loop 30 s later. At
the mild end, any overlap prints the driver's "waiting for … session epoch lock"
diagnostic into the human's terminal between sessions, a new side effect of
merely running the viewer beside the driver. The requirements forbid the viewer
waiting behind a long driver handoff (`.grove/_BRIEF.md:81`) and the design meets
that; the converse, the driver waiting behind the viewer with a loop-fatal
bound, is not stated anywhere in the spec or ADR. The pre-existing shared tree
lock has the same window and stalls a session's exclusive `grove-llm` verb
indefinitely; that is older and outside this design, but the epoch guard adds
the loop-stopping consequence.

Proposed classification: real issue, with a visible trade-off to record either
way. Two repairs are available without changing the evidence model: hold the
epoch guard only around the record read, witness identity check and probe, and
do the tree capture and file read outside it (pin-before-probe survives, since
the root descriptor is opened first); or take no epoch guard at all and rely on
the nonce/identity binding plus the two-capture comparison the spec already
specifies for runtime state. Whichever is chosen, the residual should be stated
in the ADR beside the orphan case.

### item-status-k3 F2 — Duplicate-key validation in the shared selection rule is an unstated driver behaviour change (medium-low)

Anchor: `docs/specs/item-status.md:84` ("First validate the whole tree,
including duplicate keys and multiple live finish leaves") together with `:80`
("Ordinary driver selection supplies no exclusion"), which makes it one
operation for driver and viewer.

Evidence: `selected` (`crates/grove-loop/src/task_tree.rs:566`) refuses only
multiple live finish leaves. The loop tolerates duplicate keys today
(`task_tree.rs:425` records the by-key tie-break as a known miss); only the
viewer's adapter refuses them (`crates/grove-tui/src/observation.rs:118`). If the
shared operation validates duplicates, bare `grove` starts refusing a tree it
launches today, and nothing in the spec, the decision log or the user-facing
docs says so. If instead the validation is meant only when an exclusion is
supplied, "one selection rule" is two rules and the sentence should say which.
Some answer is needed: excluding by a duplicated key is genuinely ambiguous.

Proposed classification: contract stated unclearly. Decide, then say it in the
spec's selection section; if the driver's behaviour changes, the change belongs
in the spec's decisions and reaches `docs/USAGE.md` with the implementation.

### item-status-k3 F3 — The pin-outlives-witness invariant is split across two owners whose natural drop order is the reverse (medium-low)

Anchor: `docs/adr/one-live-driver-per-working-tree.md:160` ("Selection retains
an open task-root directory descriptor"), `:163` ("The driver retains this pin
through the session witness's lifetime"), `:210` ("The lease owns the witness"),
`:215` ("The driver keeps the tree pin for at least as long as that witness").

Evidence: `Selection` (`crates/grove-loop/src/task_tree.rs:534`) is a
per-iteration local of `drive` (`crates/grove-loop/src/loop_driver.rs:244`); the
lease is `run`'s by-value parameter and outlives every iteration
(`loop_driver.rs:193`). On the supervision-error-without-reap path the ADR says
the witness must not be dropped early and is released only through descriptor
lifetime, that is at lease drop; the selection, and with it the pin, is dropped
when `drive` returns, before `run` drops the lease. The invariant is then false
for that window. `Selection` is also `pub`, `Clone`, `PartialEq` and `Eq`, which a
retained `File` breaks; that is an implementation detail, but it is a sign the
pin does not naturally live where the ADR puts it.

Proposed classification: contract stated unclearly. Give the pin the witness's
owner (transfer it into the lease or a launch-scoped value at publication, with
the drop order declared), and say so once.

### item-status-k3 F4 — The spec restates the ADR's observation contract, and the ADR carries spec-grain procedure (low)

Anchor: `docs/specs/item-status.md:26`–`:75` (the result table, "Finalize
activity only after the observation has pinned…" at `:44`, the guard-release
and no-creation rules) against `docs/adr/one-live-driver-per-working-tree.md:229`
–`:300` (the six-step observer protocol, "Pinning before the final probe is
load-bearing" at `:274`).

Evidence: `SPEC-FORMAT.md`'s grain rule says a spec cites the ADRs in its area
and never restates them, because two statements of one rule will disagree;
`ADR-FORMAT.md`'s minimal record is a decision, its trade-off and the rejected
alternative. Today the two texts agree, so this is structural rather than a
defect in either statement: the pin-before-probe rule, the Idle/Busy/Unavailable/
Running conditions, the release order and the no-configuration/no-creation
discovery rule each have two homes. The ADR has also grown from one decision to
two: the lease/epoch ownership decision and the witness-not-lease evidence
decision, the latter with its own four rejected options and its own reopen
conditions at `:302` onward. Whether that second decision earns its own record
is a judgement under `ADR-FORMAT.md`'s split test, and the integration may
reasonably reject that half.

Proposed classification: real, structural. Keep in the ADR the decision, its
trade-offs and rejected options; state the observer's numbered protocol and
result semantics once, in the spec's interfaces section, with the ADR citing it.

### item-status-k3 F5 — Worktree validation should be by identity, not by the observed spelling (low)

Anchor: `docs/specs/item-status.md:28` ("for an exact worktree location");
`docs/adr/one-live-driver-per-working-tree.md:250` (step 3, "validate the
worktree").

Evidence: the record's `worktree-path-hex` is the canonical root
(`crates/grove-loop/src/driver_lease.rs:302`, from `Workspace::resolve` at
`crates/jj-workspace/src/lib.rs:97`); `grove view` makes its argument absolute but
does not canonicalise (`docs/USAGE.md`, "The path is made absolute once");
admission compares canonical path and device/inode both
(`driver_lease.rs:764`–`:781`). On macOS a temporary tree is reachable as both
`/var/…` and `/private/var/…`, so byte comparison would make every application
test through the seam observe Unavailable, and an implementer would then choose
a rule the spec never stated.

Proposed classification: contract stated unclearly. Say that the observed
directory's device/inode is compared with the record's `worktree-device` and
`worktree-inode`, with the path spelling at most informative.

### item-status-k3 F6 — The driver's exclusive witness lock is not stated as nonblocking (low)

Anchor: `docs/adr/one-live-driver-per-working-tree.md:188` ("The driver locks
the empty regular file exclusively before publishing its identity").

Evidence: the name is unknowable before publication, but the namespace is a
listable directory and the ADR forbids only the viewer from locking anything but
the named witness; a foreign process that found the file by listing could hold
it shared. Every other acquisition in the protocol is `LOCK_NB`
(`crates/grove-loop/src/driver_lease.rs:395`, `:501`, `:657`); a blocking witness
lock would hang the driver pre-spawn behind such a holder with no bound.

Proposed classification: contract stated unclearly. State nonblocking, with
contention treated as observation-only allocation failure: activity
Unavailable, launch unaffected.

### item-status-k3 F7 — Help-overlay discoverability is a requirement the spec does not carry (low)

Anchor: `.grove/_BRIEF.md:21` ("make the active view and the switch key
discoverable in the screen chrome and help"); `docs/specs/item-status.md:166`
covers chrome, and `:183` mentions help only as remaining global.

Evidence: the current help text (`crates/grove-tui/src/lib.rs:514`) says "Tab:
switch pane", which the new interaction makes wrong. One sentence closes it.

Proposed classification: real, trivial trace gap.

### item-status-k3 F8 — "Emphasis" for the cursor and for NEXT is undefined, so the RUNNING-only-highlight scenario cannot be asserted on styles (low)

Anchor: `docs/specs/item-status.md:144` (RUNNING is bold yellow), `:146` ("NEXT
emphasizes its activity word"), `:149` ("Selection adds the cursor and
emphasis"); acceptance row "Only RUNNING has the active item-text highlight".

Evidence: the requirement (`.grove/_BRIEF.md:42`) wants cursor, expansion and
activity distinguishable with text alone. Bold is the only modifier the spec
names; if cursor emphasis and NEXT emphasis are also bold, a selected LIVE row
and the RUNNING row differ in monochrome only by the gutter and the activity
word. That is text-sufficient, so not a requirement failure, but a render test
inspecting styles cannot pin the scenario until the modifiers are named.

Proposed classification: contract stated unclearly. Name the modifiers per role.

## Decisions (running log)

Findings F1 to F3 are actionable and F4 to F8 are cheap to settle alongside
them, so an integration step is warranted. Its leaf is inserted ahead of the
waiting `item-status-k4` planning step, the first later sibling entry in this
directory with live work, so the integration opens the spec and ADR at the lines
cited here before planning reads them. The integration's body names this review
by handle and carries none of the findings, so rejecting one stays an ordinary
outcome. No in-session reviewer was spent; this session is the adversarial read.
