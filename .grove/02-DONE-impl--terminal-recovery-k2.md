# terminal-recovery-k2

## Goal

Restore usable terminal input after an LLM calls Ravel's force-stop or quits
unexpectedly, even when that LLM leaves terminal modes enabled. Reproduce the
failure, fix its cause and cover both exit paths through the agreed test seam.

## Context

- Requirements and the human's corrections are recorded in `plan-k1`; the root
  brief carries the agreed behavior and test seams.
- The reported setup is an LLM running in a Herdr pane inside iTerm. The LLM
  called Ravel's force-stop; Herdr itself did not force-stop. Ordinary keys then
  appear as escape sequences and are ignored, with `c` producing something like
  `5u_`. The human also sees this after unexpected LLM-session quits.
- Existing terminal reset and result handling:
  `crates/grove-loop/src/loop_driver.rs` (`reset_terminal`, `drive`).
- Foreground ownership and saved termios restoration:
  `crates/grove-loop/src/ravel_job.rs` (`restore`).
- Existing process/PTY integration seam:
  `crates/grove/tests/loop_driver.rs`, especially
  `force_stop_leaves_remaining_work_resumable_after_continuation` and
  `a_driver_term_cancels_dispatch_and_restores_the_terminal_after_reaping_the_harness`.
- Existing execution contract:
  `docs/specs/harness-selection-and-execution.md`; relevant glossary terms are
  Terminal disposition and Exit signal / session completion.

## Done when

- The failing terminal state is reproduced and its cause established, including
  which layer of the Herdr/iTerm setup retains it. Treat extended keyboard
  reporting as a hypothesis until evidence confirms it.
- After either agent-issued force-stop or unexpected LLM termination, ordinary
  text, Enter, Backspace and Ctrl-C work when control returns. Recovery requires
  no manual terminal reset and does not depend on the LLM's own cleanup.
- Automated regression coverage through the existing process/terminal-session
  boundary demonstrates the failure before the repair and recovery after it for
  both exit paths. Observe terminal keyboard behavior or protocol state; an
  assertion only about saved termios or an emitted reset string is insufficient.
- Foreground-terminal ownership, descendant reaping, result interpretation and
  unfinished-work resumption retain their existing behavior. In particular,
  force-stop exits with status 1 and leaves live work resumable; an unexpected
  exit must not acknowledge task completion.
- The agreed smoke check with an LLM in a Herdr pane inside iTerm confirms the
  user-visible recovery. If this needs the human, request that check and keep
  unverified app behavior explicit.
- Relevant reusable project checks pass, and any durable terminal-cleanup
  contract clarified by the repair is reflected in the existing documentation.

## Notes

The requirements investigation found that Grove calls `reset_terminal()` after
the Ravel job returns, before interpreting the session result. The function
checks foreground ownership, runs `stty sane`, leaves the alternate screen,
shows the cursor and resets text attributes. Its current sequence does not reset
extended keyboard reporting. Ravel-job restoration handles termios separately.
These observations suggest investigating the shared cleanup boundary for both
exit paths; they do not establish the root cause or mandate a particular fix.

