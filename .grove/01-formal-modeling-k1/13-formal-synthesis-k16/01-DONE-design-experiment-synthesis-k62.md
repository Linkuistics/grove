# experiment-synthesis-k62


## Goal

Freeze Experiment 2's measurement of the two columns **as independently built**:
run the whole repository under the one runner and record it, then decide every
pre-registered hypothesis and measure against its own falsifier.

## Context

This is the parent's `Done when` items 1 and 3, and nothing else. It runs first
so that the later dispositions — which edit the catalogue and cascade commands
into both families — cannot move the artifacts this measurement is of.

The pre-registration is
[`docs/formalism-findings.md`](../../../docs/formalism-findings.md) §*Experiment
2 — pre-registration*, fixed at jj change `uwuvxpkowmpumtukrknzxqptvpklmlwp`
before `design-model-contract-k5` and revised once, legitimately, before any
model existed. **It is now closed**: a change to a hypothesis, a falsifier or a
counting rule here is a post-hoc amendment and must be recorded as one, in
place, naming what was already known when it was made.

The evidence is entries **026 – 048** plus the three scope READMEs. Entry 048 is
the one to read first: it re-tags six `quint-only` findings as `both`, and its
finding 1 bounds every count in the experiment — four Alloy sessions edited the
shared catalogue before the first Quint model existed, so the `alloy-only`
direction is asymmetric by construction.

## Done when

- `models/run.sh` has been run over the whole repository from a clean
  checkout-equivalent state, and the result — command count, coverage matrix,
  Q4 matrix, exit status, wall clock — is recorded in `models/README.md` as the
  phase's own run line. A red result is recorded as the finding it is, not
  explained away.
- Every model file's bounds, solver/backend, fairness assumptions, abstractions,
  deliberate omissions, and *what a green run does not prove* are confirmed
  present in the owning README, or the gap is recorded.
- **H4 – H10 each carry a verdict computed from their own pre-registered
  falsifier**, with the operands shown. H6 records `inconclusive` if the paired
  population is under five or the statistic ties, exactly as pre-registered.
- **M1 – M8 are aggregated** over entries 026 – 048, using entry 048's
  corrections to the M1 tags, and every figure names the entries it is summed
  from.
- Experiment 2 gains a **bounded synthesis** section: which formalism caught
  what, what neither established, cost/counterfactual/verdict, the combined
  workflow this subject would actually use, and the concrete changes to design,
  tests and docs — the last of these as a list the sibling children execute, not
  as work done here.
- The synthesis states its own limits: no control arm, H2 untestable, and the
  asymmetry entry 048 finding 1 establishes.

## Notes

This leaf decides no catalogue question and edits no `.qnt`, `.als` or
`docs/specs/semantic-contract.md`. Where the synthesis needs a disposition it
names it and hands it to the sibling that owns it.

"Neither tool reached a class the other missed" is a *result* for H5, not a
failure to measure. Record misses with the same care as hits — the
pre-registration says so, and Experiment 1's most-cited result is an unsupported
hypothesis.

## Decisions (running log)

**Entries 046 – 048 were sitting inside the pre-registration section, and the
fix is a pure block move.** `# Experiment 2 — pre-registration` opened at line
7723 with its intro, and `## What is being compared, and what is not` — its
first body section — did not follow until 9002, with entries 046, 047 and 048
wedged between them. Entries 044 and 045 are correctly under `## Entries`, which
is what shows the placement was accidental rather than a convention: the same
species of entry landed on both sides of a level-1 heading. Moved the 1,252
lines of entries above the pre-registration heading. **Controlled**: the sorted
line multiset is identical before and after and the line count is unchanged, so
the move cannot have altered a byte of content; the heading order now reads
`## Entries` → 044 → 045 → 046 → 047 → 048 → pre-registration → synthesis.
Nothing cites this file by line number — every citation in the corpus is by
entry number — so no reference is staled.

**The whole-repository run is the phase's instrument, and the six cells are run
beside it rather than instead of it.** A bare `models/run.sh` additionally
asserts coverage over the whole catalogue in one invocation, delegates to the
two `docs/ordinal-fs-tree/models/` runners (a positive control: those suites are
known green, so a repository run reporting them clean while finding nothing
anywhere else is reporting a broken instrument), and fails on any `.als`/`.qnt`
in no known scope. None of that is reproduced by six `--scope`/`--family` runs.
Both were started; the cells finish in minutes to an hour and the whole-repo run
is the long pole, because Alloy's task-tree cell alone is 6888 s of CPU.

