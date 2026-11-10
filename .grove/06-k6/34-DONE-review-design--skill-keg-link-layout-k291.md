# skill-keg-link-layout-k291

**Reviews:** skill-keg-link-layout-k274


## Goal

Adversarially review the revised skill-source layout and migration before
skill-keg-link-layout-k290 plans its implementation.



## Context

- The producer's committed artifact is the amendment to "The keg layout"
  in `docs/specs/codex-skill-provisioning.md`, the matching owner delivery
  obligations and the k93/finish handoffs.
- `docs/homebrew-skill-delivery-evidence.md`, "Directory-link layout
  experiment", names the real Homebrew/Ravel seam, subjects, results and
  unmeasured obligations. Its source is
  `docs/design/skill-keg-link-layout/reproduce.rb`.
- The design requires installation and upgrades alone to preserve intact
  sources in Ravel's shared default directory, in any order. Existing
  source-boundary checks, source identities and independent Ravel releases
  remain required. The owner's native route remains active until finish.
- `skill-keg-link-layout-k290` receives this review before planning. The
  producer's launch supplied no provider receipt, so no Creator is recorded.

## Done when

- Inspect the proposal against Homebrew's actual linking semantics and the
  source lifecycle. Seek counterexamples to its stated installation,
  upgrade and migration contracts, including formula ownership and retained
  source or hook paths.
- Check that the experiment establishes only what the evidence claims,
  and that implementation acceptance covers the material remaining cases.
- Record supported findings with locations, consequences and evidence, or
  state that no material finding is supported. Do not implement fixes.

## Notes

The current diagram sources and changed views are at
`docs/design/harness-selection-and-execution/`. The producer could check the
served bytes. It could not inspect their rendering, because the browser host
was unavailable. Published-formula acceptance is future implementation work,
not something this design session claimed to complete.

## Decisions (running log)

- The reviewed producer is commit `7e3673f3c834`. The review is inspection
  only. It ran no test, build, experiment or check task. It read Homebrew's
  installed `keg.rb`, `cleaner.rb`, `formula_installer.rb` and `install.rb`
  at `bc63375ac3a4`, and Ravel's specification and formula template at
  `ff14cba826b0`.
- Each linking claim of the amendment holds in the installed linker. The
  review found no counterexample to the directory-symlink remedy itself, so
  no finding is P1.
- The producer's evidence section adds hard prose findings to a clean
  document. A preparatory commit marks that section as pending. The review
  changes no word of it, as owner-switch-simplification-k261 did.
- The section "Directory-link layout review" of
  `docs/homebrew-skill-delivery-evidence.md` holds eight findings, L1 to L8.
  L1 to L3 are P2, so the review cuts the integration step before
  skill-keg-link-layout-k290 plans from the design.
