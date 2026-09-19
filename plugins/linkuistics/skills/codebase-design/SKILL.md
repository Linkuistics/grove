---
name: codebase-design
description: Shared vocabulary and checkable principles for deep modules, structural simplicity, and testable seams — Ousterhout, Hickey, and Feathers, language-neutral. Use when designing or reviewing a module's interface, restructuring coupled code or state, deciding where a seam goes, judging whether an abstraction earns its place, or making code more testable.
harnesses: [any]
---

# Codebase Design

Design modules whose interfaces reduce caller knowledge and whose internals
separate concerns that can vary independently. Place test seams where behaviour
actually needs substitution. Preserve required behaviour and account for the
cost of migration as well as the resulting design.

These terms apply to a function, class, package, or tier-spanning slice. Use the
relevant checks for a local change; a short fix does not require an architecture
exercise. A review produces findings unless edits are requested.

## Three distinct contributions

| Source | Design question | Evidence |
|---|---|---|
| **Ousterhout** | How much must a caller or maintainer understand? | Interface obligations, information leakage, change amplification, obscurity. |
| **Hickey** | Which concerns must be understood or changed together? | Entanglement of values, state, policy, timing, and implementation choices. |
| **Feathers** | Where can behaviour be replaced without editing the code at that point? | A seam and the enabling point that selects the behaviour. |

A convenient interface can conceal entangled internals. Independent internals
can impose an awkward protocol on callers. Evaluate both; tests and substitution
points do not by themselves establish either kind of simplicity.

