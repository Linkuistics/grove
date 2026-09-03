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
