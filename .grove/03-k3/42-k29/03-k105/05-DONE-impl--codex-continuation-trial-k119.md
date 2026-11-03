# codex-continuation-trial-k119

## Goal

Observe continuation in Codex, and the retained reads after a source change
and a settings change.

## Context

The leaf claude-continuation-trial-k111 states the cases, the spec sections and the notes. Read it and
its record before this leaf runs.

## Done when

- Each Done when item of claude-continuation-trial-k111 holds in Codex, as a separate launch with its own
  record.
- The record names the continuation mechanism. `codex exec resume` is the
  candidate.
- The record compares each Codex result with the Claude Code result of the
  same case.

## Decisions (running log)

- Continuation mechanism: `codex exec <the route's options> resume <thread
  id> <answer>`, outside Ravel's managed process, as the Claude Code case
  runs its resume. Two facts come from the Codex 0.166.0 source, not from
  memory. `resume` opens the existing rollout file for append
  (`open_rollout_for_append` in `codex-rs/rollout/src/recorder.rs`). The
  options before `resume`, such as `--sandbox`, are shared options, and
  `thread_resume_params_from_config` in `codex-rs/exec/src/lib.rs` passes
  their sandbox and approval policy to the resumed thread. `resume` itself
  refuses `--sandbox`. The runner cites both at the decision site.
- The same line windows apply to Codex. The runner takes the thread from
  `thread.started` and the working directory from `session_meta` of the
  session file. The run fails when the resumed turn reports another thread
  or writes a second session file of that thread.
- The Codex description now applies the window and the marker, as the Claude
  Code description does. A resumed window holds a new `turn_context`, so the
  record shows the sandbox of the resumed turn.
- The case runs unchanged: the same tree, answer, edits, settings change and
  fresh leaf as `continuation-20261102T223923Z`.
- The trial directory is prepared fresh at Grove `e13cc55b7ab5`. The 2.0
  prefix is a byte copy of the k118 prefix, with `bin/ravel` digest
  `8e10e311…`. The bookmark `skill-directory-delivery` is still at
  `1ca4aecbeae1`.
- A fake-harness test proves the Codex continuation: the resume arguments,
  the thread, the working directory, the line windows and the marker. It
  fails once against each of five broken runner functions. `continuation` no
  longer refuses for Codex.
- Results of `continuation-20261103T033051Z`, one run with no retry. The
  first turn read both mandatory files, `grove-impl`, the supporting
  procedures, `machine-environment` and `coding-style-bash`. It then wrote
  `stamp.sh` and stopped at the handback, with no ending command. The resumed
  turn kept the thread, the original descriptor and the original bytes. The
  fresh launch received a new descriptor and the new bytes, in the new
  mandatory order. The record is "continuation in Codex".
- Gap, as in Claude Code: the resumed turn did not read the mandatory or
  relevant guidance again. Its only guidance read was the retire and commit
  procedures, before the edit. The three candidate causes of k111 apply
  without change. Not retried, for the reason k111 gives.
- Observations for `acceptance-handoff-k106`: Codex adds a new skill listing
  at the resume, and Claude Code adds none. The Codex fresh launch read
  `coding-style-bash`, so `count.sh` carries the marker. It also read "Commit
  only the script" as a limit, and left the retirement in the working copy.
- No in-session reviewer. The results are observations of transcripts, and
  the new runner path has a fake-harness check that fails against a broken
  runner.
