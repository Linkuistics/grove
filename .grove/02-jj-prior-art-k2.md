# jj-prior-art-k2

**Kind:** research

## Goal

Survey existing prior art for making coding agents (Claude Code first, any
`SKILL.md` harness second) use Jujutsu instead of git. Produce
`docs/research/jj-agent-prior-art.md`.

## Context

The audience is one open planning leaf, `skill-design-k3`, which must settle:
adopt vs adapt vs write from scratch; skill names; trigger/detection
mechanisms; and what the native-workflow guidance must cover. Structure the
report so a **Synthesis for skill-design-k3** section answers these questions
directly:

1. Do published jj skills/plugins already exist (agentskills.io, GitHub skill
   repos, gists, marketplace plugins)? For each: scope, licence, quality, and
   whether it teaches native jj workflow or a git→jj translation.
2. What do people actually put in CLAUDE.md / AGENTS.md / system prompts to
   force jj usage (the user's seed query: "how to tell claude code to only use
   jujutsu")? What phrasing reportedly works or fails?
3. What failure modes are reported when agents drive jj? Bias toward
   post-mortems over feature lists — e.g. agents falling back to git in
   colocated repos, snapshotting surprises, immutable-commit errors, bookmark
   non-advancement, `jj git push` confusion, working-copy races when multiple
   agents share a repo.
4. What detection heuristics does prior art use for "this repo is jj-enabled",
   and does anything support an "offer to colocate" behaviour like ours?
5. Anything jj-upstream provides for agents (official docs on AI/agent usage,
   `jj` MCP servers, jj's own CLAUDE.md guidance)?

## Done when

- `docs/research/jj-agent-prior-art.md` exists, structured around the five
  questions, ending with a **Synthesis for skill-design-k3** section that
  makes an adopt/adapt/write recommendation.
- Every failure-mode or works/doesn't-work claim carries a primary-source
  citation (URL: issue, thread, blog post, repo file). "No primary source
  found" notes are recorded where the search came up silent — silence is a
  finding.
- Each candidate skill/config found gets a walk-away check: licence and
  self-containment (could we vendor/adapt it into this repo's plugin
  layout?).
- Committed; leaf retired.

## Notes

AFK leaf — no grilling, no tree growth. If the research surfaces a concern
that needs a human decision, note it in the report for `skill-design-k3`
rather than resolving it here.
