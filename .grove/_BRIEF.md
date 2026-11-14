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

Extended keyboard reporting left enabled in the terminal emulator is a plausible
cause; it is not yet reproduced. The reported setup is an LLM in a Herdr pane
inside iTerm, with the LLM calling Ravel's force-stop. The human also reports
this after an unexpected LLM quit. The implementation must verify the cause
before choosing the repair. Grove's
current shared reset runs before it interprets a Ravel session's result, so the
investigation should cover both paths. Preserve force-stop's exit status and
resumability and the existing handling of unexpected exits.
