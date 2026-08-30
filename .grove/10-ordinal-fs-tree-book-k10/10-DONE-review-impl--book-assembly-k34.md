# book-assembly-k34

**Reviews:** book-assembly-k18

## Goal

Perform a fresh-context technical review of the complete committed
`ordinal-fs-tree` code-walkthrough book against its authoritative source,
executable evidence, formal models, repository contracts, coverage ledger, and
Grove requirements.

Read the whole book at `docs/ordinal-fs-tree/book/`, not only the producer
commit's added assembly page. Review the artifact as committed by
`book-assembly-k18`; do not edit files and do not run verification commands.

## Context

- Requirements: `.grove/BRIEF.md`,
  `.grove/10-ordinal-fs-tree-book-k10/BRIEF.md`, and
  `.grove/10-ordinal-fs-tree-book-k10/09-DONE-impl-book-assembly-k18.md` in the
  producer commit.
- Book-system contract: `docs/specs/ordinal-fs-tree-book.md`.
- Authoritative production corpus: the fifteen files listed in the node brief
  and in `docs/ordinal-fs-tree/book/source-index.md`.
- Technical design and vocabulary: `docs/ordinal-fs-tree/{CONTEXT,ARCHITECTURE,CLI}.md`,
  the ordinal-fs-tree ADRs registered in `CONTEXT-MAP.md`, and
  `docs/formalism-findings.md`.
- Executable evidence: the crate's unit, integration, CLI-contract, conformance,
  fault-injection, and no-filesystem-boundary tests.
- Formal evidence: `docs/ordinal-fs-tree/models/{structure.als,operations.qnt}`
  and their runners' stated claim sets and limitations.
- Mechanical coverage evidence: the complete fragment graph, ownership blocks,
  fragment index, early-use ledger, and the final assembly verification record
  in `08-invariants-and-trade-offs.md`.

Inspect technical accuracy, completeness, source-to-prose correspondence,
invariant scope, refusal/error classification, concurrency and recovery limits,
model attribution, CLI behavior, exact fragment ownership, and
self-containedness of required technical context. Distinguish source-backed
facts from model-backed evidence and from stated filesystem assumptions.

## Done when

- Every finding names an exact book path and line, states the contradicted
  source, test, model, document, ledger row, or requirement, and explains the
  technically correct replacement or missing content.
- The review checks all eight numbered pages plus contents, concept index, and
  source index; it does not treat the assembly page as the whole artifact.
- Findings are limited to technical accuracy and completeness. Clarity,
  ordering, cognitive load, repetition/link choices, and prose style belong to
  the later editorial review unless they make a technical statement ambiguous
  or non-self-contained.
- No repository file is edited and no verification command is run.
- If real findings exist, append an adjacent `integrate-review-impl` leaf with
  bare stem `book-assembly`, carrying `**Integrates:** book-assembly-k34`, and
  require it to verify and fix every finding, rerun final book validation plus
  the complete crate and relevant model checks, then commission the editorial
  reviewer as its final act.
- If no technical findings exist, append the editorial `review-impl` leaf
  directly with bare stem `book-assembly`. Its `**Reviews:**` line names
  `book-assembly-k18`, its body explicitly scopes the read to the whole book,
  and it reviews clarity, concept order, cognitive load, self-containedness,
  repetition/link choices, and the declarative prose contract without editing
  or running checks.

## Notes

The editorial reviewer must name the session that last wrote the book. If a
technical integration is created, that integration handle replaces
`book-assembly-k18` in the editorial leaf's `**Reviews:**` line. An editorial
integration leaf is created only for real editorial findings and reruns the
whole-book and crate checks after fixing them.

## Findings

Six findings, most severe first. The mechanical layer was inspected first and is
clean: all fifteen source roots expand byte-for-byte to their production files,
the 92 fragment definitions partition their parents gaplessly with matching
parents and owners, every fragment is reachable from exactly one root, no
`defer` remains, all 107 fragment-index rows agree with the directives, the
ownership table matches the specification's fixed blocks exactly, the owned-line
totals reproduce the specification's per-slice table and its 6,929 total, every
early-use row is `explained`, and every local link and anchor resolves. The
findings below are all things mechanical validation cannot see.

### 1. The recorded final verification command does not run the crate's doc tests

`docs/ordinal-fs-tree/book/08-invariants-and-trade-offs.md:223` records
`cargo test -p ordinal-fs-tree --all-targets` as the crate half of final
verification, and line 229 reads that command as covering the crate.

