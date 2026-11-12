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
