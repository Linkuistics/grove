# reading-k20

**Integrates:** reading-k19

## Goal

Integrate every actionable finding from `reading-k19` before the mutation leaves
build on the reading layer. Repair the lock identity, filesystem-boundary guard,
walk-order control and public snapshot builder seam; then run all post-fix
verification. This session owns fixes — the review changed no production or
test code and ran no verification commands.

## Context

Beyond the brief chain and its *Read first* list:

- `reading-k19` in full, especially its four findings and five doubt verdicts.
  The cited `path:line` coordinates are against `reading-k9`'s commit `843c4470`.
- `reading-k9`, the reviewed producer and its recorded verification evidence.
- `docs/formalism-findings.md` entries 006 and 007: the producer's mutation
  controls and the review's counterfactual that each control varied only one
  semantic dimension.
- `docs/ordinal-fs-tree/ARCHITECTURE.md` on invisible locking, caller-spelled
  paths and total walk order. Preserve caller spelling; changing a report path
  is not an acceptable way to make lock identity converge.

## Done when

- Direct and accepted roundabout spellings of one existing tree contend on the
  same lock. Cover at least the producer's own `root/child/..` spelling and a
  final-component symlink. The reported root remains byte-for-byte the caller's
  spelling. If some aliases must instead be refused, reconcile the architecture
  and error surface explicitly rather than silently narrowing the contract.
- The no-filesystem guard cannot mistake comment delimiters inside ordinary,
  raw, byte or character literals for comments and thereby hide later Rust
  code. Prefer a syntax-aware dependency check if that is no more surface than
  maintaining a partial lexer; whichever mechanism remains has positive and
  coverage controls for its exact limits.
- The walk-order fixture distinguishes key-before-name from name-before-key:
  smaller key and lexically earlier rendered name point in opposite directions,
  while a separate equal-key pair holds the rendered-name final tie-break.
- A non-root `Place` from one `Builder` cannot silently name a node in another.
  The public contract and implementation agree: foreign places are either
  unrepresentable or deterministically rejected, with a same-index-node control
  for the formerly silent case.
- Reassess `Builder`/`Place` as a public construction seam while fixing it. Keep
  it public only if consumers, rather than this crate's integration tests alone,
  earn that interface; do not widen the production surface merely to preserve a
  test arrangement.
- The lock/path documentation, boundary-test header, mutation-control claims and
  `docs/formalism-findings.md` describe what the repaired code actually proves.
- Both model runners, the crate and grove test suites, formatting, and workspace
  clippy are green after integration. Append this episode to
  `docs/formalism-findings.md` before retiring.

## Notes

The review accepted three judgements that are not work for this leaf. The
`(ordinal, key, rendered name)` comparator itself is sound; the defect is its
control. Halting on non-UTF-8 is the sound side of the current `&str` parse seam.
An `Entry` borrow ending before a `WriteGuard` mutation is correct invalidation,
not a reason to add interior mutability or let stale handles survive a refresh.

The graph service could not index the review workspace because daemon
coordination was unavailable in the sandbox. Re-establish graph coverage if the
integration environment permits it, but treat `reading-k19`'s directly read
source coordinates as the handoff rather than re-deriving the findings from an
empty index.
