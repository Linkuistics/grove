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

## Decisions (running log)

**`${prompt}` stays as the repository's name for the prompt.** It is in source
comments three books reproduce, in ADRs, tests and the conformance manifest's
owner column, and none of those describes a slot as live. The glossary's
*Guaranteed core* entry now says what the spelling is and that nothing expands
it. The shipped methodology says *prompt* where it used the spelling as prose.
Renaming it everywhere would rewrite three books' literal bodies for no
behaviour.

**The usage guide carries a launch-policy section.** The coverage inventory
kept launch policy out of the guide, and the dispatch specification asks the
guide to explain it. Both hold: the guide says what a Grove owner does and
links the contract, and the inventory's boundary now says so.

**The guide's transcripts are regenerated, not edited.** They were produced by
the built `grove` and `harness-dispatch` under the installed sample policy, with
fake `codex` and `claude` programs on PATH, in a scratch HOME and jj tree.

**The configure-grove skill keeps its name and has one mechanics reference.**
`references/configuration.md` and `references/dispatch.md` described two owners
of a launch. There is one, so `references/policy.md` replaces both.

**Four conformance rows are renamed with their rules**: `one-policy`,
`policy-edit-lands-next-session`, `diversity-is-the-policys` and
`escalated-review-routes-through-policy`. The pinned phrases a Rust test holds
are unchanged.

**`.gitignore` keeps `.grove.kdl`**, with a comment. A copy left in a checkout
is ignored by Grove, and should not be snapshotted by jj either.

**Dated records are left as written.** `docs/preservation-baseline.md` (which
carries its own note), `docs/formalism-findings.md`, `docs/research/`,
`docs/evaluations/`, `docs/plans/`, the parity fixture's README and the
changelog's versioned sections describe configuration as it was. None claims to
be current state.

**Three defects the sweep found are fixed here, because each is one line.** The
Homebrew formula's `test do` wrote a `schemaVersion: 1` catalog policy, which
this release refuses, so `brew test` would have failed. `inspect --help` called
its JSON version 1 where it emits version 2. The publish script's documented
smoke recipe wrote a fake harness whose `%s` the outer `printf` consumed, so it
printed an empty signal path.

**The sweep enumerated, then classified.** One ripgrep run over the whole
repository with `--hidden`, outside `.jj`, `.git`, `target`, `node_modules` and
`.grove`, extracted every line carrying a configuration or catalog-contract
token: the file names, `grove config`, the template and definition vocabulary,
the `${…}` slot spellings, and `catalog`, `candidate`, `slot`, `--choice`,
`explicitChoice`, `selectedBy` and `policy.routes`. Every file with a hit was
classified by reading its hits. What remains is one of: a statement that
something is absent or ignored; a retired glossary term; a dated record; a
test or fixture that plants an old file or an old policy shape to show it is
ignored or refused; the launch document's retained `candidate` field and the
21.13.0 records dispatch still reads; or another sense of the word (a position
slot, a candidate entry, a shell or TypeScript `${…}`). A second run for
configuration said in other words (profile, binding, delta, template, personal
file) found two more stale passages, fixed here: an ADR's *delta search* and a
book's *template selection*.

**The sweep's controls.** A file planted in a hidden directory, holding three
sentences that describe configuration and the catalog as live, was reported by
the instrument. The same command without `--hidden` missed it, and so did a
mutated pattern, so the control has been seen to fail. The pattern also still
matches where the old vocabulary legitimately lives: the changelog's 21.13.0
section and the preservation baseline. The planted file was removed.

**The root brief's *Done when*, bullet by bullet.**
*Both launches go through dispatch*: `loop_driver.rs` builds the one
`harness-dispatch run` invocation and `standalone.rs` runs `inspect --json`; no
source reads a Grove configuration file. *Old files change nothing*: held by
`old_configuration_files_left_on_disk_change_nothing_about_a_launch`, its
`grove run` and tree-verb counterparts, and the sweep above. *One `select`,
every example to that contract*: four examples and the sample, type-checked by
`task dispatch:typecheck`. *A scripted deciding agent*:
`crates/harness-dispatch/tests/deciding_agent.rs`. *The sample reproduces the
pinned configuration, `init` installs it, a helper reads the choice file*:
`tests/sample.rs` against the parity fixture, and `tests/choice_file.rs`. *The
documents describe current state*: this leaf. *`scripts/check.sh` passes*: run
on the finished tree. No bullet left a gap that is work, so no leaf is cut.

**The in-session reviewer was spent once, and no review leaf is cut.** The
claim was that the rewritten owner-facing documents say only what the
specification, `--help` and the code do. One fresh context was given the
documents and those sources with a *find what is wrong* prompt. It returned
fifteen findings. Fourteen were valid and are fixed: each was a wording
correction checkable against a source line, and the one transcript finding was
confirmed against real output (the refusal's `inspect:` line names
`harness-dispatch` by its full path, which the path rewriting had dropped). One
half of one finding is a trade-off `TASK-FORMAT.md` already states: a body
carries nothing that routes its session, and the `**Creator:**` line is read by
a policy, never by Grove. The fixes are mechanical, so they do not earn a second
read, and the documents are corrected by an edit if a later reader finds more.
