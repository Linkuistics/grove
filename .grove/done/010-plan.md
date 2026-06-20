# 010-plan

**Kind:** planning

## Goal

Design how grove should borrow two techniques from `addyosmani/agent-skills`
(MIT) — *without* importing the 24-skill library — and grow the tree into the
work leaves that implement the borrows:

1. **Source-driven citation discipline for work-tasks** — generalize the
   research-leaf citation rigor already in `driving.md` to framework/library
   decisions in *work* tasks (verify against the dependency manifest, cite
   official-doc sources, flag what couldn't be verified).
2. **Fresh-context adversarial-review habit** — operationalize the existing
   "ask for pushback / WDYT" nudges in `driving.md` into a concrete doubt pass
   (CLAIM → adversarial fresh-context review → reconcile) for hard-to-reverse /
   surprising / real-trade-off decisions.

## Context

- The comparison that motivated this grove (parent session): grove is a
  *meta-process* skill; agent-skills is a flat *lifecycle library*; ~80%
  complementary, not competing.
- Borrow sources: `skills/source-driven-development` and
  `skills/doubt-driven-development` (MIT, © Addy Osmani 2025).
- Precedent: grove already bundles `mattpocock/skills` (`grilling.md`,
  `*-FORMAT.md`) with a provenance header + `.claude/skills/grove/LICENSES/`.
- Constraints in play: #2 read-don't-run, #4 lazy/optional, #5 guides-not-gates,
  #7 one-page-spine. `driving.md` is the field guide, NOT the one-page spine —
  the natural home for habits.

## Done when

- Form/placement, scope, and provenance of each borrow are settled (recorded in
  the running log below).
- The tree is grown into the work leaf/leaves that make the edits.

## Notes

Out of scope (explicit non-goal): importing the agent-skills library wholesale,
or anything already covered by grove, the harness agents, or superpowers.

## Decisions (running log)

**D1 — Form & placement.** Both borrows are **paraphrased into grove's own
voice** as new sections in `driving.md` — not bundled verbatim (as `grilling.md`
was), not a standalone skill. Rationale: the parent-session conclusion was
"principles, not packages"; the addyosmani originals carry React/Django examples
and references to their own `agents/` roster, cross-model CLIs, and
`orchestration-patterns.md` that would dangle in grove. `grilling.md` was
bundled verbatim because grove uses it as-is; these two instead *extend* habits
`driving.md` already has. Provenance becomes a light "adapted from
`addyosmani/agent-skills` (MIT)" attribution + a `LICENSES/addyosmani.LICENSE`
file (the adaptation analogue of the verbatim-bundle header).

**D2 — `driving.md` remit widens.** `driving.md` covers *any* grove session
(planning + work), not just planning/grilling. Both borrows land as cross-kind
habits (citation fires on work-task framework code; the doubt pass fires before
any non-trivial decision). A one-line pointer from `TASK-FORMAT.md`'s `work`
kind points at the relevant `driving.md` habit (discoverability hook), keeping
the substantive guidance in one place. Additions kept tight — it's a field
guide, not a manual.

**D3 — Borrow #1 design (source-driven citation).** A **should-habit,
trigger-scoped** to framework/library code *whose correctness depends on the
version* (not version-invariant logic) — never a "must" (constraint 5). Framed
as the existing research-leaf citation discipline *extended to work tasks*, so
it reads as one continuous grove principle. Stack-agnostic manifest detection
(`Cargo.toml`/`package.json`/`pyproject.toml`/`go.mod`/…) with grove's own Rust
stack as the worked example (`Cargo.toml` → docs.rs). Source hierarchy: official
docs/changelogs over SO/blogs/training data; **flag-as-unverified** when no
authoritative source is found. Mechanism names what grove already has —
**Context7 MCP** (`resolve-library-id`→`query-docs`) + `WebFetch` — which keeps
it inside constraint 2 (read, don't run). Citation lands as an **inline code
comment with the source at the decision site** (durable, walk-away-able).

**D4 — Borrow #2 design (fresh-context adversarial review).** Framed as the
*fresh-context escalation* of the existing WDYT/pushback habit (a reviewer that
never saw your reasoning, vs same-context self-doubt). **Trigger: both** — the
ADR triad (hard-to-reverse / surprising / real trade-off) **and** "a correctness
property the compiler can't check (thread-safety, ordering, idempotence, an
invariant)"; one familiar trigger, two consequences. **Mechanism: a fresh-context
subagent reviewer** (the harness's existing `code-reviewer`/`Explore` agents) with
adversarial framing, handed artifact + contract but **not** the author's
conclusion (anti-sycophancy). Does not violate constraint 2 (read-don't-run governs
*bootstrap*, not mid-session tool use). Keep the skeleton CLAIM → EXTRACT → review
→ RECONCILE (contract-misread / actionable / valid-trade-off / noise) → STOP (≤3
cycles, then escalate). **Cross-model CLI orchestration dropped entirely** — too
heavy/fragile for grove. Slots in just before the loop's Commit step.

**D5 — Decomposition, ADR, version, session boundary.** Grow the tree into a
**single work leaf** `020-driving-edits` (both `driving.md` sections + the
`TASK-FORMAT.md` pointer + the `LICENSES/addyosmani-agent-skills.LICENSE` +
attribution + re-materialize) in one focused commit — grove favors lazy
decomposition and the edits share the provenance setup. **No ADR** (reversible
doc edits, not hard-to-reverse architecture). **No methodology VERSION bump**
(release-time action; the borrow lands in `content/`). Canonical source is
`content/`; `.claude/skills/grove/` is the materialized copy (`src/extract.rs`
bundles `content/*`), so the work edits `content/` then re-materializes. Session:
complete the planning task here (commit + retire `010-plan`), then continue into
`020` in this same session as a **separate** commit.
