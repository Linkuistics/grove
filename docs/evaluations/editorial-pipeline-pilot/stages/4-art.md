# art — stage record

Stage 4 of six, and the fourth scored one. Its charter, from the
[preregistration](../preregistration.md)'s stage table: *whatever figures,
diagrams or tables plain Markdown can carry, and nothing else. No asset format,
no validator support.* It owns `A1` and `A2` and no other class.

`A1` and `A2` are the two taxonomy classes derived from no frozen standard —
art has none, and the preregistration says so. The book contract puts figures
and assets [out of scope](../../../specs/walkthrough-books.md) deliberately, so
this stage had exactly what Markdown gives it: tables, list figures, and fenced
text blocks. It used tables and nothing else. No asset was created, no manifest
group was added, and the validator's ignorance of figures is unchanged.

One structural rule shaped where every figure could land. `M105` requires a
**prose paragraph** as the nearest preceding nonblank block before every literal
fragment's opening directive, and names a table as one of the block kinds that
cannot stand there. So no table in this stage sits above a fragment: each is
placed at a section's end, or above prose that then runs into the fragment.

<a id="provenance"></a>
## Provenance

**Stage:** art (stage 4).
**Change id:** `kkktwzyx`.
**Parent change id:** `yxktpovq` — `copy-edit-k42: run the copy edit over the
jj-workspace book`, whose [`3-copy-edit.md`](3-copy-edit.md) fixes the state this
stage starts from.

| | Book-directory digest |
|---|---|
| before | `ca79d04d3f691bc3c48717e33b0469bc41da5d76996852d92586a1b169d8ee52` |
| after | `7dc08f7e213987e651c740ee522621429e547f25db8a0c38b90cb2b39e688a18` |

The before value is the copy edit's recorded final digest, re-computed at the
start of this session and unmoved. Digests follow the preregistration's recipe —
`find <dir> -type f \( -name '*.md' -o -name '*.toml' \) | LC_ALL=C sort | xargs
shasum -a 256 | shasum -a 256`.

**Preregistration read by this session:** SHA-256
`b9ee9f8e48b69170c7ecd9787b9373dfc42242895e91d37d8a181af6f09a694e` — the same
value the draft's children, the developmental edit, the technical edit and the
copy edit recorded.

<a id="frozen-inputs"></a>
## Frozen inputs, re-checked

Re-computed at the start of this session, before any edit. All nine match the
values the copy edit recorded.

| Input | SHA-256 as read | Verdict |
|---|---|---|
| `crates/jj-workspace/Cargo.toml` | `528fe93e62e0807b97a02d451ff04a86f1a1f0bd7f290de711684f3920c70936` | unmoved |
| `crates/jj-workspace/src/lib.rs` | `43ca4ad3947db34a326e5e85051617ab544bf5ee8d7df40fcc229597f7c1c9e3` | unmoved |
| `crates/jj-workspace/src/jj.rs` | `23eb9d98df0eb1906f25cfefd869c606d2750b99b703d35d0c533f5cba1f8af6` | unmoved |
| `crates/jj-workspace/src/refusal.rs` | `6ea88fc2678d7b1ad4f997adf42705699d0a9bda94b997d753902828409cdc3f` | unmoved |
| `docs/specs/walkthrough-books.md` | `d9556f93b1112fcda770aa4564c517a08b076ee241010132c7cf428abd86535d` | unmoved |
| `docs/specs/jj-workspace-book-structure.md` | `f7c9fa4ad87c79940dbb6d393d4cb03875641c17472e90f5b5eeb99327739182` | unmoved |
| `docs/USAGE.md` | `ed79be856e3427c1e56f48a33b34f9b4cd4111b7c9d9bdaa17dad9917556a59f` | unmoved |
| `CONTEXT.md` | `c0d4698440f2d925f6c67563487add708100cd8188be025267039be09b733822` | unmoved, at the post-draft value the draft record explains |
| `docs/walkthroughs/ordinal-fs-tree/` | `03c1fd3ebebffbf03202cfa5a597dae598293300c04a6abcaa4fb7199d5b7553` | unmoved |

