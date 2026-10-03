# harness-wrapper-k3

**Reviews:** harness-wrapper-k2
**Creator:** run 7b883bd2-52c6-4b91-b5c4-6dda307b40c7

## Goal

Adversarially read the supervised-dispatch design `harness-wrapper-k2`
produced — the reworked area specification, the ADR set and the specs that must
agree with it — against the root brief's settled requirements and guarantees,
and report findings. Fix nothing.

## Context

- The artifact: `harness-wrapper-k2`'s commit. Read its diff against its parent.
  The load-bearing files are `docs/specs/harness-selection-and-execution.md`
  (*Execution and authority*, *Supervision*, *Confinement*, *Grove
  integration*, *Records*, *Diagnostics*, *Agreed test seams*, *Out of scope*),
  the new `docs/adr/dispatch-supervises-the-harness.md`, the renamed
  `docs/adr/policy-evaluation-precedes-the-launch.md`, the edits to
  `the-launched-child-is-a-job`, `harness-selection-is-owned-by-policy`,
  `one-live-driver-per-working-tree` and `a-review-carries-its-creator-reference`,
  decisions 7 and 9 of `docs/specs/module-decomposition.md`,
  `docs/specs/standalone-invocations.md`, `docs/specs/item-status.md`, and the
  glossary's new and changed entries.
- `harness-wrapper-k2`'s running log (`W1`–`W13`) states each design decision
  with the alternatives it rejected; `plan-k1`'s (`D1`–`D9`) holds the owner's
  requirements in their own words.
- The visual document is under `docs/design/harness-selection-and-execution/`
  (`task design:harness-selection`).

## Done when

- Every finding is recorded with its location and why it matters, or the
  review records that it found none.
- If findings warrant action, an `integrate-review-design` leaf is
  `leaf-insert`ed ahead of the planning leaf, so planning reads an agreed design.

## Notes

The producer's own doubts, each a place it was least sure:

- **The launch carries two flags beyond D1's list.** D1 says a lifecycle launch
  carries the kind, the task file, the task identity and the prompt; the design
  adds `--exit-dir` and `--ending-file` as run mechanics that reach no policy
  (W3, W5). Is that within design's remit, or a change to D1 that should go back
  to the owner? Is there a design that needs neither?
- **Entry signal state through two runners.** The spec promises the harness the
  caller's entry mask and dispositions, while `keyed-launch` resets the
  terminal-generated signals to default in its child and installs TERM/HUP
  handlers. Decision 7 says a transparent caller passes its own entry state and
  the runner installs no handler over an ignored disposition. Is that coherent,
  implementable, and enough to keep a `nohup` caller's HUP ignored through
  dispatch?
- **Cancellation and grace arithmetic.** Grove waits 10 s for dispatch;
  dispatch's interactive cancellation forwards and kills after 5 s; a confined
  run is killed at once; the end observation can wait up to 2 s on the store
  lock. Check the nested `grove run`-inside-a-session case and the driver's own
  TERM against these bounds.
- **Dispatch's death.** The design accepts an orphaned harness and stops the
  loop. Is the terminal reliably given back to the driver when the orphan's
  group still holds it, and is "never relaunch" enough given that a teardown
  record still finishes?
- **The run ending and exit status.** Cancellation takes precedence over the
  exit signal; the exit-signal ending exits 0 unless the harness failed on its
  own. Does `grove run`'s publication rule still equal today's (token `done` and
  escalated-or-success)? Is reproducing a core-dumping signal without a core
  well-defined?
- **The teardown record.** `record-teardown` refuses while `.grove/` exists.
  Does any legitimate finish sequence, this repository's CLAUDE.md one included,
  hit that refusal? Is a launch-scoped record consistent with the
  one-live-driver record's rejected finish tombstone?
- **Confinement.** The confined harness must execute dispatch's own executable
  to send the exit signal, on macOS and under bubblewrap, and gets a minimal
  environment from dispatch rather than Grove. Is anything the old staging
  provided (the copied `grove-llm`, `root/tmp`, the control directory) lost
  without a replacement?
- **The ADR set.** Is splitting the old process-replacement record into the
  renamed worker record and the new supervision record the minimum coherent set,
  and is every citation of the old slug reconciled?
- **The seams.** Does each guarantee in the root brief map to a case in one of
  the four agreed seams, and does the first seam really absorb the runner's
  interface cases?

## Findings

