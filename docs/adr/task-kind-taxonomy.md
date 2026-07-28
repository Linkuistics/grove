# grove owns a closed, parameterised set of task kinds, gated on write and degraded on read

Every leaf task file declares a **kind** on its `**Kind:**` line, drawn from a
closed set. The set is closed — a new kind is a change to grove's code and docs,
not a free-text label a leaf may coin. A kind earns its place only by carrying
behaviour beyond a name: a distinct session discipline and its own model bucket
(*model-per-task-kind*).

The set is **parameterised, not flat**: five producers — `requirements`,
`design`, `planning`, `prototype`, `impl` — each with its own `review-` and
`integrate-review-` step, plus `research` and `combine-research`. Seventeen
kinds. The membership, each kind's discipline, and its HITL/AFK mark are
`docs/specs/task-kind-taxonomy.md`; this record owns only *why the set is closed
and parameterised*, and how the enforcement is asymmetric.

Only **`planning`** carries methodological force. It is the sole branch in the
loop's Execute step, and the only kind that grows the tree. Every other kind is a
producer of some artifact, differing in discipline, not in what the loop does
with it.

## Why the set is closed

The obvious alternative is a free-text label, and it is wrong for one concrete
reason: **grove reasons about the kind, not just reports it.** `leaf-decompose`
gives the new node's first child its parent leaf's kind, so a research leaf that
proves bigger becomes a research node. A label grove cannot enumerate is a label
grove cannot give defaults for, document a discipline for, or key a model bucket
on without the bucket namespace becoming unbounded.

The apparent tension with constraint 3 ("suggested shape, not enforced schema") is
not one: constraint 3 governs the **task file's body**, which stays freeform
markdown that nothing validates. The kind is a one-word declaration the CLI acts
on.

## Why the set is parameterised

Closed does not have to mean small, and the count is not the thing being
defended — the *earns-its-place* bar is. Parameterisation clears it because a
review step is genuinely five different reads: judging whether a decomposition is
made of vertical slices is not the same work as judging whether code is correct,
and neither is judging whether a requirements list describes the human's actual
need. The integrate steps clear it on a different axis — they share one triage
discipline but differ in **what the session is permitted to change**, which runs
from "edit code freely" to "edit what the human asked for", a change no session
can make unilaterally.

The cost is paid honestly: parameterisation grows the configuration surface to a
ceiling of 95 env vars, and it means a `review-*` policy must be written five
times unless routing can key on a *family*. That is why the family axis exists
(*model-per-task-kind*) — without it, parameterisation would not pay for itself.

**The set is closed but the patterns over it are not enforced.** grove does not
validate that a `review-X` leaf follows an `X` leaf, because a grammar is a
relation *between* leaves and grove expresses no such relation — the same
principle that keeps "the reviewer must not be the author" out of grove. The
patterns are documented conventions (`docs/specs/task-kind-taxonomy.md`).

**Nor are they units.** A chain is not something `pick` refuses to walk out of,
and not something whose close skips the Retire cascade's confirmation. Both would
need `pick` to modulate its walk, which *task-tree-scheme* forbids for a reason
that is not about chains at all. The three-leaf shape is not a workaround for a
missing construct: a leaf is the only unit the loop can be *at*, so cutting one
artifact into three leaves **is** how grove represents "one artifact, three
sessions" — the state is the tree, and there is nowhere smaller to put it.

## Gate on write, degrade on read

The enforcement is deliberately **asymmetric**, and the asymmetry is the decision:

- **Write gates.** `leaf-add` / `leaf-insert` / `leaf-decompose` reject an unknown
  `--kind` with an error listing the set. A human is present at authoring time, so
  catching `reserch` there is cheap and actionable.
- **Read degrades.** An unrecognised `**Kind:**` line — hand-edited, or written by
  a future grove version — emits a warning and is treated as `impl`. Reading never
  errors.

Read must degrade because the self-driving loop relaunches unattended: a task file
with a typo would otherwise jam the loop, and grove guides rather than gates
(constraint 5). Write may gate because gating there costs a human one retry, not a
stalled loop.

