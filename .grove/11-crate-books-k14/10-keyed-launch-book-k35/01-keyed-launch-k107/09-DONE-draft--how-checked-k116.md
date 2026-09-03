# how-checked-k116

## Goal

Draft chapter 9 of the `keyed-launch` book — *How this is checked*,
`09-how-checked.md`, slice `checked-without-meaning` — owning `src/channel.rs`
lines 272–404 (133) and `src/conformance.rs` whole (104): 237 lines. This child
takes the book to 2,073 resolved lines and zero deferred.

## Context

- Draft stage, child 9 of 10 of `keyed-launch-k107`. Responsibilities are the
  structure brief's *9 · How this is checked — checked without meaning*.
- The conformance kit as the **cross-crate seam** — a consumer's own suite can only
  assert that its configuration works with its build, and the kit is what holds a
  configuration to this crate's contract from outside the consumer; the three
  obligations `check` applies in order, and why the third is not a second spelling
  of `load`, since expansion is the only place the compiled words are walked.
- **Why an empty document fails is the chapter's best case for the spine**: a kit
  that only reports violations reads identically when handed nothing to check, so a
  configuration declaring no keys is a failure in its own right — not because an
  empty file is malformed but because a suite of must-hold claims cannot otherwise
  detect that it did not run.
- The nine inline channel tests, read as **what a `#[cfg(test)]` module inside a
  root buys that an integration test cannot** — reaching `is_channel_name`, a
  private function whose exactness chapter 6 argued and only this module can pin.
  Explain them against chapter 6's fragments, which are behind this page.
- **State once why these nine tests are in the corpus and the 1,319 lines under
  `crates/keyed-launch/tests/` are not**: a root is `src/**/*.rs`, and
  `docs/specs/walkthrough-books.md`'s corpus exception inventory carries no
  `keyed-launch` row. Do not add one.
- Required example anchor: `checking-the-same-file` — `conformance::check` over
  that primary, through an empty document failing and a violated rule reported.

## Done when

- `book-check --repo . --book docs/walkthroughs/keyed-launch --through
  checked-without-meaning --check all` is valid: 2,073 resolved lines, 0 deferred,
  `final=false`.
- `scripts/check.sh` is red on `book-check` alone — the final scope still rejects
  the missing tenth page and the `pending` ledger rows — and this file says so.

## Notes

Zero deferred is not `final=true`. The tenth page does not exist yet, and a final
run rejects the book until it does; that is `what-passes-through-k117`'s.

## Decisions (running log)

**1 · Twelve literal fragments under two composites, cut at the obligation in
the kit and at the method in the test module.** `src/conformance.rs` is one
module comment, one type and one function, and the function is a sequence of
three obligations, so seven fragments follow that sequence: thesis and imports
(1–31), `Outcome` (32–43), the contract and the placeholders (44–63), the load
(64–72), the empty-key rule (73–82), the borrowed values (83–90), the loop and
the return (91–104). The nine inline tests are cut into five by the method each
exercises — the module header (272–274), then `allocate` (275–312), `read`
(313–351), `discard` (352–365) and `discard_abandoned` (366–404) — rather than
one fragment per test, because the page's argument is what each *method's*
evidence covers and what it leaves uncovered, and nine introductions would have
had to make that argument in nine pieces. Every fragment begins on the blank line
before its item, which is the convention chapters 6, 7 and 8 already set in these
two roots. Rejected: one fragment per test, and a single fragment for the whole
inline module.

**2 · The kit is read first and the inline module second, against the manifest's
root order.** `source-channel` precedes `source-conformance` in the manifest, so
the derived tables list the test fragments first; the page does not. The chapter
opens on the kit because the required worked-example anchor is
`#checking-the-same-file`, which is `conformance::check`, and the specification
puts the complete example before any catalogue of three or more obligations. It
also puts the two halves in the order the chapter's own claim needs: the kit
checks a consumer's document from outside the consumer, the module checks this
crate's private grammar from inside the file that states it, and the second reads
as the mirror of the first rather than the first as an appendix to the second.
Fragment definition order in Markdown has no expansion meaning, so nothing in the
graph objects.

**3 · The third obligation cannot fail, and that is reported on the page rather
than routed to a leaf.** `conformance::check`'s third obligation expands every
key and reports `` key `k` does not expand `` or `` key `k` expands to an empty
program ``. Neither string is producible by any consumer configuration: `load`
fails on any diagnostic; `validate_template` diagnoses an empty word list and a
word zero that is not a non-empty `Word::Literal`; every `Word::Slot` index comes
from a `position` in the slot table `Templates` still holds; and `check`'s own
placeholder list is built from `vocabulary.slots`, one entry per declared slot,
which is exactly what `match_values` requires. No test names either string. This
is **not** a corpus defect and no leaf is owed: the doc comment's stated ground —
*expansion is the only place the compiled words are walked* — is literally true,
and the obligation is real coverage of the compile-and-expand path even though it
is redundant as a report about the document. The page says both, names the
alternative (drop it, and `check` becomes a load plus a key count), and leaves the
table's `Pinned by` cell empty with the reason stated rather than filling it with
a test that does not hold the claim.

