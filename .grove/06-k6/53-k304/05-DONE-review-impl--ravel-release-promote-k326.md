# ravel-release-promote-k326

**Reviews:** `ravel-release-promote-k306`

## Goal

Read Ravel's promotion task adversarially before its first real use. Finish
runs it after `finish-commit`, so no leaf can repair it then. That use pushes
to the public tap and moves the machine's default `ravel` command to 2.0.0.
Produce findings only.

## Context

- The artifact is the Ravel commit "Promote a complete ravel@2 release to
  unversioned ravel (ravel-release-promote-k306)" on the bookmark
  `skill-directory-delivery` in
  `~/Development/AgenticOrchestration/Ravel.skill-directory-delivery`. Its
  main file is `scripts/release-promote.sh`. It also changes
  `release-channel.sh`, `release-formula.sh`, `release-formula.test.rb`,
  `release-install.sh`, the Taskfile and `release-channel.test.sh`.
- The contract is Ravel `docs/specs/release-channels.md`, sections
  "Promotion without another release", "Promotion interface", "Consumer
  readiness receipt" and "Recovery and existing processes". Grove
  `docs/specs/coordinated-release.md` owns state D and the row "Promotion
  partial".
- The producer's decisions are in the running log of
  `ravel-release-promote-k306`.

## Done when

- Each doubt below has an answer that cites the source, a fixture or a
  reproduction.
- Each finding names its file and line, a concrete failure scenario and its
  severity.

## Notes

The fixtures replace gh and brew, and a stub replaces the Homebrew formula
test. `task release:formula` runs the real formula test. These doubts remain:

- The jj tap states. Look for a state that the classification admits but
  should refuse. Examples are a conflicted `main` after the fetch, a working
  copy away from `main`, and a remote that moves between the fetch and the
  push. Check that no refusal leaves a half-written tap.
- The evidence and the receipt. Decide whether the tag commit, the asset
  digests, the book commit and the consumer formula's version are enough
  identity. Find a receipt that the jq checks accept and that should refuse.
- The deprecation date is the tag commit's UTC date. Homebrew compares it
  with `Date.today` in local time. Find a window where `ravel@2` renders as
  not yet deprecated.
- `release-install.sh --version` runs from `main`, not from the tag. Check
  that its working-tree `installed-record.sh` and its Homebrew calls cannot
  accept the wrong keg. Compare the new retained-keg rule with Homebrew's
  real directory layout.
- The order. Check that every verification precedes the first tap write and
  the first Homebrew mutation. Check that a failed installation after the
  push leaves a state that a rerun completes.

The real upgrade rehearsal with Homebrew belongs to
`promotion-upgrade-rehearsal-k318`. Book chapter 11 belongs to the book
pipeline after this review.

## Decisions (running log)

- Reviewed producer `3f249355cd0a21daae617de5632c6bda1dbda3c5`
  against Ravel's current committed source at `62cccfdbb0d5` in
  `Ravel.skill-directory-delivery`. The intervening k327 change adds
  `--skip-link` for the first versioned installation and its fixture. The
  promotion script is unchanged. This review runs no test, build, lint,
  format, installation or publication command and makes no implementation
  change.
- The receipt parser admits a file with no JSON document. The initial
  hypothesis that its empty consumer values bypass promotion was disproved
  by `jj file show --help`: a directory argument recursively reads its
  files. Here the empty filename selects `Formula/`, whose version lines
  make the later comparison fail. Record the parser and misleading remedy
  as the lower-severity F4, not a compatibility bypass.
- The UTC tag date can be tomorrow in Homebrew's local timezone. Its real
  formula loader then reports no deprecation, and the promotion's formula
  check refuses. Record the bounded same-day failure as F2.
- The tap fetches explicitly from `origin`, but its push takes jj's
  configured destination. Record the wrong-remote publication and failed
  retry as F1. The installed `jj git push --help` documents this selection.
- The retained-keg check covers only `bin/ravel`. The real 1.1.0 keg also
  needs `libexec/ravel/ravel-policy` and its resources. Record the missing
  retained-resource verification as F3, against the coordinated acceptance
  requirement for controls that fail on removed retained resources.

## Findings

### F1 — P2: The promotion can push to a different remote from the one it verified

Location: Ravel `scripts/release-promote.sh:189`.

