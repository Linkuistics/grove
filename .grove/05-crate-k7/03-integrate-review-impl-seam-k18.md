# seam-k18

**Integrates:** seam-k17

## Goal

Integrate every actionable finding from `seam-k17` before the remaining crate
leaves build on the public seam. Reconcile the architecture, Alloy model, Rust
trait, conformance kit, reference domain, and boundary test; then run all
post-fix verification. This session owns fixes — the review changed no
production or test code and ran no verification commands.

## Context

Beyond the brief chain and its *Read first* list:

- `seam-k17` in full, especially its six findings and five doubt verdicts. The
  cited `path:line` coordinates are against `seam-k8`'s commit `57c3b8ae`.
- `seam-k8`, the reviewed producer and the recorded pre-fix verification.
- `docs/adr/entry-name-is-the-only-seam.md`: the trait is hard to reverse, so
  resolve the two trait-shape findings before preserving the current surface by
  default.
- `docs/formalism-findings.md` entry 004 and
  `docs/ordinal-fs-tree/models/structure.als`: if implementation and model
  disagree, change the model first, run its runner, then change code, and record
  the disagreement as a finding.

## Done when

- A parsed `Leaf` or `Node` cannot carry no triple unnoticed. Either the public
  view makes positioned-versus-distinguished genuinely unrepresentable, or the
  architecture/model stop claiming that and the conformance kit checks both
  directions with a broken-domain positive control.
- Positioned species is a function of `Parts`, as `SpeciesFromParts` assumes;
  changing ordinal or key while preserving parts cannot change species, and a
  deliberately ordinal-dependent domain is rejected.
- The conformance kit requires `Malformed` for every `Found` contradiction. A
  wrapper that changes `SpeciesMismatch` into `Foreign` is rejected.
- The kit checks the semantic `parse(format(n)) == n` direction by comparing
  triple and species, including the distinguished spelling. A parser that
  changes the key while preserving `Display` is rejected.
- The reference grammar returns `Malformed(BadLabel)` with recovery advice for
  both `01--i3` and `01--i3.md`; it does not silently skip the directory form.
- The filesystem-boundary guard exempts only the promised top-level `src/fs/`,
  catches grouped `std::{fs ...}` imports, and accurately states any remaining
  textual limitations. Its positive and coverage controls still prove the
  instrument can fail and reached the intended sources.
- `ARCHITECTURE.md`, `structure.als`, `src/name.rs`, `src/conformance.rs`, and
  their claim-citing tests state the same contract after the fixes.
- Both model runners, the crate and grove test suites, formatting, and workspace
  clippy are green after the integration. Append this modelling/implementation
  episode to `docs/formalism-findings.md` before retiring.

## Notes

`Triple<'_, Parts>` borrowing was reviewed and is not itself a defect: the
replaced `parts()` accessor also borrowed, so consumers already had to clone
before passing `Parts` to consuming `compose`. The workspace/release change was
also accepted; do not widen this integration into release work without new
evidence.
