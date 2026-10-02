# current-state-documents-k15

## Goal

The usage documents, the configure-grove skill and the methodology describe the
result as current state, the release notes are written, and nothing in the
repository still describes Grove configuration as live.

## Context

- `docs/specs/harness-selection-and-execution.md`: the last paragraph of *Grove
  integration* and the documentation-review sentence under the test seams say
  what the documents must explain.
- Root brief: *Done when*, and the note *The release is a major one*.
- `direct-dispatch-k4`'s note *Records this design left for the leaf that
  deletes the machinery*, its last entry.
- The skill's `references/execute.md`, *Verifying a claim about the repo
  itself*, for the sweep.

## Done when

- The usage guide, the architecture document, the repository and plugin
  READMEs, the dispatch README and the configure-grove skill explain installing
  and editing the policy, the owner settings, the choice file, inspection, the
  launch-time boundary, a refused launch with its remedy, and the
  `GROVE_SIGNAL_FILE` warning. Each invocation they quote is one `--help`
  carries or the launch-boundary suite makes.
- The methodology's references that named `config.kdl` are rewritten: how a
  session is launched, where reviewer diversity is decided, and the task
  format's account of the kind.
- The release procedure, the release scripts and the Homebrew template say what
  an owner does now, including installing the sample before `grove run
  release-notes` is needed.
- The changelog's Unreleased section describes the release as a major one, by
  subject, with what breaks and what an owner does about it.
- A sweep of the whole repository finds no code, test, document, example or
  skill that implements or describes `config.kdl` or `.grove.kdl` as live, and
  none that describes the dispatch catalog, routes, slots or `--choice` as
  live. The sweep has a positive control that was seen to fail.
- Every bullet of the root brief's *Done when* is checked against the
  repository, and a gap that is work becomes a leaf.
- `bash scripts/check.sh` passes.

## Notes

- A mention that says an old file is ignored is current state and stays.
- Earlier leaves changed documents only as far as the checks required, so
  expect prose that is green and stale.
- The root brief's *On the horizon* note is the finish cycle's to promote.
- `runner-templates-k14` deleted `docs/CONFIGURATION.md` and unlinked it. The
  guides now point at the dispatch README where they pointed at it, and the
  prose around those links is unchanged. The configure-grove skill's two
  `Source:` lines still name the deleted file by URL.
- `${prompt}` was a template slot. The glossary's **Guaranteed core** and
  **Skill delivery** entries and the methodology still use it as the name of
  the session's prompt.
- `scripts/release-publish.sh` still describes writing a `config.kdl` in its
  comments.
