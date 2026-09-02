# the-surface-k78

## Goal

Draft chapter 2 of the overview: slice `no-arguments`, `02-the-surface.md`,
owning `surface-grammar` — `crates/grove/src/cli.rs` lines 1–19.

## Context

- Draft stage, child 2 of 5 of `overview-k76`. Responsibilities are the
  structure brief's *2 · The surface* section: the audience split between a
  human binary and an agent binary; the human grammar having nothing left to
  select and why; the twelve `grove-llm` verbs as a flat surface; and one
  version constant read by both binaries. Read the brief's *What the book
  deliberately does not cover*: `crates/grove-llm/src/cli.rs` is another book's
  corpus and no byte of it appears here.
- The required example anchor is `worked-argv`: the carried invocation's argv
  — `grove` alone; `grove --help` and `grove --version` stopping before the
  flow, discovering no repository and acquiring no lease; and
  `grove --harness claude` refused, with the text clap renders. **Measure the
  refusal text by running the built binary** (`cargo run --quiet -p grove --
  --harness claude` in a scratch directory) rather than reconstructing it from
  clap's conventions; do the same for `--help` and `--version`, and compare
  against the transcripts in `docs/USAGE.md`.
- **This page must carry `<a id="the-imports"></a>` on the section that reads
  lines 1–2.** Four early-use rows in the manifest name
  `02-the-surface.md#the-imports` as their first use (`DriverLease`,
  `LoopOutcome`, `TemplateSource`, `Workspace`), all owned by `one-call`; state
  each row's minimum local statement at that anchor and leave the rows
  `pending`. `grove_loop::VERSION` is owned here and is not an early use.
- Cite `docs/USAGE.md#usage-tree-verbs` beside the twelve verbs. The catalogue
  rule binds: the twelve verbs follow the worked example, never precede it.
  Read the verb list from `crates/grove-llm/src/cli.rs` (evidence) and
  `crates/grove-llm/tests/instructed_verbs.rs`, and count them.
- Leave the page shaped to receive *Command surfaces* from
  `docs/ARCHITECTURE.md` at `architecture-move-k31`; move nothing.

## Done when

- The literal and composite fragments for lines 1–19 are defined on the page,
  the source-index defer for `surface-grammar` is replaced by an insert, its
  ownership row reads `resolved`, and the fragment index has its rows.
- `README.md` links the page, chapter 1's navigation gains a Next, and
  `concept-index.md` has this chapter's entries.
- `book-check --through no-arguments --check all` is valid: 73 resolved lines,
  131 deferred. The repository Markdown sweep passes. `scripts/check.sh` stays
  red on `book-check` alone, by design.

## Notes

**Verified rather than reconstructed.** The four argument vectors in
`worked-argv` were measured against the built binary at `20.1.0` in an empty
directory that is not a Jujutsu workspace: `--help` and `--version` print to
stdout and exit `0` there, `--harness claude` prints to stderr and exits `2`,
and bare `grove` fails at `Workspace::resolve` with exit `1` — which is the
observable proof that the first two stop before the flow. The `--help` and
`--version` transcripts match `docs/USAGE.md` byte for byte; the guide carries
no refusal transcript, and the page says the refusal is measured.

**One claim was corrected by measurement before it shipped.** A first draft
said `name = "grove"` fixes the usage line's name. A copy of the binary under
another file name prints `Usage: renamed-binary` and still prints
`grove 20.1.0`, so `name` fixes the `--version` line and the usage line takes
argv[0]. The page says so.

**Twelve, ten, flat — all read from evidence.** The `Command` enum in
`crates/grove-llm/src/cli.rs` declares twelve variants and `grove-llm --help`
lists them; `crates/grove-llm/tests/instructed_verbs.rs` pins ten instructed
verbs and asserts the surface is flat, and both tests pass at this checkout.
No byte of that file is reproduced.

**A stale claim in another book's corpus is not repeated, and is cut as a
leaf.** The `grove-llm` attribute comment says the package carries a `0.1.0`
of its own; its manifest says `version.workspace = true`. The page states the
two mechanisms that hold the numbers equal today and argues for the one the
comment still gets right. `grove-llm-version-comment-k83` is inserted beside
`manifest-function-count-k82`, after every crate book.

**The one in-session review was spent, and it paid.** A fresh context was
given the page and the sources with a *disprove it* brief and returned nine
findings; every one was classified. Three were wrong claims and are fixed:
`--version` is not added by clap unconditionally but because line 16 supplies a
version; three lines, not one, feed the two outputs; and *every member takes
`version.workspace = true`* is false, since `book-validation` carries a `0.1.0`
of its own. Five were imprecisions and are tightened: line 20 is a blank line
rather than the function's opening; `Cli` is the only type of the binary's two
source files, not of the package; one closure test, not two, keeps the struct
empty; the bare vector's exit column now carries `1`; and the verb table is
condensed from the help text and the guide rather than from the help text
alone. One is chapter 3's: the loop reads `TemplateSource` twice per iteration,
not once, and the manifest's minimum statement says once — recorded for
`three-steps-k79` in the node brief, and this page's wording made
count-neutral. The *every member* claim is also the frozen corpus's, at
`cli.rs` line 12 and two other sites, and chapter 1 repeated it; chapter 1's
sentence is narrowed to what holds, and `every-member-version-comment-k84` is
cut beside k82 and k83.

**Proof.** `book-check --through no-arguments --check all` is valid: 3 files,
73 resolved lines, 131 deferred, `final=false`. `reference_navigation` and
`corpus_exception_inventory` pass. `scripts/check.sh`'s `book-check` stage is
red by design: the final run over this book reports the three deferred blocks
that `three-steps-k79` and `proving-a-negative-k80` own, and the three later
pages as missing.

**The check script was red on one more stage, and the cause is outside this
leaf.** A first background run stalled: `crates/grove/tests/loop_driver.rs`
reported one fixture failed and another never returned, and the run sat for
2h40m with no child process and no output until it was killed. A second,
foreground run under a timeout reached the end and was red on `cargo test` as
well as `book-check`: two fixtures in `crates/grove-loop/tests/driver_lease.rs`
— `a_second_driver_refuses_before_tree_access_or_launch` and
`a_reinitialized_tree_reuses_plan_k1_without_reusing_the_old_session` — panicked
with *nothing wrote first-ready … still running after 120s*. Rerun alone, each
passes, in 117s and 103s; the whole `grove-loop` test crate passes alone; and
the whole `loop_driver` suite passes alone in 40s. Two tests that need over 100
seconds against a 120-second readiness limit overrun it under the workspace-wide
run's load. Nothing this leaf changed is read by either fixture — the diff is
Markdown under `docs/walkthroughs/overview/` and `.grove/` — so the finding is
recorded here and handed to the tree rather than fixed.

**The harness did not have `grove:grove-draft`.** The Claude Code plugin cache
this session loaded predates the four stage skills, exactly as
`overview-book-k30`'s decision 1 warned; the skill was read from
`plugins/grove/skills/grove-draft/SKILL.md` in the working tree, with
`grove:grove` and `grove:grove-impl` loaded from the cache for the spine.
