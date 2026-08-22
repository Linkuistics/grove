# plan-k1

## Goal

Establish, in the human's own words, what should be built for Linkuistics/grove
issue #13 — extracting grove's tree-on-disk facilities as a reusable library.

## Done when

- The workstream's goal, scope and constraints are recorded in the root brief.
- The interdependent open questions are settled with the human.
- The first step is cut as a leaf.

## Decisions (running log)

**The grove tree is warranted.** Issue #13 is session-to-session fog, not a
single sitting: a library extraction, a formal model, a CLI, and an experimental
agenda about deconstructing grove further. It cleared the no-fog early exit
easily.

**Full grilling was warranted.** Six interdependent questions, well past the
three-question threshold: whether grove reruns on the library, where the
genericity lives, how many crates, how much substrate, what the model is for, and
where the tests sit. Each answer constrained the next.

**grove flips in a later increment, not this one.** Rejected: flipping grove in
the same increment (validates the abstraction against a demanding consumer, but
couples two unrelated failure modes), and leaving grove permanently untouched
(guarantees drift with nothing to reconcile it). The chosen shape accepts a
bounded drift window in exchange for debuggability, and it is the reason the
model has to be load-bearing — phase 1 has no consumer to constrain it.

**Genericity lives in one trait's associated types, and the entry name is a type
wrapping a string.** Rejected: a declarative schema file read at runtime, which I
initially recommended on the grounds that a compile-time domain could not yield a
domain-independent CLI. That objection was wrong. Issue #13's "exposable as CLI"
means the library ships a *parameterised CLI implementation*, so a domain
instantiates it and `grove-llm` becomes a thin binary over it. Nothing requires
runtime domain selection, so nothing forces `dyn`, and associated types stay
free. Also rejected: domain callbacks for reserved names — the human's correction
was that reserved-ness is a variant of the name type, not a hook the library
calls back on.

The one real friction to expect: once `D: Domain` propagates, `derive` on any
type generic over `D` synthesises spurious `D: Clone`-style bounds. The standard
mitigation is a ZST marker never stored, plus hand-written impls where needed.

**One crate now.** I argued for a stack of separately-modellable crates, on
evidence from grove's own `lib.rs` — its record of a battle with in-crate seams,
and its technique of copying the source to a scratch crate and reading the
compiler's reachability warnings, because a crate boundary is the only seam Rust
enforces or measures. The human overrode this for increment size, explicitly
deferring refinement to a later workstream. The mitigation recorded in the brief
— algebra free of `std::fs`, enforced by a test — is the cheap substitute.

**The model is authoritative over a working implementation.** Rejected:
model-as-verification-afterwards, model-uncoupled-for-process-learning, and a
tripwire coupling. This is the strongest form of the experiment and it gives up
the working-implementation anchor deliberately. The consequence worth carrying
forward: every model/implementation disagreement is either a modelling lesson or
a latent grove bug that years of use never surfaced, so those disagreements are
the workstream's most valuable output and must be recorded rather than quietly
fixed in passing.

**Testing is regression cover, not assurance.** The human's reason for confidence
is that the implementation already works, so the ~130 adaptable tests exist to
stop a move from breaking something. They are CLI-contract tests, which is the
right shape for exactly that job — they survive an internal restructure that
unit tests would fight — even though it is the seam furthest from the model.

**No ADR.** Applied the three-part test to each candidate decision. All fail the
*hard to reverse* limb, and the trait design is intent rather than a landed
decision until the design step settles it. It is an AND test, so none earns a
record. Re-apply when the design lands.

**No migration anywhere.** Out of the library's scope by the human's direction,
and unnecessary for the flip by construction: grove's grammar moves into its
domain impl rather than changing, so on-disk names are untouched.

## Notes

The human's closing direction shaped the first leaf rather than the decisions:
the architecture and design come first, as an *interactive* process, delivering
diagrams and user-facing documentation that stand alone — readable on their own
terms rather than as grove-internal notes cross-referenced by task handles and
decision records.

The crate is named `ordinal-fs-tree` (the human's name; my `ordinal-tree` missed
the filesystem part).
