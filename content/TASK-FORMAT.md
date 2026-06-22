<!-- grove reference file — the task-file shape -->

# TASK-FORMAT — the leaf task file

A **leaf** in a grove is a single `.md` task file, named
`NN-[DONE-]<slug>-k<key>.md`: a 2-digit per-level **position** `NN` (its place
among its directory's children), a human **slug**, and a permanent **key**
`-k<key>` (stable identity, the terminal token, assigned once, never reused) —
e.g. `01-plan-k1.md`, `03-extract-k7.md`. A retired leaf carries a `DONE` infix
right after the position: `03-DONE-extract-k7.md`. One task is one session
(constraint: one task per session). The file is freeform markdown — a guide
follows, not a schema.

## The two kinds

Every task file states its **kind**. There are two:

- **work** — produces code, docs, or tests. The deliverable is an artifact.
  (`driving.md` carries the work-session habits: cite framework decisions to the
  source, and doubt a hard-to-reverse decision before it stands.)
- **planning** — grills, sharpens the glossary, may raise an ADR or a PRD, and
  **grows the tree**: turns an oversized leaf into a node — a **directory**
  `NN-<slug>-k<key>/` holding a `BRIEF.md` plus ordered child leaves. The
  deliverable is *more tree*.

A task too big for one focused session *is* a planning task — its job is to
decompose, not to do.

## Suggested shape

```markdown
# <slug>-k<key>

**Kind:** work          (or: planning)

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
lets a reorder or insert be a pure `git mv` with zero content rewrites, and it is
the same stable handle you cite in commit messages (ADR-0035 §5). When this leaf
is decomposed into a node, the handle gains a ` — brief` suffix
(`# <slug>-k<key> — brief`) and nothing else changes.

## Planning tasks — extra guidance

A planning task additionally:

- runs the grilling procedure (`grilling.md`) to interrogate the design;
- updates `CONTEXT.md` **inline** as terms are resolved — never batched;
- raises ADRs **sparingly** — only decisions hard to reverse, surprising, or a
  real trade-off (`ADR-FORMAT.md`);
- MAY write a PRD (`docs/prd/`) when the increment is a genuine agreement point;
- writes the child `BRIEF.md`(s) and ordered leaf files for any node it grows
  (`BRIEF-FORMAT.md`).
