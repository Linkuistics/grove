## Two triggers, two verbs

When work surfaces mid-session, default to **externalizing it as a new leaf**
rather than absorbing it into the current session — grove's value is many small,
low-context sessions, and that value is lost the moment a session quietly grows
to cover work that should have been its own leaf.

- **A new concern** — the human raises it, or a tangent appears that does not
  serve *this leaf's stated goal* — goes to the tree with `grove-llm leaf-add`
  (or `leaf-insert` when it must sequence ahead of live leaves), **never** inline.
- **The current item proves bigger** than its brief assumed — turn the leaf into
  a node with `grove-llm leaf-decompose`, doing **only the first child** this
  session. The remaining children are leaves a later session picks up fresh.

Continue inline **only** while the work still serves this leaf's stated goal
*and* fits one focused, low-context session — the bar is *"fits this session,"
not "I can finish it."* You almost always *can* finish it; that is the trap that
grows a runaway session. Decomposition stays lazy (constraint 4): grow the tree
just-in-time, at the genuine seam, never speculatively. Externalizing is cheap,
so spend it freely — a permanent key never moves, a renumber rewrites zero file
contents, and `leaf-insert` exists precisely so a late-surfacing concern can slot
ahead of queued work without disturbing it.

Every grow verb is a working-tree change only; the enclosing task's commit folds
it in. What each verb *does* — what it moves, retitles, creates, gates and
prints — is the CLI's to state, so read `--help` rather than a transcription that
ages against the binary. What a cut leaf's name and body must look like, and what
a new node's charter must carry, are `TASK-FORMAT.md`'s and `BRIEF-FORMAT.md`'s.

## What a good child leaf looks like

A good child leaf is a **vertical slice**: it cuts a narrow but complete path
through everything its goal touches, rather than a horizontal layer that sits
dead until its siblings land. The test is independence — can this leaf's work be
demoed or verified on its own, without waiting on a sibling? A leaf that fails
that test needs its lines redrawn even if it still fits one session. This is a
second axis alongside *fits this session*, not a replacement for it.

