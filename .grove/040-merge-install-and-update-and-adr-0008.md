# 040-merge-install-and-update-and-adr-0008

**Kind:** work

## Goal

Merge `grove update` into `grove install` with idempotent semantics, and
write ADR-0008 documenting the Cargo-vs-Homebrew trade-off.

## Context

- `src/install.rs:14-17` — the `Mode` enum (`Install`, `Update`); to
  collapse.
- `src/install.rs:37-51` — the pre-flight safety check that distinguishes
  the two; replaced by per-harness outcome decision.
- `src/install.rs:19,24` — `pub fn run`, `pub fn run_with_fetcher` — both
  take `mode: Mode`; signatures change.
- `src/cli.rs:14,16` — `Install(InstallArgs)`, `Update(InstallArgs)` —
  the `Update` arm goes away.
- `src/cli.rs:277-278` (approx) — `Command::Install` / `Command::Update`
  match arms.
- `docs/grove.md` — any mention of `grove update`.
- `README.md` — any `grove update` references.
- `CHANGELOG.md` — append a breaking-change entry (alongside the entries
  from [[030-remove-list-and-version-and-adr-0007]]).
- `docs/adr/` — write `0008-install-is-idempotent.md`.
- Decisions from [[010-shape-the-feature]] driving this leaf:
  - Behaviour: not installed → install; same version → no-op; different
    version → update.
  - **Always** print the per-harness outcome: `installed @ X`,
    `already at X, no change`, `updated X → Y`.
  - No safety flag (no `--update` / `--force`).
  - Name stays `grove install`; no deprecated alias for `grove update`.
  - `path-scoped commit` + `install scope` (from `CONTEXT.md`) continue
    to apply.

## Done when

- `Mode` enum is gone from `src/install.rs`; `run` and `run_with_fetcher`
  no longer take a mode parameter.
- Per-harness outcome decided by comparing existing `VERSION.md` (if any)
  to the target version, and printed using one of the three lines above.
- `src/cli.rs` no longer has `Update`; `Command::Install` is the only
  install verb.
- `grove install` over an already-installed-at-same-version target prints
  the no-change line and exits 0 (no error).
- `grove install` over an already-installed-at-different-version target
  prints the update line and overwrites.
- `grove install --version <tag>` honours the pin (no-op if already at
  `<tag>`, else update).
- Existing install-related tests pass; new tests cover the three
  per-harness outcomes and the `--version` pin idempotency.
- `docs/adr/0008-install-is-idempotent.md` written per `ADR-FORMAT.md`.
  Captures the Cargo-`--force` vs Homebrew-idempotent trade-off; cites
  the always-on outcome line as the audit-trail mechanism.

## Notes

- Part of the v4.0.0 release group (with [[030-remove-list-and-version-and-adr-0007]]).
- Make idempotency a contract worth relying on in CI/setup scripts;
  document explicitly in the ADR and CHANGELOG so users can pin on it.
- The error message `(use 'grove update')` at line 42 disappears
  entirely. The complementary `(use 'grove install')` at line 47 also
  disappears.