**Green at this revision, re-run by this session rather than quoted:**
`--scope lifecycle --family alloy` 73 commands, 25 of 25 cells, exit 0;
`--scope lifecycle --family quint` 93 commands, 25 of 25, exit 0;
`--scope finish --family quint` 228 commands, 61 of 61, Q4 matrix 10 of 10,
exit 0.

**H4 is supported and the scope that fails is the one worth reading.** Finish
(8 `alloy-only` / 3 `quint-only`) and lifecycle (10 / 2) are mutually
discriminating; task-tree is not, and its `alloy-only` count computed over
findings *reachable by both columns* is **zero** — every Alloy task-tree finding
had been written into `docs/specs/semantic-contract.md` before the Quint column
read it. The prediction was *at least two of three*; the falsifier was *fewer
than two*.

**H5 is falsified on both clauses of its own second prediction, and the corpus
supplies the axis it should have named.** Exactly two findings in the whole
experiment carry `M2 eventuality` and **both are `quint-only`** — so *no
`alloy-only` finding carries the class* and *the class lands in the opposite
formalism*, which are two of H5's three named falsifying conditions. The cause
is not that Alloy 6 cannot state an eventuality: **the catalogue both columns
were written from contains almost no eventuality claims**, so that arm had
nothing to run on. The real axis is entry 048 finding 5's — a property stated
over one action's own before/after pair cannot discover that its claim was
quantified too widely; one stated over the trace can — plus
`quint-statement-shape-k61`'s second axis, that a property discharged by a
restatement of a dial is unfalsifiable at any grain.

**H6 is inconclusive, and the reason is a conflict between two pre-registered
rules rather than a thin result.** The corrected census has seven paired
(`both`) findings, above the floor of five — but M3 is scored *per
`(finding, formalism)`, at the moment of reading, before any fix*, and a finding
only becomes paired at replay, which is by construction after both fixes. Three
of the seven have Alloy counterparts that are retained counterexamples in a
model README rather than entry-recorded findings, and carry no M3 at all. The
usable paired population is **zero**. Recorded as `inconclusive` under the
pre-registered thin-result clause rather than rescued with another statistic.

**H7 is falsified, and it is the result the following phases should act on.**
`M4 = none` is the modal outcome; eight derived tests exist against roughly
forty-five material findings. The census arithmetic, re-derived from the M1 tag
lines rather than from the entry titles: Alloy **29** (3 task-tree + 12 finish +
14 lifecycle) and Quint **17** (6 + 6 + 5) = 46 column records, less the
**seven** overlaps replay collapsed = **39 distinct column findings**, plus
entry 048's own ≈ 6 = **~45**. *Two corrections to my first pass are folded in
here: entry 041's tag line covers **two** findings (`M3` — 0 for both), not one,
and entry 046's finding 4 is a fourth collapse via entry 048's finding 6, not
just the three its finding-4 table lists.* A formal phase run against an implementation that is already shipped and
green delivers **specification corrections, not code defects** — which makes
`documentation-k2`, not `implementation-k3`, the phase that consumes this work.

**H8 is supported, on the only family whose ledger can compute it.**
Component-local (Alloy, task-tree, entries 026 – 030) is 11.83 h over 41
obligations = **0.289 h/obligation**; system-level (Alloy, lifecycle, entries
040 – 043) is 9.33 h over 25 = **0.373**. Entry 043's warning is applied rather
than ignored — the four lifecycle points are a declining marginal-cost curve
inside one file — so the first-slice-to-first-slice comparison it asked for is
also given: 0.19 against 0.42, same direction. The verdict is reported only
because it does not depend on which counting rule is used.

**H9 is falsified, and the falsification is the more useful half.** M6 is `0`
in every entry that records it, and structurally so: the Alloy column had no
sibling and the Quint column was forbidden to read it. Post-barrier M6 is one
replay session plus one restatement session — bounded generously at ~4 h against
a denominator of ≥ 30 h — so it is neither ≥ 30% of total effort nor greater
than `max(authoring_alloy ≈ 21 h, authoring_quint)`. **An independence protocol
converts synchronization into replay**, and replay is cheaper in hours and buys
a measurement that keeping two models in step destroys. Its own tax is named:
a negative result from a bounded tool is ambiguous until a run disambiguates it.

