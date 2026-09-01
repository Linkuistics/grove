# Editorial-pipeline pilot — measurement report

The pilot authored the [`jj-workspace` book](../../walkthroughs/jj-workspace/README.md)
by running six editorial stages by hand, and asked **which of those stages paid
for itself**. This is the reading of that pilot against the
[preregistration](preregistration.md) written and committed before any page of
the book was drafted.

**The answer is four stages, and it is neither of the two answers the campaign
came in holding.** The six-stage pipeline is not upheld; the stated two-stage
alternative does not win either.

> **`N_min` = `N_max` = 4.** The surviving pipeline is **draft, copy edit, art,
> proof**. The developmental edit and the technical edit are `Merge`, each one
> marginal claim short of the threshold. **No stage is `Undetermined`, and no
> stage is `Drop`.**

Written by `measurement-report-k45`, which did the reading and none of the
editing. Its subjects are the six stage commits; every one of them was digested
before the reading and again after it, and none moved.

<a id="what-this-licenses"></a>
## What this licenses, and what it does not

[`pipeline-kinds-k27`](../../adr/a-kind-is-an-open-token.md) extracts the
publishing pipeline as installed `grove-<kind>` skills, and **there is no human
between this report and that extraction**. The preregistration fixes what each
verdict licenses; this is that table filled in.

