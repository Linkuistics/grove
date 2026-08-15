# retire-next-steps-k2

## Goal

Make `grove-llm leaf-retire` and `grove-llm leaf-prune` tell the session its
remaining loop steps, so the instruction to run `grove-llm complete` arrives at
the moment of decision rather than only at session start.

## Context

Sessions do their work correctly and then never signal; under the configured
interactive harnesses that **stalls** the loop rather than stopping it (root
brief, *Notes*). The instruction already exists — `skill-signal`,
`content/SKILL.md:558` — but it is delivered in the mandate at session start, a
whole session's worth of context before the moment it applies.

`leaf-retire` is the **last grove verb a session runs**: Retire precedes Commit,
and the commit itself is jj/git, not a grove verb. Its output therefore lands in
the agent's context exactly where the reminder is useful — under every harness,
with no personal configuration involved.

## Done when

- `cmd_leaf_retire` (`src/llm_cli.rs:800`) emits, alongside its existing path
  line, a reminder naming the remaining steps: commit, then `grove-llm complete`
  as the last action.
- `cmd_leaf_prune` (`src/llm_cli.rs:809`) does the same when it marked anything.
- The reminder goes to **stderr**.
- `src/complete.rs:23-27` is corrected: its doc comment says a session that does
  not signal means "the loop stops", which is true only of a harness that exits
  on its own. Ours are interactive, so the real outcome is a stall. That comment
  is what made this failure mode hard to see.
- Tests cover both verbs' new output. `tests/retire_guidance.rs` and
  `tests/llm_cli.rs` are the likely homes; check `tests/goldens` for any output
  golden that has to move with it.
- CHANGELOG entry under `## Unreleased`.

## Notes

Two decisions are already made — each cost a grilling round, so do not reopen
them:

- **stderr, not stdout.** stdout is data: `leaf-retire`'s one line is the
  destination path and callers parse it. `cmd_leaf_prune` already writes its two
  advisory lines to stderr; follow that precedent.
- **These two verbs only.** They are the terminal-marking pair — both end a
  session's work, both are followed by commit-then-signal. The grow verbs are
  not, and should stay quiet.

Open, and yours to settle: the reminder's exact wording. Name the two remaining
steps in order; do not restate the loop.

Verification caveat: this is a meta-grove, so the change reaches no session in
this loop — it is the *installed* `grove-llm` that sessions run. Test it
directly; do not expect to observe it in a sibling session.
