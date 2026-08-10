# bound-replacement-staging-order-review-k157

**Kind:** review-impl
**Reviews:** bound-replacement-staging-order-k156

## Goal

Adversarially review `bound-replacement-staging-order-k156` and record concrete
findings for its integration step.

## Context

- Inspection-only by default: read the producer commit and reason about it.
  Running the existing suite is fine; if disproving a claim needs a temporary
  in-tree probe, disclose it under a `## Verification run` heading and leave the
  working tree byte-identical to the reviewed producer commit.
- Record findings only. `bound-replacement-staging-order-integrate-k158` owns
  every fix and all post-fix verification.
- The claim under test is that the inverted publication order removes the
  unowned-entry state entirely rather than merely narrowing it. Attack that
  directly: enumerate every interruption point in the new sequence and ask what
  is on disk with no live document describing it, and what a later recovery,
  disposal, activation, or same-attempt retry then does with it.
- Also attack the new staging namespace as an authority. The exact-name pin the
  previous review won for `staged_artifact_name` became a **shape** pin, so ask
  what the strongest reachable redirection is now, and whether the producer's
  argument for that weakening — that a forged document can only name entries its
  author created — actually holds on every path that reads it.
- Two adjacent questions the producer saw and deliberately left alone, both fair
  game: `staged_name`, the staged *marker*, still has no namespace pin at all and
  is gated only by parsing as a Grove marker; and `dispose` on a struct built
  before a mid-settle failure may act on a stale marker snapshot.

## Done when

- Findings are recorded here with severity and concrete source or diff evidence,
  or an explicit no-finding result.
- The working tree is left byte-identical to the reviewed producer commit.

## Findings

### R1 — A shape-valid substituted artifact is adopted and can be deleted — **High**

`validate_state` replaced the former exact-name check with
`is_staging_replacement_name` (`src/finish_cleanup/auxiliary/marker_replacement.rs:807-823`).
That proves only `<role-and-attempt>.staging-<32 hex>` syntax. The replacement
inode and the staged marker snapshot that are supposed to authorize the name
come from the same mutable state document, so an in-place rewrite can make all
three agree about an external regular file already at any shape-valid name.
`settle_artifact_exchange` then exchanges that file into the canonical artifact
name (`marker_replacement.rs:267-300`), and normal `dispose` subsequently
unlinks it (`src/finish_cleanup/auxiliary.rs:641-700`).

The review probe preserved the real staged artifact, put external bytes at its
shape-valid name, and rewrote the staged marker and state document in place to
describe that inode. Recovery succeeded, moved the external inode to the
canonical artifact name, and `dispose` deleted it. This directly disproves the
producer's claim that a forged document can name only entries its author
created, and violates the subtree's no-external-byte-deletion contract. The
existing redirect test covers only `victim-index`, which fails the shape check
(`src/finish_cleanup/auxiliary/tests.rs:322-370`), so it does not exercise the
strongest accepted redirection.

### R2 — `staged_name` lets the state document adopt an arbitrary marker entry — **High**

`validate_state` applies only generic component validation to `staged_name`
(`marker_replacement.rs:807-810`); unlike the canonical marker, state document,
artifact, and even the weakened staged-artifact check, it has no independent
namespace or exact-name constraint. `validate_artifact_exchange_authority` and
`classify_phase` consequently open the state-selected name
(`marker_replacement.rs:331-351,585-632`), and settlement exchanges it into the
canonical marker before later removing it through normal cleanup.

The review probe copied the valid staged-marker bytes into an arbitrary
`external-user-marker` entry, preserved the Grove-created staged marker, and
rewrote the state document in place with the arbitrary name and inode. Recovery
adopted that external inode as the canonical marker and `dispose` deleted it.
Parsing as a Grove marker is therefore not ownership evidence; this path can
move and remove a substituted entry outside every Grove temporary-marker
namespace.

### R3 — A mid-settle error invokes `dispose` through a stale marker snapshot — **High**

After the replacement state is published, `replace_artifact_from_with` can
return an error before the artifact exchange completes
(`src/finish_cleanup/auxiliary.rs:482-495`). The production caller propagates
that error and immediately calls `discard_temporary_index(&success_index)`
(`src/repo/finish_commit.rs:1071-1080`), but `success_index` still contains the
pre-replacement marker snapshot. While the canonical marker and artifact are
still in the pre-exchange phase, stale `dispose` revalidates both successfully
and removes them (`src/finish_cleanup/auxiliary.rs:641-700`). What remains is a
replacement state plus staged marker/artifact but no canonical marker;
`recover_auxiliary` then refuses that exact shape as Recovery pending
(`src/finish_cleanup/auxiliary.rs:247-263`).

The review probe injected the existing `AfterStatePublication` checkpoint
failure and then followed the production disposal call. Disposal returned
success, removed the canonical artifact and marker, and left the `.replacing`
document behind. This turns a synchronous pre-exchange failure into a wedged
recovery state and destroys the evidence settlement needs, rather than rolling
back or recovering forward.

### R4 — The new order narrows, but does not remove, unowned-entry windows — **Medium**

`replace_artifact_from_with` creates and fills the drawn staging artifact before
`bind_artifact_replacement` publishes any state
(`src/finish_cleanup/auxiliary.rs:458-493`). Inside that bind,
`publish_marker_replacement` likewise creates and writes a randomly named staged
marker before publishing the state document
(`src/finish_cleanup/auxiliary/marker_replacement.rs:81-129`). Process death in
either interval leaves a full Git-index copy or marker that no live document
names. Recovery and auxiliary discovery inspect only the deterministic artifact,
marker, and state names (`src/finish_cleanup/auxiliary.rs:225-289`), so these
entries are never validated or reaped.

The producer's own test explicitly leaves such a staged artifact in place and
asserts that it survives recovery/disposal
(`src/finish_cleanup/auxiliary/tests.rs:407-462`); the retired producer note also
accepts the leak. Random naming avoids blocking a same-attempt retry, but it does
not satisfy the parent requirement that every interruption boundary have
durable, parseable same-attempt ownership, nor the review claim that the
unowned-entry state disappeared entirely.

## Verification run

- Reviewed jj commit `fafa7b35` (`bound-replacement-staging-order-k156`).
- `cargo test --locked finish_cleanup::auxiliary` passed all 34 focused tests.
- Added three temporary `probe_*` unit tests for R1-R3 and ran
  `cargo test --locked probe_ -- --nocapture`; all three probes passed (the
  filter also ran one unrelated existing probe test).
- Removed the temporary probes. `jj st` then reported a clean working-copy child
  byte-identical to the producer before this review file was edited; no source
  or test probe remains in the review change.
