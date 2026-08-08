# session-kind-tree-review-k24

**Kind:** review-impl
**Reviews:** session-kind-tree-k23
**Producer launch:** {"producer":"session-kind-tree-k23","session":"session-kind-tree-k23","generation":"k23","harness":"codex","model":"sol-xhigh"}

## Goal

Adversarially review `session-kind-tree-k23` and record concrete findings for its integration step.

## Context

- Review `session-kind-tree-k23` against the filename grammar, task-tree scheme,
  pick semantics, promotion contract, and viewer compatibility in
  `docs/specs/config-driven-sessions.md`.
- Attack parser-prefix ambiguity, terminal handling, key monotonicity, foreign
  file lenience, finish starvation, accidental body-field fallback, and any
  current/legacy dual-reader leakage.
- This review is inspection-only. Inspect the producer's committed diff,
  source, specifications, and recorded verification evidence. Do not run test,
  build, lint, or format commands, edit production or test code, or redo the
  implementation.
- Record findings only. `session-kind-tree-integrate-k25` owns every fix and all
  post-fix verification.

## Done when

- Findings are recorded here with severity, exact reproducer or code evidence,
  and the contract each finding threatens, or an explicit no-finding result.
- The review cites the inspected source, specifications, diff, and producer's
  recorded verification evidence for every conclusion.
- No production or test code is changed.

## Findings

Inspected: commit `porwsqkxyozs` (`session-kind-tree-k23`) diff and post-change
source for `src/leaf.rs`, `src/tree_id.rs`, `src/tree_format.rs`,
`src/tree_read.rs`, `src/tree_grow.rs`, `src/tree_lifecycle.rs`,
`src/tree_promotion.rs`, `src/tree_migrate.rs`, `src/tree_access.rs`,
`src/loop_driver.rs`, `src/llm_cli.rs`, `tests/session_kind_tree.rs`, and the
inline test modules; `docs/specs/config-driven-sessions.md` sections "Session
kinds live in filenames" (226-285), "Authoritative selection and mandate"
(287-333), "Removed surfaces and compatibility" (707-753), and "Test seams"
(801-893); the producer's recorded evidence (`cargo fmt --check` 0,
`cargo test --locked` 0, 362 library tests). No code was run and no production
or test file was changed.

### F1 — high — a kind-shaped legacy slug hard-fails migration, reintroducing the exact slug-dependence the witness was added to remove

`src/tree_migrate.rs:291-296`. `detect` checks the witness first
(`src/tree_migrate.rs:269-272`), but the witness-less branch still classifies
filenames with the **kind-aware** `tree_id::parse`, and now `bail!`s on any hit:

```rust
} else if ft.is_file() {
    if matches!(tree_id::parse(&name), Some(Entry::Leaf { .. })) {
        bail!("current-format Grove leaf {:?} has no FORMAT witness; refusing legacy migration", name);
    } else if parse_legacy_v2_leaf(&name) {
```

Reproducer: a legacy `.grove/` with no `FORMAT`, containing `BRIEF.md` and
`01-design-notes-k1.md`. `tree_id::parse` peels position `01`, slug
`design-notes`, key `1`, and `Kind::split_filename_prefix("design-notes")`
(`src/leaf.rs:329-339`) strips `design` and returns
`Some(Entry::Leaf { kind: Design, slug: "notes", key: 1 })` — so `detect`
bails and `grove migrate` exits non-zero with no remedy. The byte-identical
sibling `01-notes-k1.md` in the same tree migrates fine. Ten leading tokens are
enough to trigger it (`design-`, `impl-`, `planning-`, `prototype-`,
`requirements-`, `finish-`, `research-a-`, `research-b-`, `combine-research-`,
and every `review-*`/`integrate-review-*` label); this grove's own tree already
contains `finish-lifecycle-k43`, which parses as kind `finish` + slug
`lifecycle` and only escapes because `detect` reads the top level and that leaf
is nested.

Contract threatened: `docs/specs/config-driven-sessions.md:242-245` — the
witness exists so that "already current" is *independent of slug text*, quoting
`01-design-notes-k3.md` as the case it must fix. This branch makes the same
slug decide the answer, inverted from silent-adoption to hard refusal, and
blocks the automatic path `session-kind-migration-k27` and
`lifecycle-cutover-k39` will call on every bare `grove`.

No test covers the new bail: `detect_v2_by_keyed_leaf`
(`src/tree_migrate.rs:820-824`) uses `01-plan-k1.md`, and `plan` is not a kind
label (`planning` is), so the fixtures pass over it by accident. Changing that
fixture to `01-design-notes-k1.md` fails the test.

