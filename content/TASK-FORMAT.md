<!-- grove reference file — the task-file shape -->

# TASK-FORMAT — the leaf task file

This file is the leaf's **grammar**: what constrains bytes on disk, and nothing
else. When to cut a leaf, how the steps of a composed shape relate, and what a
session may spend on doubt are conduct, and each is stated where a session meets
it — `references/decompose.md` and `references/execute.md`.

## The name, field by field

A **leaf** in a grove is a single `.md` task file, named
`NN-[DONE-|ABANDONED-]<session-kind>--<slug>-k<key>.md` — e.g.
`01-requirements--plan-k1.md`, `03-impl--extract-k7.md`. Five fields, all parsed
and all structural:

| field | what it does |
|---|---|
| position `NN` | a 2-digit per-level number, its place among its directory's children; orders the `pick` walk |
| outcome infix | absent while the leaf is live; `DONE` for retired work (`03-DONE-impl--extract-k7.md`) and `ABANDONED` for a path decided against (`03-ABANDONED-impl--extract-k7.md`, `leaf-prune`) — either keeps the leaf out of `pick`, marked in place |
| session kind | one member of the closed set below; the key one command template is configured under |
| slug | a human name for the **artifact**, not for the leaf's role |
| key `-k<key>` | stable identity, the terminal token, assigned once and never reused |

Slug-plus-key is the **work-item handle** `resolve` finds and the counter the next
`-k<key>` is allocated from. The file itself is freeform markdown — a guide
follows, not a schema.

**What is convention rather than grammar** is everything a name might imply about
*another* leaf: the shared stem a composed shape's steps carry, their relative
ordering, and the two `**Reviews:**` / `**Integrates:**` declaration lines in
their bodies. Every one of those is written by hand and parsed by nothing, and
nothing reconstructs a relationship from a filename, a position, or a body. That
is also the test the deleted step suffix failed and the bare stem passes — a
convention that *adds* what nothing parses is legible, while one that
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

A sixth section appears when a session is keeping one: **`## Decisions (running
log)`**, a paragraph per settled question. It is part of the body's shape, and
*when* to append to it is `references/execute.md`'s.

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
filename's kind and the one configuration entry it keys
(`references/driver.md`).
