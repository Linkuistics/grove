# 010-verbs-live

**Kind:** work

## Goal

Make the live `grove-llm` verbs **new-format-only**: wire the 050 modules
(`leaf_id` / `leaf_read` / `leaf_grow` / `leaf_lifecycle`) into the live dispatch
in `src/llm_cli.rs`, replacing the old directory-based verbs, and adopt the new
**id-addressed** CLI signatures (ADR-0033/0034). Retire the old verb wiring.

## Context

Read **ADR-0034** (flip; new-format-only) and **ADR-0033** (the scheme + verb
surface). Live dispatch + arg structs: `src/llm_cli.rs` (`run()` match;
`LeafAddArgs` / `LeafInsertArgs` / `LeafDecomposeArgs` / `RootInitArgs`; the
`resolve_node` / `parse_prefix_slug` helpers). New impls already exist and are
tested in isolation: `leaf_read::{pick,brief_chain,resolve,Resolution,
render_resolution}`, `leaf_grow::{leaf_add,leaf_insert,surface_cross_refs,Kind}`,
`leaf_lifecycle::{root_init,leaf_decompose,leaf_retire}`.

This grove is still old-format and driven by the installed old binary, so flipping
the *source* verb path does **not** affect this grove's driving (ADR-0034).

## Done when

- `grove-llm` dispatches `pick` / `brief-chain` / `resolve` / `leaf-add` /
  `leaf-insert` / `leaf-decompose` / `leaf-retire` / `root-init` to the **new**
  impls. `resolve` is added as a new verb (`resolve <ref>`).
- The CLI signatures move to id-addressing:
  - `leaf-add <parent-id> <slug> [--kind]` (root parent `.`); the old
    `--prefix` / `--node` escape hatches are gone.
  - `leaf-insert <target-id> <slug> [--kind]` (two clean positionals; the old
    `<NNN-slug>` single-positional form + `parse_prefix_slug` are gone).
  - `leaf-decompose <leaf-path> <first-child-slug>`.
  - `root-init [<slug>]` produces `BRIEF.md` + `1-[1]-<slug>.md`.
- The verb `--help` / doc-comments describe the new scheme (these are the verb
  help strings, not the methodology prose — prose is 070).
- The old verb modules' *wiring* is removed; old code kept only where `grove
  migrate` (020) will consume it (the old-format parser). Note any old module left
  dead for 080/090 to sweep.
- `cargo build` green; `cargo test` green (existing new-format unit tests in the
  050 modules + adjust/replace `tests/llm_cli.rs` etc. for the new signatures).

## Notes

- Keep the verb names hyphenated/flat (the `grove-llm` LLM surface convention).
- `pick` / `brief-chain` keep their argument shapes (internal dispatch only);
  `leaf-retire` keeps `<leaf-path>`. Only add/insert/decompose/resolve change shape.
- Do **not** touch the methodology prose (`content/`, `.claude/skills/grove/`)
  here — that is 070's flip step (ADR-0034).
