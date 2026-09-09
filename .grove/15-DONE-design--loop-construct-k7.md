# loop-construct-k7

## Goal

Design how an iterative loop is represented in a grove — in the filesystem
view first, because that is the minimal UI — and settle what it costs in the
name grammar, the lifecycle and the recorded decisions it reopens.

## Context

Commissioned by `plan-k1`, decision 13. Cut deliberately *after* the pilot has
run editorial cycles by hand, so the construct is designed against a loop that
actually ran rather than one imagined in advance.

Today grove has no loop and no rendering surface of any kind: `.grove/` read
with `find` is the entire UI, and a review chain's iteration is emergent — a
session cuts its own follow-on with `leaf-insert`.

## Done when

The filesystem representation is settled and recorded, with the reopened
decisions either amended or explicitly kept; whether any code changes, and
what, follows from that and is not assumed here.

## Notes

**The human's proposed shape, to design against rather than from scratch.** A
loop is a **directory carrying a marker token**, holding the steps of the loop;
inside it, **numbered subdirectories, one per pass**, holding that pass's
artifacts. Two properties are already good and worth preserving through any
revision:

- `pick` is a pre-order walk that descends a node in place, so numbered pass
  directories are visited in pass order with **no change to selection at all**.
  The iteration order falls out of the walk that already exists.
- `find .grove` shows which passes ran and where each stopped, with grove
  uninstalled. That is constraint 6 satisfied by construction, and it is the
  test the representation exists to pass.

**What the design must resolve.**

- **A node is never marked, and there is one node species.** Every node today is
  a leaf that proved bigger, carries a `BRIEF.md`, and is created by
  `leaf-decompose`. A marker token creates a second species and puts an
  outcome-shaped infix where the grammar has only ever had one on leaves. Either
  the marker is not an infix, or the one-species rule changes and says so.
- **What "the steps" are at the loop level.** A declaration the passes
  instantiate, or leaves in their own right? `.grove/` holds task files and
  never the work product, so "the artifacts of each pass" must mean that pass's
  task files unless something else is intended.
- **Keys are permanent and never reused**, so pass two's `edit` is a different
  leaf from pass one's. Confirm the key allocator behaves across a growing
  loop.
- **The exit rule and the cap.** `docs/review-yield.md` measured nine review
  chains and honestly reported that it could not read a yield curve — so
  diminishing findings per pass is *not* an available stopping rule on this
  repository's own evidence. Whatever the exit rule is, it is not that.
- **The reopened records.** *One task is one session*, *entries are never
  removed*, and the outcome-partition decisions. A loop that re-runs work
  touches all three, and they are reopened deliberately, not incidentally.
- **The name grammar is a seam, not a detail.** The filename grammar is
  `grove-loop`'s `task_name` module over the tree library's entry name, and the
  `--` separator is what makes a name have exactly one reading with no set
  consulted. A marker token must not reintroduce ambiguity that separator
  removed.

**Not now:** a render verb, a terminal surface, or the DevTUI direction. The
filesystem shape is settled first, and the rest is on the root brief's horizon.

## Decisions (running log)

**The evidence this was designed against, measured first.** Three readings, each
with a control watched to fail before it was credited.

- **No iteration has ever been represented in this tree.** Extracting
  `(parent directory, kind, slug)` from all 219 keyed entries and grouping,
  **no triple occurs twice** anywhere under `.grove/`. The extractor was
  controlled two ways: 0 of the tree's names fail its pattern, and a scratch
  directory holding `01-DONE-copy-edit--book-k1.md` beside
  `02-DONE-copy-edit--book-k2.md` makes it report the duplicate. So the clean
  read is a fact about the tree, not about a broken instrument.
- **The one pipeline that could iterate, did not.** Four books ran the editorial
  chain; each has exactly one `copy-edit`, one `art` and one `proof`. The
  feedback edge `docs/adr/a-feedback-edge-is-forward-tree-growth.md` designs, and
  the one-re-run-per-stage cap it fixes in advance, **never fired in four
  documents**.
- **The corrections went somewhere else, and they did not stop.** Of the 150
  leaves anywhere under `crate-books-k14`, **36 were allocated after the last
  editorial stage leaf in the whole tree** (`proof--grove-loop-k182`), 29 of them
  as flat siblings at the node's own level. Partition controlled: 36 + 114 = 150,
  and the same partition at thresholds 0 and 219 returns 150 and 0.

