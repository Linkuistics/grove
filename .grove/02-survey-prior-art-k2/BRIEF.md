# survey-prior-art-k2 — brief

## Goal

Survey major/popular skill repos and adjacent agent-workflow tooling, and
extract **incorporable findings** for the two targets — the **skills project**
and the **grove project** (see `CONTEXT.md`). Deliver a single cited research
doc that a later session can act on: greenlit skills-project findings become
authoring leaves here; grove-project findings become recommendations carried to
the grove repo.

## Done when

- A shortlist of sources exists, each ranked by relevance to **skills | grove**.
- Each greenlit source has a deep-dive that answers the downstream questions
  below, with primary-source citations and a walk-away note per finding.
- A synthesis splits all findings by target into a ranked, deduplicated
  recommendation list.

## Decomposition

- `01-shortlist-sources` — enumerate the universe (the named seeds + the four
  clusters), rank by relevance, and **grow the rest of this node**: one
  `dive-<repo>` leaf per greenlit source, then a `synthesis` leaf. Deep-dive and
  synthesis leaves are added here *lazily*, once the shortlist is known.
- `02..0N-dive-<repo>` — one deep-dive per top-ranked source (added by 01).
- `0X-synthesis` — fold findings into `docs/research/skill-repo-prior-art.md`,
  split by target, ranked.

## Pointers

- Seed sources: `github.com/nousresearch/hermes-agent`,
  `github.com/garrytan/gstack`.
- Source clusters (settled in `01-plan-k1`): **core comparables**
  (`obra/superpowers`, `anthropics/skills`, `mattpocock/skills`,
  `addyosmani/agent-skills`); **breadth via awesome-lists**
  (`awesome-claude-code`, VoltAgent `awesome-claude-code-subagents`,
  `wshobson/agents`); **adjacent ecosystems** (`awesome-cursorrules`, aider,
  Continue, OpenClaw).
- Glossary terms in play: **skills project**, **grove project**, **incorporable
  finding**, **survey source** (see `CONTEXT.md`).
- The grove *skill* itself lives at `~/.claude/skills/grove`; the grove
  *project* repo is `~/Development/grove` — read these when judging whether a
  grove-targeted finding is already covered.

## Downstream questions the deep-dives must answer

Per source, end with a **takeaway-for-skills** and a **takeaway-for-grove**
(either may be "none"). Across the survey, answer:

**For the skills project:**
1. What *kinds* of skill does this source ship that we don't (new language,
   craft/design, workflow), and which would earn their standing description cost
   here?
2. What SKILL.md *authoring* techniques does it use (frontmatter, progressive
   disclosure, `references/` splitting, path-based auto-load, examples) that
   would improve how our skills are written?
3. What is its packaging/distribution model (plugin marketplace, install
   script, symlinks) and does any of it beat ours?

**For the grove project:**
4. How does it handle **long-horizon / multi-session** work — memory,
   procedural-skill creation, session persistence, resumability? (hermes-agent's
   autonomous skill-creation + procedural memory; gstack's checkpoint/
   context-restore; OpenClaw's MEMORY.md/SOUL.md are the obvious probes.)
5. What **staged-pipeline / multi-agent** patterns does it use (gstack's
   Think→Plan→Build→Review→Ship→Reflect, conductor parallel sprints) and do any
   improve grove's loop, decomposition, or retire/finish cycle?
6. What **doubt / review / verification** mechanisms does it bake in that grove
   could adopt (cross-model review, adversarial verify, canary)?

## Notes — search discipline (from driving.md)

- **Bias the search toward non-obvious paradigms and post-mortems**, not
  feature-list tours. For each source ask "what did it learn the hard way?"
- **Citation per claim.** A pattern without a primary source (repo file, README
  section, issue, doc) is mood, not evidence. Quote the source.
- **Walk-away check per finding.** With the source uninstalled, what survives —
  and what does adopting it cost us in standing context / complexity?
- **Record silence.** "Searched, found no primary source" is itself a finding.
- ⚠️ The seed-repo recon used a fast summarizer that returned dubious stats
  (star/commit counts). Verify any quantitative claim against the primary repo.
