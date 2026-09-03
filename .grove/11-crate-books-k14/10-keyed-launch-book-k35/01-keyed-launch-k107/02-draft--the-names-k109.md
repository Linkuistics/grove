# the-names-k109

## Goal

Draft chapter 2 of the `keyed-launch` book — *The names a template is written
against*, `02-the-names.md`, slice `rules-about-names` — owning
`src/vocabulary.rs` whole (44) and `src/templates.rs` lines 1–91 (91): 135 lines.

## Context

- Draft stage, child 2 of 10 of `keyed-launch-k107`. Responsibilities are the
  structure brief's *2 · The names a template is written against*.
- **The chapter's whole argument is why the vocabulary is an input to `load` and
  not to `expand`** — every template rule is a rule about slot *names*, so a
  loader that will not learn the names until expansion can check none of them,
  and a vocabulary supplied per call would make every rule just-in-time. This is
  the one place in the book that takes a position a reader could disagree with,
  and it is stated as an argument with its rejected alternative and its cost.
- Then `Requirement` and its two messages; that `SlotRule` names a slot bare and
  `${name}` is the spelling; and the compiled shapes chapters 3–5 use — `SlotSpec`,
  `Template` carrying **the file it was read from** per key, `Word` as
  literal-or-slot-by-index, `DocumentRole` and its statement that the rules do
  *not* differ by role, `SourceLocation`, `ValidationDiagnostic`, `NodeValidation`.
- **The per-key `source` is the chapter's second argued claim**: after an overlay
  resolves there is no single answer, so a diagnostic naming a file has to name
  the one that actually supplied the failing key.
- Prose obligation 3 is **supply the argument**: this is a `templates.rs` chapter
  and that root is 13% comment. Do not thin where the source is silent.
- Required example anchor: `the-four-slots` — from the vocabulary value to
  `${prompt}` being required, and why that is checkable only here.
- Blocks: `vocabulary` (`source-vocabulary`, 1–44) and `template-shapes`
  (`source-templates`, 1–91). Replace both defers with inserts and move both
  ownership rows to `resolved`.

## Done when

- `book-check --repo . --book docs/walkthroughs/keyed-launch --through
  rules-about-names --check all` is valid: 331 resolved lines, 1,742 deferred,
  `final=false`.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.

## Notes

The early-use ledger gains chapter 3's forward reference to `validate_node` and
`validate_template` only when chapter 3 lands; this chapter changes only the rows
whose owner is `rules-about-names`.