| Stage | Verdict | What `pipeline-kinds-k27` may do |
|---|---|---|
| draft | not scored | it is the pipeline's first stage by construction: there is no book without it and no alternative to it |
| developmental edit | **Merge** | fold its charter into **draft**; **do not give it a kind** |
| technical edit | **Merge** | fold its charter into **draft**; **do not give it a kind** |
| copy edit | **Keep** | extract as its own `grove-<kind>` skill |
| art | **Keep** | extract as its own `grove-<kind>` skill |
| proof | not scored, and untestable by this instrument | extracting it rests on its being the **stated fallback**, not on measured evidence — see [*Proof*](#proof) |

**Nothing here is `Undetermined`, so nothing here is an absence being read as a
permission.** That matters more than the counts: the preregistration names
"a report that keeps a stage on the strength of an `Undetermined` verdict" as the
symmetric failure to a negative result, and this report reaches no such verdict to
misread. Where a verdict rests on a judgement rather than on arithmetic —
[art's does](#art) — that judgement is named at the row and its alternative is
priced.

<a id="provenance"></a>
## Provenance: the subjects, digested before and after

The subjects of this reading are six commits. *Part 3* resolves them by searching
commit descriptions for each stage's `<slug>-k<key>` handle; that search is
recorded here with what it returned.

| Stage | Handle | Change id | Book digest after that commit — **before** the reading | **after** the reading |
|---|---|---|---|---|
| draft (range, 7 commits) | `jj-workspace-book-k25` | `wnnkykqxxnot` … `nurklzvurwwp` | `0e9d3f1e…3aca14` | `0e9d3f1e…3aca14` |
| developmental edit | `developmental-edit-k40` | `qrtxukzpmuxt` | `669eee6f…0da4bde` | `669eee6f…0da4bde` |
| technical edit | `technical-edit-k41` | `rtpppznnqwtp` | `7b5d4314…da19db` | `7b5d4314…da19db` |
| copy edit | `copy-edit-k42` | `yxktpovqnxqu` | `ca79d04d…9d8ee52` | `ca79d04d…9d8ee52` |
| art | `art-k43` | `kkktwzyxyvqv` | `7dc08f7e…9e688a18` | `7dc08f7e…9e688a18` |
| proof | `proof-k44` | `rlntxrkmtyps` | `1e024f25…c2dfb267` | `1e024f25…c2dfb267` |

**Nothing moved under the reading.** The five stage records and this
preregistration were digested at both ends too, and all six are unmoved; the
working copy contained no uncommitted change to the book at either reading. The
draft's baseline is `pmyytxvyxmzz`, `pilot-preregistration-k54`, the commit before
the first drafting commit, exactly as *Part 3*'s one exception permits — and the
range's own record, [`0-draft.md`](stages/0-draft.md), lists all seven change ids
with the digest after each, so the range is diffed as a range rather than against
its last commit.

**This preregistration's SHA-256 at the report commit is
`b9ee9f8e48b69170c7ecd9787b9373dfc42242895e91d37d8a181af6f09a694e`** — the value
every one of the six stage records says it read. Its own *This document's own
digest* clause makes two records disagreeing a protocol breach; there is no
disagreement.

**No stage was re-run**, so the item-by-item confirmation *Validity* requires of a
re-run does not arise. Each stage was read once, in pipeline order, against its
own single commit.

<a id="frozen-inputs"></a>
## Frozen inputs, re-checked at the report

Eight of the nine carry the digests the preregistration froze. One does not, and
it is reported as a protocol breach rather than re-frozen.

| Input | Verdict |
|---|---|
| `crates/jj-workspace/Cargo.toml`, `src/lib.rs`, `src/jj.rs`, `src/refusal.rs` | all four unmoved — `528fe93e…`, `43ca4ad3…`, `23eb9d98…`, `6ea88fc2…`. **The corpus is untouched by all six commits**, checked against each commit's own file list rather than taken from the records |
| `docs/specs/walkthrough-books.md` | unmoved, `d9556f93…` |
| `docs/specs/jj-workspace-book-structure.md` | unmoved, `f7c9fa4a…` |
| `docs/USAGE.md` | unmoved, `ed79be85…` |
| `docs/walkthroughs/ordinal-fs-tree/` | unmoved, `03c1fd3e…` |
| `CONTEXT.md` | **moved**, `c7dde4d6…` → `c0d46984…` |

<a id="breach-context"></a>
### Protocol breach 1 — `CONTEXT.md`'s digest moved, and the instrument required it to

The move happened at `orientation-k55`, the draft's first commit, and it is not an
accident: four glossary entries gained an explicit `<a id="…"></a>` line so the
book could cite them, which the book contract's outbound-links rule demands of any
citing book.

**The breach is in the preregistration, not in the stage.** *Frozen inputs* lists
`CONTEXT.md` as a standard whose digest must not move, while *Validity*'s
allowlist expressly permits the draft — and only the draft — to touch `CONTEXT.md`
anchors. Both clauses cannot hold once a book cites the glossary, and
`jj-workspace` is the first book that does. [`0-draft.md`](stages/0-draft.md)
recorded the move, the four entries, and that no definition was altered, so the
change is classifiable by inspection rather than by trust.

**Effect on the verdicts: none, and the reason is structural rather than
charitable.** The move is entirely inside the draft range and precedes every
scored stage, so all five editorial stages read one and the same `CONTEXT.md` —
each recorded `c0d46984…` at its own start. The standard the copy edit's `C1`
class points at was constant across the whole scored window. No verdict below is
qualified by it.

Reported, not repaired. A rule that proves unworkable is a breach to report, and
what the campaign should carry forward is the observation itself: **a frozen-input
list and an edit allowlist that overlap will contradict each other the first time
the allowlisted edit happens.**

<a id="breach-charters"></a>
### Protocol breach 2 — two recorded stage-charter digests no longer match

*Frozen inputs* digests the six task files "for drift detection **during** the
campaign". Re-computed now:

| Stage charter | Verdict |
|---|---|
| `developmental-edit-k40` | unmoved, `19bd4e83…` |
| `copy-edit-k42`, `art-k43`, `proof-k44` | unmoved — `b206d9c0…`, `0a51333b…`, `767af84a…` |
| `technical-edit-k41` | **moved**, `92d131d2…` → `1e72f3cb…` |
| `jj-workspace-book-k25` (the draft) | **the file no longer exists** |
| `measurement-report-k45` (this session) | **moved**, `5c1781de…` → its value at this commit |

Three observations, none of which touches a verdict:

- **The technical edit's charter moved because that session appended its own
  running decision log to it**, which is what grove's execution procedure requires
  of every session and which the preregistration did not anticipate when it
  digested live task files. The *charter* — the standard the stage is held to — is
  restated in the preregistration and is what this report judged against; the task
  file is the session's workspace, not the instrument.
- **The draft's task file is gone because the leaf decomposed** into
  `05-jj-workspace-book-k25/`, a directory with a brief and seven children. *Part
  3* expressly permits exactly that decomposition, and the draft is unscored, so
  the missing digest costs the measurement nothing. The preregistration's own note
  that a moved *path* is not drift does not cover a leaf that stops being a file.
- **This session's own task file moved** for the first reason, disclosed here
  rather than left for a reader to find.

The general form is worth carrying forward: **`.grove/` task files are working
surfaces and cannot be frozen inputs.** Digesting them detects the drift the
methodology itself mandates.

<a id="how"></a>
## How the reading was done

*Part 1* bounds the judgement three ways so that it is verification rather than
taste. All three were applied to all 28 stage claims, and a fourth check —
*Part 3*'s completeness rule — was applied to all five diffs.

1. **Every claim's before-citation was located in the stage's parent commit and
   its after-text in the stage's own diff.** Read as `jj diff --git` over the book
   directory, one stage at a time. **All 28 located; none rejected.** No claim
   cited text absent from its parent, and no claim cited text the stage's diff
   left unchanged.
2. **Every claim's class test was applied to the cited before-text.** The two
   worth recording because they were checked rather than assumed: the developmental
   edit's `D3` at `01-orientation.md#what-it-declines` — the section before the edit
   runs from the H2 to `#package-contract` and names neither refusal 1 nor `std`,
   while its own third paragraph promises that *each chapter of this book opens on
   something this crate declines to own and names who owns it instead*, so the test
   is exhibited; and its `D5`, where both ledger rows named `#commit-tour` while
   both minimum local statements sit at `#public-surface`, confirmed by reading the
   draft-state page rather than the record's quotation of it.
3. **De-duplication was re-applied by this session and not taken on the stages'
   partitions.** It lands differently on two stages; see
   [*The two counting judgements*](#counting) below.
4. **Every prose hunk of every stage diff was accounted for**, against the claim
   table and `## Unclaimed changes` together. **No unaccounted hunk was found in
   any of the five stages**, so *Validity*'s unaccounted-hunk trigger fires for
   none of them and this report classified no hunk of its own.
5. **Regressions were searched for by this report rather than taken from
   self-report.** Each of the five diffs was read in full for text a stage
   *changed* into a taxonomy defect. **`r(S) = 0` for all five.** Three candidates
   were considered and rejected: the copy edit's lowercasing of `grove` at
   sentence-initial positions (the book's own standard spells the consumer
   lowercase, and no class tests sentence-initial capitals); three lines proof left
   at 95–110 columns against the book's ~82-column fill (the copy edit had already
   classified line fill as formatting with no reader-facing effect, and no class
   tests it); and proof's rewrite at `07-what-jj-owns.md#the-uncomfortable-one`,
   which superseded text the copy edit had produced one commit earlier — the
   mechanism the copy edit's claim 4 restored is still named there, so the earlier
   improvement is carried rather than reverted.

<a id="counting"></a>
### The two counting judgements, stated because they are load-bearing

*Part 1* makes the `book-wide` collapse mandatory and tells the report to
re-apply it. Re-applying it symmetrically — collapsing where a family exists,
keeping separate where instances need different corrective sentences — turns on
one question that the rule's final sentence settles: *do these instances need
different corrective sentences?*

**Art's six `A1` claims stay six.** The single subject-free sentence that would
collapse them is "draw the relation as a table" — which states the *class's remedy
shape*, not the correction. Each instance's correction is a different table with
different columns over different material: three resolution endings against
`.jj/repo` shape; four Git selectors; two constant lists side by side; four guards
with their counterfactual paths; two refusals against tree state and retry
outcome; ten refusal kinds against group, payload, `source()` and remedy. They
plainly "need different corrective sentences", which is the rule's own test for
staying separate. **And the alternative reading is self-defeating**: if "draw a
table" counted as one family, every `A1` defect in any book would always collapse
to exactly one claim, `A1` could never reach the threshold of three, and art would
be unmeasurable by construction — which is precisely what *Part 3*'s *Art, which
has no machinery behind it* section exists to prevent.

**The copy edit's own collapse of five `C2` figures of speech into one `book-wide`
claim is honoured rather than reversed.** By the strict reading just applied to
art, those five would also be five: the figures differ and so do the mechanisms
substituted for them. The stage collapsed them anyway, at its own cost. This
report leaves that collapse standing, in the direction that costs a stage rather
than the one that buys machinery — and reports the recount, which raises
`v(copy) = 6` to `10` and `m(copy) = 5` to `9` and **changes no row**, since both
are far above the threshold.

**Neither judgement is hidden inside an arithmetic.** Art's verdict is the one
that depends on its judgement, and [its row](#art) prices the alternative.

<a id="the-comparator"></a>
## The comparator, and that it was independent

The two-stage arm was simulated by the [proof-only probe](probe.md), produced by
**an independent context materialised by this session before this session opened
any post-draft state**. *Part 2*'s fallback was not triggered, so **no verdict
carries the `probe not independent` annotation and the fallback's withholding of
every `Keep` does not apply.**

The probe listed **21 defects**, distributed:

| Class | `T2` | `C2` | `D1` | `D5` | `D2` | `C1` | `C5` | `A1` | `A2` | others |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| entries | 10 | 3 | 2 | 1 | 1 | 1 | 1 | 1 | 1 | 0 |

**That distribution is the single most consequential fact in this report**, and it
is stated before the verdicts rather than after them. Ten of the probe's
twenty-one entries are `T2` — the technical editor's own class — and only two are
in `A1`/`A2` and six across all five `C` classes. A comparator that is good at the
technical editor's job is exactly the comparator that makes the technical editor
look marginal, and one that is weak at the art and copy classes is the one that
makes those two look strong. The instrument named this hazard in the abstract
(*"probe recall error pushes in the expensive direction"*); this run gives it a
shape, and it is not uniform across stages.

**The probe is complete rather than partial** — it terminated normally, reports a
whole-book read with the mechanical checks only a whole-book read can perform, and
its entries land on eight of the ten book files. `02-the-gate.md` carries no probe
entry. That page is the subject of exactly two stage claims, so *Validity*'s
partial-probe interval was applied to it **defensively**, as though the page were
uncovered: `net_lo(copy) = 4` and `net_lo(art) = 5`, both still `Keep`, and both
`net_hi` values are the scalars below. **The interval selects the same row at both
ends for every stage, so no stage is `Undetermined` on coverage.**

<a id="the-arithmetic"></a>
## The arithmetic

| Stage | `v(S)` | `m(S)` | `r(S)` | `b(S)` | `net(S)` | Verdict |
|---|---:|---:|---:|---:|---:|---|
| developmental edit | 3 | 2 | 0 | 0 | **2** | **Merge** |
| technical edit | 6 | 2 | 0 | 0 | **2** | **Merge** |
| copy edit | 6 | 5 | 0 | 0 | **5** | **Keep** |
| art | 7 | 6 | 0 | 0 | **6** | **Keep** |
| proof | 6 | — | 0 | — | — | not scored; see [*Proof*](#proof) |

**`b(S) = 0` for every stage: no charter breach occurred in this pilot.** Each of
the four middle stages made claims only in the classes its own charter owns, which
this report confirmed against the class codes rather than against the records'
assertions. Two consequences follow, and both matter: the pre-emption row of the
verdict table cannot fire for any stage (it requires an earlier stage's marginal
out-of-charter claim), and no stage's empty count anywhere below is another
stage's discipline failure.

<a id="developmental-edit"></a>
## Developmental edit — `Merge` into draft

**What it changed.** Three claims, all in `D1`–`D5`, over four book files in one
commit: chapter 1 gained the opening paragraph that names its own refusal
(*no dependencies*) and its owner (`std`); `main_repo_of` — used in chapter 1's
carried trace and owned by chapter 2 — gained a prose statement and a row in both
the manifest and the ledger; and two ledger rows were re-pointed from
`#commit-tour` to `#public-surface`, where their minimum local statements actually
are. One further hunk updated three counts in chapter 7 that its own new ledger row
made stale, correctly recorded as unclaimed because a stage may not score a number
it made wrong itself.

**What the judged outcome says.**

| # | Class | Subject | Reached by the probe? |
|---:|---|---|---|
| 1 | `D3` | `01-orientation.md#what-it-declines` | **no** — the probe listed no `D3` entry anywhere, and its one entry at this subject (#7) is a `T2` about the crate's public-type count |
| 2 | `D1` | `01-orientation.md#commit-tour` | **no** — the probe's two `D1` entries are ledger rows for `control_dir` and `is_tracked`; neither is the missing `main_repo_of` row |
| 3 | `D5` | the `Commit` and `control_dir` ledger rows | **yes** — probe entry 11 is the same defect at the same row, in the same class, and names the same discharge anchor |

`m = 2`, `r = 0`, `net = 2` → **Merge**.

**Merge target: draft.** The rule is deterministic — no out-of-charter claims, so
"the nearest surviving stage earlier in the pipeline", and the only earlier stage
is the draft, which survives by construction. The fold is coherent rather than an
artefact: the draft is authored from a human structure brief that already fixes
chapter sequence, theses and the early-use ledger, so the developmental editor's
charter is largely a re-check of the brief the drafter was working to.

**And it is one claim from `Keep`.** Read against the probe's class
distribution — zero `D3` entries, two `D1` entries both on ledger rows — the
comparator's coverage of the developmental classes is thin, and the instrument's
own analysis says thin probe recall pushes a stage *toward* `Keep`. The verdict
stands as the arithmetic gives it; the margin is recorded so
`pipeline-kinds-k27` reads a `Merge` at 2 rather than a `Merge` at 0.

<a id="technical-edit"></a>
## Technical edit — `Merge` into draft

**What it changed.** Six claims in one commit over five book files: five `T2`
corrections and one `T1`. The largest is a `book-wide` `T2` — the book presented
`.jj/.gitignore` as an unconditional entry of a jj workspace and then drew it
inside a tree it had declared *native*; the stage measured `git.colocate`'s
default on jj 0.44.0, established that the entry is a colocated workspace's, and
corrected six instances across four pages, including the one worked example whose
outcome depends on which shape it stands in. It also corrected a reason string the
book attributes to `validated_namespace` that the source does not contain,
re-ordered a worked example whose heading claimed an order it did not show, named
the test that actually reaches `relative`'s canonicalising fallback in place of one
that does not, and supplied the missing *no test proves this* element to two argued
claims. One unclaimed hunk corrected a false statement about a **sibling** crate,
which no taxonomy class reaches.

**What the judged outcome says.**

| # | Class | Subject | Reached by the probe? |
|---:|---|---|---|
| 1 | `T2` | `06-refusal.md#the-namespaces-two` | **yes** — probe entry 4, the same reason string |
| 2 | `T2` | `04-namespace.md#worked-reservation` | **yes** — probe entry 3, the same heading and the same guard order |
| 3 | `T2` | `book-wide` | **yes**, by *Part 2*'s rule: a `book-wide` claim is reached when the probe lists two or more instances of that class anywhere, and it lists ten |
| 4 | `T2` | `05-scope-and-commit.md#the-path-algebra` | **yes** — probe entry 8, the same sentence |
| 5 | `T2` | `03-subprocess-seam.md#a-command-a-reader-could-type` | **no** — the probe's chapter-3 `T2` is at `#worked-invocation`, a different subject |
| 6 | `T1` | `05-scope-and-commit.md#the-path-algebra` | **no** — the probe listed no `T1` entry anywhere |

`m = 2`, `r = 0`, `net = 2` → **Merge**, target **draft**, by the same rule and
the same reasoning as above.

**This is the stage the comparator was best equipped to reach, and it shows.**
Three of its four reached claims were matched to a probe entry describing the same
defect in the same words — not merely the same class at the same anchor. The
fourth, the `book-wide` claim, is reached by the generous rule rather than by any
matching entry: the probe never found the colocation defect, which is the largest
single correction any stage made in this pilot, and the matching rule credits the
two-stage arm with it anyway. That is the instrument working as designed — *Part
2* states that over-matching runs against the six-stage hypothesis — and it is
worth naming, because it is the difference between this stage's `Merge` and a
`Keep`.

<a id="copy-edit"></a>
## Copy edit — `Keep`

**What it changed.** Six claims in one commit over seven book files: the
consumer's name spelled one way throughout, three glossary terms turned from an
inline re-definition into citations of anchors the manifest already declares,
*native* re-glossed against the axis the rest of the book uses it on, five figures
of speech replaced by the mechanisms they stood for, an uncheckable line count
replaced by a surface a reader can count, and the book's one fragment introduction
that answered one of the contract's five editorial questions rewritten to answer
all five.

**What the judged outcome says.**

| # | Class | Subject | Reached by the probe? |
|---:|---|---|---|
| 1 | `C1` | `book-wide` (`Grove` / `grove`) | **no** — the rule requires two or more `C1` entries anywhere; the probe listed one |
| 2 | `C1` | `01-orientation.md#commit-tour` | **no** — the probe's one `C1` is at `README.md` |
| 3 | `C1` | `02-the-gate.md#worked-resolution` | **no** |
| 4 | `C2` | `book-wide` (five figures of speech) | **yes** — the probe listed three `C2` entries |
| 5 | `C2` | `07-what-jj-owns.md#final-verification` | **no** — the probe's three `C2` entries are at `04#the-reserved-list`, `05#the-path-algebra` and `05#worked-commit` |
| 6 | `C5` | `04-namespace.md#the-reservation` | **no** — the probe's one `C5` is at `05#the-commit` |

`m = 5`, `r = 0`, `net = 5` → **Keep**, well clear of the threshold of three.

**Robust in both directions.** Under the strict recount that un-collapses claim 4,
`v = 10` and `m = 9`; under the defensive treatment of `02-the-gate.md` as
uncovered, `net_lo = 4`. Every reading is a `Keep`.

**One honest deflation.** The probe's single `C2` entry at
`04-namespace.md#the-reserved-list` is the article error (*a interface*) that the
copy edit fixed and deliberately recorded as an **unclaimed** change, because no
taxonomy class covers grammar. It scores nothing for the stage under this
instrument, and the probe found it. The overlap is recorded rather than quietly
counted on either side.

<a id="art"></a>
## Art — `Keep`, on an adjudication this report names

**What it changed.** Seven claims in one commit over six book files, and nothing
but Markdown: no asset, no image, no manifest group, no validator change. Six `A1`
tables now carry relations the pages had been carrying in running prose — the
gate's three resolution endings, the four Git repository selectors, the two
constant lists whose confusion the page had warned about while asking the reader to
hold both in memory, the four namespace guards with the path each would otherwise
have produced, the two commit refusals against tree state and retry outcome, and
the whole ten-kind case analysis that five sections established one at a time and
never assembled. One `A2` claim gave the book's most consequential table — the
six-row verdict table returning the `Abdication` — the adjacent statement of its
role it had never had.

**What the judged outcome says.**

| # | Class | Subject | Reached by the probe? |
|---:|---|---|---|
| 1 | `A1` | `02-the-gate.md#worked-resolution` | **no** |
| 2 | `A1` | `03-subprocess-seam.md#the-selectors` | **no** |
| 3 | `A1` | `04-namespace.md#the-reserved-list` | **no** |
| 4 | `A1` | `04-namespace.md#the-validation` | **no** |
| 5 | `A1` | `05-scope-and-commit.md#the-commit` | **no** — the probe's entry at this subject is a `C5` |
| 6 | `A1` | `06-refusal.md#the-case-analysis` | **no** |
| 7 | `A2` | `07-what-jj-owns.md#applying-the-test` | **yes** — probe entry 20, the same table and the same complaint |

The probe's one `A1` entry is at `01-orientation.md#commit-tour`, a subject no art
claim touches. `m = 6`, `r = 0`, `net = 6` → **Keep**.

**The adjudication this verdict rests on, and what the alternative costs.** Art's
six `A1` claims are counted as six because they need six different corrective
sentences, for the reasons set out under [*The two counting judgements*](#counting).
Under the alternative reading — that "draw the relation as a table" is a single
family sentence — art's six would collapse into one `book-wide` `A1` claim, which
the probe's single `A1` entry would fail to reach (the rule requires two or more),
giving `v = 2`, `m = 1`, `net = 1` and a verdict of **`Merge`**. So:

> **Art is the one stage whose row moves under a defensible alternative reading of
> the counting rule.** It is `Keep` under the reading this report gives and
> `Merge` under the other. It is not `Drop` under either, and it is not
> `Undetermined` under either — the collapse question is a judgement the rule
> settles, not missing evidence.

`pipeline-kinds-k27` should know that this `Keep` is the one verdict in the report
carried by an interpretation rather than by arithmetic alone.

**Art produced something, and that is data.** *Part 3* is explicit that an art
stage running and finding nothing is a stage with zero marginal claims rather than
an absence. This one ran and found six relations worth drawing in a seven-chapter
book, using only what Markdown already gives it. That is the input
`figure-contract-k18` needs: art paid for itself **without any
asset machinery at all**, which is evidence about whether such machinery is earned,
not evidence that it is.

<a id="proof"></a>
## Proof — described, not scored, and the report says why

**A comparator cannot be tested by the comparison it defines.** Proof appears in
both arms by construction, and the probe *is* proof applied to the draft, so no
marginality is computable for it. *Part 4* requires its contribution to be reported
descriptively instead, and requires this report to say plainly what follows.

**Six claims, over four pages, in five classes:**

| Class | `T2` | `P1` | `C1` | `C4` | `A1` |
|---|---:|---:|---:|---:|---:|
| claims | 2 | 1 | 1 | 1 | 1 |

**The descriptive split is stark: one of proof's six claims is in `P1`, its own
class, and five are in classes an earlier stage's charter owned.** By *Part 4*'s
own reading of that split, **proof functioned in this pilot as a net for other
stages' misses rather than as an independent contributor.** Its one `P1` claim is
a genuine whole-book defect no single page's read reveals — two pages giving two
sizes for one test suite — and it is exactly the kind of thing the class exists
for. The other five are a `T2` over-attribution chapter 2 made about a test,
another `T2` where a chapter's closing summary contradicted its own table eight
hundred lines above it, a `C1` where *link* counted `source()` hops on one page and
chain nodes on another, a `C4` repetition the copy edit had explicitly deferred to
proof, and an `A1` the art stage had explicitly handed to proof.

Two of those five were handed forward on purpose, which is the pipeline working:
the copy edit declined to trim a passage the technical edit had landed one commit
earlier, and art declined to draw a whole-book relation because `P1` is proof's
class. A pipeline in which stages hand work to a later stage that then does it is
not a pipeline of redundant stages.

**And the statement *Part 4* requires, in its own terms: `pipeline-kinds-k27`
extracting proof rests on proof being the stated fallback, not on measured
evidence.** Nothing in this pilot tested proof, and nothing could have.

<a id="headline"></a>
## The headline: the surviving pipeline

The surviving pipeline is `{draft}` ∪ `{middle stages with verdict Keep}` ∪
`{proof}`, with merged stages folded into their targets and contributing no stage
of their own:

> **draft → copy edit → art → proof.** `N_min = 4`, `N_max = 4`, so **`N = 4`**.

`N_min = N_max` because **no stage is `Undetermined`**. The range is a point, and
the extraction table at the top of this report is what governs what may be built.

By *Part 4*'s outcome table, `2 < N < 6`:

> **an `N`-stage pipeline is what the evidence supports; neither the six-stage
> design nor the two-stage fallback is upheld.**

<a id="six-versus-two"></a>
## Did the six stages beat the stated alternative of two?

**No — and the two-stage alternative did not win either.** Both framings the
campaign came in holding are refused by the evidence.

- **Six is not upheld.** Two of the four middle stages did not clear a threshold
  that was fixed before any of them ran. Each closed exactly two marginal defects
  across a seven-chapter book — under one defect per three chapters, which is the
  volume the preregistration says a neighbouring stage absorbs without a session of
  its own.
- **Two is not upheld.** Fifteen of the twenty-two defects the four middle
  stages closed were **not** reached by a proof pass over the draft, and fifteen
  marginal defects is not a rounding error against a book of seven chapters. Two whole
  classes of work — five of the six copy-edit corrections and six of the seven art
  figures — the two-stage arm simply does not get. A draft-plus-proof pipeline
  would have shipped this book with its consumer's name spelled two ways, its one
  under-introduced fragment unfixed, and every one of its six tables absent.
- **Four is what the evidence supports**, and two of those four are the two stages
  the campaign would have been least confident about in advance: the copy edit and
  the art stage, the latter running with no machinery whatsoever.

**A result that had said two beats six would have been a successful outcome of
this pilot, not a failed one**, and the same is true of this one. The pilot was
ordered to stop expensive machinery being built ahead of the measurement that
would justify it, and it has: **it saves two skills and two sessions per book
across the four books that follow**, while establishing that two other stages earn
their sessions. That is the measure doing its job, in both directions at once.

<a id="unresolved"></a>
## What this report refuses to reach

Stated in these terms because *Part 4* requires the conclusions a report declines
to draw to be as legible as the ones it draws.

- **No stage's verdict is `Undetermined`.** The probe was independent and
  complete, every record was present and distinguished *ran* from *did not run*,
  every stage landed in exactly one valid commit, and no stage's arithmetic
  straddled two rows of the table. Where a defensive reading was available — the
  partial-probe interval over `02-the-gate.md` — it was applied and changed
  nothing.
- **Proof's contribution is undetermined by construction**, and is reported as
  such rather than as a keep. This is the one place in the report where an
  extraction proceeds on something other than measured evidence, and the
  preregistration fixed that in advance.
- **The two `Merge` verdicts are one marginal claim from `Keep` each**, and the
  probe's class distribution is thinnest exactly where those two stages work. The
  arithmetic is what it is; the margin is recorded so it is not read as a rout.
- **Art's `Keep` rests on a counting adjudication**, priced at [its row](#art).
- **The measure says nothing about whether a merged charter survives the fold.**
  Whether a drafting session working from a structure brief *actually* performs the
  developmental and technical charters, or merely inherits the obligation, is a
  question this pilot did not ask and could not have answered from one book.

<a id="no-class"></a>
## Defects the frozen taxonomy could not carry

The preregistration records that the taxonomy will miss a reader-facing defect
neither frozen standard names, forbids extending it mid-campaign, and asks the
report to list what the stages recorded instead. Four classes came up, each named
by the stage that met it:

| Would-be class | Where it was met | Recorded by |
|---|---|---|
| **an unnamed exception** — a hedge that concedes a divergence without saying where it is | `06-refusal.md#the-case-analysis`, *"in nearly the order the reader met them"* | developmental edit; **fixed** by the copy edit as an unclaimed change, scored to nobody |
| **a false claim about a cited non-corpus artifact** — the corpus is the four `jj-workspace` roots, so a false statement about a sibling crate is not `T2` | `06-refusal.md#the-opaque-type`, the claim that `grove-loop` matches on `ordinal_fs_tree::Refusal` | technical edit; **fixed** as an unclaimed change, scored to nobody |
| **a link label naming its destination but not its purpose** — `C4` is deliberately scoped past it | `06-refusal.md#the-premise`, the `anyhow` link | copy edit; **not fixed**, carried unchanged through proof |
| **an unverifiable number** — no rule stated, nothing in the source to contradict it | `07-what-jj-owns.md#final-verification`, *"its public surface is fourteen lines"* | technical edit; **fixed** by the copy edit as its `C2` claim 5, which is the taxonomy reaching it from a different direction |

**Two of these were real work that the measure scored at zero.** That is the
taxonomy's incompleteness showing up as a systematic understatement of what the
technical edit and the copy edit did, and it runs in the same direction as their
verdicts. It is a limit of the instrument, not a correction to the arithmetic; the
preregistration forbids extending the taxonomy once a stage has run, and this
report does not.

**One further conflict survived every stage and is not a defect any of them could
close.** `01-orientation.md#public-surface` is a `D2` by the catalogue rule that
both frozen standards state, and is *required* by the structure brief's own
chapter-1 description, which mandates the enumeration and fixes that section as the
First-use anchor of two ledger rows. The developmental edit, the copy edit and
proof each recorded it and each declined to act, correctly: a stage may not revise
the standard it is judged against. **The probe found it independently** — entry 14,
`D2` at that exact anchor — which is as strong a confirmation as this pilot can
produce that the conflict is real and reader-visible rather than a scruple. It
belongs to whoever next revises `docs/specs/jj-workspace-book-structure.md`, and it
is recorded here because `.grove/` will not outlive the campaign.

<a id="validation"></a>
## The book is left provable

This stage edited nothing in the book. Final validation and the umbrella were run
at the report commit, over the state the proof stage left:

```console
$ bash scripts/check.sh
...
=== book-check
  book-check docs/walkthroughs/jj-workspace
valid: 4 files, 698 resolved lines, 0 deferred lines, final=true
  book-check docs/walkthroughs/ordinal-fs-tree
valid: 17 files, 8720 resolved lines, 0 deferred lines, final=true
  2 book(s) checked, 0 failing
  ✓ book-check

check: all 8 principal checks pass
```

698 resolved lines, zero deferred, `final=true`, and the book-directory digest
`1e024f25a869dfc1faa74800749782c92e0efe1954828e6e9bf61ab7c2dfb267` — the value
[`5-proof.md`](stages/5-proof.md) recorded, unmoved across this reading.

<a id="limits"></a>
## Limits, restated

The preregistration fixed these in advance and requires the report to restate
them. They bound what `pipeline-kinds-k27` may conclude.

- **One book, one crate, no repetition.** `jj-workspace` is the smallest new
  corpus in the campaign — four roots, 698 lines, seven chapters. No stage was run
  twice, no second book was drafted, and nothing here is replicated. **This is
  evidence about this campaign's remaining four books, not a finding about
  editorial pipelines.**
- **The result is conditional on how the draft was made.** The book was drafted
  from a detailed human structure brief under a validated book contract. A
  competent draft leaves the middle stages little to do — and the two stages that
  merged are precisely the two whose charters that brief most directly pre-empts.
  The finding is *correct for books authored this way* and says nothing about books
  authored without a structure brief. **This is the most important limit on the
  page**, because the merge target for both merged stages is the draft, and the
  fold is only safe while the structure brief keeps being written.
- **The probe is a simulated arm.** It establishes what a proof pass would
  *identify*, not what it would successfully *repair*; it is scored as though
  everything it named would have been fixed. It is one context, unreplicated, with
  unknown recall. Two of its four stated biases run against the six-stage
  hypothesis, one runs for it, and one has no established direction: **no net
  conservatism is claimed.** This run adds one observation the preregistration
  could not: the probe's recall is visibly **uneven across classes**, strong in
  `T2` and thin in `D`, `C` and `A`, which means its error does not cancel between
  stages.
- **Proof is untested**, for the reason given above.
- **Self-report was the first source of claims**, but not the last word: every
  claim was checked against the diff, every prose hunk was accounted for, and
  regressions were searched for independently. What remains invisible is narrower
  and real — **a defect nobody fixed and the probe did not list.**
- **The measure is of the artifact, not of readers.** No reader was asked
  anything. Like its precedent, this campaign measures returned artifacts, not
  comprehension.
