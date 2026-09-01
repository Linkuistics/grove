# orientation-k55

## Goal

Create the book: `walkthrough.toml`, `README.md`, both lookup indexes and
`01-orientation.md`, with every later-owned block deferred, proved by
`book-check --through no-dependencies --check all`. Discharge the three
obligations that live outside the book directory, and open the draft stage
record.

## Context

- The chapter's responsibilities, its worked example and its early-use rows are
  fixed by [`docs/specs/jj-workspace-book-structure.md`](../../../docs/specs/jj-workspace-book-structure.md)
  — *1 · Orientation*, the `commit-tour` row of *Worked examples*, and the seven
  rows of *Early uses*. Do not re-decide them.
- The whole scaffolding obligation is
  [`docs/specs/walkthrough-books.md`](../../../docs/specs/walkthrough-books.md),
  *Authoring workflow and scoped proof*: **the manifest is complete from the
  start** — every `[[page]]`, `[[root]]`, `[[block]]`, `[[early-use]]`, the
  `[guide]` group and the `[[glossary]]` group — because a plan authored
  incrementally cannot be compared against a prefix.
- This slice owns two blocks and 98 lines: `manifest-no-dependencies`
  (`Cargo.toml` `1-44`, the whole manifest) and `library-crate-thesis`
  (`src/lib.rs` `1-54`). The other nine blocks are deferred.
- The **exact starting tree of the carried example** is left open by the
  structure brief and is this slice's to fix — concrete names, paths and the
  change id. Chapters 5 and 6 reuse them unchanged, so fix them deliberately.
- The concrete carried example is grove's own: `crates/grove-loop/src/driver_lease.rs`
  passes the literal namespace `"grove"`, and `crates/grove-llm/src/cli.rs`
  resolves and commits. Cite by path; never reproduce their source.

## Done when

- `docs/walkthroughs/jj-workspace/` holds `walkthrough.toml`, `README.md`,
  `01-orientation.md`, `concept-index.md` and `source-index.md`, and nothing
  else.
- `cargo run --quiet -p book-validation --bin book-check -- --repo . --book
  docs/walkthroughs/jj-workspace --through no-dependencies --check all` exits 0.
- `README.md`'s reader contract cites `docs/USAGE.md#usage-workspace-layouts`,
  and the manifest's `[guide]` declares that anchor with the two the later
  chapters reserve.
- `CONTEXT.md` carries `task-commit-boundary`, `driver-lease`, `stated-vcs` and
  `loop-control-channel` as explicit `<a id="…"></a>` lines, each immediately
  preceding a heading — the form `book-validation`'s `explicit_anchors` accepts,
  which is why an anchor before a bold paragraph does not discharge it.
- `docs/ARCHITECTURE.md`'s *Documentation ownership* table carries a row naming
  this book's contents page as canonical for the `jj-workspace` crate's
  walkthrough.
- `docs/evaluations/editorial-pipeline-pilot/stages/0-draft.md` exists in the
  shape the preregistration's *The stage record* fixes, with `## Baseline` in
  place of `## Claims`, and `## Provenance` listing this commit as the first of
  the draft range.
- `cargo test --locked --workspace` passes, `cargo clippy --workspace
  --all-targets` passes and `cargo fmt --all --check` passes. `bash
  scripts/check.sh` is **red on `book-check` alone**, because the book is a
  prefix; say so in the commit and check nothing else off against it.

## Notes

**Do not polish.** This is the draft stage, and its whole value to the pilot is
being an unedited baseline the five editorial stages are measured against.

**Orientation owns no source belonging to a later chapter** (structure brief,
*Cross-layer source in orientation*). The low-resolution trace names later-owned
types and copies none of their bytes; each such name is paid for by a row in the
early-use ledger.

## Decisions (running log)

