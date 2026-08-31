# walkthrough-books-spec-k20

## Goal

Settle where a per-book corpus lives and what governs it: reopen the
sidecar-manifest rejection now that there are six books, record the verdict as an
ADR, and rewrite `docs/specs/ordinal-fs-tree-book.md` into a specification of the
book *system* whose per-book ledgers are data.

## Context

- Decision 10 of `plan-k1`: the book spec is split **in place** — a shared
  walkthrough-books spec covering page conventions, navigation, prose contract,
  audience, assurance and the fragment language, with per-book ledgers as data
  rather than specification. Editing in place rather than writing a second
  overlapping document is what the records doctrine prescribes.
- The rejection being reopened is `docs/specs/ordinal-fs-tree-book.md`,
  *Rejected alternatives and limits* → *A TOML sidecar manifest*. Its argument is
  that a sidecar duplicates parents, children, ownership and ranges already
  visible in Markdown, that drift would force a choice about which
  representation to trust, and that raw Markdown must remain sufficient to
  reconstruct the code. Read it before you decide anything: the argument was
  made for one book and the question is whether any clause of it depends on that.
- The same document's *Intended outcome* property 3 states the same commitment
  positively — raw Markdown exposes every fragment's parent, children, source
  range and owning slice **without requiring an author-maintained sidecar
  manifest**. A verdict that admits a sidecar has to amend that property too.
