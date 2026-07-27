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
  `NN-<slug>-k<key>/` holding a `BRIEF.md` plus ordered child leaves. The
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

The two shapes above are **conventions, not a grammar**. grove does not validate
that a `review-X` leaf follows an `X` leaf, because a grammar is a relation
*between* leaves and grove expresses none. Compose them by hand.

A task too big for one focused session *is* a planning task — its job is to
decompose, not to do.

`leaf-decompose` gives a node's first child the kind of the leaf it just
decomposed, unless `--kind` overrides it — a research leaf that proves bigger
becomes a research node by default.

**`work` is the previous spelling of `impl`.** A task file still saying
`**Kind:** work` reads as `impl`, silently — it is not a typo. Writing it is
refused: `--kind work` errors and names the replacement.

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
because a policy maps each kind to *one* harness
(`docs/specs/task-kind-taxonomy.md`, *Routing*). Everything else is a policy
(`GROVE_<KIND>_HARNESS`) or falls through to the harness the grove is stamped
to. Write one with `leaf-add --harness <name>` / `leaf-insert --harness <name>`,
or by hand; `leaf-decompose` carries a declaration onto the node's first child,
as it does the kind.

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

The split is a division of *deliverable*, not a gate: a small workstream may do
all three in one `planning` leaf, and any of the three may sharpen the glossary
inline. What does not blur is tree growth — only `planning` may grow the tree
generatively. *Reactive* decomposition (a leaf proving bigger than its brief) is
kind-agnostic and available to every kind.
