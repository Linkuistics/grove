# 9-[21]-shed-inbox-and-install-machinery

**Kind:** work

## Goal

Delete the remaining machinery (ADR-0031): the **inbox / `grove-meta` branch**
subsystem and the **install/materialise-into-harness** machinery (incl. the
cli/repo/worktree `VERSION.md` drift model), now replaced by the global-skill +
Homebrew distribution (070).

## Context

Read **ADR-0031** (machinery shed) and **070** (the replacement distribution)
first. The inbox/`grove-meta` subsystem is ADR-0002–0006 + ADR-0012 and the
`grove-llm inbox-*` / `grove meta` verbs; the install/materialise machinery is
`grove install` + the `VERSION.md` stamping/drift (ADR-0001/0007/0008 + the
`cli/repo/worktree version` glossary). Both are coordination/distribution that
"less in grove" + the global-skill model make redundant.

## Done when

- The inbox / `grove-meta` subsystem is removed: the `grove-llm inbox-add/drain/
  remove`, `grove inbox show`, `grove meta *` verbs, the branch/worktree handling,
  and the bootstrap **Drain** step (the loop no longer drains an inbox).
- The `grove install` / materialise machinery and `VERSION.md` stamping are
  removed (replaced by 070's binary-provisions-global-skill).
- The superseded ADRs (0001–0008, 0012, and inbox-related) are marked
  **Superseded**, pointing at ADR-0031 + 070.
- `CONTEXT.md` glossary entries for the removed terms (Inbox, Seed, Drain,
  grove-meta branch, the version-drift trio, install-scope, etc.) are removed; the
  glossary stays a truthful map of what exists.
- The SKILL.md loop drops the Drain bootstrap step and the inbox/capture machinery
  (coordinate with 070's bundled methodology — the *methodology* stays; the
  *inbox machinery* goes).
- Build green.

## Notes

- Sequence **after** 070 (the replacement distribution must exist first) and after
  040 (new loop proven).
- Large deletion across CLI + skill + glossary — may decompose.
- The loop's cross-grove coordination need (if any survives) should be re-examined
  before deleting: confirm the new model genuinely has no use for it, rather than
  assuming. If a real need remains, raise it rather than deleting blind.