- What the validator currently compiles in, and what therefore has to come from
  somewhere: `SLICE_ORDER` and `PAGE_BY_OWNER` (canonical slice order, page
  filenames, page-to-slice mapping), `ROOTS` (source path and exact line count
  per root), `BLOCKS` (34 ownership ranges), `EARLY_USES`, and `SOURCE_INDEX`
  (the book's own source-index path). `src/cli.rs` also hard-codes the slice
  tokens as a `--through` `value_parser` list.
- The corpus each book must carry is enumerated in the root brief's *Pointers*:
  33 roots and 14,525 lines across five deliverables, plus the relocated book's
  own unchanged seventeen-file corpus.

## Done when

- An ADR under `docs/adr/` records the per-book-corpus decision with its
  trade-off — whether the ledger stays derived from the book's own Markdown, or
  becomes a sidecar, or splits — and states what would reopen it. If the original
  rejection survives contact with six books, an ADR recording *that*, with the
  clause-by-clause reasoning, is a successful outcome.
- `docs/specs/` holds one specification of the book system, with per-book data
  described as data. Whether that is a rewritten
  `docs/specs/ordinal-fs-tree-book.md` under a new slug or a rename is yours to
  decide; `CONTEXT-MAP.md`'s record-ownership list names the current slug and
  must agree with whatever you leave behind.
- The specification states the per-book corpus format precisely enough that
  `validator-structure-k21` and `validator-fragments-k22` can implement against
  it without a second design conversation.
- The specification states the **guide-link contract**: that every book links
  into `docs/USAGE.md` as the reader's entry point (decision 7 of `plan-k1`),
  which page of a book carries the link, and which guide anchors a book may cite.
  Without it, `walkthroughs-k3` decision 5's whole reason for writing the guide
  before the books — that books would otherwise inherit unstable anchors — is
  asserted by nothing and checked by nothing. Say also whether the relocated
  `ordinal-fs-tree` book is brought into conformance; if it is, cut the leaf that
  does it as your last act and place it ahead of the pilot.
- Every artifact that links into the old spec still resolves, and
  `bash scripts/check.sh` passes.

## Notes

**This is the campaign's agreement point and it earns an adversarial read.** Four
books and an overview are authored against whatever this settles, and no session
after this one has a human present. Cut `review-design` as your last act if the
decision is anything other than "the rejection stands unchanged".

**Place that review ahead of its consumers, not at this directory's end.**
`validator-structure-k21` and `validator-fragments-k22` both implement against
this spec and both sit later in this directory, so a plain `leaf-add` puts the
review behind the work it exists to redirect — which `walkthroughs-k3`
decision 10 says is worth nothing. Cut it as
`grove-llm leaf-insert <validator-structure-k21> <stem> --kind review-design`,
and tell that review to place any `integrate-review-design` it cuts the same
way.

**Do not specify the art phase here.** Figures, assets and what the validator
should know about them are deliberately deferred until the pilot has run the art
stage by hand and its measure has said whether art paid — see
`figure-contract-k18`. A specification that leaves a hole for assets is correct;
one that fills the hole now is machinery ordered ahead of its measurement.

**The prose contract, audience and assurance sections are shared as they stand.**
Decision 7 of `plan-k1` fixes the audience — a reader who knows Rust and jj and
has driven a grove, with grove vocabulary linked to `CONTEXT.md` and never
re-taught — and it applies to every book, not just this one.

## Decisions (running log)

**1 · The sidecar rejection was two rejections fused, and only one of them
holds.** Read clause by clause, the *Rejected alternatives* argument is entirely
about the **fragment graph** — "duplicates parents, children, ownership, and
ranges already visible in Markdown", "raw Markdown would no longer be sufficient
to reconstruct the code". Every clause survives contact with six books and is
restated as a positive invariant. But the validator's compiled-in `ROOTS`,
`BLOCKS`, `SLICE_ORDER`, `PAGE_BY_OWNER` and `EARLY_USES` are not the graph: they
are the **authoring contract** — which files must be reconstructed at all, who
owes which range, what the final page sequence is. That is a second question the
rejection never reached, and one book could avoid asking it by compiling the
answer into Rust. Six books cannot.

**2 · Scoped proof is what makes the contract undeducible from the book.**
`--through <slice>` compares what exists against what is *planned*: it requires a
defer, with the exact ID, owner and range, for every later-owned block, and it
requires the present pages to be an exact prefix of the final sequence. A plan is
by construction a statement about artifacts that do not yet exist, so it cannot
be derived from them. This argument is independent of book count and of format,
and it is why "delete the constants and read `source-index.md`" is not available.

**3 · The contract lives in a per-book TOML manifest,
`docs/walkthroughs/<book>/walkthrough.toml`.** The format the original document
rejected by name is adopted, deliberately, for a different subject — choosing a
different serialization to avoid the appearance of reversal would be dishonest
about what changed. TOML because `serde` derive makes the two validator leaves a
declarative parse rather than a table scrape, because array-of-tables is the
shape of roots, blocks and pages, and because the ownership plan carries
reasoning that needs comments. Named `walkthrough.toml` rather than `book.toml`
to avoid colliding with mdBook's file of that name.

**4 · The corpus boundary gains a filesystem witness, which is a stronger
control than the one it replaces.** Today
`compiled_corpus_copy_matches_the_normative_spec_tables`
(`crates/book-validation/src/validator.rs:1501`) `include_str!`s this spec and
compares its normative tables with `ROOTS`/`BLOCKS`. That is the only thing
making "complete reconstruction" a claim about an externally stated corpus rather
than a self-declaration, and moving the data into the book directory deletes it.
Replacement: the manifest declares the corpus **rule** — include globs, explicit
additions, explicit exclusions each with a reason — and the validator requires the
declared root set to equal the set derived from the real filesystem. Two
hand-written lists that can be wrong together are replaced by a list checked
against the tree it describes.

Verified against the frozen corpus before adopting it. `<crate>/Cargo.toml` plus
`<crate>/src/**/*.rs` reproduces the root brief's per-deliverable counts exactly —
`grove` 3/204, `grove-llm` 4/1,017, `jj-workspace` 4/698, `keyed-launch` 9/2,073,
and `grove-loop` 14/12,213 which is 13/10,533 after the one named exclusion
(`task_grow/tests.rs`, 1,680 lines). Exactly one inline test module exists across
the five campaign crates. The relocated book is the same rule with five
exclusions (`fixtures.rs` and four `*/tests.rs`, 3,273 lines) and one addition
(`bin/syllabus.rs`), which is why the rule is per-book data and not a constant.
The exclusion class is coherent rather than arbitrary: inline test modules and
test-support modules are evidence, not production source — intake field 2 of
`linkuistics:writing-code-walkthroughs` asks for exactly that classification.

**5 · The Markdown tables stay, as derived indexes, and `F009` keeps its
meaning.** `source-index.md` continues to carry the source-root directives and
all four tables. The trust order is stated and unchanged in shape: the manifest
is the contract, the directives are the execution, the tables are the derived
index, and a disagreement anywhere is a finding. What changed is only that the
contract moved from `validator.rs` to a file beside the book.

**6 · *Intended outcome* property 3 is amended, not dropped.** Reconstruction
stays Markdown-only: expansion reads the book's Markdown and the source files and
nothing else, the manifest contributes zero bytes, and a reader with the book and
the crate can still reconstruct every byte with no tooling. What the manifest
carries is the **obligation**, never the content. The property is rewritten to
say that, and the walk-away claim is stated as its own invariant so a later
session cannot weaken it by adding one convenient field.

**7 · The spec is renamed in place to `docs/specs/walkthrough-books.md`.** The
records doctrine says edit in place rather than write a second overlapping
document, and a book-system spec under a one-book slug misdescribes its own
subject. `CONTEXT-MAP.md`'s record-ownership list does **not** name the current
slug — checked, it names neither `ordinal-fs-tree-book` nor any walkthrough
record, so the record has had no owner since it was written. The rename fixes
that by adding the row rather than editing one. Owner is the **grove** context:
the `skills` glossary's own scope boundary excludes a skill's subject matter, and
ownership follows the validator to the `writing-code-walkthroughs` skill if and
when it moves — the transition `root-lifecycle-belongs-to-the-store` already made.

**8 · The guide-link contract: one link per book, declared anchors, and a
widened target whitelist.** Every book links `docs/USAGE.md` from exactly one
place — its `README.md` reader contract — so the anchor coupling
`walkthroughs-k3` decision 5 was bounding is one line per book rather than one
per page. A book may cite the guide with no anchor freely; citing a heading
requires the anchor to be listed in that book's manifest **and** to exist as an
explicit `<a id="…">` in `docs/USAGE.md`. Checked: `docs/USAGE.md` carries
**zero** explicit anchors today, and the book link contract currently forbids
every target outside the book and its source roots — so both the guide's anchors
and the whitelist are real work, not restatement. Decision 7 of `plan-k1` also
requires books to link grove vocabulary to the glossary, and `CONTEXT.md` has
zero explicit anchors too, so `CONTEXT.md` joins the whitelist on the same terms.

**9 · The relocated `ordinal-fs-tree` book is *not* brought into guide-link
conformance, and no leaf is cut for it.** Its audience is a reader proficient in
Rust and OS APIs who need not have driven a grove; `CONTEXT-MAP.md` holds
`ordinal-fs-tree` as a separate bounded context whose walkthrough is
"self-contained". A mandatory link from that book into the `grove` binary's user
guide would assert a dependency the context map denies. The manifest therefore
carries the guide link as declared per-book data with a stated reason for its
absence, and the rest of the shared spec binds the relocated book unchanged. Its
`walkthrough.toml` is written by `validator-structure-k21` as part of making the
validator read data, so there is nothing left for a conformance leaf to do.

**10 · The manifest is authored whole once, and the two validator leaves change
only which groups are read.** `validator-structure-k21` owns the page, slice and
path groups; `validator-fragments-k22` owns the root, block and early-use groups.
If each wrote its own half, a partial manifest would sit on disk between them and
`--check all` could not distinguish it from a malformed one. So the first leaf to
land authors the complete file against the schema; the second leaf deletes
constants and starts reading groups that were already there. A manifest is valid
when every group the validator reads is present.

**11 · Assets stay an explicit hole.** No `[[figure]]` or `[asset]` group is
defined, and the absence is recorded as deliberate with `figure-contract-k18`
named as its owner. An empty group would be machinery ordered ahead of the pilot
measure that is supposed to justify it.

**12 · The corpus control is bridged, not dropped, and the bridge says so.**
Deleting the old spec breaks `compiled_corpus_copy_matches_the_normative_spec_tables`
at compile time — an `include_str!` of a file that no longer exists — and this
leaf owes a green `scripts/check.sh`. Keeping one book's normative tables in the
shared spec to satisfy it is the one-book design the leaf exists to remove;
deleting the test drops the only corpus control for two leaves with nothing
tracking it. So it is re-pointed at `docs/walkthroughs/ordinal-fs-tree/source-index.md` —
the same rows, in the artifact that now holds them — renamed
`compiled_corpus_copy_matches_the_book_ledger_tables`, and its doc comment states
plainly that it is weaker than what it replaces (both sides are one book's account
of itself), names `validator-fragments-k22` as the leaf that deletes it, and names
the corpus-derivation check as what restores the external witness. Verified as a
control rather than assumed: mutating one ledger line count turns it red, and
restoring it turns it green.

**13 · The old spec's ownership table was a frozen plan; the book's is a live
progress record.** Re-pointing the test surfaced this — every ownership row
matched except the State column, because the spec froze "the state after
`orientation-k11`" (mostly `deferred`) while the finished book reads `resolved`
throughout. State is progress, not corpus, and `BLOCKS` never carried it, so the
comparison now drops that column. This is incidental evidence for decision 1: the
spec was carrying a plan, the book carries execution, and the two were only ever
equal at one instant. The manifest is where a plan belongs.
