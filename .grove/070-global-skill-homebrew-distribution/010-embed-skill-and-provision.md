# 010-embed-skill-and-provision

**Kind:** work

## Goal

Make the `grove` binary **embed `content/` at compile time** and **extract it
idempotently to the global `~/.claude/skills/grove/` on `grove do` launch** — the
`VERSION.md`-drift-free replacement for the fetch+materialise model. After this
leaf, the skill cannot drift from the binary because it travels inside it.

## Context

The path being replaced (keep it compiling — its **deletion is 090**, not here):
`src/install.rs` fetches a release **tarball** (`src/fetch.rs`), `src/extract.rs`
unpacks `<tarball>/<root>/content/*` into per-harness skill dirs, and
`src/version_md.rs` stamps a drift marker — all driven by the *separate*
`grove install` gesture. The new path lives entirely in `grove` (not `grove-llm` —
ADR-0006 affirmed, node BRIEF): the launch point is `grove do`
(`src/loop_driver.rs` / `src/launch.rs`, dispatched from `src/main.rs`).

Open implementation choices to settle here (not ADR-worthy — node BRIEF):
- **Embed mechanism**: `include_dir!` over `content/` (compile-time directory tree,
  no build.rs) vs a build.rs-generated tar blob via `include_bytes!`. Prefer the
  simplest that carries the whole `content/` tree faithfully.
- **Idempotency**: extract only on mismatch — compare an embedded stamp
  (e.g. a content hash, or the crate version) against the extracted copy; rewrite
  only when they differ, so a warm launch is a cheap no-op.
- **Trigger**: extract on `grove do` (the loop entry the human always hits). A
  warm `~/.claude/skills/grove/` is left untouched.

## Done when

- The `grove` binary contains the full `content/` tree (verify by extracting from a
  binary built with a since-modified `content/`).
- `grove do` extracts `content/` to `~/.claude/skills/grove/` (the global *personal*
  skill dir Claude Code reads live), creating it when absent.
- Extraction is **idempotent**: a second `grove do` with an unchanged embedded
  stamp performs no rewrite; a changed embed re-extracts.
- It is **global** (one personal dir), not the old per-repo/per-harness
  `VERSION.md`-gated materialise.
- The old fetch/install/extract/version_md code **still compiles** (deletion is
  090); tests cover extract-fresh, extract-idempotent, and extract-on-change.

## Notes

- **Inert for this grove** until the new binary is installed (040): this session is
  driven by the installed old binary.
- The extract target dir name/layout is the contract 050's mirror-removal and 030's
  formula rely on — keep it `~/.claude/skills/grove/`.
- `grove-llm` gets **no** provisioning logic.
