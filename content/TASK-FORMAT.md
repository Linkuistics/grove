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
ADR *pruning*) — pruning is
**HITL**, never an agent's own call. One task is one session (constraint: one
task per session). The file is freeform markdown — a guide follows, not a
schema.

**The kind lives in the filename and nowhere else.** It is routing metadata, not
identity: the stable [work-item handle](#suggested-shape) stays `<slug>-k<key>`,
so `grove-llm resolve`, a commit message and an in-file header are all unaffected
by it. Putting it in the name is what lets `pick`, the driver's routing lookup and
your own eye read a session's discipline out of `find .grove` without opening a
file. A **node directory** carries no kind at all (`NN-<slug>-k<key>/`), even when
its slug happens to begin with a kind word.

Reading is strict in both directions. Every task-shaped leaf name — live, `DONE`
or `ABANDONED` — must carry a known kind; a missing or unknown one is malformed
and stops tree operations, naming the path and the valid set, rather than
degrading to `impl`. No kind label plus `-` prefixes another, so a name always
separates unambiguously and round-trips without touching the slug. Foreign
non-task files in the tree stay ignored.

## The nineteen kinds

Every leaf's filename names its **kind**, drawn from a closed set (ADR
`task-kind-taxonomy`). Adding a twentieth is a deliberate change to grove's
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
prune, decompose and promotion refuse it as an operand, and `leaf-insert` may
target it only to put ordinary work *before* teardown.

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

- **requirements** (HITL) — establish *what* should be built. This is where the
  grilling lives (`grilling.md`): interview one question at a time, propose a
  recommended answer for each, walk the design tree until shared understanding
  is reached. Sharpen `CONTEXT.md` inline as terms resolve.
- **design** (AFK) — given requirements, establish *how*. The deliverable is a
  spec, an ADR set, or both. A `design` session that finds itself cutting
  *implementation* leaves has drifted into planning's job and should externalize
  a `planning` leaf instead.
- **planning** (AFK) — given the design, first find the **smallest independently
  useful working increments** and order them by dependency. Create a separate
  grove for every obvious stage that leaves the product working and delivers
  useful, verifiable behavior for its successor; changes that cannot
  independently leave the product working stay in one increment even when their
  code edits are separable. Then cut the current increment into vertical slices
  and **grow the tree**: turn an oversized leaf into a node — a **directory**
  `NN-<slug>-k<key>/` holding ordered child leaves, headed here by a `BRIEF.md`
  (a *decomposition* node; the chain node below is the brief-less species). The
  deliverable is *more tree*. The only kind with methodological force — the sole
  branch in the loop's Execute step.
- **prototype** (HITL) — a cheap, deliberately throwaway artifact built to react
  to, not to ship. The point is the reaction it provokes, not the code's
  survival.
- **impl** (AFK) — produces code, docs, or tests. The deliverable is an artifact
  that ships. (`driving.md` carries the habits: cite framework decisions to the
  source, doubt a hard-to-reverse decision before it stands, and externalize
  surfaced work into new leaves rather than absorbing it.)

**Research** — a **vendor pair**, not a chain: two independent surveys, unioned.

- **research-a** and **research-b** (both AFK) — a citation-disciplined
  literature/prior-art survey producing `docs/research/<slug>.md`.
  Breadth-seeking: a citation per
  failure-mode claim, primary sources, and an explicit note where a search found
  silence (the absence is itself a finding). No grilling, no tree growth. The two
  kinds are identical in discipline and distinct in configuration; a single survey
  that needs no pair is `research-a`.
- **combine-research** (AFK) — union two surveys' coverage and flag every
  disagreement. This kind, not either producer, carries the **adversarial** move:
  two vendors on overlapping corpora can agree on something false, so **agreement
  without independent primary sourcing is a red flag, not a confirmation**.

**finish** (HITL, driver-reserved) — the whole-grove teardown session the driver
appends once no ordinary work is live. It proposes the complete finish cycle and
waits for explicit human confirmation before any teardown; declining leaves the
leaf live for a later resume. No session creates one, and none is ever retired.

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

**In-session doubt is budgeted across the whole picked leaf**, once the current
session has run Bootstrap and adopted the driver's selected-leaf mandate:

