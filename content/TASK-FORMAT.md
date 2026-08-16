<!-- grove reference file — the task-file shape -->

# TASK-FORMAT — the leaf task file

## The name, field by field

A **leaf** in a grove is a single `.md` task file, named
`NN-[DONE-|ABANDONED-]<session-kind>-<slug>-k<key>.md` — e.g.
`01-requirements-plan-k1.md`, `03-impl-extract-k7.md`. Five fields, all parsed
and all structural:

| field | what it does |
|---|---|
| position `NN` | a 2-digit per-level number, its place among its directory's children; orders the `pick` walk |
| outcome infix | absent while the leaf is live; `DONE` for retired work (`03-DONE-impl-extract-k7.md`) and `ABANDONED` for a path decided against (`03-ABANDONED-impl-extract-k7.md`, `leaf-prune`) — either keeps the leaf out of `pick`, marked in place |
| session kind | one member of the closed set below; the key one command template is configured under |
| slug | a human name for the **artifact**, not for the leaf's role |
| key `-k<key>` | stable identity, the terminal token, assigned once and never reused |

Slug-plus-key is the **work-item handle** `resolve` finds and the counter the next
`-k<key>` is allocated from. One task is one session (constraint: one task per
session), pruning is **HITL** and never an agent's own call, and the file itself
is freeform markdown — a guide follows, not a schema.

**What is convention rather than grammar** is everything a name might imply about
*another* leaf: the shared stem, the relative ordering, and the two declaration
lines. Nothing reconstructs a relationship from a filename, a position, or a
body. That is also the test the deleted step suffix failed and the bare stem
passes — a convention that *adds* what nothing parses is legible, while one that
*duplicates* a parsed field can disagree with it.

Putting the kind in the name is what lets `pick`, the driver's routing lookup and
your own eye read a session's discipline out of `find .grove` without opening a
file. Reading is strict in both directions. Every task-shaped leaf name — live,
`DONE` or `ABANDONED` — must carry a known kind; a missing or unknown one is
malformed and stops tree operations, naming the path and the valid set, rather
than degrading to `impl`. No kind label plus `-` prefixes another, so a name
always separates unambiguously and round-trips without touching the slug. Foreign
non-task files in the tree stay ignored.

## The nineteen kinds

The set is **parameterised, not flat**: five producers, each with its own
`review-` and `integrate-review-` step, plus a research pair and one
driver-owned step.

| kind | review | integrate |
|---|---|---|
| `requirements` | `review-requirements` | `integrate-review-requirements` |
| `design` | `review-design` | `integrate-review-design` |
| `planning` | `review-planning` | `integrate-review-planning` |
| `prototype` | `review-prototype` | `integrate-review-prototype` |
| `impl` | `review-impl` | `integrate-review-impl` |
| `research-a` + `research-b` | — | `combine-research` |
| `finish` — driver-reserved | — | — |

Five producer rows of three, the research row's three, and one driver-owned step:
**nineteen**. Each kind's own file under `references/` carries the discipline it
runs under, and its HITL/AFK mark is on the skill page.

The research row holds **two** kinds rather than one kind run twice: `research-a`
and `research-b` share a discipline but are separate configuration keys, which is
what makes "two independent corpora" a fact in the tree instead of a forecast
about routing policy. `finish` is the driver's own complete-finish-cycle
sentinel: the grow verbs refuse to create one, retire, prune and decompose refuse
it as an operand, and `leaf-insert` may target it only to put ordinary work
*before* teardown.

All three research kinds write under `docs/research/`, and because a pair's three
leaves share one slug, **the kind supplies the discriminator** — the same
principle that keeps it out of the slug:

| kind | writes |
|---|---|
| `research-a` | `docs/research/<slug>-a.md` |
| `research-b` | `docs/research/<slug>-b.md` |
| `combine-research` | `docs/research/<slug>.md` — the union |

A solo `research-a` with no pair writes `-a.md` and that is the whole record; a
`-b` survey added later renames nothing, and the union lands at the unadorned
name where a reader looks first.

`leaf-decompose` gives a node's **first child** the kind of the leaf it just
decomposed, unless `--kind` overrides it — a `research-a` leaf that proves bigger
keeps producing `research-a` work in its first child. The node directory the verb
creates carries no kind, so nothing is inherited at that level.

**`work` is not a kind.** It was the previous spelling of `impl`, and only the
one-time legacy migration still reads it: `work`, `review-work` and
`integrate-review-work` are rewritten to their `impl` spellings as the tree is
converted. Afterwards no reader accepts it — `--kind work` errors and names the
replacement, and a hand-written `work` in a current filename is malformed.

## In-session doubt, by kind

The allowance is budgeted across the **whole picked leaf**, once the current
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

## Composing the kinds — the two shapes

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

### What the shapes are not

Those names are long, and that is the trade the scheme makes: the **kind** and the
**process** both show up in `find .grove`, so a session's discipline and a chain's
shape are readable without opening anything. Three things that shape looks like
it could be and is not — *neither shape gets a node directory* is stated on the
skill page, and these are the other two:

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
  convention, never grammar (see *The name, field by field* above), so nothing was
  migrated and no leaf was renamed. An older `…-review-k14` you meet in a live
  tree is a well-formed leaf; leave it alone.

  A *prefix* (`review-sync-design`) is no better and is not the alternative on the
  table. The argument once made for the terminal position — that it keeps
  stem-mates together in a directory listing — was in any case false: a leaf name
  begins with `NN`, so a listing sorts by position and never by slug, and both
  spellings glob identically under `*<stem>*`.
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

A review or an integration adds the one line that declares what it composes with,
written by hand and parsed by nothing:

```markdown
# sync-design-k14

**Reviews:** sync-design-k13
```

**The body carries no launch metadata at all** — no kind, no harness, no model,
and no record of how any past session ran. A generated leaf is the header plus
those four empty sections, and the only `**…:**` lines any leaf ever carries are
the two composition relationships the *creating session* writes by hand
(`**Reviews:**`, `**Integrates:**`), which describe how artifacts compose rather
than how a session is launched. Everything about the launch comes from the
filename's kind and the one configuration entry it keys.

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