<a id="what-was-read"></a>
## What was read, and what the survey found

All seven numbered pages, `README.md`, `concept-index.md` and `source-index.md`
were read in full before any edit, twice over: once for `A1`, looking for a
relation, sequence, partition or comparison carried only by running prose; and
once for `A2`, over every table, every non-fragment fenced block and every
diagram already in the book.

**The `A2` survey was nearly clean.** The book carries 45 non-fragment fenced
blocks — 39 `text`, 3 `console`, and the tree diagrams — and every one of them
but the case below has a lead-in or a following sentence stating what it is doing
in the page's argument. The worked examples are the reason: each is a narrated
sequence of endings, and the narration is the role statement. Three tables
existed before this stage and two of them are introduced. One was not, and it is
claim 7.

**The uniformity precedent was checked rather than assumed.** The relocated
`ordinal-fs-tree` book also uses tables sparsely and concentrates them in its
assembly chapter, with short lead-in sentences — *A syllabus has two positioned
species and one distinguished name:*, *Representative outcomes make the boundary
precise:*. Every table added below is introduced the same way, which is what
keeps `A1` work from creating `A2` defects.

<a id="claims"></a>
## Claims

Seven: six `A1` and one `A2`, over five pages. Each `A1` replaces or heads prose
that was carrying a relation; no fragment, no chapter, no section and no ledger
row moved.

