# grove.add-release-notes-task — brief

## Goal

Provide a standalone Taskfile command for generating release notes through
`grove run`, with the skill and configuration support that invocation needs.
The notes cover all changes since the last release, not just the latest change.

## Done when

- `task release:notes` can generate notes independently of publishing a release.
- Its evidence includes the full commit descriptions and complete net source diff
  for all changes since the last release, plus the prior changelog for context.
- An explicit run refreshes Unreleased using its existing content as additional
  input. Newly added changes are incorporated without duplicating entries, and
  versioned release sections remain unchanged.
- Generation uses a personally configured `grove run release-notes` invocation,
  with visible execution and validated Markdown output.
- The generation logic is shared with the existing full-release preparation.
- The configured command is usable in the standalone runner's scratch directory.
- The agreed behavior is documented and tested at the existing script/runner
  boundary.

## Decomposition

Two small vertical increments: expose and test the reusable command, then
configure and prove the actual headless invocation on this machine. The first is
verifiable through the existing deterministic runner fixture; the second closes
the runtime/configuration gap with a live confined invocation. Each fits a
focused implementation session.

- `release-notes-k2`: reusable command, shared writing skill, deterministic tests
  and documentation.
- `release-notes-runner-k3`: actual personal headless configuration and live
  confined verification.

## Command behavior

`task release:notes` works from the current jj workspace. It snapshots the current
change as the endpoint and supplies the whole interval from the last release in
that history, including unreleased ancestors. It does not fetch, change parents,
move bookmarks, cut a version, or publish. The full release workflow keeps its
existing `main` endpoint and release actions.

The explicit notes command refreshes Unreleased. It supplies existing Unreleased
text to the writer as context, together with the prior release's changelog,
full commit descriptions and the complete source diff. The writer describes
shipped behavior, fixes, compatibility changes and required user actions; it
omits routine internal churn and does not invent unsupported claims. The section
body must be nonempty and contain no `##` heading. Failed execution or invalid
output leaves the changelog unchanged. Historical sections and unrelated content
are preserved. No-change and missing-baseline cases report a clear diagnostic
without fabricating notes.

Keep the writing instructions in one standalone release-notes skill and stage
them explicitly with the invocation, so both the new task and automatic release
preparation use the same instructions. This skill works only with supplied
artifacts and follows the runner's supplied completion command; it does not
bootstrap, edit or retire a Grove task tree. The task must not depend on the
sandbox discovering a globally installed skill.

Extract generation and validated changelog updating from the existing preparation
script. Full release tasks retain their current fill-only-when-empty behavior,
including preserving notes that have already been reviewed; the explicit
`release:notes` command is the refresh operation.

The `release-notes` route selects a dedicated headless command through personal
configuration. Keep harness/model/policy selection there. Reuse the user's
existing preferences when defining it, confine writable harness state to the
invocation directory, and explicitly grant needed runtime files. Preserve
existing routes and profiles. Document the reusable setup and verify the actual
route with one live smoke invocation; deterministic tests do not call a model.

## Pointers

- `Taskfile.yml`, `scripts/release-prepare.sh` and
  `scripts/release-prepare.test.sh`: existing release workflow and test seam.
- `docs/RELEASING.md` and `docs/CONFIGURATION.md`: release and standalone
  configuration contracts.
- `crates/grove/src/standalone.rs`: the runner resolves personal policy, stages
  input files and appends its own completion instructions. It does not load a
  kind skill automatically.
- `CHANGELOG.md`: one Unreleased section, shared notes for everything the repo
  ships, prior release sections retained.

## Notes

The current script mixes evidence collection, generation and changelog writing
with fetching, changing jj parents and moving `main`. A standalone notes command
needs generation separated from those full-release actions.

The installed personal configuration currently has interactive Codex and Claude
commands and no `release-notes` route. An actual headless route must be verified;
pointing the new route at an interactive command is not sufficient.

## Agreed test seams

Extend the existing shell integration fixture: real temporary jj repositories,
with a deterministic fake substituted only at the build/standalone-runner
boundary. Exercise several changes after a tagged release so a latest-change-only
implementation fails. Cover notes validation, updates to Unreleased, preservation
of versioned history, and failure leaving the changelog unchanged. Check that the
standalone command leaves jj parents and bookmarks where they were, and retain
the existing full-release preparation checks. A separate live smoke invocation
verifies the personal headless route and confinement. Human agreement on these
seams was recorded in `plan-k1`.
