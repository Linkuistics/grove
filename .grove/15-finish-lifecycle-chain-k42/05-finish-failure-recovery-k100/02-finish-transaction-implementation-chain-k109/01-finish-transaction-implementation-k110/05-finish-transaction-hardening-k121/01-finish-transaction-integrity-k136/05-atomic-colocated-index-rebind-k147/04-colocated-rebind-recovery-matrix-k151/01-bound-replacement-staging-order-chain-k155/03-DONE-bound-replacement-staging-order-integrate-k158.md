# bound-replacement-staging-order-integrate-k158

**Kind:** integrate-review-impl
**Integrates:** bound-replacement-staging-order-review-k157

## Goal

Apply the verified findings from `bound-replacement-staging-order-review-k157`
while preserving the reviewed artifact's contract.

## Done when

- Every accepted finding is fixed, each first demonstrated by a test that fails
  against the reviewed producer commit; every rejected finding is answered here
  with its reason.
- The publication order still leaves no entry that no live document describes,
  and nothing deletes or moves an entry whose identity it has not proven.
- `cargo fmt --check` and `cargo test --locked` pass from the final tree.

## Dispositions

Three tests were written first and all three failed against reviewed producer
commit `fafa7b35`:
`recovery_refuses_a_replacement_state_that_adopts_a_foreign_staged_marker`,
`the_staged_marker_is_named_in_this_attempts_reserved_namespace`, and
`a_failed_replacement_refuses_to_dispose_through_a_stale_snapshot`.

### R2 — arbitrary staged marker name — accepted, fixed

The staged marker is no longer a generic `.grove-cleanup-<nonce>.tmp`
temporary. `publish_marker_replacement` now draws it through the same
`create_staging_entry` primitive the staged artifact uses, so it is
`<canonical-marker-name>.staging-<32 hex>`, and `validate_state` refuses any
`staged_name` outside that namespace. This is the finding that actually breached
the protocol's stated bound, and fixing it is what makes R1's remaining claim
true.

### R1 — shape-valid substituted artifact — accepted in part

Accepted: the producer's stated property — "a forged document can only name
entries its author created" — is false, and the doc comment asserting it is
corrected. It is restated as the property that does hold and is now enforced on
both staged names: every entry a rewritten state document can aim the exchange
at lies inside this auxiliary's own reserved role-and-attempt staging namespace.

Rejected: the demand that a rewritten state document be unable to name *any*
entry it did not create. No protocol whose entire authority lives in one
directory can meet it. Every anchor a cold recovery has — the state document, the
canonical marker, the staged marker, the recorded inodes — is in that directory
and is mutually self-certifying, so a writer who can rewrite one can rewrite the
set consistently. The exact-name pin the review compares against did not achieve
the property either; it fixed the name, and the price was
`reclaim_unbound_replacement` unlinking a foreign entry at that name **with no
forgery at all**, on an ordinary path. That is the capability this chain exists
to remove, and it is strictly worse than a redirection that requires write access
to the VCS administration directory — where the Git index the auxiliary protects
already sits, directly writable by the same hand. Recorded as a rejected
alternative on ADR `task-tree-transactions-fail-closed` and in the spec, with the
reopen condition: a durable ownership witness such a writer cannot forge.

### R3 — stale-snapshot disposal — accepted, fixed

`dispose` and `activate` now fail closed while a replacement state document is
present, naming it in the diagnostic. Recovery re-reads the auxiliary from disk
and settles the replacement before returning a value, so it never meets the
guard; only a caller holding a pre-replacement snapshot does — which is exactly
the production sequence the review traced. The synchronous failure now leaves the
canonical pair and the state document intact and recoverable, at the cost of one
warning and a later lease-owned reap instead of an immediate discard.

### R4 — residual unowned-entry windows — accepted in part, externalized

The parseable-ownership half is fixed by R2's change: both staged entries now
carry the role and attempt in their names, so a death before the state document
leaves an attributable entry rather than an anonymous temporary. The windows
themselves cannot be closed by reordering — a document cannot record an inode
before the entry exists — so the remaining gap is that nothing *reaps* those
attributable leftovers. That is a reaper concern whose whole difficulty is
finding a disposition that does not reintroduce unproven removal, so it is
`reap-attributable-staging-leftovers-k161` rather than a widening of this
integration.
