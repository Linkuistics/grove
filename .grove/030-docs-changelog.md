# 030-docs-changelog

**Kind:** work

## Goal

Land the user-facing documentation for the config-only TUI (ADR-0027): the
CHANGELOG entry, README/docs touches, and any glossary follow-ups not already
captured inline during `010-plan`.

## Context

ADR-0027 records the decision; `CONTEXT.md`'s **Fleet** entry was already
updated inline during the `010-plan` grilling (cwd removal + singleton session).
This leaf covers the remaining human-facing surfaces.

Likely touch points (verify before editing):
- `CHANGELOG.md` — an `## [Unreleased]` entry: "`grove tui` is now driven
  entirely by `fleet.toml` + `--repo`; the cwd git-repo requirement is removed
  (runs from any directory); the fleet TUI is a singleton `grove-fleet` session"
  (ADR-0027). Match the existing changelog voice/format.
- `README.md` — any `grove tui` description that implies a cwd git repo, or
  documents `--local`-era behaviour; document `fleet.toml` + `--repo .` for the
  "current repo" case and the empty-fleet behaviour.
- `docs/grove.md` / lifecycle walkthroughs — only if they describe `grove tui`
  needing a repo.
- Re-check `CONTEXT.md` reads coherently after the inline Fleet edit.

## Done when

- `CHANGELOG.md` has an accurate unreleased entry citing ADR-0027.
- No doc tells the user `grove tui` needs to be run inside a git repo.
- `grove tui --repo .` (the deliberate "current repo" gesture) is documented
  where the old cwd convenience was implied.

## Notes

Keep it tight — most of the durable rationale lives in ADR-0027 and the
glossary; docs should point, not duplicate (avoid the decision-summary
anti-pattern).
