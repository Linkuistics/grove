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
`02-design--decomposition-k2.md` ambiguous — kind `design` + slug `decomposition`
and kind `design-decomposition` + empty slug are both readable — and the handle
is what differs between the readings.

## Decisions (running log)

- **`Slug`, `Kind`, `Outcome`, `Parts`, `TaskName` and `TaskNameError` already
  existed** with validating constructors, from the `EntryName` flip. Re-derived
  rather than inherited from the brief: the only missing type was `Handle`, and
  the only missing *structure* was the single spelling. So this leaf is smaller
  than its `Done when` reads and adds no type the spec did not name.

- **`Handle::render` takes `(&mut Formatter, &Slug, Key)`, not `&self`.** A
  `Display for Handle` that `TaskName`'s renderings called would clone a `Slug`
  every time, and `TaskName::to_string()` runs on *every parse* — canonicity is
  checked by re-rendering. The point of the requirement is that there is one
  `write!`, not that a `Handle` value has to exist to reach it. `Display for
  Handle` delegates to the same function.

- **The peel is shared, the judgement is not.** `peel_key` returns the key digits
  unparsed, because its two callers disagree about an over-wide key: a filename
  says `NotCanonical` (rename it to this), a handle says `KeyOutOfRange` (no tree
  allocated that). Sharing the mechanism without sharing the vocabulary is what
  lets one peel serve both — and `task_tree::handle_key`, the second peel, is
  deleted rather than reimplemented.

- **`SelectedLeaf::handle` is a `Handle`, not a `String`.** The handle is the
  value that crosses every module boundary, so leaving it as text at the widest
  boundary would have left the type owning the grammar and nothing owning the
  value. `prompt::compose` still takes `&str`: it substitutes text and holds no
  grammar, so it takes what the handle renders to.

- **`finish-commit` reads its argument through `Handle::parse`.** It used to
  compare strings, so `grove-llm finish-commit nonsense` was reported as a
  *mismatch* with the live finish leaf. It is now a refusal that says what a
  handle is and names the one to rerun with (principle 2 — the advice is part of
  the error).

- **No ADR.** `docs/specs/module-decomposition.md`'s `## ADR reconciliation`
  accounts for every record and assigns none to this leaf; decision 4 already
  carries the argument, and `task-names-are-canonical` is amended at
  `grammar-separator-k15`. Adding an unassigned record here would put
  `spec-to-current-state-k23`'s checklist out of step with the table it walks.

- **Not a cutover leaf**, re-derived by k6's matrix rather than inherited: no
  name on disk changes and no `.grove/` entry gains or loses a shape, so there
  is no cell where the installed 19.3.0 meets the tree this leaf leaves and
  fails. Nothing is deployed and the session signals normally.

- **`Kind` keeps its closed set**, as the brief requires: opening it before the
  separator lands makes `02-design--decomposition-k2.md` two readable names.

- **`Handle::parse` is lenient on the key and strict on the slug**, and the
  asymmetry is deliberate in both directions. Lenient: `a-k007` is key 7, as
  `handle_key` had it — a handle is a *reference* a human types, never a name on
  disk, so `task-names-are-canonical`'s argument (two spellings of one filename
  are two files sharing one entry) does not reach it, and `parse_ref` is already
  lenient beside it. Strict: `-k3`, `A-k3` and `DONE-k3` used to resolve to key
  3 and now do not, because `handle_key` never looked at the slug. Nothing is
  lost — every slug on disk went through `Slug::new`, so the old answer was a key
  lookup wearing a slug that could not belong to the entry it matched. Both are
  pinned by `parse_is_lenient_on_the_key_and_strict_on_the_slug` and the second
  is in `CHANGELOG.md`, since it is user-visible at `resolve`.

