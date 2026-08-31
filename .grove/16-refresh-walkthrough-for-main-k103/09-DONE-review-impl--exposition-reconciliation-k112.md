# exposition-reconciliation-k112

**Reviews:** `exposition-reconciliation-k110`

## Goal

Adversarially review the committed refreshed `docs/ordinal-fs-tree/book` as one
artifact for technical accuracy and editorial coherence against its source,
navigation, and exact-reconstruction contract.


## Context

- Review the producer commit named by `exposition-reconciliation-k110`, then
  reconcile it with the working tree if the intervening
  `book-validation-diagnostic-fixtures-k111` commit has landed.
- The subject is every file under `docs/ordinal-fs-tree/book`, not only the
  producer's changed roll-up files. Check APIs, root lifecycle, errors,
  refusals, reports, whole-tree deletion, stdout/stderr boundaries, invariants,
  and trade-offs against the current production source.
- The book contract requires seventeen fixed source roots and 8,720 owned lines,
  exact byte-for-byte recursive expansion, complete local navigation, and
  declarative, direct, self-contained exposition for a Rust-proficient reader.
- The producer's final evidence includes `book-check --final --check all`, the
  complete `ordinal-fs-tree` suite, and both formal-model runners. The two known
  `book-validation` diagnostic fixture failures are owned by k111 rather than
  by this reviewed artifact.

## Done when

- Findings, if any, cite exact `path:line` locations, state the violated
  contract, and distinguish technical errors from editorial defects and visible
  trade-offs.
- The review checks the source-owning chapters and the README, concept index,
  source index, and invariants/trade-offs roll-up rather than trusting the
  producer's stale-pattern sweep or changed-file list.
- If actionable findings exist, commission the correctly placed
  `integrate-review-impl` leaf for this same bare stem; otherwise retire without
  creating one.

## Notes

This is the proportionate independent technical/editorial assurance required by
the refresh node before it can close.

## Findings

Subject: `docs/ordinal-fs-tree/book` as it stands in the working tree. The
`book-validation-diagnostic-fixtures-k111` commit changed only
`crates/book-validation/tests/`, so the reviewed book is byte-identical to the
one `exposition-reconciliation-k110` committed and no reconciliation was needed.

### F1 — editorial, actionable: the concept index omits one new section

`docs/ordinal-fs-tree/book/concept-index.md:21` ends the `02-name-seam.md`
block without an entry for `02-name-seam.md#sought-object-resolution`
("Search results are not mutation refusals"), the `##` section added by
`name-and-reference-k105` at `docs/ordinal-fs-tree/book/02-name-seam.md:959`.

Violated contract: the refresh brief's requirement that navigation describe the
current APIs, and the book contract's complete local navigation.

Why this is a defect rather than curation. The refresh added four `##` sections
across the book, and `k110`'s own diff added concept-index entries for exactly
three of them — `05-mutation-algebra.md#initialization`,
`06-filesystem-interpreter.md#opening-lifecycle`, and
`06-filesystem-interpreter.md#whole-tree-deletion`. The fourth was missed. The
omitted section is the one that introduces `Sought`, a public type re-exported
at `crates/ordinal-fs-tree/src/lib.rs:103` and owning a new fixed source root
(`src/sought.rs`). `source-index.md:272` already registers `Sought` in the
early-use table, so the ledger tracked the new material and the navigation index
did not. No concept-index entry mentions `Sought` or search resolution, leaving
the topic unreachable from the book's only topical entry point.

The three long-standing unindexed `##` sections in `03-reference-domain.md`
(`#rendered-names`, `#reusable-samples`, `#seam-mapping`) and
`06-filesystem-interpreter.md#worked-apply-and-unwind` predate this refresh and
are not part of this finding.

Suggested repair is a single line after `concept-index.md:21`; the integrating
session owns the wording and the decision.

### Checked and clean

No technical error was found. Verified directly against the current production
source rather than against the producer's changed-file list:

- Fixed source inventory. `source-index.md:11-27` lists seventeen roots whose
  claimed line counts match `wc -l` on every file exactly, summing to 8,720.
  `src/fixtures.rs` is correctly excluded — it is `#[cfg(test)] mod fixtures`
  at `crates/ordinal-fs-tree/src/lib.rs:77-78`, so it is a test-only fixture and
  evidence rather than reproduced source. No other production `.rs` file is
  unaccounted for.
