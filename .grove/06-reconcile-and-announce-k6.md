# reconcile-and-announce-k6

**Kind:** work

## Goal

Reconcile the existing git-mentioning skills with the jj design, and
announce the two new skills across the marketplace surfaces.

## Context

Settled scope (rationale in `03-DONE-skill-design-k3.md`'s running log; the
two new skills exist by now — `using-jujutsu`, `git-to-jj-mapping`):

- **guardrail** — add jj destructive patterns alongside the git ones, in
  all three places: `scripts/guardrail-hook.sh` (pattern list around lines
  82–84), `SKILL.md` (table around lines 65–67), and
  `scripts/guardrail-hook.test.sh` (ask + defer cases). Candidate
  patterns: `jj abandon`, `jj op restore`, plus whichever force-push form
  jj 0.43 actually has — verify with `jj git push --help` before writing
  the regex; do not invent flags. Keep the hook's existing style (tab-
  separated pattern + reason) and run the test suite.
- **decision-records** — generalise the "git holds the past/history"
  phrasing to the VCS at all 6 sites (`SKILL.md` lines 12, 36, 56, 69,
  157, 162), e.g. "ADRs hold the present; the VCS holds the past." Keep
  each sentence's rhythm; this is a phrasing generalisation, not a
  rewrite.
- **cli-tool-design** — NO edit (settled: "like `git`" is an apt style
  example regardless of jj).
- **README.md** — add both skills to the skill table (name | invocation |
  one-liner), matching existing row style; both are "by description".
- **plugin.json** (`plugins/linkuistics/.claude-plugin/plugin.json`) —
  extend `description` and `keywords` (e.g. `jujutsu`, `jj`,
  `version-control`).
- **CHANGELOG.md** — one entry covering both new skills and the
  reconciliation, matching the file's existing entry style.

## Done when

- guardrail's test suite passes with the new jj cases included.
- decision-records reads naturally at all 6 edited sites; no "git holds"
  phrasing remains (`grep -n 'git holds' plugins/linkuistics/skills/
  decision-records/SKILL.md` is empty).
- README table, plugin.json, CHANGELOG each mention both new skills.
- One focused commit naming `reconcile-and-announce-k6`; leaf retired.

## Notes

AFK. Test seam is the existing `guardrail-hook.test.sh` — extend it, don't
create a parallel harness.
