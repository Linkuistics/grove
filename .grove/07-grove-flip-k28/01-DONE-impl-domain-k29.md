# domain-k29

## Goal

grove's implementation of `EntryName` — the whole seam, and the only thing this
increment adds before anything moves onto it. It is consumed by nothing when the
leaf retires: the old tree modules are untouched, grove builds, and the whole
suite passes exactly as it did before.

This is the **expand** stage of the wide refactor. Its deliverable is a
conformant domain and the evidence that it is one.

## Context

- `docs/ordinal-fs-tree/ARCHITECTURE.md` — *The seam: one trait*, and especially
  *What an implementation must guarantee*: seven obligations, six assumed and one
  enforced. The document says a design missing any one of them admits a tree the
  library will quietly corrupt.
- `crates/ordinal-fs-tree/src/conformance.rs` and `tests/conformance_kit.rs` —
  the kit a domain checks itself with. The architecture says a conforming domain
  can check itself against the seam **without reading the architecture**; if that
  turns out to be false, it is a finding about the seam.
- `crates/ordinal-fs-tree/src/reference.rs` — the reference domain, and the
  nearest thing to a worked example. It is *not* a template: its vocabulary is a
  course syllabus.
- `src/tree_id.rs` — prior art, and explicitly not authority. Its `Entry`,
  `parse`, `parse_current`, `sort_key` and `validate_slug` are the grammar this
  domain must reproduce; its **leniency** is the thing this leaf must not.
- `src/leaf.rs` — `Kind` and its `split_filename_prefix` / `label` / `label_list`,
  which the domain's parts carry.
- `src/tree_access.rs` — the three sentinel constants (`MIGRATING-session-kinds`,
  `FINISHING-`, `PREPARING-FINISH-`), which become `Verdict::Reserved`.

## Done when

- A grove `EntryName` implementation exists with `Parts` carrying the session
  kind, the slug and the outcome (live / `DONE` / `ABANDONED`); `Err` carrying
  grove's own recovery advice; `distinguished()` returning `BRIEF.md`; and
  `positioned_species` deciding leaf-vs-node from the parts alone.
- The conformance kit runs green over a fixture covering every shape a real
  `.grove/` holds: a brief, live and terminal leaves, a node directory, `FORMAT`,
  a foreign `README.md`, and each sentinel.
- **Question 2 is answered and recorded.** Either the grammar is canonical and a
  lenient spelling is a refusal that names the canonical form — what the
  reference domain does — or the obligation is knowingly waived and the record
  says what breaks. The node brief carries the corpus fact this rests on: grove
  itself has never written a lenient position, so only a hand edit produces one.
- **Question 4 is answered.** `Cargo.toml`'s dependency line reads
  `default-features = false`, and a test — not a comment — holds the claim that
  the library's imposed dependency set is `libc`.
- grove builds and the whole existing suite passes **unchanged**. Nothing
  consumes this module yet; if something had to change to make it compile, that
  is a finding for the node brief.

## Notes

**The classification is where this leaf can lose data, so it is where the care
goes.** The trichotomy is `Entry` / `Foreign` / `Malformed` / `Reserved`, and
grove already knows its own answers — `tree_id::parse_current` is the shape of
them. The mapping, with the two that are easy to get backwards first:

- `.grove/FORMAT` → **Foreign**. It is grove's own file, but `Reserved` halts,
  and `FORMAT` is present in every healthy tree. Reserved here refuses every
  grove tree.
- `MIGRATING-session-kinds`, `FINISHING-*`, `PREPARING-FINISH-*` → **Reserved**.
  These are exactly the architecture's *transaction witness, lock marker,
  sentinel left by an interrupted operation*, and halting on them is what
  `tree_access::refuse_pending_*` does by hand today.
- A task-shaped name with an unknown or missing kind → **Malformed**, carrying
  the same advice `parse_current` gives now. The root brief's reason still binds:
  a task-shaped name a walk silently skips is lost work, and a whole subtree when
  it is a directory.
- A directory wearing an outcome infix → **Malformed** with its existing
  diagnostic, which is one of the better error messages in the codebase. Keep the
  wording.
- `README.md` and anything else → **Foreign**.

**`parse` takes `Found`, and that is not decoration.** A name declaring a node
over a regular file, or a leaf over a directory, is `Malformed` — grove's own
`tree_id` header already calls a species mismatch *a malformed tree, not a
foreign entry*, so this is grove's rule reaching a place that can enforce it.

**The seventh obligation is the one the library enforces**: a name renders as one
path component. grove's slugs are validated already; make sure the error path is
reachable and says something useful anyway.

**On question 2, the recommendation is to tighten**, and the argument to check
rather than accept: the failure mode of leniency is that one entry occupies two
files sharing a key and an ordinal, which
`structure.als`'s `witness_two_filenames_name_one_entry` is a picture of, and
every invariant downstream assumes it cannot happen. The cost is that a
hand-typed `5-` makes a whole tree unreadable until it is renamed — so the
refusal must name the canonical spelling, not merely report a parse failure.
Canonical means *zero-padded to at least two digits, no other leading zero*:
`05` and `100` both canonical, `5` and `005` not.

**Do not transcribe `tree_id.rs`.** It is the grammar's best description and its
worst implementation for this purpose — lenient by design, and shaped around a
per-directory reader rather than a name type that owns its own parsing.
