# dive-task-master-k8

**Kind:** work

## Goal

Deep-dive **eyaltoledano/claude-task-master**
(https://github.com/eyaltoledano/claude-task-master) as a survey source. It is
the **closest external analog to grove's task-tree**: persisted task/subtask
files, a dependency graph, status tracking, and a "next task" surface. End with
a **takeaway-for-skills** and a **takeaway-for-grove**.

## Context

- Shortlist rank #5 (`docs/research/skill-repo-prior-art.md` §1a). Verified
  27,683★ (GitHub API, 2026-06-25). MCP/CLI tool, not a skills repo.
- Read the node `BRIEF.md` for downstream questions + discipline; `CONTEXT.md`
  for the target split.

## Done when

- A `## eyaltoledano/claude-task-master` section is appended to
  `docs/research/skill-repo-prior-art.md` with cited findings, each tagged
  **target** + walk-away note, ending with takeaway-for-skills /
  takeaway-for-grove.

## Notes

- Focus — **grove Q4/Q5**: this is a *direct design comparison*. Examine the
  `.taskmaster/` task model, how decomposition works (PRD → tasks → subtasks),
  dependency tracking, status/next-task selection, and cross-session
  resumability. Contrast with grove's choices: directory-tree-as-state vs a
  task JSON store; `grove-llm pick` (depth-first live-leaf walk) vs its
  dependency-graph "next task"; DONE-infix-in-place vs status fields.
- The valuable output is *where grove diverges and why* — confirm grove's design
  or surface a borrowed improvement (e.g. dependency edges between leaves).