The workflow and decision rules here are **Linkuistics synthesis**, not a method
jointly endorsed by these authors. Source keys `O1–O5`, `H1–H2`, and `F1–F2`
are mapped in [the provenance record](../../PROVENANCE.md#codebase-design).

## Glossary

Use these terms consistently, while preserving an existing project's domain
vocabulary.

**Module** — anything with an interface and an implementation. Use this term
when discussing modularity, whatever the language calls the concrete construct.

**Interface** — everything a caller must know to use a module correctly:
signatures, invariants, ordering, failures, required configuration, ownership,
and relevant performance or consistency guarantees. A signature alone is not
the interface. **Implementation** — what fulfils that contract. (`O2`)

**Depth** — useful behaviour relative to interface complexity. A deep module
requires comparatively little caller knowledge; a shallow one exposes nearly
as much complexity as it hides. Assess obligations, not a line-count ratio.
**Leverage** is the benefit to callers; **locality** is the concentration of
change, bugs, and verification for maintainers. (`O2`; latter terms are synthesis)

**Simplicity** — independence of concerns. **Entanglement** — concerns joined so
that reasoning about one requires the others. **Ease** — familiarity or
accessibility to a particular person. Record learning cost separately from
structural dependencies; neither unfamiliarity nor brevity establishes
complexity or simplicity. (`H1`)

**Value** — information that does not change. **Identity** — an entity whose
continuity is modelled across time. **State** — the value associated with an
identity at a particular time. State transitions still need coordination when
their inputs and outputs are immutable. (`H2`)

**Seam** — a point where behaviour can be substituted without editing the code
there. Its **enabling point** selects the behaviour, for example through an
argument or linkage choice. A module boundary is not automatically a seam.
**Adapter** — a concrete implementation satisfying an interface at a seam; a
role, not a measure of implementation size. (`F1–F2`; adapter usage is synthesis)

## Principles

- **Hide decisions, expose guarantees.** Give representation and algorithm
  knowledge an owner. Keep required failure, lifetime, ordering, and consistency
  semantics in the interface. Removing their documentation removes no obligation.
- **The deletion test.** Imagine removing a module while preserving behaviour.
  If complexity disappears, inspect whether it was redundant. If the same work
  spreads across callers, it earns its keep. Include compatibility and policy
  guarantees in that accounting; a short adapter may carry a real contract.
- **Separate independent concerns; combine shared knowledge.** Identify the
  decisions or invariants that justify grouping code. Internal composition can
  support a deep external interface. File, method, and service counts decide
  neither depth nor simplicity.
- **Separate calculation from changing context.** Prefer explicit values for
  computation. Give necessary state a clear owner and transition contract;
  expose hidden I/O, ambient context, and time dependencies during analysis.
- **Pull common complexity downward; keep special policy above it.** Solve
  recurring caller problems in the responsible module. Generalise operations
  when that simplifies current uses; defer speculative variation points.
- **Explain what the code cannot say.** Use consistent names and document
  semantics absent from signatures. A difficult interface explanation is a
  reason to reconsider the design, not to omit the explanation.

These rules apply `O1–O5` and `H1–H2`; the deletion test is a retained Linkuistics
heuristic. For worked applications, counterexamples, and longer tradeoffs, read
[Design examples](references/design-examples.md) when the choice remains unclear.

## Workflow and questions

1. **Establish the obligation.** State the requested behaviour, consumers,
   invariants, compatibility needs, resource limits, and deadline. Ask which
   unknown answers could change the recommendation; record other assumptions.
2. **Inspect a real path.** Follow one representative operation and failure
   through callers, implementation, and tests. Cite source locations for claims
   about existing code; mark proposal assumptions as such. Ask what must change
   together and what knowledge a caller must reconstruct.
3. **Assign ownership and write the interface.** Name each module's responsibility
   and hidden decisions. Sketch normal and failing use. Ask who owns state,
   whether ordering is inherent or API-imposed, and what remains implicit.
4. **Design it twice for consequential choices.** Compare materially different
   interfaces or ownership arrangements, including keeping the current design
   when reasonable. Try different priorities: the common caller, minimal
   obligations, actual variation, or a required test seam. Compare depth,
   entanglement, locality, seam placement, and migration/runtime costs. (`O2`)
5. **Challenge the preference.** Try a relevant failure, overlap, repeated call,
   or plausible next change. State a counterexample and evidence that would
   reverse the decision. Use a focused experiment when that evidence is missing.
6. **Make the smallest coherent change.** When implementation is requested,
   preserve the contract or name its authorised change; update affected callers
   and documentation together. Verify meaningful outcomes. Stop when the task
   is satisfied; give deferred work a concrete revisit condition.

Keep caller burden and internal entanglement as separate observations, even
when the same remedy improves both. A useful compact record is:

| Evidence | Caller obligation | Entangled concerns | Consequence |
|---|---|---|---|
| Code, contract, trace, or proposal fact | Knowledge or protocol required | Decisions that cannot vary independently | Failure or change impact |

## Designing for testability

- **Choose a seam for a concrete need.** Show the behaviour that needs replacing
  and its enabling point. Production and test adapters are real variation; two
  production implementations are not a prerequisite. A second hypothetical
  implementation is not a reason to add an extension system.
- **Accept dependencies at the responsible composition point.** An operation
  supplied with a gateway or clock can be exercised with controlled behaviour.
  A public caller need not assemble every internal dependency.
- **Prefer values for calculation results.** Tests can inspect a returned result
  without reconstructing shared mutation. Where mutation is required, test its
  ownership, transitions, and concurrent effects explicitly.
- **Test contracts at the appropriate interface.** Keep private seams private.
  Focused tests of a meaningful internal module can coexist with public contract
  tests. Fewer entry points may reduce setup; behaviours and state combinations
  still determine coverage needs.

### Deepening a cluster

First establish that the code shares a responsibility or invariant. Then use
the dependency category to choose verification; it does not decide whether to
merge the modules.

| Dependency category | Test strategy |
|---|---|
| **In-process**: computation or memory, no I/O | Exercise the contract directly; examine shared-state and ordering assumptions. |
| **Local-substitutable**: local stand-in exists | Use an internal seam where substitution is needed. Verify the real adapter's contract as well as the stand-in. |
| **Remote but owned**: own network services | Exercise consumer behaviour with a controlled adapter and transport/protocol behaviour with integration tests. |
| **True external**: third-party service | Control relevant outcomes through an adapter; verify external assumptions with suitable contract or integration checks. |

**Preserve useful coverage when replacing tests.** Move externally observable
cases to the deepened interface. Retain useful algorithmic, property, concurrency,
and regression coverage at meaningful internal interfaces. Delete redundant
implementation-coupled assertions after their behavioural protection is accounted
for; absorbing a module does not make every test of it waste.

## Review checklist

Mark applicable items supported, unresolved, or accepted tradeoff; attach evidence
to consequential judgments.

- Required behaviour and invariants hold, including material failure cases.
- Each module owns identifiable decisions; shared implementation knowledge is
  accounted for.
- Normal and failing callers can use the documented interface correctly.
- State owners, implicit I/O, ordering, and time dependencies were examined.
- Claimed simplification identifies dependencies removed and costs introduced.
- Seams have concrete substitution needs and enabling points.
- Plausible change impact and migration cost support the chosen scope.
- Alternatives and a reversal condition support consequential decisions.
- Existing useful coverage survives, and verification limits are explicit.

Report the recommendation, evidence, alternatives and costs, then verification
and remaining uncertainty. For a finding, include the location, consequence, and
smallest useful correction. Report when no material issue is supported.

## Composition with other skills

Use `model-led-development` when deciding whether a property warrants a model;
use `doubt-driven-development` for independent adversarial review under its own
trigger and coordination rules. Parallel design exploration, when warranted,
compares alternatives; it is not a substitute for that review. This skill adds
no required agent count, review gate, or formal-model step.

Use `decision-records` when a durable design decision needs an ADR. In a Grove
session, Grove owns session procedures, artifact formats, and review scheduling.
Fit the design evidence into its required artifact rather than adding a second
report with a competing format.
