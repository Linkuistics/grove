# idle-next-k22

## Goal
Adversarially review the shipped typed tree/runtime observer against
idle-next-k11 and typed-observation-k18, using producers captured-tree-k20 and
bounded-runtime-k21 as the authoritative implementation artifacts.

## Context
Read docs/specs/item-status.md, the ancestor contracts and both producer commits.
The observation operation is consumed by real browsing. k19 separately owns
visible activity and independent two-capture runtime acceptance; inspect that
increment too if it has landed before this review runs. Witnessed RUNNING belongs
to witnessed-activity-k12 and is not claimed by legacy-active compatibility.

## Done when
- Verify independent tree/activity results, exact namespace isolation and aliases,
  mandatory parser reuse, Idle/Busy/Unavailable classification and no lease probe.
- Attack descriptor/path races, including workspace-alias changes around discovery,
  record replacement before/after lock and read, wrong types, unreadable controls,
  the 64 KiB bound and eight-attempt bound. Check nonblocking/read-only/CLOEXEC
  acquisition and filesystem preservation, including administration bytes.
- Verify tree capture precedes the runtime guard and no advisory guard escapes;
  assess the after-capture and in-epoch pause controls, concurrent shared readers,
  30-second deterministic handoff bound, recovery and stale admission tests.
- Check usage/seam prose and source-derived books against the shipped behavior.
  Principal checks are evidence, not substitutes for an adversarial read.
- Record findings with source anchors and severity. Create adjacent
  integrate-review-impl with bare stem idle-next only for actionable findings;
  its brief references this review rather than transcribing findings.

## Notes
No competing in-session reviewer ran. The graph generation predates these
producers; refresh coverage or use current source for changed/new modules.

## Findings

Reviewed the three producer commits as they stand at the working copy's parent
`1316c1275ce9` (`idle-activity-view-k19`), with no working-copy changes:
`c5fc88ee9f6e` (`captured-tree-k20`), `f51d024ede92` (`bounded-runtime-k21`)
and `1316c1275ce9` itself. Every `path:line` below is read from the files at
that parent. Read against `docs/specs/item-status.md`, the driver-lease ADR, the
`idle-next-k11` and `typed-observation-k18` briefs and the three producer
bodies. Source was read directly; no graph-coverage claim is made.

### What held

- **Ordering and guard lifetime.** `observe_with` runs `capture` to completion,
  then the `after_capture` hook, then the runtime read
  (`crates/grove-loop/src/observation.rs:121`–`:124`). The tree guard is
  consumed by `into_snapshot` inside `capture` (`:172`), so no tree lock and no
  epoch lock coexist. `ObservationGuard` carries only copied rows/bytes and an
  unlocked directory descriptor (`:15`, `:103`–`:106`). The pause controls
  prove both: an exclusive tree lock and a driver epoch activation succeed
  during the after-capture pause, and after return the driver invalidates the
  epoch (`crates/grove-loop/src/driver_lease/observation.rs:318`–`:357`).
- **Classification.** Missing worktree, `.jj`, namespace or lease → Idle
  (`driver_lease/observation.rs:17`–`:29`, `:36`–`:41`); lease present with
  missing/unreadable/wrong-type/oversized/malformed/mismatched epoch →
  Unavailable via `?` and the guarded closure (`:44`, `:69`–`:78`); shared
  epoch contention → Busy (`:55`–`:64`); matching inactive → Idle, matching
  active → Unavailable (`:79`–`:85`). No `flock` is ever issued on the lease:
  the file's single `flock` is on the epoch (`:45`). Admission is untouched;
  `bounded-runtime-k21` only extracted `parse_epoch_record`
  (`crates/grove-loop/src/driver_lease.rs:617`–`:640`).
- **Descriptor discipline.** `open` is read-only, `O_NONBLOCK|O_CLOEXEC`,
  `O_DIRECTORY` for directories, and rejects the wrong type by `fstat`
  (`:104`–`:125`); records are bounded by a 64 KiB + 1 read (`:136`–`:144`);
  identities are compared after the lock and again after the copy, on all four
  descriptors (`:48`–`:54`, `:87`–`:93`); the loop is bounded by
  `IDENTITY_RETRY_LIMIT` with no sleep (`:30`, `:95`). Aliases compare the
  pinned worktree descriptor's dev/inode with the record (`:75`–`:78`); a
  subdirectory discovers its own `.jj` only
  (`crates/jj-workspace/src/lib.rs:156`–`:167`). The worktree is pinned before
  discovery and re-checked after it (`:17`–`:26`), which closes the
  alias-retarget window around discovery.
- **Viewer acceptance.** Activity is compared independently of rows, bytes,
  selection and lifetime (`crates/grove-tui/src/observation.rs:65`–`:75`);
  a changed activity yields Busy without rejecting the tree. `refresh_at`
  resets NEXT and activity before every observation, so Busy, Unavailable,
  failure and root replacement all clear row activity
  (`crates/grove-tui/src/lib.rs:417`–`:418`, `:504`–`:512`). NEXT is the
  loop's `select_snapshot` without exclusion, shown only for accepted Idle
  (`:504`–`:508`); the row cue is `BOLD` with no foreground (`:103`–`:107`);
  the header is four lines (`:659`), leaving three bordered rows at 60 × 10.
  Chrome text passes through `fit_text`, which applies `safe_text` (`:113`–`:114`),
  so the dropped explicit `safe_text` on the location line is not a regression.