- Ownership ledger self-consistency. The `ownership-blocks` table tiles all
  seventeen roots with no gap and no overlap; every row's claimed count equals
  `end - start + 1`, and every root's blocks reach exactly its last line.
- Navigation integrity. Every internal anchor link across all eleven pages
  resolves to a declared `<a id=...>`; every `.md` link target exists; the
  prev/contents/next header and footer bars are present, symmetric, and correct
  on all eight numbered pages, and consistent on both lookup pages.
- Public API. Every one of the 296 distinct backticked identifiers in book
  prose outside source fragments exists in the production source; the nine
  non-matches are book notation (`defer`, `composite`, `explained`), model
  instance names, `EINTR`, `u32::MAX`, and `rm` — the last correctly
  asserted as *not* an alias.
- Root lifecycle. `Vacancy::initialize` does plan through `Plan::guarded`
  (`src/ops.rs:182`), so `08-invariants-and-trade-offs.md:45` is accurate; the
  root create, the pre-create `names_are_one_component` check, and the
  root-removal unwind at `src/fs/mod.rs:448-521` match `06`'s account, including
  the `NotFound` case that stays `Failed` rather than becoming
  `FailedPartiallyRolledBack`.
- Whole-tree deletion. `06-filesystem-interpreter.md:1666-1671` and
  `:1987-1995` match `src/fs/remove.rs` on all three `spelled_directly`
  refusal reasons, post-order scheduling via pushing `RemoveLevel` first,
  reverse-sorted child push, symbolic links unlinked and never descended, the
  root removed last and reported on its own field, and `RemovalStopped`
  carrying the failed step plus the already-removed paths.
- Errors. The bullet list at `06-filesystem-interpreter.md:2037-2054` is
  exhaustive over all twelve `Error` variants in `src/error.rs`, and the
  `source()` claim matches `src/error.rs:490-509`.
- CLI. Fourteen verbs, four read helpers, seven mutation helpers — all three
  counts match `run` at `bin/syllabus.rs:1067-1130`. `delete` takes the
  exclusive lock before checking `--yes` (`bin/syllabus.rs:1370-1373`), `--yes`
  is required, `ls` is the only alias in the file so "no `rm` alias" holds, and
  the default `--root .` is indeed refused by the last-component check.
- Stream boundary. Only stdout `BrokenPipe` is benign
  (`bin/syllabus.rs:778-784`); every stderr failure becomes exit 1; clap output
  is routed through the same checked seam (`bin/syllabus.rs:806-822`). The
  percent-encoding description matches `encode_path` exactly, including the
  `b' '..=b'~'` printable range, the `%` exception, and uppercase hex digits.
- Exit categories. `RemovalStopped` with an empty `removed` list maps to 6 and
  otherwise to 7 (`bin/syllabus.rs:716-717`), exactly as
  `07-syllabus-cli.md:2187-2188` states.
- Counting claims. Seven `EntryName` obligations of which the kit samples five
  (`Obligation::ALL: [Self; 5]`), eight CLI fragment ranges, five planned public
  operations, two `Decision` variants, and the eight Quint instance names in
  `08-invariants-and-trade-offs.md:172-179` all match the source and
  `models/run-quint.sh:131-132`.
- Style contract. No rhetorical question, first- or second-person address,
  hedge, or emotive qualifier appears anywhere in the 3,216 lines of prose
  outside source fragments. Every apparent hit is the Rust `?` operator or the
  token `I/O`.
- Staleness. No surviving pre-rebase claim that the CLI lacks a removal command
  or a destructive verb; no residual "fifteen roots/files" text; no `defer`
  directive anywhere.

### Visible trade-off, not a defect

`05-mutation-algebra.md:1389` attributes `OrdinalsExhausted` to "append or
insert". `initialize` also reaches it, because it shares `ops::creations`
(`src/ops.rs:534-556`) with `append_many`. Reaching it there would require
initializing a root with `u32::MAX` entries, so the compression is a reasonable
editorial choice rather than an error, and it is recorded here only so the
integration can see it was considered.
