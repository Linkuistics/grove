# 060-backwards-compat-migration — brief

**Kind:** planning (node)

## Goal

The **engine** for the flip to the new dotted-decimal scheme (**ADR-0034**): make
the live verbs **new-format-only**, add a one-time **`grove migrate`** (old→new),
and have **`grove do` migrate an old tree on adoption** before driving. There is
**no transitional dual-format reader** — once adoption migrates, every tree the
verbs see is new-format, so the verbs read only the new format and the migration
is the *only* thing that reads the old format (once).

## Context

Read **ADR-0034** (the flip decision — supersedes ADR-0033's dual-reader plank and
the root BRIEF's old rollout) and **ADR-0033** (the scheme). The new-format verbs
were built isolated in 050: `src/leaf_id.rs` (model), `src/leaf_read.rs`
(pick/brief-chain/resolve), `src/leaf_grow.rs` (leaf-add/insert), `src/leaf_lifecycle.rs`
(root-init/decompose/retire). The live (old) verb path is `src/llm_cli.rs` →
`src/pick.rs`, `src/brief_chain.rs`, `src/leaf.rs`, `src/leaf_ops.rs`,
`src/root_init.rs`. The loop driver / adoption entry is `src/loop_driver.rs` +
`src/launch.rs` (`grove do`).

**This grove is still an old `NNN-slug` tree driven by the installed old binary**,
so every source change in this node is **inert for this grove** — the loop keeps
running old until the new binary is installed at 070. Build freely; the world does
not flip here.

## Decomposition

- `010-verbs-live` (work) — wire the 050 modules into the live `grove-llm`
  dispatch; the CLI surface adopts the new id-addressed signatures
  (`leaf-add <parent-id> <slug>`, `leaf-insert <target-id> <slug>`,
  `leaf-decompose <leaf> <first-child-slug>`, `resolve <ref>`); retire the old verb
  wiring. Build green; tests green.
- `020-grove-migrate` (work) — `grove migrate`: convert an old `NNN-slug/` + `done/`
  tree to the new flat `<position>-[<key>]-<slug>` scheme in place, a reviewable
  git change preserving order, done-ness, brief-chain structure, and assigning
  fresh keys. **Fixture-tested hard** — this must be correct before adoption ever
  touches a real tree.
- `030-migrate-on-adoption` (work) — `grove do` detects an old-format `.grove/` and
  runs the migration (committed, reviewable) before driving; idempotent / no-op on
  new-format trees. The mechanism that performs the flip on next adoption.

## Notes

- Old-format **parsing** survives only insofar as `grove migrate` needs it to read
  an old tree; the old grow/lifecycle *verbs* are no longer wired (retire/delete
  the dead wiring as cleanly as the build allows — deeper dead-code removal can
  defer to the 080/090 sheds).
- The actual flip (install the new binary, migrate this grove + grove-general-
  improvements, go global, remove project-local mirrors) is **070**, not here.

### Dead old modules after 010-verbs-live (for 020 to consume / 080/090 to sweep)

010 unwired the old verb path from `src/llm_cli.rs`; the modules still compile
(all `pub`, so no dead-code warnings) but are now unreachable from the live
binary. Their old-format integration tests were rewritten in place to drive the
**new** binary surface (`tests/{pick,brief_chain,root_init,leaf,leaf_ops}.rs`
plus the new `tests/resolve.rs`), so the old library fns
(`pick::next_leaf`, `brief_chain::chain_for`, `leaf::{add,insert,surface_cross_refs}`,
`leaf_ops::{decompose,retire}`, `root_init::init`) are now **untested and
unreferenced** in the live build:

- `src/pick.rs`, `src/brief_chain.rs`, `src/root_init.rs`, `src/leaf_ops.rs` —
  fully dead; nothing in 060 needs them. **Delete in the 080/090 sheds.**
- `src/leaf.rs` — `Kind` is **still live** (used by `llm_cli` + the new
  `leaf_grow`/`leaf_lifecycle`); the rest (`add`/`insert`/`surface_cross_refs`/
  `write_template` + the old `NNN-slug`/`done/` parsing & header rewriting) is the
  **old-format reader** `020-grove-migrate` should consume, then 080/090 sweep
  whatever 020 leaves. Keep `Kind` regardless.
