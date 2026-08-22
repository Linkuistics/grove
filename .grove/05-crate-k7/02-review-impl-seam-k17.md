# seam-k17

**Reviews:** seam-k8

## Goal

An adversarial read of the crate seam — `crates/ordinal-fs-tree/`, as
`seam-k8`'s commit left it — before eight leaves are written against it. Produce
findings; change nothing.

This leaf was **inserted** ahead of its siblings rather than appended, which is
the departure from the usual `leaf-add` worth knowing about. A `review-*` step
normally does not care where it lands because it re-derives its citations from
the producer's commit. This one cares for a different reason: the trait *is* the
library's public surface, every remaining leaf in `crate-k7` is written against
it, and a finding delivered after `reading`, `interpreter`, `insert`, `promote`,
`rewrite` and the CLI have all been built on it is a rework of the subtree rather
than a correction.

## Context

Beyond the brief chain, and beyond `crate-k7`'s own *Read first* list:

- `crates/ordinal-fs-tree/` in full — it is one manifest, three source files and
  three test files, and small enough to read whole.
- `docs/formalism-findings.md` entry **004**, which is `seam-k8`'s own account of
  what it did and what it could not check. Read it before the code: it names the
  deviation below, and it names the one hazard the leaf could not close (nothing
  verifies that `src/name.rs` and `ARCHITECTURE.md`'s trait block say the same
  thing).
- `docs/adr/entry-name-is-the-only-seam.md`, whose *Why this is hard to reverse*
  section is the reason this review exists at all.

## Done when

Each doubt below has a verdict, and any finding is written where the integrating
session can act on it — the file, the line, and what is actually wrong.

**Doubt 1 — the trait deviates from the architecture document, deliberately, and
that judgement is the main thing to attack.** The document stated three `Option`
accessors, `ordinal()` / `key()` / `parts()`, with the obligation that they are
`Some` together or `None` together. `seam-k8` replaced them with one method,
`fn triple(&self) -> Option<Triple<'_, Self::Parts>>`, on the ground that this
makes the obligation unrepresentable rather than checkable — entry 002's
counterfactual, applied at the seam rather than in a domain. It then reconciled
`ARCHITECTURE.md`, `structure.als` and the conformance kit to match. Three ways
that could be wrong, and a reviewer should try all three:

- **It buys less than it claims.** A name that is positioned *and* claims species
  `Distinguished` is still representable; only *neither* was closed. Is the
  remaining half checked where the code says it is, and is the saving worth a
  deviation from the document at all?
- **It costs something not noticed.** Every consumer of the algebra now reaches
  an ordinal through `triple()`, and `Triple` borrows its `Parts`. Does that
  borrow survive contact with `reading`, `insert` and `promote` — a snapshot that
  owns its names, and an algebra that wants a `Parts` to hand to `compose`? A
  seam that forces a clone at every call site is worse than three accessors.
- **The reconciliation is incomplete.** Four artifacts state this trait —
  `ARCHITECTURE.md`, `structure.als`, `src/name.rs`, `src/conformance.rs` — and
  nothing checks that they agree. Do they?

**Doubt 2 — the conformance kit's coverage accounting is invented, not derived.**
`Report` reports two kinds of finding: a violated obligation and an *unexercised*
one, and `is_conforming()` is false while any obligation was never reached. The
rule for what counts as exercising an obligation was written by hand, per
obligation, in `check`. `distinguished()`'s own name is deliberately excluded
from coverage for the `parse`-refuses obligation while still being checked, which
is a subtle rule that a later edit could silently invert. Is each rule right? Is
there a sample set that passes while checking nothing?

**Doubt 3 — the no-filesystem test is a textual scan.** It strips comments and
looks for `fs::` and `std::fs` in every file under `src/` outside `src/fs/`,
which does not exist yet. Known holes are documented in the test's own header
(`use std::fs as f;` defeats it). The scan's coverage control asserts four named
files are reached. Two questions: does the mechanism survive `src/fs/` actually
existing — the leaf that creates it is next — and is a stronger mechanism
available for the same money?

**Doubt 4 — the reference domain's grammar is the shared fixture, so its
mistakes propagate.** Every later test and the CLI use it. Two judgement calls to
attack: a name with a position and a key but no label (`01--i3.md`) is `Foreign`
rather than `Malformed`, which is a Foreign/Malformed line-drawing decision the
architecture document does not settle; and the canonicity obligation is
discharged by a single re-render comparison at the end of `parse` rather than by
field-level rules, which is either elegant or a place where a real defect hides
behind a passing round trip.

**Doubt 5 — the workspace change touched grove.** The root `Cargo.toml` gained
`[workspace]`, `[workspace.lints]` and `resolver = "2"`, and grove's own
`[lints.clippy]` / `[lints.rust]` tables moved there and were replaced with
`[lints] workspace = true`. `seam-k8` confirmed grove's suite green afterwards
(44 test binaries, no failures) and clippy clean for both crates. Is anything
else in the repo entitled to an opinion about the manifest — `release.toml`,
`scripts/`, `docs/RELEASING.md`, the Homebrew lane — that a green test run would
not have noticed?

## Notes

**Verify, do not re-derive.** The models are green and were re-run in this
working tree at the start of `seam-k8`; re-running them is cheap
(`docs/ordinal-fs-tree/models/run-alloy.sh`, `run-quint.sh`, about two minutes
together) and is worth doing once, because both tools report *found nothing* and
*succeeded* the same way and the witnesses are what tell the two apart.

**Produce findings, not fixes.** If there is nothing worth acting on, create
nothing and retire — that empty integration is exactly what the chain's laziness
exists to remove.
