# Iteration reuses the existing species, and carries no mark

A grove represents a repeated pass over one subject with the entries it already
has: a **series node** whose children are that series' **passes**, each pass
holding its own step leaves. A pass that needs more than one session is a node,
written by `leaf-decompose`, carrying the decomposed leaf's own body as
`BRIEF.md`, never marked done; **a pass that is one session is a leaf**, and the
series node's children are then leaves rather than directories. Either way the
species is one grove already writes. **No token in any name says "this is a
loop."**

The claim is not that a pass resembles a node. It is that a multi-session pass
**is** one: a pass is a unit of work that turned out to need more than one
session, which is the entire definition of a node (`BRIEF-FORMAT.md`, *There is
one node species*). So the first pass and the fourth are created the same way as
every other node in every grove — a leaf is cut, the session that picks it finds
it bigger than one session, and it decomposes. A pass that never outgrows its
one session never becomes a node, for the same reason no other one-session leaf
does; that is the rule holding, not an exemption from it.

Two properties the shape was chosen to preserve fall out of changing nothing:

- **`pick` needs no teaching.** It is a depth-first pre-order walk that descends
  a node in place, so passes in position order are visited in pass order and a
  pass's steps in step order. Iteration order is the walk that already exists.
- **`find .grove` shows how far a series has got.** Which passes ran, which step
  each stopped at, and how many passes a series took are all visible from names,
  with grove uninstalled — constraint 6 by construction rather than by feature.
  What `find` cannot show is a series' *sequence, exit condition and cap*: those
  are the declarations, they live in `BRIEF.md`, and no filename could hold them.
  `docs/specs/pass-series.md` states that division and does not overclaim it.

## The trade-off it settles

The alternative, and the shape this was designed against, is a **marker token**:
a loop is a directory whose name carries a distinguishing infix,
`NN-LOOP-<slug>-k<key>/`. It is the obvious design and it is rejected because
**the names already carry the mark**: the token is redundant, not unread. That
distinction is the whole of the argument and it is worth being exact about.
*Nothing would read it* is the tempting objection and it is not available here —
a human running `find` is a reader, and this record leans on that same reader one
section above. The case against the mark has to survive the reader existing.

**The discriminator that exists today.** Every pass and every step leaf under a
series carries the series' bare stem as its whole slug (`pass-series.md`). So a
series that has repeated is **sibling node directories sharing one slug** — and
nothing else in a grove produces that. An ordinary node's children name distinct
subjects and therefore carry distinct slugs; the two composed shapes that *do*
share a stem — a review chain and a vendor pair — are flat sibling **leaves**,
not directories (`references/decompose.md`). Measured across the three live
groves on disk when this record was written: **zero** sibling node directories
shared a slug anywhere in them, while the same test over leaf files fired
thirteen times in this grove alone. A reader with `find` distinguishes a series
from an ordinary node by a repetition already in the output.

**And the one case that is genuinely indistinguishable should be.** A series
whose second pass has not been cut yet looks exactly like an ordinary node,
because it is not yet a repetition — its series-ness is a claim about the
*future*, and the three facts that make that claim (the step sequence, the exit
condition, the cap) cannot be carried by a name at all. A token reading `LOOP`
would tell a reader to open `BRIEF.md`, which is where the reader was going
anyway. That is the whole of the buy, and it is why the mark does not pay.

**What it would actually cost**, and one thing it would not:

- **A third `Parts` variant, or an infix on the one species that has no infix.**
  `Parts` has two variants that are "the whole of the type", carrying the
  grammar's asymmetry as an *absence* of fields — a leaf has a kind and an
  outcome, a node has neither. A marked node is a second node species however it
  is spelled; the shape to weigh is that variant, not an `Option<Marker>` field.
  A marker in the position after the ordinal is also the **outcome** slot, on the
  one species whose done-ness is deliberately the absence of a live leaf beneath
  it rather than a field. So *put the marker somewhere else* has no answer: after
  the slug it breaks the contiguous terminal `<slug>-k<key>` handle, and before
  the slug is this same position.
- **Hiding it in the slug is separately forbidden.** `CONTEXT.md`'s node entry
  carries `_Avoid_: discriminating anything by a -chain / -pair token in a slug`,
  and a `-loop` suffix is that rule's own counterexample under a new name. This
  defeats the slug placement only; it is listed below as a rejected option in its
  own right, and it says nothing about the infix.
- **What it would *not* cost — and this is worth stating, because it looks like
  it should — is any rename.** `refuse_token` admits lowercase ASCII, digits and
  dashes for both of a name's words, so no existing name in any grove can spell
  an uppercase marker. Adding one to the reserved set alongside `BRIEF`, `DONE`
  and `ABANDONED` refuses nothing that exists, and is therefore **not** the
  cutover [`task-names-are-canonical`](task-names-are-canonical.md) prices. The
  grammar cost above is the whole of the price; anyone arguing this decision from
  a migration cost is arguing from one that does not exist.

