# Concepts

The methodology grove leans on borrows vocabulary from older traditions — **Domain-Driven Design** (Eric Evans, 2003), **Architecture Decision Records** (Michael Nygard, 2011), and the **spec** (a long-standing engineering and product-management practice). This doc is the canonical anchor for those terms inside this repo: what each term means, how grove operationalises it, and where to read more.

## What's not here

- **grove's own vocabulary** — *task, leaf, node, brief, planning task, work task, grilling, retirement* — lives in [`../content/SKILL.md`](../content/SKILL.md) and its sibling files. Those are defined where they're enforced.
- **DDD's tactical building blocks** — *Aggregate, Entity, Value Object, Repository* — aren't load-bearing for grove.
- **Codebase-specific domain terms** — *install scope, path-scoped commit, lifecycle walkthrough* — live in [`../CONTEXT.md`](../CONTEXT.md) (grove's own Ubiquitous Language artifact).

## Domain-Driven Design

**Definition.** Domain-Driven Design (DDD) is a software-design approach that treats the project's *domain* — the area of activity the software serves — as the primary input to architecture. It puts modelling the domain ahead of choosing technology, and insists that the model and the language used to talk about it are the same artifact. Eric Evans introduced it in 2003.

**How grove uses it.** grove is built on two DDD ideas: the **Ubiquitous Language** and the **Bounded Context**, defined below. Both are operationalised as files (`CONTEXT.md`, `CONTEXT-MAP.md`) that every session reads on bootstrap. grove does not adopt DDD's tactical building blocks (Aggregate, Entity, Value Object, Repository) — those are domain-modelling tools that don't bear on how grove drives long workstreams. DDD here is the *framing*, not the full apparatus.

**References.**
- Eric Evans, *Domain-Driven Design: Tackling Complexity in the Heart of Software* (Addison-Wesley, 2003) — the "Blue Book".
- Vaughn Vernon, *Implementing Domain-Driven Design* (Addison-Wesley, 2013) — the "Red Book"; the pragmatic companion.

## Ubiquitous Language

**Definition.** A Ubiquitous Language is the single, shared vocabulary used by domain experts and engineers across conversation, documentation, and code. The same word means the same thing whether spoken in a meeting, written in a doc, or read in a class name. When the language drifts, the model fractures.

**How grove uses it.** The Ubiquitous Language lives in `CONTEXT.md` at the repo root (or per bounded context — see below). It is read at the start of every grove session and appended **inline** whenever a term is resolved during grilling — never batched. This is grove's forcing function against terminology drift across sessions: the glossary is always live, always current, and always the first thing a session reads. `CONTEXT.md` is a *glossary and nothing else* — implementation detail, decisions, and design notes belong in briefs, ADRs, or specs.

**References.**
- Evans, *Domain-Driven Design* (2003), Chapter 2 — introduces the Ubiquitous Language.
- [`../content/CONTEXT-FORMAT.md`](../content/CONTEXT-FORMAT.md) — grove's enforcement of the format.

## Bounded Context

**Definition.** A bounded context is an explicit boundary within which a domain model — and the vocabulary that goes with it — is internally consistent. The same word can mean different things in different bounded contexts; the boundary is what makes that survivable.

**How grove uses it.** Most repos have a single bounded context: one `CONTEXT.md` at the repo root. When a project splits into multiple bounded contexts — typically a sign the domain has clearly distinct sub-areas — each gets its own `CONTEXT.md` in its own subtree, with a `CONTEXT-MAP.md` at the repo root listing them and their relationships. A bounded context is a **domain** partition; a grove task-tree node is a **process** partition — the two are orthogonal and don't compete.

**References.**
- Evans, *Domain-Driven Design* (2003), Part IV (Strategic Design) — the original definition.
- Vernon, *Implementing Domain-Driven Design* (2013), Chapter 2 — practical guidance on identifying boundaries.

## Context Map

**Definition.** A context map is the explicit description of how a project's bounded contexts relate — which contexts share types, which emit events that another consumes, which depend on which. Without it, the boundaries are theoretical; with it, integration choices are visible.

**How grove uses it.** When a repo has more than one bounded context, grove materialises the context map as `CONTEXT-MAP.md` at the repo root. It lists the contexts (each linking to its own `CONTEXT.md`) and the relationships between them — shared types, event flows, dependencies. Most repos don't need one; a single bounded context at the root is the default. When present, the map is read on session bootstrap to locate the relevant `CONTEXT.md`.

**References.**
- Evans, *Domain-Driven Design* (2003), Part IV, Chapter 14.
- Vernon, *Implementing Domain-Driven Design* (2013), Chapter 3 — practical context-mapping patterns.

## ADR

**Definition.** An Architecture Decision Record (ADR) is a short document that captures one architecturally-significant decision: its context, the choice made, the alternatives considered, and the consequences. It exists so a future reader can reconstruct *why* the code looks the way it does without having to interview the original team.

**How grove uses it.** ADRs live in `docs/adr/`, one file per decision, **slug-named** (`docs/adr/<slug>.md`) — the slug is the ADR's identity; cite it by slug or title, never by number. Planning sessions offer them sparingly, and treat the set as a **minimum coherent set describing grove's current design**, not an append-only chronology: when a later decision changes what an earlier ADR recorded, the move is to edit that ADR in place — merge, split, or delete as understanding shifts — never to append a superseding record (the VCS holds what used to be true). Briefs cite the ADRs a leaf-executing session must read; the brief chain root→leaf is the curated path into the project's ADR set. The `linkuistics:decision-records` skill owns the philosophy, format, minimal template, and the when-to-write test that decides whether a decision earns an ADR at all.

**References.**
- Michael Nygard, ["Documenting Architecture Decisions"](https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions) (2011) — the original convention.
- [MADR](https://adr.github.io/madr/) — a widely-used template if you want more structure.
- [`../content/ADR-FORMAT.md`](../content/ADR-FORMAT.md) — grove's placement conventions (where ADRs live, one set per bounded context); the `linkuistics:decision-records` skill owns the format and template.

## Spec

**Definition.** A spec is a human-facing, team-shareable document describing *how an area of the system works* — the problem, the solution, and the decisions that settle it, written so that product, engineering, and stakeholders read the same page. Where an ADR captures one decision and the trade-off behind it, a spec captures a coherent design across many.

**How grove uses it.** Specs live in `docs/specs/<slug>.md`, slug-named, written by a planning task only at a **genuine human-facing agreement point** — never speculatively. The typical grove flow at such a point is *grill → spec (review & agree) → decompose → execute*: the spec is where the human signs off on the design's shape before further decomposition turns it into work tasks. Two rules keep the set honest. **Membership:** would a session on an unrelated future grove need to read this? If not, it belongs in a `BRIEF.md`, which dies with `.grove/`. **Grain:** a spec cites the ADRs in its area rather than restating them. Like `docs/adr/`, `docs/specs/` is a **minimum coherent set describing the design's current state** — edited, merged, and split in place, deleted when a spec no longer describes anything. The VCS holds what a spec used to say.

**References.**
- No single canonical source; the practice is shaped more by use than by a defining text.
- Marty Cagan, *Inspired: How to Create Tech Products Customers Love* (Wiley, 2nd ed., 2017) — framing for product-discovery artifacts.
- [`../content/SPEC-FORMAT.md`](../content/SPEC-FORMAT.md) — grove's shape, and where agreed test seams are recorded.
