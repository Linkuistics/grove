# restore-fmt-clean-k33

## Goal

Make `cargo fmt --all --check` pass again, and decide whether a check nothing
gates is worth keeping in `docs/ARCHITECTURE.md`'s principal list.

## Context

`spec-to-current-state-k23` walked the repository's own documented verification
list and found the first entry red:

```
cargo fmt --all --check   →  exit 1, 11 diffs
```

across three crates and eight files — `crates/grove-llm/src/cli.rs`,
`crates/grove-loop/src/lib.rs` (two), `crates/grove-loop/src/session_config.rs`,
`crates/grove-loop/tests/driver_lease.rs` (four),
`crates/grove-loop/tests/prompt.rs`, `crates/grove/tests/commit_guidance.rs` and
`crates/grove/tests/retire_guidance.rs`.

Every one is mechanical: a wrapped `include_str!`, a signature that now fits one
line, a `pub use` ordering. There is **no toolchain pin** in this repo and no
version-skew story — rustfmt 1.9.0, no `rust-toolchain.toml` — so this is not a
formatter disagreeing with the tree, it is the tree never having been formatted
since the crate split.

**Why it went unnoticed is the more useful half.** `cargo fmt --all --check` is
listed under `docs/ARCHITECTURE.md`, *Verification*, as one of the principal
checks, and it is **gated nowhere**: there is no CI workflow in this repository,
and `scripts/` carries only the release rig. Every leaf's `## Done when` names
`cargo test` and `cargo clippy --all-targets` and none names `cargo fmt`, so a
documented check drifted red across at least four leaves with nothing to report
it. `spec-to-current-state-k23` was documentation-only by its own charter and
cut this leaf rather than absorbing the fix.

## Done when

- `cargo fmt --all --check` exits 0, by running `cargo fmt --all` — this is a
  formatting change and must contain no other edit. Read the diff before
  committing and confirm every hunk is whitespace or ordering; a hunk that is
  not belongs to a different leaf.
- `cargo test --workspace` and `cargo clippy --workspace --all-targets` are still
  clean afterwards.
- **The gate question is answered rather than left open.** A check in the
  principal list that nothing runs will drift red again the same way, so decide
  between the two honest outcomes and write the answer where it binds:
  - **enforce it** — a `rust-toolchain.toml` pin (so the check means the same
    thing on every machine) plus something that actually runs it, and the
    standing `## Done when` wording that later leaves inherit; or
  - **demote it** — move it out of `docs/ARCHITECTURE.md`'s principal list and
    say plainly that formatting is not gated here.
  Either is defensible; leaving it listed-but-unrun is the one outcome that is
  not, because it is what produced this leaf.

## Notes

**Lands green.** No behaviour changes and no test should move. If a `cargo fmt`
hunk turns out to be semantic, stop and say so rather than committing it — that
is a different finding and a different leaf.

**Pin before formatting if you pin at all.** Adding `rust-toolchain.toml` after
running `cargo fmt --all` risks a second reformat on the next machine; choosing
the pin first makes the one run authoritative.
