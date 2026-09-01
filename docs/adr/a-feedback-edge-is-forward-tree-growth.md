# A feedback edge is forward tree growth

An editorial pipeline has feedback edges: a later stage meets a defect an earlier
stage's charter owns, and the work has to go back. A grove tree has no edges and
no cycles — `pick` is a depth-first pre-order walk over positions, it cannot skip
a live leaf and cannot re-enter a retired one.

**So a stage never goes back. It cuts a re-run leaf of the stage that owns the
work, and orders it ahead of the stage that runs next.** The edge points backward
in the pipeline and forward in the walk, and those are the same move.

Three rules complete it:

- **The chain is lazy.** A book leaf `leaf-decompose`s with `--kind draft`, and
  each stage's last act is `leaf-add`ing the next. Nothing is queued behind a
  running stage, so ordering a re-run before the next stage is just call order:
  cut the re-run leaf, then cut the next stage. `leaf-insert` is needed only where
  a later sibling entry already holds live work, which is the condition
  `references/decompose.md` already states for an integrate step.
- **A defect a *later* stage owns is handed forward through the book node's
  `BRIEF.md`**, under a running `## Handed forward` list naming the owning stage
  and the location. Every stage reads it, because the brief chain is root-to-leaf;
  a stage clears the entries it closes, because a brief is current-state context
  and not a log. An entry that survives `proof` is promoted at node close, to an
  ADR, a defect leaf, or the owner of the document's structure brief.
- **A second re-run of the same stage against the same book is an escalation, not
  a third leaf.** The session stops and says so.

## The trade-off

**The unit of correction is a session, not a patch.** A re-run leaf is a whole
fresh-context session of that kind reading the whole document again, which is
heavier than handing one stage a note. That is deliberate on two counts. The
stages are already whole-document reads — the pipeline has no smaller unit to
offer — and a "targeted patch" performed by the stage that *found* the defect is
the charter breach the measurement instrument exists to detect. A stage that fixes
an earlier stage's class has not saved a session; it has made the next
measurement unreadable.

**What it buys is that the tree stays the whole state.** No leaf gains a status
word, no task file gains a dependency field, and `pick` gains no knowledge of
pipelines. Constraint 1 holds — the shape is the state — and constraint 6 holds:
a reader with grove deleted sees the stage leaves in position order, a retired
`draft` first and a second live `draft` sitting between the art stage and the
proof stage, and reads the whole history off the filenames.
**The number of leaves of a kind under a document's node is the honest record of
how many times that stage ran**, which no edge annotation would give and which is
exactly the signal a later measurement needs.

**What it costs is that a re-run is indistinguishable from a first run at the
grammar level.** Both are a leaf of the same kind with the same slug under the
same node; only position, key and body separate them. Nothing validates that the
re-run leaf's body actually names the defect that caused it, and nothing detects a
stage that skips the re-run and quietly fixes the defect out of charter. Both are
discipline in a skill rather than a property of the tree, which is where this
methodology puts most things and is worth naming rather than glossing.

**The escalation bound is a judgement fixed in advance and not derived.** One
re-run per stage per document is the allowance; a stage that would be re-run a
second time is where an unattended pipeline stops being a pipeline and starts
being an oscillation, and the arm this was designed for has no human in it.
Stating the number here is what stops it being chosen once a session is already
looping.

## Considered options

- **Teach `pick` to revisit a retired leaf.** Rejected: it makes `pick` a
  scheduler. Every property that makes the walk cheap depends on its being a walk
  — a leaf's turn is its position, done-ness is a filename, and the finish trigger
  is the absence of a live leaf. A walk that can return needs a termination
  argument the tree does not carry, and `entries-are-never-removed` plus a
  revisiting walk is a loop with no bound. Reopen never.
- **Add a leaf state — `reopened`, `blocked`, `needs-rework`.** Rejected: it is
  the fourth state `references/retire.md` forbids, and for the reason stated
  there. A leaf that is not live is done or abandoned; anything else is expressed
  by ordering, which is what this record does. Reopen only if the three-state
  partition is itself reopened, which is a much larger question than a pipeline.
- **Cut all four stage leaves eagerly at decompose time**, as a research pair is
  cut. Rejected because the asymmetry runs the other way. A pair is eager so that
  the second survey cannot inherit the first's framing; an editorial stage
  *should* inherit — its whole input is what the previous stage left. Eager cutting
  also destroys the payoff `references/decompose.md` names: the cutting session is
  the one that knows what the next stage must be told, and it cannot write that
  into a body created before it ran. And an eager chain makes every re-run a
  `leaf-insert` against queued siblings rather than an append. Reopen if a stage is
  ever found needing to be launched under a template chosen before the previous
  stage ran.
- **Let the finding stage fix it, out of charter.** The cheapest option and the
  one a session under time pressure will reach for. Rejected because it silently
  deletes the evidence a later measurement reads: the preregistration's
  pre-emption rule exists precisely because one stage doing another's work makes
  the second stage's empty count unreadable. It also concentrates the whole
  pipeline in whichever stage is least disciplined. Reopen never — this is the
  failure the stage charters exist to prevent.
- **Record the edge as data — a `Blocked on:` or `Sends back to:` field in the
  task file.** Rejected because it is a second state beside the tree, and grove
  validates no cross-leaf grammar, so nothing would keep the field and the tree
  agreeing. The field would also have to be read by a session that has not been
  written yet, which is what a leaf already is. Reopen if grove ever grows a
  cross-leaf validator, which would make a declared edge checkable rather than
  decorative.
- **Hand a defect forward in the next leaf's body rather than in the node's
  brief.** Rejected as insufficient rather than wrong: a stage can only write the
  body of the leaf it cuts, so a defect owned by a stage two hops away has nowhere
  to go. The brief reaches every later stage by the chain that already exists, and
  the leaf body stays what it is good at — the specific case for the very next
  session.

## What would reopen this

- **A pipeline whose stages are not whole-document reads.** Every argument here
  rests on a stage having no unit smaller than the document; a pipeline with
  per-chapter stages has a cheaper correction than a session, and the trade-off
  changes.
- **A second escalation in practice** — that is, the bound being reached more than
  once across the campaign's books. That is evidence about the stage charters
  rather than about this mechanism, but it is the signal that would send someone
  back to this record first.
- **Grove growing a cross-leaf validator**, which would make a declared dependency
  checkable and reopen the data option above.

This record covers the mechanism; which stages exist and why is
[the editorial pipeline is four kinds](the-editorial-pipeline-is-four-kinds.md),
and why a kind is a skill rather than a variant is
[a kind is an open token](a-kind-is-an-open-token.md).
