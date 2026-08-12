# batches-k34

**Integrates:** `batches-k33`

## Goal

Repair the classification batching plan before `spine-k21` executes. Apply all
seven findings from `batches-k33` to the node brief and the twelve batch bodies;
do not write any `content/` marker in this session.

The repaired plan must have one unambiguous owner for every cross-file edge,
stable semantic region boundaries, and explicit decisions for the repeated
load-bearing rules. The integration reshapes planning artifacts only.

## Context

Read `batches-k33`'s `## Review findings` in full. The actionable findings are:

1. **F1 — mutable coordinates.** Earlier batches insert marker lines above later
   line ranges in `SKILL.md`, `TASK-FORMAT.md`, and `driving.md`. Make headings
   and predecessor `pending-*` ids authoritative; label line ranges and byte
   counts as pre-classification orientation only.
2. **F2 — lossy pending-edge ledger.** A `defers=` member parked on a coarse
   residual can be silently dropped while another inbound path keeps the target
   reachable. Forbid edges whose source is `pending-*`. If the source is already
   carved, the target/body batch owns the inbound edge; if it is pending, the
   later source batch owns the outbound edge. Remove redistribution as a
   protocol and give every edge one owner.
3. **F3 — filename grep is incomplete.** In particular,
   `prompts/continue.md:2-6` refers to “the skill's Decompose step” without the
   literal filename. Add a semantic cross-file/duplicate-rule inventory and
   reconcile it alongside the filename sweep.
4. **F4 — repeated-rule calls are incomplete and backwards.** Inventory and
   pre-decide the triggering/procedural ownership for every site of reviewer
   ownership, externalize-vs-absorb, ADR reworking, and spec current-state. The
   review names missing sites in `TASK-FORMAT.md` and a second `SKILL.md` site;
   include those, not only the original four-row table. With the calls settled,
   the existing batch order may remain.
5. **F5 — overbroad greenness lemma.** Rename (D), (R), and (T) as the complete
   cross-unit deferral-graph obligations, not all obligations a subdivision
   creates. List the local marker/id/kind rules each batch still owns.
6. **F6 — `## Reference files` false binary.** Decide its actual class and
   inbound trigger, or explicitly carry it as a narrative/design finding for the
   aggregate review. Do not leave the final child choosing only between eight
   unconditional edges and zero.
7. **F7 — incorrect coverage explanation.** The 284-byte residue is a 281-byte
   YAML preamble plus the three separator newlines at baseline `SKILL.md` L246,
   L407, and L609. Assign the separators to adjacent semantic regions and correct
   the region byte counts.

`batches-k33` dismissed the size-spread and non-contiguous-`kinds-k22` doubts;
preserve those shapes unless a finding above independently forces a change.

## Done when

- The node brief's batching contract states the stable boundary and unique edge
  ownership rules, with a complete semantic inventory for implicit edges and
  repeated rules.
- All twelve child bodies agree with that contract; none tells a later session
  to redistribute a `pending-*` list or execute against a mutable line number.
- The four repeated-rule families have explicit, corpus-wide decisions naming
  every known site and each triggering owner/procedural target.
- The greenness lemma is narrowed, the `## Reference files` issue is resolved or
  deliberately handed to aggregate review, and the coverage arithmetic assigns
  all non-preamble bytes.
- No `content/`, Rust, or test file is edited. The next live leaf remains
  `spine-k21` after this integration retires.

## Notes

- This is `integrate-review-planning`: verify each finding against the corpus,
  then repair the plan. If a finding is wrong, record the concrete reason in
  this body rather than silently skipping it.
- Keep the producer's useful batching work. The review did not find a reason to
  merge batches or move `TASK-FORMAT.md` L473–501 out of `kinds-k22`.

## Integration record

All seven findings verified against the corpus and applied. Six are confirmed as
stated; **F3's mechanism is confirmed but its named instance is not a lost edge**, for
the concrete reason below. Dismissals 5 and 6 preserved: no batch merged or split, and
`kinds-k22` keeps both of its disjoint regions. No `content/`, Rust, or test file was
edited. No leaf was created, so the next live leaf remains `spine-k21`.

### F1 — mutable coordinates: confirmed, repaired

Trivially true and unavoidable: marking inserts lines, so batch #1's markers move every
later `SKILL.md` line before #9 opens the file. Repair is an explicit authority order —
the consumed `pending-*` unit first, the semantic anchor second, baseline coordinates
last and labelled as orientation. **All nine region anchors were verified unique in
their own file** (`grep -Fc` returns 1 for each, mermaid fence included), so an anchor
match needs no line number to disambiguate. Every child body now states its region as
`<start anchor>` → line before `<end anchor>`.

