# Editorial-pipeline pilot — preregistration

## Purpose and freeze boundary

This document is the instrument for the pilot that authors the `jj-workspace`
book by running six editorial stages by hand and asks which of those stages paid
for itself. It is written and committed by `pilot-preregistration-k24` **before
any page of that book is drafted**, and it is the reason the pilot is a
measurement rather than a report on activity.

Four things are fixed here and are not reopenable by any later session in this
campaign: the judged outcome, the alternative the six stages must beat, the
attribution rule, and the decision rule. A part of this document that turns out
to be unworkable is reported as a **protocol breach**, with the evidence, in the
measurement report. It is not quietly replaced with a better rule; a rule chosen
after seeing the data is not a preregistered one, and the whole value of this
artifact is that it precedes what it judges.

**One correction point precedes the freeze taking effect, and it has passed.**
This document was reviewed adversarially by `pilot-preregistration-k53` and
corrected by `pilot-preregistration-k54`, both **before** any page of the book was
drafted and both recorded here because `.grove/` does not outlive the campaign.
Seven defects were repaired: the non-independent probe fallback licensed the same
verdicts as a blind probe (*Part 2*, *Part 4*); diff verification proved the
recorded claims true but never complete (*Part 3*); de-duplication was optional
and so let a stage select its own claim count (*Part 1*); `Undetermined` licensed
the extraction it exists to forbid (*Part 4*); a partial probe left the
stage-level arithmetic undefined (*Validity*); five taxonomy classes were weaker
than the frozen standards they derive from (*Part 1*); and the comparator's bias
tally was arithmetically wrong (*Part 2*).

**That correction point is closed.** Every part below now stands as the frozen
instrument, and the paragraph above is the whole of what changed and when. A rule
that still proves unworkable is a protocol breach to report, exactly as stated —
not a licence for a stage or the report to substitute a better one.

The precedent this follows is
[`writing-code-walkthroughs`](../writing-code-walkthroughs/README.md) — a frozen
[rubric](../writing-code-walkthroughs/baseline/rubric.md) with its digest
recorded, atomic binary scoring, explicit invalid-run rules, and a *Verdict*
section that reads like an honest negative result. This pilot is much smaller and
its instrument is correspondingly lighter, but the discipline is the same one.

### What is being decided, and by whom

`pipeline-kinds-k27` extracts the publishing pipeline as installed
`grove-<kind>` skills. A kind exists only if a skill of that name exists
([a kind is an open token](../../adr/a-kind-is-an-open-token.md)), so the
decision this measurement feeds is exactly: **which of the six stages becomes a
skill, and therefore costs one session per book for the four books that follow.**

There is no human between this document and that extraction. Everything
`pipeline-kinds-k27` is entitled to conclude has to be legible in the report this
document specifies, **including the conclusions the report refuses to reach**.

### Where the artifacts land

| Artifact | Path | Written by |
|---|---|---|
| This preregistration | `docs/evaluations/editorial-pipeline-pilot/preregistration.md` | `pilot-preregistration-k24` |
| Draft stage record | `docs/evaluations/editorial-pipeline-pilot/stages/0-draft.md` | `jj-workspace-book-k25` |
| Developmental edit record | `docs/evaluations/editorial-pipeline-pilot/stages/1-developmental-edit.md` | `developmental-edit-k40` |
| Technical edit record | `docs/evaluations/editorial-pipeline-pilot/stages/2-technical-edit.md` | `technical-edit-k41` |
| Copy edit record | `docs/evaluations/editorial-pipeline-pilot/stages/3-copy-edit.md` | `copy-edit-k42` |
| Art record | `docs/evaluations/editorial-pipeline-pilot/stages/4-art.md` | `art-k43` |
| Proof record | `docs/evaluations/editorial-pipeline-pilot/stages/5-proof.md` | `proof-k44` |
| Proof-only probe | `docs/evaluations/editorial-pipeline-pilot/probe.md` | `measurement-report-k45` |
| Measurement report | `docs/evaluations/editorial-pipeline-pilot/README.md` | `measurement-report-k45` |

The paths are fixed here rather than left to each session, because
`pilot-measure-k26`'s brief says only "under `docs/evaluations/`" and a
destination a later session chooses is a destination it can choose to satisfy by
reinterpretation.

## Frozen inputs

An instrument adjusted mid-reading has read nothing, so every input this
measurement depends on is digested here as of the freeze commit. The measurement
report re-computes each digest and reports any that moved; a moved digest is a
protocol breach naming the file, not a reason to re-freeze.

### The corpus

Frozen by the root brief and by
[`jj-workspace-book-structure.md`](../../specs/jj-workspace-book-structure.md).
No session in this campaign edits it.

| Path | Lines | SHA-256 |
|---|---:|---|
| `crates/jj-workspace/Cargo.toml` | 44 | `528fe93e62e0807b97a02d451ff04a86f1a1f0bd7f290de711684f3920c70936` |
| `crates/jj-workspace/src/lib.rs` | 343 | `43ca4ad3947db34a326e5e85051617ab544bf5ee8d7df40fcc229597f7c1c9e3` |
| `crates/jj-workspace/src/jj.rs` | 81 | `23eb9d98df0eb1906f25cfefd869c606d2750b99b703d35d0c533f5cba1f8af6` |
| `crates/jj-workspace/src/refusal.rs` | 230 | `6ea88fc2678d7b1ad4f997adf42705699d0a9bda94b997d753902828409cdc3f` |

### The standards the stages are judged against

