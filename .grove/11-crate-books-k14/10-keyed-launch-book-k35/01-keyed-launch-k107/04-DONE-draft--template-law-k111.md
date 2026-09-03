# template-law-k111

## Goal

Draft chapter 4 of the `keyed-launch` book — *What a template must be*,
`04-template-law.md`, slice `words-not-shell` — owning `src/templates.rs` lines
415–534 (120), 535–617 (83) and 618–660 (43): 246 lines, the heaviest of the
`templates.rs` chapters.

## Context

- Draft stage, child 4 of 10 of `keyed-launch-k107`. Responsibilities are the
  structure brief's *4 · What a template must be*.
- **Every rule a template must satisfy, each with the line that enforces it, the
  diagnostic it produces, and the record clause it keeps**: the node shape, the
  duplicate key reporting *every* declaration location, word zero being a literal
  executable, a substitution occupying a whole word, a declared slot's
  cardinality, and unmatched quotes.
- **The unquoted `#` is the chapter's sharpest case.** `shell-words` treats it as
  a comment start and would silently truncate the argv, so the crate scans for it
  and refuses rather than accepting a line that means less than it says.
  `ShellWordScanState` is the reason that scan cannot be a `contains`.
- `whole_substitution` as the mechanism of whole-word substitution and where
  `${a}${b}` is refused; `render_diagnostics`, `format_location` and
  `source_location` as the machinery behind *name what is wrong, name where, name
  what fixes it* — **the obligation chapter 1 stated and this chapter is the only
  one that discharges**.
- Prose obligation 3 is **supply the argument**, and this chapter is where it
  binds hardest: the rules are the reason the decision records exist and the
  source states almost none of it. Name the record each rule answers to; never
  cite one.
- This chapter completes the early-use row `validate_node`, `validate_template`
  first used at chapter 3 — move its status to `explained`.
- Required example anchor: `the-rules-on-one-line` — the `impl` template as text
  through to the compiled `Vec<Word>`, and the same line with an unquoted `#`,
  refused with its location.

## Done when

- `book-check --repo . --book docs/walkthroughs/keyed-launch --through
  words-not-shell --check all` is valid: 770 resolved lines, 1,303 deferred,
  `final=false`.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.

## Notes

Three blocks, one root, adjacent: 415–534, 535–617, 618–660. The split is by what
the reader is being shown — the rules, the scanner behind one of them, the
diagnostics all of them produce — not by the file's own type/function ordering.

## Decisions (running log)

**1 · Fifteen literal fragments over the three blocks, cut by rule rather than by
function.** The brief left the decomposition *inside* a top-level ownership block
to the book. `node-and-template-rules` is eight — the node's shape rules, its
argument rules, its return, then `validate_template`'s signature, its two gates,
its word loop and its cardinality loop — because each of those is one row of the
rules table and the page is organised as a walk down that table.
`word-scanning` is four and `diagnostics` three, both at their function
boundaries, because there the function *is* the unit being explained. Rejected:
one fragment per function throughout, which would have put the two early returns
and the accumulating half of `validate_template` in one 65-line block and made
the gate/accumulate distinction — the chapter's sharpest structural point —
invisible on the page.

**2 · Strict source order for the sections, with the scanner left where it
falls.** The `#` rule (477) and the scanner that implements it (546–583) are one
idea, and the brief pairs them. Pulling the scanner forward to sit beside the
rule would have meant reading 535–583 and then going back for 485–534. Instead
the `#` section names the scan and says it is read below, and the chapter walks
415 → 660 once. Chapter 3 set the precedent of announcing a gap rather than
reordering around it, and the gap here is two sections rather than that
chapter's 130 lines.

**3 · Three unproved rules reported in the table rather than absorbed or
omitted.** `type annotations are not allowed`, `a key must have exactly one
positional argument` and `a key's sole argument must be a string` are enforced
and untested — the first and third nowhere in the workspace, the second only from
`crates/grove-loop/tests/session_config.rs`. Prose obligation 1 requires the
test that proves each claim, so the `Pinned by` column would have been a
half-truth either way: silently blank, or filled with the aggregate test that
covers only the property arm of a different rule. The page states the gap and
names the leaf that closes it. Writing the tests here was rejected under
`references/decompose.md` — they serve the crate, not this chapter — and they are
`template-rule-tests-k118`, inserted ahead of `architecture-residue-k75` per the
root brief's rule for work found while documenting. It is not a corpus change:
`tests/` is evidence, not a root, so no ledger or page is implicated and no book
waits on it.

**4 · One figure drawn, the rest left to `art`.** The seven-state partition of
what a `#` means is drawn, because the chapter's sharpest claim — that the scan
cannot be a `contains` — is a mapping over a set of members that the prose was
otherwise asking the reader to reassemble from six sentences, which the shared
specification's *Figures* rule makes an editorial finding. Everything else the
page carries is either a fragment or a worked-example block with its role stated
adjacent. A full transition diagram of `ShellWordScanState` is a real candidate
and is deliberately **not** drawn: the match arms are reproduced verbatim two
paragraphs below it, so it is ordinary `art` work rather than a relation this
stage owed, and drawing it would leave that stage's result unreadable.

## Result

`book-check --repo . --book docs/walkthroughs/keyed-launch --through
words-not-shell --check all` is **valid: 9 files, 770 resolved lines, 1303
deferred lines, `final=false`** — the figures this leaf's *Done when* names.

`bash scripts/check.sh` exits 1: **FAILED — 1 of 8, and the one is
`book-check`**, as every child of this node but the last is expected to leave it.
The script runs `--final` over each book root by discovery, and under `--final`
the keyed-launch book still reports the six unwritten pages (`M101`), the two
navigation and contents entries that name them (`M103`), the ten source blocks
chapters 5–9 own (`F003`), and the ownership and early-use rows still `pending`
for those chapters (`F009`). Every one of those findings names a later chapter;
none names `words-not-shell`. The other seven checks — `cargo fmt`, `shellcheck`,
`cargo clippy`, plugin install, conformance, the conformance suite and the test
suite, including `every_repository_markdown_reference_resolves` and the
corpus-inventory tests — pass.

Both early-use rows this chapter owns are now `explained`: the manifest's
`validate_node`, `validate_template`, and the row chapter 3 added beyond the
manifest for `source_location`, `format_location`, `render_diagnostics`. Neither
appears in the `--final` `F009` list, which is the check that they were accepted
rather than merely edited.
