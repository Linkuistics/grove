# shortlist-sources-k3

**Kind:** work

## Goal

Enumerate the universe of candidate **survey sources** and rank each by
relevance to the two targets (**skills | grove**). Then **grow this node**: add
one `dive-<repo>` leaf per greenlit source and a final `synthesis` leaf. This is
the triage gate before any deep-dive — keep it light (READMEs, repo structure,
star/popularity signal), not a full analysis.

## Context

- Start set (from `01-plan-k1`, do not re-derive):
  - *Seeds:* `nousresearch/hermes-agent`, `garrytan/gstack`.
  - *Core comparables:* `obra/superpowers`, `anthropics/skills`,
    `mattpocock/skills`, `addyosmani/agent-skills`.
  - *Breadth indexes to mine for MORE high-signal repos:* `awesome-claude-code`,
    VoltAgent `awesome-claude-code-subagents`, `wshobson/agents`.
  - *Adjacent ecosystems:* `awesome-cursorrules`, aider, Continue, OpenClaw.
- Mine the breadth indexes to surface additional repos by popularity, then add
  the worthwhile ones to the ranking — don't stop at the seed list.
- Read the node `BRIEF.md` for the downstream questions each dive must answer,
  and `CONTEXT.md` for the skills|grove target definitions.

## Done when

- A ranked shortlist exists (in this leaf, or as the first section of
  `docs/research/skill-repo-prior-art.md`), each source scored for
  skills-relevance and grove-relevance with a one-line "why it ranks here" and a
  primary URL. Sources triaged *out* are listed with a one-line reason (record
  the silence).
- The tree is grown: `grove-llm leaf-add 02-survey-prior-art-k2 dive-<repo>`
  for each greenlit source (the two seeds are almost certainly in), then
  `grove-llm leaf-add 02-survey-prior-art-k2 synthesis`.
- Each new `dive-<repo>` leaf carries a one-line goal pointing at the source and
  the brief's downstream questions.

## Notes

- Cap the deep-dives to the genuinely high-signal sources — a dive per repo is a
  fresh session, so rank ruthlessly; a low-signal source can be a one-line
  mention in synthesis instead of its own leaf.
- Light triage only: defer quoting/citation discipline to the deep-dives. But do
  capture the primary URL now so the dive doesn't re-find it.
