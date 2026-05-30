# 020-sweep

**Kind:** work

## Goal

Remove every stale reference to the removed `grove update` verb from the glossary
and docs, and fold the `update.md` walkthrough into `install.md`. Pure doc edits.

## Context

`grove update` was removed in v4.0.0 (folded into idempotent `grove install`,
ADR-0008). See the root `BRIEF.md` for the full site inventory and the
fold-into-install decision.

## Done when

### Glossary — `CONTEXT.md`

- [ ] "Install scope": `grove install` or `grove update` → `grove install`
- [ ] "Path-scoped commit": `install`/`update` → `install`
- [ ] "grove-meta branch": `grove install` / `grove update` → `grove install`
- [ ] "cli/repo/worktree version": `grove install` / `grove update` → `grove install`
- [ ] "Lifecycle walkthrough": flow list `(install, update, start, multi-step, finish)`
      → `(install, start, multi-step, finish)`

### Workflows — `docs/workflows/`

- [ ] Fold the `update.md` worked example (refresh v2.0.0→v2.1.0, outcome line,
      ADR-bump nudge, --no-commit/--message) into `install.md` as a "Refreshing an
      existing install" section.
- [ ] `git rm docs/workflows/update.md`.
- [ ] `README.md`: drop item 2; reframe "The five flows" → four flows.
- [ ] `install.md`: fix the `See the refresh walkthrough (update.md)` self-link
      (now an in-page anchor, since the content lives here).

### Leave alone (do NOT touch)

- `CHANGELOG.md` — historical record.
- `README.md:24` (repo root) — already says "former `grove update` is removed".
- `docs/adr/0001`, `docs/adr/0008` — immutable ADRs.
- Commit-subject string `Update grove to v<ver>` — real current default subject.

## Notes

Verify at the end with `git grep -ni 'grove update'` — every surviving hit must be
in the leave-alone set (CHANGELOG / ADRs / README removal note / commit-subject).
