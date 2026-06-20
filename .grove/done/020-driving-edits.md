# 020-driving-edits

**Kind:** work

## Goal

Land the two agreed `addyosmani/agent-skills` borrows into grove's canonical
methodology source (`content/`), then re-materialize. One focused commit.

## Context

Design settled by `010-plan` grilling (decisions D1–D5; now in `done/`, in git
history). Canonical source is **`content/`**; `.claude/skills/grove/` is the
materialized copy (`src/extract.rs` bundles `content/*` into the binary). Edit
`content/`, then re-materialize.

**Edit 1 — `content/driving.md`: two new sections, paraphrased into grove's voice
(not verbatim; no dangling addyosmani cross-refs to their `agents/`, cross-model
CLIs, or `orchestration-patterns.md`).** `driving.md`'s remit widens to cover any
session (planning + work), not just planning/grilling.

- *Source-driven citation discipline (work tasks).* A **should-habit** (not a
  must — constraint 5), trigger-scoped to framework/library code **whose
  correctness depends on the version** (not version-invariant logic). Framed as
  the existing research-leaf citation discipline *extended to work tasks*.
  Stack-agnostic manifest detection (`Cargo.toml`/`package.json`/`pyproject.toml`/
  `go.mod`/…), grove's own Rust stack as the worked example (`Cargo.toml` →
  docs.rs). Official docs/changelogs over Stack Overflow / blogs / training data;
  **flag-as-unverified** when no authoritative source exists. Mechanism names what
  grove already has — **Context7 MCP** (`resolve-library-id`→`query-docs`) +
  `WebFetch` — keeping it inside constraint 2 (read, don't run). Citation lands as
  an **inline code comment with the source at the decision site** (durable,
  walk-away-able).
- *Fresh-context adversarial review.* The **fresh-context escalation** of the
  existing WDYT/pushback habit (a reviewer that never saw your reasoning).
  **Trigger:** the ADR triad (hard-to-reverse / surprising / a real trade-off)
  **OR** a correctness property the compiler can't check (thread-safety, ordering,
  idempotence, an invariant) — one familiar trigger, two consequences.
  **Mechanism:** spawn a **fresh-context subagent reviewer** (harness
  `code-reviewer`/`Explore` agents), adversarial framing ("find what's wrong;
  assume the author is overconfident"), hand it **artifact + contract but NOT the
  author's conclusion** (anti-sycophancy). Skeleton: CLAIM → EXTRACT → review →
  RECONCILE (classify: contract-misread / actionable / valid-trade-off / noise) →
  STOP (≤3 cycles, then escalate). **No cross-model CLI machinery.** Slots in just
  before the loop's Commit step. Note the constraint-2 reasoning inline:
  read-don't-run governs *bootstrap*, not mid-session tool use.

**Edit 2 — `content/TASK-FORMAT.md`:** a **one-line pointer** from the `work`
kind ("produces code, docs, or tests") to the relevant `driving.md` habit — the
discoverability hook, keeping the substantive guidance in one place.

**Edit 3 — Provenance:** add `content/LICENSES/addyosmani-agent-skills.LICENSE`
(MIT, © 2025 Addy Osmani) + a light **"adapted from `addyosmani/agent-skills`
(MIT) — see LICENSES/…"** attribution near the new sections (the adaptation
analogue of `grilling.md`'s verbatim-bundle provenance header). Pin the upstream
commit hash in the attribution.

**Edit 4 — Re-materialize:** propagate `content/` → `.claude/skills/grove/` so the
repo's own install matches (both are tracked + currently identical). Use
`grove install` or a direct copy; confirm `.claude/` tracking before committing.

## Done when

- The two `driving.md` sections, the `TASK-FORMAT.md` pointer, and the LICENSES
  file + attribution have landed in `content/` and been re-materialized to
  `.claude/skills/grove/`.
- Additions are tight, in grove's voice, with no dangling addyosmani cross-refs.
- One focused commit.

## Notes

Out of scope: methodology VERSION bump (release-time), and importing any more of
the agent-skills library. Upstream borrow sources:
`skills/source-driven-development` and `skills/doubt-driven-development`.