Also added, because F7 turns out to depend on it: a **marker-placement convention** —
the marker goes immediately above its unit's first prose line, so a blank separator
belongs to the *preceding* unit. That makes separator ownership mechanical instead of a
per-batch judgement.

### F2 — lossy pending-edge ledger: confirmed, repaired

Confirmed as stated, and the two-owners defect was verbatim in the child bodies:
`guides-k24` and `doubt-moves-k27` both offered "park it on the `pending-` unit **or**
leave it to the later batch and say so". Repair:

- **`pending-*` units never carry `defers=`** — pure coverage placeholders. The
  redistribution protocol is gone.
- **The later-carved endpoint's batch owns the edge.** Ownership is now a fact about
  the batch table, not a hand-off.
- **No procedural unit may be rooted from a unit that does not state a condition it
  answers** — added because F2's *deeper* lesson is that reachability is satisfiable
  dishonestly, and an artificial root is how a redundant inbound path appears in the
  first place.

**One thing the review did not anticipate, and it constrained the F4 work.** Forbidding
`pending-*` sources means a procedural unit must be rooted from a triggering unit that
*already exists at the end of its own batch* — so pre-deciding a repeated rule's owner
is not free: the owner must sit no later than every batch creating one of its bodies, or
that batch goes red. Three families needed that check. Two resolved by choosing the
earliest complete site as owner (A, F); one resolved because the corpus supplies an
honest same-file root for the early body (`driving.md`'s framing unit for family B;
§*When to retire research into ADRs…* for family C). Both are recorded in the brief as
inventory rows rather than left to a batch to notice.

The review's F2 proposal — "if the source is already carved, the target/body batch owns
the inbound edge; if it is pending, the later source batch owns the outbound edge" — is
exactly the later-endpoint rule, adopted in that shorter form.

### F3 — filename grep incomplete: mechanism confirmed; the named instance owes no edge

The mechanism is real and the repair is the semantic **edge inventory** (33 rows), which
each batch reconciles row by row *in addition to* running the sweeps. The sweeps are now
labelled evidence, not completeness.

**But `content/prompts/continue.md` L6 is not a lost edge.** Its target is `SKILL.md`
`**Decompose.**`, which the family-B call makes the **owner** — `class=triggering`. A
`defers=` naming a triggering unit is a build error (spec, *A unit names the procedure it
defers to*), and no edge is needed: a triggering unit ships in every mandate, so the
condition reaches the session anyway. The review's claim that `finish-cycle-k32`
"creates no procedure, and lists no edge back to the Decompose body" describes the
correct outcome, not a silent failure.

That does not weaken F3. A sweep that cannot see a reference cannot classify it *either*
way, so the finding stands on its mechanism; what changes is that this instance
illustrates the blind spot rather than demonstrating a lost edge. Both halves are
recorded in the brief and in `finish-cycle-k32`, so no batch spends a cycle trying to
write an illegal edge.

### F4 — overlap table incomplete and backwards: confirmed, repaired, and extended

Every extra site the review named was verified present:

- reviewer ownership at `TASK-FORMAT.md` L164–177 — and it is the corpus's **fullest**
  statement (predicate plus a five-row allowance table keyed by picked session kind);
- externalize-vs-absorb inside `TASK-FORMAT.md` L80–101 — **two** sites, not one: the
  `design` bullet's drift clause (L82–83) and the `impl` bullet's parenthesis (L99–101);
- ADR reworking at `SKILL.md` L217–224 and L550–554, and `TASK-FORMAT.md` L157–158;
- the spec current-state rule at `SKILL.md` L217–224.

Repair: **six** families settled in the node brief with a per-site verdict, using four
verdicts — **owner** (triggering, one per rule), **body** (procedural, reached by a
listed edge), **second condition** (triggering, a different moment or a kind-scoped
specialization), and **mention** (a clause inside a unit about something else: no
decision, no edge). The mention verdict does most of the work — most of the "extra
sites" are table rows, index entries and per-kind parentheses, and saying so is what
stops eight batches re-litigating the same grep hit.

**Two deliberate extensions beyond the four the leaf named**, both flagged rather than
smuggled:

- **Family E — "raise ADRs sparingly"** had to be separated from C and D because it
  lives in the same paragraph and has its own site list.
- **Family F — the two shapes are built in opposite ways** was not in the review's list,
  but `shape-cutting-k30`'s body already called it "third overlap in this grove", and it
  has the identical three-file shape and failure directions. Leaving it out would have
  knowingly shipped the defect for one family while fixing it for five.