Reviewed `harness-wrapper-k2`, commit `f653dd72`, against its parent
`a77b89b6`, the settled requirements and the current source. These are design
findings, not assertions that the proposed implementation has already failed.
The design needs integration before planning.

### F1 — P1: leader reap is not completion of group cleanup

**Location:** `docs/specs/harness-selection-and-execution.md:767–795`,
`docs/specs/module-decomposition.md:424–435`, and
`docs/design/harness-selection-and-execution/ending.mmd:11–15`.

The escalation promises that descendants die with the harness, but the ending
is selected once the harness is reaped, and the diagram goes straight from
*Terminating* to *Reaped* when the harness exits. Consider a harness that sends
the exit signal, then exits on TERM while its ordinary, same-group tool ignores
TERM. The leader has ended; the tool has not. Ending supervision at that point
skips the later group KILL and allows an `exit_signal` ending to relaunch Grove
beside a surviving tool, possibly one holding shared epoch admission. The
same distinction matters for a confined harness that acknowledges and exits
0 while leaving a writer behind: publication needs the group stopped first.

This is not about deliberately detached processes. The current interactive
`watch` returns on leader reap (`crates/keyed-launch/src/run.rs:771–783`),
whereas the detached path explicitly observes without reaping, kills the
remaining group while its identity is reserved, and drains it
(`Child::try_wait`, lines 645–669; `drain_group`, lines 614–635). Merely moving
the current interactive runner under dispatch does not establish the promise.

**Smallest useful correction:** give group cleanup its own completion condition
and safe identity lifetime, before a successful ending or publication. State
how it handles leader exit during grace, after TERM, and on ordinary exit;
preserve the leader's status independently. Add command-seam cases where the
leader exits on TERM but a descendant ignores it, and a confinement-seam case
where an acknowledged clean leader leaves a writer. The existing case with a
TERM-ignoring leader does not distinguish these behaviors.

### F2 — P1: permitted confinement roots can contain the files it promises to hide

**Location:** `docs/specs/harness-selection-and-execution.md:830–850`.

The new public `run --confine` accepts every directory except `/` as its cwd,
grants that directory and the exit directory, and accepts any regular
`--runtime-read` file, while promising that the policy, settings and store stay
outside. Those conditions conflict on valid inputs. Running from HOME puts the
default policy, settings and store beneath the writable cwd. An explicit
`--config ./policy.ts` inside an ordinary project has the same problem without
using HOME. A state directory or an exit directory can overlap too.

Grove's staged directory avoids the common case, but dispatch is now the
generic confinement boundary for every caller. The existing backends admit
whole roots (`crates/keyed-launch/src/confinement.rs:134–144,196–201`); no
separate protected-path rule is specified. This exposes owner execution data
and can grant writes as well as reads, contrary to the root brief's guarantee.

**Smallest useful correction:** specify which rule wins when a writable root,
runtime grant or runtime resource overlaps the selected policy, owner settings
or record store, including canonical aliases. Refuse incompatible roots or
define effective exclusions that the supported backends can enforce. Add a
dispatch confinement case with a policy inside cwd and a protected store
inside a granted root; testing only Grove's separated staging does not cover
the public contract.

### F3 — P2: terminal recovery on dispatch death needs a rule beyond direct-child reclaim

**Location:** `docs/specs/harness-selection-and-execution.md:820–825,1215–1226`
and `docs/adr/the-launched-child-is-a-job.md:3–19`.

The orphan trade-off is explicit, but its terminal recovery is not resolved.
While harness group H owns the terminal, kill dispatch group D's leader. The
driver reaps D, but the foreground group is still H. The runner's existing
reclaim guard accepts only its direct child's pgid
(`crates/keyed-launch/src/run.rs:563–570`), and the driver's existing reset does
nothing unless the driver already owns the foreground
(`crates/grove-loop/src/loop_driver.rs:500–506`). Thus the specified nested job
chain cannot rely on taking the terminal back "as it always has": neither
guard admits this accepted death case. The driver can stop correctly while
the surviving raw-mode harness continues consuming the user's input.

