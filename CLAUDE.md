# Working in this repository

`AGENTS.md` is a symlink to this file, so Codex and Claude Code read the same
rules from the same bytes.

## Finish, integrate, and release

The usual end of a grove in this project includes integration and a minor
release. Unless the human requests a different scope or version increment,
propose the whole sequence together and obtain the finish skill's explicit
confirmation once. A request to carry out that sequence is authorization;
do not ask again for its individual steps.

1. Preserve durable decisions and documentation, verify the work, and commit
   any remaining project changes through jj.
2. Run `grove-llm finish-commit <finish-handle>` from the session workspace.
   Let the helper remove and commit `.grove/`; never delete it by hand.
3. Fetch current `main`, rebase the grove's changes onto it, resolve conflicts,
   and run `bash scripts/check.sh` on the integrated result. Move `main` to
   the finished change and push it through jj.
4. Follow `docs/RELEASING.md` to cut the next minor version from the default
   colocated workspace, publish the tag and three binary archives, update the
   Homebrew tap, and verify the installed release. Preserve unrelated work in
   both repositories. Use jj for commits and bookmark pushes wherever enabled;
   the documented cargo-release and release-tag operations are exceptions.
5. Return to the original session workspace and run
   `grove-llm complete --done` as the last action, only after the requested
   integration and release have completed.

This project extends the finish skill's normal teardown-only scope: when the
whole sequence is authorized, integration and release happen before its final
signal. A failed step leaves the remaining work explicit; never signal success
for an incomplete release. Do not delete the workspace as part of this sequence.

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
