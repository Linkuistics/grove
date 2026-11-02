# grove-dual-archive-k100

## Goal

Ship both caller-asset sets in Grove's archives and formula. Prove the
installed layout against each explicit Ravel build.

## Context

- The "Packaging and rollout" section and the "Installed distribution" and "No
  native mutation" rows of the delivery specification.
- `scripts/release-build.sh`, `scripts/templates/grove.rb.tmpl`,
  `scripts/grove-installed-smoke.sh` and the asset gate of `docs/RELEASING.md`.
- Tap states A to D of `docs/specs/coordinated-release.md`.

## Done when

- The archive layout and its manifest carry both asset sets under
  `libexec/ravel-policy`. They carry no skill payload, private worker or skill
  resolver.
- The formula keeps its unversioned Ravel dependency. Its caveats show both
  configuration formats, both setup routes and the `ravel@2` prefix until
  promotion.
- The formula test and the installed smoke check the TypeScript set against the
  1.1 build. They check the Rhai set against the 2.0 build through an explicit
  prefix.
- An installed upgrade from a 22.8 layout changes no old or foreign native
  entry, link, mode, pointer or byte. An active old reader keeps working.
- `release:test` and the release preparation checks pass. The asset gate of
  `docs/RELEASING.md` names both sets.

## Notes

No tap mutation and no publication here. The fresh-install and upgrade
rehearsal of states A to D needs the `ravel@2` formula, so rollout owns it.
Record which state each check in this leaf models.

Controls: an archive without the Rhai set must fail the manifest check. A
caveat that names only `policy.ts` must fail the formula test. Watch each fail
once.