**Smallest useful correction:** state the launch-scoped authority that lets the
outer runner reclaim and restore a terminal delegated beyond its direct child,
including dispatch's death, while preserving the rule against stealing a
terminal it never owned. This need not give Grove the harness's escalation.
Require the Grove PTY seam's dispatch-death case to observe foreground ownership
and restored modes with the orphan still alive, alongside a background launch
that must not take the foreground. The current acceptance row checks stopping
and a live leaf, not recovery in this case. POSIX permits background reclaim
with SIGTTOU blocked or ignored; it does not reclaim on behalf of a dead
supervisor ([tcsetpgrp](https://pubs.opengroup.org/onlinepubs/9699919799/functions/tcsetpgrp.html),
[terminal interface](https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap11.html)).

### F4 — P2: automatic end observations contradict the schema-1 migration rule

**Location:** `docs/specs/harness-selection-and-execution.md:1050–1054,1107–1113`.

The records contract still says that only `record observe` migrates a schema-1
store and that every other command writes its existing version. A supervised
`run` must now append an observation automatically. On an upgraded owner's
schema-1 store, that table does not exist. An implementation either violates
the migration restriction by using the shared append operation, or predictably
fails to record every supervised ending until the owner imports an observation
by hand. Best-effort failure reporting does not resolve an incompatible normal
upgrade path. The existing `append_observation` already performs the additive
migration in its transaction (`crates/harness-dispatch/src/store.rs:316–331`).

**Smallest useful correction:** make the migration contract admit dispatch's
own end append, with its transactional failure semantics, and reconcile the
roll-up statements. Require the record seam to supervise a run against an
existing schema-1 store and show its end observation and execution confirmation,
without an intervening manual import.

### F5 — P2: spawning does not transfer dispatch's pending blocked signals

**Location:** `docs/specs/harness-selection-and-execution.md:696–713`.

The spec correctly preserves the entry mask and ignored dispositions, but also
says a caller-blocked signal reaches the harness pending. That followed from
the old same-process exec handoff; it does not follow from the new spawn.
For example, start dispatch with SIGUSR1 blocked and send SIGUSR1 to dispatch
while policy selects. It remains pending in dispatch. The forked harness gets
the blocked mask and an empty pending set; reinstating dispositions and the
mask does not copy the pending signal. This is distinct from an unblocked
handled cancellation after the linearization point, which the retained parent
handler can forward. POSIX explicitly initializes the child's pending set
empty ([fork](https://pubs.opengroup.org/onlinepubs/007904875/functions/fork.html)).

**Smallest useful correction:** distinguish inherited mask/dispositions from
pending-signal ownership and settle the latter's contract. Either explicitly
accept the spawn semantics, reconciling the transparency claim, or specify an
intentional forwarding mechanism and its limits. Add a command-seam case that
targets a blocked signal at dispatch before spawn; checking only that the
harness's mask contains it cannot verify the pending-signal promise.

## Review bounds and resolved questions

- The selection inputs remain the four the owner settled. The additional
  `--exit-dir` and `--ending-file` flags are generic run mechanics hidden from
  policy; they do not reintroduce a selection parameter or jj knowledge into
  dispatch. No owner escalation is warranted on that point.
- The worker-runtime decision and the supervision decision are independently
  reversible responsibilities. Splitting them earns the new ADR; no further
  record is needed merely for a channel or ending-file format. The linked
  replacement citations inspected in the changed artifacts agree.
- Cancellation precedence, teardown precedence, launch-directory epoch
  admission, and publication on `exit_signal` plus dispatch exit 0 are coherent
  as stated. `record-teardown` after `finish-commit` accommodates this
  repository's integration/release-before-final-signal sequence. The launch
  record is ephemeral, so it does not resurrect a finish tombstone.
- Reproducing signal death without a second core dump is a defined supervisor
  behavior, also specified for POSIX
  [timeout](https://pubs.opengroup.org/onlinepubs/9799919799/utilities/timeout.html).
  Fixed grace arithmetic leaves room for the stated kill-grace and store-lock
  wait; it is not a bound on arbitrary filesystem or transcript I/O.
- The four seams are suitable public enabling points. They need the cases in
  the findings to distinguish the promises at risk; useful internal runner
  coverage remains supporting evidence. No new public seam is proposed.

Tier 2 graph verification used the index built for this workspace at generation
`2026-10-03T09:17:43Z`. Coverage was checked for the evidence paths: the relied-on
Rust files had no recorded gap and matched metadata; `docs/` was excluded, so
the specifications, ADRs and diagrams were read directly. Graph traces were
used for discovery, with exact source used for behavioral claims. No exhaustive
repository-wide absence claim is made. No test, build, lint or format command
was run, and no production or test code was changed.

## Decisions (running log)

**R1 — integrate before planning.** F1–F5 warrant triage in a fresh
`integrate-review-design` session. Insert it immediately before
`supervised-dispatch-k4`; its charter names this review rather than restating
the findings as mandatory fixes. This review makes no design changes.