| picked session | in-session reviewer allowance | next substantial review |
|---|---|---|
| plain producer | at most one; every independently materialised reviewer counts | `leaf-promote-chain` the picked producer |
| producer already in a review chain | none; `review-*` is already scheduled | finish to the scheduled review boundary |
| `review-*` | none; this session is the adversarial read | record findings for integration |
| `integrate-review-*` | at most one narrow reviewer | add a new producer review chain inside the owning chain node |
| `research-a` / `research-b` / `combine-research` | none; the pair and combiner own breadth and doubt | put a derived decision in its own reviewed producer chain |

Outside that Bootstrap-and-mandate predicate, doubt-driven development keeps its
standalone bounded cycles. The allowance is leaf-wide, not per artifact or
decision, and a diverse-lens pass with N fresh contexts spends N reviewers.

A task too big for one focused session *is* a planning task — its job is to
decompose, not to do.

`leaf-decompose` gives a node's **first child** the kind of the leaf it just
decomposed, unless `--kind` overrides it — a `research-a` leaf that proves bigger
keeps producing `research-a` work in its first child. The node directory the verb
creates carries no kind, so nothing is inherited at that level.

**`work` is not a kind.** It was the previous spelling of `impl`, and only the
one-time legacy migration still reads it: `work`, `review-work` and
`integrate-review-work` are rewritten to their `impl` spellings as the tree is
converted. Afterwards no reader accepts it — `--kind work` errors and names the
replacement, and a hand-written `work` in a current filename is malformed.

## Composing the kinds — the two chains

The kinds compose into two habitual shapes. A session cutting leaves should
reach for them **by default**, and argue itself *out* of one rather than into
it:

- **The review chain** — `X` → `review-X` → `integrate-review-X`. Sequential and
  adversarial; each step is a *different* kind, so each resolves its own
  configured command and per-kind configuration alone expresses the shape. Cut it
  when the artifact is
  load-bearing (a spec, a decomposition you will build on for months, a
  subsystem); a one-file change wants a mid-session subagent instead
  (`driving.md`).
- **The vendor pair** — `research-a` → `research-b` → `combine-research`. Two
  independent surveys unioned. The producers are **two distinct kinds** sharing
  one discipline, which is how the tree states "two configured targets" without
  any per-leaf routing metadata.

**Each shape is one call**, and each is a **node directory** whose children the
verb names off a shared stem for you:

```
grove-llm leaf-add-chain [12] sync-design --kind design
01-sync-design-chain-k12/                        # chain node — no BRIEF.md
  01-design-sync-design-k13.md
  02-review-design-sync-design-review-k14.md
  03-integrate-review-design-sync-design-integrate-k15.md

grove-llm leaf-add-pair [12] sync-survey
02-sync-survey-pair-k16/                         # chain node — no BRIEF.md
  01-research-a-sync-survey-a-k17.md
  02-research-b-sync-survey-b-k18.md
  03-combine-research-sync-survey-combine-k19.md
```

You name the chain's **producer** kind and the verb derives the other two; the
pair's three kinds are fixed, so it takes no kind at all. The chain's derivation
is not something to do by hand — a `--kind review-impl` beside a `design`
producer is a valid invocation nothing downstream catches. Four keys per shape,
not three — the node holds the first, and
stdout is four absolute paths with the node's leading. The whole shape lands or
none of it does, and a generated shape is byte-identical to the same directory
and leaves cut by hand with `mkdir` plus `leaf-add` **and the same stable
relationships**: the review carries `**Reviews:** <producer-handle>` and the
integration carries `**Integrates:** <review-handle>`.

Those names are long, and that is the trade the scheme makes: the **kind** and the
**process** both show up in `find .grove`, so a session's discipline and a chain's
shape are readable without opening anything.

Two things that shape looks like it could be and is not:

- **The suffix goes on the end**, not the front. A suffix keeps a chain's handles
  together under their stem; a prefix (`review-sync-design`) sorts every review
  beside every *other* review and scatters the chains it was meant to reveal. And
  the children keep the **stem** rather than shortening to `01-design` /
  `02-review` now that the node supplies the context: `resolve` matches a bare
  slug exactly and reports more than one match as ambiguous, and `.grove/` dies at
  the finish cycle leaving commit messages as the only record — where `review-k14`
  names a role and no artifact. The node's `-chain` / `-pair` token is the same
  rule one level up; without it the node's slug collides with its first child's.