**1 · The manifest declares five early-use rows, not the structure brief's
seven, and the missing two become ledger rows at `the-gate-k56`.** Forced by the
validator rather than chosen. `check_early_uses`
(`crates/book-validation/src/ledger.rs`) requires every manifest `[[early-use]]`
row to appear in the ledger *and* resolves each ledger row's first-use anchor by
looking the page up in the snapshot's book files — while scoped mode forbids a
later page from existing at all. The two rows whose first use is
`02-the-gate.md#worked-resolution` are therefore unsatisfiable while chapter 1 is
being proved: including them fails the anchor check, omitting them fails the
mandatory-row check. The specification's *Early-use ledger* already says authors
add rows beyond the manifest's, so the two obligations survive intact as ledger
rows; what is lost is the manifest's authority over them, which is a real
weakening and is why the underlying disagreement is cut as `early-use-scope-k63`
rather than absorbed. Rejected: editing the manifest at `the-gate-k56` to add
them — the manifest is the plan, and a plan authored incrementally cannot be
compared against a prefix, which is the whole basis of scoped proof.

**2 · The carried example is a native workspace at `/work/atlas`, a caller in
`crates/gateway/src`, one `.grove/` task file, and change id
`vrxqnwzomtklpsuvyzqrnwmtkxlpsoun`.** The structure brief fixed the *shape* and
left the concrete values to this slice; chapters 5 and 6 reuse them unchanged, so
they are recorded here rather than left to be re-derived. Three properties were
chosen deliberately. The workspace is **native** — `.jj/repo` is a directory —
so chapter 1's trace spawns no jj at all during resolution, which is what makes
*reads add no history* visible before any subprocess has been explained; the
borrowed case is chapter 2's, where the pointer-file premise is stated. The
caller sits four levels down, so the ancestor walk has four steps to show rather
than one. And the committed path is a retired task file, because that is the
`.grove/` write grove actually path-scopes a commit around.

**3 · The trace shows argument lists, not command lines.** Changed after first
drafting it as rendered command lines. The crate builds `&["commit", "-m",
message, fileset]` and the rendered form exists only for a refusal to quote, so a
command line in the trace would show the crate's *error-reporting* spelling in
the position where a reader is being taught the *call*. The message contains
spaces and the fileset contains quotes, and both are exactly the places where the
two spellings stop being interchangeable.

**4 · Both owned blocks are refined into five intent-named literal children
each.** The specification permits refinement and the structure brief leaves the
partition to the owning slice. Forty-four and fifty-four lines would each have
been one fence, and a fence that long takes one prose paragraph for five separate
arguments — the empty dependency table, the dev-dependency boundary, the lint
inheritance and the release lane are four claims, not one. The `M105` rule that
every literal fragment is preceded by a paragraph is what makes the refinement
pay: five fragments buys five paragraphs, each answering the five editorial
questions for a range a reader can hold. The top-level block IDs, owners and
ranges are unchanged, so line-count credit is unaffected.

**5 · The public surface is stated before the worked example, and the six-refusal
table after it.** The book contract forbids any section earlier than the worked
example from *primarily enumerating* three or more public operations, and permits
"vocabulary needed for the trace" to precede it. `#public-surface` is written to
sit on the permitted side of that line: one sentence per name, no signatures, no
behaviour a later chapter owns. The six-refusal table is a genuine catalogue and
is placed after `#commit-tour` for that reason — and independently, the early-use
ledger sorts by anchor occurrence, so `#the-six-refusals` carrying `is_tracked`'s
row must follow `#commit-tour` on the page.

**6 · The consumer's-half convention is a blockquote opening `**The consumer's
half.**`.** Decision 5 of `jj-workspace-structure-k17` requires these passages to
be marked and left the form open. A blockquote is visible in raw Markdown as well
as rendered — which matters, because `docs/walkthroughs/` is read as Markdown and
nothing compiles it — and it needs no validator support. The convention is
introduced in `#what-it-declines` with a worked instance, so every later chapter
inherits it rather than re-deciding it.
