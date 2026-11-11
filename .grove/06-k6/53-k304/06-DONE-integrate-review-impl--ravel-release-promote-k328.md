# ravel-release-promote-k328

**Integrates:** `ravel-release-promote-k326`

## Goal

Triage the promotion review against the release contract, apply the
actionable corrections and verify the resulting tooling before the book
describes it or a public promotion runs.

## Context

- Read the committed findings and source-backed doubt dispositions in
  `ravel-release-promote-k326`.
- The producer is `ravel-release-promote-k306`. Its artifact is on
  `skill-directory-delivery` in
  `~/Development/AgenticOrchestration/Ravel.skill-directory-delivery`.
  The review reads current source at `62cccfdbb0d5`, including k327's
  first-versioned-installation correction.
- Ravel `docs/specs/release-channels.md` owns the promotion contract.
  Grove `docs/specs/coordinated-release.md` owns publication order and
  retention of the original prefixes.

## Done when

- Every finding has an evidence-backed disposition. Accepted corrections
  and appropriate fresh verification are recorded in their owning repo.
- No release, tag, public tap change or Homebrew installation is published
  by this leaf.

## Notes

Anchor source coordinates to the reviewed commit while triaging. The
review ran no test or installation command. The actual Homebrew upgrade
rehearsal remains with `promotion-upgrade-rehearsal-k318`. Preserve the
running driver's full original 1.1.0 keg and ending command.

## Decisions (running log)

- Triage reads the findings from the review commit `4a5f51cd5fdd` against
  Ravel `62cccfdbb0d5`, where `scripts/release-promote.sh` is unchanged
  from the producer.
- F1 is a real issue, latent on this machine. The installed `jj git push
  --help` states that `--remote` defaults to `git.push`. The owner's tap has
  only `origin` and no `git.push`. The contract does not exclude a second
  remote, and the fix costs one argument. The same unpinned destination is
  in the cut. `release-publish.sh` fetches and pushes the tap without a
  remote. The Taskfile's `release` pushes Ravel's `main` without one, but
  it pushes the tag to `origin`. Fix all three through one tap helper
  and one explicit `origin`. The cut's two sites are the same defect class,
  and they run before finish.
- F2 is a real issue. Installed Homebrew `formula.rb:5300–5302` sets
  `deprecated` only when the date is at most the local `Date.today`. Fix:
  the deprecation date is the tag commit's date at UTC−12 (`TZ=AOE12`). No
  inhabited zone is behind UTC−12, so that date is at most the local date
  everywhere after the tag. It still depends only on the tag, so a retry
  renders the same bytes. `release-formula.sh` takes the same helper. A
  library control pins the boundary values. The fixture's tag commit has a
  fixed date at which UTC and UTC−12 differ.
- F3 is a real issue. The real 1.1.0 keg holds `libexec/ravel/ravel-policy`,
  the SDK, the examples and the notices. Fix: before the Homebrew mutation,
  record a manifest of each earlier keg. The manifest holds each path, its
  type, a symbolic link's target, and a file's executable bit and SHA-256.
  After the mutation, every manifest must be unchanged. Homebrew's own
  `INSTALL_RECEIPT.json` is compared by presence only. Installed Homebrew
  writes a tab only for the keg that it installs (`formula_installer.rb`
  648, 1117, 1770 and 1811). But `brew tab` and a repeated request can
  rewrite a receipt, and no reader of Ravel reads it. The fixture's legacy
  keg gains a worker, an SDK file and a receipt. Controls cover removal of
  the worker, a changed SDK file and an accepted receipt rewrite.
- F4 is a real issue of lower severity. Fix: `json_problems` requires
  exactly one JSON object in the evidence and in the receipt, before it
  tests any field. Controls cover an empty receipt and whitespace-only evidence.
- Applied in Ravel `5196906162d4768e7e37134431b932e23df0bf94` on
  `skill-directory-delivery`, pushed to `origin`. It changes the release
  scripts, `release-channel.test.sh`, `release.test.sh`, the Taskfile, the
  spec `docs/specs/release-channels.md` and one recovery command in book
  chapter 11. The chapter 7–11 drafts take the promotion rules from the
  spec.
- Results: `task release:lifecycle:test` passed
  (`target/release-lifecycle.ZwOyHP`). All 48 declared mutants were killed.
  Each of the seven new mutants fails the control written for its finding.
  The digests of every script, the Taskfile and the example receipt matched
  before and after the run. `shellcheck scripts/*.sh` passed.
  `task release:formula` passed with the real Homebrew loader
  (`target/release-formula.9zuBhm`). Today the UTC and UTC−12 dates of HEAD
  agree, so that run does not reproduce F2's window. The library control
  pins the window. The book builds, and its excerpt check passes. Ravel's
  STE checker and the generic `ste-lint.py` report no new problem in the
  changed spec or chapter.
- No release, tag, tap change or Homebrew installation ran. The keg
  `/opt/homebrew/Cellar/ravel/1.1.0` still holds its worker and reports
  1.1.0. The real promotion upgrade stays with
  `promotion-upgrade-rehearsal-k318`, which now also measures the manifest
  check against a real Homebrew upgrade.