Suggested direction: the witness check above already decides currency, so the
kind-aware branch has no work left to do — drop it and let
`parse_legacy_v2_leaf` classify the witness-less tree (restoring
`Format::V2` → `AlreadyV2` for a pre-session-kind tree), and add a `detect`
case with a kind-prefixed legacy slug.

### F2 — medium — the legacy-v2 adapter recognises `DONE-` but not `ABANDONED-`

`src/tree_migrate.rs:317-336`. `parse_legacy_v2_leaf` strips only `DONE-`:

```rust
let rest = rest.strip_prefix("DONE-").unwrap_or(rest);
let Some((slug, key)) = rest.rsplit_once("-k") else { return false };
… && tree_id::validate_slug(slug).is_ok()
```

Reproducer: a witness-less `.grove/` holding `BRIEF.md` and
`01-ABANDONED-spike-k1.md`. `rest` stays `ABANDONED-spike-k1`, the slug comes
out `ABANDONED-spike`, and `validate_slug` rejects it as uppercase
(`src/tree_id.rs:440-447`), so the leaf is invisible to all four format probes:
`has_v2`/`has_v1`/`has_old` stay false and `detect` returns `Format::Empty`
(`src/tree_migrate.rs:313`) → `Outcome::NothingToMigrate` → "no migratable tree
in …" (`src/tree_migrate.rs:246-249`) instead of "already v2".

