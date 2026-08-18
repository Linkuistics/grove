---
name: decision-records
description: Decision records (ADRs) as a minimum coherent set describing the design's current state — current-state over changelog, edit/merge/split/delete in place, identity by slug not number, the when-to-write test, a minimal template. Use when writing, revising, pruning, citing, or reviewing ADRs / decision records under docs/adr/.
---

# Decision Records

An **Architecture Decision Record (ADR)** captures one design decision — what was
decided and why it binds. The set under `docs/adr/` is **the minimum coherent set
describing the design's current state**: as few records as coherently explain what
the design *is* today, its still-binding constraints, and the alternatives
rejected for a reason. **ADRs hold the present; the VCS holds the past.**

> The minimal template, the three-part when-to-write test, and the qualifying
> examples are distilled from the ADR-format material in `mattpocock/skills`
> (MIT) at `b8be62ff`; full attribution is in
> [`../../PROVENANCE.md`](../../PROVENANCE.md). The minimum-coherent-set framing —
> current-state over changelog, edit-in-place rework, identity by slug rather than
> number — is original to this skill.

## The set is the state

Treat the ADR set the way a well-run codebase treats its source tree: it describes
what exists **now**, not the sequence of edits that produced it.

- **Minimum coherent set.** Prefer few self-contained ADRs over many
  cross-referencing ones. An ADR a reader must chase through three predecessors to
  understand is one ADR pretending to be four. The target is the *smallest* set
  that explains the current design, not the *complete* set of decisions ever taken.
- **Current-state, not a changelog.** An ADR states what *is* and why it binds.
  There is no `superseded by`, no `Status: deprecated` chain, no record of the
  decision you changed your mind about — that one is simply *gone from the current
  design*, and the commit that edited or deleted it is the audit trail.

## What one ADR holds

The **distilled lesson**, not the meeting minutes:

1. **The decision** — what the design does, as a present-tense fact ("the write
   model is event-sourced; the read model is projected into Postgres").
2. **Why it binds** — the constraint that makes it non-obvious and costly to
   reverse.
3. **The alternative rejected for a reason** — only when the rejection is
   non-obvious and someone will otherwise re-propose it.

Discard how the team arrived there: who argued what, which options were floated,
the order of the reasoning. That story belongs to the VCS.

## Keep the set minimal and coherent

Rework the set **in place** as understanding changes — normal maintenance, not an
exceptional event. **Edit** when a decision shifts (never append a superseding
record). **Merge** when one decision has absorbed another. **Split** when a record
has quietly grown two independent decisions. **Delete** when a decision no longer
describes the current design.

After any rework, **reconcile every citation** to the surviving records. A merge
or delete that leaves a dangling reference is a defect.

## Cite by slug, never by number

An ADR's identity is its **slug / title**. A number encodes creation order —
chronology, the thing the current-state rule discards — and fights the rework
above: merge two ADRs and every later number is a lie about position.

- Filenames are slug-only: `docs/adr/event-sourced-write-model.md`, not
  `0007-event-sourced-write-model.md`.
- Slugs are unique within a `docs/adr/` directory; the slug is the handle.
- Citations name the slug or title, so they survive the set being reworked.

## When to write one

Write an ADR only when **all three** hold:

1. **Hard to reverse** — changing your mind later costs something meaningful.
2. **Surprising without context** — a future reader will look at the code and
   wonder "why on earth did they do it this way?"
3. **The result of a real trade-off** — genuine alternatives existed and you
   picked one for specific reasons.

Miss any one and there is no ADR to write: an easy reversal will just be reversed,
an unsurprising choice raises no question, and where there was no alternative
there is nothing to record beyond "we did the obvious thing."

What clears all three, typically: architectural shape ("we use a monorepo");
integration patterns between contexts ("Ordering and Billing communicate via
domain events, not synchronous HTTP"); technology choices carrying real lock-in
(database, message bus, auth provider — not every library); boundary and scope
decisions, where the explicit *no*s matter as much as the *yes*es; deliberate
deviations from the obvious path ("manual SQL instead of an ORM because X"),
which stop the next engineer "fixing" something intentional; and constraints
invisible in the code ("response times must stay under 200ms because of the
partner API contract").

## The template

An ADR can be a single paragraph. The value is in recording *that* a decision was
made and *why*, not in filling out sections.

```md
# {Short title of the decision}

{1–3 sentences: the context, what was decided, and why it binds.}
```

Add a section only when it earns its place — most need none:

- **Considered options**, when the rejected alternatives are worth remembering.
  Each should name **what would reopen it**. Without a trigger a rejection is a
  **tombstone**; with one it is a **gate with a key** — a future reader tests the
  trigger against present conditions instead of re-litigating from scratch. Some
  rejections are closed forever (a naming call, a dead technology), and "nothing
  would reopen this" is a legitimate answer; the point is asking.
- **Consequences**, when non-obvious downstream effects need calling out.

Do **not** add a `Status` line. Status framing is changelog machinery: an ADR that
exists *is* the current decision, and one that no longer describes the design is
edited or deleted, not marked "superseded."
