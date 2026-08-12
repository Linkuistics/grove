# evidence-moves-k26

## Goal

Classify **`content/driving.md` from `## When to retire research into ADRs versus
leave it` to the line before `## Doubting inside a picked Grove leaf`** (baseline
L264–414, 8,528 bytes): that section, `## Reworking ADRs and briefs as understanding
shifts`, `## Verifying framework decisions against the source`, and `## Verifying a
claim about the repo itself`.

This is batch 6 of 12. The theme is **evidence discipline** — where a claim's
proof lives and when it becomes binding.

## Context

Read the node brief's *The batching contract* first.

### Region and residual

- **Anchors are authoritative; L264–414 is a baseline coordinate.** Carve from `##
  When to retire research into ADRs versus leave it` to the line **before** `##
  Doubting inside a picked Grove leaf`, consuming the front of
  `pending-driving-evidence`.
- Mint exactly one residual, **`pending-driving-doubt`**, covering `## Doubting
  inside a picked Grove leaf` to end of file, as `class=triggering kinds=*` **with
  no `defers=`**.
- **There is nothing to inherit from `pending-driving-evidence`.** A residual never
  carries `defers=`, so there is no list to redistribute and no member to account
  for. `batches-k13`'s redistribution protocol was removed by `batches-k33` F2: a
  member parked on a residual can be dropped while the build stays green, because
  the target usually has another inbound path.

### The pre-decided calls in this region

The three-way overlap on ADR reworking that `batches-k13` asked you to *decide* is
settled in the node brief. Apply it:

