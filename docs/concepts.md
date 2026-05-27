# Concepts

The methodology grove leans on borrows vocabulary from older traditions — **Domain-Driven Design** (Eric Evans, 2003), **Architecture Decision Records** (Michael Nygard, 2011), and **Product Requirements Documents** (a long-standing product-management practice). This doc is the canonical anchor for those terms inside this repo: what each term means, how grove operationalises it, and where to read more.

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
