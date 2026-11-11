# ravel-release-promote-k306

## Goal

Deliver `release:promote VERSION=2.0.0`, the repeatable task that publishes
an already complete `ravel@2` release as unversioned `ravel`. Finish runs it
after the compatible Grove release and its installed verification.

## Context

- Ravel `docs/specs/release-channels.md`, sections "Promotion without another
  release" and "Recovery and existing processes".
- Grove `docs/specs/coordinated-release.md`, sections "Compatible tap
  states" (state D) and "Recovery boundaries" (row "Promotion partial").
- Grove `docs/adr/finish-promotes-after-the-grove-release.md`.
- The channel work of ravel-release-channel-k305.

## Done when

- The task cuts no version, rebuilds no archive, creates no tag or release and
  rewrites no book.
- Before any tap mutation, it verifies the public tag, the exact asset
  digests, the book identity, the published `ravel@2` formula and the
  consumer readiness receipt. Missing or mismatched evidence refuses with a
  remedy.
- The consumer readiness receipt has a documented shape. Grove's finish can
  write it from its release and installed verification. Ravel's runtime reads
  no Grove version table.
- One tap commit renders unversioned `ravel.rb` from the verified assets and
  marks `ravel@2` deprecated in favour of `ravel`. It changes nothing else in
  `ravel@2.rb`. The commit is pushed through jj.
- Installation upgrades unversioned `ravel` with `HOMEBREW_NO_INSTALL_CLEANUP=1`
  and verifies the selected prefix. The old 1.1.0 keg stays.
- A retry verifies and keeps a matching formula, tap commit or installation.
  It reports a mismatch and does not overwrite it.
- Fixtures cover unchanged archives, the deprecation, idempotent retries,
  preserved old prefixes, refusal before the readiness receipt and refusal of
  a mismatched archive. Each control is seen to fail against a deliberate
  mutation before it counts.
- `task check`, `task test` and `task release:lifecycle:test` pass. Nothing
  is published.

## Notes

Work in `Ravel.skill-directory-delivery` on `skill-directory-delivery`.

The real upgrade rehearsal with Homebrew, the tap states and a HOME that
holds only `policy.ts` is the later leaf `promotion-upgrade-rehearsal` of the
parent node. This leaf delivers the task and its fixtures. Write the exact
command line, the receipt shape and an example receipt where the finish
handoff can cite them.

If an adversarial read is earned, cut `review-impl` with `leaf-insert` before
`ravel-book-inside-k307`.
