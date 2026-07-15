# A fresh grove is scaffolded by `grove-llm root-init`, which creates the root brief *and* a first planning leaf

A brand-new grove has a working tree (user-provided — *user-owned-worktrees*) but no `.grove/` tree yet, and every step
of the loop assumes `.grove/` is already present (`grove-llm pick` on a treeless grove
errors "grove root not found"). **`grove-llm root-init [<slug>]`** (default slug
`plan`) resolves that: it creates `.grove/`, writes the root `BRIEF.md` stub, and lays
down a first **planning** leaf `01-<slug>-k1.md`. Working-tree change only, no commit;
it refuses to clobber an existing `.grove/`.

## A new verb, not an extension of `grove do`

The scaffold lives on `grove-llm` as a sibling of the tree-mutation verbs
(`leaf-add`, `leaf-insert`, `leaf-decompose`, `leaf-retire`) — all LLM-driven,
working-tree-only, printing the created path — rather than as a new-grove branch
inside `grove do`. Creating the root is the same *kind* of operation as growing the
tree, so it belongs in the same verb family. And `grove do` is deliberately kept
purely mechanical (see *do-is-sole-lifecycle-verb*); folding tree-authoring into it
would put content-generation logic on the lowest-level, highest-traffic lifecycle
path.

## Create the first leaf, not just the root brief

This is the load-bearing, surprising half of the decision. A root brief with **no
leaves** is empirically indistinguishable from a **finished** grove: `grove-llm pick`
skips every `BRIEF.md`, so a brief-only `.grove/` yields zero live leaves and prints
"no live leaves; this grove is done" — the exact signal that triggers the finish
cycle (see *in-session-finish-cycle*). Scaffolding that stopped at the root brief
would leave the next routine `pick` proposing to tear the newborn grove down.

Creating the first leaf in the same atomic verb makes a fresh grove distinguishable
from a retired one and drops it straight into the steady-state loop (`pick` →
planning task). An integration test asserts the invariant directly. The first leaf is
**planning** kind: the entry into a fresh grove is the grilling session that grows the
tree, not work.

- *Rejected:* root-brief-only plus hardening `pick` to distinguish new-from-done —
  that touches `pick`'s load-bearing finish-trigger semantics for no added benefit.

## Consequences

- The rewritten start prompt names `grove-llm root-init` as step 1 of a fresh grove.
- `root-init` reuses the same leaf-creation path as `leaf-add`, so the scaffolded
  leaf stays byte-identical to a hand-added one — no second template to drift.