The abandoned infix is part of the pre-session-kind v2 grammar — ADR *pruning*
predates this change, and `tree_id::Outcome::Abandoned` and the
`ABANDONED` render/parse pair already existed before the diff
(`src/tree_id.rs:185-189`, `304-319`). Contract threatened: the isolated
legacy adapter this slice promises ("The legacy adapter needed by migration
remains isolated and explicitly tested", producer Done-when) misreads a shape
the legacy format really produces, and `session-kind-migration-k27` inherits
the misclassification. `render` in the same file has the mirror-image gap: it
emits only `DONE-`/`""` (`src/tree_migrate.rs:448-451`), which is correct for
v1 input but is now the only writer of the "legacy v2" spelling the reader
above must accept.

### F3 — medium — the format-witness mismatch diagnostic can contradict itself

`src/tree_format.rs:5-6, 21-29`. Two independent constants hold the same value
with different trailing bytes, the comparison is raw-byte, and the diagnostic
prints the found value **trimmed**:

```rust
pub const CURRENT: &str = "session-kinds-v1";
const CURRENT_FILE_CONTENTS: &[u8] = b"session-kinds-v1\n";
…
if contents != CURRENT_FILE_CONTENTS {
    let found = String::from_utf8_lossy(&contents);
    bail!("unsupported Grove tree format {:?} in {}; this binary requires {:?}", found.trim_end(), …, CURRENT);
```

Reproducer: write `.grove/FORMAT` containing `session-kinds-v1` with no
trailing newline (or CRLF), then run any tree verb. Every
`tree_access::{read,write,write_for_promotion}` call
(`src/tree_access.rs:34,44,55`) and `tree_migrate::detect`
(`src/tree_migrate.rs:270`) fails with:

```
unsupported Grove tree format "session-kinds-v1" in …/.grove/FORMAT; this binary requires "session-kinds-v1"
```

— an error that names the same string as both the problem and the requirement,
leaving the operator no way to see what to change. This is reachable from the
spec itself: `docs/specs/config-driven-sessions.md:231-233` shows the required
contents in a fence with no trailing newline, so a hand-authored or
tool-normalised witness lands in the failing state. `tests/session_kind_tree.rs`
covers only the missing witness (137-147) and a *different* value (149-163),
never a whitespace-only difference.

Contract threatened: `docs/specs/config-driven-sessions.md:228-233, 240-242` —
"Ordinary current-format readers require the known value, while an unknown
value stops with an upgrade diagnostic"; here the diagnostic does not identify
an unknown value. Secondary risk: `CURRENT` and `CURRENT_FILE_CONTENTS` are
hand-kept in sync rather than derived, so a future edit to one silently
mismatches the other.

### F4 — medium — longest-kind matching is inert, so the test that names it proves nothing, and the invariant the split really depends on is unasserted

`src/leaf.rs:325-339`; `tests/session_kind_tree.rs:50-64`;
`docs/specs/config-driven-sessions.md:253-256`.

No member of `Kind::ALL` is another member followed by `-`. Checking every
label against the set (`src/leaf.rs:293-315`): nothing starts with
`requirements-`, `design-`, `planning-`, `prototype-`, `impl-`,
`combine-research-`, `finish-`, any `review-<producer>-`, or any
`integrate-review-<producer>-`; `research-a`/`research-b` share a
`research-` stem, but bare `research` is *not* a kind (it exists only as a
migration alias, `src/leaf.rs:285`). So `max_by_key(label().len())` never
breaks a real tie — `split_filename_prefix` yields at most one candidate for
every input, and `filename_kind_uses_the_longest_known_prefix_and_body_routing_metadata_is_ignored`
(`tests/session_kind_tree.rs:51`, input
`01-integrate-review-requirements-review-notes-k7.md`) passes identically under
a first-match implementation. The Done-when's "Tests cover longest matching" is
not met by that test.

The spec's justification is also wrong as written: "This is necessary because
`review-design` and `design`, for example, can both prefix a slug"
(`docs/specs/config-driven-sessions.md:254-255`). `design` does not prefix
`review-design-x`; prefixing is anchored at the start of `<kind>-<slug>`, so
those two labels can never both match the same name.

And longest-match would not rescue a genuine collision if one were added:
with `research` restored beside `research-a`, `01-research-a-foo-k1.md` would
parse as `research-a`/`foo`, so a `research` leaf with slug `a-foo` renders to
a name that parses back to a different entity — `Entry::name`/`parse` stop
round-tripping (`src/tree_id.rs:172-205, 236-264`), and the inline round-trip
tests (`src/tree_id.rs:780-821`) would not catch it because they only exercise
`Kind::Impl`.

Suggested direction: assert the real invariant in `src/leaf.rs`'s inline tests
— for every ordered pair of labels, neither is the other plus `-` — so adding a
twentieth kind that collides fails at compile-test time rather than corrupting
names; correct the spec sentence to state the non-prefix invariant instead of a
false example.

### F5 — low — dead legacy body readers, with doc comments that now describe behaviour the branch removed and two broken intra-doc links

`src/tree_read.rs:268-315` (`read_harness`) and `src/tree_read.rs:317-369`
(`read_legacy_kind`) have no callers anywhere: `grep -rn
"read_legacy_kind\|read_harness" src tests` returns only their definitions and
prose. That part is correct and matches the Done-when ("without reading
`Kind`, `Harness`, or `Producer launch` from current task bodies"), but the
surviving documentation asserts the opposite contract:

- `src/tree_read.rs:330-331`: "`leaf_decompose` reuses this to inherit a leaf's
  kind, so the same degrade applies there." False — `leaf_decompose` reads the
  filename (`src/tree_lifecycle.rs:114-142`) and the spec forbids read-side
  degradation (`docs/specs/config-driven-sessions.md:262-263`).
- `src/leaf.rs:259-260`: "see [`Kind::parse_read`] and its caller
  `tree_read::read_kind`" — `read_kind` does not exist.
- `src/tree_read.rs:272` and `:279` use `[read_kind]` as rustdoc intra-doc
  links to that same missing item; both are broken links. The producer's
  evidence records `cargo fmt --check` and `cargo test --locked` only, so
  `cargo doc` was not exercised and these would not have surfaced.

Also unmet in the same Done-when clause ("The legacy adapter needed by
migration remains isolated and **explicitly tested**"): the
`"research"` → `Kind::ResearchA` alias (`src/leaf.rs:285`), which
`docs/specs/config-driven-sessions.md` and the root brief both call out as the
standalone-legacy mapping, has no test. `src/leaf.rs`'s inline suite covers
`work` → `impl` (527-536) and every current label (459-464) but never
`Kind::parse_read("research")`; `read_legacy_kind` and `read_harness` have no
tests at all.

### F6 — low — the producer-receipt write path is now unreachable while the shipped methodology still instructs it

`prepare_producer_receipts` (`src/task_relationship.rs:515`) and
`review_evidence_unlocked` (`src/task_relationship.rs:212`) have no callers:
`grep -rn "prepare_producer_receipts\|review_evidence_unlocked" src tests`
returns only the definitions. So `leaf-retire` no longer writes a
`Producer launch` receipt, and `tree_read::launch_peek` hard-codes
`review: None` (`src/tree_read.rs:221-222`) while `loop_driver`'s
review/routed-kind cross-check was deleted (diff, `src/loop_driver.rs`).

That is consistent with this slice's Context ("launch harness metadata and
producer receipts are not composition relationships") and with
`legacy-review-removal-k47` owning the contraction, so it is **not** a defect
here. Recording it as a sequencing hazard for the integration step: the
provisioned methodology still describes the receipt as a retirement side effect
(`content/SKILL.md`, `content/TASK-FORMAT.md`, `content/driving.md`; the
installed skill at `SKILL.md:265` and `:335`), and live task bodies in this
grove still carry `**Producer launch:**` lines — including this one. Binary and
shipped skill now disagree; `review-methodology-k87` and
`review-receipt-removal-k84` should not both assume the other closed it.

### Verified without finding

- **Body-field fallback is gone.** `kind`, `pick`, `resolve`, `brief-chain`,
  `leaf-decompose` inheritance, promotion's producer-kind read, and every
  mutator derive kind from the filename only
  (`src/tree_read.rs:177-183, 251-266`; `src/tree_lifecycle.rs:114-142`;
  `src/tree_promotion.rs:590-599`). `tests/session_kind_tree.rs:51-64` pins
  that a body `**Kind:** impl` / `**Harness:** codex` is ignored.
- **Finish reservation is complete on every operand and every creator.**
  `leaf-add` (`src/tree_grow.rs:47`), `leaf-insert`
  (`src/tree_grow.rs:365`), `leaf-add-chain` (via
  `review_steps_or_refuse`, `src/leaf.rs:232-234`), `leaf-decompose` both as
  operand and as `--kind` (`src/tree_lifecycle.rs:139-145`), `leaf-retire`
  (`:195`), `leaf-prune` single and subtree (`:311`, `:454`), and
  `leaf-promote-chain` (`src/tree_promotion.rs:630`). `leaf-add-pair`'s kinds
  are fixed. Covered end-to-end by
  `every_agent_side_mutation_refuses_the_driver_reserved_finish_kind`.
- **Finish cannot starve non-finish work.** `pick_unlocked` collects the whole
  DFS pre-order live set before choosing, so a non-finish leaf appended *after*
  a finish leaf still wins (`src/tree_read.rs:52-76`), duplicates bail, and a
  lone finish is returned. Matches
  `docs/specs/config-driven-sessions.md:292-299`;
  `non_finish_work_can_be_inserted_before_a_reserved_finish_leaf` and the two
  eligibility tests cover it.
- **Foreign-file lenience survives strictness.** `parse_current`
  (`src/tree_id.rs:270-287`) errors only for a name that is positioned, keyed
  and `.md`; `README.md`, `notes.txt`, `done/`, `PROMOTING-*`, and
  filesystem-kind mismatches stay foreign, and all four walks share one
  reconciliation (`src/tree_read.rs:577-605`, `src/tree_grow.rs:607-631`,
  `src/tree_lifecycle.rs:390-412`, `src/tree_promotion.rs:708-723`).
- **Key monotonicity holds across the rewrite.** `next_key`/`next_keys`
  (`src/tree_id.rs:372-424`) max over live, `DONE`, `ABANDONED` and node names
  from a full-tree scan; `leaf-insert` allocates pre-renumber
  (`src/tree_grow.rs:393`) and `add_run` takes the whole run from one snapshot
  before its first write (`src/tree_grow.rs:150-154`), with the exhaustion
  refusal landing before any mutation.
- **No current/legacy dual reader.** The witness gate is enforced in one place
  for every verb (`src/tree_access.rs:34, 44, 55`); `root-init` writes
  `FORMAT` last (`src/tree_lifecycle.rs:76`); `tree_migrate` is the only
  module that still speaks legacy grammar. The accepted consequence — this
  branch's binary cannot read any pre-`FORMAT` tree, including this grove's own
  `.grove/`, until `session-kind-migration-k27` lands — is the staging
  `legacy-launch-removal-k46`'s brief already records, and the meta-grove
  continues on the installed v16.5.0 driver.

## Notes

The reviewer produces findings only; `session-kind-tree-integrate-k25` owns
fixes and all post-fix verification. F1 and F2 are both in `detect`; fixing F1
by deleting the kind-aware branch and F2 by teaching `parse_legacy_v2_leaf` the
`ABANDONED-` infix are independent one-line-scale changes, each wanting its own
`detect` test. F4 and F5 are the two Done-when clauses this slice does not yet
satisfy ("Tests cover longest matching"; "The legacy adapter … explicitly
tested"). F6 needs no code change here — only sequencing awareness.
