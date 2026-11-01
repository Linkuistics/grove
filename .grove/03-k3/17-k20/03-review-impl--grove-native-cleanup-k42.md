# grove-native-cleanup-k42

**Reviews:** grove-native-cleanup-k33

## Goal

Read the optional manual cleanup procedure adversarially, after its rewrite. Find
any case in which a user who follows it literally loses a foreign, uncertain or
still-needed entry.

## Context

The artifact is the section of `docs/USAGE.md` from the anchor
`usage-earlier-codex-installations` to the anchor `usage-tree-verbs`, as the
producer's commit left it. The contract is the delivery specification's section
*Existing installations and active readers*. The ground truth for the old writer
is `crates/grove/src/provision.rs` and `crates/grove/build.rs` at `175e5c7d259^`.
The manual installer is `plugins/install.sh`.

The producer spent its one in-session reviewer before the rewrite. The rewrite
changed the ownership conditions, the reference and alias search, the installer
guidance, the recovery order and the removal order. Its own disposable exercise
is recorded on the producer leaf. That exercise ran only on macOS, with
canonical home paths.

## Done when

- The specific doubts below each have a finding or an argued dismissal.
- Other findings are reported with a concrete scenario and evidence.
- No fix is applied. Findings worth acting on get an
  `integrate-review-impl` leaf, as the review family directs.

## Notes

These doubts remain after the rewrite:

- Whether "aliases" and "retained references" are now told apart correctly.
  The Codex skill directory itself can be a link, and the Codex home can be a
  link given in `CODEX_HOME`. A link to an ancestor such as `~` is not searched.
- Whether the first link condition is checkable in practice. It asks who made a
  link at a path. The provisioner kept no record, and it replaced owner links.
- Whether a `link-XXXXXX` staging directory is classified safely. A harness
  can scan nested directories in a skill directory.
- Whether the search commands behave on Linux and on a non-canonical `HOME`.
  The exercise did not cover either.
- Whether the step 5 recovery order and the step 6 removal are complete when
  only part of the layout moved.
