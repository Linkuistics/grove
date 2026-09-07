# bin-target-privacy-claim-k87

## Goal

Correct the claim, made in three places, that *a binary target inside
`grove-loop` could reach that library's private items* —
`crates/grove/Cargo.toml` lines 15–19, `docs/specs/module-decomposition.md`
decision 1 (*a binary target can reach its own library's private items*), and
the *Command surfaces* residue of `docs/ARCHITECTURE.md` that restates it —
so that each names the shape the clause holds for, and land the one frozen-root
edit as a corpus change the book contract permits.

## Context

- Observed at `proving-a-negative-k80` under its in-session technical review,
  and measured there on a scratch package with one `[lib]` and one `[[bin]]`
  beside it: a binary that depends on the library and names a `pub(crate)`
  item through the library's path is refused with `E0603`; a binary that
  includes the library's source file as its own module (`#[path]`, or `mod` of
  a file it lists itself) compiles and reaches the same item. So the clause is
  true of a `[[bin]]` that compiles the loop's modules as its own crate, and
  false of a `[[bin]]` that depends on the library — which is the ordinary
  shape, and the one the comment's phrase *binary target inside `grove-loop`*
  reads as.
- The overview's chapter 1 (`docs/walkthroughs/overview/01-orientation.md`,
  *A crate, not a `[[bin]]` target*) now states both shapes beside the fragment
  that reproduces the comment, says which one the clause holds for, and says
  that the comment and decision 1 are reproduced as written. Once the comment
  changes, that adjudicating paragraph is wrong the other way and must be
  rewritten in the same commit; the same applies to chapter 4's table row about
  a `[lib]` on the `grove` package.
- One site is a frozen root: lines 15–19 of `crates/grove/Cargo.toml` are the
  overview's (`manifest-crate-not-a-bin`, lines 14–19). The other two are
  prose documents in no corpus.
- The property the manifest actually holds is narrower than its clause and
  still worth stating: a separate package cannot take the same-crate shape
  without a `#[path]` attribute pointing outside itself, which a reader of the
  two Rust files sees at once, and everything named through `grove_loop::` is
  a `pub` re-export whether the binary is a package or a target. Prefer a
  wording that names the shape over one that keeps the universal claim.

## Done when

- The three sentences state something a scratch package bears out, and the
  chapter-1 and chapter-4 paragraphs that adjudicate the clause are rewritten
  to match.
- The corpus-freeze rule in `.grove/BRIEF.md` is honoured: **one commit**
  carries the source change, every affected fragment and page of the overview,
  and a green `book-check --final` over it. A rewording that keeps the line
  count of lines 15–19 moves no boundary; one that changes it re-proves every
  range below it in that root.
- `bash scripts/check.sh` passes.

## Notes

**Placed after every crate book deliberately**, beside
`manifest-function-count-k82`, `grove-llm-version-comment-k83`,
`every-member-version-comment-k84` and `template-source-read-count-k86`, and
ahead of `architecture-residue-k75` because the `ARCHITECTURE.md` sentence is
residue that leaf may otherwise delete unread: editing a byte of a frozen root
while a book that quotes it is being written invalidates the ranges the
freeze protects.

## Decisions (running log)

**1 · The measurement was re-derived rather than taken from
`proving-a-negative-k80`.** A scratch package with `[lib]` and one `[[bin]]`
beside it: `privtest::secret()` — a `pub(crate)` fn named through the library's
path — is refused with `error[E0603]: function 'secret' is private`; the same
file rewritten as `#[path = "../lib.rs"] mod inner;` plus `inner::secret()`
compiles and prints `7`. The two runs differ only in the shape, so the passing
run is the failing run's positive control: one instrument, one dirty read and
one clean read.

