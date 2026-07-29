# distribution-k5

**Kind:** impl

## Goal

Register `using-codebase-memory` in the `linkuistics` plugin manifest, and prove
the skill actually reaches the three non-Claude harnesses through the existing
`install.sh`.

This is the plan's Task 3, unchanged. It stays a separate leaf because it edits a
different file and its verification has a side effect **outside the repo** — it
writes symlinks into `$HOME`.

## Context

- `docs/superpowers/plans/2026-07-29-using-codebase-memory-skill.md` § Task 3 —
  the exact manifest edit and the verification commands.
- `plugins/linkuistics/.claude-plugin/plugin.json` — the file to edit.
- `install.sh` — **read it, do not edit it.** It globs
  `"${skills_dir}"/*/`, so a new skill directory is picked up with no change.
  Confirmed at scoping time; this leaf verifies rather than assumes.

Runs last, after the review chain, so the symlinks resolve to final content.

## Done when

- `plugin.json`'s `description` mentions the graph-query capability and its
  `keywords` array contains `codebase-memory`, `knowledge-graph`, `code-search`.
- The JSON still parses:
  `jq -e '.name, .description, (.keywords | index("knowledge-graph"))'`.
- `./install.sh` has been run, and a symlink into
  `plugins/linkuistics/skills/using-codebase-memory` is confirmed in each harness
  directory that exists. A `skip` line for a harness whose home directory is
  absent is normal, not a failure.
- The skill reads correctly *through* one non-Claude symlink — i.e. the
  frontmatter is what shipped, resolved through the link, not read from the repo.
- Committed with `jj describe` / `jj new`.

## Notes

**No `version` goes into `plugin.json`.** Both plugins here deliberately use the
commit-SHA version — the cache key an update is decided on, which moves with
every commit. Adding an explicit semver *pins* delivery on a bump nothing in this
repo checks (`docs/adr/skills-monorepo.md`; glossary § Commit-SHA version). If
`claude plugin validate --strict` complains about the missing field, that is the
intended trade, not a defect to fix.

**Three harnesses, not two.** `install.sh` targets `~/.codex/skills`,
`~/.gemini/skills` **and** `~/.pi/agent/skills`. Both `plugins/CONTEXT.md` and
`CONTEXT-MAP.md` named only the first two; corrected in the `scope-k1` commit.
Pi is the harness this whole grove exists for — it refuses MCP by design — so
its link is the one that most matters.