Scenario: a clean jj tap has `origin` for the public tap and a second remote,
with `git.push` configured to select that second remote. Both remotes can
initially hold the same state B. Preflight fetches and verifies `origin`
(`release-channel.sh:147` and `release-promote.sh:285`), but the promotion
runs `jj git push -b main` without naming a remote. The installed jj help
states that this uses `git.push`, independently of the tracked bookmark.
The promotion therefore publishes to the second remote. Its subsequent
fetch from `origin` still finds state B and refuses at line 233, after the
wrong remote has already changed. The advertised retry repeats that push
destination and cannot complete publication to `origin`.

The accepted tap contract requires a jj repository and a clean working
copy; it does not exclude a configured push destination. Use one explicit
destination throughout fetch, verification and push, or refuse a mismatch
before writing the promotion. The fixture has only one remote and covers a
rejected push, not selection of another destination
(`release-channel.test.sh:646–660,722–733`). This is a source-derived
failure scenario; no push was performed in this review.

### F2 — P2: A same-day cut can refuse promotion west of UTC

Location: Ravel `scripts/release-promote.sh:300–304`; also
`scripts/release-formula.sh:84–88`.

Scenario: the tag commit is dated `2026-11-12T00:15:00Z`, and promotion runs
at `00:30:00Z` in a timezone eight hours behind UTC. The rendered
deprecation date is `2026-11-12`, but the machine's local date is still
`2026-11-11`. The installed Homebrew implementation parses that date and
sets `deprecated` only when it is at most `Date.today`
(`/opt/homebrew/Library/Homebrew/formula.rb:5300–5310`). Consequently
`release-formula.test.rb:95–96` raises before any tap mutation. The same
date choice also makes the ordinary formula task fail in this window.

Using the tag date does make retries deterministic, but does not ensure
that the formula is already deprecated in the local timezone. Settle an
effective date that preserves both properties. Merely forcing the local
test to UTC would still leave users west of UTC seeing a future
deprecation date. The promotion fixture stubs this Homebrew test
(`release-channel.test.sh:629–637`), so its saved passing result cannot
exclude this window. This is source evidence, not a simulated-clock run.

### F3 — P2: The retained-prefix check accepts removal of the legacy worker

Location: Ravel `scripts/release-install.sh:108–112`; related fixture
`scripts/release-channel.test.sh:763–780,809–811`.

Scenario: the retained 1.1.0 keg keeps its executable `bin/ravel`, but its
`libexec/ravel/ravel-policy` or SDK resources are removed during a package
operation. The new check still passes, and the formula test exercises only
the selected 2.0.0 installation. Promotion can therefore record success
without an intact original prefix. This is a gap in detection, not a claim
that the current Homebrew upgrade normally removes those resources.

The real `/opt/homebrew/Cellar/ravel/1.1.0` contains that private worker,
SDK, examples and notices. Its installed `.brew/ravel.rb:35–40` installs
both the front executable and `libexec/ravel`, explicitly describing the
matching worker. An executable front alone cannot establish that legacy
launches still work. Grove's `docs/specs/coordinated-release.md`,
"Acceptance and limits", explicitly requires controls to fail on removed
retained resources, and "Recovery and existing processes" in Ravel's spec
requires full original prefixes.

The fixture's legacy keg is a shell script that only prints a version
(`release-channel.test.sh:437–449`); its cleanup fault removes the whole
keg (line 482). Neither distinguishes this failure from a complete legacy
installation. Verify the required legacy resources as well as the front,
and give the retention seam a resource-removal control. The actual
Homebrew upgrade rehearsal remains with k318.

### F4 — P3: An empty receipt escapes local validation and produces the wrong remedy

Location: Ravel `scripts/release-promote.sh:63–67,94–112`.

Scenario: receipt generation was interrupted after opening its output
file, leaving a regular zero-byte or whitespace-only file. The file test
passes. `jq -r` receives no JSON document, invokes none of the schema
checks, emits no problems and succeeds. The validator therefore accepts
the missing receipt and starts public-release verification.

The command does eventually refuse: its empty consumer filename makes
`jj file show` read all of `Formula/`, and the first formula version cannot
equal the empty consumer version (`release-promote.sh:270–271,287–289`).
The resulting error instead directs the operator to publish and verify a
consumer release for an empty version. This is not permission to promote
before compatible Grove, but it violates the local-input validation and
actionable-remedy contract. Require exactly one JSON object before running
the field checks. The receipt fixtures cover an absent path and malformed
fields, but not an existing file with no document
(`release-channel.test.sh:689–699`).

## Answers to the mandated doubts

### Tap states and refusal boundaries

