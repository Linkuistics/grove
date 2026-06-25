# synthesis-k13

**Kind:** work

## Goal

Fold all deep-dive findings into a single ranked, deduplicated recommendation
list in `docs/research/skill-repo-prior-art.md`, split by target
(**skills | grove**). This closes the survey node.

## Context

- All `dive-*` sections will be appended above in the doc by the time this runs.
  Also fold the §1b "examined but not deep-dived" notes (continue, moai-adk,
  plannotator, aider, cursorrules, trailofbits, K-Dense, pchalasani) — each is a
  candidate finding to **promote or explicitly drop**.
- Read the node `BRIEF.md` for the cross-survey downstream questions; `CONTEXT.md`
  for the target split; the root `BRIEF.md` for what happens after (authoring
  leaves for skills, hand-off for grove).

## Done when

- A `## Synthesis` section splits findings by target into a ranked, deduplicated
  recommendation list; each item carries source citation(s) + a walk-away note.
- **skills**: ranked candidate skills / authoring / packaging changes, each
  flagged "author a leaf" vs "decided not to" (per root `BRIEF.md` done-when).
- **grove**: ranked recommendations to carry to the grove repo (NOT implemented
  here — `Linkuistics/grove` is a separate repo).
- The brief's cross-survey questions are answered: skills Q1–3, grove Q4–6.

## Notes

- **Dedupe across sources** — several patterns recur; merge each to one
  recommendation citing all sources:
  - *staged pipeline / review gates:* gstack, addyosmani, moai-adk.
  - *file-based / procedural memory:* openclaw, hermes-agent, task-master.
  - *doubt / adversarial verify:* addyosmani (`doubt-driven-development`),
    gstack (`canary`/`guard`), plannotator (plan-review gate).
  - *skill-as-rules + self-authoring:* continue (`create_rule_block`),
    hermes (skill-from-experience).
  - *authoring/packaging:* anthropics (spec/skill-creator), gstack (`skillify`),
    mattpocock (invoked-type split), wshobson (multi-harness), cursorrules/aider
    (description-cost model, read-only conventions).
- After this leaf retires, the survey node has no live leaf — the retire cascade
  should prompt the user, then the root grows authoring/hand-off leaves.
