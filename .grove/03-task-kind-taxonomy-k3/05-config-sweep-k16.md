# config-sweep-k16

**Kind:** work

## Goal

Make the seventeen-kind set discoverable and migrate the live configuration onto
it, so the feature is usable by someone who did not sit through the grilling.

## Context

- `src/cli.rs` — `MODEL_ENV_HELP`, the `grove do --help` env section. It
  currently enumerates five model vars by hand and gives one worked example.
- `content/TASK-FORMAT.md` — "The five kinds" and their disciplines; each kind
  is marked HITL or AFK. This is the file a session actually reads.
- `CONTEXT.md` — the **Task kind** and **Per-kind model selection** entries, both
  of which say "five".
- `content/driving.md` — where the two patterns belong as *habits* (the review
  chain; the vendor pair and its adversarial combine), if the grammar is
  documented rather than enforced.
- `README.md` and `docs/` — same counts, same lists.
- `docs/specs/task-kind-taxonomy.md` from `taxonomy-spec-k12` is the authority;
  this leaf propagates it, and **cites** rather than restates.

## Done when

- Every user-facing surface describes the seventeen kinds and both routing
  mechanisms: `--help`, `TASK-FORMAT.md`, `CONTEXT.md`, `README.md`, `driving.md`.
- No surviving text claims the set has five members. Grep for `five` across
  `content/`, `docs/`, `src/`, `README.md` — several are prose, not lists, and
  are easy to miss.
- The live environment is migrated: `GROVE_CODEX_WORK_MODEL` and
  `GROVE_PI_WORK_MODEL` become their `IMPL` spellings, and the change is written
  down somewhere the user will find it (release notes / CHANGELOG), because
  **an unset model var means no flag at all** — a stale `WORK` var does not
  error, it silently hands the session to the harness's own default.
- `GROVE_CODEX_REVIEW_MODEL` is set. It is absent today while
  `GROVE_REVIEW_HARNESS` is configured, which is the same silent-default trap
  already live and independent of this whole node.
- The `--help` worked example matches the user's real configuration (claude
  leads, codex reviews, claude integrates) rather than the old trial's.

## Notes

The failure mode this leaf exists to prevent is **silent**, and it is worth
stating plainly: grove's no-fallback rule means a renamed env var does not fail
loudly, it just stops applying. Someone who renames `work` → `impl` and forgets
their shell profile gets sessions on the wrong model with nothing on stderr to
say so. Consider whether a *recognised-but-retired* var name should warn — that
is a cheap guard and the only place in the design where a legacy spelling could
be detected at all.

Do not fold the ADR reworks in here; they belong to `taxonomy-spec-k12` and
doing them twice produces two divergent accounts.