| Input | Role | SHA-256 |
|---|---|---|
| [`docs/specs/walkthrough-books.md`](../../specs/walkthrough-books.md) | the book contract, and the source of the prose taxonomy below | `d9556f93b1112fcda770aa4564c517a08b076ee241010132c7cf428abd86535d` |
| [`docs/specs/jj-workspace-book-structure.md`](../../specs/jj-workspace-book-structure.md) | the human structure brief: chapter sequence, theses, worked examples, early uses | `f7c9fa4ad87c79940dbb6d393d4cb03875641c17472e90f5b5eeb99327739182` |
| `docs/USAGE.md` | the guide whose anchors the book reserves | `ed79be856e3427c1e56f48a33b34f9b4cd4111b7c9d9bdaa17dad9917556a59f` |
| `CONTEXT.md` | the glossary the book links rather than re-teaches | `c7dde4d628a0f4cd8556b7c5238e1abe80d482b724f27efdcca4c9071e190e52` |
| [`docs/walkthroughs/ordinal-fs-tree/`](../../walkthroughs/ordinal-fs-tree/README.md) | the uniformity precedent the copy edit measures against | `03c1fd3ebebffbf03202cfa5a597dae598293300c04a6abcaa4fb7199d5b7553` |

The book-directory digest is the aggregate of its Markdown and TOML files, and
the recipe is stated so the report can reproduce it rather than guess at it:

```sh
find <dir> -type f \( -name '*.md' -o -name '*.toml' \) \
  | LC_ALL=C sort | xargs shasum -a 256 | shasum -a 256
```

### The stage charters, as they stood at freeze

The six stage task files live under `.grove/`, which the finish cycle deletes, so
their digests are recorded for drift detection **during** the campaign and their
operative content is restated in this document, which outlives them. The paths
are as they stood at the freeze commit; a leaf's position moves under `leaf-insert`
while its `<slug>-k<key>` handle does not, so the handle is the identity and a
moved path is not drift.

| Stage | Task file | SHA-256 |
|---|---|---|
| draft | `.grove/09-pilot-k12/04-impl--jj-workspace-book-k25.md` | `126bff6f0e5309077802563bee40806ae24e57d65d45618697dbca2c152453be` |
| developmental edit | `.grove/09-pilot-k12/05-pilot-measure-k26/01-impl--developmental-edit-k40.md` | `19bd4e837e76b4b7eddcb11f0e90ef9e39de9265ea9c2fbd2485c3fa750041d0` |
| technical edit | `.grove/09-pilot-k12/05-pilot-measure-k26/02-impl--technical-edit-k41.md` | `92d131d2f4a9afe99485c932dd4f20fd72418c28a633bcecbf426a524508740e` |
| copy edit | `.grove/09-pilot-k12/05-pilot-measure-k26/03-impl--copy-edit-k42.md` | `b206d9c0e51c4dc681ba78fadd07b22d2c94189b59dd6785e5474dea6b6ef40e` |
| art | `.grove/09-pilot-k12/05-pilot-measure-k26/04-impl--art-k43.md` | `0a51333b67c79cf8976064d01908440e89aa0379fdb05cfddd2b29c5c6ae6e40` |
| proof | `.grove/09-pilot-k12/05-pilot-measure-k26/05-impl--proof-k44.md` | `767af84afd822b44abe94bea844867bcc63046f89534c90478fa4ae4e0ce0318` |
| report | `.grove/09-pilot-k12/05-pilot-measure-k26/06-impl--measurement-report-k45.md` | `5c1781decf8a226bb05bbbd052954f91d127b2a9ad57e7df69df830fa103378a` |

### This document's own digest

This file cannot carry its own digest. The **measurement report records the
SHA-256 of this file as it stood at the report commit**, and every stage record
records the digest of the copy it read. Two records disagreeing, or either
disagreeing with the report, is a protocol breach: the instrument moved while it
was being read.

## The six stages and their charters

The stage list is decision 11 of `plan-k1`. Each charter below is the standard
the stage is held to and the boundary a **charter breach** is measured against.
They are stated here rather than only in the task files because the task files
die with `.grove/`, and because the proof charter is quoted verbatim to the probe
in *Part 2*.

| # | Stage | Charter |
|---:|---|---|
| 0 | **draft** | Produce the book from the corpus and the structure brief, to green final validation. Not an editorial stage; the unedited baseline the others are measured against. |
| 1 | **developmental edit** | Structure, conceptual order, what a chapter is for, and whether the book delivers the reader outcome the structure brief asked for. Structure, not sentences. |
| 2 | **technical edit** | Whether what the book says about the code is true, complete for its scope, and not silently stale against the frozen corpus. |
| 3 | **copy edit** | Sentences, terminology, consistency with the book contract's prose rules, and uniformity with the `ordinal-fs-tree` precedent. Does not restructure. |
| 4 | **art** | Whatever figures, diagrams or tables plain Markdown can carry, and nothing else. No asset format, no validator support. |
| 5 | **proof** | The final read for errors the earlier stages introduced or missed, and the last check that the book is whole. Whole-book scope; every class is in charter. |

## Part 1 — The judged outcome

### What a reader-facing improvement is

`book-check` proves structure and byte-exact reconstruction and **never proves
that a page explains anything**. That gap is what this part fills.

> A **reader-facing improvement** is the closing of one defect drawn from the
> frozen taxonomy below, at one anchored subject, evidenced by a citation into
> the state before the stage's commit that exhibits the defect's observable test,
> and by the corresponding text after that commit that does not.

