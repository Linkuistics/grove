<!-- grove reference file — the task-file shape -->

# TASK-FORMAT — the leaf task file

A **leaf** in a grove is a single `.md` task file, named
`NN-[DONE-|ABANDONED-]<slug>-k<key>.md`: a 2-digit per-level **position** `NN`
(its place among its directory's children), a human **slug**, and a permanent
**key** `-k<key>` (stable identity, the terminal token, assigned once, never
reused) — e.g. `01-plan-k1.md`, `03-extract-k7.md`. A leaf ends one of two
ways, marked in place right after the position: retired work carries a `DONE`
infix (`03-DONE-extract-k7.md`); a path decided against carries an `ABANDONED`
infix (`03-ABANDONED-extract-k7.md`, `leaf-prune`, ADR *pruning*) — pruning is
**HITL**, never an agent's own call. One task is one session (constraint: one
task per session). The file is freeform markdown — a guide follows, not a
schema.

## The seventeen kinds

Every task file states its **kind**, drawn from a closed set (ADR
`task-kind-taxonomy`). Adding an eighteenth is a deliberate change to grove's
code and docs, not a free-text label a leaf may coin. The set is
**parameterised, not flat**: five producers, each with its own `review-` and
`integrate-review-` step, plus a research pair.

| producer | review | integrate |
|---|---|---|
| `requirements` | `review-requirements` | `integrate-review-requirements` |
| `design` | `review-design` | `integrate-review-design` |
| `planning` | `review-planning` | `integrate-review-planning` |
| `prototype` | `review-prototype` | `integrate-review-prototype` |
| `impl` | `review-impl` | `integrate-review-impl` |
| `research` | — | `combine-research` |

Each kind is marked **HITL** (resolves through live exchange with a human who
speaks for themselves) or **AFK** (driven by the agent alone). The mark
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
- **planning** (AFK) — given the design, cut it into vertical slices and **grow
  the tree**: turn an oversized leaf into a node — a **directory**
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

- **research** (AFK) — a citation-disciplined literature/prior-art survey
  producing `docs/research/<slug>.md`. Breadth-seeking: a citation per
  failure-mode claim, primary sources, and an explicit note where a search found
  silence (the absence is itself a finding). No grilling, no tree growth.
- **combine-research** (AFK) — union two surveys' coverage and flag every
  disagreement. This kind, not `research`, carries the **adversarial** move: two
  vendors on overlapping corpora can agree on something false, so **agreement
  without independent primary sourcing is a red flag, not a confirmation**.

**review-\*** (all AFK) — a fresh-context adversarial read of *one* artifact,
producing findings, not a fix. Five reads, because they look for different
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
session has run Bootstrap and adopted its own `grove-llm pick` result:

| picked session | in-session reviewer allowance | next substantial review |
|---|---|---|
| plain producer | at most one; every independently materialised reviewer counts | `leaf-promote-chain` the picked producer |
| producer already in a review chain | none; `review-*` is already scheduled | finish to the scheduled review boundary |
| `review-*` | none; this session is the adversarial read | record findings for integration |
| `integrate-review-*` | at most one narrow reviewer | add a new producer review chain inside the owning chain node |
| `research` / `combine-research` | none; the pair and combiner own breadth and doubt | put a derived decision in its own reviewed producer chain |

Outside that Bootstrap-and-pick predicate, doubt-driven development keeps its
standalone bounded cycles. The allowance is leaf-wide, not per artifact or
decision, and a diverse-lens pass with N fresh contexts spends N reviewers.

A task too big for one focused session *is* a planning task — its job is to
decompose, not to do.

`leaf-decompose` gives a node's first child the kind of the leaf it just
decomposed, unless `--kind` overrides it — a research leaf that proves bigger
becomes a research node by default.

**`work` is the previous spelling of `impl`.** A task file still saying
`**Kind:** work` reads as `impl`, silently — it is not a typo. Writing it is
refused: `--kind work` errors and names the replacement.

## Composing the kinds — the two chains

The kinds compose into two habitual shapes. A session cutting leaves should
reach for them **by default**, and argue itself *out* of one rather than into
it:

- **The review chain** — `X` → `review-X` → `integrate-review-X`. Sequential and
  adversarial; each step is a *different* kind, so one `GROVE_REVIEW_HARNESS`
  line routes every review a grove ever cuts. Cut it when the artifact is
  load-bearing (a spec, a decomposition you will build on for months, a
  subsystem); a one-file change wants a mid-session subagent instead
  (`driving.md`).
