# jj-workspace-structure-k17

## Goal

Elicit, from the human, the structure brief for the `jj-workspace` book:
audience refinements, conceptual order, what deserves emphasis, and what the
reader must be able to do at the end. Commit it as an input artifact.

## Context

- Decision 15 of `plan-k1`: the source does not contain enough to structure a
  walkthrough — audience, conceptual order and emphasis are nowhere in the code —
  so every book takes a human-authored structure brief. **Do not invent one.**
  This leaf exists precisely so a session does not.
- What is already settled and must not be re-elicited: the audience (decision 7 —
  knows Rust and jj, has driven a grove, grove vocabulary linked and never
  re-taught), the depth (decision 1 — complete source-exact, every byte in a
  fragment graph), the corpus (the root brief's table: four roots, 698 lines),
  and the method (`linkuistics:writing-code-walkthroughs`, whose eight-field
  intake `plan-k1` already answered). What is open is this book's *shape*.
- The interesting tension to put to the human: `CONTEXT-MAP.md` argues that
  `jj-workspace` is deliberately **not** a bounded context — every term in it is
  Jujutsu's, and what the crate adds is a namespace it will not name for its
  consumer. A book has to decide whether that argument is its spine or a footnote.
- The existing `ordinal-fs-tree` book's page sequence is the shape to react
  against: orientation, then one concept per chapter in reader-dependency order,
  then an assembly-and-trade-offs close, with a concept index and a source index
  as lookup surfaces.

## Done when

- A committed structure brief for the `jj-workspace` book states, in the human's
  own words: the chapter sequence and what each chapter is *for*, the concepts
  that carry the book, what to emphasise and what to pass over, and the
  reader-facing outcome.
- Where it lives is settled and consistent with `docs/ARCHITECTURE.md`'s
  ownership table — a structure brief is an input to a book, not a second
  description of the crate.
- `bash scripts/check.sh` passes.

## Notes

**This is a HITL leaf and the loop stalls here by design.** An agent answering
its own structure questions has broken the distinction the whole leaf exists for.
If the human is not available, stop and say so rather than proceeding.

**This is also the manual form of P4.** How to capture such input generally is
what `specification-capture-k4`/`k5`/`k6` are surveying; nothing here waits on
that, and nothing here should anticipate its answer.

## Decisions (running log)

**1 · The spine is subtraction — "jj already owns it".** Every chapter opens on
something the crate refuses to own and names who owns it instead. `CONTEXT-MAP.md`'s
argument — that this is not a bounded context because it will not name its
consumer — becomes the namespace chapter's thesis and is restated in the close,
rather than being the whole book's proposition. Chosen over the literal
context-map spine because that claim bites in only two places (`control_dir` and
`Refusal` naming jj's remedies), leaving roughly half the corpus to be bent to
fit; and over a plain layer order because the crate's own doc comments are
already organised as a list of refusals — three headed theses in `lib.rs`, one in
`Cargo.toml`, two in `jj.rs`, one in `refusal.rs`. The spine is recovered from the
source, not imposed on it.

**2 · Seven pages, six source-owning, cut on source coherence.** Orientation,
the gate, the subprocess seam, the namespace, scope and commit, refusal, and a
final-only close. Each source-owning chapter is a contiguous region of the corpus
whose thesis is one of the six refusals; `lib.rs` splits five ways and the other
three roots are whole-file blocks. Chosen over one-chapter-per-refusal (nine
pages, two of them owning under 50 lines) and over a five-page compression (two
chapters carrying two unrelated theses each, and the spine no longer visible in
the contents page). The order is narrative rather than strictly bottom-up:
`refusal.rs` is sixth, not second, so every refusal is met at the site that
creates it before its ten variants are read as a set — the same reason the
precedent put `error.rs` sixth of eight rather than first.

**3 · One command, two endings, and it is grove's real one.** The carried
example is `Workspace::resolve` from a subdirectory, `control_dir("grove")`, a
path-scoped `commit` of one `.grove/` task file, and the change id read back —
low resolution in chapter 1, full resolution in chapter 5, and chapter 6
retracing the *identical* attempt where `jj commit` declines, ending in
`CommitNotRecorded` and jj's own `jj undo` / `jj op log` remedy instead of a
`Commit`. Chapter 2 carries the same starting tree with no `.jj/`, chapter 3 the
exact argv jj is spawned with, chapter 4 the reserved `"grove"` beside a refused
`"repo"`. The example is grove's actual usage — `crates/grove-loop/src/driver_lease.rs`
reserves the literal namespace `"grove"`, and `crates/grove-llm/src/cli.rs`
resolves and commits — chosen over a consumer-free scratch tree despite the
apparent tension with the crate's thesis, because watching `"grove"` be passed
**in** as an ordinary string argument is what makes *it will not name its
consumer* checkable rather than asserted. Rejected: the refusal as the carried
trace (a one-step example stretched over six chapters, with the crate's purpose
as the aside) and a secondary-workspace setup (the atypical case as the spine).

**4 · jj is explained at the point of dependence, never as background.** Decision
7 settles that the reader knows jj, but six of the seven chapter theses rest on a
specific jj behaviour, and a book that states none of them turns its arguments
into unfalsifiable assertions. So each behaviour is stated once, in the chapter
whose refusal rests on it, as that chapter's premise — one paragraph, linked to
jj's documentation, and never a primer section. Chapter 2 states the pointer-file
shape of a borrowed `.jj/repo`; chapter 3 states that jj selects its repository by
walking up from the working directory and that `JJ_*` configure the user rather
than the repository; chapter 5 states working-copy snapshotting, the operation log
as the transaction record, and that a change id survives rewrites. Rejected: a
single jj-model section in orientation (five premises read before any of the five
arguments needing them — the catalogue before the example), and assuming all of it
(which would strip the one *measured* claim in the crate, `is_tracked` on jj
0.44.0, of the context that makes it a measurement rather than a preference).

**5 · The prose adjudicates, narrates and supplies the consumer's half.** Measured
before deciding: 37% of this corpus is comment prose — `Cargo.toml` 54%, `lib.rs`
47%, `jj.rs` 39% — and most of `refusal.rs`'s remainder is user-facing remedy text
inside string literals. The standing risk is therefore paraphrase, not obscurity,
because the fragment graph quotes the argued comment verbatim on the page anyway.
Three things a comment structurally cannot do are what each chapter's prose owes:

- **Adjudicate.** For every argued claim, name the jj behaviour it rests on, the
  test that proves it, and the alternative rejected with what it would have cost.
  A doc comment cannot cite an integration test three files away.
- **Carry the through-line.** Trace one operation across four roots and show where
  a decision in one file is only explicable by a decision in another — `resolve`
  canonicalising the root is *why* `relative` must canonicalise the caller's path,
  and canonicalising the **parent** is why a just-deleted path is still committable.
  No single comment holds that argument.
- **Supply the consumer's half.** The crate refuses to know its consumer, so its
  comments can never say what a call is *for*; the book says what grove does with
  each one.

Rejected: faithful exposition — cheapest and most uniform with an already-argued
corpus, but it would leave the developmental and technical edits almost nothing to
change, weakening the very measurement `pilot-k12` exists to take.

**The third carries a tension the human accepted deliberately**, and it needs a
convention: naming grove in every chapter can blur the boundary the spine is built
on. Proposed resolution, for confirmation at the close — the consumer's half is
always **marked as the consumer's**, set apart from the crate's own vocabulary, so
a reader can see at a glance which sentences the crate could have written and
which it could not.

**6 · The reader-facing outcome is the transferable subtraction test.** At the end
the reader can take a boundary in their own code, ask what the tool underneath
already owns, and tell a justified subtraction from an abdication — the closing
chapter states that test and applies it to all six of this crate's refusals. Ruled
out by fact rather than by preference: *take this crate as a dependency* is not an
available outcome, because `docs/RELEASING.md` (*One release, six packages, one
tag*) settles that only `crates/grove` is released and the five library members
"ship inside grove's cut, wearing grove's version", with no release lane of their
own. The maintainer outcome — locate any behaviour, name the invariant a change
would break, name the test that catches it — follows for free from source-exactness
and is therefore not the *stated* goal; the reference-card outcome was rejected
because a source-exact book is a poor way to deliver a reference card for a
fourteen-line public surface.

**7 · The wider outbound set: three guide anchors and four glossary anchors.**
Half of this was closed by the contract rather than chosen: `walkthrough-books.md`
(*Outbound links*) requires a book whose subject is a Grove runtime crate to link
`docs/USAGE.md` from exactly one place, its `README.md` reader contract, and the
`[guide] omitted` escape is specifically `ordinal-fs-tree`'s — justified by a
bounded-context status `CONTEXT-MAP.md` explicitly denies for `jj-workspace`. What
was chosen is the breadth. Guide: `usage-workspace-layouts` (the required README
citation, and the anchor whose subject is exactly this crate's resolution cases),
`undoing-a-mistake` (chapter 6, beside `CommitNotRecorded`'s `jj undo` / `jj op
log` remedy) and `usage-driver-lease` (chapter 4, beside `control_dir("grove")`).
Glossary: `task-commit-boundary`, `driver-lease`, `stated-vcs` and
`loop-control-channel`. The asymmetry is worth recording — all three guide anchors
already exist in explicit `<a id="…"></a>` form (`docs/USAGE.md` lines 737, 154 and
716), while `CONTEXT.md` carries **none**, so this book is the first-citing book
the specification puts that obligation on: `jj-workspace-book-k25` owes four
explicit anchor lines in `CONTEXT.md`, outside the book. The minimal set (one
guide anchor, three glossary) was rejected in favour of navigation out of the
consumer's-half passages decision 5 puts in every chapter.

**8 · Slice IDs carry no Grove task key, and are named for the refusal.** The
human's correction: those keys are not relevant to the books. The precedent's
`orientation-k11` … `book-assembly-k18` are the task keys of the grove that
produced the first book — one work item per chapter — and
`docs/specs/walkthrough-books.md` already records that spelling as *historical*
and the values as meaningful because the manifest enumerates them. A book-local
`jjw-` prefix was the same mistake in different clothes: a token shaped like a
handle `resolve` could find, which it is not. The property that must survive is
the specification's own stated reason for a separate slice domain — a
presentation-level page rename must not become a fragment-ownership, ledger,
manifest and CLI migration — and it is bought here by making the two vocabularies
differ **by construction**: pages are named for their subject, slices for the
refusal each chapter carries. `no-dependencies`, `one-lane`, `nothing-ambient`,
`no-consumer-vocabulary`, `no-transactions`, `no-remedy-of-its-own`, `assembly`.
No slice token equals any page ID, and the slice list reads as the book's spine.
