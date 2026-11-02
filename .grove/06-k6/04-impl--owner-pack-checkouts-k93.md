# owner-pack-checkouts-k93

## Goal

Put each of the owner's pack checkouts in place in the real HOME at its
accepted pin. Make `mac-check` report checkout drift. Discovery stays
native.

## Context

- The contract is the section "Pack checkouts" of
  `docs/specs/owner-skill-delivery.md`, and the real-HOME preparation of
  cutover order 6.
- owner-pack-checkouts-k79 settled the owner's tool: the `[bootstrap.repos]`
  part of mise. owner-pack-checkouts-k82 integrated its first review.
- The staging leaves of owner-ravel-skill-inventory-k38 supply the drafted
  entries, the accepted pins and the accepted `resourceDigest` of each
  source. Read them from the staging evidence.
- `task owner:checkouts` measures mise's repository part in a fixture HOME.

## Done when

- `~/.config/mise/config.toml` has one `[bootstrap.repos]` entry for each
  Ravel source. Each entry names the checkout path, the HTTPS URL and the
  accepted pin as a full commit ID.
- `mise -C "$HOME" bootstrap repos apply` cloned each checkout, and
  `mise -C "$HOME" bootstrap repos status --missing` exits 0.
  `git status --porcelain --ignored` prints nothing in each checkout.
- `mac-check` runs the status command with `--json`. It tells drift from a
  configuration error, and a fixture control shows each outcome.
- The machine contract and the `mac-bootstrap` guidance state that each
  `apply` and each complete `mise bootstrap` is an advance, behind a process
  check.
- Native discovery is unchanged. The real HOME's Ravel settings name no skill
  source, and no harness root holds a new entry.

## Notes

Before `apply`, list the live processes as "Pack checkouts" states. Run no
Ravel command and no release task until `apply` ends.

owner-ravel-skill-inventory-k38 puts its leaves for policy conversion, drafts
and staging trials before this leaf with `leaf-insert`. The marketplace
freeze and the rehearsal on the real plan follow this leaf, also in order 6.
