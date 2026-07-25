# plugin-versioning-k5

**Kind:** work

## Goal

Give each plugin an explicit version so plugin versions stop moving every time an
unrelated grove commit lands.

## Context

`plugins/linkuistics/` and `plugins/testanyware/` carry no `plugin.json`, so
Claude Code falls back to versioning a plugin by **repo HEAD SHA**. Observed
locally in `~/.claude/plugins/installed_plugins.json`:

```jsonc
"linkuistics@linkuistics": [{
  "installPath": ".../cache/linkuistics/linkuistics/e0ba6f40f6e8",
  "version": "e0ba6f40f6e8",
  "gitCommitSha": "e0ba6f40f6e88d77468534dc527b208015beb989"
}]
```

That was tolerable in a repo whose commits were nearly all skill changes. In this
repo nearly all commits are grove changes, so with `autoUpdate: true` every one
of them re-versions both plugins and triggers a re-install of content that did
not change.

This is a *consequence* of the merge, not a distribution redesign —
`docs/adr/skills-monorepo.md` scopes distribution changes out, and this leaf
stays inside that scope.

## Done when

- Both plugins declare an explicit version in a `plugin.json` under the plugin's
  `.claude-plugin/` directory. **Verify the exact filename, location and schema
  against current Claude Code documentation before writing it** — this is
  version-specific structure and the `driving.md` rule applies: fetch the
  official source, do not write it from memory, and cite the source in a comment
  or in the commit message.
- A grove-only commit no longer changes either plugin's reported version.
- A skills-only change still produces a version bump — verify by inspecting the
  installed record after an update, not by reasoning about it.
- A versioning policy is written down wherever `docs-reconciliation-k4` landed
  the CHANGELOG policy: who bumps these, and on what.

## Notes

- Check `~/.claude/plugins/marketplaces/claude-plugins-official/` for real
  examples — 273 plugins are catalogued there and many declare versions
  explicitly (`"version": "6.2.0"` for superpowers, `"1.0.0"` for several
  others), which makes it a good corpus for confirming the shape.
- If it turns out that a `plugin.json` version does *not* override the SHA
  fallback, say so plainly and stop rather than inventing a workaround — the
  churn is cosmetic, and a wrong fix here is worse than the noise.