**4 · No early-use row is possible from this chapter, and the one row it owns
flips to `explained`.** An early-use row is triggered by a first use of a symbol a
**later** slice owns. The only slice after `checked-without-meaning` is
`assembly`, which owns no source, so this chapter cannot owe a row however many
later-owned symbols its 237 lines called — there are none to call. The single row
this slice owns, `conformance::check` first used at `01-orientation.md#the-cast`,
moves from `pending` to `explained`, and `--through` accepted the ledger, which is
the check that the edit was reconciled against the manifest rather than merely
made.

**5 · The leaf's one in-session reviewer was spent on the page's test-behaviour
and reachability claims, and returned fifteen findings.** The brief handed it the
page, both owned roots, the four roots the page cites, all five files under
`crates/keyed-launch/tests/`, `crates/grove-loop/tests/session_config.rs` and
chapters 1–8, with the conclusions stripped and an instruction to try to
construct a counterexample rather than to confirm. Classified:

- **Thirteen valid and actionable, all applied.** The private items a
  `#[cfg(test)]` module reaches are seven, not four. Public fields are not
  `Ended` and `Outcome` alone — six public types hide their fields and seven do
  not, and the line is whether a field would let a caller fabricate a value the
  crate authored, which is a better claim than the one it replaced. `Channel::read`'s
  `.ok()?` collapses *unreadable* into *nothing written*, not *empty*, and
  chapter 6 says so at `06-the-channel.md`, so the original sentence broke the
  chapter-attribution rule as well as the source one. The failures field is read
  in four shapes across the repository, not two. Only the **first** obligation
  ends the run; the other two record and fall through. The module comment is 25
  of 104 lines, a quarter, not a third — the 33 that is a third counts `///`
  comments too. The doctest is not the only thing that breaks when a public name
  moves; `tests/conformance_kit.rs` imports the identical four. The `.unwrap()`
  form carries claims in three more places in the module. `allocate`'s second
  refusal is about the drawn path, not the directory's entries. The crate's
  refusals do not divide into justification and instruction; several end in
  neither. The retry *loop* runs on every allocation — only its `continue` arm
  and the post-loop refusal go unreached. The example's call figure wrote
  `Path::new("~/…")`, which `fs::read_to_string` would not expand, and named a
  `SLOTS` const that belongs to a different test's two-slot vocabulary. And
  `a_grove_configuration_conforms_to_the_runners_own_kit` writes four keys, only
  the first of which uses all four slots.
- **One contract stated unclearly.** The reviewer reported the absent
  `10-what-passes-through.md` and chapter 9's missing `Next:` link as a defect.
  Both are the scoped-authoring rule working: `docs/specs/walkthrough-books.md`
  says the last page in a scoped prefix takes the final-page navigation form even
  when the manifest gives it a successor. The brief named the final-scope
  specification and not that clause. No change.
- **One noise.** A wording slip the reviewer read in an earlier build; already
  corrected before its report arrived.
- **Category A survived.** The reviewer attacked the unreachability argument of
  decision 3 with named constructions — a template splitting to zero words, word
  zero as a slot, word zero as an empty literal, a duplicated vocabulary name, a
  zero-slot vocabulary, `${}`, `${a}${b}`, `foo${bar}baz` — and found every one
  of them diagnosed before `load` returns. The argument stands as the page states
  it.

Fifteen findings on one page, of which the three that mattered most were a false
uniqueness claim, a false count and a mis-attributed line of chapter 6 — the same
class the node brief's earlier leaves hit. Every one came from a sentence written
from the shape of the code rather than from re-reading it.

## What the checks said

`book-check --repo . --book docs/walkthroughs/keyed-launch --through
checked-without-meaning --check all` reports **valid: 9 files, 2073 resolved
lines, 0 deferred lines, final=false**. Every byte of all nine roots is now owned
by a page, and `source-index.md` carries no `defer` line and no `pending`
early-use row.

`bash scripts/check.sh` fails **1 of 8**, on `book-check` and nothing else — the
other seven (`cargo fmt`, `shellcheck`, `cargo clippy`, `plugin install`,
`conformance`, `conformance suite`, `cargo test`) all pass. The three findings are
the missing tenth page and its two consequences: `M101` for the absent
`10-what-passes-through.md`, `M103` for the `README.md` contents entry that
cannot yet be a link, and `M103` for chapter 9's navigation line, which under
`--final` is required to carry a `Next:` to a page that does not exist. Under a
scoped run the same line is correct — `docs/specs/walkthrough-books.md` gives the
last page of a prefix the final-page form even when the manifest names a
successor — so all three close together at `what-passes-through-k117` and none is
a defect in this slice. The `pending` ledger rows this leaf's `Done when`
predicted are already gone: the last one, `conformance::check`, was this slice's
own to flip.

**Chapter 10 is next**, and it is the book's only leaf that owns no source:
`what-passes-through-k117` states the pass-through test and applies it to all
nine source-owning chapters, taking the book to green `--final` validation and a
green `bash scripts/check.sh`. Its last act is the pipeline's — `grove-llm
leaf-add keyed-launch-book-k35 keyed-launch --kind copy-edit`, unless a live later
sibling under that node already holds the stage.
