# driving-conditions-k15

## Goal

Finish the universal rewrite: `content/driving.md`'s **8 universal triggering
units** and `content/SPEC-FORMAT.md`'s **1**, plus the placement of the remaining
format documents in `content/references/`. After this child, every universal
triggering unit in the corpus is a condition line.

## What this covers

**`driving.md`** (44 kB, 8 universal triggering + 18 procedural — the corpus's
most procedure-heavy file, and therefore the cheapest of the three rewrite
children). Its universal conditions: when to commission prior-art research, when
to retire research into ADRs, verifying framework decisions against the source,
verifying a repo-wide claim, recording fog without pre-slicing it, prune/reorder/
file-an-issue triage, no session summary, and never closing by inviting questions
in general.

**`SPEC-FORMAT.md`**'s one universal triggering unit (when a spec is written).

**The remaining format documents** — `ADR-FORMAT.md`, `BRIEF-FORMAT.md`,
`CONTEXT-FORMAT.md`, `SPEC-FORMAT.md`, `grilling.md` — carry **no** universal
triggering units at all (0, 0, 0, 1, 0). They are already procedure. So they move
into `content/references/` largely as-is; the work is placement and cross-linking,
not rewriting.

## The file-set budget closes here

This child is where the "under about eight files beside the ten per-kind ones"
bound is actually testable, because it is the last child to add one. Count the set
`loop-conditions-k13`, `task-format-conditions-k14` and this child have produced
and hold it against that bound. Any file over ~300 lines gets a table of contents
— the format documents and `driving.md`'s remainder are the ones most likely to
cross it.

## Attribution travels with the prose

`driving.md` carries adapted-from notices — `addyosmani/agent-skills` (MIT) and
`mattpocock/skills` (MIT), with license texts under `content/LICENSES/`. Those
notices are attached to specific sections and **must move with the sections they
attribute**, not stay behind in a file that no longer holds the text. Check
`content/LICENSES/` still resolves from wherever the attributed prose lands.

## Done when

- `driving.md`'s 8 and `SPEC-FORMAT.md`'s 1 universal units are condition lines
  with deferred remainders.
- The format documents sit at their final `content/references/` paths.
- Every universal `class=triggering` unit in the corpus is now a condition line —
  the composed-mandates golden is the check, and at this point every kind's mandate
  should be near the ~8 kB the spec's arithmetic predicts.
- Attribution notices resolve to the prose they attribute.
- The build gate passes and `cargo test` is green.

## Notes

`tests/reference_navigation.rs` walks the user-facing documentation surface and
requires each entry to actually contain a relative link — so it cannot pass by
finding nothing to do. Moving the format documents will move links; reconcile it
rather than relaxing it.

`tests/composition_guidance.rs` reads `content/driving.md` directly and is the
other file to watch.

## Running log — decisions this session made

### The format documents stay at `content/` root — the human's call, asked for

The goal line and a `Done when` limb asked for them under `content/references/`,
and `task-format-conditions-k14` had already decided the opposite one session
earlier, keeping `TASK-FORMAT.md` at root as "the fifth format document" rather
than splitting one species across two directories. Those cannot both be honoured,
and the disagreement was visible *before* any prose was written, because it
decides how every condition line cites its reference. So it went to the human as
a named trade-off with the evidence attached (`TASK-FORMAT.md` alone is cited in
six `src/`+`tests/` files and two under `docs/`), and the answer was **root**.

That also settles `driving.md` and `grilling.md`, which are not `*-FORMAT.md` but
are the same species — whole documents cited by bare name, read when you are doing
one particular thing. `driving.md` therefore keeps its path, its `order=9`, and
its `LICENSES/` relative links, and this child adds **no** file at all: the set
`loop-conditions-k13` and `task-format-conditions-k14` produced is
`bootstrap`, `commit`, `decompose`, `driver`, `execute`, `grove`, `retire` —
**seven**, and it closes there against the brief's "under about eight". The
brief's expectation that this child would spend the eighth was wrong only because
`driving.md`'s remainder already had a home.

### Nine units became nine conditions, but two of them grew

Seven compressed as expected; `driving-recording-fog` (769 → 794 bytes) and
`driving-when-a-leafs-place-is-in-doubt` (576 → 620) got **larger**. That is
deliberate, and it is `task-format-conditions-k14`'s rule applied: the deciding
distinction belongs in the condition, not the remainder. The fog unit's whole
point is the **fog-or-ticket test** — *can you state the question precisely right
now* — and a condition that says "record fog in a horizon note" without it gives a
session no way to tell a note from a leaf. The triage unit's point is the **three
sentences** — not now but still ours / not ours at all / decided against — and
each names a different verb, so a condition carrying only "never use a status
word" would state the prohibition and withhold the remedy. Both remainders keep
the elaboration.

Net across the slice: **7,734 → 5,525 bytes, 28% off** — below
`task-format-conditions-k14`'s 42%, and honestly so: ~500 of the 5,525 are the two
attribution comments, which are fixed cost and do not compress. Every kind's
mandate falls by 2,208 bytes; the corpus's triggering-unit union goes
**41,206 → 38,998**, picking up exactly where k14 left it.