<!-- adapted (paraphrased into grove's voice, not bundled verbatim) from
     mattpocock/skills@d574778f94cf620fcc8ce741584093bc650a61d3
     (skills/engineering/to-tickets/SKILL.md, the wide-refactor
     expand-contract exception) — MIT licensed; see
     LICENSES/mattpocock-skills.LICENSE. -->

**Wide refactors are the exception.** A mechanical change whose blast radius fans
across the whole codebase — rename a shared column, retype a symbol every caller
touches — breaks too many call sites at once for any vertical slice to land
green. Sequence it **expand → migrate → contract** instead, one leaf per stage,
added in order: a leaf that adds the new form beside the old so nothing breaks;
then a leaf per migration batch, sized by blast radius, each keeping CI green
because the old form still exists alongside the new; finally a leaf that deletes
the old form once no caller remains.

<!-- adapted (paraphrased into grove's voice, not bundled verbatim) from
     mattpocock/skills@d574778f94cf620fcc8ce741584093bc650a61d3
     (skills/engineering/wayfinder/SKILL.md, "Fog of war" / "Not yet
     specified") — MIT licensed; see LICENSES/mattpocock-skills.LICENSE. -->

## Work you can see coming but cannot yet shape

Which of the two a dim thought earns is the **fog-or-ticket test**: can you state
the question precisely *right now* — not whether you can answer it. A question
you can already phrase precisely earns a leaf immediately, even if it is blocked
and unanswerable today. One still too dim to phrase stays an **On the horizon**
note in the nearest brief (`BRIEF-FORMAT.md`) until a later session sharpens it
enough to graduate. There is no third option: laziness means a leaf either exists
or it does not, so a speculative leaf is the wrong shape for fog and the dim view
is otherwise simply lost.

## Commissioning prior-art research

A leaf whose design depends on lessons prior tools learned the hard way — and
that the current codebase cannot show — is under-served by grilling alone, which
would rediscover those failure modes from scratch. That is a **research leaf of
its own, cut ahead of the leaf that needs it**, never a tangent inside it. The
signs: the leaf sits in an architectural neighbourhood with well-known prior art;
several downstream `requirements` / `design` leaves share one evidence base; or
the grilling surfaces *"has anyone tried this before, and what happened to
them?"* — that question is itself the signal. Place it *after* the design
questions are visible and *before* the leaves that need the answers.

**The research leaf's brief names the downstream questions**, leaf by leaf, so
the researcher does not have to guess what is load-bearing: name the audience by
subject rather than by position, list the concrete questions each downstream leaf
needs answered, and ask for the output to be structured around them. **Bias the
search** while you are there — *"what's already been tried"* produces broad,
shallow surveys, while *"what went wrong after years of real use"* produces
post-mortems, and naming the obvious candidate as the fallback to beat keeps the
survey off it.

## Choosing a composition shape

When more than one leaf serves *one* artifact, two shapes are the habitual
answer. Both are **flat siblings** — neither gets a node directory, because a
node means *this work proved bigger than one session* and a composed shape is
neither.

**A review chain — `X` → `review-X` → `integrate-review-X` — is earned by a
load-bearing artifact**: a landed spec, a decomposition others will build on for
months, a subsystem. That is an orchestration boundary, and **artifact size and
vendor preference are not the test**. Reactively, a picked plain producer that
reaches its second-review boundary has earned one too (`references/execute.md`).

**A vendor pair — `research-a` → `research-b` → `combine-research` — is earned by
a question load-bearing enough to pay for two corpora.** One survey is one
vendor's corpus and one vendor's blind spots; one survey is also the default, and
two is a cost you argue yourself into.

## The two shapes are built in opposite ways

**A chain is lazy**, and each step is an ordinary `leaf-add` cut as the **last
act** of the session before it:

    grove-llm leaf-add <parent> <stem> --kind review-<producer>
    grove-llm leaf-add <parent> <stem> --kind integrate-review-<producer>

A producer cuts the first only once its artifact exists and it judges an
adversarial read necessary; a review cuts the second **only if it has findings
worth acting on**. A review that finds nothing creates nothing and simply
retires — that empty session is what the laziness exists to remove, and deciding
against review is a normal outcome at either step.

**A pair is eager**: it lands in one call or not at all.

    grove-llm leaf-add-pair <parent> <stem>

Cutting `research-b` lazily at the end of `research-a`'s session would let `b`
inherit `a`'s framing and corpus, destroying the independence the pair is run
for.

**The creating session writes the new leaf's body, and that is the payoff.**
Because the leaf is cut by the session that knows *why* it is needed, it can
carry the specific case its producer could not cover, or the findings verbatim —
strictly more than a generic goal sentence a constructor could render from a stem
and a kind. The template you get is bare on purpose: a handle and empty sections,
nothing to edit around.

**Name a chain step's kind off the producer that actually ran** —
`review-<producer>`, then `integrate-review-<producer>`. Nothing derives it, so
this is the one place to be careful: `--kind review-impl` beside a `design`
producer is a perfectly valid invocation that buys a reviewer reading for
correctness, security and tests where it should be asking whether the ADRs are a
minimum coherent set, and nothing downstream detects the mismatch.

**Give every step of a composed shape the producer's bare stem as its whole
slug** — no `-review` or `-a` suffix, and no leading kind word. The kind field
beside it already says which step you are looking at, and the slug's job is to
name the artifact. One consequence to expect: a bare stem stops naming one leaf,
so name a specific step by its `<slug>-k<key>` handle or its key — in a reference
you write, and as a `leaf-insert` target. What each verb does with an ambiguous
stem is the CLI's to state; read `--help`.

**Diversity is the configuration's, not the tree's.** Whether a `review-*` or
`research-b` template reaches a different harness or model from its producer's is
a property of two entries in `~/.config/grove/config.kdl`. Grove cannot recover a
target from an opaque command string, so it records nothing about how the
producer ran, compares nothing, and warns about nothing. If the axis matters,
read the two entries before you pay for the second leaf.

## Which hop a gap costs

Steps are appended at the parent's next free position, so a step cut after some
unrelated leaf already exists lands *after* that leaf, and a later `leaf-insert`
can split a chain that was contiguous. Grove refuses none of it — it validates no
cross-leaf grammar, `pick` is a walk and not a scheduler, and contiguity was
never an enforced unit. What decides whether a gap costs anything is **what the
next step consumes**:

- **A `review-*` step re-derives, so `leaf-add` is right wherever it lands.** Its
  handoff is the producer's **stable handle**: the review's own body names it
  under `**Reviews:**`, every task commit message names the work item by that
  handle, so the reviewer finds the producer's commit from the handle and reads
  that diff against the current source. Nothing it consumes had been written down
  for intervening work to stale. That is narrower than *free* — work that rewrote
  the reviewed artifact, its requirements, or its recorded evidence leaves the
  reviewer reconciling a historical diff with a tree that has moved — but the
  reconciliation is visible, and the reviewer performs it deliberately.
- **An `integrate-review-*` step consumes, so a gap corrupts.** A review's
  findings are anchored to a commit and to `path:line` coordinates, and the
  integrating session opens the working tree as it *then* stands. Any intervening
  edit to a cited file moves those lines, and the drift is **silent**: nothing
  errors, the finding simply points somewhere slightly wrong, and the integrating
  session has to re-derive what the reviewer meant from a codebase the reviewer
  never saw.

So an integration is cut **where `pick` reaches it next**, and the condition is
mechanical and **directory-local**. Read the review's *own parent* directory at
the entries after the review's position, and find **the first sibling entry after
the review whose subtree still holds live work** — leaf file or node directory,
whichever comes first. That entry is the `leaf-insert` target.

Three things that condition gets right where "the first live leaf after it" does
not:

- **Terminal entries never block.** A later `DONE` or `ABANDONED` leaf, and **a
  node whose subtree is wholly terminal**, are stepped straight over; so is the
  driver's `finish` sentinel, which is skipped while any ordinary work is live.
- **A later sibling *node* blocks as one entry, and the node is the target.**
  `pick` descends a node in place, so a live leaf anywhere beneath a later
  sibling node runs before anything appended after that node. Insert at the
  **sibling directory**, never the live leaf inside it — targeting the descendant
  inserts at the wrong level, dropping the integration inside a node whose brief
  does not charter it.
- **Nothing outside the directory can intervene.** Pre-order finishes the
  review's own directory — including an integration just appended to its end —
  before it visits any later sibling of an *ancestor*. So live work in an outer
  sibling node is irrelevant, and when no sibling entry blocks, plain `leaf-add`
  is exactly right.

## Why there is no exception to check

Adjacency is unconditional guidance, not because departing is forbidden — grove
enforces none of this — but because the check an exception would need cannot be
performed: at the moment a review cuts its integration the intervening leaf has
not run, and grove makes no leaf's eventual file set part of its contract, so a
goal or pointer list is not proof of what it will touch. A session that departs
anyway owns the drift.