| # | Class | Anchored subject | Before (verbatim citation) | After (verbatim citation) | What a reader gains |
|---:|---|---|---|---|---|
| 1 | `A1` | `02-the-gate.md#worked-resolution` | the section ended at *…Here it is `jj::output`; The subprocess seam owns both and shows how an invocation is assembled.* — the three endings appear only as three separately narrated traces, ~90 lines apart | *The three endings differ in one thing the ancestor walk never sees — the shape of `.jj/repo` …* followed by a five-column table (`Ending`, `.jj/ found`, `.jj/repo`, `jj spawned`, `Result`) and *The third row is the only one in which the two stored paths differ, and it is the only one in which anything is spawned.* | The chapter's premise — pointer file versus directory — and its cost, one subprocess, become one row-to-row comparison. The reader stops having to hold three traces to see that only the borrowed case spawns anything |
| 2 | `A1` | `03-subprocess-seam.md#the-selectors` | *The four are Git's, and each answers that question in its own way: `GIT_DIR` names the repository directory outright, `GIT_WORK_TREE` names the tree it belongs to, `GIT_COMMON_DIR` redirects the shared part of a repository split across worktrees, and `GIT_INDEX_FILE` names the index a command stages through.* | the same lead-in clause, then a two-column table with one row per variable | Four parallel items in one comma-run become four rows. The paragraph that immediately follows singles out `GIT_INDEX_FILE`, and the reader can now find its row rather than re-parse the sentence |
| 3 | `A1` | `04-namespace.md#the-reserved-list` | *…the two lists are worth reading together because they are lists of opposite things: that one holds names **Git reads**, and this one holds names **jj owns**. A reader who merges them in memory will mis-remember both.* | the same hazard sentence, then a four-column table (`Constant`, `Declared in`, `Holds names that`, `What the crate does with them`) with one row per list | The page states a memory-merge hazard and then asks the reader to hold the comparison in memory anyway. Two rows side by side is the form that answers it |
| 4 | `A1` | `04-namespace.md#the-validation` | *…the four that remain are exactly the ones that would break a clause of the postcondition: empty and `.` reach the administrative directory, `..` reaches the tracked tree, a separator escapes the component, and a jj-owned name collides with the system the whole crate is a seam onto.* | a five-column table (`#`, `Guard`, `A name it refuses`, what the `join` would have produced instead, and `Evidence`), then *Nothing downstream catches any of the four. `create_dir_all` succeeds on every path in the fourth column…* | The section's own heading promises *four refusals, in the order they run*, and the worked example had to state the order in a parenthetical about a guard that *sits between the second and the third*. The table is the order, the counterfactual path and the evidence per guard, in the sequence the code runs them |
| 5 | `A1` | `05-scope-and-commit.md#the-commit` | *The refusal from the commit itself means there is no commit: the working copy still holds everything the caller prepared… The refusal from reading the change id afterwards means the commit landed and could not be named: the work is committed, the tree is sealed, and what the caller lost is the identifier — retrying the whole operation would commit nothing…* | a four-column table (`Refusal`, `What it means`, `What is true of the tree`, `What a retry of the whole operation does`) with one row per refusal, then the unchanged sentence about wrapping | The page says a consumer that merges the two *loses work or duplicates it*. A two-row comparison puts the tree state and the retry outcome under the same headings, which is what makes them non-mergeable at a glance |
| 6 | `A1` | `06-refusal.md#the-case-analysis` | the section ended at *…because those three lines are one decision split across three implementations, and reading them apart is what turns a design into a catalogue.* — the ten kinds are then read in five sections, with `source()` membership stated once more, sixteen hundred words later, as *Four kinds have a cause* and *The remaining six return `None`* | a five-column table (`Kind`, `Group`, `Carries`, `source()`, `The remedy its message names`) with one row per kind, then two sentences pointing at where columns four and five are argued | This is the chapter's whole case analysis, and no section of it shows the shape. The reader can now see that four of ten carry a cause, that three of ten name no remedy at all, and which group each belongs to — the facts the five sections establish one at a time and never assemble |
| 7 | `A2` | `07-what-jj-owns.md#applying-the-test` | the six-row verdict table sat between the `## The test applied to all six` heading and the `### 1 · Every dependency` heading, with no prose on either side | *Both clauses, run over every row of the assembly table. This is the verdict and not the argument: each row is taken on its own below, in order, and the six sections that follow are where the two clause columns are shown rather than asserted.* | The book's most consequential table — the one that returns an `Abdication` verdict — had no statement of its role. A reader met six verdicts with no indication of whether the sections below repeated them or justified them |

**Every claim's before-text is prose or an absence, and every after-text is a
Markdown table with an adjacent role statement.** That is the whole of what this
stage produced: no asset, no image, no diagram format, and no change to the
manifest or the validator.

<a id="out-of-charter"></a>
## Out of charter

None. `A1` and `A2` are art's own classes and no other stage's charter contains
them, so every claim above is in charter by construction.

<a id="unclaimed-changes"></a>
## Unclaimed changes

None. This stage's diff is seven hunks over six files, and each hunk is one
claim above — hunk to claim, one to one, in file order: `02-the-gate.md`
claim 1, `03-subprocess-seam.md` claim 2, `04-namespace.md` claims 3 and 4,
`05-scope-and-commit.md` claim 5, `06-refusal.md` claim 6, `07-what-jj-owns.md`
claim 7. No hunk touches a fragment, a directive, a ledger row, a manifest, a
heading or an anchor.

The one edit inside a hunk that is not the table itself is claim 5's re-wrapping
of the sentence *The comment says exactly that, in two sentences, and the code
distinguishes them by wrapping one and not the other* onto two lines. Its text is
unchanged; the line fill moved because the paragraph above it did. It is inside
claim 5's hunk and is not a separate change.

<a id="findings-not-fixed"></a>
## Findings not fixed

Four. Three are `A1`s this stage decided not to draw and one is an `A2` the book
contract forbids fixing.