What the marker would have bought beyond the repeated slug — *this directory is
a series, sayable without opening a file* — is bought instead by the series
node's `BRIEF.md`, which has to exist anyway, is where process context belongs,
and carries the part a token never could: the step sequence, the exit condition
and the cap.

## Considered options

- **A marker infix on the node name** — `NN-LOOP-<slug>-k<key>/`. Rejected as
  redundant with the repeated slug, at the price of a second node species.
  Reopen when a reader needs series-ness **before** the second pass exists, or
  when something must group passes without opening briefs — a render verb, a
  terminal surface. That is a genuine change of premise, not a preference.
- **A marker in the slug** — `NN-<slug>-loop-k<key>/`. Rejected as the same
  decision with the grammar cost hidden: it needs no parser change precisely
  because nothing validates it, so the tree gains an unchecked convention that
  disagrees with itself the first time someone names a subject `event-loop`.
- **A second distinguished child** beside `BRIEF.md` — a reserved `SERIES.md`
  declaring the loop. Rejected: `BRIEF.md` already reaches every session in the
  subtree by the root→leaf chain, so a second file is a second place to look for
  one node's context, and the first thing a reader would have to learn is which
  of the two wins. It also adds a reserved name to the grammar for content the
  existing reserved name already carries.
- **A leaf per declared step at the series level**, which the passes instantiate.
  Rejected because `.grove/` holds task files and nothing else: a step leaf that
  names a step without being one is a task no session runs, and `pick` would
  return it. The declaration is prose in the brief and the steps are the passes'
  own leaves.
- **A `pass-N` slug on each pass**, so the pass number is in the name. Rejected
  by `TASK-FORMAT.md`'s own test — a convention that *adds* what nothing parses
  is legible, one that *duplicates* a parsed field can disagree with it. Position
  is the parsed field that means order, and `pass-2` sitting at position 03 after
  any insert is exactly that disagreement. A pass takes the series' bare stem, as
  an editorial stage leaf takes the document's — and that shared stem is the
  discriminator the marker would have duplicated.
- **A leaf state meaning *iterate*** — a fourth outcome beside `DONE` and
  `ABANDONED`. Rejected under the partition `references/retire.md` already fixes,
  and for the reason
  [`a-feedback-edge-is-forward-tree-growth`](a-feedback-edge-is-forward-tree-growth.md)
  gives one level down: *another pass is needed* is expressed by the next pass
  existing. Reopen only if the three-state partition is itself reopened.

## What reversing this would cost, and why it is cheapest to decide now

Adding a marker later costs no renames — see above — but it does change what the
absence of a mark *means*. Every series created under this record is **unmarked
and correct**, so a later reader that tests the mark under-reports them in
silence: it does not refuse them, it reads them as ordinary nodes and reports
fewer series than the tree holds. That is the failure class the
`Malformed`/`Foreign` split exists to prevent — a subtree disappearing from a
reader because a name did not carry something — and it is silent in exactly the
way a refusal is not. So adding the mark later means marking every live series in
every live grove at the same moment.

**That cost is zero today, and this record is written while it is.** No pass
series exists in any grove on disk, so the population a later cutover would have
to reach is empty. Reader-facing identity is therefore being settled *before* the
first series rather than after a legacy set accumulates — which is the reason to
decide now, and the reason the reopen conditions below are worth reading rather
than filed. Someone who thinks the repeated slug is too weak a discriminator
should say so before the first series is cut, not after the tenth.

## What would reopen this

- **A reader that needs series-ness before the second pass.** The repeated slug
  identifies a series that has repeated; it says nothing about one that has not
  yet. A consumer that must find a series at pass one — a render verb, a terminal
  surface, a report that counts series rather than passes — reaches for the one
  thing the names do not carry, and that is the load-bearing objection.
- **Sibling node directories that share a slug without being a series.** The
  discriminator is unenforced and a false positive is constructible: a review
  chain's steps are flat sibling leaves sharing the producer's stem, so if two of
  them each decompose, the tree holds two sibling *nodes* with one slug and no
  series anywhere. That had happened nowhere in the three groves measured, but
  nothing refuses it — the reading above is a measurement, not an invariant. A
  grove where it becomes ordinary has lost the discriminator, and the marker
  question is open again on its original terms.
- **A series whose passes are not units of work at all** — so small that neither
  a node nor a leaf per pass is the right unit. A one-step pass is already a leaf
  and is accounted for above; this condition is about a pass smaller than one
  leaf, which would mean the pass is not the unit and the shape is wrong.

The mechanism a stage uses to send work back inside one pass is
[a feedback edge is forward tree growth](a-feedback-edge-is-forward-tree-growth.md);
how a series declares its steps, its exit condition and its cap, what that cap
does and does not bound, and where the discipline that carries all of it is
shipped, is [`docs/specs/pass-series.md`](../specs/pass-series.md).
