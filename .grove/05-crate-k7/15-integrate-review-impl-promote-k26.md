# promote-k26

**Integrates:** promote-k25

## Goal

Close the equality/species gap found by `promote-k25`: a domain permitted by
the current `EntryName` contract can make leaf and node parts compare equal
while `positioned_species` distinguishes them, causing `Plan::guarded` to refuse
a valid promotion as `DestinationOccupied`.

## Context

Beyond the brief chain and its *Read first* list:

- `promote-k25` in full. Its coordinates are against producer commit
  `fd5a7567`; this integration is adjacent, so no implementation leaf has moved
  them.
- The false claim at `crates/ordinal-fs-tree/src/ops.rs:370`–375 and the view
  comparisons at `src/plan.rs:216` and 222. The promoted node deliberately
  reuses the leaf's ordinal and key, so `Parts::Eq` is the only discriminator.
- The seam contract at `src/name.rs`, especially `Parts: Clone + Eq`, the claim
  that `positioned_species` discharges species-following-from-parts, and the
  name/triple isomorphism; the conformance checks beginning at
  `src/conformance.rs:293`; `structure.als`'s `SpeciesFromParts`; and
  `docs/adr/entry-name-is-the-only-seam.md`.
- The adversarial domain is small: two parts values whose equality deliberately
  ignores a leaf/node discriminator while `positioned_species` reads it. This
  remains a lawful equivalence relation and passes every current conformance
  check, which is why a reference-domain-only regression cannot close the gap.

## Done when

- The public contract and implementation agree on what makes two positioned
  names identical. A valid promotion cannot be refused merely because its leaf
  and node parts compare equal under a lawful domain `Eq` implementation.
- Choose the correction at the seam rather than special-casing `promote`: either
  make positioned species part of the identity occupancy compares, or state and
  check an explicit congruence obligation that equal parts imply equal species.
  Reconcile the architecture, trait docs, conformance kit, obligation count and
  ADR/model commentary wherever that choice changes their claims.
- An adversarial domain reproduces the reviewed failure before the fix. After
  it, the promotion proceeds through the algebra and public on-disk surface, or
  the conformance kit rejects that domain with actionable advice before it has a
  tree. Include a mutation control that proves the new test reaches the chosen
  mechanism.
- Re-read occupancy's other comparison at `src/plan.rs:222` under the same rule
  so arrived effects and snapshot entries cannot disagree about name identity.
- Re-run both model suites, the crate and grove test suites, formatting and
  workspace clippy. Append this episode to `docs/formalism-findings.md` before
  retiring.

## Notes

The review accepted the other five doubts: the transient-duplicate test's
structural proof, `level_of`, the duplicated test-only `Contentless` domain, the
model's refusal priority, and both architecture corrections. Do not widen this
leaf into those settled questions.

Codebase-memory could not index the review workspace because active-daemon
coordination was unavailable, and its current-workspace coverage call was
blocked by the never-approve harness policy. Retry graph coverage if this
session permits it, but treat `promote-k25`'s complete direct-source citations
as the authoritative handoff rather than re-deriving the finding from the stale
base-repository graph.
