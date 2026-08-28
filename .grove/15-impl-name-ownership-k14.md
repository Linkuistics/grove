# name-ownership-k14

## Goal

Make **one type own the name, end to end**, and make the handle a rendering of
that same name rather than a grammar spelled six more times.

## Context

`docs/specs/module-decomposition.md`, decision 4, stated verbatim — `Slug`,
`Kind`, `Outcome`, `Handle`, `Parts`, `TaskName`, and
`impl EntryName for TaskName`.

`minimalism-k1` measured the damage: the handle grammar `<slug>-k<key>` has
**six implementations, none behind a type** —
produce at `src/task_tree.rs:513`, `src/tree_lifecycle.rs:220`,
`src/finish_cleanup.rs:121`, `src/task_grow.rs:475`, `src/tree_lifecycle.rs:1174`;
parse at `src/task_tree.rs:952` (`handle_key`), whose own comment concedes it
*"mirrors the filename grammar"*, and `src/task_name.rs:609` (`split_shape`),
which is the same peel written twice. Some of those sites are already gone by
now; re-derive the set.

The handle is **the value that crosses every module boundary**: the store
produces it, the loop puts it in the prompt, the skills pass it back to verbs.

## Done when

- `Handle` exists with `of(&TaskName) -> Option<Self>` (`None` for the charter
  brief, which has no key), `parse`, `slug`, `key`, and `Display` rendering
  `<slug>-k<key>`.
- **Both `TaskName` renderings end in the handle's own rendering**, so there is
  exactly **one place** the `<slug>-k<key>` grammar is spelled and drift between
  the filename and the handle is **not expressible**. This structural form is the
  deliverable; the disciplinary form — *a rule review has to hold* — is not
  enough.
- `Slug`, `Kind` and `Outcome` are types with validating constructors. `Kind`
  **keeps its closed set for now**; only `open-kind-k20` removes it.
- Every hand-rolled produce and peel site is gone, and the commit message names
  them.
- `src/task_name.rs` and `src/leaf.rs` are reconciled; `TaskNameError` keeps its
  shape — every variant carrying what is on disk **and** what it should be, which
  is the model the rest of this design's errors follow.
- `cargo test` and `cargo clippy --all-targets` clean; `CHANGELOG.md` updated.

## Notes

**Lands green.** It is the **expand** stage of the name work: the types arrive,
the grammar on disk does not change, and nothing has to be renamed.

**This is why the separator is worth its rename**, and stating it here is not
redundant: the `--` grammar `grammar-separator-k15` lands next exists partly to
leave the handle a **contiguous terminal substring** of every name that has one.
Build `Handle` so that property is visible in the code, and k15 becomes a small
change to one function rather than a rewrite.

**Do not open `Kind` here.** Doing so before the separator lands makes
`02-design-decomposition-k2.md` ambiguous — kind `design` + slug `decomposition`
and kind `design-decomposition` + empty slug are both readable — and the handle
is what differs between the readings.
