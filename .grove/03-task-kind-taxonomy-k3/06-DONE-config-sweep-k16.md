# config-sweep-k16

**Kind:** work

## Goal

Make the seventeen-kind set discoverable and migrate the live configuration onto
it, so the feature is usable by someone who did not sit through the grilling.

## Context

- `docs/specs/task-kind-taxonomy.md` from `taxonomy-spec-k12` is the authority;
  this leaf propagates it, and **cites** rather than restates.
- `src/cli.rs` — `MODEL_ENV_HELP`, the `grove do --help` env section. It
  currently enumerates five model vars by hand and gives one worked example.
- `content/TASK-FORMAT.md` — "The five kinds" and their disciplines, each marked
  HITL or AFK. This is the file a session actually reads.
- `content/SKILL.md` — its **Execute** step still opens a *planning* task with a
  grilling session. That sentence moves to `requirements`; `planning` keeps the
  methodological force (sole Execute branch, only kind that grows the tree) but
  no longer interrogates. Also the Decompose step's kind-inheritance sentence.
- `content/driving.md` — where the two patterns belong as *habits* (the review
  chain; the vendor pair and its adversarial combine), since the grammar is
  documented rather than enforced.
- `README.md` — `## Configuration`: the five-row model-var table, plus two bullets
  ("Unset ⇒ inherit your own default", and the `/model`-persistence asymmetry)
  that state the **inverted** rule and must be rewritten, not merely extended.
- `docs/grove.md` — one long sentence enumerating the five kinds.
- `CONTEXT.md` — **already done** by `taxonomy-spec-k12` (the entries are now
  **Task kind**, **Review chain / vendor pair**, **HITL / AFK**, **Kind
  routing**). Verify rather than rewrite.

## Done when

- Every user-facing surface describes the seventeen kinds and both routing
  mechanisms: `--help`, `TASK-FORMAT.md`, `SKILL.md`, `driving.md`, `README.md`,
  `docs/grove.md`.
- No surviving text claims the set has five members, that `work` is a live kind,
  or that an unset model var is benign. Grep `content/`, `docs/`, `src/`,
  `README.md` for `five`, `\bwork\b` as a kind label, and "inherit"— several are
  prose, not lists, and are easy to miss.
- The live environment is migrated: `GROVE_CODEX_WORK_MODEL` and
  `GROVE_PI_WORK_MODEL` become their `IMPL` spellings, and `GROVE_CODEX_REVIEW_MODEL`
  is set (absent today while `GROVE_REVIEW_HARNESS` is configured).
- The `--help` worked example matches the user's real configuration — claude
  leads, codex reviews, claude integrates — rather than the old trial's, and shows
  a **family** var doing the work of five.

## Notes

**The silent-failure premise this leaf was written under is gone.**
`required-model-vars-k18` inverts the no-fallback rule: a kind that resolves no
model var now *errors*. So a stale `GROVE_WORK_MODEL` cannot silently hand a
session to the harness's own default — the launch fails and names the var. The
env migration below is therefore a convenience, not a guard, and the
"recognised-but-retired var should warn" idea this leaf used to carry is moot —
do not implement it.

That also reorders the risk: the migration cannot be forgotten silently, but it
*will* stop the loop the first time it is reached. Do it in the same session as
the doc sweep.

Do not fold the ADR reworks in here; they belong to `taxonomy-spec-k12` and doing
them twice produces two divergent accounts.
