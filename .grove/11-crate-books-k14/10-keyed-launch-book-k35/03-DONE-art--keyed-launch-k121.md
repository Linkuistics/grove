# keyed-launch-k121

## Goal

The `art` stage over the `keyed-launch` book — `docs/walkthroughs/keyed-launch/`,
ten chapters plus `README.md`, `concept-index.md` and `source-index.md` over a
2,073-line corpus. Two obligations and nothing else: every relation the reader
would otherwise reassemble from running prose is drawn as a table, a list figure
or a text-fence diagram; and every figure states its role adjacent to it. Ends
green on `book-check --final --check all` and `bash scripts/check.sh`.

## Context

- The charter is `docs/specs/walkthrough-books.md`, *Figures*. It fixes the class
  (a table, a diagram, **or any other non-fragment fenced block** — `text`,
  `console` or otherwise, whether or not it carries a relation), the medium (the
  page's own Markdown; `docs/adr/a-book-carries-no-asset.md` — no file beside the
  page), the two rules, and the placement consequences: a table may not be the
  nearest preceding nonblank block before a literal fragment's opening directive
  (`M105`), so a figure near a fragment sits at its section's end or above the
  paragraph that runs into the directive.
- **The role-rule exemption is exactly the four `source-index.md` tables** under
  the headings *Source roots*, *Ownership blocks*, *Fragments* and the early-use
  table — machine-reconciled derived indexes, and `F009` forbids a lead-in there.
  The fifth table on that page, the owned-source totals, is **not** exempt and
  states its role like any other figure.
- Four-backtick fences are literal fragments, not figures, and are exact source
  bytes. Never touch one: the corpus is frozen and an edit inside a fence turns
  `F008` red.
- `## Carried forward from the draft` in the node brief binds here. In
  particular: the prose obligation is **directional** (chapters 3–5 supply an
  argument the source does not make; chapters 7–8 do not restate one the fragment
  graph has just quoted), so a figure added to even the chapters out is a defect;
  chapter 8 must not attribute the child's group leadership to `setpgid` on both
  sides of the fork; and chapter 10's *seven tests* / *nine tests* counts are both
  right and are not to be reconciled.
- Figures are not the copy-edit's or the draft's work. A structural finding is a
  `draft` re-run leaf and a contiguous correction run through `proof`; a
  sentence-level defect is `copy-edit`'s and, if it is still open, an earlier
  stage's — recorded, not absorbed.

## Done when

- Every figure in the thirteen pages has been enumerated and classified, and each
  non-exempt one carries an adjacent statement of its role.
- Every relation carried only by running prose where a figure would carry it is
  either drawn or recorded as declined, with the reason named as one of the three
  the charter distinguishes — an editorial judgement, a rule in the contract that
  forbids it, or *the medium could not carry it*.
- `cargo run --quiet -p book-validation --bin book-check -- --repo . --book
  docs/walkthroughs/keyed-launch --final --check all` is valid, unchanged at 9
  files and 2,073 resolved lines.
- `bash scripts/check.sh` passes.
- **Last act:** `grove-llm leaf-add keyed-launch-book-k35 keyed-launch --kind
  proof`, unless a live later sibling under `keyed-launch-book-k35` already holds
  that stage.

## Notes

**No asset, ever.** The medium is the page's own bytes. A relation that genuinely
cannot be carried by a table, a list figure or a box-drawing diagram inside a
`text` fence is recorded as such and named as evidence about the medium — the one
finding that reopens the contract — and is not reached for when the real reason is
editorial judgement.

## Decisions (running log)

**Sixty-two figures enumerated, and the role rule holds across all of them.** A
figure is a table, or any non-fragment fenced block, so the enumeration was
mechanical: three-backtick and five-or-more-backtick fences plus every contiguous
run of `|`-leading lines, with four-backtick literal fragments excluded. Sixty-two
in thirteen files — 5, 4, 6, 6, 10, 5, 3, 4, 9, 5 across chapters 1–10, five in
`source-index.md`, and none in `README.md` or `concept-index.md`. Every one
outside the four exempt `source-index.md` tables carries an adjacent role
statement, `source-index.md`'s owned-source totals table included. Nothing was
added or reworded for that rule.

**Four relations were carried only by running prose, and each is now drawn.**
Chapter 10's three arms of the pass-through test, as a table of the move, the
cost and what the crate does instead — the README states this as the reader's
exit competence and it existed nowhere as one figure, only as three sections
spread over 250 lines. Chapter 9's five fixture files, against chapter 6's three
grammar conditions, which its own heading (*Four decoys, each wrong in a
different way*) promised and its prose left the reader to map. Chapter 8's `Watch`
against `End`, whose whole point is that the two are confusable and which stood
as two parallel sentences. And chapter 10's three split source roots as a
box-drawing diagram in a `text` fence, because the fact that matters there is an
interleaving — chapter 5's block inside chapter 3's pair, chapter 8's inside
chapter 7's — and no list of line ranges can show it.

**No medium was introduced.** Everything is the page's own Markdown; there is no
file beside a page and nothing that would need one. The one relation that argued
for something a table cannot do got a box-drawing diagram inside a `text` fence,
which the contract names as in scope and in use. **No finding of the third kind
was made** — nothing here was declined because the medium could not carry it.

**Five relations were declined, all on editorial judgement, and none on the
contract or the medium.** `README.md`'s nine chapter-opening refusals map
positionally onto nine chapters, but the per-member mapping is not what the reader
takes from the sentence, chapter 10's opening table carries that division
formally, and annotating the contents list would break the bare numbered list all
five books share. Chapter 6's *three ways to have no token* is a partition whose
answer is uniform — three rows all reading `None` carry less than the sentence,
and the fragment above it quotes the three verbatim. Chapter 6's three prefix
obligations and chapter 7's seven signals are sets where the interesting facts
attach to one or two members, so most rows of a table would be empty; both are
handled in bolded prose. Chapter 8's *three fields differ between the two runs*
is one sentence rather than several, and its point is an asymmetry a three-row
table would flatten. Chapter 10's nine-addend owned-source sum is already the
`source-index.md` totals table, cited.

**Nothing was added to chapters 7 or 8 that pads prose.** The node brief's
directional obligation binds the *prose*, and the one figure added in that range
is a comparison the source makes nowhere — the `Watch`/`End` pair whose types
share nothing and whose comments do not mention each other, which is the brief's
through-line obligation rather than a restatement.

**No source byte, no fragment, no manifest row and no structure changed.**
`book-check --final --check all` is unchanged at 9 files and 2,073 resolved lines
before and after. Every factual cell in a new figure was read off the frozen
source or the manifest: `End`'s three variants and `Watch`'s three from
`crates/keyed-launch/src/run.rs` lines 95–130, the twenty ownership blocks and
the three roots' line counts from `walkthrough.toml` as `source-index.md`
reconciles them, and the five fixture filenames from `src/channel.rs` lines
369–379.

**The review allowance was not spent.** Every claim a new figure makes is either
a range or a variant name read directly off the frozen source and the manifest,
or a restatement of a claim the page already made in prose — neither is the
narrow unexpected claim the four-step pass exists for.
