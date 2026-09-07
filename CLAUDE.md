# Working in this repository

`AGENTS.md` is a symlink to this file, so Codex and Claude Code read the same
rules from the same bytes.

## Invoke `grove-llm` directly, never through `cargo run`

Use the installed binary — `grove-llm <verb> …` — or `./target/debug/grove-llm
<verb> …`. Not `cargo run -p grove-llm --bin grove-llm -- <verb> …`.

`.cargo/config.toml` force-clears `GROVE_SIGNAL_FILE` to an empty value for
everything cargo runs. This repository is a **meta-grove**: its own test suite
*is* the loop machinery, that suite is normally run from inside a live `grove do`
session, and its fake harnesses write to `"$GROVE_SIGNAL_FILE"`. Authority to end
a session is ambient — every descendant inherits the variable — so without the
guard a `cargo test` kills the session it was typed into.

The guard cannot tell a test from a verb, so it covers `cargo run` too, and the
two failures it produces both look like success:

- A cargo-launched `grove-llm complete` finds no channel, reports that there is
  no `GROVE_SIGNAL_FILE` and that it is not running under the loop driver, and
  signals nothing — while the driver that launched the session goes on waiting
  for a file that will never appear. Exit code `0`.
- A cargo-launched tree verb runs without the session-epoch admission that a
  nonempty signal path carries, because there is no path left to admit against.
  The verb still does its work.

`.cargo/config.toml` carries the full reasoning, including why an empty value is
deliberately stronger than an inert path.
