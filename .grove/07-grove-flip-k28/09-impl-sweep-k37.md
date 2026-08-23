# sweep-k37

## Goal

The **contract** stage: delete grove's tree algebra now that nothing calls it,
and establish the root brief's claim — *grove's tree modules are gone, grove
supplies a domain impl* — as something checked rather than asserted.

## Context

- `src/lib.rs` — the module list and the long header essays, several of which
  argue about modules this leaf deletes. An essay left behind arguing about a
  module that no longer exists is worse than no essay.
- `references/execute.md`, *Verifying a claim about the repo itself*. This leaf
  makes exactly the kind of claim that section is about, so its method is not
  optional here.
- The technique `src/lib.rs` already records for finding dead surface: copy `src/`
  to a scratch crate, make every module private except `cli` and `llm_cli`, and
  read the compiler's reachability warnings.
- `docs/ARCHITECTURE.md` and `CONTEXT-MAP.md` — both describe grove's structure
  and both go stale here.
- `tests/removed_surface.rs`, which is the existing home for *this is gone*
  assertions.

## Done when

- `src/tree_id.rs`, `src/tree_read.rs`, `src/tree_grow.rs` and
  `src/tree_rename.rs` are deleted, and the tree algebra inside
  `src/tree_lifecycle.rs` and `src/tree_access.rs` is gone — what remains in those
  two is the lifecycle and the guards, which were never algebra.
- The deletion is **verified the way a repo-wide claim has to be**: enumerate then
  classify rather than sweeping a pattern list, with a positive control (the same
  command finds something known to be present) and a cross-tree control (the same
  pattern still finds the class where it legitimately lives — the docs that
  discuss it). A clean grep alone is not evidence; a broken instrument reads clean
  everywhere.
- `docs/ARCHITECTURE.md` and `CONTEXT-MAP.md` describe the structure that now
  exists. The claim's scope is the claim, not a file list — so sweep the summary
  and overview layers too, not only the sections a finding landed against.
- `src/lib.rs`'s module-header essays are true of the modules that remain.
- The whole suite passes, and the node brief's *Done when* holds. This leaf is the
  last in the node, so it also runs the node close: check the brief's `Done when`
  against what the subtree delivered, `leaf-add` any named gap, escalate an
  unnameable one, and promote what is still live upward — `references/retire.md`
  has the four steps.

## Notes

**Two things are permitted to survive that look like they should not.**
`tree_lifecycle` and `tree_access` keep the lifecycle, the guards' grove-specific
refusals, and the transaction sentinels; `tree_format` keeps `FORMAT`. None of
that is tree algebra and none of it has a library counterpart. Deleting a module
because its name starts with `tree_` is the mistake available here.

**`docs/formalism-findings.md` gets this increment's last entry if one is owed.**
By this point the flip has either produced modelling episodes or it has not, and
the honest answer either way is worth recording — a refactor that touched a
checked library and needed no model is itself a finding about when models earn
their place, and `formalism-skill-k38` is about to read this log.

**Do not delete `.grove/` or promote artifacts.** Retiring the last live leaf in a
node is an ordinary retirement; the finish cycle is the driver's, on explicit
human confirmation, and there is still a live leaf at the grove root after this
one.
