# plan-k1

**Kind:** planning

## Goal

Bootstrap the grove: grill the workstream's shape, write the root brief, and
grow the first leaves.

## Context

Fresh grove; the branch name `use-jujutsu-when-possible` was the only mandate.

## Done when

Root `BRIEF.md` states goal and done-when; the tree has a next actionable
leaf; decisions are logged; committed.

## Decisions (running log)

1. **Deliverable is "both"** — new jj skill(s) *and* a reconciliation pass
   over existing skills that mention git.
2. **"When possible" semantics** — `.jj/` present → jj is the primary VCS
   interface; jj installed but repo not initialized → offer
   `jj git init --colocate` once per session, never silently; no jj binary →
   stay silent.
3. **Two skills, not one** — the main skill teaches jj's native workflow
   (plus detection/offer behaviour); the git→jj mapping is a separate skill
   loaded on demand, so the translation table costs no context unless needed.
4. **Research before design** — survey existing jj skills and "make Claude
   Code use jj" configurations first (user pointer: the search
   "how to tell claude code to only use jujutsu"); adopt or adapt beats
   rewrite. Deferred to `jj-prior-art-k2`; skill naming, trigger mechanisms,
   and harness scope deferred to `skill-design-k3` so research can inform
   them.