- **Family C body — `## Reworking ADRs and briefs as understanding shifts`
  (L285–310).** **Procedural.** Its owner is `SKILL.md` L217–227 (#9), which is
  carved *after* you — so **root it from `## When to retire research into ADRs versus
  leave it` in this same batch** (row 9). The corpus itself points there: L277 reads
  *"see *Reworking ADRs and briefs* below"*. #9 later adds the owner's address as
  row 22; that second inbound edge is a genuine condition→body address, and it is
  not yours to write.
- **Family C second condition — `## When to retire research into ADRs versus leave
  it in docs/research/` (L264–283).** **Triggering.** It states a *different*
  trigger — a research finding becoming binding on future work — so it ships on its
  own account. Its own *"you are editing the ADR **in place** — the set is
  current-state"* clause (L274–277) rides along as a **mention**: do not carve it
  out as a third statement of the rule.
- **`ADR-FORMAT.md` is already rooted** by `guides-k24` (rows 2 and 5). Row 10 —
  this region's `ADR-FORMAT.md` citation at L285ff — is yours, and it is a genuine
  second address only if the citation is a trigger→body reference rather than a
  provenance pointer. Read it and decide; **declining with a reason is a legitimate
  outcome**.
- The `linkuistics:decision-records` pointer is **not embedded** and can never be a
  `defers=` target.

### The judgement this batch exists for

All four sections are self-rooting — each names its own condition in its heading —
but the *grain* is the question. Two of them are conspicuously large relative to
their neighbours:

- **`## Verifying a claim about the repo itself`** (3,764 bytes) is the biggest
  section in the region and is mostly a worked procedure for turning a grep into
  evidence. The condition is small and sharp (*a session is about to assert
  "every X is now Y" about its own codebase*); the body is long. That asymmetry is
  the design working — keep the `if`, defer the `then` — and this section is the
  cleanest example of it in the whole corpus. Classify it that way deliberately,
  not by inertia.
- **`## Reworking ADRs and briefs as understanding shifts`** (1,914 bytes) states
  a rule stated in four other places. **The call is made for you** (above): this is
  the family-C **body**. What is still yours is the *grain* — how many units it
  becomes, and whether its three bullets (edit in place / keep the set minimal /
  reconcile every citation) are one body or three.

`SKILL.md` references in this region point at constraints, and they sit inside
`pending-skill-*`. Not `defers=`; report them as *not yours*.

### Size note

At 8,528 bytes this is the smallest `driving.md` batch. That is deliberate: the
region's grain is fine and its four sections are independent, so the session cost
is in the judgements, not the bytes. If it finishes early, **do not absorb
`pending-driving-doubt`** — `doubt-moves-k27` carries the review chain, which is
the single largest section in `driving.md` and needs its own context.

## Done when

- The region between the two anchors is subdivided into real units;
  `pending-driving-doubt` covers the rest of the file and nothing else, and carries
  no `defers=`.
- `cargo build` and `cargo test` are green.
- `EMBEDDED_UNITS` updated in the same commit, each new id named deliberately.
- **Rows 9 and 10 are reported** — row 9 written (it is what makes
  §*Reworking ADRs…* reachable at the end of *this* batch), row 10 written or
  declined with a reason.
- The family-C body's id is named in this leaf's body, so #9 can write row 22
  without re-deriving it.

## Notes

- `## Verifying a claim about the repo itself` contains fenced shell examples.
  Do not split mid-fence.
- Doubts to carry forward, by id.

## Batch record

Region carved between the two anchors. **6 units minted, `pending-driving-evidence`
consumed, one residual minted** (`pending-driving-doubt`); `EMBEDDED_UNITS`
87 → 93. `cargo build` green; `cargo test` green — exit 0, 39 test binaries plus
doc-tests, 0 failures in every one (595 lib unit tests, `tests/methodology.rs` 14
passed, `tests/composition_guidance.rs` 16 passed).

**The family-C body's id is `driving-reworking-adrs-and-briefs`** — that is the
answer `execute-k29` needs for row 22, and it is deliberately the corpus's own
name for the section (baseline L277 reads *"see *Reworking ADRs and briefs*
below"*, which row 9 has now turned into an address).

### The units, in document order

Ranges are pre-classification baseline coordinates; byte counts are prose only,
excluding the marker line.

| id | region (baseline) | class | scope | defers |
|---|---|---|---|---|
| `driving-when-to-retire-research-into-adrs` | L264–284, 1,096 B | triggering | `*` | `driving-reworking-adrs-and-briefs` |
| `driving-reworking-adrs-and-briefs` | L285–311, 1,627 B | procedural | — | — |
| `driving-when-code-depends-on-a-framework-version` | L312–324, 709 B — licence comment included | triggering | `*` | `driving-cite-framework-decisions-to-the-source` |
| `driving-cite-framework-decisions-to-the-source` | L325–348, 1,332 B | procedural | — | — |
| `driving-when-asserting-a-repo-wide-claim` | L349–356, 387 B | triggering | `*` | `driving-turning-a-sweep-into-evidence` |
| `driving-turning-a-sweep-into-evidence` | L357–414, 3,377 B | procedural | — | — |
| `pending-driving-doubt` | L415–754, 20,636 B | triggering | `*` | — (never) |

Three condition/body pairs, and that is the whole shape of the batch: **the
region's four sections became three `if`/`then` pairs and one that was already a
pair** (§*When to retire research…* keeps the `if` and §*Reworking ADRs…* is its
`then`, which is what pre-decided family C says). 2,192 B of condition ships in
all nineteen mandates; 6,336 B of procedure ships in none.

### Coverage proved by reconstruction, and the vacuous-check trap avoided

`research-moves-k25` flagged that `jj file show '@-' <path>` reads both arguments
as paths and silently diffs the working copy against itself. The `-r` form was
used, and the fetched baseline's byte count was confirmed against the node brief's
corpus table (42,744 for `driving.md` at the `batches-k34` commit) **before** any
comparison — so the instrument was proved non-blind first. That is
§*Verifying a claim about the repo itself*'s own positive control, applied to the
batch that classified it.

- the 6 units, fetched in one `grove-llm methodology` call and stripped of their
  marker lines, are **byte-identical to baseline L264–414** (8,528 B — the leaf
  brief's figure exactly);
- `pending-driving-doubt`, likewise stripped, is **byte-identical to baseline
  L415–754** (20,636 B);
- 8,528 + 20,636 = 29,164 = the consumed `pending-driving-evidence`'s coverage, so
  there is no gap and no overlap;
- the whole file with every `<!-- unit: ` line removed is **byte-identical to the
  pre-batch file likewise stripped** (42,696 B both sides) — no prose, filename or
  fence moved, and the trailing newline and fence balance are untouched. Every
  changed line in `content/` is a marker line (`jj diff --git content/ | grep -E
  '^[+-][^+-]' | grep -v '<!-- unit: '` is empty).

### Edge inventory rows owned: 9, 10 and 38

| row | source | target | outcome |
|---|---|---|---|
| 9 | `driving-when-to-retire-research-into-adrs` (triggering `*`) | `driving-reworking-adrs-and-briefs` | **written** |
| 10 | `driving-when-to-retire-research-into-adrs` | `ADR-FORMAT.md` bodies | **declined — three reasons below** |
| 38 | `task-producer-impl` (triggering `kinds=impl`) | `driving-cite-framework-decisions-to-the-source` | **written** |

**Row 10 declined.** Three independent reasons, the first of which is decisive:

1. **The citation the row is predicated on does not exist.** The leaf brief says
   *"this region's `ADR-FORMAT.md` citation at L285ff"*, and there is **no
   `ADR-FORMAT.md` reference anywhere in `driving.md`** — not now, and not at the
   `batches-k34` baseline (`jj file show -r <baseline> content/driving.md | grep -n
   ADR-FORMAT` → nothing; all six corpus hits are in `TASK-FORMAT.md` ×1,
   `grilling.md` ×2 and `SKILL.md` ×3 — the cross-tree control, so the pattern is
   not simply blind). What sits at that address is the
   `linkuistics:decision-records` pointer, which the leaf brief itself excludes as
   *"not embedded and can never be a `defers=` target"*. The row's own test —
   *"a genuine second address only if the citation is a trigger→body reference
   rather than a provenance pointer"* — has no citation to apply to.
2. **The condition's own answer is the in-place rework mechanics, not placement.**
   §*When to retire research…* is about an existing ADR — *"cited in the relevant
   ADR's rationale section"*, *"you are editing the ADR **in place**"*. Its `then`
   is row 9's target. `ADR-FORMAT.md`'s bodies answer *where a new ADR goes* and
   *why the set stays small*, which is family E's when-to-write question and has
   its own owner.
3. **Redundant in delivery, which is the shape the plan warns about.**
   `adr-placement-note` is already addressed from a `kinds=*` chain today —
   `driving-when-to-invoke-grilling` (`*`) → `grilling-interrogate` →
   `grilling-offer-adrs-sparingly` → `adr-placement-note` — and row 20 will add
   `execute-k29`'s direct `kinds=*` address. So no session lacks an address, the
   brief's justification for a second inbound edge (*"or the second condition's
   session has no address for it"*) does not fire, and writing it would add a
   fourth path to a body whose real edges could then be dropped while the build
   stayed green. This is `research-moves-k25`'s declined `task-two-shapes` →
   vendor-pair edge, same reasoning.

**Row 38 written, and it is the member `research-moves-k25` could not write.**
`task-producer-impl`'s parenthesis names three habits in three batches;
*"cite framework decisions to the source"* is mine, and the id
`driving-cite-framework-decisions-to-the-source` is deliberately the source
prose's own words, so #7's and #8's members are discoverable by the same route.
The marker gained one member and nothing else — the decoupling lemma's easy
direction, no id, class or boundary changed.

**Inventory additions — two, recorded in the node brief as rows 41–42** rather
than only here, because their owner is a **later** batch (#7) that will not read
this body. Both of my procedural bodies end by naming the doubt pass as the paired
procedure (baseline L305 and L346); the target is §*Doubting inside a picked Grove leaf*,
so the later-endpoint rule makes both #7's. They are **intra-file**, so #7's
filename grep cannot see them — the same silent shape as rows 38–40, and the
reason the inventory exists.

**One edge considered and declined:** `driving-when-code-depends-on-a-framework-version`
→ `driving-how-to-write-a-research-leaf-brief`. My condition opens *"The
research-leaf discipline … is not only for research"*, which reads like a pointer
at that body — but the two answer different questions. The research-leaf body is
how to *write a research leaf's brief*; my condition's `then` is how to cite a
framework decision in code, which is its own body. Naming it would be
`guides-k24`'s "root that answers nothing" in miniature.

### The pre-decided calls, applied not re-decided

- **Family C body** — `driving-reworking-adrs-and-briefs`: procedural, rooted from
  §*When to retire research…* in this batch (row 9). #9's row 22 is not mine.
- **Family C second condition** — `driving-when-to-retire-research-into-adrs`:
  triggering, ships on its own account. Its *"you are editing the ADR **in
  place** — the set is current-state"* clause (baseline L274–277) **stayed inside
  it as a mention**; it was not carved out as a third statement of the rule. That
  is also why the section's middle paragraph could not be deferred: moving it
  would have moved the clause into a unit that ships nowhere.
- **`ADR-FORMAT.md`** — already rooted by `guides-k24`; nothing added (row 10).

### A repeated-rule family the inventory does not list

Recorded per the node brief's default rather than decided freehand.

> **H — the citation discipline: a citation per claim, primary sources, an explicit
> note for what you could not verify.** Sites: `TASK-FORMAT.md`
> `task-research-pair` (#2, `kinds="research-a research-b"`) — states it completely
> and earliest → **Owner**; `driving-how-to-write-a-research-leaf-brief` (#5) —
> the expansion for a research leaf's brief → **Body**;
> `driving-when-code-depends-on-a-framework-version` (#6) → **Second condition**,
> `kinds=*`. The second condition is not duplication: its trigger is *writing code
> whose correctness depends on a library version*, which is a different moment from
> *running a survey*, and the section's own first sentence says so outright
> (*"is not only for research"*). `TASK-FORMAT.md` L118's phrasing is the owner's
> own prose, not a fourth site.

### The three grain judgements this batch existed for

**§*Verifying a claim about the repo itself* is one small condition and one long
body — 387 B shipping everywhere, 3,377 B shipping nowhere.** The leaf brief asked
for this to be decided deliberately, so: the condition is the opening paragraph,
which names the situation (*sessions assert "every X is now Y" constantly*) and the
reason it is dangerous (*every failure mode produces a clean-looking result*), and
that is a complete `if`. The boundary is above **"Check the output resembles what
you asked for"** — the first imperative, and the lead-in to the four failure-mode
bullets. A session that receives the condition and nothing else knows there is a
procedure and has its address; a session that never receives it greps, gets a clean
result, and asserts. 8.7:1 is the widest asymmetry in the region and the design
working as intended.

**Its body stayed one unit.** The temptation was four or five — failure modes / the
two controls / enumerate-then-classify / the three silent narrowings / the two
recording rules — and `guides-k24` established that fine grain is close to free
(`grove-llm methodology` takes several ids in one call, so a `defers=` list is the
argv of one fetch). Two things decided against it. First, `guides-k24`'s boundary
rule is *carve at a heading or a distinct block*, and this body has **no heading
and no `###`**: it is bold-lead paragraphs, which is exactly the shape
`research-moves-k25` kept whole in §*How to write a research leaf brief* ("one
addressee and one subject"). Second, and specific to this body, the parts are
**mutually referential**: failure-mode bullet 4 ends *"only the control pair
catches it"*, which is a dangling reference in any unit that does not also carry
the controls paragraph — and a procedural unit is fetched alone, so a split would
have produced exactly the standing-alone defect no build checks.

**§*Verifying framework decisions against the source* took the same shape, split
above "For that kind of code — and only that kind…".** The condition is the
opening paragraph; the exclusion clause went with the body because the positive
statement already carries the scope (*"whenever you write framework- or
library-specific code whose correctness depends on the version"*), so the condition
is complete without it, and putting the clause with the body keeps the colon
attached to the list it introduces. The alternative boundary — one line lower,
between the colon and the first bullet — would have left the condition ending in a
colon pointing at nothing.

**§*Reworking ADRs and briefs* stayed one unit, which is the grain question the
leaf brief left open.** Its three bullets are **items in one list** introduced by
*"Instead:"* — `guides-k24`'s rule forbids carving inside one list, and
`CONTEXT-FORMAT.md`'s eight `## Rules` bullets are the precedent. Bullet 1 read
alone also loses the *instead of superseding* contrast that is the whole point of
the rule. The closing paragraph (*"The natural checkpoints are the two the loop
already has…"*) stayed in the body too: it names *when in the loop* to run a
procedure whose trigger the owner and row 9's source already state, so it is part
of the `then`, not a fourth condition.

**§*When to retire research into ADRs…* stayed one unit** for the reason under
*The pre-decided calls* above, plus the same addressee-and-subject test: all three
paragraphs address the session holding a research finding, about one subject (how
that finding relates to the ADR set). The "Findings adopted" bridge paragraph was
the only candidate for a carve and it is two sentences.

### The one scope call, and the residue it accepts

**`driving-when-code-depends-on-a-framework-version` is `kinds=*`, not `kinds=impl`.**

The narrow reading is written into the prose — *"It applies to **`impl` tasks**
too"* — and `task-producer-impl` is already `kinds=impl`. That is precisely the
argument against `kinds=impl` here: **row 38 already delivers this to `impl`**, so
the only thing the condition unit adds over row 38 is reach to the kinds row 38
cannot reach. Scoping it `impl` would make it a second `impl`-only statement of a
trigger and leave every other kind unable to ask.

And the other kinds can genuinely act. `prototype` writes framework-specific code
(a throwaway built on a deprecated API misleads the reaction it exists to provoke);
`integrate-review-impl` applies code fixes; `design` writes specs naming framework
APIs; `review-impl` can notice a missing citation, and fetching official docs is
**reading, not running** — the section says so itself, so it does not breach the
inspection-only discipline. The node brief's default settles the rest: `kinds=*` is
for anything that is not one kind's discipline, and this is a situation, not a
discipline.

**Accepted residue:** eighteen mandates receive a paragraph whose first sentence
names `impl` explicitly, which reads slightly off-target for, say, a `finish`
session. Same species as `research-moves-k25`'s accepted residue on
`driving-when-not-to-start-a-grove`, and one paragraph wide.

### The sweeps

```
grep -rn 'driving\.md' content/          # inbound
```

Seven hits, the same seven `research-moves-k25` enumerated. **One is mine:**

- **`TASK-FORMAT.md` L108** — `task-producer-impl`, row 38, **written**.

Six are not, and none is missed:

- **`TASK-FORMAT.md` L224** — inside `task-two-shapes`, pointing at §*Doubting*.
  #7's, and #7 should read it against rows 11 and 39 as a redundancy call.
- **`SKILL.md` L236, L252, L280** — all inside `pending-skill-loop`. **No edge may
  have a `pending-*` source**; rows 23 and 19 (#9) and #10's territory.
- **`SKILL.md` L755** — the `## Reference files` index, whose text names this batch's
  first section outright (*"and when research findings retire into ADRs"*).
  **Standing sweep exclusion**, settled in the node brief.
- **`BRIEF-FORMAT.md` L81** — both endpoints are #8's.

A second sweep was run for the region's *subjects* rather than its filename, since
two of my four sections are cited nowhere by name:

```
grep -rn 'primary source\|source-driven\|doubt pass\|Context7\|manifest\|ripgrep\|positive control\|cross-tree' content/
```

One hit outside `driving.md`: **`TASK-FORMAT.md` L118**, `task-research-pair`'s
citation discipline — family H's owner, above. Nothing in the corpus discusses
verifying a claim about the repo, so `driving-turning-a-sweep-into-evidence` has
exactly **one** inbound edge and no hidden second path.

Outbound, inside the region — **one cross-file reference, not written:**

- **baseline L308–309 (current L320–321)** — *"**retiring** a leaf or node
  (`SKILL.md`'s Plan and Retire steps)"*, inside
  `driving-reworking-adrs-and-briefs`. The target
  is `SKILL.md` `**Retire.**`, which is inside `pending-skill-*` and is carved by
  **#11 `lifecycle-k31`** — the later endpoint, so the edge is #11's to write or
  decline. Reported as *not mine*; nothing parked. **#11 should note that the
  citation is stale as well as pending:** `SKILL.md` has no *Plan* step
  (`grep -n '^\*\*[A-Z][a-z]*\.\*\*' content/SKILL.md` → Pick, Bootstrap, Execute,
  Decompose, Retire, Commit, Signal, Finish), so the parenthesis names one step
  that exists and one that does not.

Two further outbound references are **intra-file** and forward, into #7's region;
they are rows 41–42 in the node brief, not sweep hits (the filename grep cannot see
a reference that names no file).

### Doubts, by id — for `finish-cycle-k32`'s aggregate handoff

1. **`driving-turning-a-sweep-into-evidence` (grain).** The one unit in this batch
   that a reviewer could reasonably want split, and the reasoning above is the case
   for the fusion, not a claim it is obvious. 3,377 B is the largest procedural
   unit `driving.md` has so far. The honest seam, if the reviewer wants one, is
   above *"**Never document a claim with a count of itself.**"* (baseline L405):
   everything below it is about the *durable statement and the edit that follows*
   rather than about running the sweep, and it carries no backward reference, so
   splitting there is the decoupling lemma's easy direction. What I would not split
   is the failure-modes/controls pair, for the dangling-reference reason.
2. **`driving-when-code-depends-on-a-framework-version` (scope).** `kinds=*` on a
   709 B unit whose prose names `impl`. Narrowing it to
   `kinds="impl prototype integrate-review-impl integrate-review-prototype"` is the
   competing call and is a one-attribute edit; what it loses is the `design`
   session's spec asserting a framework fact and the `review-impl` session
   noticing an uncited one. Row 38 keeps the `impl` path either way, which is what
   makes the narrow option survivable and the call debatable rather than forced.
3. **Row 10's decline (`driving-when-to-retire-research-into-adrs` has no
   `ADR-FORMAT.md` address).** Reason 1 is factual and settles the row as written,
   but a reviewer could hold that the *semantic* edge is real regardless of the
   missing citation — a research finding that warrants a **new** ADR does need
   placement conventions. I read that as family E's question with its own owner
   (#9's row 20, `kinds=*`), so no session is left without an address. If the
   reviewer disagrees, adding `adr-placement-note` to this unit's `defers=` is one
   line and changes nothing else.
4. **`driving-when-asserting-a-repo-wide-claim` opens *"The counterpart to the
   section above."*** A positional reference in a unit that ships **alone** into a
   mandate, where there is no section above. It is the region's clearest instance
   of the standing-alone rule no build checks, and it is unfixable while
   marking — the fix is a prose edit. The unit is still complete without the
   sentence, which is why it was not a reason to fuse S3 and S4.

### Design findings

- **Mandate delivery separates adapted prose from its attribution, and the corpus
  already contains the case.** `driving.md`'s addyosmani licence comment attributes
  *"the two sections below"* — but the two are **non-adjacent**:
  §*Verifying framework decisions against the source* (source-driven-development)
  and §*Doubting inside a picked Grove leaf* (doubt-driven-development), with
  grove's own §*Verifying a claim about the repo itself* between them. A comment is
  one contiguous block, so it can only travel with the first; it now opens
  `driving-when-code-depends-on-a-framework-version`, and #7's doubt units will
  carry none. Two consequences worth the reviewer's attention: the comment's
  *"two sections below"* now reads, in the file, as the next two sections (one of
  which is grove's own), and a session served the doubt unit alone receives adapted
  MIT-licensed prose with no attribution attached. `LICENSES/` still carries the
  licence, so this is an attribution-locality question rather than a compliance
  one — but it is the first place where unit delivery makes provenance a
  *classification* concern. Recorded in the node brief too, so #7 does not
  rediscover it.
- **A stale cross-reference the classification cannot fix and should not hide.**
  Baseline L308 cites *"`SKILL.md`'s Plan and Retire steps"*; there is no Plan
  step. Left alone (this pass edits no prose) and handed to #11, which owns both
  the edge and the paragraph the citation points at.
- **Three baseline coordinates in this leaf's own brief were wrong, in the way the
  contract predicts.** Recorded because the aggregate reviewer is auditing the
  plan as much as the marking. (a) Row 10's *"`ADR-FORMAT.md` citation at L285ff"*
  does not exist — that address holds the `linkuistics:decision-records` pointer,
  and the brief's own next bullet excludes it. (b) The Notes warn that
  §*Verifying a claim about the repo itself* *"contains fenced shell examples"*;
  `driving.md`'s only fences are at baseline L135–137 (#5's, already handled) and
  L468–474, which is inside **#7's** region — this region has no fence at all, and
  the shell in it is inline code. (c) The family-C table's ranges stop one line
  short at the tail (L264–283 and L285–310 for sections that actually end at L284
  and L311) — the same separator-ownership slip `batches-k34` corrected at the four
  `SKILL.md` boundaries and did not re-check here; its 1,914 B figure for
  §*Reworking ADRs…* is 1,627 B plus the 287 B licence comment that belongs to the
  next section. None of the three changed a decision, because all three were caught
  by executing against the anchors, which is what the contract says to do.
- **No prose in this region was neither condition nor procedure.** The node brief
  asked for that to be said if it turned up. The closest call was
  §*When to retire research…*'s *"The sync-semantics grilling's 'Findings adopted'
  pattern"* — a reference to one past workstream, which the framing unit's own
  authoring note discourages — but it states a reusable bidirectional-pointer
  shape, so it is procedure with a historical example, not narrative residue.
