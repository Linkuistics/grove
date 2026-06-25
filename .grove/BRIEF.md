# research-new-skills — brief

## Goal

Survey major/popular **skill repos** (and adjacent agent-workflow tooling) and
extract anything worth incorporating, split by **two targets**: the **skills
project** (this repo) and the **grove project** (`Linkuistics/grove`, separate
repo). Named seed sources: `nousresearch/hermes-agent` and `garrytan/gstack`,
plus other major/popular repos discovered via curated indexes. See `CONTEXT.md`
for the target definitions.

## Done when

- A cited research doc (`docs/research/skill-repo-prior-art.md`) exists with
  findings split by target (**skills | grove**), each ranked with a
  primary-source citation and a walk-away note.
- Greenlit **skills-project** findings have become authoring leaves here (or an
  explicit, recorded decision not to author each).
- **Grove-project** findings are written up as recommendations to carry to the
  grove repo (they are *not* implemented from this worktree).

## Decomposition

- `01-plan-k1` — _(planning, this session)_ scope + method settled; tree grown.
- `02-survey-prior-art-k2` — _(node)_ the survey: triage → deep-dive top sources
  → synthesis. First child `01-shortlist-sources-k3` grows the rest lazily.
- _Later:_ authoring leaves (skills findings) / recommendation hand-off (grove
  findings), added once the synthesis ranks them.

## Pointers

- `README.md` — what the marketplace is and its design philosophy
  ("each skill's one-line description is the only standing context cost").
- `.claude-plugin/marketplace.json` — the two plugins.
- `plugins/linkuistics/skills/*` and `plugins/testanyware/skills/*` — the
  existing 9 skills (the baseline a new candidate must beat).
- `CONTEXT.md` — glossary; the **skills | grove** target split.

## Notes

- Grove name: `research-new-skills`.
- The **grove project** is a *separate* repo — grove-targeted findings are
  recommendations produced here, implemented there later. Never edit grove from
  this worktree.
