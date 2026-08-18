---
name: codebase-design
description: Shared vocabulary and checkable principles for designing deep modules — a lot of behaviour behind a small interface, placed at a clean seam, testable through it (Ousterhout depth + Feathers seams), language-neutral. Use when designing or restructuring a module's interface, deciding where a seam goes, judging whether an abstraction earns its place, or making code more testable.
---

# Codebase Design

Design **deep modules**: a lot of behaviour behind a small interface, placed at a
clean seam, testable through that interface. Depth buys **leverage** for callers,
**locality** for maintainers, **testability** for everyone.

The terms are **scale-agnostic** and **language-neutral** — they apply to a
function, a class, a package, or a tier-spanning slice.

## Glossary

Use these terms exactly; consistent language is the whole point.

**Module** — anything with an interface and an implementation. _Avoid_: unit,
component, service.

**Interface** — everything a caller must know to use the module correctly: the
type signature, and also invariants, ordering constraints, error modes, required
configuration, and performance characteristics. _Avoid_: API, signature — they
name only the type-level surface.

**Implementation** — what is inside a module. Distinct from **adapter**: a thing
can be a small adapter over a large implementation (a real database repository)
or a large adapter over a small one (an in-memory fake). Say "adapter" when the
seam is the topic.

**Depth** — leverage at the interface: how much behaviour a caller or test can
exercise per unit of interface it must learn. **Deep** = large behaviour behind a
small interface; **shallow** = an interface nearly as complex as what it hides.
_Avoid_: depth as a lines-of-implementation ratio (Ousterhout's own metric) — it
rewards padding the implementation.

**Seam** _(Michael Feathers)_ — a place where you can alter behaviour without
editing in that place; the *location* at which a module's interface lives. Where
the seam goes is its own decision, separate from what goes behind it. _Avoid_:
boundary — overloaded with DDD's bounded context.

**Adapter** — a concrete thing satisfying an interface at a seam. A *role* (which
slot it fills), not substance (what is inside).

**Leverage** — what callers get from depth. **Locality** — what maintainers get:
change, bugs and verification concentrate in one place instead of spreading
across callers.

## Principles

- **Depth is a property of the interface, not the implementation.** A deep module
  may be internally composed of small swappable parts — they are just not in the
  interface. A module can have **internal seams** (private, used by its own
  tests) as well as the **external seam** at its interface.
- **The deletion test.** Imagine deleting the module. If complexity vanishes, it
  was a pass-through and earned nothing. If the same complexity reappears
  scattered across N callers, it was earning its keep.
- **The interface is the test surface.** Callers and tests cross the same seam.
  Wanting to test *past* the interface means the module is the wrong shape.
- **One adapter means a hypothetical seam; two mean a real one.** Don't introduce
  a port or injection point unless something actually varies across it.
- **Name one concept one way.** Two names for one thing (`get_thing` and
  `fetch_thing`; `BlahName` and `FooId` both wrapping a string id) is interface
  surface a caller has to learn twice. Pick one and rename the outliers.

## Designing for testability

1. **Accept dependencies, don't create them.** `process_order(order, gateway)` can
   be given a fake; a `process_order` that constructs its own gateway cannot.
2. **Return results, don't mutate in place.** A returned `Discount` is inspected
   directly; a mutated cart forces the test to reconstruct shared state.
3. **Small surface area.** Fewer entry points mean fewer tests; fewer parameters
   mean simpler setup. Depth and testability pull the same way.

## Deepening a cluster

To merge shallow modules behind one interface, first classify the cluster's
dependencies — the category decides how the deepened module is tested.

| Dependency category | What it is | Test strategy |
|---|---|---|
| **In-process** | Pure computation, in-memory state, no I/O | Always deepenable. Test through the new interface directly — no adapter. |
| **Local-substitutable** | Has a local stand-in (in-memory store, embedded database) | Deepenable if the stand-in exists. The seam is **internal**; run the stand-in in the suite. |
| **Remote but owned** | Your own services across a network | Define a **port** at the seam; the transport is an injected adapter (in-memory for tests, network for production). |
| **True external** | Third-party services you don't control | Injected port; tests supply a mock adapter. |

Hold to "two adapters mean a real seam" here, and keep **internal seams
internal** — don't expose one through the interface just because the module's own
tests use it.

**Replace, don't layer.** Unit tests on the absorbed shallow modules become waste
— delete them rather than stacking new tests on top. Write fresh tests at the
deepened interface, asserting observable outcomes rather than internal state: a
test that must change when the implementation changes was testing past the
interface.

## Design it twice

Your first interface is rarely your best. Before committing, design the same
module's interface **at least two radically different ways** — one minimising
entry points, one maximising flexibility, one optimising the most common caller.
Compare on **depth**, **locality**, and **seam placement**, then pick
deliberately or compose a hybrid. (After Ousterhout's "design it twice".)

For a candidate substantial enough to warrant it, run the alternatives as **3–4
parallel sub-agents** — each given the module's constraints, its dependency
categories, and one divergent design pressure: the three above, plus a fourth
designed around a real ports-and-adapters seam whenever one is in play. That
fourth is the point of the exercise when a seam is load-bearing; three generic
interface variants will not probe it. Each returns an interface, a usage
example, what it hides, its dependency strategy, and its trade-offs. Present them
sequentially, then compare and recommend — be opinionated; the point is a strong
read, not a menu (`mattpocock/skills` `codebase-design/DESIGN-IT-TWICE.md`).