### Two units left no remainder at all, and one heading moved instead

`driving-when-code-depends-on-a-framework-version` and
`driving-when-asserting-a-repo-wide-claim` were *entirely* condition — their
procedures already sat in the units they deferred to
(`driving-cite-framework-decisions-to-the-source`,
`driving-turning-a-sweep-into-evidence`). So neither got a new remainder unit;
each donated its section heading to the procedure below it, and `driving.md` lost
a unit rather than gaining one. `driving-no-session-summary` and
`driving-ask-about-the-trade-off` are the same case with nothing at all left
behind — their sections are simply gone from `driving.md`, and the first now
defers to `driving-record-decisions-inline`, which is where the running-log
mechanics it names already lived.

Only two new procedural ids exist: `driving-signs-of-a-research-leaf` (the signs
list and the placement paragraph) and `driving-the-findings-adopted-bridge` (the
two-way evidence-chain pattern). The pinned unit set moved by exactly those two,
nothing removed.

### The build gate refused a table-of-contents unit, and was right to

`driving.md` is 632 lines, twice the brief's ~300-line threshold, so it gained a
title and a section table. Written as its own `driving-contents` unit, the gate
rejected it: *no chain of `defers=` from any triggering unit reaches
`driving-contents`*. A ToC is navigation, and navigation has no referrer — so the
corpus structurally cannot hold one as a unit. The corpus's own convention already
answered it (`TASK-FORMAT.md`'s head unit carries that file's title), and the ToC
folded into `driving-signs-of-a-research-leaf` the same way.

The table carries **no `#anchor` links**. It had them, and two were ambiguous:
GitHub renders ` — ` in a heading as a *double* hyphen while most other slugifiers
collapse it, so "The review chain — when doubt earns its own leaves" has two
defensible anchors and no way to satisfy both. The reader here greps a file rather
than clicking it, so the links were removed rather than guessed at.

### The golden diff was checked as a claim, across all nineteen kinds

Not read. Regenerated, then verified programmatically against the recorded copy:
the **kind set is unchanged**, each kind's **unit set is identical**, and with the
nine moved ids removed from both sides **every remaining unit is in the same
order**. The nine now sit as one contiguous block (positions 43–50 in `impl`) plus
`spec-when-a-spec-is-written` at 39, which is where it was placed on purpose —
inside `## Artifacts`, beside `skill-adrs-and-specs`, which states the same
minimum-coherent-set rule one grain coarser. That is the whole diff: a relocation
from file positions 7 and 9 into file position 2.

`the_embedded_unit_set_is_pinned_complete` and its `[&str; 163]` length were
updated for the two new procedural ids. No other assertion in the suite moved —
notably `tests/composition_guidance.rs` and `tests/session_kind_guidance.rs`, both
of which read `content/driving.md` directly and pass unchanged, because the file
kept its path and every claim they pin is procedural prose that stayed in it.

### The size symptom `skill-opening-k16` predicted has arrived — measured, not fixed

`SKILL.md` is now **383 lines / 24.5 KiB**, and each kind's composed mandate is
**26.5–29.7 KiB** (of which 23.5 KiB is `SKILL.md`; the rest is that kind's
narrowed reference units, `MANDATE.md` and `SIGNAL.md`). The spec's arithmetic
predicted **~200 lines and roughly 8 KiB**. This child's `Done when` asked for
"near the ~8 kB the spec's arithmetic predicts", and that limb **does not hold** —
the corpus has landed at roughly three times it.

The cause is visible in the arithmetic itself: 8 KiB over ~51 conditions is ~160
bytes each, a *single sentence*. All three rewrite children independently produced
*paragraph* conditions averaging ~480–610 bytes, each judged well-compressed
against its own source. So either the estimate assumed a density the material does
not support, or the conditions are still carrying prose that belongs in
`references/`.

**That question is not this child's, and the tree already holds the leaf for it.**
`skill-opening-k16`'s Notes name this exact symptom in advance — *"If the file
lands materially above that, the condition lines are still carrying prose that
belongs in `references/` — which is the no-procedure finding, arriving as a size
symptom"* — and its `Done when` cuts a `review-impl` leaf for the no-procedure
obligation, which the spec forbids discharging by any budget test. Deciding
between "compress k13's and k14's conditions" and "the spec's arithmetic was
wrong" means re-reading forty-five conditions this child did not write; that is a
second session's work, not an absorption into this one. So it is measured here and
handed forward, with the numbers above so k16 need not re-derive them.

The budgets that *are* enforceable still hold: `SKILL.md` is 383 lines against the
500-line house ceiling, and its loop section is inside the alarm of 100 (both
asserted in `tests/methodology.rs`, both green).

### No review leaf, and no in-session reviewer

Same reasoning `loop-conditions-k13` and `task-format-conditions-k14` recorded:
the no-procedure limb is a review obligation the spec refuses to let a mechanical
check stand in for, and the node brief assigns it to `skill-opening-k16` after the
whole corpus has moved. The golden verification above is mechanical checking, not
doubt, so this child's one-reviewer allowance went unspent.
