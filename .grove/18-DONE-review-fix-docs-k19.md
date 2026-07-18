# review-fix-docs-k19

**Kind:** work

## Goal
Correct the docs that are now actively wrong — each would misdirect a user
following the migration runbook, including the author a month from now.

## Context
Findings D1–D4 in `.grove/14-DONE-branch-review-k14.md`.

- **D1** `CHANGELOG.md` and `src/cli.rs:26` both say codex profiles are
  defined in `~/.codex/config.toml`. Verified false on disk: task 10 created
  `~/.codex/sol-high.config.toml` and `~/.codex/sol-xhigh.config.toml`, and
  `config.toml` holds no profile table. The spec has it right — this codex
  build layers `$CODEX_HOME/<name>.config.toml`. Following either doc puts the
  profile where codex will not look.
- **D2** `README.md:29`, `docs/grove.md:73`, `docs/workflows/start.md:101`
  still state the pre-change policy ("In single-harness repos the stamp is not
  written"), now false for the explicit path. `start.md` is what runbook step 6
  sends users to.
- **D3** `docs/adr/self-driving-loop.md:15` and `docs/workflows/finish.md:129`
  still name `GROVE_CLAUDE_PID` as the mechanism. The ADR is the repo's
  current-state record, so this is not a changelog matter. Leave
  `docs/research/cross-family-review-providers.md` alone — a dated research
  artifact.
- **D4** `--harness` has no help text on either `StartArgs` (src/cli.rs:77-78)
  or `RetireArgs` (`:95-96`); `--help` renders it blank. It should name the
  valid values (from `known_names()`) and say that it writes a permanent
  binding.

## Done when
- D1–D4 corrected; `grove do --help` shows a useful `--harness` description.
- `grep -rn "GROVE_CLAUDE_PID" docs/ README.md content/` returns only the
  research artifact and any deliberate one-release-fallback mentions.
- No doc still claims the stamp is multi-harness-only.
- `cargo test`, `fmt`, `clippy` clean. One focused commit.

## Notes
- The plan's own verification step for the PID rename grepped
  `src/ content/ tests/` — `docs/` was a structural blind spot, which is why
  D3 survived. Widen the grep in the plan file too, in this same commit, so
  plan and repo don't drift (the root brief's standing rule).
- Per `linkuistics:decision-records`, edit the ADR **in place** — do not append
  a superseding record.
