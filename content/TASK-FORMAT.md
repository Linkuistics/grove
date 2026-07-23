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

## The five kinds

Every task file states its **kind**, drawn from a closed set of five (ADR
`task-kind-taxonomy`). Adding a sixth is a deliberate change to grove's code
and docs, not a free-text label a leaf may coin. Each is marked **HITL**
(resolves through live exchange with a human who speaks for themselves) or
**AFK** (driven by the agent alone) — a HITL leaf reached by an unattended
relaunch of the self-driving loop simply waits for a human, which is correct
behaviour, not a fault:

- **planning** (HITL) — grills, sharpens the glossary, may raise an ADR or a
  spec, and **grows the tree**: turns an oversized leaf into a node — a
  **directory** `NN-<slug>-k<key>/` holding a `BRIEF.md` plus ordered child
  leaves. The deliverable is *more tree*. The only kind with methodological
  force — the sole branch in the loop's Execute step.
- **research** (AFK) — a citation-disciplined literature/prior-art survey.
  Produces `docs/research/<slug>.md`; no grilling, no tree growth.
- **prototype** (HITL) — a cheap, deliberately throwaway artifact built to
  react to, not to ship. The point is the reaction it provokes, not the code's
  survival.
- **work** (AFK) — produces code, docs, or tests. The deliverable is an
  artifact. (`driving.md` carries the work-session habits: cite framework
  decisions to the source, doubt a hard-to-reverse decision before it stands,
  and externalize surfaced work into new leaves rather than absorbing it.)
- **review** (AFK) — a fresh-context adversarial read of already-done work.
  Produces findings, not a fix.

A task too big for one focused session *is* a planning task — its job is to
decompose, not to do.

`leaf-decompose` gives a node's first child the kind of the leaf it just
decomposed, unless `--kind` overrides it — a research leaf that proves bigger
becomes a research node by default.

## Suggested shape

```markdown
# <slug>-k<key>

**Kind:** work          (or: planning, research, prototype, review)

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

## Planning tasks — extra guidance

A planning task additionally:

- runs the grilling procedure (`grilling.md`) to interrogate the design;
- updates `CONTEXT.md` **inline** as terms are resolved — never batched;
- raises ADRs **sparingly** — only decisions hard to reverse, surprising, or a
  real trade-off (`ADR-FORMAT.md`);
- MAY write a spec (`docs/specs/<slug>.md`) when the increment is a genuine
  agreement point (`SPEC-FORMAT.md`);
- writes the child `BRIEF.md`(s) and ordered leaf files for any node it grows
  (`BRIEF-FORMAT.md`).
