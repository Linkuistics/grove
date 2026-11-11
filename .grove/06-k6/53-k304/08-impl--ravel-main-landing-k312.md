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