**Decision 1 — a pass is a leaf that proved bigger, so the construct needs no new
species.** A series node's children are pass nodes; a pass node's children are
that pass's step leaves. Both are ordinary nodes: written by `leaf-decompose`,
carrying the decomposed leaf's own body as `BRIEF.md`, never marked. This is not
an analogy — a pass genuinely *is* one unit of work that turned out to need more
than one session, which is the whole definition of a node. So the one-species
rule is untouched and does not have to change to say anything.

**Decision 2 — no marker token, and the grammar does not move.** Six costs, none
of which buys a reader. (i) The slot right after the position is the *outcome*
slot, so a node marker there is outcome-shaped on the one species that has no
outcome. (ii) `refuse_token` reserves exactly `BRIEF`, `DONE` and `ABANDONED`; a
new marker joins that set and every existing slug spelling it becomes refused —
`task-names-are-canonical`'s cutover cost, paid again. (iii) The handle
`<slug>-k<key>` is a contiguous terminal substring of every name, so a marker can
only go before the slug, which is (i). (iv) `Parts::Node { slug }` would gain a
field, reintroducing the "fields nothing may fill" asymmetry the type was shaped
to remove. (v) A marker carried in the *slug* instead is the `-chain` / `-pair`
discriminator `CONTEXT.md` already forbids. (vi) Decisive: **nothing would read
it.** A render verb is out of scope by this leaf's own charter, `pick` needs
nothing, and no verb creates a series. A discriminator with no consumer is state,
which constraint 1 forbids.

**Decision 3 — the steps are a declaration in the series brief; a pass's steps
are ordinary leaves cut lazily.** `.grove/` holds task files and never the work
product, so "the steps of the loop" cannot be leaves at the series level — a step
leaf there would be a task nobody runs. The sequence is prose in `BRIEF.md`;
each pass's actual steps are cut one at a time by the session before them,
because the cutting session is the only one that knows what the next must be
told. This is `a-feedback-edge-is-forward-tree-growth`'s rejection of eager
cutting, unchanged and reused.

**Decision 4 — a pass node's slug is the series' bare stem, and the pass number
is its position.** Not `pass-2`. `TASK-FORMAT.md`'s own test decides it: a
convention that *adds* what nothing parses is legible, one that *duplicates* a
parsed field can disagree with it — and a `pass-2` slug sitting at position 03
is exactly that disagreement. This also matches the editorial rule that every
stage leaf carries the document's bare stem as its whole slug.

**Decision 5 — the exit rule is a checkable condition plus a cap fixed before
pass 1, and the cap escalates rather than exits.** Yield is not available and the
evidence is this repository's own: `docs/review-yield.md` could not separate flat
from falling across nine chains, and — the sharper half — **no review came back
empty**, so an exit-on-empty rule would have fired zero times in nine
opportunities. What is left is a condition a pass can evaluate and a number
written down before anyone is inside the loop, which is
`a-feedback-edge-is-forward-tree-growth`'s own reasoning for stating its bound in
advance.

**Decision 6 — the cap bounds the series, not the work, and the spec says so.**
The 36 leaves above are what an exit actually buys: when the book nodes closed,
correction traffic did not stop, it relocated to flat siblings at the parent with
no cap and no charter over it. A design claiming to bound the work would be
false on this tree's own record.

**Decision 7 — all three reopened records are kept, none amended.**
*One task is one session* holds unchanged: a pass's step is one leaf and one
session, and what a series repeats is *kinds*, not leaves.
`entries-are-never-removed` holds unchanged, and this tree is its live witness —
219 entries, max key 219, so keys 1..219 are each used exactly once and nothing
was ever deleted. The outcome partition holds unchanged: *this pass ran and
another is needed* is expressed by the next pass node existing, never by a fourth
state — which is `a-feedback-edge-is-forward-tree-growth`'s answer applied one
level up, and reaching for a state word here would be the `blocked` / `deferred`
/ `superseded` taxonomy `references/retire.md` forbids.

**Decision 8 — "loop" is not the word.** `CONTEXT.md` uses *loop* 35 times and
heads two entries with it, and every one means the **driver** loop — the session
loop bare `grove` runs. The construct is named a **pass series** so the glossary
keeps one reading per term.

**Decision 9 — artifacts: one ADR and one spec, and no code changes.** The
marker rejection is hard to reverse (a series created before a marker exists is
unmarked, and any later reader testing the mark under-reports it in silence),
surprising without context, and has real rejected alternatives — so it earns a
record. The exit rule, the cap and the shape are *how the area works* and go to
the spec. Nothing in `grove-llm` moves: the construct is expressible today.
