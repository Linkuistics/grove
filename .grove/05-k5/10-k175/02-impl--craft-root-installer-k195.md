# craft-root-installer-k195

## Goal

Teach the manual installer the `craft/` root of a pack. A pack can then ship
craft skills in `craft/` beside `skills/`, or in `craft/` alone. This change
is Grove-only and comes before the DesignSkills repository, so the pack's
README can give a working install command.

## Context

- `docs/specs/pack-membership.md`: the rows "The first `craft/` root" and "A
  source whose only root is `craft`" in "The acceptance of each move", and the
  test seam "The installer".
- `docs/specs/external-pack-contract.md`: "Layout" and the installer item of
  "Distribution". Also the item "The installer refuses a shared skill name" in
  "Differences from the decisions".
- `plugins/install.sh` and `plugins/install.test.sh`. At planning, `--pack`
  required a plugin directory with `skills/`. The link test `is_ours`
  recognised only the path segment `plugins/<plugin>/skills/`. Read both
  again before you change them.

## Done when

- `install.sh` scans `craft/` of each scanned plugin directory, with the
  eligibility and the workspace guard of `skills/`. Each eligible craft skill
  gets the same links as a skill under `skills/`.
- `--pack` accepts a plugin directory with `skills/`, with `craft/`, or with
  both. It refuses a directory with neither, and the diagnostic names both
  roots.
- The link test recognises a link into `plugins/<plugin>/craft/` as the
  installer's own. A later run then removes the link of a removed craft skill.
- The refusal of a shared skill name covers both roots. A name that `skills/`
  and `craft/` of the scanned plugins ship twice refuses before any link
  changes.
- `install.test.sh` installs a fixture pack with `craft/` beside `skills/` and
  a fixture pack with `craft/` and no `skills/`. For each fixture, a later run
  without the removed craft skill removes its link. A run without `--pack`
  leaves the craft links of that pack unchanged.
- Each new assertion fails against the unchanged installer, and the record of
  the leaf names that reading.
- The installer usage text, `plugins/README.md` and the comments of
  `install.sh` name the craft root where they name `skills/`.
- `bash scripts/check.sh` passes.

## Notes

personal-pack-k183 consumes the craft-only fixture as its static proof. Keep
that fixture a plain shape that a later leaf can name.

No pack exists yet with `craft/`. Use fixtures only. Do not change the bundled
`plugins/linkuistics` layout here.