1. **`A1`, `01-orientation.md#public-surface`: the public surface is a catalogue
   of nine names in running prose.** Three types, four operations and two
   accessors, each with what it is and which chapter owns it, over six
   paragraphs — a partition with two attributes per member, which is exactly what
   a table carries. **Not drawn deliberately, for two reasons that compound.**
   Six of those paragraphs are the *First-use anchor* of an early-use row, and
   each carries that row's minimum local statement verbatim from
   [`source-index.md`](../../../walkthroughs/jj-workspace/source-index.md)'s
   ledger; compressing them into cells risks `D5`, which is the developmental
   edit's class and not this stage's to adjudicate. And
   [`1-developmental-edit.md`](1-developmental-edit.md)'s finding 3 already
   records this exact anchor as an unresolved conflict between two frozen
   standards — the structure brief's chapter-1 description against the two
   catalogue rules — so a table here would make the section more explicitly a
   catalogue at the one anchor where that is already contested.

2. **`A2`, `source-index.md`: none of the four index tables states its role.**
   `Source roots`, `Ownership blocks`, `Fragment index` and `Early uses` each
   follow their H2 heading with no prose. **The book contract forbids the fix**:
   *After the heading and one blank line, the header is the next line, followed
   by the separator and one or more data rows*
   (`docs/specs/walkthrough-books.md`, *Source and ownership ledger*), and a
   lead-in paragraph there is `F009`. This is recorded rather than claimed
   because it is a defect no stage of this pipeline can close, and because the
   contract's reason is good: the four tables are machine-reconciled derived
   indexes on a lookup page that is not part of the reading order.

3. **`A1`, `05-scope-and-commit.md#the-path-algebra`: `relative`'s branches are
   a decision tree in prose.** Relative or absolute, textual strip or
   canonicalised parent, the three `outside_workspace` detections, the empty-path
   refusal, and a coverage story in which one branch's succeeding end is untested
   — a route table with an evidence column would carry it. **Not drawn**: the
   section is already stepwise beside the five fragments it explains, and it
   carries an inline measured trace of the symlink-and-deletion case that a table
   would either duplicate or displace. A route table there would restate the code
   rather than a relation the code does not already show in order.

4. **`A1`, `06-refusal.md`: the book's unasserted claims are never assembled.**
   Chapters 3, 4, 5 and 6 each record something proved by measurement or by
   inspection rather than by a test — `NotRunnable` and `OutputNotText` reached
   by nothing, the fileset quoting characters, the workspace root as a scope, the
   canonicalised parent's succeeding end, both non-`/` separator characters. A
   whole-book evidence table would carry that partition. **Not drawn here**: it
   is a relation over the whole book rather than over any one page, and `P1` —
   whole-book coherence — is proof's class. Recorded for the proof
   stage rather than claimed.

None of the four is one of the seven the draft recorded in
[`0-draft.md`](0-draft.md)'s `## Findings not fixed`, and none is one of the four
the copy edit carried forward. The copy edit's finding 1 — chapter 4's
`.gitignore` mechanism restated in `07-what-jj-owns.md#the-uncomfortable-one` —
was left for proof and is untouched here: it is a `C4` repetition judgement, and
no figure of this stage's is placed in that section.

<a id="scope-and-validation"></a>
## Scope and validation

**One commit**, carrying the seven book-file hunks above, this record, and the
leaf's `.grove/` bookkeeping. No corpus file, no other stage's record, no
`CONTEXT.md`, no `docs/USAGE.md`, no manifest, and nothing else outside the
preregistration's allowlist.

**Final validation, green after the stage:**

```console
$ cargo run --quiet -p book-validation --bin book-check -- \
    --repo . --book docs/walkthroughs/jj-workspace --final --check all
valid: 4 files, 698 resolved lines, 0 deferred lines, final=true
```

No edit in this stage touched a fragment directive, a fence, or a source range,
so the reconstruction the draft proved is byte-identical afterwards: every change
is prose or a table between fragments, and `M105`'s structural predecessor rule
was the constraint that decided where each table could go.
