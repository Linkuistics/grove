# A fresh grove is scaffolded by a new `grove-llm root-init` verb that creates the root brief *and* a first planning leaf

A brand-new grove (worktree + branch exist, no `.grove/` tree yet) had no
documented bootstrap path: `start.md` pointed at a "start-a-new-grove flow" that
did not exist, every step of the loop assumed `.grove/` was already present, and
no verb could create the root — `grove-llm pick` on a fresh grove exits with
`Error: grove root not found`. We add **`grove-llm root-init [<slug>]`** (default
slug `plan`), which creates `.grove/`, writes the root `BRIEF.md` stub, and lays
down a first planning leaf `010-<slug>.md`. Working-tree change only, no commit;
refuses to clobber an existing `.grove/`.

## Status
accepted

## Decision 1 — a new verb, not an extension of `grove do`
The scaffold lives on `grove-llm` as a sibling of the existing tree-mutation
verbs (`leaf-add`, `leaf-insert`, `leaf-decompose`, `leaf-retire`) — all
LLM-driven, working-tree-only, no commit, print the created path — rather than
as a new-grove branch inside `grove do`.

- **Symmetry.** Creating the root is the same *kind* of operation as growing the
  tree; it belongs in the same verb family, invoked the same way (the rewritten
  `start.md` names `root-init` as step 1).
- **Blast radius.** ADR-0009 made `grove do` the sole lifecycle entry verb and
  deliberately kept it purely mechanical. Folding tree-authoring into `do` would
  put content-generation logic on the lowest-level, highest-traffic lifecycle
  path. A separate verb keeps `do` mechanical.
- *Rejected:* extend `grove do`'s new-grove path; do both (belt-and-suspenders).

## Decision 2 — create the first leaf, not just the root brief
`root-init` creates `.grove/BRIEF.md` **and** `.grove/010-<slug>.md`, not the
brief alone. This is the load-bearing half of the decision and the reason it is
worth an ADR: it is *surprising* (why does a newborn grove auto-carry a planning
leaf?) and the alternative is *actively dangerous*.

A root brief with **no leaves** is empirically indistinguishable from a
**finished** grove: `grove-llm pick` skips every `BRIEF.md`, so a brief-only
`.grove/` yields zero live leaves and prints `no live leaves; this grove is
done` — the exact signal that triggers the Complete finish cycle (delete
`.grove/`, merge, tear down the worktree, see ADR-0010). Scaffolding that stopped
at the root brief would leave the next routine `pick` proposing to tear the
newborn grove down. Creating the first leaf in the same atomic verb makes a fresh
grove distinguishable from a retired one, and drops it straight into the
steady-state loop (`pick` → planning task). An integration test asserts this
invariant directly (`after_root_init_pick_returns_the_new_leaf_not_done`).

The first leaf is **planning** kind: the entry into a fresh grove is the grilling
session that grows the tree, not work.

- *Rejected:* root brief only + harden `pick` to distinguish new-vs-done (touches
  `pick`'s load-bearing Finish-trigger semantics for no added benefit); both.

## Why this is recorded
The start contract binds every future new grove (hard to reverse); the
auto-planning-leaf is surprising without this context; and real alternatives
(extend `do`; root-brief-only) were rejected for specific reasons. All three
ADR tests pass. Evidence for the failure modes is the `grove-startup-confuses-
the-LLM` grove's root `BRIEF.md` (primary evidence items 1–4), captured live in
the session that surfaced the confusion.

## Consequences
- `start.md` is rewritten to name `grove-llm root-init` as the first step of a
  fresh grove (leaf 020).
- Hardening `pick` to distinguish new-from-done, and extending `grove do`, are
  explicitly out of scope; if a strong case to harden `pick` surfaces it is a
  follow-up leaf, not a widening of this contract.
- `root-init` reuses `leaf::add(.., Kind::Planning)` for the first leaf, so the
  scaffolded leaf stays byte-identical to a hand-added one — no second template
  to drift.
