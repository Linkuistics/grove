# usage-guide-k51

**Reviews:** usage-guide-k23

## Goal

An adversarial read of `docs/USAGE.md` against
`docs/specs/user-guide-coverage.md`. The root brief makes this read what *closes*
`user-guide-k11`: the inventory exists so that completeness is a checkable claim,
and a standard nobody reads against is a standard nobody tested.

## Context

- The producer is `usage-guide-k23`; its commit names that handle, so its diff is
  where the work is. It rewrote the guide's structure, not just its gaps.
- The inventory is the specification. `usage-guide-k23` **changed two of its
  rows** — L7's `--kind` repeatability, and J14 plus the `usage-workspace-layouts`
  entry-point description — on the ground that the binary contradicted them. Those
  two corrections are the highest-value thing to re-derive: a producer allowed to
  edit its own standard can widen it to fit what it wrote, and the argument
  recorded in the inventory is exactly what an adversary should try to break. The
  claimed evidence is `grove-llm leaf-insert --kind a --kind b` (clap refuses a
  second), `tree_lifecycle::finish_commit` (deletes and commits, renames nothing),
  and `jj_workspace::Refusal` (no cross-device case).
- `crates/grove/tests/user_guide_coverage.rs` is new and enforces three
  agreements between the two documents. A test the producer wrote against its own
  work is itself in scope: ask what it does **not** catch.

## Done when

Findings are written down, anchored to file and line, with no fixes applied. In
particular, say for each:

- **A row covered in name only.** The map answers all 34 rows; the map is not the
  coverage. Sample rows across `G`, `L` and `J` and check the section the map
  names actually gets a reader from the start state to the end state, and that
  every command row carries a *worked* invocation rather than a mention.
- **A transcript that is not what the binary prints.** Every fenced `console`
  block claims to be real output with paths rewritten to `/home/you/app`. Check
  the shapes against `--help` and the source — especially the ones that could not
  be run here: the second-driver refusal, the control-directory refusal, the
  `finish-commit` live-work refusal, and the `128 + N` exit.
- **A boundary crossed.** The guide may link `CONFIGURATION.md`, the `README`,
  `CONTEXT.md` and the plugin skills; it may not restate them. The verbs section
  is the likely offender — the inventory's *register, not depth* rule says what a
  verb does to the tree, not the calling contract a session works from.
- **The summary layer.** The opening paragraphs and the coverage map are
  roll-ups; check they still describe the body after the restructure.
- **The anchor set.** Eight anchors are a published surface the next five books
  reserve from. Are they on the headings a book would actually want to cite, and
  is any of them attached to a heading likely to be retitled?

## Notes

Inspection only — no fixes, and no edits to the guide or the inventory. If the
findings are worth acting on, cut the `integrate-review-impl` step as this
session's last act; place it with `leaf-add` unless a later sibling entry still
holds live work, in which case `leaf-insert` at the first one that does.
