# one-build-owns-a-session-k17

## Goal

Enact `docs/adr/one-build-owns-a-session.md`. The docs already describe the
decided state; this leaf makes the code match them. **Read the ADR first** — and
read `shared-skill-dir-clobber-review-k16`'s outcome, which may have changed it.

## Context

The defect: `~/.claude/skills/grove` and its Codex and Pi siblings are global and
shared, while the driver lease is per working tree, so any `grove` build can
write the directory a live loop is reading. `cargo run --bin grove` from this
checkout is the likely instance — it lays the checkout's `content/` beside the
*installed* `grove-llm`.

Today's guard cannot see it, for two independent reasons, and both are fixed
here: it compares crate **versions** (all `17.0.0` here, across a release of
divergence), and it resolves the **sibling** of `current_exe()` — a binary that
agrees with the driver by construction — while the session resolves `grove-llm`
through its own `PATH`. `loop_driver.rs:103` discards the resolved path
entirely; only the check's success is used.

## Done when

1. **`build.rs` emits the methodology identity as a compile-time constant.**
   `cargo:rustc-env=GROVE_CONTENT_HASH=<hex>`, computed to match
   `provision::content_hash` exactly: every embedded file's path relative to
   `content/` and its bytes, sorted by path, each field `u64`-LE
   length-prefixed, SHA-256, lowercase hex (`provision.rs:175-203` is the rule).
   An in-crate test asserts `content_hash(&CONTENT) == env!("GROVE_CONTENT_HASH")`
   so the two traversals cannot drift silently.
2. **`grove-llm --content-hash` prints that constant** and exits. A **flag**, not
   a verb: no session calls it, the methodology instructs nothing about it, and
   it must stay invisible to `tests/provision.rs`'s `exposed_verbs()` and
   `scan_instructed_verbs`. It must read the constant — calling
   `provision::content_hash` would relink the embed into `grove-llm` and cost the
   ~320 KB that binary currently saves by not carrying it. Assert that saving
   holds (the content marker string must still appear only in `grove`).
3. **The per-iteration pair check resolves through `PATH` and compares
   identity.** Replace `checked_grove_llm`'s sibling preference and version
   comparison. Keep it per iteration (a mid-loop `brew upgrade` is the case a
   start-time check misses) and keep it a resumable no-mutation stop — no
   `.grove/` created, an existing tree byte-identical, rerunning bare `grove`
   continues. The diagnostic names both identities and the one remedy:
   `cargo install --path .`. `parse_checked_version` and `DRIVER_VERSION` lose
   their caller unless the review kept a version check beside the identity one.
4. **Provisioning re-verifies per iteration and repairs.** Before each launch,
   compare each installed harness skill directory's stamp with the driver's
   identity; a match is the ordinary case and costs one small read per root. On a
   mismatch, restore this driver's embed and print one line naming the directory.
   Do **not** re-extract unconditionally — a driver never re-execs, so that is
   identical bytes at full cost. Sequence it before the pair check (see
   `docs/ARCHITECTURE.md` §Runtime flow, already updated).
5. **`grove-llm` warns on a stamp mismatch and never refuses.** On any verb,
   compare its own constant with each installed skill directory's stamp; on
   disagreement print one stderr line naming the directory and both identities.
   Absence is not disagreement — an unprovisioned or missing directory is
   silent — and no verb's exit status changes. This is the only check a clobber
   landing mid-session can reach.
6. `cargo test` passes; `cargo clippy --all-targets` clean; `cargo fmt --check`
   clean.
7. **`CHANGELOG.md`'s `## Unreleased` gains the entry.** The design commit
   deliberately has none — this repo logs a decision when its behaviour lands,
   and names the ADR there.

## Pointers

- `src/provision.rs` — `STAMP_FILE`, `content_hash`, `sync_to_stamp`,
  `provision_installed`, `provision_target`. The re-verify entry point belongs
  here; `skill_dir_in` + `HARNESSES` is how both binaries enumerate the roots.
- `src/loop_driver.rs:103` (call site), `:382-428` (`checked_grove_llm`,
  `parse_checked_version`, `DRIVER_VERSION`).
- `src/harness.rs` — the registry `grove-llm` also needs for its warning.
- `src/llm_cli.rs`, `src/bin/grove-llm.rs` — the flag.
- `build.rs` — currently emits only `rerun-if-changed`; the hash goes beside it.
- `tests/provision.rs` — existing stamp/foreign-dir/sweep coverage to extend.

## Notes

The documents are already reconciled and must not be contradicted:
`docs/adr/one-build-owns-a-session.md`, `docs/ARCHITECTURE.md` (§Runtime flow,
§Command surfaces, §Embedded methodology and its new *shared directory*
subsection), `docs/specs/config-driven-sessions.md`, `docs/USAGE.md`,
`CONTEXT.md` (**Methodology identity**, **Build pairing**) and `CONTEXT-MAP.md`.
If implementation shows one of them wrong, fix the document in place rather than
letting the code and the record disagree.

This leaf changes only the binary, not `content/`, so it carries none of the
release-boundary coupling the root brief describes for `flat-lazy-review-k2`.