The clean-working-copy check is at `release-promote.sh:199–200`.
The remote promotion is retained only when local `main` equals
`main@origin` (lines 205–208). An unpushed promotion must be exactly one
commit above the fetched remote, have that remote as its parent, contain
both expected renderings, and change only the two Ravel formulae
(lines 221–225). That excludes the ordinary divergent/conflicted-main
case after a fetch. The nonempty check at line 203 alone does not establish
that `main` is single; the later comparisons supply the refusal. No fixture
here specifically exercises conflicted bookmarks.

An empty working-copy commit away from `main` is admitted. For a new
promotion, `jj new main` then switches to a child of the intended bookmark
(line 174). The earlier committed work remains in jj; this case does not
by itself lose work or publish its contents. A remote advance between the
fetch and push is protected by jj's last-fetched-state check, documented
by the installed `jj git push --help`. The rejected-push fixture verifies
that a local promotion remains recoverable. F1 concerns the separate
failure to pin which remote receives that push. Concurrent writers to
the same local tap remain outside this transition's sole-executor
contract.

### Release identity and consumer receipt

The tag is compared with the pinned commit (lines 118–123), all three
downloaded assets with their recorded SHA-256 values (lines 129–137),
and the Pages commit that last changed the versioned book with the cut's
book identity (lines 144–149). Both formulae render from those assets and
the template at the tag commit (lines 293–301). The remote versioned
formula must match the ordinary or deprecated rendering
(lines 205–215). These checks bind the promoted release without requiring
the cut's tap ancestry.

For a nonempty valid receipt, the tag and asset identities must match the
release evidence, routes must be a nonempty string array, and the fetched
consumer formula must carry the receipt's version. The coordinating owner
is trusted to assert that its named routes passed. The design expressly
avoids a consumer-version table and interpreting route names; absence of
such independent semantic verification is an accepted contract boundary.
F4 supplies an input that the jq checks wrongly accept, while explaining
why the later tap check still prevents promotion.

### Deprecation date

F2 answers this doubt. Homebrew uses a local calendar date, while rendering
uses the tag commit's UTC date. No real formula invocation was run here.

### Installation away from the tag and the retained keg layout

`--version` deliberately avoids deriving a version from the working
checkout. The installer then checks the requested version in Homebrew's
installed list, the concrete cellar path, the resolved prefix, linked-keg
metadata, the shared command's file identity, the selected executable's
reported version and its installation record
(`release-install.sh:115–140`). The working-tree `installed-record.sh`
requires exact record bytes for schema 1 and the shared skill directory.
Using a different PATH executable or another selected version does not
satisfy these checks. They verify the selected installation's stated
contract; they do not independently hash an already installed binary
against the downloaded archive. The receipt inputs and current helpers
are trusted local artifacts.

The real 1.1.0 layout differs from the fixture's self-contained version
printer. F3 records the consequent retained-resource verification gap.

### Order and recovery after publication

The public tag, asset, book and consumer checks and the real formula-method
test precede `publish_promotion` (lines 279–306). The first formula copy
is line 175; the first installation mutation is `brew update` in
`release-install.sh:93`, after the push. Subject to F4's local-input
validation gap, no earlier missing identity deliberately falls through to
those mutations.

An installation failure leaves the pushed formulae in place and reports
the repair-and-retry route (lines 308–309). On retry, matching remote
formulae are retained, then installation is attempted again. Existing
fixtures cover the matching-installation retry and refusal of a corrupt
installation, plus recovery of a rejected tap push
(`release-channel.test.sh:722–733,782–811`). They do not establish that
every real Homebrew interruption repairs itself: an unlinked or damaged
installation can require the explicit installation repair that the error
message names. No new cut is needed. F1 identifies a configured state
where the advertised publication retry cannot progress unchanged.

## Evidence limits and handoff

This review is a source inspection. The Ravel graph project is
`Users-antony-Development-AgenticOrchestration-Ravel.skill-directory-delivery`,
generation `2026-11-02T05:30:19Z`. Coverage excludes `scripts/` and `docs/`
and reports changed metadata for `Taskfile.yml`; all relevant scripts and
the task definition were read directly. No exhaustive graph claim is made.

The producer's saved `target/release-lifecycle.ItSdSk` records its promotion
controls and mutant failures. It is an ignored local artifact, not fresh
verification of this review's scenarios. The previous integration records
its later run separately. This session ran no test, build, lint, format,
installation or publication command. The Ravel working copy remains empty.
The paired integration owns triage, corrections and new verification;
k318 owns the real Homebrew rehearsal.