A **default for an unlisted family** is stated too, since a 145 kB corpus cannot be
proven exhaustively inventoried by a planning session: earliest complete statement is
the owner, later complete statements are bodies, everything else is a mention — and the
batch records the family and its call.

**One thing the review did not find, and it changes an outcome.** `SKILL.md` L217–227
states **four** rules and its sentence boundaries fall **mid-line** (L220 carries the end
of the spec clause and the start of the ADR-set clause). Markers are whole unindented
lines and this pass edits no prose, so **the paragraph cannot be split** and takes one
class for all four rules. It must be triggering, because two of the four ship nowhere
else at `kinds=*`. Consequence: one unit owns families C, D and E, and rows 20–23 all
leave from it. Recorded as a design finding for the aggregate review — de-fusing the
paragraph is a prose edit for a later grove.

With the calls in the brief, the existing batch order stands, exactly as F4 allowed.

### F5 — overbroad greenness lemma: confirmed, repaired

Checked against the spec's full malformation list. (D), (R) and (T) are renamed as the
complete set of **cross-unit deferral-graph** obligations, and the **local per-marker**
obligations a subdivision genuinely creates are now listed: attribute order, `class`
presence, `kinds` required-on-triggering / forbidden-on-procedural, `kinds` membership in
the nineteen, id kebab-case and embed-wide uniqueness, unindented-whole-line at neutral
fence state, and no body text before the first marker. The four properties that *are*
preserved by construction (trailing newline, EOF fence balance, path control characters,
the leading `---` block) are named as the short list they are, with an explicit warning
not to extend it — which is the generalisation the plan made and the review caught.

### F6 — `## Reference files` false binary: confirmed, settled, and carried

Settled rather than handed to the final batch: **`class=procedural`, rooted from
`prompts/continue.md`'s framing unit (row 31), writing no `defers=` of its own.** The
decisive argument is neither size nor redundancy — **the index's rows name files, and a
session cannot fetch a file.** `grove-llm methodology` addresses units by id, so an
index of filenames in a mandate promises navigation the delivery path cannot honour,
while every genuine trigger→body edge for those guides is written at its point of use.
Its eight filename mentions are recorded as a **standing sweep exclusion** for all
twelve batches.

Two by-products. The `linkuistics` prerequisite note (L746–760) is separated out as
`class=triggering kinds=*` with no `defers=` — it states three real conditions and its
targets are not embedded. And the index is **also** carried as a design finding: it is
narrative residue of the provisioned-skill era, and whether it survives at all is the
successor grove's call. F6 offered "settle it **or** carry it"; both is strictly better,
because the final batch gets a decision and the reviewer still gets the question.

### F7 — coverage arithmetic: confirmed exactly, figures corrected

Verified byte-for-byte. `SKILL.md` L1–4 is **281** bytes; L246, L407 and L609 are each an
empty line (1 byte). 281 + 3 = 284. Separators assigned to the **preceding** region per
the marker-placement convention, which moves three regions and three byte counts:

| batch | was | now |
|---|---|---|
| `execute-k29` | L167–245, 5,454 | L167–246, **5,455** |
| `shape-cutting-k30` | L247–406, 10,067 | L247–407, **10,068** |
| `lifecycle-k31` | L408–608, 13,711 | L408–609, **13,712** |

The correction is not a choice between two conventions: `batches-k13` **already used
this one** at the first `SKILL.md` boundary — baseline L166 is blank and sits inside
`spine`'s L5–166 — and dropped it at the other three. The four boundaries are now
consistent with each other and with how the parser attributes bytes.

Twelve regions now sum to **144,952**; 145,233 − 144,952 = **281**, the preamble exactly.
The arithmetic is a genuine coverage proof: every non-preamble byte belongs to exactly
one batch. Also verified in passing: `finish-cycle-k32`'s 11,621 is 10,783 (`SKILL.md`
L610–760, unaffected by the separator correction) + 838 (`continue.md`), and
`kinds-k22`'s 13,189 is 11,596 + 1,593.

### One defect neither `batches-k13` nor `batches-k33` named

`finish-cycle-k32` told the aggregate review to use **`batches-k13`'s** commit as the
pre-classification baseline, calling it "the last commit before `spine-k21` touched
`content/`". It is not: `batches-k33` and `batches-k34` both land after it. The corpus
bytes are identical across all three, so nothing was misclassified — but a reviewer
diffing from there would carry two sessions of planning churn. Corrected to the commit
retiring **`batches-k34`**, and the brief's corpus table now labels that same commit as
the baseline for every coordinate in the plan.
