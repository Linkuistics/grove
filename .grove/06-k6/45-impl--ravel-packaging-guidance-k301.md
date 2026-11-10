# ravel-packaging-guidance-k301

## Goal

Make Ravel's published documentation show the packaging rule that keeps a
default source intact. The rule is a private payload behind a relative
directory symlink. Land it on Ravel's 2.0 line before the 2.0.0 cut.

## Context

- Ravel's 2.0 line is the bookmark `skill-directory-delivery`, in the
  workspace `~/Development/AgenticOrchestration/Ravel.skill-directory-delivery`.
- Ravel's `docs/specs/skill-discovery.md` owns the default skill directory.
  It says packages "put their sources" there.
  `docs/design/skill-discovery/homebrew-directory.mmd` shows a real
  directory installed at `share/ravel/skills/ID` in the keg.
- Finding L7 of the evidence section "Directory-link layout review" of
  Grove's `docs/homebrew-skill-delivery-evidence.md` states the failure
  that this guidance must prevent.
- "The keg layout" of Grove's `docs/specs/codex-skill-provisioning.md` owns
  the rule for Linkuistics formulae. Ravel's documentation states it for
  any package.

## Done when

- Ravel's skill discovery specification states the packaging rule. It
  explains why a real directory under `share` can expand into file links
  that Ravel then refuses. Ravel's resolver and source boundary stay
  unchanged.
- The Homebrew directory view shows the private payload and the alias, and
  its manifest and viewer anchors stay valid.
- Any other packaging guidance in Ravel's book or README agrees with the
  rule. A search for the earlier statement finds only history.
- Ravel's applicable checks pass. The change is a commit on
  `skill-directory-delivery`. The Ravel release leaves of
  discovery-rollout-k12 publish it with 2.0.0.

## Notes

Ravel's formula template writes `share/ravel/examples`, `notices` and
`installation.json`. Those are not skill sources and need no alias.
shared-directory-seam-k294 covers their participation in linking.

Do not push the line or cut a release here. The coordinated release
design owns the publication order.