Cargo's `--all-targets` is `--lib --bins --tests --benches --examples` and
suppresses documentation tests. The in-scope corpus contains five doc tests,
all reproduced in the book: `src/lib.rs:45-63` (which *executes*
`conformance::check::<SyllabusName>` and `report.assert_conforming()`),
`src/name.rs:536-568` (the `compile_fail` control on obligation 4's discharge),
`src/name.rs:574-600` (its positive control, present because "a `compile_fail`
that fails for the wrong reason proves nothing"), `src/conformance.rs:15-22`,
and `src/fs/mod.rs:17-25`. None of them ran.

That contradicts the node brief's "the existing crate verification" criterion
and `09-DONE-impl-book-assembly-k18.md`'s "The crate's complete existing
verification ... passes". It also leaves three book claims without the control
their own source cites: `02-name-seam.md:492` ("The signatures discharge the
visible structural forms of obligations 3 and 4"),
`03-reference-domain.md:727` ("Rust discharges obligations 3 and 4
structurally"), and `08-invariants-and-trade-offs.md:96` ("Rust's `NameView` sum
type and the static `positioned_species` function make several inconsistent
shapes unrepresentable"). It is precisely the failure mode the book reproduces
verbatim on `01-orientation.md:99-103` from `Cargo.toml:34-42` — "a suite that
did not run reports what a suite that found nothing reports",
`docs/formalism-findings.md` entry 003.

Correct replacement: `cargo test -p ordinal-fs-tree`. The crate has no benches
and no examples, so dropping the flag is a strict superset of what
`--all-targets` runs and adds the doc tests. (`cargo test -p ordinal-fs-tree
--all-targets` plus `cargo test -p ordinal-fs-tree --doc` is equivalent.) Run it
and record the actual result rather than editing the command alone.

### 2. `03-reference-domain.md` has no `worked-reference-name` section

`docs/specs/ordinal-fs-tree-book.md`, *Worked examples*, is normative and
requires page `03-reference-domain.md` to carry the anchor
`worked-reference-name`, bounded as "One syllabus entry through parts,
formatting, parsing, species, and relevant conformance obligation."

The page's anchors are `syllabus-vocabulary` (:30), `rendered-names` (:281),
`classification-walkthrough` (:444), `seam-mapping` (:603),
`conformance-obligations` (:713), `findings-and-coverage` (:920),
`checking-the-seam` (:1043) and `reusable-samples` (:1439). There is no
`worked-reference-name`, and no section follows a single entry end to end — the
material is spread across `rendered-names` (formatting), the nine-row table in
`classification-walkthrough` (parsing), `seam-mapping` (species) and
`checking-the-seam` (obligations).

The final Markdown check passing is not evidence against this: the validator
checks anchor validity, uniqueness and link resolution only
(`crates/book-validation/src/markdown.rs:154-190` and `:296-340`); it holds no
list of required worked-example anchors.

Missing content: an anchored section tracing one entry — `02-draft-matrices-i6.md`
is already the page's running example — through
`Parts::lesson(Status::Draft, Label("matrices"))`, the `Display` arm
`{ordinal:02}-{status}-{label}-i{key}.md` (`src/reference.rs:246-249`),
`split_shape` returning `("02", "draft-matrices", "6")`
(`src/reference.rs:530-555`), the whole-grammar canonicity re-render
(`src/reference.rs:455-458`), `positioned_species` yielding
`PositionedSpecies::Leaf` and `Species::Leaf.requires() == Found::File`
(`src/name.rs:124-129`), and the obligation the kit exercises for that step,
`ParseRefusesWhatFoundContradicts` (`src/conformance.rs:542-636`).

### 3. Five literal fragments on page 06 have no preceding prose introduction

The specification's *Source-fragment introductions* rule: "Every literal
fragment's opening directive is immediately preceded by a prose paragraph ... No
other nonblank block may intervene."

Five literal fragments in `06-filesystem-interpreter.md` are immediately
preceded by `<!-- /fragment -->` instead:

- `lock-modes` at :150
- `lock-take` at :173
- `apply-plan` at :624
- `apply-run-state` at :667
- `error-taxonomy` at :1299

`error-taxonomy` is the heaviest case: 140 lines of the `Error<N>` enum — the
page's central taxonomy — introduced by nothing, and separated from its own doc
comment, which ends `error-boundary` at :1296. Every other numbered page
satisfies the rule for every literal fragment, so this is local to page 06.

Each needs a paragraph answering the five review questions the specification
lists (why here, whose behaviour, what input becomes what output, which
invariant, what role in the page's example).

### 4. Three fragment boundaries on page 04 cut a doc comment from its item

The specification requires descendants to be "intent-named"; these three land
mid-documentation, so a reproduced block opens with a sentence fragment and the
preceding block ends documenting an item it does not contain.

- `read-tree-discovery` (`04-read-path.md:87`, `src/fs/read.rs:1-86`) ends with
  the first five lines of `listing`'s doc comment. `read-directory-listing`
  (`04-read-path.md:199`, `read.rs:87-131`) therefore opens with the orphaned
  tail `/// filesystem rather than on the tree.` immediately above `fn listing`,
  under a paragraph that introduces `listing` as if its documentation were here.
- `read-directory-listing` ends mid-sentence inside `containing_directory`'s doc
  comment, so `read-lock-location` (`04-read-path.md:1240`, `read.rs:132-179`)
  opens with `/// accepted spelling ... reads the tree`.
- `snapshot-entry-views` (`04-read-path.md:581`, `src/snapshot.rs:268-440`) ends
  with `/// A level of the tree: the root, or a node.` and a bare `///`, so
  `snapshot-containers` (`04-read-path.md:783`, `snapshot.rs:441-532`) opens
  with the remainder of `Container`'s doc comment.

Correct boundaries are `read.rs:82` (`/// One directory's names and what is
under each, sorted.`), `read.rs:125` (`/// The directory whose lock covers this
tree ...`) and `snapshot.rs:439` (`/// A level of the tree: the root, or a
node.`). The specification's *Graph and expansion rules* permit exactly this:
the top-level blocks `read-filesystem-source` (1-179) and `read-snapshot-source`
(1-650) keep their IDs, owners and ranges, and only the descendant rows in the
fragment index change.

### 5. `07-syllabus-cli.md:99` names `Cli::parse`; the source uses `Cli::try_parse`

`bin/syllabus.rs:915` is `let code = match Cli::try_parse() {`, reproduced in the
book at `07-syllabus-cli.md:712`. `Cli::parse` is a different clap method: it
renders and exits inside clap, which is exactly what this binary refuses to do.
The same page states why, at `07-syllabus-cli.md` *Records, advisories, and
paths*: "Clap's help, version, and usage text also passes through `Streams`",
implemented at `bin/syllabus.rs:713-726` with its own comment that
`Error::exit()` "discard[s] print failures". The sentence sits in
`worked-cli-insert`, the section whose contract is to resolve the operation at
full source resolution.

Replace `Cli::parse` with `Cli::try_parse`.

### 6. Page 03's "stable example tree for later pages" is not the tree later pages use

`03-reference-domain.md:49` introduces its tree as "A stable example tree for
later pages". Only `04-read-path.md:28` reuses it. `01-orientation.md:23-38`,
`05-mutation-algebra.md:263-271`, `06-filesystem-interpreter.md:181-190` and
`07-syllabus-cli.md:81-91` use a different tree.

The two disagree on load-bearing values, not decoration: page 03's module holds
`01-published-vectors-i5.md` and `02-draft-matrices-i6.md` and the root holds
`03-draft-assessment-i9.md` (:58), so key 5 is at ordinal 1, key 6 at ordinal 2,
and the tree-wide greatest key is 9. Pages 01/05/06/07 add
`01-published-foundations-i3.md`, putting key 5 at ordinal 2 and key 6 at
ordinal 3, and drop key 9, so the greatest key is 6 — which is what makes the
worked insert allocate key 7 (`src/ops.rs:534-541`, `greatest_key` over the
whole snapshot). A reader who carries the tree page 03 calls stable into the
worked insert computes key 10.

Each later page does restate its own level locally, so no page is wrong in
isolation; the false statement is the cross-page one at :49. Correct
replacement: scope it to the page that actually reuses it ("the tree the read
path reads on the next page"), or reconcile the two trees.

## Decisions (running log)

No in-session reviewer was materialised: `references/execute.md` gives a
`review-*` session none, because the session is itself the adversarial read.

Mechanical claims in this review were established by reading the directives out
of the eleven book files and comparing the expansions against the production
files directly, not by running the project's validator or any crate command; no
repository file was edited and no verification command was run.

Findings exist, so the editorial review is not commissioned here. An adjacent
`integrate-review-impl` leaf carries `**Integrates:** book-assembly-k34` and
owns every fix, the reruns, and the creation of the editorial reviewer.
