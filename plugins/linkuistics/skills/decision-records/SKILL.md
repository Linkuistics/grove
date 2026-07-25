---
name: decision-records
description: Decision records (ADRs) as a minimum coherent set describing the design's current state — current-state over changelog, edit/merge/split/delete in place, identity by slug not number, the when-to-write test, a minimal template. Use when writing, revising, pruning, citing, or reviewing ADRs / decision records under docs/adr/.
---

# Decision Records

An **Architecture Decision Record (ADR)** captures one design decision — what was
decided and why it binds. The set of ADRs under `docs/adr/` is **the minimum
coherent set describing the design's current state**: as few records as coherently
explain what the design *is* today, its still-binding constraints, and the
alternatives rejected for a reason. **ADRs hold the present; the VCS holds the past.**

> The minimal template, the three-part when-to-write test, and the qualifying
> examples below are distilled from the ADR-format material in `mattpocock/skills`
> (MIT) — the snapshot frozen at `@b8be62ff`, surveyed in
> `docs/research/skill-repo-prior-art.md`. The minimum-coherent-set framing —
> current-state over changelog, edit-in-place rework, and identity by slug rather
> than number — is original to this skill.

## The set is the state

Treat the ADR set the way a well-run codebase treats its source tree: it describes
what exists **now**, not the sequence of edits that produced it. Two consequences,
and everything else follows from them:

- **Minimum coherent set.** Prefer few, self-contained ADRs over many
  cross-referencing ones. An ADR a reader must chase through three predecessors to
  understand is one ADR pretending to be four. The target is the *smallest* set
  that still coherently explains the current design — not the *complete* set of
  every decision ever taken.
- **Current-state, not a changelog.** An ADR states what *is* and why it binds —
  never the path taken to get there. There is no `superseded by`, no
  `Status: deprecated` chain, no record of the decision you changed your mind
  about. The decision you reversed is not "superseded history" to preserve; it is
  simply *gone from the current design*, and **the VCS already holds it**. A commit
  that edits or deletes an ADR is the audit trail.

This is the same discipline a task tree or a source tree lives by — the artifact
is the state, the version-control history is the history — applied to design
documentation.

## What one ADR holds

Write the **distilled lesson**, not the meeting minutes. Keep three things:

1. **The decision** — what the design does, stated as a present-tense fact
   ("the write model is event-sourced; the read model is projected into Postgres").
2. **Why it binds** — the constraint or rationale that makes it non-obvious and
   costly to reverse.
3. **The alternative rejected for a reason** — but only when the rejection is
   non-obvious and someone will otherwise re-propose it.

Discard the narrative of *how the team arrived there*: who argued what, which
options were floated and dropped, the order of the reasoning. That story is real,
but it belongs to the VCS, not to the record of the current design.

## Keep the set minimal and coherent

As understanding changes, **rework the set in place** — this is normal
maintenance, not an exceptional event:

- **Edit** an ADR when the decision it describes shifts. Change the text; do not
  append a new ADR that supersedes it.
- **Merge** two ADRs when one decision has absorbed another, or when a live lesson
  belongs folded into a surviving record.
- **Split** an ADR when it has quietly grown to hold two independent decisions.
- **Delete** an ADR when its decision no longer describes the current design.
  Deleting is not losing it — the VCS holds it.

After any rework, **reconcile every citation** to the surviving records. A merge
or delete that leaves a dangling reference is a defect, not acceptable collateral.

## Cite by slug, never by number

An ADR's identity is its **slug / title**, never a sequence number. A number
encodes creation order — which is chronology, the exact thing the current-state
rule discards. Numbering also fights the rework above: merge two ADRs and every
later number is a lie about position.

- **Filenames are slug-only:** `docs/adr/event-sourced-write-model.md`, not
  `0007-event-sourced-write-model.md`.
- **Slugs are unique** within a `docs/adr/` directory — the slug is now the handle.
- **Citations name the slug or title** ("see the *event-sourced-write-model*
  decision"), so a citation survives the set being reworked around it.

## When to write one

Write an ADR only when **all three** hold:

1. **Hard to reverse** — the cost of changing your mind later is meaningful.
2. **Surprising without context** — a future reader will look at the code and
   wonder "why on earth did they do it this way?"
3. **The result of a real trade-off** — there were genuine alternatives and you
   picked one for specific reasons.

If a decision is easy to reverse, skip it — you will just reverse it. If it is not
surprising, nobody will wonder why. If there was no real alternative, there is
nothing to record beyond "we did the obvious thing." Miss any one of the three and
there is no ADR to write.

### What qualifies

- **Architectural shape.** "We use a monorepo." "The write model is event-sourced,
  the read model projected into Postgres."
- **Integration patterns between contexts.** "Ordering and Billing communicate via
  domain events, not synchronous HTTP."
- **Technology choices that carry lock-in.** Database, message bus, auth provider,
  deployment target — the ones that would take a quarter to swap out, not every
  library.
- **Boundary and scope decisions.** "Customer data is owned by the Customer
  context; other contexts reference it by ID only." The explicit *no*s matter as
  much as the *yes*es.
- **Deliberate deviations from the obvious path.** "Manual SQL instead of an ORM
  because X." Anything where a reasonable reader would assume the opposite — it
  stops the next engineer from "fixing" something deliberate.
- **Constraints not visible in the code.** "Response times must stay under 200ms
  because of the partner API contract." "We cannot use AWS for compliance reasons."
- **Rejected alternatives when the rejection is non-obvious.** Considered GraphQL,
  picked REST for subtle reasons — record it, or someone re-proposes GraphQL in six
  months.

## The template

An ADR can be a single paragraph. The value is in recording *that* a decision was
made and *why* — not in filling out sections.

```md
# {Short title of the decision}

{1–3 sentences: the context, what was decided, and why it binds.}
```

Add a section **only when it earns its place** — most ADRs need none:

- **Considered options** — when the rejected alternatives are worth remembering.
  Each rejected alternative should also name **what would reopen it** — the
  condition under which the path becomes live again. Without one, a rejection is a
  **tombstone**; with one, it's a **gate with a key** — a future reader can test the
  trigger against present conditions instead of re-litigating the decision from
  scratch, which is the entire reason the entry exists. Not every rejection has a
  meaningful trigger — some are closed forever, a naming call or a dead technology —
  and "nothing would reopen this" is a legitimate answer; the point is asking the
  question, not manufacturing a trigger that isn't there.
- **Consequences** — when non-obvious downstream effects need calling out.

Do **not** add a `Status` line (`proposed | accepted | deprecated | superseded
by …`). Status framing is changelog machinery: an ADR that exists *is* the current
decision, and one that no longer describes the design is edited or deleted, not
marked "superseded."

## Rejected framings

- **Sequential numbering (`0001-…`, `0002-…`).** Bakes creation order — chronology
  — into the filename. Use a slug.
- **`superseded by` / `deprecated` status chains.** Reconstructs a changelog inside
  the set. Edit or delete the record; the VCS holds the prior version.
- **One new ADR per revised decision.** Grows an append-only log, the opposite of a
  minimum coherent set. Rework the existing ADR in place.
- **A complete decision archive.** The goal is the *smallest coherent* description
  of the current design, not the *complete* record of every decision — that record
  is the VCS history.
