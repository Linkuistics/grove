# `grove install` is idempotent; `grove update` is removed

`grove install` and `grove update` previously differed only in a pre-flight safety check — `install` refused if grove was already present, `update` refused if it was absent — over the same `InstallArgs` and the same backing function. We collapse them into a single idempotent verb: `grove install` installs when absent, no-ops when the bundled version already matches, and updates when it differs. `grove update` is removed outright, with no deprecated alias. The decision is recorded because the mode split was load-bearing for years and a future reader will otherwise wonder why a deliberate create-only guard was dropped.

## Status
accepted

## Why idempotent, not two guarded verbs

The two-mode design forced the caller to know the repo's current state before choosing a verb — exactly the knowledge an install tool should establish for you, not demand. The guards turned an ordinary "make this repo current" intent into an error (`already installed (use grove update)` / `not installed (use grove install)`) whenever the caller guessed wrong, which is hostile to the common case (a setup script or CI step that just wants grove present at the bundled version, whatever the starting state).

Idempotence removes the guessing: one verb, safe to run from any starting state, converging on "materialised at the target version." This is the standard contract for a desired-state operation, and it is the contract CI and setup scripts can lean on — re-running is always safe and never errors on "already installed."

## The audit trail is the always-on outcome line

Dropping the guards removes the old signal that *something was already there*. We replace it with a **per-harness outcome line that always prints**: `installed @ X`, `already at X, no change`, or `updated X → Y`. Because it fires on every run (not only on a state change), it is a reliable, scriptable record of what the invocation actually did — the audit mechanism that the create-only guard used to provide implicitly, now explicit and machine-readable. The ADR-bump nudge fires only on a real `updated X → Y`, so a no-op refresh stays quiet.

## The Cargo `--force` vs Homebrew-idempotent trade-off

Two established models for "install something that may already be installed":

1. **Cargo's `cargo install` is create-only**: it errors if the crate is already installed and requires an explicit `--force` to overwrite. The flag is a deliberate are-you-sure gate; the safe default is to refuse.
2. **Homebrew's `brew install` is effectively idempotent**: re-running on an installed formula is a no-op (or upgrades), not an error, and `brew` reports what it did.

grove deliberately follows the **Homebrew** model, not Cargo's. The rejected Cargo-style alternative would have kept a `--update` / `--force` safety flag to authorise overwriting an existing install. We reject it because grove's materialisation is *fully reproducible from the bundled CLI version* — there is no irreplaceable local state to protect, so the are-you-sure gate guards nothing real. (Grove still refuses one specific case: pre-existing **staged** changes inside the install scope, where an auto-commit could silently bundle in-flight work. That guard is about the commit, not about overwriting the materialisation, and it survives unchanged.) Choosing idempotence over a `--force` flag is the trade-off this ADR records, because a reader familiar with `cargo install` will expect the create-only default and wonder why grove diverges.

## Canonical stamp, raw fetch ref

Folded in here because it lands in the same change: from v4 onward `version_md::write` stores the **canonical** version — the incoming git tag with its leading `v` stripped (`v3.0.1` → `3.0.1`), matching `CARGO_PKG_VERSION`. The idempotency comparison is then plain `stored == canonical`. Critically, the version string does double duty in `src/install.rs`: it is *also* the GitHub fetch ref (`.../refs/tags/v3.0.1.tar.gz`). Only the stamp is canonicalised; the fetch ref keeps its `v`. Because this binary ships only under a new version number, once writes are canonical any stamp still bearing a `v` is a genuinely pre-v4 release, so `grove status` flagging it as drift (plain `!=`, no normalisation) is the correct verdict — there is no read-side compatibility shim (see `CONTEXT.md`'s drift-rule entry and ADR-0007).

## Note on ADR-0006

ADR-0006 enumerates `install` and `update` among the `grove` binary's repo-admin verbs as the surface stood then. That ADR is a historical record and is not edited; this ADR supersedes the slice that names `update` as a distinct verb. (This pairs with ADR-0007, which removed `list` and `version`; together they are the v4.0.0 CLI-surface cleanup.)