Three consequences follow immediately, and each is deliberate:

- **A change that closes no taxonomy defect is not an improvement**, however
  sensible. It is recorded and it scores nothing.
- **A change with no before-citation is not an improvement**, because a claim
  with nothing to point at cannot be checked and cannot be wrong.
- **Mechanical properties are excluded.** Anything `book-check` already proves —
  fragment identity, reachability, coverage, byte equality, ledger
  reconciliation, link and anchor resolution — belongs to the validator, not to
  this measure. A stage that only kept the validator green did not improve the
  book for a reader; it kept the book legal.

### The taxonomy

The classes are derived from standards that were frozen before this pilot
existed: the book contract's
[*Prose contract*](../../specs/walkthrough-books.md#prose-contract) — its
[audience](../../specs/walkthrough-books.md#audience),
[self-containedness](../../specs/walkthrough-books.md#self-containedness),
[direct declarative style](../../specs/walkthrough-books.md#direct-declarative-style),
[local context and repetition](../../specs/walkthrough-books.md#local-context-and-repetition),
[source-fragment introduction](../../specs/walkthrough-books.md#source-fragment-introductions)
and [worked example](../../specs/walkthrough-books.md#worked-examples) rules — and
the structure brief's
[chapter sequence](../../specs/jj-workspace-book-structure.md#chapter-sequence),
[spine](../../specs/jj-workspace-book-structure.md#the-spine-what-the-crate-refuses-to-own),
[prose obligations](../../specs/jj-workspace-book-structure.md#what-each-chapters-prose-owes),
[worked examples](../../specs/jj-workspace-book-structure.md#worked-examples) and
[early-use ledger](../../specs/jj-workspace-book-structure.md#early-uses-the-order-forces).

Deriving the taxonomy rather than inventing one is what makes it credibly frozen:
these are the standards the stages are already told to work to, and no clause of
them was written with this measurement in view.

| Code | Class | Observable test — a defect exists when… | Charter owner |
|---|---|---|---|
| `D1` | Order | a concept is used before the page that owns it establishes it and no early-use row covers that use, or a section's position contradicts the structure brief's chapter sequence | developmental |
| `D2` | Catalogue before example | a section earlier than its chapter's worked example primarily enumerates three or more public queries, operations, verbs, refusal kinds, error variants, environment variables, validation rules or conformance obligations — the union of the two standards' enumerations, both of which bind | developmental |
| `D3` | Chapter purpose | a chapter does not open on the refusal its row in the spine table names, does not name who owns that responsibility instead, or states no thesis | developmental |
| `D4` | Stated outcome | the closing chapter does not state the transferable test, or does not apply it to all six refusals | developmental |
| `D5` | Self-containedness | one of the contract's four chapter questions cannot be answered from the page alone, or an early-use row's minimum local statement is absent at its First-use anchor | developmental |
| `T1` | Unadjudicated claim | an argued claim fails to name **any one** of the three its source standard requires of every argued claim: the jj behaviour it rests on, the test that proves it, and the alternative rejected with what it would have cost | technical |
| `T2` | False or stale claim | a statement about the corpus, about jj, or about a test is contradicted by the frozen source or by the cited test | technical |
| `T3` | Broken through-line | a decision explicable only by a decision in another file is presented without that connection | technical |
| `T4` | Consumer's half | a passage saying what a call is *for* is missing where the reader needs it, or is present but not set apart from the crate's own vocabulary | technical |
| `C1` | Terminology | a term drifts from jj's usage, from the glossary, or from the book's own earlier usage; or a grove term is re-taught rather than linked | copy |
| `C2` | Style | a rhetorical question, narrative suspense, metaphor, idiom, emotive or persuasive framing; or an actor, input, behaviour, result or invariant left unnamed where the distinction matters | copy |
| `C3` | Failure categories | modelled refusal, environmental failure, rollback failure and implementation defect are not labelled as different categories where the text distinguishes them | copy |
| `C4` | Links and repetition | a link label `M201` accepts still does not name its destination and purpose; a paragraph stops being intelligible when its link is removed; repeated context meets none of the contract's five conditions; or repetition meeting one of them exceeds the minimum semantic contract by restating a full earlier explanation | copy |
| `C5` | Fragment introduction | a literal fragment's introducing paragraph leaves one of the contract's five editorial questions unanswered | copy |
| `A1` | Undrawn relation | a relation, sequence, partition or comparison the reader must otherwise hold in their head is carried only by running prose, where a Markdown table, list figure or diagram would carry it | art |
| `A2` | Unplaced figure | a table, diagram or non-fragment code block has no adjacent statement of its role in the page's argument | art |
| `P1` | Whole-book coherence | two pages contradict each other, a cross-page promise is unkept, a ledger row never resolves, or a deferred hole survives in prose — a defect no single page's read reveals | proof |

`P1` is proof's own class and no other stage's charter contains it. Every other
class is **also** in proof's charter, because proof is a final read over the whole
book; proof cannot commit a charter breach.

**Each derived class has been reconciled clause by clause with the standard it
comes from**, and five were corrected against their sources before the draft
began. A derived class weaker than the obligation it derives from would let a
passage pass the taxonomy while breaching the standard the stages are actually
held to, and the taxonomy is not extendable once a stage has run — so this is the
last point at which the derivation can be made faithful. The corrections: `T1`
now fails on **any one** missing element rather than on all three, because the
structure brief requires an argued claim to name all three; `D3` now tests the
spine table's *Who owns it instead* column, which its source sentence states
coequally with the refusal; `D2` takes the union of the brief's enumeration and
the book contract's, since both bind this book; `C2` restores *behaviour* to the
contract's list of what a sentence identifies; and `C4` is scoped past `M201`,
because a non-descriptive label is already mechanical and mechanical properties
score nothing here. `A1` and `A2` are the two classes derived from no frozen
standard — art has none — and they are stated, not reconciled.

### Who judges, and how

The measurement report session judges, and its judgement is bounded three ways so
that it is verification rather than taste:

1. **Every claim carries a citation into the before-state.** The report locates
   that text in the stage's parent commit and locates its replacement in the
   stage's own commit. A claim whose citation is not in the parent, or whose
   text is unchanged by the stage's diff, is **rejected** and scores nothing.
2. **Every claim names one class, and the class's observable test is applied to
   the cited before-text.** A claim whose before-text does not exhibit the test
   is rejected.
3. **Claims are de-duplicated before counting.** Two claims by the same stage
   sharing a class and an anchored subject are merged into one. This is what
   stops a stage inflating its count by splitting one defect.

**One optional strengthening, reported either way.** If the report session can
materialise a second independent context, one stage's complete claim set is
re-adjudicated by it, blind to stage labels, and the criterion-level disagreement
count is reported. If it cannot, the report states *no re-adjudication was
performed*. This changes no verdict; it reports how reliable the adjudication was,
and its absence is an observation rather than a gap in the protocol.

### The unit: a claim

> A **claim** is one taxonomy class at one **anchored subject**, where an anchored
> subject is the nearest enclosing explicit anchor or heading on a book page, or
> the named fragment, early-use row or worked-example anchor the defect is about.

A defect of one class recurring at three or more anchored subjects across the
book **is** recorded as a single claim with the subject `book-wide`, listing its
instances. This is mandatory rather than permitted, and the equivalence that
triggers it is checkable: instances belong to one recurring family when **a single
sentence stating the correction applies to all of them without naming any
subject** — one term replaced by another throughout, one construction rewritten
the same way, one missing element supplied in the same form. Instances needing
different corrective sentences are different defects and stay separate claims.

**The report re-applies this collapse before counting and does not take a stage's
partition on trust.** Left optional, the threshold in *Part 4* would count
whichever subject partition a stage chose rather than three distinct defects. One
terminology drift fixed under three headings can be recorded as three `C1` claims
sharing no anchored subject; all three then survive pairwise de-duplication and
reach `Keep`, where the same family recorded as its one `book-wide` claim reaches
`Merge`. A count a stage can select by how it writes its record is not a
measurement.

A stage may not record the same class at the same subject twice.

The unit is a claim rather than a diff hunk deliberately. A copy edit correcting
one terminology drift in forty places would otherwise outscore a developmental
edit that closed four structural defects, and the decision would be made by hunk
arithmetic instead of by distinct defects closed.

## Part 2 — The alternative to beat

### The alternative

**Two stages: draft and proof.** The six-stage pipeline must be shown to deliver
reader-facing improvements that draft plus proof alone would not have delivered.

### Why an instrument is needed at all

The pilot runs **one arm**. Six stages run in sequence over one book; no second
book is drafted and proofed alone, and none will be. Without something standing
in for the two-stage arm, every middle stage's contribution reads as marginal by
default, and "six beat two" becomes unfalsifiable — which is precisely the
after-the-fact justification this whole leaf exists to prevent.

### The proof-only probe

The comparator is a **defect list a context produces from the draft under the
proof charter, having seen nothing else**.

**Producer.** An independent context materialised by `measurement-report-k45`.
This is the instrument the preregistration requires, not a discretionary review,
and it has first call on whatever independent context that session can obtain:
where only one is available it goes to the probe, and the re-adjudication in
*Part 1* is what goes unperformed.

**Inputs, and only these:**

- the complete text of the book as of the **draft** commit;
- the four corpus files, at the digests recorded above;
- the taxonomy table from *Part 1*, **with its Charter owner column removed** —
  that column names the pipeline the probe must not know about;
- the proof charter, quoted verbatim in the frozen prompt below.

**What the producer must not receive:** any state of the book after the draft
commit; any stage record; this document beyond the taxonomy table and the
prompt; the decision rule; the six-versus-two question; or the fact that a
pipeline is under evaluation.

**The frozen prompt.** The report session issues this text unchanged, and records
that it did:

```text
You are performing the proof stage over a code walkthrough book. Your charter is:
"The final read for errors the book contains or that its drafting missed, and the
last check that the book is whole." You have the book, the source files it
documents, and a taxonomy of reader-facing defect classes.

Do not edit anything. Produce a list of the defects you would fix under that
charter. For each one give: the taxonomy class code; the anchored subject (the
nearest enclosing anchor or heading on the page, or the named fragment, ledger
row or worked-example anchor it concerns); a verbatim citation of the text that
exhibits the defect; and one sentence saying what a reader loses to it.

List every defect you would fix, including small ones. Do not list anything you
would not fix. Do not propose improvements that close no listed defect class.
```

**The charter is quoted with one deliberate difference from the table above.**
Proof's charter there reads "errors the earlier stages introduced or missed",
because in the pilot four stages precede it. In the two-stage alternative there
are none, so the prompt says "the book contains or that its drafting missed". The
scope is identical; naming stages that do not exist in the arm being simulated
would tell the probe what it must not know.

**Output.** The report session commits the probe's list verbatim to
`docs/evaluations/editorial-pipeline-pilot/probe.md`, together with the exact
inputs supplied and a statement of what was withheld. The probe is committed
whether or not it is convenient.

**Fallback, preregistered with its trigger.** If the report session's harness
cannot materialise an independent context, the report session produces the probe
itself under this discipline: it reads the draft state of the book **and nothing
later**, writes and commits `probe.md`, and only then opens any post-draft state.
The report then annotates every verdict `probe not independent` and records that
the producer had read this preregistration, which biases the probe in an
undetermined direction. The trigger is mechanical and outcome-independent;
choosing the fallback because the probe's content is unwelcome is a protocol
breach.

**The annotation is not the only consequence, and cannot be.** A fallback probe
comes from a context that had read this document — the hypothesis, the threshold
and the decision rule — so its recall error is not merely unknown but
*interested*, and under-listing is the direction that manufactures marginality.
Under the fallback the comparator therefore **licenses no `Keep`**: a stage whose
arithmetic reaches the threshold is Undetermined instead, by the fallback clause
of the verdict table in *Part 4*. `Merge` and `Drop` stand, because those are the
directions in which a compromised comparator cannot buy machinery — the negative
result this campaign calls a success stays reachable, and the expensive verdict
does not.

The asymmetry is the point. A fallback that changed a verdict's epistemic status
without changing what that verdict licenses is not a fallback; it is an escape
hatch, and failing to obtain the capability the comparator requires would cost
nothing.

### Matching a probe entry to a stage claim

A probe entry `P` **reaches** a stage claim `S` when both hold:

- `P` and `S` name the same taxonomy class; and
- `P` and `S` concern the same anchored subject — the same explicit anchor,
  heading, fragment, ledger row or worked-example anchor — traced back through
  the intervening stage diffs to the draft state. A `book-wide` claim is reached
  when the probe lists two or more instances of that class anywhere in the book.

A claim reached by the probe is **not marginal**: the two-stage alternative gets
it, so the stage that made it is not credited with it.

### The biases, stated in advance, with their direction

- **The probe identifies; it does not fix.** It is scored as though everything it
  names would have been repaired. This inflates the two-stage arm and denies the
  middle stages credit — **against** the six-stage hypothesis.
- **A real two-stage proof session would have more time and a wider mandate than
  one probe pass.** This deflates the two-stage arm — **for** the six-stage
  hypothesis. It is the only bias that runs that way, and it is the reason the
  matching rule is deliberately generous to the probe.
- **Matching at class-plus-anchored-subject over-matches** rather than under-
  matches, since a probe entry and a stage claim can share both without being the
  same defect — **against** the six-stage hypothesis.
- **The probe is one context, unreplicated.** Its recall is unknown and there is
  no second probe to estimate it against. Direction unknown; reported as a limit.

**Two of the four run against the pipeline the campaign built, one runs for it,
and one has no established direction.** A six-stage result surviving this
comparator is therefore some evidence beyond a neutral comparison — but the
instrument claims no net conservatism, because it has not established one, and a
count of three would be arithmetic the list above does not support.

**Unknown recall is not a conservative unknown, and it is why *Part 4* is
asymmetric.** A defect the probe misses is a defect the two-stage arm never gets
credited with, so the matching stage claim reads marginal and the stage moves
*toward* `Keep`. Probe recall error therefore pushes in the expensive direction —
extracting a stage and paying a session per book for four books — while the two
biases that run against the pipeline push in the cheap one. Wherever probe
reliability is itself in question the instrument withholds `Keep` and leaves
`Merge` and `Drop` standing, rather than the reverse: that is the rule for the
non-independent fallback above. Where the doubt is about *coverage* rather than
about the producer, the partial-probe interval under *Validity* applies the same
instinct in the form that fits it, resolving a stage only where the missing
coverage cannot change its row.

## Part 3 — The attribution rule

### A stage owns its diff

**The five editorial stages each land in exactly one commit, and that commit's
diff against its parent is the complete record of what the stage changed.** No
judgement decides which stage made a change; the diff does. A session folding two
stages into one commit destroys the evidence this pilot exists to gather, and its
stage is Undetermined.

The **draft is the exception**, and only this one: `jj-workspace-book-k25` is
permitted to decompose into one child per slice, in which case the draft stage is
every commit from the first drafting commit to the last inclusive, and its
baseline is the commit before the first of them. The draft is not a scored stage,
so a range costs the measurement nothing.

**Resolving the commits.** Every task commit names its work item by its
`<slug>-k<key>` handle, so the report resolves the six boundaries by searching
descriptions for `jj-workspace-book-k25`, `developmental-edit-k40`,
`technical-edit-k41`, `copy-edit-k42`, `art-k43` and `proof-k44`. It records the
resolved change id and the book-directory digest at each, before and after the
reading.

### The stage record

Each stage commits its record at the path fixed above, in this shape. Records are
written **as the stage goes**, not reconstructed afterwards — reconstruction is
exactly what the one-commit boundary exists to make unnecessary.

```markdown
# <stage> — stage record

## Provenance
Stage, its change id, its parent's change id, the book-directory digest before
and after, and the SHA-256 of the preregistration this session read.

## Claims
| # | Class | Anchored subject | Before (verbatim citation) | After (verbatim citation) | What a reader gains |

## Out of charter
Claims above whose class this stage's charter does not own, listed by number,
each with why it was made here anyway.

## Unclaimed changes
Every remaining hunk of this stage's diff that touches book prose, by page and
anchored subject, each with why it closes no taxonomy defect — mechanical or
validator-driven, formatting with no reader-facing effect, or reader-facing but
deliberately not claimed. The claim table and this section together account for
**every** prose hunk in the diff, and a stage with none says so.

## Findings not fixed
Defects seen and deliberately not fixed — out of charter, blocked by the corpus
freeze, or belonging to a later stage — as class plus anchored subject, with no
citation of a fix.

## Nothing to record
Present only when the stage made no claims, stating whether it ran and found
nothing, or did not run.
```

**The draft's record differs in one section and only one.** It has no before-state
to cite, so it carries `## Baseline` — the page inventory, the book-directory
digest, and the final-validation result — where an editorial stage carries
`## Claims`. If the draft decomposes into one child per slice, the record is
appended to by each child and its `## Provenance` lists every commit in the
range.

### Crediting

- **A claim is credited to the stage whose commit contains it.** Provenance is
  the diff, not the record's assertion.
- **One claim, one class.** A change closing defects in two classes is recorded
  under the class whose observable test its before-citation most directly
  exhibits; where two apply equally, the earlier code in taxonomy order wins.
  Stated as a tie-break so it is not decided at reading time.
- **Out-of-charter claims are still credited to the stage that made them**, and
  flagged. They are not discounted — the work was done — but they carry a
  consequence for the pre-empted stage, in *Part 4*.
- **A finding not fixed scores nothing for the stage that found it.** The stage
  that fixes it is credited. This is what stops one defect being counted twice.
- **A regression is a claim with negative polarity.** An edit that *opens* a
  taxonomy defect, evidenced the same way in reverse, counts against the stage
  that made it. Regressions are found by the report from the diffs, not only from
  self-report, and a stage does not certify its own innocence.
- **The record accounts for the whole diff, and the report checks that it does.**
  A prose hunk appearing in neither the claim table nor `## Unclaimed changes` is
  **unaccounted**. The report classifies every unaccounted hunk itself under the
  taxonomy and credits it to the stage whose commit contains it — in particular as
  an out-of-charter claim where its class belongs to another stage's charter.

  This is what makes verification of the record a verification of *completeness*
  rather than only of truth. Checking each reported claim against the diff proves
  that what a stage said it did, it did; nothing in that check rejects a
  substantive hunk carrying no claim. Without this rule a developmental edit could
  record its one `D1` fix, silently make three `C1` fixes, and leave the copy edit
  reading as `Drop` — its work already done and its emptiness scored as
  worthlessness — because `b(S)` was never set for work the stage chose not to
  report. A stage's silence about its own out-of-charter work would otherwise
  delete the pre-empted stage's verdict.

### Art, which has no machinery behind it

The art stage produces Markdown and nothing else — no asset format, no figure
group in the manifest, no validator concept of an asset, all deliberately absent
until `figure-contract-k18` reports. The attribution rule must nonetheless be
able to credit it, or the decision rule drops art for being unmeasurable rather
than for being unhelpful.

**It can, and needs nothing new.** An art claim is a claim like any other: class
`A1` or `A2`, an anchored subject, a before-citation showing prose carrying a
relation it should not have to, and an after-citation showing the table, list
figure or diagram that carries it instead. The product is bytes in the book's
Markdown, which is exactly what a diff records. No asset machinery is required
for art to be measured, and building any would be machinery ordered ahead of the
measurement meant to justify it.

**Art producing nothing is data, not absence.** A stage that ran and found
nothing worth drawing records that in `## Nothing to record`, and the decision
rule treats it as a stage with zero marginal claims. A stage that was not run
records that instead, and is Undetermined. The report cannot tell those apart
unless the session says which happened.

## Part 4 — The decision rule

### The quantities

For each stage `S`, computed by the report from the verified claim set:

| Symbol | Definition |
|---|---|
| `v(S)` | claims verified under *Part 1* — citation located, class test exhibited, de-duplicated |
| `m(S)` | **marginal** claims: those in `v(S)` that the probe does not reach |
| `r(S)` | verified regressions attributed to `S` |
| `b(S)` | claims in `v(S)` whose class `S`'s charter does not own |
| `net(S)` | `m(S) − r(S)` |

Where the probe is partial, `net(S)` is not a scalar and the interval
`[net_lo(S), net_hi(S)]` defined under *Validity* is what the table below reads.

### Per-stage verdict

Applied to the four middle stages — developmental edit, technical edit, copy edit
and art — and to no others. First matching row wins.

| Verdict | Condition |
|---|---|
| **Undetermined** | the probe is missing; **or** the probe was produced under *Part 2*'s fallback and `S`'s arithmetic reaches `Keep`; **or** the probe is partial and `net_lo(S)` and `net_hi(S)` select different rows of this table, as *Validity* defines them; **or** `S`'s record is absent, or does not distinguish *ran and found nothing* from *did not run*; **or** `S`'s commit is invalid under *Validity* below; **or** `S` was **pre-empted** — `net(S) ≤ 0` and some **earlier** stage made a marginal out-of-charter claim in a class `S`'s charter owns |
| **Drop** | `net(S) ≤ 0` |
| **Merge** | `1 ≤ net(S) ≤ 2` |
| **Keep** | `net(S) ≥ 3` |

**The threshold is three, and it is a judgement made in advance rather than a
derivation.** A `keep` buys a `grove-<kind>` skill and costs one session per book
for the four books that follow. A stage closing fewer than three distinct
marginal defects across a seven-chapter book is contributing less than one defect
every two chapters — a volume a neighbouring stage absorbs without a session of
its own. Stating the number here is what stops it being chosen once the counts
are visible.

**The pre-emption row is what keeps absence honest.** If the developmental edit
also fixed the terminology drift, the copy edit's empty count is evidence about
the developmental edit's discipline, not about copy editing. Reading it as a drop
would let one stage's charter breach delete another stage.

**Only an earlier stage can pre-empt.** A stage cannot be excused by work done
after it: a `C1` defect that survived the copy edit and was fixed by proof is
evidence about the copy edit, not an obstacle to judging it. Proof runs last and
its charter owns every class, so without the ordering qualifier proof would
pre-empt every middle stage that happened to find nothing, and `N_max` would
reach six whatever the book contained.

**The merge target** is the stage in pipeline order whose charter owns the most
classes appearing in `S`'s out-of-charter claims; failing that, the nearest
surviving stage earlier in the pipeline; failing that, the nearest later one.
Deterministic, so the report computes it rather than choosing it.

### Draft and proof

**The draft is not scored.** There is no book without it and no alternative to it.

**Proof is the comparator, and a comparator cannot be tested by the comparison it
defines.** It appears in both arms by construction, and the probe *is* proof
applied to the draft, so no marginality is computable for it. Its contribution is
reported descriptively — `v(proof)`, its class distribution, and the split between
claims in `P1` and claims in classes an earlier stage's charter owned — and the
report must state plainly that `pipeline-kinds-k27` extracting proof rests on
proof being the stated fallback rather than on measured evidence.

The descriptive split is still worth reading. If proof's claims are almost all in
classes earlier charters owned, proof functioned as a net for other stages' misses
rather than as an independent contributor, and the report says so.

### The headline: the surviving pipeline

The **surviving pipeline** is `{draft}` ∪ `{middle stages with verdict Keep}` ∪
`{proof}`, with merged stages folded into their targets and contributing no stage
of their own. Its size is reported as a range:

- `N_min` counts established keeps only.
- `N_max` additionally counts every Undetermined stage.

If `N_min ≠ N_max` the verdict **is** that range, and the report says which
stages are unresolved and why. An Undetermined stage is never silently counted as
kept, and never silently counted as dropped. Where `N_min = N_max`, that value is
`N` and the table below reads directly.

**`N_max` measures unresolvedness; it licenses nothing.** It is how much of the
pipeline this pilot failed to decide, and the extraction table below is what
governs what may be built — where an Undetermined stage authorises no extraction
whatever the range's upper end reads. A report whose range is wide has said the
measurement did not resolve those stages, which is a different statement from
having kept them.

| Outcome | Reading |
|---|---|
| `N = 6` | the six-stage pipeline is upheld in full |
| `2 < N < 6` | an `N`-stage pipeline is what the evidence supports; neither the six-stage design nor the two-stage fallback is upheld |
| `N = 2` | the stated alternative wins: draft and proof are the pipeline |

**The middle outcome is the most likely one and is a legitimate result.** The
question is not framed as a binary because the pipeline is not a binary, and a
report forced to answer *six or two* would have to discard whichever stages did
not fit its answer.

### The negative result, and that it is a success

> **A negative result is `N_min = N_max = 2`**: no middle stage reaches `Keep`,
> and none is `Undetermined`. The probe reached what the middle stages found, or
> the middle stages found little, and draft plus proof is the pipeline this
> campaign should build.

**That is a successful outcome of this pilot, not a failed one**, and the report
states it in those terms. It saves four skills, four sessions per book across four
books, and the extraction of machinery nothing justified — which is the entire
reason the measure was ordered before the machinery. A report that treats a
two-stage result as a disappointment has misread its own charter.

The symmetric failure is worse, and is named too: a report that keeps a stage on
the strength of an `Undetermined` verdict has shipped six stages on the strength
of nothing.

### What each verdict licenses downstream

`pipeline-kinds-k27` reads the report and extracts kinds. What it may do with each
verdict is fixed here, because there is no human between the two.

**No row licenses extraction on an absence.** The root brief's governing rule is
that a stage which cannot be shown to have paid for itself is not extracted into a
kind, and the pilot brief repeats it. An `Undetermined` stage has not been shown to
have paid; letting the extraction leaf state its own basis and call that basis
"not this pilot's evidence" would satisfy the rule's words while producing exactly
the pipeline it forbids — six stages derived after the fact, which is the failure
this whole leaf was ordered to prevent.

| Verdict | What `pipeline-kinds-k27` may do |
|---|---|
| **Keep** | extract the stage as its own `grove-<kind>` skill |
| **Merge** | fold the stage's charter into its merge target's skill; do not give it a kind |
| **Drop** | do not extract it, and do not fold it in |
| **Undetermined** | **this measurement licenses nothing, and nothing in this campaign licenses it instead.** The stage is not extracted and not folded in. A genuinely new basis for it is a new preregistered measurement, not an assertion in the extraction leaf |

## Validity

### What makes a stage invalid

- Its edits span more than one commit, or share a commit with another stage
  (draft excepted, above).
- Its commit touches `crates/jj-workspace/`. The corpus is frozen; a defect found
  while documenting becomes its own leaf under the root brief's cross-book rule.
- Its commit touches files outside this allowlist: the book directory
  `docs/walkthroughs/jj-workspace/`; its own stage record; `.grove/` bookkeeping;
  and, for the draft only, `CONTEXT.md` anchors, `docs/ARCHITECTURE.md`'s
  ownership row, and the book inventories in
  [`walkthrough-books.md`](../../specs/walkthrough-books.md).
- Final validation over the book is not green at the end of the stage —
  `book-check --final --check all`, by way of `bash scripts/check.sh`.
- The stages ran out of pipeline order.
- Its record is absent, or its claims cannot be located in its diff.
- Its record leaves a prose hunk of its diff unaccounted — in neither the claim
  table nor `## Unclaimed changes` — and the report's own classification of that
  hunk finds a taxonomy defect closed or opened by it. The stage is Undetermined;
  the hunk is still credited under *Part 3*, so a stage that hid its own
  out-of-charter work cannot thereby drop the stage it pre-empted.

### What makes the reading invalid

- Any edit to the book, to a stage record, or to this document between the proof
  commit and the report. Finish every edit, then measure.
- The probe produced by a context that saw any post-draft state.
- Two writers appending to one output. One measurement, one writer.
- A re-run confirmed by matching totals rather than item by item. Two runs can
  agree on a total while disagreeing about which item did what, and where there
  is no per-item record a re-run confirms nothing.

### Replacement

A stage may be re-run **once**, and only for a mechanical invalidity — a
mis-scoped commit, a lost record, a red validator. The invalid attempt is
preserved and reported. **No stage is re-run because its result is unwelcome**,
and no stage is re-run after the probe has been produced.

### What an invalid or missing observation licenses

Stated in advance, so no arithmetic hole is filled at reading time.

- **A missing or invalid stage record** makes that stage Undetermined. It is
  never a zero, and never a keep. Absence of evidence is not evidence of absence,
  and it is not evidence of presence either.
- **A missing probe** makes *every* middle stage Undetermined, and **no
  six-versus-two verdict is licensed at all**. Marginality is undefined without
  the comparator, so the report presents per-stage class distributions
  descriptively and says, in those words, that the pilot did not measure what it
  set out to measure.
- **A partial probe** — one that terminated early or covers only some pages —
  licenses marginality only for claims whose anchored subject lies inside the
  pages the probe covered. Claims outside them are Undetermined, and the stage's
  arithmetic is then an interval rather than a scalar: `net_lo(S)` counts every
  uncovered claim as reached by the probe and so contributing nothing,
  `net_hi(S)` counts every uncovered claim as marginal, and both subtract `r(S)`,
  which does not depend on the probe.

  **The verdict is read from the interval.** If `net_lo(S)` and `net_hi(S)`
  select the same row of the per-stage table, that verdict stands and the report
  names the unresolved claims anyway. If they select different rows the uncovered
  claims are load-bearing, the probe did not cover them, and the stage is
  Undetermined. A stage all of whose claims fall outside coverage is the limiting
  case of the same rule. Stating the propagation here is what stops a later
  session choosing, once the counts are visible, between dropping the unknown
  claims and propagating them — a stage with one covered marginal claim and two
  uncovered ones would otherwise read as `Merge` or as Undetermined at the
  reader's discretion.
- **A charter breach** is reported for both stages: the making stage keeps the
  credit, and the pre-empted stage takes the Undetermined verdict its row above
  specifies.
- **A protocol breach in any of the four parts** is reported with its evidence
  and its effect on the verdicts it touches. The affected verdicts become
  Undetermined; the unaffected ones stand.

## Limits, stated in advance

These bound what `pipeline-kinds-k27` may conclude, and the report restates them.

- **One book, one crate, no repetition.** `jj-workspace` is the smallest new
  corpus in the campaign — four roots, 698 lines, seven pages. There is no second
  book, no second run of any stage, and no independent replication. The result is
  evidence about this campaign's remaining four books, not a general finding about
  editorial pipelines.
- **The result is conditional on how the draft was made.** The book is drafted by
  a session working from a detailed human structure brief under a validated book
  contract. A competent draft leaves the middle stages little to do, and that
  would show here as a two-stage verdict. That verdict would be correct **for
  books authored this way** and would say nothing about books authored without a
  structure brief.
- **The probe is a simulated arm.** It establishes what a proof pass would
  *identify*, not what it would successfully *repair*, and it is one context's
  output with unknown recall. Two of its four stated biases run against the
  six-stage hypothesis, one runs for it, and one has no established direction; no
  net conservatism is claimed.
- **Proof is untested**, for the reason given in *Part 4*.
- **Self-report is the first source of claims.** The report verifies every claim
  against the diff, accounts for every prose hunk, and finds regressions
  independently, so work a stage *did* and chose not to report is still
  recovered. What remains invisible is narrower and real: a defect **nobody
  fixed** and the probe did not list. The rule moves that hazard to where a
  session can act on it; it does not remove it.
- **The measure is of the artifact, not of readers.** No reader is asked anything.
  Like its precedent, this campaign measures returned artifacts, not comprehension.

## What this document does not settle

Recorded as open rather than filled in, so a later reader can tell a decision from
an invention.

- **Whether the taxonomy is complete.** It is derived from two frozen standards
  and will miss a reader-facing defect neither of them names. A stage meeting one
  records it in `## Findings not fixed` with the class it would need; the report
  lists them and the extraction reads them. **The taxonomy is not extended
  mid-campaign** — a class added after a stage has run is a class chosen with the
  data in view.
- **What `figure-contract-k18` should conclude.** This document makes art
  measurable; it takes no position on whether a figure format is earned.
- **How a kept stage is written as a skill.** Extraction is
  `pipeline-kinds-k27`'s, and a `keep` verdict is evidence for it rather than a
  specification of it.
