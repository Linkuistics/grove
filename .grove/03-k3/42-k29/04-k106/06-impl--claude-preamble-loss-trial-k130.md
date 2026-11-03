# claude-preamble-loss-trial-k130

## Goal

Produce the loss of the whole preamble in Claude Code, and observe whether the
agent recovers the descriptor through `RAVEL_SKILL_DIRECTORIES`.

## Context

- The record sections "Compaction", `compaction-preamble` and "Compaction:
  Codex and Claude Code".
- The triage section of `docs/common-loading-acceptance-record.md`.
- The compaction feeder and gate of `scripts/harness_trial.py`.

## Done when

- A variant of `compaction-preamble` writes at least five work files after
  the last guidance read of its first turn. It uses the summary instructions
  of the second attempt.
- The record states, from the session file, which files Claude Code restored
  after the boundary, and whether any of them lies in a snapshot.
- When the gate passes, the record states whether the agent recovered the
  descriptor through the variable and read the mandatory guidance again before
  it acted. Recovery is a pass. Anything else is a gap.
- When the gate fails again, the record holds the mechanism evidence, and the
  leaf asks the human a scope question about the unmeasured case.

## Notes

Record in the task file, before the run, how many attempts the case gets.
A prompted retry is no credit, and the retained variable alone is no evidence.

## Decisions (running log)

- **The restoration rule is documented for this version.** Claude Code's
  page "Explore the context window" says that after compaction, Claude Code
  "re-reads up to five of the files Claude has read or edited in the session,
  choosing the ones modified most recently"
  (https://code.claude.com/docs/en/context-window#what-survives-compaction).
  The order is by modification time, not by read time.
- **The first attempt agrees with the rule.** Its run directory still exists.
  Claude Code restored `stamp.sh` first, then the two mandatory `SKILL.md`
  files. Both snapshot files have the launch time 23:16:13Z as their
  modification time. `stamp.sh` has 23:16:50Z. Only three files were
  restored, so only three files were in the read and edited set. The run
  directory of the second attempt no longer exists.
- **The variant is a new case, `compaction-preamble-five`.** Its leaf asks
  for five work files, each written in its own step, before the label
  question. Its summary instructions are those of the second attempt, word
  for word. Its answer is the answer of `compaction-preamble`.
- **The case gets one counted attempt.** One further run is allowed only if
  the first turn of the first run writes fewer than five work files after its
  last guidance read. That run did not test the variant. A gate failure after
  five such writes ends the case, with no further run.
- **The run reuses the preparation at Grove commit `1f7b8013d4f6`.** It is
  the preparation of `claude-force-stop-report-k136`, and its digests are in
  the record. Cases come from this working tree, so a new case needs no new
  preparation. Claude Code is now 2.1.317. The earlier attempts ran 2.1.316.
- **The variant met its precondition, and the gate failed.** The run is
  `compaction-preamble-five-20261103T085255Z`. The first turn wrote the five
  work files after its last guidance read, in three Bash calls. The summary
  held no preamble text. Claude Code restored only the two mandatory
  `SKILL.md` files, both in a snapshot. By the attempt rule, no further run
  follows.
- **Bash heredoc writes do not enter the restoration set.** The agent wrote
  every work file through `cat > <file> <<'EOF'`, and Claude Code restored
  none of them. In the first attempt, the Write tool wrote `stamp.sh`, and
  Claude Code restored it. A Bash call that is only `cat "<path>"` puts the
  file in the set. A compound Bash read does not. The record section
  "compaction-preamble-five" holds the table and its limit.
- **The leaf asks the human a scope question.** The question: accept the
  Claude Code `compaction-preamble` row as an explicit unmeasured gap, or cut
  one more repair whose leaf directs the Write tool for the work files.
