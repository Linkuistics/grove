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

## Decisions (running log)

**The route that guidance describes (2026-11-07).** Guidance sets up one
route: Homebrew installs Grove, core and the five packs, and Ravel 2.0 reads
them from its default skill directory. Ravel's skill discovery specification
states that a settings file without `sources` selects that directory. That
includes a settings file without `skills` and an absent file. The two native
routes stay in the route tables as caller routes that Grove admits, with no
setup guidance. Until promotion, guidance selects `ravel@2` by PATH, and the
text is conditional on the unversioned formula's major, as the caveats are.

**No separate HOME for the default route (2026-11-07).** Ravel 1.1.0 accepts
a settings file without 2.0-only keys. The default route needs no key, so it
needs no separate HOME. Guidance keeps the separate HOME only for a 2.0 route
that needs a 2.0-only key: `sources`, `"sources": []`, `configEnv`,
`harnesses` or `budgets`. Explicit `sources` stay documented as Ravel's
option to replace the default, with keg paths in the example, not checkouts.

**Hook commands in the usage guide (2026-11-07).** `docs/USAGE.md` holds the
table of "Hook adapters" as the anchor `usage-hook-adapters`. The README,
`plugins/README.md` and the caveats point to it or print the same commands.

**Guidance beyond the named files (2026-11-07).** The sweep found routes in
`references/grove.md`, `CONTEXT.md`, `CONTEXT-MAP.md`, `docs/ARCHITECTURE.md`
and `plugins/CONTEXT.md`. Each was a sentence or two, so this leaf corrected
it. The dialogue of `plugins/CONTEXT.md` still described the symlink
installer and its skip-loudly default, which k236 retired as terms.

**Versioning (2026-11-07).** `plugins/README.md` keeps the rule of no
`version` in `plugin.json`. The marketplace reason ended. The reason now is
that a cut must leave the `grove-core` bytes of a release equal to those of
its candidate keg.

**The formula test (2026-11-07).** `release-formula.test.rb` expects the
default skill directory and refuses the retired routes. `check.sh` does not
run it, because it needs built archives. A scratch script ran its caveat
assertions against the template's `caveats` method: they pass, the parent
revision's template fails them, and a mutated template fails the retired-route
assertion.

**The sweep (2026-11-07).** `docs/repository-moves-record.md`, "The delivery
guidance", records the method, the controls and the result.
