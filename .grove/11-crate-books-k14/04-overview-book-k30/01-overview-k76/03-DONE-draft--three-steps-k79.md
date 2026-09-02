# three-steps-k79

## Goal

Draft chapter 3 of the overview: slice `one-call`, `03-three-steps.md`, owning
`entry-point-three-steps` (`crates/grove/src/main.rs` 1–13) and
`surface-resolve-lease-run` (`crates/grove/src/cli.rs` 20–53).

## Context

- Draft stage, child 3 of 5 of `overview-k76`. Responsibilities are the
  structure brief's *3 · Three steps* section: the three things the loop cannot
  do for itself; the workspace resolved **once** here and handed to both the
  lease and the loop, and what that seam replaced (`loop-crate-driver-k22`,
  `docs/adr/one-live-driver-per-working-tree.md` — evidence, cited by path);
  the shape of one foreground iteration; and the signal path, which is the
  chapter's centre of gravity.
- The required example anchor is `worked-run`: the carried invocation at full
  resolution — `current_dir`, `Workspace::resolve`, `DriverLease::acquire`,
  `TemplateSource::from_env`, `grove_loop::run` — and both endings. `Finished`
  and `Stopped` reach `Ok(())`; `Interrupted(SIGTERM)` reaches `reraise`, and
  whoever started `grove` reads a wait status of `128 + 15`. The two error
  endings `run`'s doc comment names are the same trace stopping earlier.
  `docs/USAGE.md`'s *Stopping the loop* transcript shows `143`; the loop
  fixtures in `crates/grove/tests/loop_driver.rs` and `lifecycle_cutover.rs`
  are the tests that prove the reraise. Signal semantics are stated once, as
  the chapter's premise, with `128 + N` named rather than derived.
- This chapter **owns** all five early-use rows: flip each to `explained` and
  supply the full explanation at the site that reads it. `Workspace` is
  `jj-workspace`'s type re-exported at `crates/grove-loop/src/lib.rs`, and
  `reraise` is `keyed-launch`'s; name that without explaining either crate.
- Citations placed by the brief: `docs/USAGE.md#usage-driver-lease` beside
  `DriverLease::acquire`, `docs/USAGE.md#usage-session-lifecycle` beside
  `grove_loop::run`; glossary `driver-lease`, `stated-vcs` beside
  `Workspace::resolve`, and `loop-control-channel` beside the completion signal.
- `main.rs` is owned here rather than in chapter 1 because its module
  documentation is the three-steps argument in miniature (brief, *The mapping
  onto the corpus*).
- Leave the page shaped to receive *Runtime flow* at `architecture-move-k31`;
  move nothing.

## Done when

- Both blocks' fragments are defined on the page, both defers are replaced by
  inserts, both ownership rows read `resolved`, the fragment index has the rows,
  and all five early-use rows read `explained`.
- Contents, navigation and the concept index are updated.
- `book-check --through one-call --check all` is valid: 120 resolved lines, 84
  deferred. The repository Markdown sweep passes. `scripts/check.sh` stays red
  on `book-check` alone, by design.

## Decisions (running log)

**1 · The block is partitioned into five literals along the doc comment's own
paragraphs and the body's own seam.** `run-seam-doc` (20–28, the blank line
and the workspace-resolved-once paragraph), `run-signal-doc` (29–37, the
killed-driver paragraph), `run-errors-doc` (38–41, `# Errors`),
`run-three-steps` (42–47, the signature through `TemplateSource::from_env`)
and `run-call-and-endings` (48–53, the `match`, the closing brace and the
trailing blank line). The body splits at the `match` because the chapter's
argument does: three steps, then the signal path. The trailing blank at line
53 stays inside this block because the block boundary is fixed at 53 and blank
lines lead the fragment that follows them everywhere else in the book; the
page says so. `main.rs` is two literals, the module documentation (1–7) and
the code (8–13), so the comment can sit beside the counting argument and the
code beside `cli::run`.

**2 · Reader order is argument order, not file order.** The seam paragraph,
then the three steps, then the signal paragraph with the `match`, then the
worked example, then the `# Errors` paragraph, then the iteration shape. The
`# Errors` fragment is read after the worked example because the two refusals
it names are that trace stopping earlier, which the brief places in the
example. The iteration shape comes last so that no section before `worked-run`
enumerates the loop's operations.

**3 · `TemplateSource` is read twice per iteration, and the ledger now says
so.** Verified against `crates/grove-loop/src/loop_driver.rs`: `templates.load`
at the pre-transition line and again after selection. The manifest statement,
the ledger row and the structure brief's early-use table were amended in this
commit to *the loop re-reads it every iteration — twice, before and after the
tree transition — rather than holding a copy*; chapter 2's count-neutral table
row stands. The type's own doc comment in `session_config.rs` says *once* and
is another book's corpus, so `template-source-read-count` is cut beside k82,
k83 and k84, ahead of `architecture-residue-k75`, under the root brief's
cross-book rule.

**4 · The three counts of three are stated, not reconciled.** `main.rs` counts
parse, resolve, lease; `run`'s title counts resolve, lease, run; the body has
five statements before the `match`, and neither comment counts
`TemplateSource::from_env`. The page states the structural fact — four things
happen before the loop is entered, and locating the configuration is handed to
the loop as a value so a fixture can substitute a home — and reproduces both
comments as written.

