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
4. From the default colocated workspace, run `task release:minor`, or
   `task release:patch` / `task release:major` when the human requests that
   increment. These tasks check, cut, build, publish the tag and three binary
   archives, update the Homebrew tap, and verify the installed release.
   The tasks inspect jj state, preserve unrelated Grove working-copy changes,
   and generate missing release notes. Keep intended changes on `main` and
   the tap clean; see `docs/RELEASING.md` for prerequisites and recovery.
   Use jj for commits and bookmark pushes wherever enabled; the documented
   cargo-release and release-tag operations are exceptions.
5. Return to the original session workspace and run
   the ending commands specified by this session's launch prompt and kind
   skill, as the last action, only after the requested integration and release
   have completed. A launch under v22 ends with `grove-llm complete --done`;
   a supervised-dispatch launch first runs `grove-llm record-teardown`, then
   `harness-dispatch exit`. The installed driver and launch prompt decide which
   contract this session must use, even if the release has installed new binaries.

This project extends the finish skill's normal teardown-only scope: when the
whole sequence is authorized, integration and release happen before its final
signal. A failed step leaves the remaining work explicit; never signal success
for an incomplete release. Do not delete the workspace as part of this sequence.

For a standalone release, use the same `task release:patch`,
`task release:minor`, or `task release:major` command. A request for a point
release means `task release:patch`. Do not rerun a release task after a partial
cut or publication: resume the unfinished steps in `docs/RELEASING.md` so the
version is not bumped again.

## Invoke `grove-llm` directly, never through `cargo run`

Use the installed binary — `grove-llm <verb> …` — or `./target/debug/grove-llm
<verb> …`. Not `cargo run -p grove-llm --bin grove-llm -- <verb> …`.

`.cargo/config.toml` force-clears `GROVE_LAUNCH_DIR` and
`HARNESS_DISPATCH_EXIT_FILE` for everything cargo runs. This repository is a
**meta-grove**: its test suite runs inside a live session and inherits its
authority. Without the guard a test could impersonate that session through
Grove's epoch admission or end it through dispatch's exit channel. The guard
also retains the retired `GROVE_SIGNAL_FILE` and `GROVE_RUN_SIGNAL_FILE` names
while installed v22 binaries drive this cutover.

The guard covers `cargo run` too, so verbs launched through cargo lack that
authority: a tree verb runs without session admission, `record-teardown`
reports no launch directory and records nothing, and `harness-dispatch exit`
reports no supervised run and signals nothing. Those last two no-ops exit 0.
Under an installed v22 launch, a cargo-launched `grove-llm complete` likewise
reports no channel and signals nothing. Invoke the binary directly, and use
the version and ending contract named by the session's launch prompt.

`.cargo/config.toml` carries the full reasoning, including why an empty value is
deliberately stronger than an inert path.
