# research-new-skills

The ubiquitous language for the prior-art survey of skill/agent repos. The one
distinction that must not drift across sessions is the **two extraction
targets** — every finding is sorted into exactly one of them.

## Language

**Skills project**:
This repository — the `Linkuistics/skills` marketplace (`linkuistics` +
`testanyware` plugins). One of the two extraction targets. An incorporable
finding aimed here becomes a candidate `SKILL.md` (new or improved) authored
*in this repo*.
_Avoid_: "the marketplace" (ambiguous once grove is also in play), "this repo"
in findings (be explicit — say **skills project**).

**Grove project**:
The *separate* repository `Linkuistics/grove` (local clone `~/Development/grove`).
The other extraction target. An incorporable finding aimed here is a
**recommendation only** — it is written up in this survey but implemented later
in the grove repo, never edited from this worktree.
_Avoid_: treating grove findings as actionable here; conflating grove the
*skill* (the `.grove/` driver) with grove the *project*.

**Incorporable finding**:
A concrete pattern, mechanism, or idea lifted from a surveyed source that is
judged worth bringing into one target. Always tagged with its **target**
(skills | grove), a primary-source citation, and a walk-away note (what the idea
costs / what survives without it).
_Avoid_: "takeaway" used vaguely — a finding names its target and its source.

**Survey source**:
One repo (or curated index) under examination — e.g. `garrytan/gstack`. A
**deep-dive** is the per-source research leaf that examines one source against
the brief's downstream questions.
_Avoid_: "skill repo" as a catch-all — some sources (cursor rules, aider) are
adjacent, not SKILL.md repos.

## Example dialogue

> **Dev:** gstack has a `/retro` slash command that runs a weekly
> retrospective. Is that an incorporable finding?
> **Reviewer:** For which target? As a *skills-project* finding it'd be a new
> `SKILL.md`; as a *grove-project* finding it's a recommendation about
> retiring-with-reflection in the loop. Pick one, cite the gstack source, and
> add the walk-away note — what's lost if we don't take it.
