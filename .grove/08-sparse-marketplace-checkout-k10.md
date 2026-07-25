# sparse-marketplace-checkout-k10

**Kind:** work

## Goal

Decide whether the `linkuistics` marketplace should be checked out sparsely —
`.claude-plugin` and `plugins/` only — instead of cloning grove's whole tree, and
either adopt it everywhere it is documented or decline it on the record.

## Context

Surfaced by `cutover-k6` while verifying the re-point, not planned. Once the
marketplace pointed at `Linkuistics/grove`, the clone at
`~/.claude/plugins/marketplaces/linkuistics/` came to **2.7M, of which `plugins/`
is 172K** — the other ~2.5M is grove's `src/`, `tests/`, `docs/`, `content/` and
Cargo files, none of which a marketplace consumer has any use for. That is the
monorepo's first visible cost to a *consumer* rather than to us.

Claude Code ships the cure and documents it for exactly this shape:

```
claude plugin marketplace add --sparse <paths...>
  Limit checkout to specific directories via git sparse-checkout (for
  monorepos). Example: --sparse .claude-plugin plugins
```

The vendor's own example is this repo's layout verbatim.

**Why this is worth more than the 2.5M suggests.** `docs/adr/skills-monorepo.md`
*accepts* that every unrelated grove commit re-versions both plugins and
re-installs content that did not change — churn was chosen over silent staleness,
deliberately. With `autoUpdate: true` and grove's commit rate, that accepted churn
is a repeatedly refreshed full-tree clone. Sparse checkout would make the same
churn roughly 16× cheaper without touching the decision that produces it. The
trade the ADR records is unaffected either way — **do not reopen the version pin
here** (`version-bump-guard-k9` settled it; a `version` in either manifest
reinstates the staleness the ADR rejects).

## Done when

One of two outcomes, both explicit:

**Adopted** — sparse checkout is in use and every published instruction agrees; or

**Declined** — the leaf is pruned (HITL) with the reason recorded in the commit
message, and in `docs/adr/skills-monorepo.md` only if it clears the when-to-write
bar.

Three questions decide which, in this order — the first is a gate:

1. **Does the `/plugin marketplace add` slash command accept `--sparse`, or is it
   CLI-only?** If it is CLI-only, this can never be a documented instruction for a
   consumer, which caps the whole idea at "an optimisation on this one machine" —
   and that is probably a decline. Check before spending effort on 2 and 3.
2. **Does sparseness survive a marketplace auto-update?** A refresh that re-clones
   fully would undo it on the next update and make the local win illusory. The
   `add` run in `cutover-k6` logged *"Cleaning up old marketplace cache…"*, which
   hints at re-clone rather than fetch — unverified, and worth confirming against
   observed behaviour rather than the log line alone.
3. **If it is documentable, what has to change with it?** Three published texts
   currently say plain `/plugin marketplace add Linkuistics/grove`:
   `plugins/README.md`, the migration bullet at the head of `CHANGELOG.md`, and
   **the pointer README on `Linkuistics/skills`, which is now archived** — that
   one costs unarchive → edit → re-archive. Changing the documented command is no
   longer free, so adopt it in all three or in none.

## Notes

- **The stakes are small and a fast decline is a good outcome.** 2.5M of dead
  clone is waste, not breakage; nothing is failing. Do not let the investigation
  outgrow the prize — if question 1 answers "CLI-only", say so and stop.
- Whatever is decided, do not re-run `/plugin marketplace remove` casually:
  removing the marketplace **uninstalls both plugins** (observed in `cutover-k6`)
  and drops `autoUpdate` from the new record, so a remove/add cycle needs a
  reinstall of both plus `autoUpdate` restored. Back up
  `~/.claude/plugins/{known_marketplaces,installed_plugins}.json` first.
- After this leaf the root node has no live leaves left, which puts the grove into
  the **complete finish cycle** — see the tail of `07-DONE-cutover-k6.md`, which
  also carries the promote-step note the root brief flags.
