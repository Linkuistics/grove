# ravel-skill-confinement-k69

**Integrates:** ravel-skill-confinement-k65

## Goal

Triage the findings of `ravel-skill-confinement-k65` against the current
Ravel feature line. Apply the findings that hold and record the disposition
of every finding. Own the fixes and their verification.

## Context

- Read the committed review by its handle. Its findings refer to Ravel
  `e343587583eb`, parent `4b0f54c575a7`.
- Follow the local-delivery parent brief's cross-repository ownership and
  handoff contract. Preserve later integrations already on the feature line.

## Done when

- Each finding has an evidence-based disposition.
- Accepted changes have appropriate regression evidence and affected checks.
- The owning documentation and the recorded Ravel output commit agree with
  the resulting behaviour. Advance the local bookmark through jj without
  pushing it.

## Notes

## Decisions (running log)

- Base: Ravel workspace `Ravel.skill-directory-delivery`, bookmark
  `skill-directory-delivery` at `f1aff8e5562f`, with an empty working copy.
  The findings were read from the review commit `9ea7772081e`. The cited
  confinement and acquisition code is unchanged between the reviewed
  commit `e343587583eb` and this tip.
- Finding 1, the changed grant destination: a real issue. The managed
  check, the harness preparation and `keyed-launch` each canonicalise the
  path string again. A tree replaced by a link to the protected directory
  becomes a grant of that directory. Fix: `keyed-launch` holds each read
  tree by its canonical path and its device and inode. The probe and the
  launch refuse a tree whose canonical path or identity changed. The
  managed check gives its held trees to each harness run, which verifies
  them in place of a fresh resolution.
- Finding 1, residue: a visible trade-off. Both backends name a tree by its
  path. A replacement in the interval between the last identity check and
  the bind of the backend is not closed, and Seatbelt matches the path at
  each access after the launch. A replacement inside the cache needs a
  process of the same account, which the cache does not authenticate
  against. The specification records this limit.
- Finding 2, the source that holds the inherited result channel: a real
  issue. The specification promises a refusal for a source boundary that
  overlaps a known result channel, and `Protected::new` omits the
  inherited `RAVEL_EXIT_FILE`. Fix: the protected set of acquisition
  includes that channel, so `skills resolve` and every launch refuse such a
  source before capture, with confinement or without it.
- No finding is noise or a contract stated unclearly, and neither needs a
  redesign.
- Public route codes, read from source: a harness run whose preparation
  fails gives the script `harness_launch_failed`. A runner error before the
  managed process starts is `supervision_failed`, exit 126. The
  documentation states these codes, not `skill_access_unavailable`, for a
  change after the check. The old message "is not a directory" for a tree
  that is a file is now the runner's "must be a directory other than /".
  No test or document named the old text.
- Regression evidence: `keyed-launch` `tests/confinement.rs` replaces a
  held tree with a link to a protected directory, then with a fresh
  directory. The probe and the launch refuse each, and nothing runs. The
  Ravel unit test `a_harness_run_refuses_a_tree_that_changed_after_the_managed_check`
  shows that a held harness basis refuses both changes. Its control shows
  that the unheld basis grants the new destination. The public fixture
  `a_source_that_holds_the_inherited_result_channel_refuses_before_capture`
  covers `skills resolve`, an unconfined launch and a confined launch, each
  directly and through an alias, with a success control.
- Mutation controls, each run and then reverted: `ReadTree::verify` that
  always passes fails the `keyed-launch` test and the Ravel unit test.
  `Basis::holding` as a no-op fails the unit test. Removing the channel
  from `Protected::new` fails the public fixture.
- Documentation: `docs/specs/skill-discovery.md`, "Read access and
  delegation", states the inherited channel, the held trees, the route
  codes and the path-based residue. `docs/specs/session-execution.md`,
  `CHANGELOG.md`, book chapter 5 and the chapter 9 test table agree. The
  book STE check reports 0 problems in both chapters, and a planted
  sentence reports problems. Two long paragraphs in the specification
  section are older than this change.
- Verification on macOS, after the last edit, with the changed files
  digested before and after: `task check` exit 0, with 727 of 727 run tests
  passed and 48 skipped. `task docs:book:build`, `task docs:excerpts:test`
  (24 of 24 mutants killed), `task docs:breaks:test`, `task docs:ste:test`
  and `task book:integrity` exit 0. `cargo clippy` for
  `aarch64-unknown-linux-gnu` on all targets with `-D warnings` is clean.
  `task test:native-linux PLATFORM=arm64` exit 0 in the
  `ravel-native-linux:arm64` image, Ubuntu 26.04.1, kernel 7.0.14-linuxkit:
  720 of 720 run tests passed, the three new tests among them, with its
  subjects unchanged. No amd64 run occurred.
- Ravel output commit `bc737938a72e291a67735c366a0c3e054db5401f`, parent
  `f1aff8e5562f`. The local bookmark `skill-directory-delivery` moved there
  through jj. It is not pushed.
- No in-session reviewer was spent. An executable test with a firing
  control covers each fix.
