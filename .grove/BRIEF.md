# pruning — brief

## Goal

Give grove a representation for **abandoned work** — the "we went down this path
and chose not to take it" outcome that is normal in exploratory work — and a
durable home for the *why* that survives the grove which produced it.

Today the tree can say **done** and nothing else. Abandoning a leaf is a bare
`git rm`: it leaves no trace in `.grove/`, it lowers the permanent-key counter
(so the next `leaf-add` reuses a key the glossary forbids reusing), and the
"do not re-litigate this" note has nowhere to live except a `BRIEF.md` that is
deleted by the finish cycle.

Grove's own metaphor names the two outcomes: a `DONE` leaf is **harvested**; an
abandoned one is **pruned**.

## Done when

- Abandonment has an agreed representation in the tree (or an agreed, documented
  reason it should not have one).
- The durable rejection record — what was rejected, why, and what would reopen
  it — has a named home that outlives `.grove/`.
- The key-counter defect is closed: `leaf-add` can never re-issue a key.
- Whatever is decided is *shipped*: `content/` (methodology), CLI, and glossary
  agree, and GitHub issue #2 is closed by the change.

## Decomposition

Design settled by `plan-k1` (see its running log for D1–D7, and ADR *pruning* for
what binds). The rest is execution, ordered by dependency:

- `prune-verb-k2` — the CLI: the `ABANDONED` infix and `leaf-prune`. Code first, so
  the prose that follows describes something real.
- `methodology-k3` — `content/` (canonical): teach the loop to prune, and state the
  HITL guard and the durable-record rule.
- `decision-records-skill-k4` — upstream the **reopening trigger** into
  `linkuistics:decision-records`. It is a fact about good ADRs, not a grove-local habit.
- `review-k6` — fresh-context adversarial read of the whole change. Earned: this
  alters the tree's grammar, and it asserts a property no compiler checks (the key
  counter is monotonic).
- `release-k5` — cut the release, **verify the live binary's behaviour**, close #2.
  The methodology is inert until this runs — the binary embeds `content/`.

(`release-k5` sits at position 06: it was added before `review-k6` and shifted down
when the review leaf was inserted ahead of it. The key stayed put — that is the
scheme working.)

## Pointers

- **Charter**: `gh issue view 2 --repo Linkuistics/grove` — the gap, the
  key-reuse defect, and the (deliberately open) design space. The comment on it
  is the better framing; the body is the narrow symptom.
- **ADR** `docs/adr/task-tree-scheme.md` — where the permanent-key = `max + 1`
  rule and the `DONE` infix are defined.
- **ADR** `docs/adr/task-kind-taxonomy.md` — the precedent for "does a taxonomy
  earn its place?", the same question abandoned-vs-blocked-vs-deferred raises.
- **Glossary** (`CONTEXT.md`): *Permanent key*, *DONE infix*, *Leaf*,
  *Complete finish cycle*, *Spec*.
- **Worked example of the gap**: commit `5177ea4` — the session that abandoned
  `review-provider-spike-k12` / `review-provider-impl-k13` by hand, with no
  support from the methodology.

## Notes

**Binary lag — and a trap it sets.** The installed `grove` CLI is v9.1.0 (brew);
`main` is ~22 commits ahead (5-kind taxonomy, `docs/specs/`). Canonical methodology
is the repo's `content/`, **not** the older global skill dir (which still describes
two task kinds and PRDs).

The trap: v9.1.0 **gates on read** — `grove-llm kind` *errors* (exit 1) on any kind
outside `work`/`planning`, and the loop driver calls it every iteration to pick a
model. `main` fixes this (unrecognised kind → warn, treat as `work`), but it is
unreleased. **So every leaf in this grove must carry `**Kind:** work` or
`planning`** even where the canonical taxonomy would say otherwise — see
`review-k6`, which is a review by *discipline* while saying `work` on disk. Verified
by probe, not assumed.

**Sequencing wart** (issue #3): `leaf-insert` renumbers with `git mv`, which fails on
an untracked source — so a leaf added and then inserted-ahead-of in the *same*
session needs a `git add .grove/` first. Filed, not fixed here.
