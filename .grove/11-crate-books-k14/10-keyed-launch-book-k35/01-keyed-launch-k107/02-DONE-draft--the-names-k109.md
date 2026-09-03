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

## Decisions (running log)

**1 · Nine sections, and the compiled-shapes catalogue sits after the worked
example.** The chapter's two arguments — the vocabulary's position in the
signature, and the per-key `source` — get a section each, `SlotRule` and
`Requirement` get one each, and the eight shapes of `templates.rs` 1–91 are read
last. The order is forced by the specification's worked-example rule: no section
before the complete example may primarily enumerate three or more members, and
the shapes are exactly such a catalogue. `#the-four-slots` therefore falls between
the vocabulary types and the shapes, which is also where the argument becomes
concrete. Fragment definitions sit in the sections that explain them; the two
composites list their children in source order, which is what the graph requires.

**2 · Block decomposition: four descendants under `vocabulary`, seven under
`template-shapes`.** `vocabulary` splits 1–13, 14–20, 21–27, 28–44, at its own
type boundaries with each trailing blank line kept with the type above it.
`template-shapes` splits 1–14 (imports), 15–28 (`Templates`), 29–33 (`SlotSpec`),
34–45 (`Template`), 46–52 (`Word`), 53–73 (`DocumentRole`), 74–91 (the three
diagnostic shapes). The last groups three types in one fragment because they carry
no comment between them and are explained as one report shape. Both partitions are
gapless and neither moves a top-level boundary.

**3 · Behaviour was verified against a scratch crate, not against the suite
alone.** Four claims the chapter makes are not asserted by any test in
`crates/keyed-launch/tests/`: that an empty slot table turns `${prompt}` into a
literal and silences the word-zero rule; that quoting does not escape a
substitution, so `'${prompt}'` still substitutes; that expansion demands a value
for every declared slot including an `AtMostOnce` one; and the exact rendered text
of a cardinality refusal. Each was run against the shipped crate from a throwaway
package outside the repository with a path dependency on it, because the corpus is
frozen and a temporary test inside `crates/keyed-launch/` would have shifted every
fragment range below it. Where a test does pin a claim the chapter names the test
instead.

**4 · Three claims drafted from the structure brief or from the source's own
phrasing were corrected against the code before landing.** The brief and this
leaf's body both say `templates.rs` 1–91 is "the seven types"; it is eight
declarations, and the page says eight and gives the four-and-four split. The page
first credited `Requirement`'s `PartialEq`, `Eq` and `Debug` derives with in-crate
uses they do not have — no caller exists anywhere in the workspace — and now says
so. And `ValidationDiagnostic`'s optional location was first described as a
document-level case; it is not, it is `locations.first().copied()` in
`validate_document` and the `None` is unreachable. None of the three is a defect
in the corpus, so none engages the freeze or the adjudication rule the brief
retired.

**5 · The chapter states which of the three substitution rules the slot table is
needed for, rather than repeating the doc comment's grouping.** `vocabulary.rs`
lines 3–9 say none of the three is checkable without the names; the whole-word
rule alone is, since `parse_template_word` reaches it through
`word.contains("${")` without consulting `slots`. The page's figure marks that
column honestly and then makes the stronger point the code actually supports: the
whole-word rule is the *gate* on the name lookup, since only a word that is
nothing but `${name}` reaches it, so the rule that survives without the table
decides nothing on its own. This is precision about a claim, not a finding against
the comment, and nothing is handed forward.

**6 · `scripts/check.sh` is red on `book-check` alone, as this leaf's brief
predicted for every child but the tenth.** The script runs `book-check --final`
over each book root by discovery, and a prefix leaves later blocks deferred by
design. This slice proves itself with
`book-check --repo . --book docs/walkthroughs/keyed-launch --through
rules-about-names --check all`: valid, 331 resolved lines, 1,742 deferred,
`final=false`.
