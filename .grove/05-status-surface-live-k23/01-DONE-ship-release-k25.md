# ship-release-k25

**Kind:** impl

## Goal

Ship a grove release carrying the reporter and the seventeen-kind taxonomy, and
prove the *installed* binary contains it — so that the only thing still standing
between grove and a live status surface is the herdr server restart, which is a
human's call.

## Context

HEAD is 20 commits past `v15.0.0` with `Cargo.toml` still reading 15.0.0, so the
released version number and a large pile of unreleased work share a name. Five
CHANGELOG entries sit above the first `## v` heading, several of them breaking
(the kind rename `work` → `impl`, and a kind that resolves no model variable now
failing the launch), which makes the next release a **major**.

Release machinery is git-shaped — `cargo release` commits and tags through git,
`release-build.sh` refuses a dirty tree and demands `git describe --tags
--exact-match HEAD`, `release-publish.sh` reads `git describe`. This grove's
working tree is a **jj-native secondary workspace with no `.git`**, so the
release cannot be cut from here; the colocated default workspace at
`~/Development/grove` is where git can see the repo (*symmetric-vcs-rule* — jj
performs every mutation, git is read-only, but tag creation has no jj verb).

The `main` bookmark still points at the pre-grove commit, 20 changes back.

## Done when

- The v16.0.0 CHANGELOG section is cut over the five unreleased entries.
- `Cargo.toml` is off 15.0.0, the release is tagged, tarballs built for all three
  targets, GitHub Release created, tap formula updated, `brew upgrade` clean.
- `strings "$(readlink -f "$(which grove)")"` shows all three of
  `HERDR_SOCKET_PATH`, `HERDR_PANE_ID`, `pane.report_agent` — the cheap check
  that the shipped binary is the one with the reporter.
- The shipped `grove-llm` accepts `--kind impl`, retiring the
  `./target/debug/grove-llm` workaround this grove has needed all along.
- The human is asked how they want herdr restarted, and told what it costs.

## Notes

Ends without the restart. Restarting kills this pane, and the driver above this
session is the *old* binary regardless — so the observation half is
`observe-live-surface-k26`, in a session launched under both new processes.

Green baseline before the release: `cargo test` — 534 tests, 20 suites, 0
failures, exit 0.
