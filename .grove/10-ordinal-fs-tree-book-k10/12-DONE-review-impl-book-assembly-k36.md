# book-assembly-k36

**Reviews:** book-assembly-k35

## Goal

Perform a fresh-context editorial review of the complete ordinal filesystem tree book. Review the whole assembled book, not only the producer diff.

## Context

Read the complete `docs/ordinal-fs-tree/book/` in order, together with the root brief, the `book-assembly-k10` node brief, and `docs/specs/ordinal-fs-tree-book.md`. The committed producer under review is `book-assembly-k35`.

This is an editorial review. Do not edit the book or implementation while reviewing it.

## Done when

- The whole book has been assessed for clarity, concept order, cognitive load, self-containedness, repetition, and cross-reference choices.
- The review checks that the prose remains declarative and satisfies the book specification's prose contract.
- Findings identify precise locations and actionable corrections; do not run implementation or model checks for this review.
- If there are substantive findings, append an adjacent `integrate-review-impl` leaf with the same bare stem, set `**Integrates:** book-assembly-k36`, and require the integrator to rerun the complete book, crate, Quint, and Alloy checks after making corrections.
- If there are no substantive findings, retire this leaf without creating an integration leaf.

## Notes

Approach the material as a reader encountering the assembled explanation for the first time. Do not limit the review to changes made by `book-assembly-k35`.

## Findings

Twelve findings, most severe first. The whole book was read in canonical order —
`README.md`, the eight numbered pages, `concept-index.md`, `source-index.md` —
against the prose contract in `docs/specs/ordinal-fs-tree-book.md` (*Self-
containedness*, *Direct declarative style*, *Local context and repetition*,
*Source-fragment introductions*, *Worked examples*, *Audience boundary*).

Two whole classes were checked and are clean. **Declarative style**: no
rhetorical question, narrative suspense, metaphor, idiom, emotive word, or
persuasive framing occurs in authored prose on any page, and refusal,
environmental failure, rollback failure, and implementation defect are labelled
as separate categories wherever they meet (`04-read-path.md#read-errors`,
`06-filesystem-interpreter.md#errors`, `08-invariants-and-trade-offs.md#outcome-map`).
The crate's own comment voice inside four-backtick fences is quoted source, is
authoritative under *Status and authority*, and is correctly not held to the
book's prose contract. **Fragment-introduction structure**: every one of the 68
literal fragments is now immediately preceded by a prose paragraph with no
intervening heading, list, table, fence, comment, or directive — the structural
half of `book-assembly-k34`'s finding 3 is fully discharged. **Catalogue
ordering**: on every page carrying a worked example, no earlier section
primarily enumerates three or more queries, operations, verbs, error variants,
or obligations.

Findings 1 and 2 are the editorial residue of two technical findings that were
fixed narrowly; both are stated as such rather than as regressions.

### 1. The running example silently changes tree, ordinals, and root spelling between pages

