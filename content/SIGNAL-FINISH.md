<!-- file: order=27 -->
<!-- unit: skill-finish-endings kinds=finish class=triggering defers="skill-finish-steps skill-finish-nothing-after skill-finish-resume skill-finish-no-signal-stop" -->
**How this session ends is decided by what it did**, and all three outcomes are
open to you. In the two that signal, the signal is your **last action — then do
nothing else**; the loop driver is watching for it and ends the session itself.

| what the session did | ending |
|---|---|
| teardown completed | `grove-llm complete --done` — the loop stops |
| externalised work instead | `grove-llm complete` — the loop relaunches and picks the new leaf; the sentinel waits |
| declined, or no human present | no signal — the loop stops, the leaf stays live and resumable |

The middle outcome is the one worth holding on to. You are told, like every
session, to externalize surfaced work rather than absorb it, and a session that
does so **cannot** tear down: ordinary work is live, and `pick` passes the
sentinel over until it is terminal. That is a plain relaunch rather than a
failure, and it banks no confirmation — the sentinel is never retired, so the
next `finish` session proposes the cycle and waits for a confirmation of its
own. Declining, or finding no human to ask, leaves the leaf live and next, so
the following bare `grove` proposes the cycle again. On confirmation, run the
teardown steps in `references/finish.md`, then end on whichever row above
matches what this session actually did.