**2 · The fix names the shape and keeps the universal clause's line count.**
`crates/grove/Cargo.toml` lines 15–19 become *a binary target inside
`grove-loop` that compiled the library's modules as its own would be the same
crate and could reach its private items*. Five lines in, five lines out, the
file still 54 lines, so `manifest-crate-not-a-bin` stays at `14-19` and no
range below it in the root moves. The alternative — stating the narrower
package-boundary property instead — was rejected because the clause the
comment is *making* is about the rejected alternative, and a comment that
stopped talking about it would no longer justify decision 1.

**3 · Decision 1 of `docs/specs/module-decomposition.md` gets the long form.**
It is in no corpus, so it has the room the manifest does not: it now names both
shapes, quotes `E0603`, and states that a separate package cannot take the
same-crate shape without a `#[path]` attribute pointing outside itself. That is
the site the manifest comment cites, so the full explanation lives there once
rather than in three roots.

**4 · `docs/ARCHITECTURE.md`'s *Command surfaces* residue is corrected in
place, not deleted.** `architecture-residue-k75` is last in this node and may
remove the sentence; this leaf's job is that it is not false in the meantime,
and correcting it costs four lines.

**5 · The overview's chapter 1 paragraph is rewritten, not cut.** It kept
everything it owned — the two shapes, the `E0603` measurement, the `#[path]`
argument — and its two adjudicating sentences flipped: *holds for one of the
two shapes* became *which is why the comment names that shape*, and *both are
reproduced as written* became *both name the shape the clause holds for*, with
the reason the manifest states it in one clause (five lines is its whole room)
and the note that this section is where the excluded shape is spelled out. One
sentence was added saying the depends-on shape is the *ordinary* one, which is
what makes naming the first shape necessary rather than pedantic.

**6 · Chapter 4's `[lib]` table row now says *which of the two shapes … that
property holds for*** and adds the `#[path]` clause, so the cell no longer
promises an adjudication chapter 1 has stopped making.

**7 · Chapter 5's evidence row 1 is left alone, and that is a finding, not an
oversight.** *A binary target that depends on the library cannot name a
`pub(crate)` item, and one that includes the library's source as its own module
can* is a claim about Rust, held by measurement and by no test; it was true
before this leaf and is what the leaf re-measured. *Nothing in the repository
builds either shape* is still true.

**8 · `docs/specs/overview-book-structure.md` line 252 is deliberately not
touched.** It quotes the old clause as the chapter's thesis, but its own
*Status and provenance* says *this document is authored, not recovered, and it
precedes its book*: it records what `overview-structure-k29` settled, and
editing it would make it false about what it asserts. Same treatment as the
evaluation records and the pre-authoring briefs elsewhere in this campaign.

**9 · A fourth site exists and is externalised to `k102` rather than fixed
here.** `crates/grove-llm/Cargo.toml` lines 11–12 carry the same universal —
*A binary target can reach its own library's private items* — and the
`grove-llm` book adjudicates it at `01-orientation.md` and
`07-what-order-holds.md`. This leaf's goal names three places; the fourth is
inside `manifest-crate-not-a-target`, lines 9–15, which is exactly the fragment
`grove-llm-dependency-comments-k102` must already rewrite. A separate leaf
editing the same seven lines a second time would make whichever ran second
re-derive the first's ranges, so k102's task file is amended to carry all three
clauses in that fragment.

**10 · The leaf's one in-session reviewer was spent on the corrected text, and
it found two defects in wording written this session.** Contract handed over:
every sentence about binary targets and privacy must survive a build the
reviewer runs itself, and every count, uniqueness claim and cross-reference in
the overview must hold against the source as it now stands. Nine findings, all
four classifications used.

**11 · Valid: *would be the same crate and could reach its private items* was
false, and decisions 2–6 above were written on it.** A `#[path]`-included module
is compiled into the **including** crate, not shared with the library: a type
declared that way is distinct from the library's, `E0308`, and rustc says
*defined in the current crate*. Re-measured here on a third scratch package. So
such a target is the same crate as *the code it includes* — a second copy — and
what it reaches is its own instance of the private items rather than the
library's. The frozen root now reads *compiled the library's modules into itself
could name the items that library keeps private*, which is true of the source
without asserting a crate identity that does not hold; still five lines, still
54 in the file. Decision 1 and chapter 1 carry the `E0308` fact explicitly,
because the honest form of the argument is that the shape defeats the
**source-level** discipline, and that is the only discipline *the binary is
thin* was ever about.

