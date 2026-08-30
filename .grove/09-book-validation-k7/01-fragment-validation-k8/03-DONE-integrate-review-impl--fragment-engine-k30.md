# fragment-engine-k30

**Integrates:** fragment-engine-k29

## Goal

Fix the five defects the fresh-context review of `fragment-engine-k26` found,
and re-run the verification those fixes touch.

## Context

- Findings below are anchored to commit `lnpukulmyzsw`
  (`fragment-engine-k26: add exact fragment validation foundation`) and to the
  working tree as it stood immediately after it. No leaf ran in between, so the
  cited lines should still be exact; re-derive any that has moved.
- `docs/specs/ordinal-fs-tree-book.md` is the contract; `fragment-engine-k26`'s
  task file carries the producer's running decision log.
- Boundaries the review checked and left alone: `F009` ledger-table
  reconciliation and `F010` page placement belong to `ledger-and-pages-k27`; the
  `related`/`source` evidence fields, the full total-order sort, `I001`, and
  adversarial-graph hardening belong to `diagnostic-contract-k28`; every
  Markdown code belongs to `markdown-validation-k9`. Do not absorb them.

## Done when

Each finding below is either fixed with a test that fails without the fix, or
recorded in the running log with the reason it was rejected. Crate tests and
Clippy pass; repository-wide verification is run.

### F-1 — a frozen source file that grows past its declared last line still validates (high)

`crates/book-validation/src/validator.rs:902` — `source_range` clamps the
comparison to the declared range: when `range.last < starts.len()` it ends at
`starts[range.last]`, so bytes after the declared last line are never read.
Nothing else checks a source file's real length. `check_inventory`
(`crates/book-validation/src/validator.rs:337-360`) asserts only that the
*book's* declared root range equals the hard-coded `ROOTS` constant, and that
`snapshot.source_files` contains the path.

Consequence: append lines to any of the fifteen frozen files and final mode
still reports `15 files, 6,618 resolved lines, deferred 0, final=true, valid`
while the expansion no longer reproduces the file. Truncation *is* caught —
`source_range` returns `None` and `F008` fires — so the hole is one-sided and
covers the likeliest drift, code being added.

The corpus test cannot catch it either. `crates/book-validation/tests/support/mod.rs:259-337`
synthesizes the book *from* the live source bytes using hard-coded per-root
`lines`, and its `source_lines` helper
(`crates/book-validation/tests/support/mod.rs:323`) applies the same clamp, so
`crates/book-validation/tests/corpus_validation.rs:6` compares the source
against itself and cannot fail on byte identity.

Checked against the current tree: all fifteen files match the ledger today and
total 6,618 lines, so this is latent rather than live.

Suggested remedy: have `check_inventory` compare each loaded source file's
actual line count against the `ROOTS` constant and report the disagreement; and
give the byte-identity claim at least one assertion whose expected bytes are not
derived from the source under test.

### F-2 — a malformed literal fragment silently discards the rest of the book file (high)

`crates/book-validation/src/parser.rs:218-226` — when the four-backtick body has
no closing ```` ```` ```` line, or the close is not immediately followed by
`<!-- /fragment -->`, the parser emits one `P002` and then `return`s from
`parse_file`. Every later source root, fragment and directive in that file is
dropped, and the end-of-file checks for an unclosed ordinary fence
(`crates/book-validation/src/parser.rs:301`) and an unclosed active construct
(`crates/book-validation/src/parser.rs:309`) never run.

Consequence: in `source-index.md`, which holds all fifteen roots, one mistyped
closing fence becomes fourteen spurious `F006` "required source root is missing"
findings plus every dependent `F002` and `F005`, and buries any genuine finding
after it.