- **Tests.** Real-lease idle → legacy-active → contention → recovery with a
  stale-admission control (`driver_lease/observation.rs:171`–`:201`); the
  malformed/oversized/invalid-UTF-8/missing/directory/FIFO matrix (`:203`–`:257`);
  fcntl flag checks and both sides of 64 KiB (`:288`–`:315`); the eight-attempt
  bound with attempts counted (`:409`–`:438`); concurrent shared readers and the
  deterministic 30 s handoff bound (`:359`–`:408`); administration-byte
  snapshots (`:455`–`:482`). Viewer tests exercise the real operation through
  hand-written legacy records, both views, help, undersized frames, aliases,
  subdirectories, folded NEXT, finish-only NEXT and no-allocation
  (`crates/grove-tui/tests/browser.rs:55`–`:244`).
- **Prose.** USAGE's quoted diagnostic matches `epoch_contention_diagnostic`
  (`driver_lease.rs:353`–`:358`); the 30 s bound and restart guidance match the
  ADR; ARCHITECTURE, module-decomposition, CONTEXT-MAP and G6 describe the
  shipped classification and the deferred witnessed increment accurately. The
  book fragments' line ranges cover the whole of both observation files
  (`1-177`, `1-504`) and their prose matches the code they explain.

### idle-next-k22 F1 — a tree error discards the independent activity result, and a missing root never samples it (low)

Anchor: `crates/grove-tui/src/observation.rs:55`, `:60`;
`crates/grove-tui/src/lib.rs:417`–`:426`, `:517`;
`crates/grove-tui/tests/browser.rs:173`–`:178`;
`docs/specs/item-status.md:33`–`:34`, `:64`–`:66`.

Evidence: `capture_with` applies `?` to `first_sample.tree` before looking at
`first_sample.activity`, so a malformed tree returns `Err` and the runtime
result is dropped. `refresh_at` had already set activity to
`Unavailable("tree unavailable")`, so the RUNNING line reads
`RUNNING: unavailable — tree unavailable` although the observer established
Idle; the test at `:177` pins that text. When `sync_root` fails or the root is
absent, `refresh_at` returns before `capture` runs at all, so runtime is never
sampled. The spec makes independence symmetric — "tree failure does not
discard a verifiable mandate" — and states that on a failing tree "a fresh
mandate may still appear in the summary with `tree unavailable`". Today the
only visible cost is a freshness qualifier on the RUNNING line that names the
tree rather than the runtime; but this is the seam `witnessed-activity-k12`
must carry the mandate summary through, and the structure drops it.

Proposed classification: real, small. Return the sampled activity beside the
tree error (and sample runtime on the missing-root path), keep NEXT and row
attachments withheld, and let the RUNNING line show `none (idle)` beside a
`NEXT: unavailable — tree unavailable`. The producer's documented choice to
clear old activity is unaffected. Reject if the integration prefers to leave
this to k12, which then owns the change explicitly.

### idle-next-k22 F2 — the Unavailable reason for an active legacy epoch names a session that may not exist (low)

Anchor: `crates/grove-loop/src/driver_lease/observation.rs:82`–`:84`;
`crates/grove-tui/src/lib.rs:767`–`:770`.

Evidence: the same record is left behind by a driver that died mid-session,
and the observer cannot tell that from a live older driver. The chrome then
prints `RUNNING: unavailable — active session has no supported observation
witness`, which reads as an assertion that a session is active. The spec's
Unavailable row is "required evidence cannot be established".

Proposed classification: wording, trivial. Say what is known — an active
epoch record with no supported witness — without asserting a live session.

### idle-next-k22 F3 — two replacement windows and the alias-retarget window are handled in code but not exercised (low, coverage)

Anchor: `crates/grove-loop/src/driver_lease/observation.rs:22`–`:26`,
`:44`–`:54`, `:65`, `:87`–`:93`, `:409`–`:438`.

Evidence: the only injection point is `in_epoch`, called after the lock and
its identity check and before the copy. Replacement between open and `flock`
(`:44`–`:45`), replacement between the copy and the second identity check
(`:86`–`:87`), and an alias retargeted between the pin and discovery
(`:17`–`:26`) are each caught by the checks that follow them, by reading, but
no test drives them; the eight-attempt test covers only the guarded window.
The k22 charter asked for these attacks, so the gap is recorded here.

Proposed classification: visible trade-off unless cheap. Two more private
hooks would make the windows deterministic; otherwise accept and keep the
code-reading argument in the book's "One bounded runtime sample" paragraph,
which already states the order of checks.

### idle-next-k22 F4 — the forecast is computed twice per capture (info)

Anchor: `crates/grove-loop/src/observation.rs:140`;
`crates/grove-tui/src/observation.rs:85`–`:86`.

Evidence: `capture` calls `select_snapshot` to validate and discards the
selection; `display_capture` calls it again on the same snapshot to obtain
NEXT. Results agree by construction, and the viewer still owns no pick rule.

Proposed classification: noise for correctness; a simplification if the
loop's `CapturedTree` carried its selection, which would also give k12 one
place to apply exclusion. No action required.

## Decisions (running log)

- No competing in-session reviewer was spent; this session is the adversarial
  read. No test, build, lint or format command was run; the producers' recorded
  principal-gate results are their evidence, and the reading above is this
  review's. Source was read directly, not through the graph, whose generation
  predates these commits.
- F1 is actionable and small, F2 is trivial beside it, and F3/F4 are recorded
  for the integration to accept or reject; an integrate step is warranted.
  The review's own directory has no later sibling entry holding live work
  (the `witnessed-activity-k12` node is a sibling of an ancestor, which pre-order
  reaches only after this directory finishes), so plain `leaf-add` at the end
  of `14-k11` places the integration where `pick` reaches it next. Its body
  names this review by handle and carries no findings.
- The negative claims above — no `flock` on the lease, no epoch guard held
  across capture, no runtime sample on the missing-root path — rest on reading
  the whole of the two observation files and `refresh_at`, not on a pattern
  sweep.