- **The single-spelling claim is verified by two complementary enumerations, not
  a pattern list**, because either alone reads clean while the other is dirty.
  A production spelling must *either* use the private `KEY_MARK` constant *or*
  write a fresh `-k` literal, and nothing else is expressible — so:
  - every occurrence of `KEY_MARK` across `src/`, `crates/` and `tests/`: three,
    all in `src/task_name.rs` — the definition, `Handle::render`'s one `write!`,
    and `peel_key`'s one `strip_suffix`. `KEY_MARK` is **private**, so no other
    module can reach it.
  - every string literal containing `-k` adjacent to an interpolation hole,
    everywhere: the survivors are `--kind` and `duplicate-key` (the substring, not
    the grammar) and seven **test** fixtures that spell a filename to pin the
    production renderer — legitimately independent, since a test that reused the
    renderer would assert nothing.
  With a positive control: the same pattern finds all three of the produce sites
  this leaf deleted, so it is not a broken instrument reading clean. It does
  *not* find `Handle::render` — which uses the constant, not a literal — and that
  blind spot is precisely why the first enumeration exists beside it.

- **The leaf's one in-session reviewer was spent on the single-spelling claim**
  (`references/execute.md`), and it came back with three findings against the
  *behaviour* claim rather than against the structural one. Classified:

  - **Valid and actionable — `finish-commit` widened acceptance and wrote the raw
    argument into the permanent record.** `Handle::parse` is not canonicity-checked,
    so `finish-k0001` equals the live `finish-k1` where the old string comparison
    refused it — and `delete_and_commit` was still handed the *raw text*, so the
    teardown commit subject read `finish-k0001: remove completed grove task tree`,
    a handle no name on disk ever wore. The widening is kept, deliberately: an
    operator who typed that meant that leaf, and it matches `parse_ref`'s existing
    leniency. What is fixed is the record — `selection.handle` is what goes past
    the guard, and the raw text goes no further than the refusals quoting it back.
    Pinned by `a_lenient_key_spelling_is_accepted_and_committed_canonically`.
  - **Valid and actionable — the mismatch message canonicalised the operator's
    own argument**, reporting `other-k7` at someone who wrote `other-k007` and
    removing the one half they could act on. Restored to the raw text; pinned by
    `a_refused_handle_is_quoted_as_the_operator_wrote_it`.
  - **Valid and actionable, and the one that mattered — routing `resolve`'s
    fallback through `Handle::parse` narrowed what `resolve` accepts.**
    `handle_key` never looked at the slug, so pasting a retired leaf's whole stem
    (`01-DONE-impl--build-k5`) resolved by key 5; under `Handle::parse` it does
    not, because that head is not a slug. **A contract I stated to myself
    unclearly**: `lookup` wants *does this end in a key*, not *is this a handle*,
    and I conflated them. Fixed by asking the owner the narrower question —
    `task_name::terminal_key`, which wraps the same `peel_key`, so there is still
    one peel and two questions asked of it. Behaviour is now identical to
    `handle_key`'s, the CHANGELOG's disclosure of a tightening is withdrawn
    because there is no longer one, and `resolve_reads_a_terminal_key_whatever_
    precedes_it` pins the rows that were lost.
  - **A contract stated unclearly, corrected in the docs, not the code** — the
    claim *taken apart in exactly one place* is true of production code, not of
    the crate: `tests/session_kind_guidance.rs`'s doc scanner has a looser
    `rsplit_once("-k")` of its own. It produces no name and pins nothing this
    grammar renders, so it stays; `docs/ARCHITECTURE.md`'s wording now says
    *in production code*.
  - **Noise, and one confirmation.** The byte-index safety of `peel_key` on
    non-ASCII input survived an attempt to break it (an ASCII digit is never a
    UTF-8 continuation byte, so the boundary before a trailing digit run is
    always a char boundary). Three stale doc lines the reviewer noticed in
    passing are fixed because they are cheap and unambiguous and this leaf edited
    their neighbourhood: `docs/ARCHITECTURE.md`'s module table listed `repo`,
    `finish_transaction` and `finish_cleanup`, none of which exist, and
    `crates/ordinal-fs-tree/src/reference.rs` still contrasted its canonicity
    with a lenient grove that has not existed since the `EntryName` flip. The
    surviving `finish_transaction::preflight_root` reference in prose at
    `docs/ARCHITECTURE.md`'s finish-verb paragraph needs an author who knows what
    `delete-finish-transaction-k8` replaced it with, so it is left for
    `spec-to-current-state-k23`'s walk rather than guessed at here.

  **No `review-name-ownership-k14` leaf is cut.** Every fix above is mechanical
  and conclusively covered by an executable test seam, which
  `references/execute.md` names as not forcing one.
