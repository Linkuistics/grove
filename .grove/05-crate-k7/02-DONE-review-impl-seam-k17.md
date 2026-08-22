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

## Findings

### High — the supposedly discharged name-shape obligation is still representable

`crates/ordinal-fs-tree/src/name.rs:356` and
`crates/ordinal-fs-tree/src/conformance.rs:36` claim that one
`Option<Triple>` makes *a name is positioned or distinguished, never neither*
unrepresentable. It only makes the three triple fields travel together. An
`EntryName` implementation can still return `None` from `triple()` and
`Species::Leaf` (or `Node`) from `species()`. That is exactly
`witness_leaf_name_without_an_ordinal`, and the kit will accept it when it comes
from `parse`: parsed non-distinguished names are never required to carry a
triple. The independent `triple()` and `species()` methods also leave the inverse
inconsistency representable; the kit samples that half but the type does not
forbid it. Either the public view must encode the whole positioned/distinguished
choice, or this must remain a checked obligation. The architecture's discharge
claim at `docs/ordinal-fs-tree/ARCHITECTURE.md:225`, the model commentary at
`docs/ordinal-fs-tree/models/structure.als:164`, and
`DISCHARGED_BY_THE_TYPE_SYSTEM` all currently overstate what Rust guarantees.

### High — “species follows from parts” is assumed by the model but absent from the seam

`docs/ordinal-fs-tree/models/structure.als:109` makes `SpeciesFromParts` a law,
and the architecture relies on it to make a shift merely
`compose(new_ordinal, key, unchanged_parts)`. The Rust trait instead exposes
`fn species(&self)` independently at `src/name.rs:376`, and neither its type nor
the conformance kit requires equal `Parts` values to imply equal species. A
conforming implementation can make species depend on the ordinal: it passes
`ComposePlacesWhatItIsGiven`, then a shift changes a leaf into a node while
preserving the exact triple the kit checks. This breaks the derivation the seam
exists to protect. Put the positioned species behind a function of `Parts` (or
otherwise make that dependency part of the public contract), and add a broken
domain control proving ordinal/key-dependent species is rejected.

### High — the conformance kit accepts `Foreign` for a species contradiction

The obligation says a recognised name over the wrong `Found` is
`Verdict::Malformed`, not merely “not `Entry`”
(`ARCHITECTURE.md:248`). At `src/conformance.rs:470`, however, the kit counts any
candidate that parses for one or two of the three `Found` values as “refused”;
it never inspects whether the other verdicts are `Malformed`, `Foreign`, or
`Reserved`. A wrapper around the reference domain that converts every
`SpeciesMismatch` to `Foreign` therefore reports conforming: the agreeing case
sets `agreed`, the two foreign cases set `refused`, and no violation is emitted.
That implementation silently hides the exact contradictory directory subtree
`SpeciesAgreementIsParsed` exists to stop. Require `Malformed` for every
contradictory `Found`, and add this wrapper as the positive control.

### Medium — the kit does not check `parse(format(n)) == n`

`src/conformance.rs:345` reparses each composed name but compares only the second
rendering with the first rendering. It never compares the reparsed triple or
species with the composed value, despite the test at
`tests/conformance_kit.rs:56` claiming to discharge `RoundTripDisplay`. A domain
can compose the requested triple, render it, then parse that string into a name
with a different triple while retaining the string for `Display`; the kit sees
equal strings and passes, while snapshots read the wrong ordinal/key/parts.
`EntryName` need not gain `Eq`: compare the reparsed triple and species with the
original (and likewise ensure parsing the distinguished spelling returns a
triple-less distinguished value). Add a broken-domain control whose parse
changes the key without changing `Display`.

### High — the reference grammar silently skips a malformed node with no label

`src/reference.rs:365` defines this domain's recognition boundary as a leading
ordinal and terminal key, but `split_shape` returns `None` when the middle is
empty at `src/reference.rs:519`. Consequently `01--i3` is `Foreign`; if it is a
directory, the walk skips its entire subtree while reporting a healthy tree.
That is not a remote lookalike: it has exactly the two markers the module says
make a name this domain's, with its required label damaged. The fixture at
`tests/reference_domain.rs:204` cements the unsafe side of the trichotomy.
Classify both `01--i3` and `01--i3.md` as `Malformed(BadLabel)` with recovery
advice; reserve `Foreign` for names outside the positioned-and-keyed shape.

### Medium — the no-filesystem guard has ordinary false-negative spellings and over-broad exemptions

`tests/algebra_has_no_filesystem.rs:37` scans only `fs::` and `std::fs`.
Ordinary grouped imports such as `use std::{fs as f, path::Path};` contain
neither token and then allow every `f::…` call to pass. The header's stated
example, `use std::fs as f`, is actually caught, so it documents the wrong hole
and understates the natural one. Separately, `algebra_sources` at lines 59–64
skips every nested directory named `fs` and every nested `fs.rs`, not only the
promised top-level `src/fs/`; `src/algebra/fs.rs` is silently exempt. The exact
future `src/fs/` directory works, but the test does not establish the rule its
header claims. Restrict the exemption by path relative to `src/`, cover grouped
imports in the detector's positive controls, and either use a syntax-aware
dependency check or state the residual textual limits accurately.

## Doubt verdicts

1. **Trait deviation: reject as justified today.** `Triple`'s borrow is not a
   cost: the previous `parts()` also returned `&Parts`, so `compose` required the
   same clone. The type-system saving, however, is not real, and the four
   artifacts do not agree on `SpeciesFromParts` or the old witness's
   representability.
2. **Coverage accounting: reject.** Empty inputs and the distinguished-name
   coverage exclusion behave as intended, but the kit has the two silent-pass
   paths above.
3. **No-filesystem scan: reject as an assertion of the boundary.** It survives
   the intended top-level `src/fs/`, but ordinary Rust syntax and nested names
   bypass it.
4. **Reference grammar: mixed.** The whole-grammar re-render is the right
   canonicity check and no field-level defect escapes it in this implementation;
   the no-label Foreign/Malformed decision is unsafe.
5. **Workspace change: accept.** The root lint policy is byte-for-byte the same
   policy inherited by both members, the new member opts out of cargo-release,
   the root remains the default package built by the release scripts, and the
   Homebrew lane consumes only the two staged grove binaries. No actionable
   workspace/release finding.
