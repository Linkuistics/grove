# 020-adr-cross-repo-handoff

**Kind:** work

## Goal

Write the ADR recording the cross-repo inbox handoff rule: a write from
repo A to a grove `Y` in repo B is a write to
`<repo-B>/.grove-inboxes/inboxes/Y.md`, requiring repo B to be checked
out locally with its `grove-inboxes` worktree present. Capture *why* a
transport queue, a shared global store, and out-of-scope deferral were
all rejected.

## Context

- `docs/research/seed-capture-prior-art.md` — the survey explicitly
  flagged cross-repo handoff as having no inheritable precedent ("Every
  system surveyed assumes one repo"). The ADR is the recorded path
  through that absence.
- `CONTEXT.md` glossary entry `grove-inboxes branch` — already names
  the rule; the ADR explains the reasoning.
- This sibling-leaf's ADR
  (`docs/adr/0002-grove-inboxes-branch-and-inbox-model.md`) — the
  cross-repo decision builds on it and should cite it.

## Done when

- `docs/adr/0003-cross-repo-inbox-handoff.md` exists.
- The ADR documents the rule clearly and names the four alternatives
  explicitly considered: layered F1+F2 (with a pending-cross-repo
  bucket), shared global store at `~/.grove-inboxes/`, separate repo,
  and out-of-scope deferral. One paragraph each.
- The ADR records that repo-path discovery (how the writer locates
  repo B on disk) is **deliberately out of scope** for v1 — left to
  user config, CLAUDE.md hints, or convention. The rationale: the
  convention should not bake in a discovery mechanism that may prove
  wrong; the rule is "if you can reach the path, the write is identical
  to a same-repo write."
- The ADR notes that this is the weakest cross-repo use-case coverage
  among the four BRIEF cases, and that a follow-up grove for path
  discovery is a likely seed candidate.

## Notes

- The "weakest coverage" framing is important — the BRIEF's done-when
  said the convention must be "shown to handle" all four use cases. The
  ADR is part of showing how the cross-repo case is handled, including
  its limits.
- The ADR should be short. Most of the design weight sits in the
  sibling inbox-model ADR; this one only adds the cross-repo rule on
  top.
