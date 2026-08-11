# task-shaped-node-name-strictness-k168

**Kind:** impl

## Goal

Decide, record, and implement whether a task-shaped *directory* name that is not
a valid node is a malformed tree or an ignorable foreign entry — today it is
silently foreign, and a whole live subtree disappears with it.

## Context

- Found by `tree-vcs-acceptance-verification-k90`'s sweep. Reproduce it in a
  current-format tree:

      .grove/01-DONE-node-k1/01-impl-a-k2.md      # a live leaf

  `grove-llm pick` prints `no live leaves; this grove is done` and exits 0. The
  directory carries an outcome infix, which `tree_id::parse` refuses for a node,
  and `parse_current`'s task-shaped test only fires on names ending in `.md` —
  so the directory is classified foreign and never descended.
- The consequence is the one the leaf-side strictness exists to prevent: real
  live work is invisible, and the driver's next step is to allocate a finish
  leaf and propose teardown. The human gate stands between that and deletion,
  and the work is in VCS history either way, but the tree lies in the meantime.
- Grove never writes such a name: `leaf-retire` and `leaf-prune` refuse a node
  directory operand ("cannot operate on a node directory"). It is reachable only
  by hand — which is exactly the mistake the design predicts, since "a node is
  never marked done" is a rule a human has to know rather than one the filename
  grammar makes unstateable.
- **This is a contract question, not a bug fix.** `docs/specs/config-driven-sessions.md`
  ("Every positioned, keyed **Markdown** filename is task-shaped") and
  `docs/ARCHITECTURE.md` both scope the strictness rule to Markdown names on
  purpose, and `src/tree_id.rs`'s header states the complementary rule — a
  directory named like a leaf is foreign, "that reconciliation is the verb's
  job". Extending strictness to directory names amends all three; leaving it
  alone needs the silent-subtree consequence written down where a reader of
  `read_level` will meet it.
- If the answer is "malformed", the change is narrow: `tree_id::parse_current`
  already has the shape (`parse_parts(stem, true)` succeeding where the node
  parse failed), and `tree_read::read_level` is the single call site. Keep
  reserved witnesses (`PROMOTING-`, `FINISHING-`, `PREPARING-FINISH-`,
  `MIGRATING-session-kinds`) out of the new refusal — none is positioned and
  keyed, and each already has its own earlier, better-worded guard.

## Done when

- The spec and `docs/ARCHITECTURE.md` state one rule for task-shaped *directory*
  names, and `src/tree_id.rs`'s header no longer implies a different one.
- A live leaf under a task-shaped-but-invalid node directory is either read or
  refused — never silently dropped — with the behaviour pinned through the
  `grove-llm` seam for both `pick` and `resolve`.
- `cargo fmt --check` and `cargo test --locked` pass.

## Notes

The interesting question is scope, not mechanism: the same argument reaches a
directory named `01-impl-a-k1.md` (a leaf name on a directory), which the module
header calls foreign deliberately. Answer both together or say why they differ.
