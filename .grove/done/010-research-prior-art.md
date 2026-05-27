# 010-research-prior-art

**Kind:** work

## Goal

Survey existing tools, systems, and conventions for capturing work items
across distributed/local boundaries, and assess them against the four
driving use cases in the parent brief. Deliverable: a markdown artifact
under `docs/research/` that a follow-up planning task can grill on to
decide adopt / adapt / invent.

## Context

Organise the survey by **paradigm**, not by tool. The interesting
distinctions are structural, not feature-by-feature. Suggested paradigm
buckets (extend as warranted):

- **Distributed bug trackers embedded in VCS** — git-bug, fossil tickets,
  Bugs Everywhere (BE), ditz, sit, git-appraise. Items live in git refs
  or repo files; sync is push/pull.
- **In-repo markdown conventions** — `TODO.md`, `BACKLOG.md`,
  `docs/issues/NNN-*.md`, ADR-style numbered notes, the "issues-in-repo"
  pattern. No tool, just a convention plus grep.
- **Local-first / PKM / task systems** — taskwarrior, todo.txt, Obsidian
  with backlinks, Dendron, ripgrep-over-notes.
- **Coordinator daemons / local servers** — Sourcehut todo self-hosted, a
  filesystem location watched by a script, an LSP-style local service.
- **Centralised-tracker integrations** — Linear, GitHub Issues
  (incl. cross-repo refs and Projects v2), Jira, Pivotal. Treated as the
  *baseline to beat* — the obvious fallback the convention must justify
  improving on.
- **Other paradigms** — anything that doesn't fit the above (e.g.
  email-based, pubsub queues, IPLD/CRDT-backed stores). Genuinely
  surprising shapes are the highest-value finds.

For each paradigm and each notable tool within it, characterise:

- **Mechanism** — where items live, how capture happens, how aggregation
  happens.
- **Use-case coverage** — score against the four cases in the parent
  brief (deferred-future, parallel-grove, multi-source, cross-repo).
- **Grove ergonomic fit** — how disruptive is mid-flow capture; how
  cheaply can a seed be renamed; can it be invoked without leaving the
  worktree.
- **Walk-away-ability** — what remains legible if the tool disappears.
- **Verdict** — adopt / adapt / reject, with a one-line reason.

## Done when

- `docs/research/seed-capture-prior-art.md` exists.
- Each paradigm bucket above is represented (even if only to note "no
  serious candidate found").
- A short **Shortlist** section at the top names 2–3 candidates worth
  serious consideration in the next planning task, with one-line
  rationale each.
- The artifact resists prescribing a solution — recommendations belong
  to the next planning leaf where they can be grilled.
- ADR candidates discovered during the survey are flagged in a closing
  section (not raised as ADRs in this task).

## Notes

- The bias is **paradigm exploration first**, centralised-tracker
  integration as fallback. If two days of research produces "use GitHub
  Issues," that is a legitimate outcome — but the artifact must show its
  work by characterising the rejected paradigms, not skipping them.
- Use the WebSearch / WebFetch tools and read primary sources where
  possible (project READMEs, design docs) rather than aggregator blog
  posts.
- Time-box: aim for one focused session. If a paradigm bucket turns out
  to need its own deep dive, note it as a follow-up leaf candidate
  rather than expanding this task.
