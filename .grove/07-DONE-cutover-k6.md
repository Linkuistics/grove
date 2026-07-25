# cutover-k6

**Kind:** work

## Goal

Publish the monorepo as the marketplace source, archive `Linkuistics/skills` with
a pointer, announce the move, and verify the skills actually resolve from here.

## Context

Runs **last**, and only once the merge is pushed to `Linkuistics/grove`.

The decision and its rejected alternatives are in `docs/adr/skills-monorepo.md`.
The rejected pointer-marketplace option is worth re-reading before starting: if
during this session it turns out there *are* consumers beyond this machine, that
option becomes live again and the ADR says so.

**The hazard this leaf exists to manage.** Archiving is not breakage — an
archived GitHub repo stays readable, so `autoUpdate: true` keeps *succeeding*
against it and the content simply freezes with no error surfaced. A consumer who
misses the announcement gets silently stale skills. The announcement is the
mitigation, so it is a completion condition, not a courtesy.

## Done when

- The merge is pushed and `Linkuistics/grove` serves
  `.claude-plugin/marketplace.json` at its root.
- The local marketplace is re-pointed and **verified end to end**, not assumed:
  - `/plugin marketplace remove linkuistics` then
    `/plugin marketplace add Linkuistics/grove`
  - `~/.claude/plugins/known_marketplaces.json` shows
    `source.repo: "Linkuistics/grove"` under the marketplace name `linkuistics`
  - `installed_plugins.json` still keys `linkuistics@linkuistics` and
    `testanyware@linkuistics` — the names must not change
  - both records show a **commit SHA** as their `version`, with an `installPath`
    ending in that same SHA, and the SHA is this repo's HEAD rather than
    `e0ba6f40f6e8` (the old skills repo's, which is what they carry until the
    re-point). Both plugins should show the *same* SHA. This is the **deferred half
    of `plugin-versioning-k5`**, inverted by `version-bump-guard-k9`: that leaf
    pinned an explicit `1.0.0`, `k9` reversed it and removed `version` from both
    manifests, so a SHA here is now the *expected* result and a `1.0.0` would mean
    a stale manifest or a stale marketplace cache. `docs/adr/skills-monorepo.md`
    carries the trade; do not "fix" a SHA by re-pinning
  - a `linkuistics:<skill>` invocation resolves and loads the skill body
- `Linkuistics/skills` is archived, its README replaced first with a pointer to
  the monorepo and the two commands a consumer must run.
- The move is announced in `CHANGELOG.md` with those same two commands.
- The brew tap is checked: confirm the formula still resolves and that nothing in
  it referenced the skills repo.
- `.claude-plugin/marketplace.json` gains a `description` — it has none, which is
  what a user sees when they run the `/plugin marketplace add` in the pointer
  README above, and it is the marketplace's only `claude plugin validate` warning.
  Surfaced by `version-bump-guard-k9`; routed here because this is the leaf that
  publishes and verifies the marketplace. Note that it does **not** make
  `validate --strict` pass: both plugin manifests fail `--strict` by design now
  (no `version`), so `--strict` is not a gate this repo can use.

## Notes

- Order matters. Push, then verify from the new source, **then** archive.
  Archiving first leaves no working source if the new one has a problem.
- Archiving is reversible on GitHub (unarchive), so this is less hard-to-reverse
  than it feels — but the announcement is not: check the wording before
  publishing it.
- After this leaf the root node has no live leaves left, which puts the grove
  into the **complete finish cycle**: promote anything durable still sitting in
  the briefs, delete `.grove/` in one focused commit, then `grove-llm complete
  --done`. Propose it and wait for explicit human confirmation — never run the
  teardown unprompted.
