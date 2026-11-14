# grove.force-stop-leaves-terminal-in-bad-state — brief

## Goal

After a Grove force-stop or an unexpected LLM-session exit, return control to a
usable terminal even when the agent did not clean up its terminal modes. The
reported failure turns ordinary keys into visible escape sequences and leaves
input unusable.

## Done when

The reported terminal failure has a verified cause and regression coverage, and
both exit paths return to ordinary shell input while preserving unfinished work
for resumption. Automated coverage and the agreed Herdr/iTerm smoke check confirm
recovery. Existing result interpretation, process reaping and foreground-terminal
ownership rules still hold.

## Decomposition

The bootstrap requirements leaf establishes the symptom, expected recovery and
agreed acceptance checks. A focused implementation leaf reproduces the leftover
terminal state, repairs cleanup and verifies both exit paths through the same
boundary.

## Pointers

- Glossary terms: Terminal disposition; Exit signal / session completion.
- Existing execution contract: `docs/specs/harness-selection-and-execution.md`.
- Shared terminal-session regression boundary: Grove loop-driver process/PTY
  integration tests, including force-stop and terminal restoration.

## Agreed test seams

The human agreed to automated checks at the existing process/terminal-session
boundary plus a smoke check with an LLM in a Herdr pane inside iTerm. The LLM
invokes Ravel's force-stop; Herdr itself is not the process being force-stopped.
A fake agent should enable the suspected extended keyboard reporting, then
force-stop or quit unexpectedly without restoring it. Verify ordinary text,
Enter, Backspace and Ctrl-C after
cleanup; saved termios alone cannot establish keyboard-protocol recovery.
Keep the existing descendant-reaping and unfinished-work resumption checks.
Exercise both exit paths with these same checks, and confirm recovery in the
affected terminal app. If that smoke check needs the human, ask for it rather
than reporting an automated PTY test as iTerm/Herdr confirmation.

## Notes

Extended keyboard reporting survived termios restoration in the affected Herdr
pane. The PTY regressions reproduced retained flags on independent main and
alternate screen stacks after force-stop and SIGKILL. Shared foreground cleanup
now clears both stacks and flags before result interpretation. The human
confirmed ordinary text, Enter, Backspace and Ctrl-C after both Codex exit paths
in Herdr/iTerm; a fresh pane was unaffected by the original failure.

The human requested integration, a minor release and local installation in this
session. This authorizes the project's complete finish/integrate/release cycle
without another confirmation. Ravel 1.x is no longer installed: normal checks
must use the current Ravel 2 without requiring a runtime environment override.

Implementation is verified on main v22.10.0: all 18 principal checks pass,
including 1,346 workspace tests, doctests and all five walkthroughs, with no
Ravel overrides. The release prerequisite doctor passed. No version has been
cut, published or installed for this repair yet. The impl session retires and
seals its commit, then returns to the driver to allocate the reserved finish
session; that session owns the authorized cleanup/integration/minor release and
local install. No additional confirmation is needed. Invoke the installed
`grove-llm` directly for finish-commit, and preserve the ending contract from the
finish session's own launch prompt after the release completes.
