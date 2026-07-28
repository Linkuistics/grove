# compose-task-chains-k29

**Kind:** impl

## Goal

Make the two composition patterns over the kind set — the **review chain**
(`X` → `review-X` → `integrate-review-X`) and the **vendor pair**
(`research` → `research` → `combine-research`) — the *habitual* shape a session
cuts leaves in, and make a cut chain **legible from the tree on disk**. Both
patterns exist in the taxonomy and in `CONTEXT.md`; nothing in the guidance a
planning session actually reads tells it to reach for them, so in practice it
does not.

Three surfaces, one change: what a session is *told* to do when it decomposes,
what the **bootstrap** session is told (a fresh grove's first session cuts the
whole initial decomposition and is the one that most needs the habit), and how a
chain **names** itself so `find .grove` shows the process, not just the work.

## Context

Raised by the human during **herdr-turn-hooks-k4**; independent of the herdr
work entirely. Two asks, one concern:

1. Encourage `X` → `review-X` → `integrate-review-X` and the research vendor
   pair when breaking work down — including in the bootstrap.
2. Encourage a naming structure that makes the process obvious on inspecting
   `.grove/`.

The surfaces that would carry it, in the order a session meets them:

- `content/prompts/start.md` — the bootstrap launch prompt. The initial
  decomposition happens here, in a `requirements` session, and it is the one
  cut that shapes every later session.
- `content/SKILL.md` — the **Decompose** step, and the `planning` branch of
  **Execute**. Today Decompose is entirely about *when* to externalise work
  (`leaf-add` vs `leaf-decompose`), and says nothing about what shape the
  externalised leaves take.
- `content/TASK-FORMAT.md` — the per-kind discipline, where the chain's steps
  are described one kind at a time but never as a chain to reach for.
- `content/driving.md` — already the field guide for the review chain and for
  commissioning research leaves; the natural home for the worked example.

## Done when

- A planning (or bootstrapping) session reading only the guidance reaches for a
  review chain and a vendor pair without being told to by a human.
- A cut chain is recognisable **from the filenames alone**, without opening a
  file.
- The change is *encouragement*, not enforcement — see the constraint below.

## Notes

**The constraint that shapes this, and it is not negotiable.** grove
**validates no ordering between leaves** — a grammar is a relation *between*
leaves and grove expresses none (ADR *task-kind-taxonomy*; `CONTEXT.md`'s
_Avoid_ on *Review chain* / *vendor pair* says exactly this). So this leaf may
add **prose, defaults and naming habits**, and must not add a checker, a
sequencing rule, or a verb that refuses a chain-less decomposition. Constraint 5
— grove guides, it does not gate. If the work starts wanting a validator, that
is the signal it has left this leaf's goal, not a reason to widen it.

**The naming question is the open one.** The two obvious routes, neither yet
chosen:

- **Slug convention** — a chain shares a stem and the step is a suffix
  (`parser-k30`, `parser-review-k31`, `parser-integrate-k32`), so the chain
  sorts together and reads off `find .grove` directly. Costs nothing
  structurally; the whole cost is that the slug now encodes two things.
- **A node per chain** — `NN-parser-k30/` holding `01-impl`, `02-review`,
  `03-integrate`. Structurally honest and needs no convention at all, but
  spends a node directory on every chain and makes `leaf-decompose` the entry
  move for work that may not need decomposing.

Decide on the evidence of what a real tree looks like — this grove's own
`.grove/` is the sample to read. Note that the **key** is what a durable
reference uses either way (task-tree-scheme §5), so the naming choice is about
human legibility only and is cheap to change later.

**Do not make the chain mandatory-by-default in `leaf-add`.** Auto-creating
three leaves where the human asked for one would grow the tree speculatively,
which is exactly what constraint 4 (lazy, just-in-time) forbids.
