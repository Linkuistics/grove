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

## Decisions (running log)

**All six findings were real; none demanded a redesign.** Classified as
`integrate-review.md` directs: five were *real issues* and one — the boundary
guard's header documenting a hole it did not have — was *a contract stated
unclearly* as well. Nothing was noise and nothing was accepted as a trade-off.
The two doubt verdicts the review accepted (the `Triple` borrow, the workspace
change) were left alone, as this leaf's own notes required.

**The two trait-shape findings were one hole seen from two sides, and one change
closes both.** `species()` was an independent method, so it could contradict
`triple()` (finding 1) *and* depend on the ordinal (finding 2). The seam now has
`fn view(&self) -> NameView<'_, Parts>` — `Positioned(Triple)` or
`Distinguished`, so the whole choice is one value — and
`fn positioned_species(parts: &Parts) -> PositionedSpecies`, an associated
function with no `self`, no ordinal and no key. `triple()` and `species()`
survive as readings on `EntryNameExt`, blanket-implemented and sealed: a
*provided* method would have been overridable, which is the same hole with a
default. Both obligations are now discharged by the shape rather than checked,
so the count of stated obligations went from five to six — *the species follows
from the parts* was always an obligation and had been filed as a consequence of
the isomorphism.

**A discharge claim gets a positive control, like any other instrument here.**
`positioned_species` carries a `compile_fail` doctest — a domain wanting a node
at an even ordinal — beside an otherwise identical one that compiles. The
failure was checked to be `E0424` (*this function doesn't have a `self`
parameter*) and not incidental breakage.

**The kit's two silent-pass paths were closed with a broken domain each**, which
is what the four existing ones are for: `Evasive` turns every species
contradiction into `Foreign` and `KeyDrift` parses its own rendering into a
different key while `Display` says what it said. Each is rejected for exactly
one obligation. The `Malformed`-and-not-merely-refused rule is a real
constraint on every future domain, grove's own included, and it is stated in
`ARCHITECTURE.md` rather than left in the kit's code.

**The boundary guard now scans for the identifier `fs` as a whole word** — the
grouped `use std::{fs, path::Path};` was what defeated it, not the aliased form
its header named — and exempts by path relative to `src/` rather than by file
name, with `the_exemption_is_the_promised_path_and_nothing_else` proving the
rule the header claims. Its residual limits are restated accurately: it reads
text, so a macro-assembled path is invisible to it. The manual control was
redone — a real module, a violation added, watched failing, removed.

**No new review chain, and no in-session reviewer.** Nothing here demanded the
seam be rethought rather than repaired, which is the condition
`integrate-review.md` gives for externalising; and the session's one narrow
reviewer was not spent because this harness's own instructions forbid
dispatching a subagent unasked. The one claim it would have gone to — that
requiring `Malformed` does not over-constrain a legitimate domain — was checked
by hand instead: a name a domain owns under one listing and disclaims under
another is exactly `witness_species_mismatch_is_unclassifiable`, so there is no
legitimate domain the rule refuses.