- **The vendor pair** — `research` → `research` → `combine-research`. Two
  independent surveys unioned. The producers are the *same* kind differing only
  by vendor, which is the entire reason `**Harness:**` exists.

**Each shape is one call**, and each is a **node directory** whose children the
verb names off a shared stem for you:

```
grove-llm leaf-add-chain [12] sync-design --kind design
01-sync-design-chain-k12/            # chain node — no BRIEF.md
  01-sync-design-k13.md              # design
  02-sync-design-review-k14.md       # review-design
  03-sync-design-integrate-k15.md    # integrate-review-design

grove-llm leaf-add-pair [12] sync-survey --harness-a claude --harness-b codex
02-sync-survey-pair-k16/             # chain node — no BRIEF.md
  01-sync-survey-a-k17.md            # research,  **Harness:** claude
  02-sync-survey-b-k18.md            # research,  **Harness:** codex
  03-sync-survey-combine-k19.md      # combine-research
```

You name the chain's **producer** kind and the verb derives the other two; you
name the pair's **two vendors** and it declares both. Neither derivation is
something to do by hand — a `--kind review-impl` beside a `design` producer is a
valid invocation nothing downstream catches, and a pair with only its second
producer declared is not a pair, just a forecast that the first will route
somewhere else. Four keys per shape, not three — the node holds the first, and
stdout is four absolute paths with the node's leading. The whole shape lands or
none of it does, and a generated shape is byte-identical to the same directory
and leaves cut by hand with `mkdir` plus `leaf-add` **and the same stable
relationships**: the review carries `**Reviews:** <producer-handle>` and the
integration carries `**Integrates:** <review-handle>`.

The naming matters because the *kind* lives inside the file while the **process**
shows up in `find .grove`:

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

The names and ordering remain **convention, not grammar**. grove does not read a
suffix, require a `review-X` after every `X`, or reject a partial chain. It does
parse the explicit `Reviews` / `Integrates` relationships and the review's
best-effort `Producer launch` receipt for promotion, retirement, and the
advisory target-diversity warning; it never reconstructs those facts from a
filename or position. `leaf-add` still means one leaf, and skipping a chain is a
normal choice.

## Suggested shape

```markdown
# <slug>-k<key>

**Kind:** impl          (one of the seventeen above)

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
mutable position `NN` lives only in the filename, never in the body. That is what
lets a reorder or insert be a pure file move with zero content rewrites, and it is
the same stable handle you cite in commit messages (task-tree-scheme §5). When this leaf
is decomposed into a node, the handle gains a ` — brief` suffix
(`# <slug>-k<key> — brief`) and nothing else changes.

## Naming a harness — optional, and rare

A leaf MAY declare the harness its session launches on, with a `**Harness:**`
line beside `**Kind:**`:

    **Kind:** research
    **Harness:** codex

Almost no leaf carries one. It exists for the **vendor pair** — two `research`
leaves that differ only by which vendor runs them, plus the `combine-research`
step after them — which is the one shape a kind→harness policy cannot express,
because a policy maps each kind to *one* harness. Everything else is a policy
(`GROVE_<KIND>_HARNESS`) or falls through to the harness the grove is stamped
to. For the pair itself, `leaf-add-pair` writes **both** producers' declarations
— that is the shape it exists for. Otherwise write one with
`leaf-add --harness <name>` / `leaf-insert --harness <name>`, or by hand;
`leaf-decompose` carries a declaration onto the node's first child, as it does
the kind.

The line **beats every policy var and the stamp** — leaf beats kind beats family
beats stamp — so it is read strictly: a name grove does not recognise, or an
empty `**Harness:**`, **refuses to launch** rather than degrading. That is the
opposite of how `**Kind:**` is read, deliberately: a wrong discipline label costs
a warning, while a wrong harness would run the leaf on a vendor the tree
explicitly said not to.

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
it is `requirements` (`root-init` mints it, with no `--kind` to change it). Its
only input is the human's own words — nothing else is on disk yet — which is the
HITL rule, so it is labelled for the discipline that *always* applies. A small
workstream's bootstrap session may go on to cut the leaves itself; a larger one
adds a `planning` leaf and lets a fresh session do the decomposition
(ADR *fresh-grove-start-contract*).
