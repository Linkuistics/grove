# ravel-release-channel-k327

**Integrates:** `ravel-release-channel-k325`

## Goal

Triage the review's findings against Ravel's release-channel contract, apply
the actionable corrections and verify the resulting release tooling before
its first publication.

## Context

- Read the findings and source-backed doubt dispositions in
  `ravel-release-channel-k325` from its committed review.
- The producer is `ravel-release-channel-k305`. The current artifact is on
  `skill-directory-delivery` in
  `~/Development/AgenticOrchestration/Ravel.skill-directory-delivery`.
- Ravel `docs/specs/release-channels.md` owns the channel mechanics. Grove
  `docs/specs/coordinated-release.md` owns the publication order.

## Done when

- Every finding has an evidence-backed disposition, and the accepted
  corrections have appropriate verification recorded in the owning repo.
- No release, tag, tap change or installation is published by this leaf.

## Notes

Keep the review's source coordinates anchored to its reviewed commit when
classifying the findings. Preserve the running driver's full Ravel 1.1.0
keg and original ending command. The promotion review follows this step.

## Decisions (running log)

- F1 is a real issue. Ravel's decision record
  `a-caller-transition-publishes-a-versioned-formula-first` states that
  keg-only `ravel@2` never becomes the default command. The spec and Grove's
  state B make the same promise. The installed Homebrew `7.0.13-45-g95d4938`
  confirms the review's source reading:
  `FormulaInstaller#auto_link_versioned_keg_only?` (`formula_installer.rb:2003`)
  returns true for a request on a prefix with neither formula, and the
  constructor (line 130) then links the keg. `--skip-link` makes `link`
  create only the opt link (lines 1349–1357).
- An upgrade keeps the keg's link state (`upgrade.rb:594–606` passes
  `link_keg = keg.linked?`), so only a first installation needs
  `--skip-link`. A plain installation beside an installed unversioned `ravel`
  does not link either, so the owner's machine is not at risk. The fix still
  applies `--skip-link` to every first keg-only installation, because the
  promise holds on every prefix.
- Fix: `release-install.sh` installs the keg-only channel with `--skip-link`.
  The fixture's fake Homebrew models the automatic link, a new control
  installs on a prefix without either formula, and a new mutant removes
  `--skip-link`. The spec, the decision record and Grove's state B row name
  `--skip-link` as the public route on such a prefix. No public installation
  instruction for `ravel@2` exists yet in the book. The chapter 7–11 drafts
  take it from the spec.
- Applied in Ravel `62cccfdbb0d51b404340b86099c3f6cb520a892a` on
  `skill-directory-delivery`, pushed to `origin`. It changes
  `scripts/release-install.sh`, `scripts/release-channel.test.sh`,
  `docs/specs/release-channels.md` and the decision record. This Grove commit
  amends the state B row of `docs/specs/coordinated-release.md`.
- Verification: `task release:lifecycle:test` passed
  (`target/release-lifecycle.Rix2K6`). All 42 declared mutants were killed. The
  mutant `install links ravel@2` fails the new fresh-prefix control with
  `linked_keg: "2.0.0"`, which is F1's failure. The digests of the install
  script, the channel test and the Taskfile matched before and after the run.
  `shellcheck scripts/*.sh` passed. Ravel's STE checker and the generic
  `ste-lint.py` report no new problem in the changed documents.
- No release, tag, tap change or Homebrew installation ran. The real
  `--skip-link` behaviour rests on the installed Homebrew source above, not on
  an installation rehearsal. The publication leaf's installed verification
  exercises it.
