**Signal.** Once the task is retired and committed (and any parent-chain cascade
is settled and included), run **`grove-llm complete`** as your **last action — then do
nothing else**. This is how the self-driving loop ends this session and starts
the next task with fresh context: the verb only writes the relaunch flag to a
signal file (`GROVE_SIGNAL_FILE`) and returns. Ending the session is the **loop
driver's** job, not this verb's — the driver launched this session and is
watching for the signal file while it runs, so it applies grace → SIGTERM →
kill-grace → SIGKILL to its own child once the file appears (driver-side
watcher: the driver can always signal its child, unlike an in-agent self-kill,
which some harness sandboxes — e.g. codex's Seatbelt — silently deny). Run
outside a `grove` loop (no `GROVE_SIGNAL_FILE`) it is a safe no-op that just
tells you to exit manually. Ending *without* signalling stops the loop instead —
a crash or a Ctrl-C is exactly that — so the signal is what separates a task you
finished from a session that died, and it is the whole of what you have to do to
hand back.
