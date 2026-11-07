# homebrew-delivery-guidance-k237

## Goal

Make Grove's shipped guidance describe Homebrew installation and Ravel
delivery, as the release will ship them.

## Context

- The planning inputs of `ravel-default-skill-directory-k218` and the
  section "Shipped guidance for planning" of
  `homebrew-delivery-design-k220` name these files:
  - `plugins/grove/skills/configure-grove/SKILL.md`.
  - The directory route of `docs/USAGE.md`.
  - `README.md`, which says that Ravel 2.0.0 reads the checkouts that its
    settings name.
  - `plugins/README.md`, which names the Codex marketplace of WritingSkills.
- "Installation and bootstrap", "Hook adapters" and "Supported routes" of
  `docs/specs/codex-skill-provisioning.md`.
- "The pack record and the release" states how a pack change reaches users.
- The "Skills" text of `caveats` in `scripts/templates/grove.rb.tmpl` is
  shipped guidance too. `native-marketplace-retirement-k235` removed only its
  Claude Code marketplace commands. The rest still describes a native route
  and a `skills` object in Ravel's settings. Rewrite it for the final route,
  after `manual-installer-removal-k236` removes the installer.

## Done when

- The four files above, `plugins/grove/README.md` and `docs/USAGE.md` agree
  with the delivery specification. `brew install` and `brew upgrade` are the
  whole skill setup. Ravel 2.0.0 reads the default skill directory with no
  settings. The hook adapters' commands match the table of "Hook adapters".
- `docs/RELEASING.md` states the four steps by which a pack change reaches
  users.
- Enumerate every guidance line that names `skills.sources`, a checkout, a
  marketplace installation or the installer as the route. Classify each one.
  A sweep with controls shows no unclassified line.
- The prose checker reports no hard finding in each edited document.
- `bash scripts/check.sh` passes.

## Notes

These edits change `plugins/grove`, so they change the `grove-core`
candidate. `candidate-keg-k238` runs after this leaf.
