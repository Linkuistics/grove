---
name: codebase-design
description: Shared vocabulary and checkable principles for designing deep modules — a lot of behaviour behind a small interface, placed at a clean seam, testable through it (Ousterhout depth + Feathers seams), language-neutral. Use when designing or restructuring a module's interface, deciding where a seam goes, judging whether an abstraction earns its place, or making code more testable.
---

# Codebase Design

Design **deep modules**: a lot of behaviour behind a small interface, placed at a
clean seam, testable through that interface. Use this vocabulary and these principles
wherever code is being designed or restructured. Deep modules buy three things —
**leverage** for callers, **locality** for maintainers, **testability** for everyone.

The terms below are deliberately **scale-agnostic** and **language-neutral**: they
apply to a function, a class, a package, or a tier-spanning slice, in any language.
The code examples are illustrative pseudocode, not any one language's syntax.

## Glossary

Use these terms exactly — consistent language is the whole point. Don't substitute
"component", "service", "API", or "boundary".

**Module** — anything with an interface and an implementation. Deliberately
scale-agnostic: a function, class, package, or tier-spanning slice. _Avoid_: unit,
component, service.

**Interface** — everything a caller must know to use the module correctly: the type
signature, but also invariants, ordering constraints, error modes, required
configuration, and performance characteristics. _Avoid_: API, signature (too narrow —
they name only the type-level surface).

**Implementation** — what is inside a module: its body of code. Distinct from
**adapter** — a thing can be a small adapter over a large implementation (a real
database repository) or a large adapter over a small implementation (an in-memory
fake). Reach for "adapter" when the seam is the topic, "implementation" otherwise.

**Depth** — leverage at the interface: how much behaviour a caller (or test) can
exercise per unit of interface they must learn. A module is **deep** when a large
amount of behaviour sits behind a small interface, **shallow** when the interface is
nearly as complex as the implementation.

