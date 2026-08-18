<!-- bundled in grove from mattpocock/skills@b8be62ffacb0118fa3eaa29a0923c87c8c11985c
     — MIT licensed; see LICENSES/mattpocock-skills.LICENSE. Deliberately NOT
     re-synced to upstream's later trim (which removed the example-dialogue,
     relationships, and flagged-ambiguities rules) — grove keeps the richer
     pre-trim superset. One rule below is a further deliberate addition past
     that pin, from mattpocock/skills@d574778f94cf620fcc8ce741584093bc650a61d3
     (skills/productivity/teach/GLOSSARY-FORMAT.md). *Keeping the language
     sharp* below is from the same later pin's
     skills/engineering/domain-modeling/SKILL.md, and reached this file from
     grove's own `grilling.md`, which bundled that skill and stated the rules a
     second time. -->

# CONTEXT.md Format

## Record a term inline, never batched

The acute failure mode of multi-session work is terminology drift: a later
session, with no memory of an earlier one, reinvents a term under a new name or
reuses the words with a shifted meaning. `CONTEXT.md` is the forcing function
against that, and it works only if it is written **as terms are resolved**.

When a term is resolved, write it into `CONTEXT.md` right there. Never batch a
session's terms for the end — a term resolved and not written down is a term the
next session re-resolves differently, which is the drift itself.

## Keeping the language sharp

The glossary is only worth its forcing function if the words going into it are
challenged as they are used. Three moves, whenever a term is in play:

- **Challenge a term that conflicts with the glossary**, immediately: "your
  glossary defines *cancellation* as X, but you seem to mean Y — which is it?"
- **Propose a precise canonical term for a fuzzy one.** "You're saying *account*
  — do you mean the Customer or the User? Those are different things."
- **Cross-check a stated behaviour against the code.** When someone states how
  something works, check whether the code agrees, and surface the contradiction
  if it does not: "your code cancels entire Orders, but you just said partial
  cancellation is possible — which is right?"

## Structure

```md
# {Context Name}

{One or two sentence description of what this context is and why it exists.}

## Language

**Order**:
{A one or two sentence description of the term}
_Avoid_: Purchase, transaction

**Invoice**:
A request for payment sent to a customer after delivery.
_Avoid_: Bill, payment request

**Customer**:
A person or organization that places orders.
_Avoid_: Client, buyer, account
```

## Rules

- **Be opinionated.** When multiple words exist for the same concept, pick the best one and list the others as aliases to avoid.
- **Flag conflicts explicitly.** If a term is used ambiguously, call it out in "Flagged ambiguities" with a clear resolution.
- **Keep definitions tight.** One or two sentences max. Define what it IS, not what it does.
- **It is a glossary and nothing else.** No implementation detail, no design, no scratch pad — `CONTEXT.md` is not a spec, and a decision that wants a home has `docs/adr/` or `docs/specs/`.
- **Use the glossary's own terms inside definitions.** Once a term is in the glossary, prefer it everywhere — including inside other definitions. This is what makes complex terms easier to grasp on a later read.
- **Show relationships.** Use bold term names and express cardinality where obvious.
- **Only include terms specific to this project's context.** General programming concepts (timeouts, error types, utility patterns) don't belong even if the project uses them extensively. Before adding a term, ask: is this a concept unique to this context, or a general programming concept? Only the former belongs.
- **Group terms under subheadings** when natural clusters emerge. If all terms belong to a single cohesive area, a flat list is fine.
- **Write an example dialogue.** A conversation between a dev and a domain expert that demonstrates how the terms interact naturally and clarifies boundaries between related concepts.

## Single vs multi-context repos

**Single context (most repos):** One `CONTEXT.md` at the repo root.

**Multiple contexts:** A `CONTEXT-MAP.md` at the repo root lists the contexts, where they live, and how they relate to each other:

```md
# Context Map

## Contexts

- [Ordering](./src/ordering/CONTEXT.md) — receives and tracks customer orders
- [Billing](./src/billing/CONTEXT.md) — generates invoices and processes payments
- [Fulfillment](./src/fulfillment/CONTEXT.md) — manages warehouse picking and shipping

## Relationships

- **Ordering → Fulfillment**: Ordering emits `OrderPlaced` events; Fulfillment consumes them to start picking
- **Fulfillment → Billing**: Fulfillment emits `ShipmentDispatched` events; Billing consumes them to generate invoices
- **Ordering ↔ Billing**: Shared types for `CustomerId` and `Money`
```

The skill infers which structure applies:

- If `CONTEXT-MAP.md` exists, read it to find contexts
- If only a root `CONTEXT.md` exists, single context
- If neither exists, create a root `CONTEXT.md` lazily when the first term is resolved

When multiple contexts exist, infer which one the current topic relates to. If unclear, ask.
