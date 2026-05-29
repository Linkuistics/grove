# 070-fix-leaf-add-node-default

**Kind:** work

## Goal

Make `grove-llm leaf-add` / `leaf-insert` default their target node to the
grove root (`.grove/`), not the current working directory, so a leaf created
from the worktree root lands where `pick` walks.

## Context

Deferred from an inbox observation (2026-05-29): leaves created with
`leaf-add` from the worktree root landed at `<worktree>/NNN-*.md`, one level
above `.grove/`, so `pick` never saw them and they had to be removed and
recreated by hand.

Root cause — `src/llm_cli.rs:279-286`:

```rust
fn resolve_node(arg: Option<&std::path::Path>) -> Result<PathBuf> {
    let cwd = std::env::current_dir().context("getting cwd")?;
    let node = match arg {
        Some(p) => cwd.join(p),
        None => cwd,            // defaults to cwd (worktree root), not .grove/
    };
    Ok(node)
}
```

The asymmetry: `cmd_pick` (`src/llm_cli.rs:184`) and `cmd_brief_chain`
(`src/llm_cli.rs:203`) both resolve against `worktree.join(".grove")`, but the
leaf verbs resolve against cwd. The decompose/retire verbs already have the
right pattern — `src/leaf_ops.rs:119-137` tries a path as absolute, then
cwd-relative, then grove-root-relative.

Pointers:
- `src/llm_cli.rs:279` — `resolve_node`, the fix site (used by `cmd_leaf_add`
  at :226 and `cmd_leaf_insert` at :239).
- `src/llm_cli.rs:97-98`, `:109-110` — the `--node` help text ("Default:
  current working directory") must change to match.
- `src/leaf_ops.rs:119-137` — the existing absolute / cwd-relative /
  grove-root-relative resolution to mirror.
- `src/repo.rs` — `git_toplevel` gives the worktree root.

## Done when

- `grove-llm leaf-add <slug>` run from the worktree root creates the leaf
  under `.grove/`, visible to `grove-llm pick`.
- `--node <path>` is honoured as before (absolute, or relative to cwd);
  consider also accepting grove-root-relative to match `leaf_ops`.
- The same fix applies to `leaf-insert`.
- The `--node` `#[arg]` help text and the `--help` output no longer say the
  default is the current working directory.
- A unit/integration test covers the default-node case (leaf lands in
  `.grove/`, not the worktree root).
