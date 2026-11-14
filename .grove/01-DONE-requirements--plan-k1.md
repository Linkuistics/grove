# plan-k1

## Goal

Establish the expected terminal behavior when a Grove session is force-stopped
or its LLM session quits unexpectedly, and agree the scope and test boundaries
for repairing the reported failure.

## Context

The root brief was empty at bootstrap. The reported failure is the starting
point; the mechanism and fix have not been established.

## Done when

The observed failure, expected recovery, repair scope and agreed test boundaries
are recorded, with executable follow-up work stated where needed.

## Notes

### Investigation evidence

Grove's session loop calls `reset_terminal()` after the Ravel job returns and
before interpreting the session result. The reset checks foreground ownership,
runs `stty sane`, leaves the alternate screen, shows the cursor and resets text
attributes. Its current control sequence does not reset extended keyboard
reporting. The lower-level terminal restoration restores saved termios
attributes. These are distinct from a terminal emulator's keyboard protocol
state.

The [Kitty keyboard protocol](https://sw.kovidgoyal.net/kitty/keyboard-protocol/)
defines a mode that reports ordinary keys as escape sequences. Main and alternate
screens have separate keyboard mode stacks. This fits the reported symptom, but
the failure has not been reproduced and its cause remains a hypothesis.

The existing force-stop process/PTY integration test already checks exit status
1, descendant reaping, a live remaining leaf and successful resumption. The
terminal restoration test checks saved terminal modes after driver termination.
Neither of these inspected tests exercises a leftover extended keyboard mode.

### Agreed acceptance checks

Exercise the existing process/terminal-session boundary with a fake agent that
enables extended keyboard reporting and leaves it enabled when it either
force-stops or quits unexpectedly. Observe terminal keyboard behavior after
cleanup, covering ordinary text, Enter, Backspace and Ctrl-C; a termios-only
assertion would miss the reported failure. Retain the process-lifecycle checks
and live, resumable unfinished work for both paths. Confirm the result in the
affected terminal app.

The affected setup is an LLM session running in a Herdr pane inside iTerm. The
LLM called Ravel's force-stop; Herdr itself did not force-stop. Use that nested
combination for the terminal-app confirmation. The human agreed to the
automated typing/control-key checks, resumability checks and affected-terminal
smoke check, then reported the same symptom after an unexpected LLM-session
quit. Apply those same checks to both exit paths. The cause and implementation
remain to be verified in a focused implementation leaf.

## Decisions (running log)

The human reports that, after force-stop, pressing any key shows an escape
sequence and the terminal ignores the intended key; for example, pressing `c`
shows something like `5u_`. The requirement is to return control to a usable
terminal. Extended keyboard mode is an investigation hypothesis, not a verified
root cause.

Terminal recovery is the behavioral requirement: force-stop must return control
to a shell that accepts ordinary input without a manual terminal reset. The
repair must preserve force-stop's existing meaning: unfinished work stays live
and resumable.

The human initially described the affected setup as "herd in iterm". The
agent-issued force-stop path is the reproduction target; the precise caller was
clarified below.

The human agreed to the proposed process/terminal-session test seam: automated
ordinary typing, Enter, Backspace and Ctrl-C checks, existing lifecycle and
resumability checks, and a smoke check using the affected terminal setup. This
agreement is recorded in the root brief for the implementation session; no
separate spec is needed for this focused repair.

The human also reports the same unusable terminal when the LLM session quits
unexpectedly. Terminal recovery therefore covers both agent-issued force-stop
and unexpected LLM exit, even when the agent cannot perform its own cleanup.
Unexpected exit must retain its existing failure/completion semantics and leave
unfinished work live. Use the agreed checks for both paths and investigate the
shared terminal-cleanup boundary rather than assuming a force-stop-only defect.

The human clarified the setup and caller: the LLM inside a Herdr pane called
Ravel's force-stop; Herdr did not itself force-stop. This corrects the earlier
interpretation of the environment. Reproduce with Herdr inside iTerm and observe
recovery at the affected pane as well as the containing terminal where relevant.
