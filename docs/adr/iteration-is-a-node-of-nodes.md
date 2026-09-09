# Iteration is a node of nodes, and carries no mark

A grove represents a repeated pass over one subject with the shape it already
has: a **series node** whose children are **pass nodes**, each pass node holding
that pass's step leaves. Both levels are the one node species — written by
`leaf-decompose`, carrying the decomposed leaf's own body as `BRIEF.md`, never
marked done. **No token in any name says "this is a loop."**

The claim is not that a pass resembles a node. It is that a pass **is** one: a
pass is a unit of work that turned out to need more than one session, which is
the entire definition of a node (`BRIEF-FORMAT.md`, *There is one node
species*). So the first pass and the fourth are created the same way as every
other node in every grove — a leaf is cut, the session that picks it finds
it bigger than one session, and it decomposes.

Two properties the shape was chosen to preserve fall out of changing nothing:

- **`pick` needs no teaching.** It is a depth-first pre-order walk that descends
  a node in place, so pass nodes in position order are visited in pass order and
  a pass's steps in step order. Iteration order is the walk that already exists.
- **`find .grove` is the whole reader.** Which passes ran, which step each
  stopped at, and how many passes a series took are all visible from names, with
  grove uninstalled — constraint 6 by construction rather than by feature.

## The trade-off it settles

The alternative, and the shape this was designed against, is a **marker token**:
a loop is a directory whose name carries a distinguishing infix. It is the
obvious design and it is rejected, because the mark costs six things and buys no
reader.

- **The slot is taken.** A leaf name is
  `NN-[DONE-|ABANDONED-]<kind>--<slug>-k<key>.md`, so the position is
  immediately followed by the **outcome** infix. A marker there is
  outcome-shaped, on the one species that deliberately has no outcome — a node's
  done-ness is the absence of a live leaf beneath it, which is why
  `Parts::Node` has no outcome field rather than a field constrained to one
  value.
- **It joins the reserved set, and that is a cutover.** `refuse_token` reserves
  exactly `BRIEF`, `DONE` and `ABANDONED`, for both of a leaf name's words. A
  fourth marker refuses every existing name spelling it, and
  [`task-names-are-canonical`](task-names-are-canonical.md) records what that
  costs: not a migration but a cutover, a `mv` per entry, paid once per tree.
- **It cannot go anywhere else.** The handle `<slug>-k<key>` is a **contiguous
  terminal substring** of every name that has one — the property
  `grammar-separator-k15` bought and `TaskName`'s single renderer enforces. A
  marker after the slug breaks it; before the slug is the outcome slot again.
- **It puts a field back that was removed on purpose.** `Parts::Node { slug }`
  carries the grammar's asymmetry as an *absence* of fields. A marker makes it
  `Option<Marker>`, which is the "fields nothing may fill" shape the type was
  written to avoid.
- **Hiding it in the slug is already forbidden.** `CONTEXT.md`'s node entry
  carries `_Avoid_: discriminating anything by a -chain / -pair token in a slug`.
  A `-loop` suffix is that rule's own counterexample under a new name.
- **Decisive: nothing would read it.** There is no render verb and no terminal
  surface — both are deliberately out of scope, and on the root brief's horizon.
  `pick` needs nothing. No verb creates a series. A discriminator with no
  consumer is a status field in a filename, which constraint 1 forbids as
  squarely as a phase file does.

What the marker would have bought — *this directory is a loop, sayable without
opening a file* — is bought instead by the series node's `BRIEF.md`, which has to
exist anyway, is where process context belongs, and can say the part a token
never could: the step sequence, the exit condition and the cap.

## Considered options

- **A marker infix on the node name** — `NN-LOOP-<slug>-k<key>/`. Rejected for
  the six reasons above. Reopen when a **reader** exists: a render verb, or a
  surface that must group passes without opening briefs. That is a genuine
  change of premise, not a preference — the sixth objection is the load-bearing
  one, and it dissolves the moment something parses the mark.
- **A marker in the slug** — `NN-<slug>-loop-k<key>/`. Rejected as the same
  decision with the grammar cost hidden: it needs no parser change precisely
  because nothing validates it, so the tree gains an unchecked convention that
  disagrees with itself the first time someone names a subject `event-loop`.
- **A second distinguished child** beside `BRIEF.md` — a reserved
  `SERIES.md` declaring the loop. Rejected: `BRIEF.md` already reaches every
  session in the subtree by the root→leaf chain, so a second file is a second
  place to look for one node's context, and the first thing a reader would have
  to learn is which of the two wins. It also adds a reserved name to the grammar
  for content the existing reserved name already carries.
- **A leaf per declared step at the series level**, which the passes
  instantiate. Rejected because `.grove/` holds task files and nothing else: a
  step leaf that names a step without being one is a task no session runs, and
  `pick` would return it. The declaration is prose in the brief and the steps are
  the passes' own leaves.
- **A `pass-N` slug on each pass node**, so the pass number is in the name.
  Rejected by `TASK-FORMAT.md`'s own test — a convention that *adds* what nothing
  parses is legible, one that *duplicates* a parsed field can disagree with it.
  Position is the parsed field that means order, and `pass-2` sitting at position
  03 after any insert is exactly that disagreement. The pass node takes the
  series' bare stem, as an editorial stage leaf takes the document's.
- **A leaf state meaning *iterate*** — a fourth outcome beside `DONE` and
  `ABANDONED`. Rejected under the partition `references/retire.md` already
  fixes, and for the reason
  [`a-feedback-edge-is-forward-tree-growth`](a-feedback-edge-is-forward-tree-growth.md)
  gives one level down: *another pass is needed* is expressed by the next pass
  node existing. Reopen only if the three-state partition is itself reopened.

## Why this is hard to reverse

Not because a marker cannot be added later — it can, and the grammar cost is the
one `task-names-are-canonical` already prices. It is hard to reverse because of
what a marker would then mean about the series that predate it. Every pass series
created under this record is **unmarked and correct**, so a later reader that
tests the mark under-reports them in silence: it does not refuse them, it reads
them as ordinary nodes and reports fewer loops than the tree holds. That is the
same failure class the `Malformed`/`Foreign` split exists to prevent — a whole
subtree disappearing from a reader because a name did not carry something — and
it is silent in exactly the way a refusal is not. Adding the mark therefore means
marking every live series in every live grove at the same moment, which is a
cutover across trees nobody enumerates, not an additive feature.

## What would reopen this

- **A consumer for the mark.** A render verb or a terminal surface that must find
  series without opening briefs. Objection six is the only one that dissolves,
  and it is the one the decision rests on.
- **A series whose passes are not whole units of work** — passes small enough
  that a pass node is heavier than the pass. Then a pass is not a leaf that
  proved bigger, and the central claim of this record is false of it.

The mechanism a stage uses to send work back inside one pass is
[a feedback edge is forward tree growth](a-feedback-edge-is-forward-tree-growth.md);
how a series declares its steps, its exit condition and its cap, and what that cap
does and does not bound, is [`docs/specs/pass-series.md`](../specs/pass-series.md).
