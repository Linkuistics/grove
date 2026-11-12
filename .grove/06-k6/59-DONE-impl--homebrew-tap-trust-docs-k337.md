# homebrew-tap-trust-docs-k337

## Goal

Make the documented Homebrew installation of Grove and Ravel work for a new
user under Homebrew's tap trust.

## Context

scratch-homebrew-rehearsal-k333 found this on 2026-11-12. In a scratch
Homebrew at the owner's version, 7.0.13-83-ga6fea6b, `brew tap` did not trust
the tap. `brew install linkuistics/taps/grove` trusted only `grove` and then
refused its dependency:

```text
Error: Refusing to load formula linkuistics/taps/ravel from untrusted tap linkuistics/taps.
Run `brew trust --formula linkuistics/taps/ravel` or `brew trust linkuistics/taps` to trust it.
```

The owner does not see the refusal, because the owner's
`~/.homebrew/trust.json` already trusts `linkuistics/taps`.
`Library/Homebrew/trust.rb` at the owner's HEAD holds the rules:
`trust_fully_qualified_items!` and `raise_untrusted!`.
`docs/promotion-upgrade-rehearsal-record.md` records the refusal.

Pages that give the installation command include `README.md` (the `brew tap`
and `brew install grove` lines), `plugins/README.md`, `docs/USAGE.md`, the
`configure-grove` skill's `references/policy.md` and `docs/RELEASING.md`.
Grove's missing-Ravel message names `brew install linkuistics/taps/ravel`.
Ravel's README and book give their own installation commands.

## Done when

- Each installation route that a page documents names the trust step,
  `brew trust linkuistics/taps`, before the first install from the tap. A
  route that does not need the step says why.
- Read Homebrew's documentation for tap trust and cite it beside the step.
  If no official page exists, say so in the record.
- A new user's route, run in a scratch Homebrew with an empty trust store,
  installs a usable pair. `task promotion:rehearsal` can serve as that
  instrument.
- Ravel's pages carry the same correction in Ravel's own workspace, or a
  leaf there owns it.

## Decisions (running log)

- The trust step is `brew trust linkuistics/taps`, without `--tap`. That is
  the whole-tap form in Homebrew's
  [Tap Trust](https://docs.brew.sh/Tap-Trust#installing-from-a-tap) page
  (`docs/Tap-Trust.md` at the owner's HEAD `a6fea6bd59bb`, last reviewed
  2026-10-25). `Homebrew::Trust.infer_target` in `trust.rb` reads a name
  with one slash as a tap. The page states that tap trust is the default
  since Homebrew 6.0.0.
- Whole-tap trust, not per-formula trust. The page prefers per-item trust,
  but Grove's route installs Grove, its dependency `ravel` and, later,
  `ravel@2`, `personal` and `testanyware` from the same tap that this
  project administers. The page names that case as appropriate for
  whole-tap trust.
- Only `grove.rb` depends on another formula of the tap
  (`depends_on "linkuistics/taps/ravel"`). A fully qualified install of
  `ravel` or `personal` therefore trusts that formula and loads nothing else
  from the tap, so those routes need no trust step. Their pages say so, and
  the rehearsal's fresh-untrusted scenario measures both.
- Correction, measured on 2026-11-13: `ravel@2` does need the tap trust.
  The first rehearsal run stopped in fresh-b. Homebrew trusted `ravel@2`
  and then refused `linkuistics/taps/ravel` from the untrusted tap, also with
  `--skip-link`. `FormulaInstaller#auto_link_versioned_keg_only?` loads
  `Formula#link_overwrite_related_formula_names`, which holds the
  unversioned formula. Ravel `main` briefly held the wrong claim in
  `ca3b39ee`, and `f34db9a9` corrects it. Grove's README, user guide, spec
  and CHANGELOG state the need. fresh-b now records the refusal before its
  trust step. `testanyware` is unmeasured, so no page claims that it needs
  no trust.
- The README route changes from `brew tap Linkuistics/taps` to the
  lower-case name. Homebrew stores the trust in lower case, and the three
  commands then name the tap the same way.
- Grove's missing-Ravel message stays. Its route is a fully qualified
  `ravel` install, which needs no trust.
- The pack READMEs (RequirementsSkills, DesignSkills, EngineeringSkills,
  ResearchSkills, WritingSkills and its TECHNICAL-COMMUNICATION guide,
  PersonalSkills) carry the same short-name route. They are in the
  enumeration of cross-repo-docs-reconciliation-k319, so that leaf owns
  their correction. This leaf adds the finding to its task file.
- The pair controls `versioned-2-at-c` and `unversioned-at-d` run in a
  copy-on-write copy of the Ravel clone. They required Ravel `main` to have
  no change since v2.0.0. This leaf's commits on Ravel `main` failed the
  third run, and k332's commit would fail any later rerun. Each control
  must print the planned cut of its channel. It must stop at the change
  check when `main` has no change. The clone's `main` must not move.
- The fourth run of `task promotion:rehearsal` passed, with the instrument
  digests unchanged. `docs/promotion-upgrade-rehearsal-record.md`, section
  "Tap trust routes", holds the readings and the conditions of the
  instrument.