**H10 is supported by the pre-registration's zero-rule rather than by a
comparison, and that is stated rather than papered over.** False-confidence
hours recorded ≈ 2.8 h; genuine-failing-check hours recorded **nowhere**, and
counted as zero by the rule that an unrecorded duration counts as zero. The
control arm was specified precisely — M8 was revised before any model existed
*because* it was one-sided — and then almost never populated. The uncounted half
points the same way and is larger: entry 048's `TT-20` tautology stood for the
life of the file, and two whole review-plus-integration chains exist because
columns that published their own incident ledgers had shipped more.

**The ledger's gaps are reported as a result, not smoothed over.** M5 authoring
hours exist for nine slices of twenty-three — nothing for the whole Alloy finish
column (031 – 039) or for any Quint column beyond entry 044's estimate. M2 and
M3 tags are absent from entries 034 – 039, which carry seven of the Alloy finish
column's twelve findings. The cause is the same in both cases and it is
transferable: **a per-finding recording obligation with no instrument behind it
decays into the shape of whatever the previous entry looked like**, and here it
decayed exactly at a leaf boundary. The runner checks obligations, not entries.

**One documentation gap, recorded rather than fixed here.** The Quint half of
`crates/grove-task-tree/models/README.md` has no *what a green run does not
prove* section of its own; the material is in entry 044's *Missed*, so the
pre-registration's recording obligation is met at the entry level and the gap is
placement. Handed to `handoff-audit-k66`.

**The four sibling children were cut by this session so their bodies could carry
the inherited items verbatim.** `obligation-placement-k63`,
`catalogue-disposition-k64`, `finish-verdicts-k65`, `handoff-audit-k66`. Cutting
them here also removes a hazard: had this leaf retired alone, the node would
have held no live leaf and `pick` would have walked past the whole disposition
into `02-documentation-k2`.

**No ADR from this leaf, and the reason is the AND test rather than restraint.**
`ADR-FORMAT.md` requires all three of *hard to reverse*, *surprising without
context*, and *a real trade-off with a rejected alternative*. The candidates
were the routing advice (*route by statement shape, not by tool*), independence
rule 4, and the `(claim, control)` review unit. **Each fails the first clause**:
they are findings about method, freely revisable by the next experiment, and
each already lands where a reader meets it — the first two in
`docs/formalism-findings.md`, the third in the review chains that produced it.
An ADR here would be a chronology entry, which is what the minimum-coherent-set
rule exists to prevent. `catalogue-disposition-k64`'s items are a different
matter and several of them will pass the test.

**No glossary entry either.** `CONTEXT.md` is grove's domain vocabulary — the
task tree, the finish protocol, the loop. *Material finding*, *mutually
discriminating* and *statement shape* are this experiment's vocabulary, each
defined at the point of use in the pre-registration or the synthesis, and
`CONTEXT-FORMAT.md`'s rule is that a glossary carries terms specific to the
project's context and nothing else. Recording the decision so a sibling does not
re-open it.

**The measurement host was contended by an unrelated build for part of this
session** — a Homebrew `gerbil-scheme` compile running sixteen `cc1` processes,
with load averages above 140 on a 16-core host. **No wall-clock figure in the
synthesis comes from this session's runs**: every timing cited is quoted from
the producing session's README, and what this session's re-runs contribute is
command counts, coverage matrices and exit status, none of which contention can
change. Recorded because a timing taken here would have been worthless and a
reader is entitled to know it was not taken.