- **The node carries no `BRIEF.md`, and nothing reads its name.** A charter means
  *this work proved bigger than one session*, which a chain is not; a stub written
  because a step demanded it is what constraint 4 forbids, and the verb knows only
  a stem anyway. Its absence is what lets the Retire cascade give a closing chain
  node **nothing to do** — no `Done when` to check against the subtree and no
  brief to promote — where a decomposition node's close has both
  (*confirmation-boundary*; the close asks nothing of either species). That
  discriminator is a **file test**: the
  `-chain` / `-pair` token is ordinary slug text, and anything that parsed it would
  reintroduce exactly the convention-reading this section forbids.
- **A chain is not a unit, either.** Containment is not immunity: `pick` descends
  a chain node in pre-order like any other and walks straight out into the next
  sibling once its steps are done — it returns the first live leaf in the whole
  tree and nothing groups leaves for it (*task-tree-scheme*). What the directory
  *does* buy is that a sibling-level `leaf-insert` can no longer split the steps,
  and that a step decided on *after* its producer ran is
  `leaf-add <chain-node> <stem>-late-step`, appending **inside** the node —
  immediately after its stem-mates, ahead of everything outside. A **currently
  picked plain producer** that now needs review uses `grove-llm
  leaf-promote-chain <picked-producer>`: the operation atomically creates the
  brief-less node, preserves the producer's handle and bytes, derives the review
  and integration leaves, and writes their stable relationships.

The pair peers are `-a` and `-b` rather than one bare stem and one suffixed,
because they are peers: a bare stem beside a `-second` implies a producer/step
relation the pair does not have.

**The kind is the only part of a name grove parses.** Everything else about a
filename — the stem, the step suffix, the position, the ordering — remains
**convention, not grammar**: grove does not read a
suffix, require a `review-X` after every `X`, or reject a partial chain. The two
explicit `Reviews` / `Integrates` declarations are the one exception, and even
they are read as a **lookup, never a parse**: grove arrives holding a handle and
asks *which sibling declares it*, matching the rest of the line exactly, so a
half-edited `**Reviews:** sync-design-k12 (stale)` declares nothing. It never
asks the opposite question — *what does this task declare?* — and never
reconstructs a relationship from a filename or a position.

**Nothing else in a body is metadata**, and no verb writes one there. Retirement
and pruning change a filename and stop; a node close writes nothing at all. So a
`review-*` session has exactly one thing to read — its producer's committed
artifact — and never a note the producer left behind about how its own session
ran. `leaf-add` still means one leaf, and skipping a chain is a normal choice.

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
those four empty sections, and the
only `**…:**` lines any leaf ever carries are the two composition relationships a
chain writes for you (`**Reviews:**`, `**Integrates:**`), which describe how
artifacts compose rather than how a session is launched. Everything about the
launch comes from the filename's kind and the one configuration entry it keys.

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

## The three design kinds — extra guidance

The work today's `planning` label used to cover is split across three kinds, and
each carries part of the old checklist:

- **requirements** runs the grilling procedure (`grilling.md`) to interrogate
  *what* is wanted, and updates `CONTEXT.md` **inline** as terms are resolved —
  never batched.
- **design** raises ADRs **sparingly** — only decisions hard to reverse,
  surprising, or a real trade-off (`ADR-FORMAT.md`) — and MAY write a spec
  (`docs/specs/<slug>.md`) when the increment is a genuine agreement point
  (`SPEC-FORMAT.md`).
- **planning** writes the child `BRIEF.md`(s) and ordered leaf files for any node
  it grows (`BRIEF-FORMAT.md`).

The split is a division of *deliverable*, not a gate: a small workstream may
resolve all three in one leaf, and any of the three may sharpen the glossary
inline. What does not blur is tree growth — only `planning` may grow the tree
generatively. *Reactive* decomposition (a leaf proving bigger than its brief) is
kind-agnostic and available to every kind.

**A fresh grove's bootstrap leaf is the standing example of that fusion**, and
it is `requirements` (the driver mints it before any agent exists, with no
`--kind` to change it). Its
only input is the human's own words — nothing else is on disk yet — which is the
HITL rule, so it is labelled for the discipline that *always* applies. A small
workstream's bootstrap session may go on to cut the leaves itself; a larger one
adds a `planning` leaf and lets a fresh session do the decomposition
(ADR *fresh-grove-start-contract*).