The [Kitty keyboard protocol](https://sw.kovidgoyal.net/kitty/keyboard-protocol/)
can encode ordinary keys as escape sequences and maintains independent keyboard
mode stacks for the main and alternate screens. Account for that separation
when reproducing and testing a possible protocol leak. Preserve the existing
boundary between Ravel's process supervision and Grove's result conventions.

Use the root brief's agreed test seams. A fake agent may leave extended keyboard
reporting enabled and either force-stop or quit without cleanup. Retain the
existing lifecycle/resumption assertions alongside keyboard recovery. The app
smoke check must use the corrected Herdr-pane setup, rather than stopping Herdr.
Implementation and verification results follow below.

## Decisions

- 2026-11-14: Keep recovery at `reset_terminal`, after Ravel returns and
  before result interpretation, under the existing foreground ownership check.
  The process/PTY regression reproduced the leak for both force-stop and SIGKILL
  exit: main-screen flags remained 9 with two nested pushes after existing
  cleanup. Termios restoration cannot change this emulator state.
- Clear the alternate and main keyboard stacks and set their flags to zero,
  finishing on the main screen. Visit the alternate screen explicitly because
  a harness may already have returned to main while leaving alternate modes.
  The Kitty protocol specifies independent screen stacks and resetting flags
  when a pop empties the stack. Use an over-pop of 65535 (the supported 16-bit
  parameter range in Herdr's tracker), followed by an explicit zero for peers
  without stack support. No process-supervision or result rules change.
- Keep a small protocol peer in the existing PTY suite rather than add an
  emulator dependency. It interprets screen selection and push/pop/set commands;
  assertions inspect both resulting stacks, including hidden nested modes,
  rather than match Grove's reset bytes. Run each exit path with either screen
  active at death. The fake harness writes protocol commands to `/dev/tty`;
  Grove's reset stdout is captured separately and replayed after that transcript.
- Herdr source in the adjacent checkout (version 0.9.5, unindexed) identifies
  pane libghostty as the live keyboard state/encoder (`src/pane.rs`,
  `src/pane/terminal.rs`); its handoff tracker is not the live input authority.
  Host-mode forwarding exists in `src/server/headless/render.rs` and
  `src/terminal_modes.rs`. This explains the plausible retention boundary but
  does not prove the reported running app has the same state. Human Herdr/iTerm
  smoke confirmation succeeded for both exit paths: the returned shell recovers,
  and the human observed that a fresh Herdr pane was unaffected by the original
  failure, supporting retention in the affected pane.

## Verification

- Before repair, both new regression cases failed at the intended protocol
  assertion: screen 0 stack `[0, 9, 9]` instead of `[0]` (nextest run
  `cfd4fcc3-8162-492f-85c5-5da33b709761`). Initial fixture-control failures were
  corrected before treating this as evidence: Ravel forwards harness stdout to
  diagnostics, so protocol output must explicitly reach `/dev/tty`.

- Human smoke: Codex in Herdr/iTerm now returns usable shell input after
  force-stop and unexpected termination, including text, Enter, Backspace and
  Ctrl-C. The human confirmed both paths in this session.
- The human requested a release and local installation, then clarified that
  Ravel 1.x no longer exists. Make Ravel 2 the default in Taskfile, Rust fixture
  support and release preparation, preserving explicit historical fixtures.
  Existing terminal regressions without a runtime override failed before this
  adjustment, both reporting the obsolete missing-1.x-worker requirement.
- The principal gate's only failures were the changed source inventory in the
  loop walkthrough and the Codex helper fixture rejecting an inherited
  SSL_CERT_FILE. Preserve and compare the inherited certificate setting in the
  fixture; the helper must leave the owner's environment unchanged.

- Integration: fetched origin and rebased the two task changes onto main
  v22.10.0. Upstream already defaulted Taskfile/check.sh to Ravel 2 and fixed
  the helper certificate fixture. Retained those upstream fixes; completed the
  remaining defaults in Rust support and release-preparation fixtures here.
- Updated the walkthrough's literal reset function, source ranges and derived
  totals together. Its complete 15,295-line source reconstruction now passes.

- Final integrated verification: `mise exec -- task check`, with no Ravel
  environment overrides, passed all 18 principal checks. Workspace nextest run
  `ab24510b-59e8-4e5e-9d3a-0f209b43ac6a` passed all 1,346 tests (29 skipped),
  including both new terminal regressions. Doctests and all five walkthrough
  source reconstructions passed. Log: `/tmp/grove-terminal-recovery-integrated-check.log`.
- SHA-256 checks before/after matched for both changed Rust production/test
  files, Rust runtime support, release-preparation fixture, execution spec,
  and every loop walkthrough markdown file and manifest. `git diff --check`
  passed. No code changed after this verification.
- Release prerequisite doctor passed: all three target standard libraries,
  Zig, cargo-zigbuild, authenticated gh, Docker, Homebrew and release tooling
  available. No release cut or publication has started.
- Lifecycle handoff: this mandate remains impl; retiring the last ordinary
  leaf lets the driver allocate its finish sentinel. The finish session must
  carry out the already authorized cleanup, integration, minor release and
  local installation before its final teardown signal; do not manufacture a
  finish leaf or re-request the human's authorization.
