# changelog-release-rename-k18

**Kind:** impl

## Goal

Make `cargo release` rename `CHANGELOG.md`'s `## Unreleased` heading to the
version being cut, so the standing-heading convention `changelog-unreleased-k13`
ratified cannot be defeated by a releaser forgetting a manual step.

## Context

`k13` ratified `## Unreleased` as a standing heading that sessions log into as
they make changes. `release.toml` carries no `pre-release-replacements`, so
`cargo release` does not touch `CHANGELOG.md` at all — nothing renames the
heading. `k13` discharged that obligation the cheap way, in prose: a `BEFORE the
cut:` note in `release.toml`'s usage comment, beside the tag push and the
`jj new main` step it already documents as manual.

That is honest but weak. A forgotten rename files a release's entries under
"Unreleased" above its own empty version heading — visible only on inspection,
and wrong in the one file whose whole value is an accurate record of what changed
and when. `pre-release-replacements` is cargo-release's standard idiom for
exactly this and would make the rename structural.

`k13` did not do it inline because verifying it costs a `cargo release` dry run,
and `release.toml`'s own preamble records that the harness's command classifier
refuses `cargo release … --execute` as an opaque invocation — so the check likely
needs a human at the terminal. That is a session boundary, not a step.

## Done when

- `release.toml` renames `## Unreleased` to `## v<version>` as part of the cut,
  or the leaf records why automating it is worse than the prose note and the note
  is left as the answer.
- If automated: a dry run is observed rewriting the heading — not merely
  configured — and the `BEFORE the cut:` note in `release.toml` is reconciled to
  whatever ends up true.
- The `## Unreleased` paragraph in `CHANGELOG.md`'s preamble asserts
  "`cargo release` does not rewrite this file". If that stops being true, fix it
  there too.

## Notes

**Two things to check before writing the replacement.** The heading must survive
a release that cuts with an *empty* `## Unreleased` section — a release of binary
work only, where every entry is already under a version. And
`consolidate-commits = false` means replacements run per crate; grove releases one
crate (`harness-pane` is `release = false`), so this should be a single
application, but it is worth confirming rather than assuming.

**Re-seeding the heading is the other half.** A replacement that renames
`## Unreleased` and does not put a fresh empty one back leaves the next session
with the same nowhere-to-log problem `k13` was raised for.
