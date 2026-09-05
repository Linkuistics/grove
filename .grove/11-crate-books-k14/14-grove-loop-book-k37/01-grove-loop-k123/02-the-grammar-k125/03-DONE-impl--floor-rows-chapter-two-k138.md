# floor-rows-chapter-two-k138

## Goal

Apply the early-use criterion `display-first-use-k137` wrote into
`docs/specs/grove-loop-book-structure.md` across the whole of chapter 2's
reproduced blocks, and add every floor row chapter 2 owes but does not carry.

## Context

- **The criterion, now stated.** `docs/specs/grove-loop-book-structure.md`,
  *Early uses the order forces*, says a reproduced block owes a row for a
  later-owned symbol it **names or exercises**, unless an existing row already
  states the minimum a reader needs there. `entry_path`'s mandatory row is the
  precedent for *names* — it is anchored on the module header that names it, not
  on the chapter that reads the function.
- **This leaf exists because k137 settled the criterion on one symbol and did not
  re-run it over the block.** That leaf's goal was the disputed `Display` /
  `compose` row; a partial enumeration is the exact failure the *enumerate, then
  classify* rule warns about, so the whole sweep is here rather than split across
  two sessions.
- **Four candidates, found by an adversarial read of k137 and not yet
  verified by enumeration.** Each is used by bytes chapter 2 reproduces and
  defined by a block a later chapter owns:
  - `impl Display for TaskNameError` — defined at `task_name.rs` 727, block
    `591-1020` (chapter 4). Chapter 2 calls it at source 1,344, 1,386 and 1,472,
    all in block `1313-1521`, and its prose analyses the rendered advice text.
    This is the **sharpest** one: it is the same shape as the row k137 added for
    `TaskName`'s `Display`, in the same block, and k137 added one and not the
    other. Note the bytewise sort — `` `impl Display for TaskNameError` ``
    sorts **before** `` `impl Display for TaskName` `` (`E` is 0x45, `` ` `` is
    0x60), so the row goes above it.
  - `peel_key` — `task_name.rs` 1,012, block `591-1020` (chapter 4). Named in
    chapter 2's reproduced header at source 68 and load-bearing in the page's
    prose. A private free function, so no cast row can cover it.
  - `split_shape` — `task_name.rs` 967, block `591-1020` (chapter 4). Named in
    the same reproduced header and used in the `#classified` analysis. Also
    private.
  - `Handle::render` — `task_name.rs` 517, block `221-590` (chapter 3). Named in
    the reproduced header and analysed in the prose beside it. The chapter-1 cast
    row covers `Handle`, not this associated function — and the book already
    decided that question in the affirmative for `Parts::leaf`.
  Lower confidence, same shape and to be classified rather than assumed:
  `Kind::new` and `Slug::new`, named in the reproduced header.
- **Do not trust that list.** It is a starting set from one reviewer, not an
  enumeration. Extract every candidate token from chapter 2's three blocks —
  `1-220`, `1178-1199`, `1313-1521` — and classify each one, which is complete by
  construction; the list above is what to expect to find, not what to look for.
- The row k137 added is `` `impl Display for TaskName` `` at
  `02-the-tokens.md#refusals-inside-the-shape`; its statement and placement are
  the model for these.

## Done when

- Every later-owned symbol chapter 2's blocks name or exercise either carries a
  row in `docs/walkthroughs/grove-loop/source-index.md`'s *Early uses* table or
  is recorded here as classified out, with the reason.
- Each added row has its minimum local statement on the page, at the anchor the
  row names — the *Refusals inside the shape* paragraph that currently opens
  *Four names this section uses before the chapter that explains them* is where
  most of them belong, and its count moves with them.
- No manifest change: these are floor rows, and `[[early-use]]` carries only the
  rows a book may not omit.
- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  four-verdicts --check all` is still valid, with resolved and deferred line
  counts unchanged at 887 and 9,646.
- `bash scripts/check.sh` is red on `book-check` alone.

## The sweep, and what it found

**Method: enumerate, then classify.** Every backticked token and every
identifier in the 451 reproduced lines — `task_name.rs` `1-220`, `1178-1199` and
`1313-1521`, extracted whole rather than read for candidates — was intersected
with the definition set of `crates/grove-loop/src/`, and each survivor was
classified against the block that defines it. The starting set of four was right
about all four; its two lower-confidence candidates were confirmed and recorded
as one row; and the sweep found two more rows the list did not have.

**Seven rows added** to `docs/walkthroughs/grove-loop/source-index.md`'s *Early
uses*, in the order the sort key `(chapter, anchor byte offset, owner, symbol
cell)` forces:

| Symbol | Named or exercised at | Defined | Owner |
|---|---|---|---|
| `Kind::new`, `Slug::new` | header 54–55, `#both-words-one-rule` | 258, 325 | 3 |
| `Handle::render` | header 66, `#the-handle-in-this-grammar` | 517 | 3 |
| `peel_key` | header 67, same | 1,012 | 4 |
| `split_shape` | header 62, same | 967 | 4 |
| `Parts::Node` | `Outcome` doc 108, `#the-outcome` | 536 | 3 |
| `pick` | `Outcome::Live` doc 112, `#the-outcome` | walk at 582 | 7 |
| `impl Display for TaskNameError` | `error.to_string()` 1,344 / 1,386 / 1,472, `#refusals-inside-the-shape` | 727 | 4 |

**`pick` is the one the starting list could not have contained**, and it is the
argument for enumerating: it is named in a doc comment, not called, and by a
sentence about `Outcome::Live` rather than about the walk. The owner is
`first-live-leaf` and not `twelve-not-fourteen` because that is the book's own
settled mapping — chapter 1's twelve-verb table reads `` `pick` | 7 ``.

**Classified out, with the reason each time.**

- `task_tree::handle_key` (header 62) — **no referent**. It was deleted with the
  other name model; `task_name.rs` 464 and 983 and `task_tree.rs` 905 are its
  epitaphs. No block owns it, so no chapter can explain it, and a row promising
  otherwise would be a promise the book cannot keep. The page now says it is
  gone.
- `TaskName::Brief`, `TaskName::Positioned` — variants of a type the mandatory
  family row covers, and `TaskName::distinguished`'s own row already states what
  `Brief` is: *what `parse` makes of `BRIEF.md` and what the domain advertises
  are the same variant*.
- `TaskNameError::{NotCanonical, MissingSeparator, BadKind, BadSlug,
  NodeWearsOutcome, SpeciesMismatch}` — variants of a type the family row
  covers, and each is **chapter 2's own subject**, explained in the section
  beside the test that produces it. A forward reference the page resolves where
  it makes it is not one a row records; the reproduced bytes' claim about the
  *rendering* of those variants is what earned the `Display` row above.
- `Handle`, `Kind`, `Slug`, `Parts`, `TaskName`, `TaskNameError` bare — chapter
  1's cast rows and chapter 2's mandatory family row.
- `parse` (`EntryName for TaskName`, 789) — the family row states the minimum:
  *the only way to a verdict is through the `EntryName` implementation chapter 4
  owns*.
- Chapter 2's own: `BRIEF`, `KEY_MARK`, `SEPARATOR`, `Outcome` and its three
  variants, `Outcome::infix`, `Outcome::strip`, `TokenError`, `refuse_token`,
  and the eight test functions in the two reproduced test blocks.
- Already rowed: `TaskName::distinguished`, the family, `Parts::leaf`, `a_kind`
  / `slug`, `impl Display for TaskName`.
- Not this corpus: every `ordinal_fs_tree` item the block imports or matches on
  — `EntryName`, `Found` and its three cases, `Key`, `Key::new`, `NameView`,
  `Ordinal`, `Ordinal::new`, `PositionedSpecies`, `Species` and its three cases,
  `Triple`, `Verdict` and its four — plus `core::fmt`, `std::error::Error` and
  the `write!` / `format!` / `split_once` macros and methods. **The ledger
  records forward references inside one book, and no block of this corpus owns a
  library symbol**; the two crates' books may not link to each other either. The
  mandatory family row names `Verdict` for its own reason and was not touched.
- `witness_two_filenames_name_one_entry` (header 28) — an Alloy predicate in
  `docs/ordinal-fs-tree/models/structure.als`, and `src/leaf.rs` (header 54) — a
  deleted file. Neither is a symbol of this corpus.

**No manifest change**, as the brief required: `[[early-use]]` still carries the
three mandatory rows and these seven are floor rows the book adds.

## Found while sweeping

**A technical-truth defect in chapter 2's prose, corrected in passing.** Both
`#the-handle-in-this-grammar` and `#refusals-inside-the-shape` said that **both
of `TaskName`'s arms** end in a call to `Handle::render`. `TaskName`'s two arms
are `Brief` and `Positioned`, and the `Brief` arm writes `BRIEF.md` and touches
no handle (`task_name.rs` 616). The two arms that do end in `Handle::render` are
`Parts::Leaf` and `Parts::Node` **inside** the positioned case, at 638 to 654,
which is what the source comment at 627 says. Both sentences now read *both arms
of its positioned case*. The `Handle::render` row states the accurate form, so
leaving the neighbouring sentence wrong would have put the ledger and the page in
contradiction.

**The same *both arms* slip is in the mandatory `impl Display for TaskName` row,
and it was left alone.** That row reads *both arms ending in the handle's own
renderer*, which is true of the positioned case's two arms and not of
`TaskName`'s own `Brief` and `Positioned`. It is a `[[early-use]]` row matched
byte-for-byte by `check_early_uses`, so correcting it is a manifest change and
this leaf is forbidden one. The page's two prose statements of the same claim are
now precise, so a reader is not misled; the row is a wording defect for
`kind-slug-handle-k135` or a later stage to carry, since chapter 3 is the page
that reads `Handle` and would be reconciled against it. **Recorded, not acted
on.**

**A uniqueness claim the row nearly overstated.** The `Handle::render` statement
was first written as *the only `write!` in the crate that spells
`<slug>-k<key>`*, which is false: `task_tree.rs` 1,569 and `task_grow/tests.rs`
390, 992 and 993 build fixture names with `format!`. The claim is about
production code, and the checkable form of it is `KEY_MARK` — exactly three uses
in the crate, its definition at 90, the `write!` at 518 and `peel_key`'s
`strip_suffix` at 1,017. Both the row and the page now say that instead. The
header's own wording — *the only `write!` the grammar appears in* — is
narrower than it reads, and chapter 2's existing prose was already careful to
call it a fact *about this file*.

**`pick` is also named on chapter 1's page, and chapter 1 carries no row for
it.** Its twelve-verb table names all twelve verbs and the chapter that reads
each. The structure brief already rules on that shape — chapter 1's naming of the
cast is *one block naming the cast at low resolution, not a per-term forward
reference* — so chapter 2's `#the-outcome` is the first use that owes a row, and
that is where it was recorded. If a later stage rejects that ruling the row moves
to chapter 1, whose leaf is retired. **Recorded, not acted on.**

**The criterion may belong one level up.** Restated unchanged from *Notes* below:
`docs/specs/walkthrough-books.md`'s two-clause trigger carries no *unless an
existing row already covers it* exception, and this sweep leaned on that
exception seven times to classify symbols out. It is book-general. It governs
four books already proved `final=true`, so promoting it is a cross-book claim
against four ledgers. **Recorded, not acted on.**

**No evidence turned up either way on `entry_path`'s anchor**, which the leaf
scoped out. The criterion as written explains the row as it stands, and chapter 5
does not exist to contradict it.

## Notes

**This runs before chapter 3 deliberately**, for the same reason k137 did:
`kind-slug-handle-k135` and `the-name-k136` owe the same enumeration over their
own blocks, and they should inherit a settled criterion and a worked example
rather than discovering both.

**The corpus is frozen.** Nothing here touches `crates/grove-loop/`.

**The criterion may belong one level up, and that is a finding to record rather
than a change to make.** `docs/specs/walkthrough-books.md`'s *Early-use ledger*
states the two-clause trigger — *first uses* or *reproduces source bytes whose
referent belongs to a later slice* — but carries no *unless an existing row
already covers it* exception, and `display-first-use-k137` wrote that exception
into the `grove-loop` structure brief because that is where it was needed. It is
book-general, so it arguably belongs in the specification; but that specification
governs five books, four of them already written and proved `final=true`, so
changing it is a cross-book claim that has to be checked against those four
ledgers. Note it here, do not act on it.

**Whether `entry_path`'s anchor is right is not this leaf's.** The criterion
above was written to explain the row as it stands; if the sweep turns up evidence
that it cannot, that is a finding to record, not to act on — chapter 5 does not
exist yet.
