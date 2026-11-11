# ravel-main-landing-k312

## Goal

Land the release line on Ravel `main` with every check green, ready for one
2.0.0 cut.

## Context

- Grove `docs/specs/coordinated-release.md`, step 3 of "Publication ownership
  and order".
- Ravel `CHANGELOG.md`, section `Unreleased`, and the paused leaf
  `Ravel.gh-issue-2/.grove/21-impl--release-ravel-2-0-0-k37.md` for the
  changelog content it expected (read only).
- Ravel specifications whose status still describes unreleased work, such as
  `docs/specs/release-channels.md` and `docs/specs/skill-discovery.md`.

## Done when

- A fresh fetch shows Ravel `main` on the remote. The line is rebased onto it.
  Each conflict is resolved and recorded.
- The `Unreleased` changelog opens with the 2.0.0 summary. It names the
  configuration-format change, the `config_format_retired` refusal with exit
  3 and book chapter 4. It names the new settings keys, the host functions,
  schema 4, the removed Bun worker and the withdrawal of confinement.
- Ravel's specifications, `CONTEXT.md` and `README.md` describe the state
  that the release publishes. Historical records stay history.
- From the default workspace, `task check`, `task test`, `task release:smoke`,
  `task release:lifecycle:test`, `task release:formula` and every book check
  pass on the rebased commit.
- `main` points to that commit and is pushed through jj.
- `task release:doctor` passes with `RAVEL_TAP_DIR` set to the tap clone. No
  version is cut.

## Notes

If `main` moved since 2026-11-11, reinspect manifests, tags, releases, book and
tap before the rebase. If any 2.0.0 cut or public tag exists, stop. Reconcile
it under the recovery table, and do not cut again.

`task release:lifecycle:test` fails at the mutant “install keeps cleanup”
when the shell exports `HOMEBREW_NO_INSTALL_CLEANUP=1`. The fake `brew` of
`scripts/release-channel.test.sh` logs that variable from the inherited
environment, so the mutant of `scripts/release-install.sh` passes every
control. The draft ravel-book-inside-k308 saw it fail with the variable and
pass with `env -u` and no other change, on 2026-11-12. The operator notes of
this node ask for the variable on Homebrew commands. Run the suite without
it. A scrub of the variable inside the test's fixture would close the gap.
Chapter 11 of the book now tells the reader to run the suite without it.

The draft's review of chapter 11 also found two statements of the
specification `docs/specs/release-channels.md`, "Release interface", that
the scripts do not hold. "Every script that writes or reads a formula takes
`--channel`" omits `release-promote.sh`, which takes none, and the optional
channel of `release-formula.sh`. The chapter now states the exceptions.
Make the specification agree when it moves to the released state.
