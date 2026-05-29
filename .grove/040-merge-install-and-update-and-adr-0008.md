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
- **Decision (2026-05-29, while implementing [[020-extend-status-with-worktree-versions]]):**
  store the **stripped** version in `VERSION.md`. The leading `v` is a
  git-tag artifact — `3.0.1` is the version's identity and matches
  `CARGO_PKG_VERSION`. `version_md::write` should `strip_v` the stamp it
  writes so the stored artifact is canonical and directly comparable to
  the cli version. **Critical constraint:** in `src/install.rs` the
  `version` string does double duty — it is also the *fetch ref*
  (`fetch_tarball(&version)` → `.../refs/tags/v3.0.1.tar.gz`). Strip
  **only** the stamp, never the fetch ref; `latest_version()` and
  `--version` keep returning/accepting the `v`-prefixed tag. The
  read-side `status::strip_v` stays as a backward-compat shim for stamps
  already materialised with the `v` (it is no longer load-bearing once
  new writes are canonical). This dovetails with the per-harness compare
  below: comparing canonical stored stamps to the target is cleaner if
  both sides are normalised via `status::same_version`.

## Done when

- `Mode` enum is gone from `src/install.rs`; `run` and `run_with_fetcher`
  no longer take a mode parameter.
- Per-harness outcome decided by comparing existing `VERSION.md` (if any)
  to the target version, and printed using one of the three lines above.
- `version_md::write` stores the stripped version (no leading `v`); a
  test asserts the stamp written for tag `v3.0.1` reads back `3.0.1`,
  while the fetch ref passed to the fetcher remains `v3.0.1`.
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