**12 · Valid: the package boundary makes the shape *visible*, not
*unavailable*.** Decision 3's first draft said *which shape a later commit takes
is not something review has to watch*. A `#[path]` can point outside a package —
three test targets in this workspace do exactly that
(`crates/{grove,grove-llm,grove-loop}/tests/support/mod.rs`, all reaching
`testing/support.rs`) — so the boundary forces the attribute and puts it in the
entry point's own files, and no more. Chapter 1's original wording was already
careful here (*which the two Rust files visibly do not carry*); the spec and the
architecture residue had overclaimed past it, and now say *visible rather than
unavailable*.

**13 · Valid, and it falsifies decision 7: a **fifth** site existed.** The
workspace root `Cargo.toml`, lines 9–12, carried the repudiated universal
verbatim while citing the decision that now contradicts it. Decision 9 above
enumerated four sites and counted wrong — the exact defect class this campaign
keeps finding. The root manifest is the workspace's and in **no** book corpus,
by the root brief's own exclusion, so it is both free to change and the right
home for the long form: it now carries both shapes, `E0603`, `E0308`, and the
visibility argument, and the three corpus sites state only what their room
allows.

**14 · Valid, and decision 7 above was wrong on its own terms.** Chapter 5's
evidence row 1 said *nothing in the repository builds either shape*.
`crates/grove-llm` is a package with a library and a `[[bin]]` beside it whose
`main.rs` calls `grove_llm::cli::run()`, and `crates/ordinal-fs-tree` ships
`syllabus` the same way under its `cli` feature: the depends-on shape is built
here twice. What is true is the weaker thing the row was reaching for — neither
names a private item, so no build here would go red if the claim's first half
were false — and the second shape over a library's own modules is built nowhere.
The cell now says that. This one was in reach of decision 7 and was asserted
rather than checked.

**15 · Valid, pre-existing, and fixed because it sat inside a paragraph being
rewritten.** Chapter 1 illustrated the same-crate shape as *`mod driver_lease;`
beside its `main`* at `src/bin/grove.rs`. Module resolution for a file under
`src/bin/` is relative to `src/bin/`, so that is `E0583`; the illustration needs
`#[path = "../driver_lease.rs"]` as much as a separate package does, and the
sentence two lines later rests on the contrast. The fragment is unaffected —
this is the book's own prose, not reproduced source.

**16 · Trade-off, held: `crates/grove-llm/Cargo.toml` stays inconsistent until
`k102`.** The reviewer is right that the repository now carries two manifests
giving different accounts of one Rust rule. The window is one leaf wide, inside
one node, and closing it here would mean two leaves rewriting the same seven
lines in succession — the second re-deriving the first's ranges under the
freeze. Decision 9 stands, and k102's task file now names the clause, its
measurement, and the chapter-1 paragraph that moves with it.

**17 · Noise: *both binaries are separate crates* in `docs/ARCHITECTURE.md`.**
The workspace declares three `[[bin]]` targets, `ordinal-fs-tree`'s `syllabus`
included, so the quantifier is not literally over every binary in the tree. It
sits in *Command surfaces*, whose subject is the two product surfaces and which
introduces them as such; the reviewer conceded the intended scope. Out of this
leaf's goal, and `architecture-residue-k75` is last in this node.

**18 · Noise, on the reasoning of decision 8: the structure brief keeps its
quotation.** The reviewer asked for at least a dated qualifier at
`docs/specs/overview-book-structure.md` line 252. The document's own *Status and
provenance* is that qualifier — *authored, not recovered, and it precedes its
book* — and it is the frame every quotation in it is read under.
