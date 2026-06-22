# `grove status` is the canonical visibility surface; `grove list` and `grove version` are removed

The repo-admin surface had three overlapping read-only verbs: `grove version` (CLI version + per-harness installed version), `grove list` (grove names, one per line), and `grove status` (install state + per-grove summary). As of the v4 version-drift work, `grove status` reports the full cli/repo/worktree version picture *and* the per-grove summary (see `CONTEXT.md`'s `cli version`/`repo version`/`worktree version` entry), so both other verbs became strict subsets of it. We remove `grove list` and `grove version` outright — no flag-gated migration on `grove status`, no deprecation alias — and designate `grove status` the single canonical visibility surface. `grove --version` (clap's auto-flag) still answers the CLI-only question that `grove version` used to.

## Status
superseded by ADR-0031 — `grove status` and the cli/repo/worktree version-drift model it surfaced were deleted in leaf 090; one binary provisioning one global skill leaves no install state to visualise (`grove --version` still answers the CLI version).

## Why one surface, not three
Three verbs that each show a slice of the same state force the reader to know which slice lives where, and they drift apart as the underlying model grows — exactly what happened when the three-version model (cli/repo/worktree) landed only in `status`, leaving `version` showing a stale two-layer view and `list` showing names with no version context at all. A single surface that the version work already had to extend is the one place a reader can trust to be complete. The principle: **visibility is a canonical surface, not a per-concern verb.** This is recorded as an ADR because the trade-off below is real and a future reader will otherwise wonder why the obviously-convenient `grove list` is gone.

## Considered options
1. **Keep all three, keep them in sync.** Rejected: synchronisation is a standing tax, and the version work already demonstrated the failure mode — `status` got the new model, the others lagged.
2. **Remove `list`/`version`, add a `--names-only` (and similar) flag to `status`** so scripts get a migration path. Rejected during grilling (running log of `010-shape-the-feature`, 2026-05-29): a migration flag re-introduces the per-concern surface inside `status`, just spelled differently, and commits us to maintaining the bare-names contract forever. Cleaner to break once.
3. **Remove `list`/`version` outright (chosen).** Anyone scripting against `grove list` adapts to parse `grove status` or pins to an older grove version; the CLI-only need that `grove version` served is covered by `grove --version`.

## Why the scriptability loss is acceptable
`grove list`'s bare one-name-per-line output was the most machine-parseable surface, so its removal has the highest migration cost. The trade-off is accepted because grove is pre-1.0 in spirit and ships breaking changes within a single iteration (cf. ADR-0006's hard-cutover stance); the only known scripting consumers are local to the maintainer, and `grove status` remains parseable. This removal lands as a breaking change under the v4.0.0 bump (`CHANGELOG.md`), bundled with the `install`/`update` merge (ADR-0008).

## Note on ADR-0006
ADR-0006 enumerates `version` and `list` among the `grove` binary's repo-admin verbs as the surface stood then. That ADR is a historical record and is not edited; this ADR supersedes that slice of it.
