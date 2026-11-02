# ravel-skills-resolve-k66

## Goal

Triage `ravel-skills-resolve-k62` and apply the actionable findings to the
current Ravel feature line. Keep each accepted or rejected finding explicit.

## Context

- Read the review's committed findings through its stable handle
  `ravel-skills-resolve-k62`. It reviews producer `ravel-skills-resolve-k54`.
- Work in `Ravel.skill-directory-delivery`, on the local
  `skill-directory-delivery` bookmark. The feature line has advanced since
  the reviewed commit; reconcile the findings with the current implementation.
- The parent brief's ownership, publication and verification rules bind.

## Done when

- Every finding has a reasoned disposition, with fixes for those accepted.
- Applicable Ravel checks and meaningful regression cases verify the result.
- Record the Ravel output commit and verification evidence, advance the local
  bookmark through jj, and leave publication to the existing rollout tasks.

## Notes

This task owns fixes and post-fix verification. It does not treat a review
finding as an instruction that cannot be rejected.

## Decisions (running log)

- Base: Ravel workspace `Ravel.skill-directory-delivery`, bookmark
  `skill-directory-delivery` at `ba0f388ecb63`, empty working copy. The
  findings were read from the review commit `16ee116f04a`. Each cited code
  path is unchanged at this tip, so each finding applies to it.
- Finding 1, cold cache below an aliased ancestor: a real issue.
  `Protected::new` appends the cache suffix to the canonical HOME, and
  canonicalises only complete paths. The cache walk follows links segment by
  segment and creates absent segments, also through a dangling link. Fix:
  project the real path that the walk would reach, with no writes, and
  protect it.
- Finding 2, a case alias of a reserved entry: a real issue. The link walk
  compares the literal component only. Fix: refuse a component whose entry
  is a reserved entry of its directory by identity, unless the exact
  component name is a distinct listed entry (a hard link to a reserved file).
  An alias of an ordinary entry keeps its current behaviour; this leaf
  changes only the reserved rule.
- Finding 3, duplicate mandatory references: a real issue. The specification
  says that two references to the same file refuse. Fix: after each
  mandatory check, key the reference by the snapshot digest and the real
  path inside the captured tree. A repeated key refuses as
  `skill_mandatory_invalid` at the later reference, because the descriptor
  would order one retained file twice. That includes two distinct origins
  with identical content, which share one retained file.
- Finding 4, an unreadable target through a link: a real issue. Fix: add
  the real path inside the source of each mandatory reference to the set
  that capture consults, so an unreadable target reports as
  `skill_mandatory_invalid`.
- No finding is noise or a trade-off, and none needs a redesign.
- Documentation: the discovery specification now names
  `skill_mandatory_invalid` for duplicates that only capture can detect. It
  also covers a link to another spelling of a reserved entry and the cold
  cache path. `CHANGELOG.md` records the three refusals. The book quotes none
  of the changed source, and `task docs:excerpts` passes 12 of 12. The other
  checker findings in the specification predate this change.
- Verification on macOS (APFS, case-insensitive): four new public fixtures in
  `tests/skills_resolve.rs` failed against the unfixed sources and pass with
  the fixes. A fifth, the hard link of a reserved file, passes both ways. It
  guards against an over-broad reserved check. `task check` exit 0: format,
  lint, 719 of 719 run tests passed with 48 skipped, and the fake-harness smoke
  test passed. No Linux run: the reserved-alias fixture returns early on a
  case-sensitive filesystem, and the other fixtures are portable.
- Ravel output commit `aa0e6e88a8dd22cd5c293fa7099244710236ea27`, parent
  `ba0f388ecb63`. The local bookmark `skill-directory-delivery` moved there
  through jj. It is not pushed. Publication stays with the rollout tasks.
- No in-session reviewer was spent.