**5 · Both endings and both refusals are measured, not rendered from
convention.** The built binary at `20.1.0` was run from a scratch directory
that is not a Jujutsu workspace (exit `1`, the seam's refusal text), from a
scratch `jj git init` tree with `$HOME` unset (exit `1`, the `$HOME` message),
and as a second driver against a tree whose first driver was live (exit `1`,
the lease refusal naming the canonical root). The first driver, run as a
direct child with a stub session that never signals, was sent `SIGTERM` and
`SIGHUP` in two runs and reported wait statuses `143` and `129`, after
printing *interrupted by signal 15* and *interrupted by signal 1*. A stub
session killed with `SIGTERM` instead produced *session ended without a
completion signal — status signal: 15 (SIGTERM)*, `Stopped`, and exit `0`,
which the page uses to fix what *the driver, not the session* means. One
false measurement was discarded on inspection: a first harness backgrounded a
`cd && env … grove` list, so the recorded PID was a bash subshell's and the
signal never reached the driver; the harness was corrected before any number
was written.

**6 · The task file's claim that `lifecycle_cutover.rs` proves the reraise is
not borne out, and the page cites only the fixture that does.** That file
contains no signal assertion; `a_sigtermed_driver_stops_and_reaps_its_child`
in `crates/grove/tests/loop_driver.rs` asserts `status.signal() ==
Some(SIGTERM)` and `status.code() == None`. No fixture asserts the `SIGHUP`
path; the page says so and shows it measured.

**7 · `driver-lease` is cited on this page as well as in chapter 1**, per the
node brief's decision 7, at the `DriverLease::acquire` paragraph beside the
guide link the brief places there. `stated-vcs` is cited beside
`Workspace::resolve` and `loop-control-channel` beside the completion signal in
the `LoopOutcome` paragraph, as placed.

**8 · The page is shaped to receive *Runtime flow* and moves nothing.** The
last section, `one-iteration`, states the loop's shape from the caller's side
in one paragraph and names the modules chapter 5 will map; the diagram, the
`--help`/`--version` sentence and the foreground sentence stay in
`docs/ARCHITECTURE.md` for `architecture-move-k31`. The page's own sentences do
not restate those three.

**9 · Proof.** `book-check --through one-call --check all` is valid: 3 files,
120 resolved lines, 84 deferred, `final=false`. `reference_navigation` (5
tests) and `corpus_exception_inventory` (12 tests) pass.

**10 · The one in-session review was spent, and it paid.** A fresh context was
given the page and every source it draws on with a *disprove it* brief and
returned nineteen findings; every one was classified. Twelve were wrong or
imprecise claims and are fixed: `Workspace::resolve` asks `jj` for a secondary
workspace's main repository, so *no repository discovery* is true of the walk
only; the fixtures set `$HOME` rather than substituting a `TemplateSource`; the
pre-transition load gates only the `finish` leaf, not every leaf the loop
writes; the terminal is restored by the loop, not the runner; the lock is on
the lease file and the root descriptor is held for revalidation; the no-signal
report is two lines when the status is not success; the loop takes the tree
root from the lease and the main repository from the workspace; the fixture
also asserts the printed line, and the reap-drop-die order is by inspection;
the lease record is written once at `acquire`, not per iteration; the
`exit(128 + N)` fallthrough is a defect guard and is labelled as one; the
loop's error list mixes refusals with environmental failures and the page now
says which is which; and the trailing blank at line 53 is stated as the one
exception to the blank-line convention rather than derived from it. Two were
contract findings the draft owns and are fixed: both `main.rs` fragment
introductions now state their role in the carried invocation, and `run` is a
symbol, not a *type family*. One reached across pages: chapter 1 said the
fixtures send `SIGTERM` and `SIGHUP`; only `SIGTERM` is sent, and chapter 1's
sentence is narrowed in this commit under the draft's technical-truth charter.
One is a visible trade-off, recorded rather than fixed: the iteration section
explains more of the loop than *named, not explained* strictly allows, because
the structure brief charters this chapter with *the shape of one foreground
iteration*; its inaccurate reads-and-writes claims were removed and the shape
now includes the second revalidation and the epoch. Five style findings —
figurative wording — are copy-edit's charter and are under `## Handed forward`
in `overview-book-k30`'s brief. One finding the reviewer raised about the ADR
was checked and is noise: `docs/adr/one-live-driver-per-working-tree.md` says
the seam *does not follow* a secondary workspace's repository link, and the
code does not — it asks `jj` instead, as the guide says.

**11 · Proof, after the review's fixes.** `book-check --through one-call
--check all` is valid: 3 files, 120 resolved lines, 84 deferred,
`final=false`. `reference_navigation` (5) and `corpus_exception_inventory`
(12) pass. `bash scripts/check.sh` was run under a ten-minute limit and cut
off inside its `cargo test` stage at 9m50s, with `cargo fmt`, `shellcheck`,
`cargo clippy`, `plugin install`, both conformance stages and every test that
finished green, and the `book-check` stage not reached in that run; the loop
fixtures in `crates/grove/tests/loop_driver.rs` are the slow stage, as
`the-surface-k78` also recorded. Run directly, the script's `--final` check
over this book reports only what the shape predicts: the deferred
`surface-closure-tests` block, and the missing chapters 4 and 5 with the
navigation and contents links they will bring. `scripts/check.sh` is red on
`book-check` alone, by design, until `what-the-call-reaches-k81`.
