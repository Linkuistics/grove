# ravel-skill-publication-k67

**Integrates:** ravel-skill-publication-k63

## Goal

Triage `ravel-skill-publication-k63` against the current Ravel feature line
and apply the actionable findings. Keep each disposition explicit.

## Context

- Read the committed review through its stable handle
  `ravel-skill-publication-k63`. It reviews producer
  `ravel-skill-publication-k56` at Ravel commit `cba154857d41`.
- Work in `Ravel.skill-directory-delivery` on the local
  `skill-directory-delivery` bookmark. The review reconciled its findings
  with tip `aa0e6e88a8dd22cd5c293fa7099244710236ea27`; recheck any later drift.
- The parent brief owns the cross-repository boundary, checks and publication
  rules. This task owns fixes and post-fix verification.

## Done when

- Every finding has a reasoned disposition, with fixes for those accepted.
- Appropriate regression cases and affected Ravel checks verify the result.
- Record the Ravel output commit and verification evidence, advance the local
  bookmark through jj and leave publication to the existing rollout tasks.

## Notes

The review's findings are evidence to assess, not an instruction list that
cannot be rejected.

## Decisions (running log)

- Base: Ravel workspace `Ravel.skill-directory-delivery`, bookmark
  `skill-directory-delivery` at `aa0e6e88a8dd`, empty working copy. This is
  the tip that the review reconciled, so no later drift exists. The findings
  were read from the review commit `b222abf058be`. Each cited code path is
  unchanged at this tip.
- Finding 1, a replaced cache child directory: a real issue. `Cache::check`
  compares the root alone. Fix: hold the identity of each of the three
  directories, and check that both cache paths still reach each one.
- Finding 2, an unchecked staging name: a real issue in publication and in
  cleanup. Fix: compare the identity at the staging name with the held
  staging directory before the rename. Compare the published name with it
  after the rename too, which also covers a substitution between the two
  steps. Cleanup removes the contents through the held directory, and it
  removes the name only when the name still holds that directory.
- Finding 3, lossy UTF-8 in the readback: a real issue. Fix: a name or a
  link target that is not exact UTF-8 reads back with a spelling that holds
  NUL, so that no manifest entry can match it. Capture refuses NUL.
- Finding 4, a link that the cache filesystem does not resolve: a real
  issue. Fix: after the readback, resolve each admitted link in the written
  tree, on the cache filesystem. A link that is dangling there refuses as
  `skill_resource_invalid`. The check uses the cache filesystem and not
  exact spelling, so a link that works on a case-insensitive cache keeps
  working. That keeps the current behaviour of the default macOS setup.
- Finding 5, the concurrent editor fixture: a real evidence defect. Fix: the
  editor replaces the file by a rename from outside the source, so every
  successful capture holds one complete state. The fixture resolves until it
  observes a refusal, within a bound of attempts.
- Finding 6, the killed-publisher fixture: a real evidence defect. Fix: a
  child counts as interrupted only when it was killed after its staging
  directory appeared. The rename-to-seal window stays outside fixture reach,
  which the fixture states.
- No finding is noise or a trade-off, and none needs a redesign.
- Regression cases: five new cases failed against the unfixed sources,
  because each resolution succeeded where it must refuse. Four are internal
  fault cases in `src/skills/publication_tests.rs`: replaced cache
  directories, a staging replaced before the identity check, a staging
  replaced between the check and the rename, and a winner that differs only
  in bytes that are not UTF-8. The fifth is the public macOS case
  `a_link_that_only_the_source_filesystem_resolves_refuses_on_a_case_sensitive_cache`.
  It also proves that the link keeps working on a case-insensitive cache.
  All five pass with the fixes. The new fault hook `renaming` places the
  substitution between the check and the rename.
- APFS refuses a file name that is not UTF-8, so the file-name half of the
  UTF-8 case reports a skip on macOS and runs only on Linux. The link-target
  half runs on macOS. No Linux run occurred. `cargo clippy` for
  `aarch64-unknown-linux-gnu` on all targets with `-D warnings` is clean.
- Documentation: `docs/specs/skill-discovery.md` states the link rule on the
  cache filesystem, the readback of bytes that are not UTF-8, the checks
  before publication and the bound cleanup. `CHANGELOG.md` records the two
  refusals. The book quotes no changed source, and `task docs:excerpts`
  passes 12 of 12. The STE checker reports the same five hard findings in
  the specification as at the base, and no new finding in the changelog.
- Verification on macOS: `task check` exit 0, with format, lint, 724 of 724
  run tests passed, 48 skipped, and the fake-harness smoke test passed. The
  full public file `tests/skills_publication.rs` passes 11 of 11. After the
  check, only the prose of the specification changed.
- Ravel output commit `0cc7f8c4f3367ef8f7fd99aa151e196a1b72edc6`, parent
  `aa0e6e88a8dd`. The local bookmark `skill-directory-delivery` moved there
  through jj. It is not pushed. Publication stays with the rollout tasks.
- No in-session reviewer was spent. An executable case covers each fix.
