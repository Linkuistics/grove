# methodology-and-viewer-integrate-k69

**Kind:** integrate-review-impl
**Integrates:** methodology-and-viewer-review-k68

## Goal

Apply the verified findings from `methodology-and-viewer-review-k68` while preserving the reviewed artifact's contract.

## Context

## Done when

## Notes

## Disposition

- **F1 (High) — fixed.** The loop's procedural order in `content/SKILL.md` is now
  Produce → **Retire** → parent-chain cascade → **Commit** (sealed) → Signal, in
  the mermaid diagram and in the step headings alike. The Commit step states why
  Retire precedes it and that sealing is the boundary's last act; the Retire step
  states that it runs before the commit so the rename and every close-time edit
  land inside it. The `leaf-promote-chain` paragraph carried the same
  commit-then-retire order and was corrected. `CONTEXT.md`'s *Task commit
  boundary* entry now records the step order and carries an `_Avoid_` for the old
  one. `tests/commit_guidance.rs` pins both the heading order and the flowchart
  edges, and its Commit-step slice was re-anchored from `**Retire.**` to
  `**Signal.**`.
- **F2 (Medium) — fixed.** `content/SKILL.md` and `content/TASK-FORMAT.md` no
  longer claim the kind is the only parsed field. Both now state the real
  grammar — position, outcome infix, kind, slug, key, all five structural — and
  scope "convention, not grammar" to what a name might imply about *another*
  leaf (stem, step suffix, relative ordering), which is the claim `tree_id.rs`
  actually supports.
- **F3 (Medium) — fixed, both halves.** `content/SKILL.md` no longer says every
  grow verb takes `--kind`; it enumerates the verbs that do (`leaf-add` /
  `leaf-insert` defaulting to `impl`, `leaf-add-chain` requiring a producer,
  `leaf-decompose` overriding an inherited kind) and states that `leaf-add-pair`
  takes none. `tests/session_kind_guidance.rs` replaces the globally flattened
  `real_long_flags` with `real_long_flags_by_verb`, and the sweep now resolves
  which hyphenated verbs a line names (matched at token boundaries, so
  `leaf-add-chain` is not also read as `leaf-add`) and checks each flag against
  only those verbs.
- **F4 (Medium) — fixed** by this section; see *Verification evidence* below.
  The producers' unchecked "pass" clauses were not treated as evidence.
- **F5 (Medium) — fixed.** `classify_example` no longer decides admission. A
  concrete example is accepted only if `tree_id::parse` returns a leaf — the same
  call `pick`, `resolve` and the grow verbs make; the old shape check survives
  only to explain a rejection, and reports `Malformed` when it thinks a name is
  fine but the parser refused. A grammar sketch (a candidate containing `<` or
  `[`) takes an explicit placeholder path and is still judged, not waved through.

## Out of scope, left to its owner

`docs/USAGE.md:112-135` carries the same commit-then-retire ordering *and*
receipt-era prose ("Grove records the finishing producer's effective harness and
model best-effort in the linked review task"). Both are `user-docs-reconciliation-k80`'s
scope by the `durable-docs-reconciliation-k49` brief, whose `Done when` covers
bare lifecycle behaviour and the absence of hidden harness policy in user docs.
Left there rather than absorbed. `CONTEXT.md`'s *Task commit boundary* entry now
binds the order for that session.

## Verification evidence

Run after the fixes, from this working tree, with `.cargo/config.toml`'s
force-cleared `GROVE_SIGNAL_FILE` guard in effect (meta-grove: the suite is a
descendant of this session).

- `cargo fmt --check` — exit 0, no diff. (`cargo fmt` was run once mid-task to
  reflow an array literal in `tests/commit_guidance.rs`; the check above is the
  post-fix state.)
- `cargo test --locked` — **39 test binaries, 936 passed, 0 failed.**

Both strengthened guards were mutation-tested against the real corpus rather
than only their own unit controls, then the injections reverted:

- Injected `grove-llm leaf-add-pair <parent> <stem> --kind design` into
  `content/SKILL.md` → `every_documented_grove_llm_flag_exists_on_the_real_verb`
  failed with `content/SKILL.md:264: --kind on `leaf-add-pair``. This is the
  exact case the flattened flag set passed.
- Injected `03-impl-bad_slug-k7.md` into `content/TASK-FORMAT.md` →
  `every_leaf_filename_example_in_the_methodology_matches_the_shipped_grammar`
  failed with `content/TASK-FORMAT.md:10: 03-impl-bad_slug-k7.md (Malformed)`.
  This is the exact case the hand-rolled classifier passed.
