# growing-k33

## Goal

Move the grow verbs onto the library: `leaf-add` → `append`, `leaf-add-pair` →
`append_many`, `leaf-insert` → `insert`. With them goes grove's key allocation —
`tree_id::next_key` and `next_keys` die rather than move, because the library
allocates keys and ordinals itself.

## Context

- `crates/ordinal-fs-tree/src/ops.rs` — `Target`, `NewEntry`, and the plan
  documentation for `append`, `append_many` and `insert`.
- `docs/ordinal-fs-tree/ARCHITECTURE.md`, *How an operation runs*, *The plan is
  checked against itself, in order*, and *Why the shift runs highest-first*. The
  sibling shift is `compose(new_ordinal, key, parts)` and nothing else, so it is
  structurally incapable of disturbing a key, a label or an attribute — the
  property `docs/adr/entry-name-is-the-only-seam.md` sells the seam on.
- The `NoOccupantAtOrdinal` refusal and its **three** messages — past the end, a
  gap, and a hole below the first — and `docs/formalism-findings.md` entry 003,
  which is where the gap case came from.
- `src/tree_grow.rs` — `leaf_add_unlocked`, `leaf_add_pair_unlocked`,
  `leaf_insert_unlocked`, `Renumber`, `surface_cross_refs_unlocked`,
  `next_child_position`, `collect_all_names`, `write_task_template`,
  `task_template_body`.
- The reachability table from `refusals-k30`.
- Suites that are the net: `leaf`, `leaf_ops`, `composition_verbs`,
  `session_kind_tree`.

## Done when

- All three grow verbs run through the library, under the library's write guard,
  resolving their `<parent>` / `<target>` reference against the guard's own
  snapshot before the mutation consumes it.
- `tree_id::next_key` / `next_keys` and `tree_grow::next_child_position` /
  `collect_all_names` have no callers.
- `leaf-insert` still rewrites **zero file contents** — in-file `# …` headers are
  position-free, and the shift is renames only. This is the property its help
  text promises and the one a reader will check.
- The renumber summary and the stray position-prefixed cross-reference warnings
  still reach stderr. `surface_cross_refs` is grove's own concern and has no
  library counterpart; it now reads the snapshot rather than a directory listing.
- `leaf-add-pair` still lands three flat siblings at consecutive positions with
  consecutive keys, as one unit — which is `append_many`'s definition, so this
  should get *simpler*.
- The whole suite passes; changed tests recorded in the node brief.

## Notes

**Key exhaustion changes hands.** `next_keys` is fallible today for a stated
reason: keys are `u32` and never reused, so `max + 1` can leave the space, and
unchecked that is a release wrap which hands a composite verb a wrapped `k0` and
lowers the visible max so the next `leaf-add` re-issues a live key. The library
has `KeysExhausted` and `OrdinalsExhausted` refusals covering the same ground.
Check that grove's *message* survives the handover, or accept the library's — and
note which, because it is the same vocabulary question `refusals-k30` settled.

**`insert` takes an ordinal, and grove's `<target>` is a reference.** grove's
`leaf-insert` names an existing leaf or node by key or path and inserts at the
slot it holds; the library's `insert` takes the ordinal directly. So grove
resolves the target to its ordinal against the snapshot first. `cli-k16` found the
ordinal argument to be **good** rather than awkward — an operator who guesses is
told the level's occupied span by the refusal itself — but grove's operators do
not see it, so grove's own refusal has to carry the equivalent.

**A shifted node carries its whole subtree**, child names and keys untouched,
because each shift is a single rename of one directory. Both designs agree on
this; it is worth an assertion rather than an assumption.

**The template body is grove's, not the library's.** `NewEntry` carries `parts`
and `content`, and the library has no content model — templates, headers and
formats are the consumer's. `write_task_template` / `task_template_body` become
the bytes handed to `NewEntry::new`, and a node gets `NewEntry::empty` because
supplying bytes for a node is `ContentForANode` rather than a silent discard.
