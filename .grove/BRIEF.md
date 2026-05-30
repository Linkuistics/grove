# sweep-stale-grove-update-references — brief

## Goal

Sweep stale references to the **removed `grove update` verb** out of the docs and
glossary. `grove update` was removed in v4.0.0 (folded into the idempotent
`grove install`, ADR-0008), but several docs still cite it as a current verb.
Pure doc/glossary sweep — **no code changes**.

## Done when

- No doc or glossary entry presents `grove update` as a live verb.
- Historical record is preserved, not falsified: CHANGELOG entries, ADRs, and the
  default commit-subject string `Update grove to v<ver>` (real current behaviour)
  stay as-is.
- The `docs/workflows/update.md` walkthrough's fate is decided and enacted (see
  Decomposition).

## Decomposition

One work leaf — `020-sweep.md` — does the whole sweep in one focused commit:

1. **Glossary** (`CONTEXT.md`): drop `grove update` from the four entries that cite
   it as a live verb (Install scope, Path-scoped commit, grove-meta branch,
   cli/repo/worktree version); fix the "Lifecycle walkthrough" entry's flow list to
   the four remaining flows.
2. **Workflows**: **fold `update.md` into `install.md`** as a "Refreshing an
   existing install" section, then `git rm update.md`; drop README item 2 so it
   reads as four flows; fix `install.md`'s self-cross-link.

**Decision (planning leaf):** the `update.md` walkthrough is *folded into*
`install.md`, not renamed — refresh *is* re-running the idempotent install, so it
belongs as a section of the install flow rather than a co-equal flow. No ADR:
ADR-0008 already records the verb removal; this is downstream doc housekeeping.

## Pointers

- ADR-0008 — `grove install` is idempotent; `grove update` removed.
- ADR-0001 — install/update create commits (historical; the update half superseded
  by 0008). **Immutable — supersede, never rewrite.**

## Notes

### Sites that cite `grove update` as a current verb (fix these)

- `CONTEXT.md` "Install scope" — "`grove install` or `grove update` invocation"
- `CONTEXT.md` "Path-scoped commit" — "grove's `install`/`update` use"
- `CONTEXT.md` "grove-meta branch" — "during `grove install` / `grove update`"
- `CONTEXT.md` "cli/repo/worktree version" — "written by `grove install` / `grove update`"
- `CONTEXT.md` "Lifecycle walkthrough" — flow list "(install, update, start, multi-step, finish)"
- `docs/workflows/README.md` — "The five flows", item 2 framing `update.md` as a co-equal flow
- `docs/workflows/update.md` — whole walkthrough named after the removed verb
- `docs/workflows/install.md:95` — cross-links "the refresh walkthrough (update.md)"

### Sites that correctly describe the REMOVAL (leave alone)

- `CHANGELOG.md` (all `grove update` mentions) — historical record
- `README.md:24` — already says "The former `grove update` is removed"
- `docs/adr/0001`, `docs/adr/0008` — immutable ADRs
- Commit-subject string `Update grove to v<ver>` — real current default subject, not the verb
