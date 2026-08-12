<!-- unit: task-leaf-filename kinds=* class=triggering defers=task-suggested-shape -->
<!-- grove reference file — the task-file shape -->

# TASK-FORMAT — the leaf task file

A **leaf** in a grove is a single `.md` task file, named
`NN-[DONE-|ABANDONED-]<session-kind>-<slug>-k<key>.md`: a 2-digit per-level
**position** `NN` (its place among its directory's children), one member of the
closed **session-kind** set below, a human **slug**, and a permanent
**key** `-k<key>` (stable identity, the terminal token, assigned once, never
reused) — e.g. `01-requirements-plan-k1.md`, `03-impl-extract-k7.md`. A leaf ends
one of two ways, marked in place right after the position: retired work carries a
`DONE` infix (`03-DONE-impl-extract-k7.md`); a path decided against carries an
`ABANDONED` infix (`03-ABANDONED-impl-extract-k7.md`, `leaf-prune`,
pruning) — pruning is
**HITL**, never an agent's own call. One task is one session (constraint: one
task per session). The file is freeform markdown — a guide follows, not a
schema.

<!-- unit: task-kind-in-the-filename kinds=* class=triggering defers=task-name-reading-is-strict -->
**The kind lives in the filename and nowhere else.** It is routing metadata, not
identity: the stable [work-item handle](#suggested-shape) stays `<slug>-k<key>`,
so `grove-llm resolve`, a commit message and an in-file header are all unaffected
by it. Putting it in the name is what lets `pick`, the driver's routing lookup and
your own eye read a session's discipline out of `find .grove` without opening a
file. A **node directory** carries no kind at all (`NN-<slug>-k<key>/`), even when
its slug happens to begin with a kind word.

<!-- unit: task-name-reading-is-strict class=procedural -->
Reading is strict in both directions. Every task-shaped leaf name — live, `DONE`
or `ABANDONED` — must carry a known kind; a missing or unknown one is malformed
and stops tree operations, naming the path and the valid set, rather than
degrading to `impl`. No kind label plus `-` prefixes another, so a name always
separates unambiguously and round-trips without touching the slug. Foreign
non-task files in the tree stay ignored.

<!-- unit: task-nineteen-kinds kinds=* class=triggering defers=task-work-is-not-a-kind -->
## The nineteen kinds

Every leaf's filename names its **kind**, drawn from a closed set
(task-kind-taxonomy). Adding a twentieth is a deliberate change to grove's
code, its configuration schema and its docs, not a free-text label a leaf may
coin — each kind is the key one command template is configured under, so a kind
grove cannot spell is a session it cannot launch. The set is
**parameterised, not flat**: five producers, each with its own `review-` and
`integrate-review-` step, plus a research pair and one driver-owned step.

| kind | review | integrate |
|---|---|---|
| `requirements` | `review-requirements` | `integrate-review-requirements` |
| `design` | `review-design` | `integrate-review-design` |
| `planning` | `review-planning` | `integrate-review-planning` |
| `prototype` | `review-prototype` | `integrate-review-prototype` |
| `impl` | `review-impl` | `integrate-review-impl` |
| `research-a` + `research-b` | — | `combine-research` |
| `finish` — driver-reserved | — | — |

Five producer rows of three, the research row's three, and one driver-owned step.
The research row holds **two** kinds rather than one kind run
twice: `research-a` and `research-b` share a discipline but are separate
configuration keys, which is what makes "two independent corpora" a fact in the
tree instead of a forecast about routing policy. `finish` is the driver's own
complete-finish-cycle sentinel: the grow verbs refuse to create one, retire,
prune and decompose refuse it as an operand, and `leaf-insert` may
target it only to put ordinary work *before* teardown.

<!-- unit: task-hitl-afk kinds=* class=triggering -->
Each kind is marked **HITL** (resolves through live exchange with a human who
speaks for themselves) or **AFK** (driven by the agent alone). Three are HITL —
`requirements`, `prototype` and `finish` — because each needs input the session
cannot supply for itself: the human's own words, their reaction, or their teardown
decision. The mark
**predicts, it does not permit**: *any* kind may stop and ask a human, and doing
so is always legitimate. A HITL leaf reached by an unattended relaunch of the
self-driving loop simply waits for a human, which is correct behaviour, not a
fault.

**Producers**

<!-- unit: task-producer-requirements kinds=requirements class=triggering -->
- **requirements** (HITL) — establish *what* should be built. This is where the
  grilling lives (`grilling.md`): interview one question at a time, propose a
  recommended answer for each, walk the design tree until shared understanding
  is reached. Sharpen `CONTEXT.md` inline as terms resolve.
<!-- unit: task-producer-design kinds=design class=triggering -->
- **design** (AFK) — given requirements, establish *how*. The deliverable is a
  spec, an ADR set, or both. A `design` session that finds itself cutting
  *implementation* leaves has drifted into planning's job and should externalize
  a `planning` leaf instead.
<!-- unit: task-producer-planning kinds=planning class=triggering -->
- **planning** (AFK) — given the design, first find the **smallest independently
  useful working increments** and order them by dependency. Create a separate
  grove for every obvious stage that leaves the product working and delivers
  useful, verifiable behavior for its successor; changes that cannot
  independently leave the product working stay in one increment even when their
  code edits are separable. Then cut the current increment into vertical slices
  and **grow the tree**: turn an oversized leaf into a node — a **directory**
  `NN-<slug>-k<key>/` holding ordered child leaves, headed by a `BRIEF.md`
  charter. A node is always a leaf that proved bigger, so it always carries one.
  The deliverable is *more tree*. The only kind with methodological force — the sole
  branch in the loop's Execute step.
<!-- unit: task-producer-prototype kinds=prototype class=triggering -->
- **prototype** (HITL) — a cheap, deliberately throwaway artifact built to react
  to, not to ship. The point is the reaction it provokes, not the code's
  survival.
<!-- unit: task-producer-impl kinds=impl class=triggering -->
- **impl** (AFK) — produces code, docs, or tests. The deliverable is an artifact
  that ships. (`driving.md` carries the habits: cite framework decisions to the
  source, doubt a hard-to-reverse decision before it stands, and externalize
  surfaced work into new leaves rather than absorbing it.)

<!-- unit: task-research-pair kinds="research-a research-b" class=triggering defers=task-research-write-paths -->
**Research** — a **vendor pair**, not a review chain: two independent surveys,
unioned.

- **research-a** and **research-b** (both AFK) — a citation-disciplined
  literature/prior-art survey. Breadth-seeking: a citation per
  failure-mode claim, primary sources, and an explicit note where a search found
  silence (the absence is itself a finding). No grilling, no tree growth. The two
  kinds are identical in discipline and distinct in configuration; a single survey
  that needs no pair is `research-a`.
<!-- unit: task-combine-research kinds=combine-research class=triggering defers=task-research-write-paths -->
- **combine-research** (AFK) — union two surveys' coverage and flag every
  disagreement. This kind, not either producer, carries the **adversarial** move:
  two vendors on overlapping corpora can agree on something false, so **agreement
  without independent primary sourcing is a red flag, not a confirmation**.

<!-- unit: task-research-write-paths class=procedural -->
All three write under `docs/research/`, and because a pair's three leaves share
one slug, **the kind supplies the discriminator** — the same principle that keeps
it out of the slug:

| kind | writes |
|---|---|
| `research-a` | `docs/research/<slug>-a.md` |
| `research-b` | `docs/research/<slug>-b.md` |
| `combine-research` | `docs/research/<slug>.md` — the union |

A solo `research-a` with no pair writes `-a.md` and that is the whole record; a
`-b` survey added later renames nothing, and the union lands at the unadorned
name where a reader looks first.

<!-- unit: task-finish-session kinds=finish class=triggering -->
**finish** (HITL, driver-reserved) — the whole-grove teardown session the driver
appends once no ordinary work is live. It proposes the complete finish cycle and
waits for explicit human confirmation before any teardown; declining leaves the
leaf live for a later resume. No session creates one, and none is ever retired.

<!-- unit: task-review-kinds kinds="review-requirements review-design review-planning review-prototype review-impl" class=triggering -->
**review-\*** (all AFK) — an inspection-only, fresh-context adversarial read of
*one* artifact. Inspect the producer's committed changes, source, requirements
or specifications, and recorded verification evidence. A review does not run
test, build, lint, or format commands, edit production or test code, or redo the
implementation. Its output is findings only; the paired `integrate-review-*`
task owns every fix and all post-fix verification. Five reads look for different
things: `review-requirements` (is anything missing? is each requirement
falsifiable? is a solution smuggled in as a requirement?), `review-design` (does
it satisfy the requirements? are the ADRs a minimum coherent set? are the seams
at the right height and count?), `review-planning` (are the slices vertical?
does each land green without waiting on a sibling? is anything missing?),
`review-prototype` (does it probe the question it was built for? — *not* a code
review; polish is a defect in a prototype), `review-impl` (correctness,
security, tests, project conventions).

<!-- unit: task-integrate-review-kinds kinds="integrate-review-requirements integrate-review-design integrate-review-planning integrate-review-prototype integrate-review-impl" class=triggering -->
**integrate-review-\*** (all AFK) — triage one review's findings and apply the
real ones. Shared discipline: verify each finding rather than performatively
agreeing, then classify it as *a contract stated unclearly* (fix the contract),
*a real issue* (fix the artifact), *a real trade-off* (accept it visibly), or
*noise raised for want of context*. What separates the five is **what the
session may change** — `integrate-review-impl` edits code freely;
`integrate-review-design` reworks the ADR set under its in-place discipline
(merge / split / delete, never a superseding record);
`integrate-review-planning` reshapes the tree; `integrate-review-prototype`
decides what the prototype *taught* and normally discards it;
`integrate-review-requirements` edits what was asked for, which it cannot always
do alone — the kind most likely to stop and ask.

<!-- unit: task-in-session-doubt-budget kinds=* class=triggering -->
**In-session doubt is budgeted across the whole picked leaf**, once the current
session has run Bootstrap and adopted the driver's selected-leaf mandate:

| picked session | in-session reviewer allowance | next substantial review |
|---|---|---|
| plain producer | at most one; every independently materialised reviewer counts | `leaf-add` a `review-<producer>` leaf, with the doubt written into its body |
| producer whose review leaf already exists | none; `review-*` is already scheduled | finish to the scheduled review boundary |
| `review-*` | none; this session is the adversarial read | record findings for integration |
| `integrate-review-*` | at most one narrow reviewer | add a new producer review chain beside the leaf being integrated |
| `research-a` / `research-b` / `combine-research` | none; the pair and combiner own breadth and doubt | put a derived decision in its own reviewed producer chain |

Outside that Bootstrap-and-mandate predicate, doubt-driven development keeps its
standalone bounded cycles. The allowance is leaf-wide, not per artifact or
decision, and a diverse-lens pass with N fresh contexts spends N reviewers.

<!-- unit: task-too-big-is-planning kinds=* class=triggering defers=task-decompose-inherits-kind -->
A task too big for one focused session *is* a planning task — its job is to
decompose, not to do.

<!-- unit: task-decompose-inherits-kind class=procedural -->
`leaf-decompose` gives a node's **first child** the kind of the leaf it just
decomposed, unless `--kind` overrides it — a `research-a` leaf that proves bigger
keeps producing `research-a` work in its first child. The node directory the verb
creates carries no kind, so nothing is inherited at that level.

<!-- unit: task-work-is-not-a-kind class=procedural -->
**`work` is not a kind.** It was the previous spelling of `impl`, and only the
one-time legacy migration still reads it: `work`, `review-work` and
`integrate-review-work` are rewritten to their `impl` spellings as the tree is
converted. Afterwards no reader accepts it — `--kind work` errors and names the
replacement, and a hand-written `work` in a current filename is malformed.

<!-- unit: task-two-shapes kinds=* class=triggering defers="task-review-chain-mechanics task-vendor-pair-mechanics task-leaf-never-names-a-harness" -->
## Composing the kinds — the two shapes

The kinds compose into two habitual shapes. A session cutting leaves should
reach for them **by default**, and argue itself *out* of one rather than into
it:

- **The review chain** — `X` → `review-X` → `integrate-review-X`. Sequential and
  adversarial; each step is a *different* kind, so each resolves its own
  configured command and per-kind configuration alone expresses the shape. Decide
  for it when the artifact is load-bearing (a spec, a decomposition you will
  build on for months, a subsystem); a one-file change wants a mid-session
  subagent instead (`driving.md`).
- **The vendor pair** — `research-a` → `research-b` → `combine-research`. Two
  independent surveys unioned. The producers are **two distinct kinds** sharing
  one discipline, which is how the tree states "two configured targets" without
  any per-leaf routing metadata.

**They are built in opposite ways**, and the asymmetry is the design. A chain's
steps are cut **lazily, one at a time, by the session that needs the next one**;
a pair is cut **eagerly, whole, in one call**.

<!-- unit: task-review-chain-mechanics class=procedural -->
### The review chain — each session cuts the next step

Its steps are ordinary **flat siblings**. There is no chain node and no chain
verb: each is an ordinary `leaf-add` — or, for the integration, a `leaf-insert`
when some later sibling entry would otherwise run first — and it is the **last
act** of the session before it.

```
grove-llm leaf-add [12] sync-design --kind design
05-design-sync-design-k13.md          # cut by the planning session

  …the design session runs, and decides review is required:
grove-llm leaf-add [12] sync-design --kind review-design
06-review-design-sync-design-k14.md

  …the review session runs, and has findings worth acting on:
grove-llm leaf-add [12] sync-design --kind integrate-review-design
07-integrate-review-design-sync-design-k15.md
```

Three leaves, **one slug**, differing only by kind and key. That is the whole
naming rule and *What the shapes are not* below carries why.

Three things follow from cutting them late:

- **Each step is created only if it is required.** The producer cuts a review
  leaf only if review is warranted, and the review cuts an integrate leaf **only
  if it has findings worth acting on**. A review that finds nothing creates
  nothing and simply retires, so the empty triage session is gone; a producer
  that judges review unnecessary cuts no review leaf, and that too is a normal
  outcome rather than a skipped step.
- **The creating session writes the new leaf's body**, and that is the real
  payoff. A review leaf can name the exact case its producer could not cover; an
  integrate leaf can carry the findings verbatim. That is strictly more than a
  constructor rendering a goal sentence from a handle could ever produce, which
  is why the session that knows *why* the step is needed is the right author. The
  template is deliberately bare — a handle and empty sections — so there is
  nothing to edit around.
- **You name the step kind yourself**, so name it off the producer that actually
  ran: `review-<producer>`, then `integrate-review-<producer>`. `--kind
  review-impl` beside a `design` producer is a perfectly valid invocation that
  silently gives the reviewer the wrong discipline, and nothing downstream
  catches it. The five producers are `requirements`, `design`, `planning`,
  `prototype` and `impl`; every one of them has both steps, spelled exactly that
  way.
- **You choose the verb too, and the two hops choose differently.** A review is
  cut with `leaf-add` wherever it lands; an integration is cut **adjacent to the
  review it integrates**, which means `leaf-insert` at **the first sibling entry
  after the review whose subtree still holds live work** whenever there is one.
  *What the shapes are not*, below, carries the reason and the exact condition.

<!-- unit: task-declare-the-relationship kinds=* class=triggering -->
**Declare the relationship in the body, by hand.** A review's body carries
`**Reviews:** <producer-handle>` and an integration's carries `**Integrates:**
<review-handle>`, on their own line, naming the stable handle:

```markdown
# sync-design-k14

**Reviews:** sync-design-k13
```

Nothing writes those lines and **nothing parses them**. They are a convention for
the human reading `find .grove` and for the session that picks the step up —
constraint 3, task files are freeform markdown and nothing validates them. Write
them because the next session benefits, not because a verb requires it.

<!-- unit: task-vendor-pair-mechanics class=procedural -->
### The vendor pair — one eager call

```
grove-llm leaf-add-pair [12] sync-survey
08-research-a-sync-survey-k16.md
09-research-b-sync-survey-k17.md
10-combine-research-sync-survey-k18.md
```

Three consecutive flat siblings, three consecutive keys, three absolute paths on
stdout. **The whole shape lands or none of it does**, and a generated pair is
byte-identical to the same three leaves cut by hand with `leaf-add`.

**Laziness would be wrong here, which is why the pair kept its verb.** If
`research-a` cut `research-b` at the end of its own session, `b` would inherit
`a`'s framing and corpus — destroying the independence the pair is run for. The
pair's three kinds are fixed by the shape, so it takes no `--kind` at all — and
because they are fixed, the three slugs need carry nothing at all beyond the
stem. The two producers are peers, and `research-a` / `research-b` already say
so in the field that routes them.

<!-- unit: task-what-shapes-are-not kinds=* class=triggering defers="task-bare-stem-reasoning task-chain-contiguity" -->
### What the shapes are not

Those names are long, and that is the trade the scheme makes: the **kind** and the
**process** both show up in `find .grove`, so a session's discipline and a chain's
shape are readable without opening anything. Three things that shape looks like
it could be and is not:

<!-- unit: task-bare-stem-reasoning class=procedural -->
- **The slug is the bare stem, and it does not restate the kind.** The kind field
  is the canonical statement of a leaf's role; the slug names the **artifact**.
  So a chain's three steps carry one slug and differ by kind and key, and so do a
  pair's — no `-review`, no `-integrate`, no `-a` / `-b` / `-combine`.

  **Why the marker went.** Each of those five was a 1:1 restatement of the kind
  sitting immediately beside it, and therefore a second, *unvalidated* source of
  truth for a fact grove already parses and routes on. Nothing rejects `leaf-add
  <parent> foo-review --kind impl`; when the two disagree the slug lies while the
  filename tells the truth. Deleting the marker leaves exactly one statement of
  the role, in the field that is checked.

  **What that costs, exactly.** A bare stem stops naming one leaf: `grove-llm
  resolve <stem>` on a chain matches every step and reports
  `Resolution::Ambiguous`. That was never an invariant — grove enforces no slug
  uniqueness anywhere, and `resolve` was built for collisions — and the listing
  prints each match's **path**, which carries the kind, so `resolve <stem>` now
  shows the whole chain with every step's role spelled out; only "give me the
  producer specifically" costs a second lookup.

  Two exactness notes, because both are easy to get backwards. **`resolve` is
  pick-style**: an ambiguous reference is empty stdout, the diagnostic on stderr
  and **exit zero**, because a listing is information rather than a failure. A
  script doing `path=$(grove-llm resolve <stem>)` on a chained stem therefore does
  *not* stop — it succeeds with an empty `path`, and `set -e` will not catch it.
  And **every recommended reference is unaffected, but not every accepted
  one**: the driver's mandate, `**Reviews:**` / `**Integrates:**`
  lines, commit messages and grow-verb targets all name a `<slug>-k<key>` handle,
  a bare key or a path, and keys are unique tree-wide — while `leaf-add`'s
  `<parent>` and `leaf-insert`'s `<target>` *also* accept a bare slug as a
  convenience, and there the same ambiguity is a **refusal** that names the
  matching keys and mutates nothing. So a bare stem that once selected one step of
  a chain now selects none through that path. The read verb reports and the
  mutation verb refuses; the answer to both is the key or handle the guidance
  already recommends.

  **What it does not cost: the surviving commit record.** The stem is *kept*, so a
  handle still names its artifact after `.grove/` dies — the alternative that
  argument was written against is shortening to bare `review` / `integrate`, where
  `review-k14` names a role and no artifact, and that is not what happened here.
  The **role** survives too, structurally: Retire-then-Commit puts the leaf's
  `DONE` rename in the task's own commit, teardown removes `.grove/` from the tip
  and not from history, so `git show --stat <commit>` names the kind-bearing
  filename forever. Rename detection is not needed — without it you get a
  delete/add pair, both kind-bearing. **Do not compensate with a commit-subject
  convention** (`review:` / `impl:` prefixes): that would re-introduce the same
  unvalidated restatement one layer along.

  **Both spellings stay legal and no existing tree is invalidated.** A suffix was
  convention, never grammar (see *The grammar is the five fields* below), so
  nothing was migrated and no leaf was renamed. An older `…-review-k14` you meet
  in a live tree is a well-formed leaf; leave it alone.

  A *prefix* (`review-sync-design`) is no better and is not the alternative on the
  table. The argument once made for the terminal position — that it keeps
  stem-mates together in a directory listing — was in any case false: a leaf name
  begins with `NN`, so a listing sorts by position and never by slug, and both
  spellings glob identically under `*<stem>*`.
<!-- unit: task-no-node-for-a-shape kinds=* class=triggering -->
- **Neither shape gets a node directory.** A charter means *this work proved
  bigger than one session*, which a composed shape is not, and the hierarchy the
  node bought was not worth the navigation cost. So there is **one node species**
  again — a node is a leaf that decomposed, and it carries a `BRIEF.md` — and
  Retire's close has the same work to do at every node it meets.
<!-- unit: task-chain-contiguity class=procedural -->
- **A chain is not a unit, and is not contiguous by construction.** `pick`
  returns the first live leaf in the whole tree and nothing groups leaves for it
  (*task-tree-scheme*). Steps are appended at the parent's next free position, so
  a step cut after some unrelated leaf already exists lands *after* that leaf,
  and a later `leaf-insert` can split a chain that was contiguous. Grove refuses
  none of it — it validates no cross-leaf grammar, and contiguity was never an
  enforced unit. **But that does not make every gap equally free**, and the
  difference is mechanical. A `review-*` step re-derives its inputs: its body
  names the producer's stable handle, task commits name the work item by that
  handle, so it locates the producer's commit and reads that diff against the
  current source — nothing was written down for intervening work to stale. An
  `integrate-review-*` step consumes citations the review already froze into
  prose, against a working tree that has since moved, and the drift is
  **silent** — nothing errors, the finding just points somewhere slightly wrong.
- **So an integration is cut where `pick` reaches it next**, by a condition that
  is mechanical and **directory-local**: `leaf-insert` at **the first sibling
  entry after the review whose subtree still holds live work**. Entry, not
  leaf — `pick` descends a node in place, so a later sibling *node* with a live
  descendant blocks, and the **node directory** is the target, never the live
  leaf inside it (that inserts at the wrong level). A later terminal leaf, **a
  node whose subtree is wholly terminal**, and the driver's `finish` sentinel do
  not block. When nothing blocks, plain `leaf-add` is exactly right: pre-order
  finishes the review's own directory, including a leaf just appended to its end,
  before it visits any later sibling of an ancestor. There is no exception to
  check — the intervening leaf has not run yet and grove makes no leaf's eventual
  file set part of its contract, so nothing could supply the proof.

<!-- unit: task-grammar-is-five-fields kinds=* class=triggering -->
**The grammar is the five fields above and nothing more.** Position, outcome
infix, kind, slug and key are all parsed and all structural — the position orders
the walk, the infix keeps a terminal leaf out of `pick`, the kind keys the
configuration lookup, and slug-plus-key is the handle `resolve` finds and the
counter the next `-k<key>` is allocated from. What is **convention, not grammar**
is everything a name might *imply about another leaf*: the shared stem, the
relative ordering, and the two declaration lines. Grove does not require a
`review-X` after every `X`, reject a partial chain, or parse a `**Reviews:**`
line. It never reconstructs a relationship from a filename, a position, or a
body. That is also the test the deleted step suffix failed and the stem passes: a
convention that *adds* what nothing parses is legible, while one that *duplicates*
a parsed field can disagree with it.

<!-- unit: task-nothing-in-a-body-is-metadata kinds=* class=triggering -->
**Nothing in a body is metadata**, and no verb writes anything there. Retirement
and pruning change a filename and stop; a node close writes nothing at all. So a
`review-*` session has exactly one thing to read — its producer's committed
artifact — and never a note the producer left behind about how its own session
ran. `leaf-add` still means one leaf, and deciding against review is a normal
choice.

<!-- unit: task-suggested-shape class=procedural -->
## Suggested shape

```markdown
# <slug>-k<key>

## Goal
What this one session must deliver.

## Context
Pointers *beyond* the brief chain — specific files, prior leaves, ADRs — that
this task in particular needs. The brief chain and glossary are read anyway;
list only the extras.

## Done when
Concrete, checkable completion conditions for this task.

## Notes
Anything else the executing session should know.
```

The first-line header is the **position-free handle** `# <slug>-k<key>` — the
mutable position `NN` and the routing kind both live only in the filename, never
in the body. That is what
lets a reorder or insert be a pure file move with zero content rewrites, and it is
the same stable handle you cite in commit messages (task-tree-scheme §5). When this leaf
is decomposed into a node, the handle gains a ` — brief` suffix
(`# <slug>-k<key> — brief`) and nothing else changes.

**The body carries no launch metadata at all** — no kind, no harness, no model,
and no record of how any past session ran. A generated leaf is the header plus
those four empty sections, and the only `**…:**` lines any leaf ever carries are
the two composition relationships the *creating session* writes by hand
(`**Reviews:**`, `**Integrates:**`), which describe how artifacts compose rather
than how a session is launched. Everything about the launch comes from the
filename's kind and the one configuration entry it keys.

<!-- unit: task-leaf-never-names-a-harness class=procedural -->
## A leaf never names a harness

There is no way for a leaf to choose the harness, model or wrapper its session
runs on, and no reason to want one: the **kind in its filename is the whole
routing input**, and `~/.config/grove/config.kdl` maps that one key to one
complete command template. No grow verb offers a harness flag, no task body
carries a declaration, and no environment variable or repository stamp
supplements the file.

The shape that used to need a per-leaf declaration was the **vendor pair**, whose
two producers were once the *same* kind and could only be told apart by naming
their vendors in their bodies. Splitting them into two kinds — `research-a` and
`research-b` — moved that distinction into the taxonomy, where the configuration
can address it. That is the whole reason the pair costs two kinds instead of one:
the tree states *two independently configured sessions* as a fact, and whether
their two templates actually reach two different vendors is the configuration
owner's policy, which grove can neither infer from an opaque command string nor
warn about.

<!-- unit: task-three-design-kinds kinds=* class=triggering -->
## The three design kinds — extra guidance

The work today's `planning` label used to cover is split across three kinds, and
each carries part of the old checklist:

<!-- unit: task-deliverable-requirements kinds=requirements class=triggering -->
- **requirements** runs the grilling procedure (`grilling.md`) to interrogate
  *what* is wanted, and updates `CONTEXT.md` **inline** as terms are resolved —
  never batched.
<!-- unit: task-deliverable-design kinds=design class=triggering -->
- **design** raises ADRs **sparingly** — only decisions hard to reverse,
  surprising, or a real trade-off (`ADR-FORMAT.md`) — and MAY write a spec
  (`docs/specs/<slug>.md`) when the increment is a genuine agreement point
  (`SPEC-FORMAT.md`).
<!-- unit: task-deliverable-planning kinds=planning class=triggering -->
- **planning** writes the child `BRIEF.md`(s) and ordered leaf files for any node
  it grows (`BRIEF-FORMAT.md`).

<!-- unit: task-deliverable-split-not-a-gate kinds=* class=triggering -->
The split is a division of *deliverable*, not a gate: a small workstream may
resolve all three in one leaf, and any of the three may sharpen the glossary
inline. What does not blur is tree growth — only `planning` may grow the tree
generatively. *Reactive* decomposition (a leaf proving bigger than its brief) is
kind-agnostic and available to every kind.

<!-- unit: task-bootstrap-leaf-is-requirements kinds=requirements class=triggering -->
**A fresh grove's bootstrap leaf is the standing example of that fusion**, and
it is `requirements` (the driver mints it before any agent exists, with no
`--kind` to change it). Its
only input is the human's own words — nothing else is on disk yet — which is the
HITL rule, so it is labelled for the discipline that *always* applies. A small
workstream's bootstrap session may go on to cut the leaves itself; a larger one
adds a `planning` leaf and lets a fresh session do the decomposition
(fresh-grove-start-contract).
