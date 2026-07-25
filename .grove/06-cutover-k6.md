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
  - a `linkuistics:<skill>` invocation resolves and loads the skill body
- `Linkuistics/skills` is archived, its README replaced first with a pointer to
  the monorepo and the two commands a consumer must run.
- The move is announced in `CHANGELOG.md` with those same two commands.
- The brew tap is checked: confirm the formula still resolves and that nothing in
  it referenced the skills repo.

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