`book-assembly-k34`'s finding 6 removed the false promise at
`03-reference-domain.md:49` ("A stable example tree for later pages" became "The
tree the read path reads on the next page is"). That corrected the claim. It did
not tell the reader that two different trees run through the book, and the
reader-facing half is the editorial half.

Pages 01, 05, 06, and 07 use one tree; pages 03 and 04 use another:

| | `01`/`05`/`06`/`07` | `03`/`04` |
|---|---|---|
| root spelling | `s/` | absent (`03:50`), `syllabus/` (`04:31`) |
| `…-foundations-i3.md` | present at ordinal 1 | absent |
| key 5 (`vectors`) | ordinal 2 | ordinal 1 |
| key 6 (`matrices`) | ordinal 3 | ordinal 2 |
| root-level `…-assessment-i9.md` | absent | present |
| greatest key | 6 | 9 |

The consequences a reader actually meets: the same entity, key 6, is
`03-draft-matrices-i6.md` at `03:284` and `03-reference-domain.md:638`, and
`02-draft-matrices-i6.md` at `01:37` — and `03:638` says "Composing ordinal 3
… produces `03-draft-matrices-i6.md`: only the ordinal changes", which is the
same string the reader has already been shown as matrices' *starting* name on
page 01 and its *post-shift* name on page 05. A reader carrying page 03's tree
into `05-mutation-algebra.md#worked-insert-decision` computes a fresh key of 10,
not the 7 that page prints. Each page is internally correct, as
`book-assembly-k34` established; the defect is that nothing marks the transition
in either direction, so the divergence reads as contradiction rather than as
different setup.

`04-read-path.md:28` sharpens it by asserting continuity that the display then
breaks: it says "The reference tree from the previous page is the worked input"
and then prints a root, `syllabus/`, that the previous page did not show and
that no other page uses.

Correction, at three locations:

- `03-reference-domain.md:49`, where the tree is introduced: state in one
  sentence that this is the read path's variant of the orientation tree and name
  what differs and why — it adds a second top-level lesson so the walk crosses a
  subtree boundary, and omits `foundations` to keep the module small.
- `04-read-path.md:28`: either drop `syllabus/` in favour of `s/` and adjust the
  three dependent strings on that page (`fs::read::<SyllabusName>(Path::new("syllabus"))`,
  `syllabus/..`, and "For the `syllabus` example" at `04:1230`), or say
  explicitly that this page spells the root `syllabus` where orientation spelled
  it `s`. Unifying is preferable: the root spelling carries no teaching load, and
  a third spelling of the same thing is pure cost.
- `05-mutation-algebra.md:265`, where the orientation tree returns: one clause
  marking the return ("the orientation tree, not the read-path variant"), so the
  ordinal change the reader is about to see is attributed to the setup rather
  than to an operation.

The alternative — reconciling the two trees into one — is a larger change and is
not recommended: the read path genuinely needs `03-draft-assessment-i9.md` to
demonstrate that a node's complete subtree precedes its later siblings
(`04:75`), and the mutation pages genuinely need the greatest key to be 6.

### 2. Seven fragment introductions on page 06 answer three or fewer of the five required questions

*Source-fragment introductions* requires each introducing paragraph to answer
five questions: why the fragment is here, which actor owns the behavior, what
input becomes what output, which invariant it establishes or uses, and what role
it has in the page's current example. "An unanswered question is an editorial
finding."

`book-assembly-k34`'s finding 3 named five directives on this page that had *no*
introduction. `book-assembly-k35` wrote full introductions for exactly those
five — `lock-modes`, `lock-take`, `apply-plan`, `apply-run-state`,
`error-taxonomy` are now 69, 70, 76, 71, and 73 words and answer all five
questions. The neighbouring class was not swept. Seven introductions on the same
page remain one-sentence content summaries:

| Directive | Line | Introduction | Unanswered |
|---|---|---|---|
| `write-guard-append` | `06:326` | "Append and append-many share this dispatch shape." (7 words) | input/output, invariant, example role, and the actor's behavior |
| `write-guard-dispatch` | `06:543` | 15 words | invariant, example role |
| `write-guard-accessors` | `06:276` | 19 words | invariant, example role |
| `lock-contract` | `06:102` | 18 words | input/output, invariant, example role |
| `apply-contract` | `06:583` | 20 words | input/output, invariant, example role |
| `error-boundary` | `06:1299` | 28 words | invariant, example role |
| `error-debug` | `06:1485` | 25 words | example role |

`write-guard-append` is the clearest case: it introduces 39 lines of the public
`append`/`append_many` surface with a sentence that names no input, no output,
no invariant, and no connection to the worked insert. The comparison that makes
this a defect rather than a preference is inside the book — every other numbered
page holds its introductions to the pattern. `05-mutation-algebra.md` uses "X
owns Y. This fragment turns A into B, establishes C, and D for the worked
insert" for all twelve of its fragments; `04-read-path.md` does the same for all
twelve of its. `error-debug` has an exact counterpart in `05:1596`
(`report-debug`), which makes the same `Debug`-without-`N: Debug` point and then
closes with "and completes the exact report source owned by this page."

Correction: raise these seven to the pattern the other seven pages use. The
inputs are all present in the surrounding prose already — for `write-guard-append`,
that append and append-many take the consumed guard and its captured snapshot to
one `Decision`, that the consuming signature is what makes one guard authorize
one decision, and that `append_many` is the page's stated way to place several
entries under one rollback boundary (`06:244`).

### 3. `06:565` labels its section "failed unwind" when the failed unwind is the next section, and the worked-example section states no start

Two problems at one location.

`06-filesystem-interpreter.md:565` is `## Successful application and failed
unwind`, carrying the anchor `worked-apply-and-unwind`. Its three subsections
are `### Ordered application` (`:568`), `### Effect-specific steps and paths`
(`:763`), and `### Forward failure and reverse unwind` (`:963`). The last of
those describes an unwind that *succeeds*: "If both restores succeed, every
filesystem change this run made has been reversed when `Error::Failed` returns"
(`:985`). The unwind that fails is in the *next* H2, `## Failed unwind and
repair` at `:1108`, which returns `Error::FailedPartiallyRolledBack`. The book
therefore has two adjacent H2s whose titles both say "failed unwind" and which
mean opposite outcomes — the one distinction `06`'s own contract turns on.

Second, `:565` is followed immediately by `:566` (blank) and `:567` (the anchor
for the first H3). The section has no prose at all. *Worked examples* requires
that "the stated start, transition steps, and observable end are all present",
and this is the page's required `worked-apply-and-unwind` boundary — "The same
plan through successful application and a second trace through forward failure
and unwind outcome". A reader entering here is given no statement of which plan,
which starting tree, or what the two traces will show.

Correction: retitle `:565` to name what it contains — "Successful application
and reverse unwind" — leaving `:1109` as the only heading claiming a failed
unwind; and add a lead paragraph under it stating the start (the guarded
three-effect plan from `05-mutation-algebra.md#worked-insert-decision` over the
orientation module level), the two traces it runs, and the observable ends
(`Report`, then `Error::Failed`).

### 4. The seven `EntryName` obligations and their enforcement split are explained in full twice

*Local context and repetition*: "The repetition is the minimum semantic
contract, not copied source or a second full explanation."

`02-name-seam.md:480` opens "The trait establishes seven obligations:" and
enumerates all seven. `03-reference-domain.md:739` is `## Seven obligations,
with two enforcement mechanisms` and enumerates the same seven again. That much
is defensible on its own — page 03 owns `conformance.rs` and a reader may enter
there. The duplication that is not is the enforcement split, which both pages
state completely:

| | `02-name-seam.md` | `03-reference-domain.md` |
|---|---|---|
| types discharge 3 and 4 | `:492-496` | `:752-757` |
| kit covers the other five | `:497-498` | `:758` |
| library enforces 7 at the boundary | `:499-500` | `:758-760` |

Page 02 also forward-references a mechanism it never introduces. "The
conformance kit reports the two structural forms as discharged and exercises the
five remaining consumer obligations" (`02:497`) attributes specific coverage
behavior to a "conformance kit" whose only prior appearance in the book is a
bare listing in `01-orientation.md:203` ("Its public surface exposes the
conformance kit and reference domain"). The kit is owned by
`reference-domain-k13` and defined on page 03. It carries no row in
`source-index.md#early-uses`, so it has neither a minimum local statement nor
the ledger entry *Early-use ledger* requires before "introducing any additional
later-owned type". A reader on page 02 cannot evaluate the claim, and the two
questions self-containedness asks of it — what checks this, and how much does it
prove — are answerable only on page 03.

Correction, on page 02 rather than page 03, since page 03 owns both the source
and the `DISCHARGED_BY_THE_TYPE_SYSTEM` table that makes the split concrete:
keep the seven-obligation list and the signatures-discharge-3-and-4 explanation,
which are the trait's own business, and reduce `:497-500` to the minimum
semantic contract plus a link — that the remaining five are semantic assumptions
Rust cannot check, exercised by the reference domain's reusable conformance kit
on [Reference domain](03-reference-domain.md#conformance-obligations), and that
the filesystem layer separately enforces the seventh. If the kit keeps a
behavioral claim on page 02, add its early-use row.

### 5. `07:1637` puts stream-contract-test content under a heading promising omissions and retry limits

`07-syllabus-cli.md:1636-1637` is the anchor `omitted-features` and the heading
`## Deliberate omissions and retry limits`. `concept-index.md:50` links to it as
"CLI omissions and retry limits".

The section opens with two paragraphs and a 180-line fragment about the in-file
terminal-failure contract tests (`:1639-1647`, then `cli-stream-contract-tests`
at `:1648`). That content is neither an omission nor a retry limit. The
section's actual subject — no removal command, no dry-run, no lock options, no
label lookup, then exits 6 and 7 and signal termination — begins at `:1833`,
after the fragment.

The seam is visible in the tree: this range was added by
`08-DONE-impl-cli-stream-boundaries-k33.md` after the page's section structure
was settled, and it was appended into the last existing section rather than
given one.

Correction: give the contract tests their own anchored section immediately
before `omitted-features` — `<a id="stream-contract-tests"></a>` and a heading
such as `## Terminal-failure contract tests` — moving `:1639-1647` and the
fragment under it, and add the concept-index entry. `omitted-features` then
begins at its own first sentence, and the index entry leads where it says.

### 6. `01:358` and `05:304` give contradictory reasons for highest-first shifting

`01-orientation.md:357` states the rationale: "This ordering vacates every
destination before it is needed and leaves distinct ordinals after every landed
effect."

`05-mutation-algebra.md:303` denies it: "It is not needed to prevent an ordinary
filename collision because each sibling's key remains in its name. Its general
purpose is the intermediate state: a process stopped between moves leaves a gap,
whereas lowest-first shifting would temporarily duplicate an ordinal."

A reader builds a destination-vacating model on page 01 and has to discard it on
page 05. The book's own most precise statement is
`08-invariants-and-trade-offs.md:98-103`, which keeps the vacating sentence but
immediately supplies the real consequence ("An interrupted shift can therefore
leave a gap but does not transiently duplicate an ordinal") and scopes the
collision claim to corrupted trees ("can also collide on a tree already carrying
duplicate keys and equal parts"). Page 01 has the clause without either
qualifier.

This is progressive disclosure done in the wrong direction: a low-resolution
page may state less than a later one, but it should not state a *reason* the
later page refutes.

Correction: replace `01:357-359` with the consequence rather than the
mechanism — that the displaced siblings move highest ordinal first so that a
mutation stopped between moves leaves a gap rather than two siblings sharing an
ordinal, matching `08:100`. The following sentence about `Plan::guarded`
(`01:359`) is unaffected and remains correct.

### 7. `08:174` states a count of four omissions and then names a fifth

`08-invariants-and-trade-offs.md:174` opens "The models omit four material
boundaries." Four sentences follow, one per boundary: filesystem bytes and the
directory-rename guarantee, walk-order tie-breaking, the uncooperative writer,
and a coarser parts `Eq`. Then, after one intervening sentence, `:180` adds a
fifth: "String-specific failures such as non-UTF-8 names and multi-component
rendering are also outside the models and are exercised at the Rust boundary."

A reader counting what is outside the models finds five. The count is also the
kind of claim that goes stale the moment anything is added to the list.

Correction: drop the number — "The models omit these material boundaries." —
and move `:180`'s sentence into the enumeration, before the "crate tests cover
those seams" sentence that currently separates it from the list it belongs to.

### 8. `01:49` presents a consumer-specific filename spelling as a general rule

`01-orientation.md:48-50`: "An **ordinal** is the mutable position among
siblings in one level. It is the leading number in the syllabus names and
changes when insertion shifts an entry. A **key** is tree-wide identity, written
after `i`; it remains unchanged by shifts, moves, and rewrites."

The ordinal sentence scopes itself ("in the syllabus names"). The key sentence
does not, so "written after `i`" reads as a property of keys rather than of the
syllabus grammar. The book then contradicts it at its next concrete example:
`02-name-seam.md:36` renders a key as `k6` in `02-draft-plan-k6.note`, and the
grammar that licenses this — "a hypothetical document consumer whose grammar is
`<ordinal>-<state>-<label>-k<key>.note`" — is not stated until `02:154`, a
section later. Between `02:36` and `02:154` the reader has two contradictory
key spellings and no rule distinguishing them; `02:503` then returns to `-i5`
without comment.

The book's own position is at `01:53` — "the library never interprets the
consumer's parts" — and at `02:37`, "`Ordinal` and `Key` wrap `u32` without
imposing a filename format".

Correction, two clauses: scope `01:49` the way `01:48` is already scoped
("written after `i` in the syllabus names"), and at `02:36` name the grammar at
its first use rather than at `02:154` — that this is a second consumer whose
grammar spells the key `k6`, since the sigil and extension are the consumer's
choice and not the library's.

### 9. Page 07's fragment introductions compress five questions into one ~48-word sentence, six times

Six consecutive introductions on `07-syllabus-cli.md` use one sentence of the
form "The X range is present because A, B becomes C, D, and E": `:191`
(`cli-command-line`, 54 words), `:700` (48), `:821` (50), `:992` (48), `:1154`
(48), `:1341` (48), `:1586` (48).

They answer all five contract questions, so this is a cognitive-load finding
rather than a contract violation. The cost is that the four clauses hung off one
"because" are not parallel in grammatical role, so each has to be re-parsed:

> The read-helper range is present because the consumer owns filtering and path
> rendering around generic snapshot queries, a locked snapshot becomes ordered
> records or one explicit missing-target refusal, filters never widen the
> library's name seam, and it supplies the read half of the same consumer that
> the insert example exercises.

Clause 1 is a subordinate clause of "because"; clause 2 is an independent
statement; clause 3 is a different independent statement; clause 4 attaches "it"
to the range rather than to the filters that immediately precede it.

Correction: split each into the two-sentence shape pages 02, 04, and 05 use — an
ownership sentence, then a "This fragment turns A into B, establishes C, and D"
sentence. No content changes; the same five answers survive in a form that does
not require backtracking.

### 10. Page 05 uses `Level::Created` and `Landing` in prose without introducing either

`05-mutation-algebra.md:124` describes the destination vocabulary without naming
the type: "A plan names a destination level as the root, a snapshot node, or a
node created by an earlier effect in the same plan." The type is then used as
though it had been introduced, at `05:433` (`A Level::Created begins empty`),
`05:846` (`Level::Created(0)`), and `06:577` (`Level::Created(effect_index)`).

`05:598` is the single occurrence of `Landing` anywhere in the book's prose:
"The separate `Landing` sequence is necessary because two species-specific
vectors cannot reconstruct an interleaving such as promotion's create, move,
create." The reader is asked to accept a necessity argument about a type the
prose never identifies.

Both types are defined in the source fence that follows each sentence
(`plan.rs`'s `enum Level` inside `plan-effects`; `report.rs`'s `enum Landing`
inside `report-structure-and-order`), so the fact is on the page — which is why
this is minor. It is still the one place the book relies on the fence to
introduce a name its prose has already spent, and it is out of step with the
care taken everywhere else.

Correction: name `Level` and its `Created` variant in `05:124`, and at `05:598`
either gloss `Landing` in the same clause or write the claim in terms of
`paths()`, which the reader already has from the report meaning printed directly
above.

### 11. `concept-index.md` ordering breaks at the end

`concept-index.md:9-65` lists concepts in page order — page 01 entries, then 02,
03, 04, 05, 06, 07, 08. Then `:62-65` appends four more, after the page-08
block:

- `Package and dependency boundary` → `01-orientation.md#package-contract`
- `Public crate surface` → `01-orientation.md#public-surface`
- `Complete insert tour` → `01-orientation.md#insert-tour`
- `Fragment notation` → `README.md#reading-fragments`

Page 01's concepts are therefore split: two at the top of the index and three at
the bottom, below page 08's. A reader scanning for the insert tour looks in the
first four lines and does not find it. The links all resolve, which is why
mechanical validation does not see this.

Correction: move the three `01-orientation.md` entries into the page-01 block in
anchor order (`purpose-and-boundary`, `working-vocabulary`, `package-contract`,
`public-surface`, `insert-tour`), and either place `Fragment notation` first, as
`README.md` precedes every numbered page, or set it apart under its own
sub-heading as the one non-page entry.

### 12. `08:228-241` records one authoring run's sandbox workarounds in the reader-facing book

`08-invariants-and-trade-offs.md#final-verification` ends with two paragraphs of
harness narrative: Seatbelt returning `EPERM` for the non-UTF-8 filename test,
`TMPDIR=/private/tmp` fixing that test but breaking the CLI-contract tests'
child process, and the Alloy run requiring `ALLOY_JAR` set to a jar copy with
its native-solver service registration removed.

This is deliberate. `book-assembly-k35` wrote it under an explicit instruction
to record what actually ran and its actual result, and the honesty is right —
the alternative, a bare "all checks pass", would be the failure mode the book
itself quotes at `01:105`. The finding is about placement and level of detail,
not about the disclosure.

The audience contract (`README.md:4`) is a developer learning
`ordinal-fs-tree`. These paragraphs are about neither the library nor its
consumer; they describe one macOS sandbox on one day, and they will read as
noise, or as a defect in the crate, to every later reader. The durable facts —
that every suite including the doc tests passes, and that one test's *setup*
cannot create a non-UTF-8 filename under a sandboxed temp root, which is a
harness path policy rather than a crate assertion — are worth keeping.

Correction: compress `:228-241` to two or three sentences carrying exactly those
durable facts, and leave the reproduction detail where it already lives and
belongs, in `11-DONE-integrate-review-impl-book-assembly-k35.md`'s running log.
If the integrator judges that the book should carry the full record, that is a
defensible call and this finding can be closed as an accepted trade-off with a
line in its running log — but the two positions should be chosen between rather
than left to the current placement.

Also within this section: "6,929" and "fifteen" are each stated twice, at `:208`
and `:243`, sixteen lines apart. The second occurrence adds nothing the first
did not and is a candidate to fold into the surrounding sentence about what the
check reports.

## Decisions (running log)

No in-session reviewer was materialised: `references/execute.md` gives a
`review-*` session none, because the session is itself the adversarial read.

The whole book was read in canonical order rather than the producer diff, as the
task requires. No repository file was edited, and no test, build, lint, format,
or validator command was run; the mechanical observations here (fragment
introduction structure, heading structure, index ordering, cross-page example
values) were established by reading the eleven book files and comparing their
contents directly.

Quoted source inside four-backtick fences was excluded from the prose-contract
assessment. `docs/specs/ordinal-fs-tree-book.md` *Status and authority* makes
the crate source authoritative and quoted exactly, so the crate's argumentative
comment voice is not a style finding against the book. Holding it to the book's
declarative contract would have produced a large class of false positives.

Findings 1 and 2 are the editorial residue of `book-assembly-k34`'s findings 6
and 3. Both were verified as narrowly-scoped-but-correct fixes rather than
regressions: `03:49`'s false cross-page promise is gone, and all 68 literal
fragments now have prose introductions. What remains in each case is the half
that `book-assembly-k34` explicitly assigned to editorial review — the reader's
experience of the divergence, and the quality rather than the presence of the
introductions. Finding 2's seven directives are disjoint from the five that
`book-assembly-k35` fixed.

Finding 12 concerns content a prior session wrote deliberately under
instruction. It is reported as a placement judgement with the trade-off named
and the alternative stated, not as a defect, and the integrator may close it as
an accepted trade-off.

The findings are substantive, so an adjacent `integrate-review-impl` leaf is
appended with `**Integrates:** book-assembly-k36`.
