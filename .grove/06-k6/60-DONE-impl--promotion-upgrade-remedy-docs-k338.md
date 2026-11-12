# promotion-upgrade-remedy-docs-k338

## Goal

Correct two statements about the upgrade to state D that
promotion-state-rehearsal-k335 measured as wrong, before finish publishes
Grove's caveats and the promotion.

## Context

- `docs/promotion-upgrade-rehearsal-record.md`, section "State D",
  "Observations for the decision".
- `docs/specs/coordinated-release.md`, the paragraphs after the table of
  "Compatible tap states".

## Done when

- The spec no longer says that Homebrew normally upgrades outdated
  dependents. It states the measured behaviour. `Library/Homebrew/upgrade.rb`
  at the owner's HEAD upgrades an outdated dependent only when the dependent
  and all its dependencies are bottled. Grove has no bottle, so
  `brew upgrade ravel` at D leaves Grove 22.8.0. Grove 22.8.0 then refuses
  Ravel 2.0.0 before any launch, and its message tells the user to run
  `brew upgrade ravel`.
- Each page that tells a Grove 22.8 user how to upgrade to D says to run
  `brew upgrade` or to upgrade Grove first. No page relies on Homebrew to
  upgrade Grove after `brew upgrade ravel`. The pages include Grove's
  caveats and the remedy that the spec requires before promotion. They also
  include the Ravel 2.0.0 release notes, or a leaf in Ravel that owns them.
- Each statement that `"sources": []` turns delivery off names its place,
  the `skills` object: `{"skills": {"schemaVersion": 1, "sources": []}}`.
  Ravel 2.0.0 refuses a top-level `sources` key as `settings_invalid`. The
  record shows the refusal. The statements include
  `scripts/templates/grove.rb.tmpl`, `docs/USAGE.md`, the
  `configure-grove` skill and its `references/policy.md`, and spec row D.
- `bash scripts/check.sh` passes.

## Notes

The candidate formula of the rehearsal carries the current caveats. A
change to the template changes only the formula, not the archives, so it
needs no new rehearsal of the packaging.

## Decisions (running log)

- The remedy text names `brew upgrade` with no formula name first, because
  upgrade-a-d measured it. "Upgrade Grove first" uses `brew upgrade grove`.
  After a lone `brew upgrade ravel`, the remedy is also `brew upgrade grove`.
  The pages do not claim that `brew upgrade grove` upgrades Ravel too, because
  no rehearsal measured that.
- The Ravel 2.0.0 release notes go to the new leaf
  ravel-release-notes-upgrade-order-k339, before cross-repo-docs-reconciliation-k319.
  The book of v2.0.0 is final, and promotion refuses a changed book. The
  tagged changelog section cannot change the released notes. The GitHub
  Release body is outward-facing, so its edit needs the human's approval.
- The formula test `scripts/release-formula.test.rb` now requires the upgrade
  order, the remedy for an earlier Grove and the off value in the `skills`
  object. It refuses the earlier phrase `"sources": [] in settings.json`.
- `configure-grove` also gets the remedy for the refusal of Grove 22.8,
  because the skill remedies refused launches.
- Records stay as history: the rows of `docs/grove-docs-reconciliation-record.md`,
  the rehearsal record and `docs/owner-skill-delivery-evidence.md`. The fixture
  table of `docs/grove-dual-ravel-stage-record.md` changed, because the
  fixtures wrote the `skills` object. The code constants `OFF` are the value of
  the `skills` object and are correct.
