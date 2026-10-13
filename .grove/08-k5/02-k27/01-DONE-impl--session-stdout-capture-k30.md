# session-stdout-capture-k30


## Goal
Capture a supervised child's stdout separately from terminal/log stderr, without
changing foreground, cancellation, descriptor or reap behavior.



## Context
Ravel 0.1.0 returns the opaque result only on stdout. Grove's interactive and
standalone runners currently inherit both streams or merge both into a log.
Expand keyed-launch's API beside its existing entry points; consumers migrate
in grove-ravel-consumers-k31.

## Done when
- New interactive-observed and noninteractive entry points accept a caller-owned
  regular stdout file. Interactive stdin/stderr and terminal lending remain;
  noninteractive stdin is EOF, stderr goes to its log and the job is detached.
- Tests distinguish stdout from stderr, verify exact Started/Reaped events,
  terminal foreground/raw-mode recovery, cancellation and group cleanup under
  capture. Invalid output descriptors refuse before spawn.
- Existing APIs keep their behavior, all keyed-launch nextest checks and relevant
  doctests pass, and the source-pinned keyed-launch book reconstructs exactly.
- A reusable root Taskfile task runs this suite with nextest. No installation or
  active Grove consumer changes occur.

## Notes
Inline plan: add failing process/PTY tests using existing subprocess roles;
add the two capture entry points over the shared runner; update exact book
fragments/ranges/index and explanatory prose; freeze inputs and run focused
then full keyed-launch, formatting/lint and book checks; retire and seal.

## Decisions (running log)

Decompose k27 at the missing stdout transport seam rather than separating the
two consumer migrations. The existing launch structs remain source-compatible;
the new entry points accept owned Files and never interpret captured bytes.
The public functions will be `run_observed_with_stdout(Launch, File, observer)`
and `run_noninteractive_with_stdout(NoninteractiveLaunch, log: File, stdout: File)`.
The process tests contest stream routing and the PTY test contests the crucial
claim that stdout redirection must not turn an interactive launch into a
detached launch. These executable seams supply disproof; no reviewer is needed
for this expansion.

Keep output routing independent of process mode. A separate stdout File only
replaces fd 1, after ordinary log setup; it never determines detachment. Capture
Files are checked before spawn and retained at the caller's current offset.
The existing APIs delegate with no capture, while the two new APIs reuse the
same spawn/watch/reap/terminal paths. No new dependency or caller semantics is
introduced.

The capture tests first failed against capture-free delegates, then passed with
the new routing. Existing subprocess/PTY roles cover mode, signal and group
behavior; integration tests remain outside the book's production corpus. The
book retains its declared roots and ownership, with exact ranges, literals,
indexes and related prose updated together.

## Verified handoff

- `task test:keyed-launch`: all 64 nextest tests passed; the 12 ignored roles
  are driven by their owning tests. Doctest check passed (no examples).
  Focused stream/descriptor/PTY checks were observed failing before routing
  was implemented, then passing; capture cancellation/kill-grace and orphan
  helper checks passed too.
- `cargo fmt --all --check` and
  `cargo clippy --locked -p keyed-launch -p grove-loop -p grove --all-targets`
  passed. Existing downstream consumers compile against the unchanged APIs.
- Final keyed-launch `book-check --check all` reconstructs seven roots,
  2,357 lines, and no deferred source. Root/range/fragment/index validation and
  Markdown checks passed. Capture preserves the three process modes; the
  source comment and its explanation now name modes rather than API counts.
- Full logged handoff and complete focused logs are in
  `target/session-stdout-capture/`: `handoff-nextest.log`, `red.log`,
  `green.log`, `capture.log`, `fmt.log`, `clippy.log`, `book.log`.
  `initial-results.json`, `handoff-results.json` and `handoff-summary.json`
  compare each test identity/outcome with the initial UI run, not totals alone.
  The logged handoff followed the final process-mode comment correction.
- All 1,991 individually frozen inputs matched before/after that final run,
  including repository inputs, the complete original Grove 22.1.0 and Ravel
  0.1.0 installed kegs and owner Ravel configuration. Records are
  `subjects-before.json` and `subjects-after.json`. These describe inputs held
  fixed during verification; this terminal handoff is written afterwards.
- Ravel's checkout remains clean at the empty successor of 3916a4a1; the tap
  remains clean. No installation, release, owner-policy or live-driver ending
  channel changed.

This child meets its expansion contract. grove-ravel-runtime-k27 remains live
through grove-ravel-consumers-k31; both actual consumers still use the original
launch/report path. k31 owns their atomic migration and installed-pair runtime
fixtures. k28/k29 and the broader delivery/release work remain live.
