# book-outbound-links-k49

## Goal

Implement the specification's outbound-link contract: the `[guide]` citation
obligation on `README.md`, and the rule that every anchor a book *declares* in
`[guide]` or `[[glossary]]` exists in its target as an explicit
`<a id="…"></a>` line. Today the validator checks neither, and the manifest data
both need is parsed and discarded.

## Context

- Cut by `validator-structure-k21` after its adversarial read found the gap. That
  leaf introduced `walkthrough.toml` and made `[guide]` and `[[glossary]]`
  schema-checked at load — one of two shapes for `[guide]`, non-empty `anchors`
  — and then dropped both, because implementing their *checks* was new
  behaviour rather than the compiled-in-structure removal it was cut for. The
  data becomes available at exactly the point it is thrown away, which is why
  this is a leaf and not a note.
- **This is Markdown-contract work, not corpus work**, so it is not
  `validator-fragments-k22`'s and does not wait for it. The obligations are
  listed in `docs/specs/walkthrough-books.md`, *Markdown and link validator
  contract*, and specified in *Outbound links: the guide and the glossary*.
- The three rules, from *Anchors are declared, and the declaration is what is
  checked*:
  1. a book declaring a `[guide] path` must cite one of its declared guide
     anchors from its `README.md` reader contract; a `README.md` guide link
     carrying no anchor is `M201`;
  2. a citation naming an anchor not listed in that book's `[guide]` or
     `[[glossary]]` `anchors` is `M201`;
  3. a listed anchor the target document does not carry is `M201` **against the
     manifest**, reported once per missing anchor whether or not any page cites
     it — so a guide edit that removes an anchor fails the books that reserved
     it.
- The permitted local target set widens with this: *The link contract* names
  this book's declared pages, its declared source roots, the `[guide]` path and
  each `[[glossary]]` path — and the CLI must therefore load the declared
  outbound documents into the snapshot, which it does not do today. The
  *Fragment validator contract*'s description of `BookSnapshot` already names
  "the bytes of its declared outbound documents" as part of it.
- The relocated `ordinal-fs-tree` book declares `[guide] omitted` and no
  `[[glossary]]` entries, so rule 1's *absence* branch is what it exercises.
  A book that declares a guide path does not exist yet, so this leaf owes a
  synthetic fixture for the positive branch — `tests/second_book.rs` is the
  place, and it is already a book that shares nothing with the ordinal one.

## Done when

- `Manifest` retains `[guide]` and `[[glossary]]`, and the CLI loads the
  declared outbound documents into `BookSnapshot`.
- All three rules above are implemented, each **seen to fail**: a `README.md`
  citing the guide with no anchor; a page citing an anchor the manifest does not
  list; and a manifest listing an anchor its target does not carry.
- A book declaring `[guide] omitted` is `M201` if its `README.md` cites the
  guide anyway — the *absence* half of rule 1, which is what the relocated book
  exercises.
- `book-check --book docs/walkthroughs/ordinal-fs-tree --final --check all` is
  still green, and `bash scripts/check.sh` passes.

## Notes

**The obligation this discharges is the reason two sessions sit ahead of the
pilot.** `walkthroughs-k3` decision 5 ordered the guide ahead of the books so
books would not inherit unstable renderer-generated anchors, and the
specification says plainly that this check "is what makes that ordering mean
something rather than merely happen". Without it the ordering buys nothing: a
book can satisfy every link check while reserving anchors nothing guarantees.

**`docs/USAGE.md` and `CONTEXT.md` carry no explicit anchors today**, and this
leaf does not add them — the specification places those obligations on the leaf
that writes the guide and on the first book that cites the glossary. This leaf
implements the check; it does not make any existing document conform.

## Decisions (running log)

**The declared-anchor rule binds citations of the declared outbound documents,
and only those.** *Anchors are declared, and the declaration is what is checked*
reads, taken alone, as though every anchored citation must name an anchor listed
in `[guide]` or `[[glossary]]` — which the relocated book's own intra-book
citations (`01-orientation.md#insert-tour` and every navigation anchor) would
fail on the spot. The section is under *Outbound links: the guide and the
glossary* and declaration exists only for those documents, so the two conditions
are read as the contract for citing a declared outbound target. Anchors into the
book's own pages keep the existence check they already had, which `M102` backs
by proving every anchor in a book page precedes a heading.

**A declared outbound document that cannot be read is `U002`, not a finding.**
The specification's `U002` row already names an "outbound-document input" among
the required inputs, and the alternative is worse than an exit code: a document
that was not read carries no anchors, so every reservation over it would be
reported missing — a load failure dressed as a manifest that lied.

**Rule 3 is checked from the manifest and rule 2 from the page, and neither
repeats the other.** A citation of a *declared* anchor is not re-checked for
existence at the citation site; existence is reported once per reserved anchor
against `walkthrough.toml`. That is what makes a guide edit fail the books that
reserved the anchor rather than only the pages that happened to cite it, and
`never-cited` in the second book's fixture is the anchor that proves the
difference — reserved, cited by nothing, and a finding when the target drops it.

**The outbound documents are scanned, not parsed as book pages.** `parser::parse`
applies the book's own directive, fence and encoding rules and reports `P001`–
`P003` against whatever it reads; `docs/USAGE.md` and `CONTEXT.md` never agreed
to those rules. `markdown::explicit_anchors` is a fence-aware line scan that
answers the one question the contract asks of a target document: does it carry
this anchor, explicitly, immediately before a heading.

**The second book declares both an outbound guide and a glossary.** The
relocated book exercises only the *absence* branch, so the positive branch had
nowhere to live. `tests/second_book.rs` now materialises a `docs/USAGE.md` and a
`CONTEXT.md` in its temporary repository, cites the guide from its `README.md`
reader contract and the glossary from a chapter, and each of the six new tests
breaks exactly one condition of the green fixture.

**One limit of the absence branch, stated rather than closed.** `omitted`
declares no path, so the validator has no name for "the guide" and enforces the
absence through the permitted-target set alone. A book declaring `[guide]
omitted` *and* a `[[glossary]]` whose `path` is `docs/USAGE.md` would make the
guide a permitted target again and could cite it. Nothing in the specification
addresses that shape and no book takes it; closing it would mean either
compiling the guide's path into the validator — the thing this campaign spent
two leaves removing — or a new manifest rule, which is an amendment to the
specification rather than a fix here.