**Seam** _(Michael Feathers)_ — a place where you can alter behaviour without editing
in that place; the *location* at which a module's interface lives. Where to put the
seam is its own design decision, distinct from what goes behind it. _Avoid_: boundary
(overloaded with DDD's bounded context).

**Adapter** — a concrete thing that satisfies an interface at a seam. Describes a
*role* (which slot it fills), not substance (what is inside).

**Leverage** — what callers get from depth: more capability per unit of interface they
learn. One implementation pays back across N call sites and M tests.

**Locality** — what maintainers get from depth: change, bugs, knowledge, and
verification concentrate in one place instead of spreading across callers. Fix once,
fixed everywhere.

## Deep vs shallow

A **deep module** is a small interface over a large implementation:

```
┌─────────────────────┐
│   small interface   │  ← few entry points, simple parameters
├─────────────────────┤
│                     │
│        deep         │  ← complex behaviour, hidden
│   implementation    │
│                     │
└─────────────────────┘
```

A **shallow module** is a large interface over a thin implementation — avoid it:

```
┌─────────────────────────────────┐
│         large interface         │  ← many entry points, complex params
├─────────────────────────────────┤
│       thin implementation       │  ← mostly pass-through
└─────────────────────────────────┘
```

When designing an interface, ask:

- Can I reduce the number of entry points (methods, functions, options)?
- Can I simplify the parameters?
- Can I hide more complexity inside?

## Principles

- **Depth is a property of the interface, not the implementation.** A deep module can
  be internally composed of small, swappable parts — they just are not part of the
  interface. A module can have **internal seams** (private to its implementation, used
  by its own tests) as well as the **external seam** at its interface.
- **The deletion test.** Imagine deleting the module. If complexity vanishes, it was a
  pass-through and earned nothing. If the same complexity reappears, scattered across N
  callers, the module was earning its keep.
- **The interface is the test surface.** Callers and tests cross the same seam. If you
  find yourself wanting to test *past* the interface — reaching into internals — the
  module is probably the wrong shape.
- **One adapter means a hypothetical seam; two adapters mean a real one.** Don't
  introduce a seam (a port, an injection point) unless something actually varies across
  it. A single-adapter seam is just indirection.

## Designing for testability

Good interfaces make testing natural. Three habits, in language-neutral pseudocode:

1. **Accept dependencies, don't create them.** A module that receives its collaborators
   can be tested with substitutes; one that constructs them internally cannot.

   ```
   # Testable — the gateway is passed in; a test can supply a fake
   process_order(order, payment_gateway)

   # Hard to test — the gateway is created inside; a test cannot reach it
   process_order(order):
       gateway = new PaymentGateway()
       ...
   ```

2. **Return results, don't mutate in place.** A function that returns a value is tested
   by inspecting the return; one that mutates shared state forces the test to
   reconstruct and inspect that state.

   ```
   # Testable — the discount is returned
   calculate_discount(cart) -> Discount

   # Hard to test — the cart is mutated as a side effect
   apply_discount(cart):
       cart.total = cart.total - discount
   ```

3. **Small surface area.** Fewer entry points mean fewer tests; fewer parameters mean
   simpler setup. Depth and testability pull in the same direction.

## Deepening a cluster

To deepen a cluster of shallow modules — merge them behind one interface — first
classify the cluster's dependencies. The category decides how the deepened module is
tested across its seam.

| Dependency category | What it is | Test strategy |
|---|---|---|
| **In-process** | Pure computation, in-memory state, no I/O | Always deepenable. Merge and test through the new interface directly — no adapter. |
| **Local-substitutable** | Has a local test stand-in (in-memory store/filesystem, embedded database) | Deepenable if the stand-in exists. The seam is **internal**; test with the stand-in running in the suite — no port at the external interface. |
| **Remote but owned** | Your own services across a network (internal APIs, microservices) | Define a **port** at the seam. The deep module owns the logic; the transport is an injected **adapter** — in-memory for tests, network for production (ports & adapters). |
| **True external** | Third-party services you don't control (payments, messaging) | Take the dependency as an injected port; tests supply a mock adapter. |

**Seam discipline.** Hold to "two adapters mean a real seam" here: don't define a port
unless at least two adapters are justified (typically production + test). And keep
**internal seams internal** — don't expose them through the interface just because the
module's own tests use them.

**Replace, don't layer.** When you deepen, the old unit tests on the now-absorbed
shallow modules become waste — delete them, don't stack new tests on top. Write fresh
tests at the deepened module's interface, asserting on observable outcomes through the
seam, not on internal state. Tests written this way survive internal refactors: if a
test has to change when the implementation changes, it was testing past the interface.

## Design it twice

Your first interface is rarely your best. Before committing to one, design the same
module's interface **at least two radically different ways** — for example, one
minimising the entry points, one maximising flexibility, one optimising the most common
caller. Then compare the candidates on **depth** (leverage per unit of interface),
**locality** (where change concentrates), and **seam placement** (what varies across
it, and whether two adapters justify it). Pick deliberately, or compose the strongest
elements into a hybrid. (After Ousterhout's "design it twice".)

**Running it as parallel sub-agents.** For a candidate substantial enough to warrant it,
spawn 3-4 sub-agents in parallel, each given the module's constraints, its dependency
categories (see the table above), and a single divergent design pressure: minimise the
interface (1-3 entry points, maximum leverage per one); maximise flexibility (many use
cases, extension points); optimise for the most common caller (make the default case
trivial); and, when a real ports-and-adapters seam is in play, design around it. Each
returns an interface, a usage example, what it hides behind the seam, its dependency
strategy, and its trade-offs. Present the candidates sequentially so they can be absorbed
one at a time, then compare and recommend as above — be opinionated, the point is a
strong read, not a menu (`mattpocock/skills` `codebase-design/DESIGN-IT-TWICE.md`).

## Relationships

- A **module** has exactly one **interface** — the surface it presents to callers and
  tests.
- **Depth** is a property of a **module**, measured against its **interface**.
- A **seam** is where a **module**'s **interface** lives.
- An **adapter** sits at a **seam** and satisfies the **interface**.
- **Depth** produces **leverage** for callers and **locality** for maintainers.

## Rejected framings

- **Depth as the ratio of implementation lines to interface lines** (Ousterhout's own
  metric): it rewards padding the implementation. Use depth-as-leverage instead —
  behaviour exercised per unit of interface learned.
- **"Interface" as the language's `interface` keyword, or a class's public method
  list.** Too narrow: the interface here includes every fact a caller must know —
  invariants, ordering, error modes, configuration, performance.
- **"Boundary".** Overloaded with DDD's bounded context. Say **seam** (a place to alter
  behaviour) or **interface** (what a caller must know).