The same asymmetry decides the `work` → `impl` rename. `work` named both a member
of the set and the category containing it, so it was renamed — but a live grove's
task files must keep working. On **read**, `work` resolves silently to `impl`: it
is the previous spelling, not a typo, and warning on a correct file is noise. On
**write**, `--kind work` is refused with an error naming the replacement. No
version gate, no deprecation window; the read alias is one line and `.grove/`
trees are ephemeral, so it expires on its own.

## Considered options

- **A free-text kind label (rejected).** Needs no enum, extends without a code
  change, and reads as truer to constraint 3. But `leaf-decompose` then has nothing
  it can meaningfully inherit, a typo becomes a silent new kind rather than an
  error, and the disciplines documented per kind become an unbounded set grove
  cannot describe. The closed set is what buys the inheritance and the defaults.
- **A flat set — one `review`, one `integrate-review` (rejected).** Thirteen kinds
  instead of seventeen, a smaller config surface, and no need for a family axis at
  all. Rejected because the five reads are genuinely different disciplines, and
  because the asymmetry costs more to explain than the four extra kinds cost to
  carry: a set where producers are parameterised but their reviews are not leaves
  `leaf-decompose`'s kind inheritance ambiguous at exactly the step where a review
  proves oversized. The honest weak point is the integrate side, whose five
  members share one triage discipline; they are kept because their *permissions*
  differ, not their procedure.
- **Enforcing the review chain as a grammar (rejected).** See *Why the set is
  parameterised*. A non-blocking lint was costed as the middle option and rejected
  too: it would fire on a tree the human deliberately shaped, demand no action,
  and re-trigger on every `leaf-insert`.
- **Making a chain a first-class *group* — a unit `pick` will not leave, closing
  without confirmation (rejected).** Two of its three motivating costs do not
  exist. A chain at adjacent positions is already ordered by exactly the mechanism
  a node's children are, so `pick` does not wander out of one; and the Retire
  cascade's confirmation is asked **per node**, so a chain — deliberately not a
  node — already closes with none. That second cost is *created* by giving a chain
  a directory, not removed by it, which is a second and independent reason
  node-per-chain loses. The one real gap is that a sibling-level `leaf-insert` can
  split a chain where containment would not, and the machinery that would close it
  costs `pick`'s walk (*task-tree-scheme*) against a split repaired by one command.
  It would also **gate**: a chain `pick` will not leave is grove overruling the
  order a human set with the one verb that exists to preempt, so the request to
  remove a gate would have installed a larger one (constraint 5).
  `docs/specs/task-kind-taxonomy.md` costs both candidate marks and the tempting
  middle option (`leaf-add` inferring placement from a shared stem).
- **Upstream wayfinder's four types (rejected as a set).** `research` / `prototype`
  / `grilling` / `task` decompose *decision-reaching* work only — even wayfinder's
  `task` "earns its place by unblocking a decision, not by delivering the
  destination." They are a decomposition of grove's producer half, not a superset
  of grove's set: grove's `impl` has no analogue there at all. grove takes
  `research` and `prototype`, maps `grilling` onto `requirements`, and drops
  `task` — the tree already sequences prerequisites, so grove has no
  blocked-decision concept for it to earn its place against. The review and
  integrate steps are grove's own addition.
- **Keeping the `planning`/`work` binary (rejected).** Cheapest, but it forces
  genuinely different sessions — a citation-disciplined literature survey, a
  deliberately throwaway spike, a fresh-context adversarial read, and the triage
  that follows one — to share one label, one discipline, and one model bucket.

## Consequences

- Adding a kind is a deliberate code change (`leaf::Kind`,
  `content/TASK-FORMAT.md`, `--help`, README, and the spec) rather than a leaf
  coining a word. That friction is the point: a kind that cannot justify a
  discipline and a model bucket should not exist.
- **Grilling moves off `planning` and onto `requirements`.** `planning` keeps its
  methodological force but no longer opens with an interrogation, so the loop's
  Execute step and `content/SKILL.md` name `requirements` as the grilling kind.
- Existing trees keep working without edits: `**Kind:** work` reads as `impl`, and
  every other pre-existing label is unchanged.
- Each kind is additionally marked **HITL** or **AFK**
  (`docs/specs/task-kind-taxonomy.md`). This is documented guidance with no
  machinery behind it, and the mark *predicts* rather than permits: any kind may
  stop and ask a human, and a HITL leaf reached by an unattended relaunch simply
  waits, which is correct behaviour, not a fault to engineer around.
