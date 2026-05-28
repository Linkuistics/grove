# 030-remove-list-and-version-and-adr-0007

**Kind:** work

## Goal

Delete the `grove list` and `grove version` verbs (their output is a
subset of the new `grove status`), and write ADR-0007 documenting the
"`grove status` is the canonical visibility surface" principle.

## Context

- `src/list.rs` — to delete.
- `src/version.rs` — to delete.
- `src/cli.rs` — `Command::List` (line 276 approx) and `Command::Version`
  arms, plus the corresponding enum variants and any `clap`-derive
  attributes.
- `docs/grove.md:62` — the `grove list` doc line; check for `grove
  version` doc lines nearby.
- `README.md` — any references to `grove list` / `grove version`.
- `CHANGELOG.md` — add an entry under a new unreleased section, noting
  these as **breaking** changes.
- `docs/adr/` — write `0007-status-is-canonical-visibility-surface.md`.
- Decisions from [[010-shape-the-feature]] driving this leaf:
  - Both verbs are removed wholesale, no flag-gated migration on
    `grove status` (no `--names-only`).
  - `grove --version` (clap auto-flag) still answers the CLI-only need
    after `grove version` is gone.

## Done when

- Files deleted: `src/list.rs`, `src/version.rs`.
- `src/cli.rs` no longer has `List` or `Version` enum arms; `cargo
  check` clean.
- `grove --version` still prints the CLI version (verify by hand).
- `docs/grove.md`, `README.md`, any other doc with references — updated.
- `docs/adr/0007-status-is-canonical-visibility-surface.md` written per
  `ADR-FORMAT.md`. Cites the running log of [[010-shape-the-feature]]
  for the surface-reduction principle; notes the two removed verbs and
  the canonical-status replacement.
- `CHANGELOG.md` has a breaking-change entry.

## Notes

- This is part of the v4.0.0 release group (with [[040-merge-install-and-update-and-adr-0008]]).
  The version bump itself lands in [[060-changelog-and-release-v4]].
- An ADR is appropriate here because the *principle* (canonical
  visibility surface = `grove status`, not a per-concern verb) is
  what future readers will want to know, and the choice was a real
  trade-off against scriptability of `grove list`.
