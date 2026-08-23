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

## Decisions (running log)

- **Baseline measured before touching anything: 1272 passing**, workspace-wide,
  which is exactly what `migration-k36` recorded. Both sides of this leaf's
  test-count comparison are therefore measured the same way, which is
  `lifecycle-k35`'s finding applied to its own successor.
- **`migration-k36`'s falsifiable prediction is false, and the interesting part
  is that it could not have fired.** Re-aiming `tests/session_kind_guidance.rs`'s
  oracle from `tree_id::parse` to `TaskName::parse` surfaced **no** guidance
  example the canonical grammar refuses. But the withdrawn grammar's leniency was
  baked into two *more* places in that file — `candidates_in`'s position pattern
  and `classify_shape`'s — both of which admitted a two-digit position and
  nothing else. Under a lenient parser that cost nothing, since a skipped name
  would have been accepted anyway; under a canonical parser it is the sweep
  reporting clean about the one class the tightening is about. Both are widened
  to a digit run, `the_candidate_scan_offers_every_position_width_to_the_parser`
  is the control, and the corpus is clean **measured** rather than by
  construction.
- **The four modules are deleted and the whole suite is green:
  `src/tree_id.rs`, `src/tree_read.rs`, `src/tree_grow.rs`,
  `src/tree_rename.rs`.** `tree_lifecycle` and `tree_access` needed no surgery —
  neither held any algebra by the time this leaf opened. What they hold is the
  lifecycle, the guards, the sentinels and the grove-specific refusals, exactly
  as the task's Notes said, and the only edit either needed was to one test
  helper that grew a leaf through the withdrawn appender and now calls the verb.
- **`docs/ARCHITECTURE.md` already carried the dead-surface technique and the
  `pub`-item rule that `src/lib.rs`'s longest essay restated**, so deleting that
  essay with its module loses nothing. Checked before deleting rather than
  after — the essay was the task file's own named context, and a restatement is
  only safe to drop once the statement is found.
- **The deletion is checked by enumerate-then-classify in
  `tests/removed_surface.rs`, and it found 33 references a `use`-line grep would
  not have.** Every module-shaped `tree_*` / `task_*` token under `src/` and
  `tests/` is enumerated — code and prose alike — and classified against a live
  set read **off disk** and a listed withdrawn set. The instrument's first run
  reported stale essays in twelve files, none of them a `use` line and all of
  them arguing about a module that no longer exists.
  - The **positive control** is the tokeniser finding `tree_id` in a line that
    carries one, and a live module beside it, and refusing to invent one out of
    `subtask_grow`.
  - The **cross-tree control** is the same tokeniser over `docs/` and
    `CHANGELOG.md`, where every withdrawn name must still be found — which is
    both the proof the instrument is not broken and the thing that stops the
    withdrawn list becoming a fossil naming modules grove never had.
  - A second test holds `live_modules()` (disk) equal to what `lib.rs` declares,
    in both directions, so the sweep's generative half cannot silently diverge
    from the crate's.
- **The docs sweep is the claim's scope, not a file list, and that is what
  reached `Cargo.toml`.** `docs/ARCHITECTURE.md` gains
  *The withdrawn tree algebra* — what went, the three things that deliberately
  survived, and how the deletion is checked — and its module-seams table, its
  module-ownership sentence, its species-half paragraph, its two-locks section
  and its version-control seam are all corrected. `CONTEXT-MAP.md` stops calling
  the library *being extracted*. `docs/specs/doubt-grove-review-mechanics.md`'s
  selection seam is re-pointed at `task_tree::selected`, whose composition is
  unchanged in shape. Both ADRs that named a withdrawn module are reworked in
  place to current state rather than left predicting a deletion that has since
  happened. **`Cargo.toml`'s header was the find**: it said the library is what
  grove's tree modules *are being extracted into*, present tense, and no sweep
  rooted at `src/` and `tests/` can reach a root manifest — which is the
  narrowing `references/execute.md` names and the reason the summary-layer pass
  is a separate reading rather than a longer pattern list.
- **The withdrawn names are kept in `docs/` deliberately, and the check depends
  on it.** The cross-tree control asserts each is still found there, so tidying
  the deletion out of the record breaks the sweep rather than passing it. The
  failure message says so, because the benign-looking third cause — someone
  cleaned up the history — is the one a future reader would otherwise fix in the
  wrong direction.
- **No CHANGELOG entry, and that is the file's own rule.** This leaf changes
  nothing a reader runs; only `marking-k32` of the seven flip leaves earned an
  entry, for the `git status` change. An entry here would log a refactor as a
  release note.
- **`docs/formalism-findings.md` entry 025 is owed and written**, and it is the
  aggregate the task's Notes asked for as much as this leaf's own: four of seven
  flip leaves reached for no formalism, and the honest reading is not that
  refactors are model-free but that a refactor *onto an already-checked library*
  inherits its models and needs consumer-side instruments instead. Three routing
  rows added.