**All six model files audited against the node brief's seven required fields;
three gaps, all in the Quint halves, all of placement.** Tool version, bounds or
trace limits, solver/backend, fairness assumptions, abstractions, deliberate
omissions, and *what a green run does not prove*. **The three Alloy halves carry
all seven.** The Quint halves state no fairness assumption anywhere (entry 044
and the catalogue's `SY-13` do), give the tool version only inside prose in two
of three, and the task-tree Quint half has no *does not prove* section of its
own. Recorded rather than fixed here, and handed to `handoff-audit-k66` — which
runs after `catalogue-disposition-k64` may have changed what those READMEs say,
so fixing now would be two passes over the same files.

**Entry 048's product claim about `reap` was re-verified at the source rather
than repeated on trust**, because `catalogue-disposition-k64` will act on it.
`grep -E "classify|RootState|PartialScaffold" src/loop_driver.rs` finds **0**;
the same pattern finds **8** in `src/tree_lifecycle.rs`, which is where the
recovery classification lives, and matches in seven other files besides. **Clean
here plus dirty there** is what makes the negative evidence rather than a broken
instrument reading clean everywhere. So the shipped driver's reap path reads no
root classification, the catalogue gap entry 046 finding 4 names is real, and
there is no product defect behind it — which is why that finding legitimately
records `M4 = none` and falsifies H7 by H7's own terms.

**Entry 048's `TT-24.c` claim re-verified at the source, by enumeration rather
than by a pattern.** `crates/grove-task-tree/models/task-tree-controls.qnt`
declares fourteen controls; extracting *all* of them and classifying — rather
than grepping for the one in question — shows `TT-24.**d**` carries
`inv_fail_EN_13_TT_24d_the_reaper_stops_declining` and `TT-24.**c**` carries
none. The two sub-obligations are **not** symmetric, which entry 048 states
correctly and which a fix treating `TT-24.c`/`TT-24.d` as one case would get
wrong. Handed to `obligation-placement-k63` with the enumeration behind it.

**Two more claims verified at the source rather than carried on inference, both
because they are repo-wide and a clean grep is not evidence for one.**

- *The Alloy task-tree file is the only one in the corpus carrying `Int`.*
  Enumerated over all four `.als` files, not sampled: `task-tree.als` declares
  an integer scope on **104** commands; `finish.als`, `lifecycle.als` and
  `docs/ordinal-fs-tree/models/structure.als` declare **none**. Control: all
  four carry `check` commands (47, 64, 30, 7), so every file was really read.
- *No Quint half of any model README states a fairness assumption.* Zero
  mentions across 1,605 lines of the three Quint halves; the three Alloy halves
  each carry one. Clean-here plus dirty-there again.

**The whole-repository run is GREEN, and this is the first time any session has
measured it end to end.** `models/run.sh`, all four scopes, both families:
**791 commands, 256 cells complete, 2 declared gaps, 0 empty, of 258**, with
both delegated `docs/ordinal-fs-tree/models/` runners executed (20 Alloy
commands, 148 Quint) — the positive control the per-cell runs do not carry. Q4's
removal matrix asserted in both directions per family: alloy 10 of 10 rows
(2 `none`, 1 abstracted), quint 10 of 10 rows (3 `none`, 1 abstracted). That
matrix result is an independent confirmation of the Q4 reading handed to
`finish-verdicts-k65`, straight from the runner rather than from a README.

**Exit 0, and the claim is established by enumeration rather than by the absence
of the word FAIL.** The run was launched with `2>&1`, so stderr is in the log.
Every site in `models/run.sh` that sets `fail=1` was enumerated — twenty-three of
them — and each emits a distinctive diagnostic: `placement error:`,
`runner error:`, `MISSING SCOPE`, `NO ROW`, *rows that do not resolve*,
*commands naming no obligation the catalogue defines*, *declared gaps in BOTH
families*, or a `FAIL ` line. Searching the log for **all** of them together
returns exactly one hit, and it is the Q4 matrix's unconditional header. The
search pattern was itself controlled against a synthetic three-line input, which
it matched three times — so a pattern that failed to compile could not have
produced the clean read.

**The run's only blemish is the placement problem, surfaced by the instrument
rather than argued in prose.** The coverage section prints two lines and no
others: `TT-24.c alloy:gap quint:ok` and `TT-24.d alloy:gap quint:ok`. Entry
048's observation — *the coverage matrix scores a transcription above an honest
declaration* — is therefore visible in the runner's own output, which is a
stronger form of the finding than the entry could give it. Handed to
`obligation-placement-k63`.

**One correction to a figure I had copied from a README.** The task-tree Alloy
file runs **104** commands, not the 103 its own README records; the extra one is
`cross-model-replay-k15`'s retained replay,
`witness_finding_a_world_write_during_an_open_scaffold_reaches_legacy`, added
when that session corrected `step`. The README's figure predates it. Recorded in
the synthesis and in `models/README.md`; correcting the scope README itself is
`handoff-audit-k66`'s, with the other README fields.