This contradicts `docs/specs/ordinal-fs-tree-book.md:334` ("Lexical findings do
not prevent the lexer from resuming at the next LF when that is safe") and the
`fragment-engine-k26` criterion "safe malformed-input recovery are tested". The
mis-close case is trivially safe to resume from: `close` already names the
delimiter line.

`crates/book-validation/tests/lexer.rs` has no unclosed- or mis-closed-literal
case, so nothing enshrines the current behaviour.

Suggested remedy: resume at `close + 1` when a `````` ```` `````-only line was
found, and at `index + 1` when it was not; add a fixture for each that asserts a
later root in the same file is still recognized.

### F-3 — an exact `book-page` directive in a disallowed context emits no `P002` (medium)

`crates/book-validation/src/parser.rs:165-168` short-circuits on
`valid_book_page(line)` before any context test and before the
`active.is_some()` branch at `crates/book-validation/src/parser.rs:295`. So
`<!-- book-page id="contents" role="contents" -->` sitting between two `insert`
lines inside a `source-root` block is silently accepted.

`docs/specs/ordinal-fs-tree-book.md:297` requires "An exact directive in a
disallowed context is `P002`", and
`docs/specs/ordinal-fs-tree-book.md:329` forbids non-directive content inside a
root or composite.

`crates/book-validation/tests/lexer.rs:69` only asserts that a well-placed
`book-page` emits no `P001`; the disallowed-context case is untested.

Boundary: page *placement* (`F010`) is `ledger-and-pages-k27`'s. This is the
shared lexer's context rule, which this increment owns and which
`markdown-validation-k9` will also consume.

Suggested remedy: gate the `valid_book_page` arm on `active.is_none()` and fall
through to `invalid_context` otherwise.

### F-4 — the committed spec's stable author command no longer runs (medium)

`docs/specs/ordinal-fs-tree-book.md:662` and `:672` give `--check all` as the
stable scoped and final author commands, and
`docs/specs/ordinal-fs-tree-book.md:676` states that `--check` "accepts
`fragments`, `markdown`, or `all` and defaults to `all`". The shipped CLI
accepts only `fragments` and defaults to it
(`crates/book-validation/src/cli.rs:43-45` and `:51-54`), so both documented
commands now exit 2 with `U001` from the flag parser.

Narrowing the surface was a deliberate and well-argued decision — silently
aliasing `all` to fragments can report false success — but it was recorded only
in `fragment-engine-k26`'s running log, which no later authoring slice reads.
`orientation-k11` follows the spec, and gets an invocation error.

Two consequences to settle at the same time:

- the *default* has moved from `all` to `fragments`, so once
  `markdown-validation-k9` lands, an author who omits `--check` keeps getting
  fragment-only checking unless the default is flipped back;
- `crates/book-validation/README.md` and the `--check` help text document
  `fragments` without saying the value set is staged.

Suggested remedy: amend `docs/specs/ordinal-fs-tree-book.md` to record the
staged `--check` surface and name the increment that completes it — or accept
`markdown` and `all` and reject them with an explicit `U001` naming
`markdown-validation-k9`, which fails with an actionable message rather than at
the flag parser.

### F-5 — scoped coverage totals count an ambiguous definition (low)

`crates/book-validation/src/validator.rs:962` resolves a root's top-level insert
with `parsed.fragments.get(id).and_then(|items| items.first())`, silently taking
the first of a duplicated definition, while every other check in the module
gates on `items.len() == 1` (`crates/book-validation/src/validator.rs:512`,
`:530`, `:605`, `:708`, `:826`). A book with a duplicated top-level ID therefore
prints `resolved_lines` derived from an ambiguous definition beside its `F001`.

The report is already `findings`, so this misleads only a reader of the coverage
numbers rather than changing the verdict.

Suggested remedy: skip ambiguous IDs in `coverage`, matching the rest of the
module.

## Notes

The review confirmed the rest of `fragment-engine-k26`'s contract and found no
defect in it: the eight required fragment diagnostic classes are present and
distinct; a defer whose named later slice never fills it is `F003`, not `F002`
(`crates/book-validation/tests/fragment_failures.rs:140`); cycle witnesses are
rotated to their lexicographically smallest ID; literal bodies are read as raw
bytes with no trimming or newline normalization; `source-index` scoped and final
ownership states are enforced positionally against the ledger; the CLI is
read-only, non-interactive, requires `--repo`, rejects a `--book` with a `..`
component before loading anything, and exposes only the check it implements; and
the crate is a workspace member inheriting the deny-all Clippy gate.

One item the review confirmed but did **not** hand over, because
`diagnostic-contract-k28` already charters it: `visit`
(`crates/book-validation/src/validator.rs:768`) and `expand_children`
(`crates/book-validation/src/validator.rs:872`) are unmemoized recursions, so a
deep or diamond-shaped fragment graph can overflow the stack or expand
exponentially before any finding is reported. That is `k28`'s "Deep or
adversarial graphs cannot stack-overflow or expand exponentially" criterion, and
the two function names above are where it lands.

## Decisions (running log)

All five findings survived verification. F-1 is an inventory defect: validation
now compares each loaded frozen source's actual physical-line count with the
fixed corpus before range comparison. Its regression appends an independently
known extra line to the final corpus; the existing hand-written recursive
fixture remains the byte expectation not derived from live source.

F-2 is a lexer recovery defect. A literal with a delimiter but no exact fragment
close resumes at the delimiter's following line; one with no delimiter resumes
after the fragment opener. Both malformed forms discard the incomplete
definition, retain `P002`, and expose a later root to the shared lexer.

F-3 is a context-gating defect. Exact `book-page` directives remain recognized
tokens only outside an active root or composite; inside one they emit `P002`
without also becoming malformed `P001` directives.

F-4 is a contract stated unclearly rather than a missing check implementation.
The spec, CLI help, and crate README now name the current fragments-only stage,
its `fragments` default, and `markdown-validation-k9` as the increment that adds
`markdown` and `all` and restores the `all` default. The help regression checks
the staged boundary is visible to an operator.

F-5 is a reporting defect. Coverage now credits an inserted definition only
when its ID resolves uniquely, matching expansion and the other validator
passes; an ambiguous top-level ID contributes zero resolved lines beside its
`F001`.

Repository-wide tests reached an unrelated pre-existing failure in
`reference_navigation::every_repository_markdown_reference_resolves`: the
specification's illustrative `relative/path` is treated as a real repository
reference. The parent revision contains the same example. The issue is recorded
on `markdown-validation-k9`, whose local-link contract owns the reconciliation;
this integration does not absorb it.
