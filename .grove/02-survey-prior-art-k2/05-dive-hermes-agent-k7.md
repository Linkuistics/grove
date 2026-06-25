# dive-hermes-agent-k7

**Kind:** work

## Goal

Deep-dive **NousResearch/hermes-agent**
(https://github.com/NousResearch/hermes-agent) as a survey source. Primary
grove interest: its self-improving loop that **creates skills from experience**
plus cross-session procedural memory. End with a **takeaway-for-skills** and a
**takeaway-for-grove**.

## Context

- Shortlist rank #4 (`docs/research/skill-repo-prior-art.md` §1a). Named seed.
  Verified 202,144★ (GitHub API, 2026-06-25). Python; NOT a Claude-Code skills
  repo — adopt *ideas*, not files.
- Read the node `BRIEF.md` for downstream questions + discipline; `CONTEXT.md`
  for the target split.

## Done when

- A `## NousResearch/hermes-agent` section is appended to
  `docs/research/skill-repo-prior-art.md` with cited findings, each tagged
  **target** + walk-away note, ending with takeaway-for-skills /
  takeaway-for-grove.

## Notes

- Focus — **grove Q4** (skill-creation-from-experience, procedural memory
  (`hermes_state.py`, `trajectory_compressor.py`), routines/cron, session
  persistence/resumability). Compare to grove's "artifacts, not state" — does
  hermes keep *state*, and is that a feature or the anti-pattern grove avoids?
- Secondary **skills Q2** — does its auto-skill-authoring suggest a technique for
  *writing* our skills?
- ⚠️ Earlier recon of this repo used a fast summarizer; quote primary files for
  any mechanism claim.
