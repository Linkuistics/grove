# 050-cross-repo-harness

**Kind:** work

## Goal

Make selecting a grove in **any** repo open its working set (harness + detail + aux
tools) correctly (070 Q7). Mostly verification — the driving path already carries the
repo — plus the one real fix: **repo-qualify the workspace/tab keys** so same-named
groves across repos don't collide.

## Context

- The harness-driving path already "carries the repo explicitly so the cross-repo
  fleet reuses this path unchanged" (`src/tui.rs:1129`, `:278`). cwd for `grove do
  <name>` comes from the grove's owning repo, not the process cwd.
- **The wrinkle**: harness tabs/workspaces are keyed by **bare grove name** today —
  `open_harnesses: BTreeSet<String>` (`src/tui.rs:3505`, "the native, name-keyed
  analogue") and `mounted_grove: Option<String>` (`:3511`). Two repos can each have a
  grove named `fix-bug`, so the key must become **repo-qualified**: a `(repo, name)`
  pair or a derived `<repo>:<name>` string. Audit every place a grove is keyed by name
  (open/focus/close/mount) and migrate to the qualified key.
- The per-grove detail surface (`DetailSurface`, `src/tui.rs:3897`) is also fixed to a
  grove — ensure it is fixed to a `(repo, grove)`, not just a name.

## Done when

- Opening a grove in a non-current repo launches its `grove do` in that repo's worktree
  (correct cwd) with its harness + detail + working set.
- Two groves sharing a name across two repos each get their **own** workspace/harness —
  no collision, no mis-focus (manually verified with a same-name pair).
- All name-keyed grove maps (`open_harnesses`, `mounted_grove`, detail surface binding)
  are repo-qualified.
- Switching between groves across repos focuses the right one.

## Notes

Depends on `040` (the nav selection now carries repo). This is the leaf that makes the
root brief's "lists groves spanning multiple repos … selecting any grove in any repo
opens its embedded harness" actually work end-to-end. Verify with `/run` or a live
two-repo fleet, not just unit tests — the collision is a runtime/UX bug.
