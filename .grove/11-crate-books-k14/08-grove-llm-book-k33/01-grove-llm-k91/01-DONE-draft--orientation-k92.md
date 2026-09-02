# orientation-k92

## Goal

Create the `grove-llm` book and prove its first slice:
`one-call-plus-rendering`, `01-orientation.md`, owning the whole of
`crates/grove-llm/Cargo.toml`, `src/lib.rs` and `src/main.rs`, and `src/cli.rs`
lines 1–34.

## Context

- Draft stage, child 1 of 7 of `grove-llm-k91`. The structure brief is
  `docs/specs/grove-llm-book-structure.md`, and chapter 1's responsibilities
  are its *1 · Orientation* section: what `grove-llm` is and who drives it; the
  manifest's crate-not-a-target argument stated once as the ground the book
  stands on; why there is a library target; the three-line `main`; the header's
  thesis — one call plus rendering, and the three orders — as the book's map;
  the import block as evidence of what the binary reaches; the four
  dev-dependencies and which test needs each; `release = false` as an answered
  question. It adjudicates the two stale claims under *Known in advance* (the
  reachability clause, and `lib.rs`'s *or `grove`*).
- The first slice carries the book's scaffolding under
  `docs/specs/walkthrough-books.md`, *Authoring workflow and scoped proof*: the
  complete manifest, `README.md`, both lookup indexes, every source-root
  directive, the full ownership ledger with a defer for every later-owned
  block, and all fourteen early-use rows `pending`.
- The required example anchor is `one-session`: the carried session at low
  resolution — the mandated handle resolved, the brief chain printed, the work
  done, a review leaf added, the session's own leaf retired, the commit, and
  `complete` writing the relaunch flag — every verb named, no handler shown.
- **This page must carry `<a id="the-imports"></a>` on the section that reads
  lines 24–34**, and state the ten early-use rows' minimum statements there.
- The two obligations outside the book are this child's: the `CONTEXT.md`
  anchors `session-epoch` and `tree-access-lock`, and the book's row in
  `docs/ARCHITECTURE.md`'s *Documentation ownership* table.

## Done when

- `book-check --book docs/walkthroughs/grove-llm --through
  one-call-plus-rendering --check all` is valid: 4 files, 107 resolved lines,
  910 deferred, `final=false`.
- `every_repository_markdown_reference_resolves`,
  `every_book_root_has_a_documentation_ownership_row` and the corpus-inventory
  tests pass with the new book root present.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.

## Notes

The worked example fixes the values every later chapter reuses; they are
recorded in `grove-llm-book-k33`'s brief under *Pointers*.

## Decisions (running log)

**1 · The structure brief is `docs/specs/grove-llm-book-structure.md`, and it
was read before anything else.** It states the reader and outcome, the ordered
plan and each chapter's responsibilities, and the emphasis and exclusions, so
the `grove-draft` precondition is met from a named artifact and not inferred
from the node's brief.

**2 · The carried session reuses the overview's tree and adds one template.**
`/work/atlas/`, `01-impl--rate-limit-k3.md`, and `config.kdl` declaring `impl`
and `review-impl` with the same command; the driver's control files
`driver.lease` and `session.epoch` under `.jj/grove/`, and a
`GROVE_SIGNAL_FILE` under the same directory whose name follows the
`signal-<32 hex>` shape `driver_lease.rs`'s fixtures use. `review-impl` is
declared because chapter 4's happy path is `leaf-add … --kind review-impl`
and the presence rule would otherwise refuse it; chapter 4's refusal uses
`--kind prototype`, which the configuration does not declare. Recorded in the
node brief's *Pointers*.

**3 · The trace is stream-labelled and every verb is prefixed *admitted*.**
`run` admits before the exhaustive dispatch, so a low-resolution trace that
showed a verb's output without that step would misstate the order chapter 2
opens on. The stderr lines are quoted from `eprint_next_steps` and
`cmd_complete` verbatim; the fresh key follows
`add_assigns_fresh_key_as_max_over_whole_tree_plus_one` in
`crates/grove-loop/src/task_grow/tests.rs`.

**4 · Both stale claims are adjudicated as two facts each rather than one
verdict**, per the node brief's decision 7: the reachability sentence is
checked as *reached* (holds — every used `grove_loop::` name is `pub` at the
root, `Workspace` re-exported at `lib.rs` line 81) and as *reachable* (does
not hold — the direct `jj-workspace` line makes that crate's surface
reachable); `lib.rs`'s *or `grove`* is set beside the manifest's own record
that the dependency went at `loop-crate-driver-k22`. Neither is fixed;
neither is leafed this session, because the fix invalidates this chapter's
fragments and belongs after every book with `grove-llm-version-comment-k83`.

**5 · The import block is stated as evidence with a stated gap.** Fourteen
items and the `verbs` module are imported; `VERSION`,
`admit_ambient_session`, `read` and `write` are reached by path and never
imported. The page gives the total as eighteen items and one module so the
count on the page equals a count a reader can make with one grep.

**6 · No comment-line count appears on the page.** The brief's 45% is its
measurement and a naive grep gives a different number for `cli.rs` (`///`
lines included or not); the overview's *97* was wrong by five. The page says
*most of that module is comment* and stops.

**7 · One `M201` on the first scoped run, fixed before anything else.** A
concept-index label carried `` `[[bin]]` ``, and nested brackets are outside
the link subset; the label now reads *a bin target*. After the fix
`book-check --through one-call-plus-rendering --check all` is valid: 4 files,
107 resolved lines, 910 deferred, `final=false`. `reference_navigation` (13
tests, including `every_book_root_has_a_documentation_ownership_row`),
`corpus_exception_inventory` (5) and `grove-llm`'s `composition_guidance` and
`session_kind_guidance` (12) pass with the new root and the two promoted
glossary headings.

**8 · The harness did not have `grove:grove-draft`.** The Claude Code plugin
cache this session loaded predates the four stage skills; the skill was read
from `plugins/grove/skills/grove-draft/SKILL.md` and the editorial family file
from `plugins/grove/skills/grove/references/editorial.md` in the working tree,
with `grove:grove` and `grove:grove-impl` from the cache for the spine.

**9 · The one in-session review was spent, and it paid.** A fresh context was
given the page, the scaffolding, the corpus, `grove-loop`'s root, the tests,
the contract and the brief with a *disprove it* brief, checked about sixty
claims and returned nineteen findings; every one was classified. Ten were
wrong and are fixed: *most of the module is comment* (nearly half);
*fourteen imported items are `pub` at the root* (twelve are, `Resolution` and
`Signalled` are `verbs`'s); `Workspace` used in one function (two —
`worktree` and `cmd_finish_commit`); *every verb resolves it once* (`complete`
never does; now *every verb but `complete`*, in the manifest, the ledger and
the page); `tree_lock.rs` reaching `ordinal-fs-tree` (it names the crate only
in comments and a scanned string); the self-deadlock proved by a `libc` test
(it is proved by a source scan, `no_production_lock_grove_takes_for_itself_ever_blocks`;
the `libc` fixture serves the four contention tests); `default-features =
false` dropping a *filesystem interpreter* (it drops the `cli` feature and
the `clap` behind the demonstration binary); *every handler calls one
function of `verbs`* (four make a second call through a helper — the page
now adjudicates the slogan instead of restating it); *six of those lines are
grove's* (five); and *four dev-dependencies* in the heading (five; the brief's
own list omits `assert_cmd`). Five imprecisions tightened: the opening's
*can reach* now uses the narrower form the page itself settles on; the
path-reached items are five, not four, `verbs::Inserted` and `inherited_kind`'s
`read` included, so the total is nineteen items and one module; `name` is
stated beside `description`; admission is an `Option` — present under a
driver, absent for a manual command — in the ledger row and the trace's
framing; *two dependencies* reads *two workspace dependencies*. One contract
finding fixed: the rhetorical question before the release block is now
declarative. Noise, fixed anyway: the lints and release fragments now sit
under their own heading, `the-release-block`, and the concept index follows.
Visible trade-offs, unchanged: *spine*, *ground* and *map* are the structure
brief's own vocabulary and are handed to `copy-edit` under `## Handed
forward`; chapter 1 cites three glossary anchors the brief's table places on
later pages, which the contract permits at first use and the node brief's
decision 5 records. After the fixes `book-check --through
one-call-plus-rendering --check all` is valid (107 resolved, 910 deferred)
and `reference_navigation`'s thirteen tests pass.

**10 · `bash scripts/check.sh` ran over the page before the review edits
and is red on `book-check` alone, by design.** Seven of eight checks pass;
the three existing books report `final=true`; the `grove-llm` final run
reports the twenty-one deferred blocks, the six pages that do not yet exist,
and the `pending` ledger rows a final scope rejects. The review edits are
Markdown under `docs/walkthroughs/grove-llm/`, which only `book-check` and
`reference_navigation` read, and both were re-run green after them.
