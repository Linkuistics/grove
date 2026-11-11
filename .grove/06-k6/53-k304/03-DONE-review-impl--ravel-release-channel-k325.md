# ravel-release-channel-k325

**Reviews:** `ravel-release-channel-k305`

## Goal

Read Ravel's release-channel tooling adversarially before its first public
use. That use publishes `ravel@2` to GitHub, Pages and the tap, which cannot
be undone. Produce findings only.

## Context

- The artifact is the Ravel commit "Carry one explicit release channel
  through the release tasks (ravel-release-channel-k305)" on the bookmark
  `skill-directory-delivery` in
  `~/Development/AgenticOrchestration/Ravel.skill-directory-delivery`.
- The contract is Ravel `docs/specs/release-channels.md`, sections "One
  release, two formula channels", "Release interface" and "Recovery and
  existing processes".
- The producer's decisions are in the running log of
  `ravel-release-channel-k305`.

## Done when

- Each doubt below has an answer that cites the source, a fixture or a
  reproduction.
- Each finding names its file and line, a concrete failure scenario and its
  severity.

## Notes

The producer's own fixtures replace gh and brew. They cannot settle these
doubts:

- Real Homebrew 7.0.13 behaviour for a first `brew install
  linkuistics/taps/ravel@2` beside a linked unversioned `ravel` 1.1.0. Does
  anything relink, upgrade or clean up the 1.1.0 keg that the running driver
  uses? Does `brew --prefix ravel@2` resolve to the keg before and after the
  opt link exists? Does `brew info --json=v2` report
  `keg_only_reason.reason` as `":versioned_formula"`?
- Task 3.54 reads environment values as variables. An exported `CHANNEL` in
  the operator's shell therefore satisfies the required variable. Decide
  whether that defeats "omission never publishes unversioned `ravel`".
- `release-prepare.sh` predicts the target with `next_version`. The
  admission then runs after `jj new main`. Check that every refusal leaves the
  owner's work recoverable and that no refusal comes after a mutation of
  `main`.
- Publication checks admission against the tap at `check_tap`'s fetch. Look
  for a window between that fetch and the tap commit, and for a resume that a
  stale `target/dist` misleads.
- The formula test now writes `"sources": []` into its `HOME`. Check that
  the real `brew test ravel@2` still exercises the default skill directory
  through the relocated copy.

The recovery prose of book chapter 11 is out of scope. The book pipeline
after this review owns it.

ravel-release-promote-k306 changed two files of this artifact after its
commit. `release-install.sh` gained `--version` and a check that every earlier
`ravel` and `ravel@2` keg keeps its `bin/ravel`. `release-formula.sh` gained
`--test` and loads the promotion's deprecated `ravel@2.rb` through Homebrew.
Read the current files on `skill-directory-delivery`.

## Decisions (running log)

- Reviewed Ravel producer `ba3298f2c70585f3d8ece57249e251eecda4c108`
  against the current committed files at
  `3f249355cd0a21daae617de5632c6bda1dbda3c5` (`skill-directory-delivery`). The latter includes k306's
  changes to installation and formula testing. Ravel's working copy was
  empty. This session ran no test, build, lint, format, install or publication
  command. Source inspection and the producer's saved lifecycle output are
  the evidence; they do not establish a new passing run.
- The installed Homebrew is `7.0.13-45-g95d4938`. Its installer and the
  upstream `7.0.13` source both automatically link a requested versioned
  keg when no related formula is installed. This contradicts the channel's
  unconditional promise that its installation changes no default command.
  Record this as F1 and hand it to an integration before publication.
- The other mandated doubts have source-backed dispositions below. Cut the
  paired integration immediately after this review, before the promotion
  review, so that no intervening edit moves the finding's source locations.

## Findings

### F1 — P2: A first versioned installation can become the default command before refusing

Location: Ravel `scripts/release-install.sh:96` (and the check at line 117).
The owning contract is `docs/specs/release-channels.md`, "One release, two
formula channels": versioned installation writes no competing command in
the shared prefix. Grove's coordinated-release state B makes the same
promise for fresh installation.

Failure scenario: use `release-install.sh --channel versioned-2 --version
2.0.0` on a Homebrew prefix with neither formula installed. The plain
`brew install linkuistics/taps/ravel@2` does not suppress linking.
Homebrew's `FormulaInstaller#auto_link_versioned_keg_only?` returns true for
this case, and its constructor enables linking even though the formula is
keg-only. The shared `bin/ravel` therefore becomes 2.0.0. The script then
refuses because `linked_keg` is non-null, leaving the changed command in
place. A retry does not undo that link. A direct public installation has
the same automatic-link behaviour.

This is derived from the
[Homebrew 7.0.13 installer](https://github.com/Homebrew/brew/blob/7.0.13/Library/Homebrew/formula_installer.rb#L1847-L1856),
its constructor at lines 124–126, and Ravel's install/check order above.
It was not reproduced by installing a formula. It does **not** imply that
the owner's existing linked 1.1.0 installation will be relinked: the
related-formula check excludes that case.

The installation fixture always seeds unversioned 1.1.0
(`scripts/release-channel.test.sh:437`). Its optional linking fault tests
that a changed installation is detected after mutation; it does not test
prevention on a fresh prefix. Integration should settle how the channel
prevents automatic linking, including its public installation instructions,
and cover a prefix without either formula.

## Answers to the mandated doubts

### Homebrew beside the linked 1.1.0 keg

No defect found for the stated existing-installation case. The installed
`formula_installer.rb:2003–2015` rejects automatic linking when the related
unversioned formula has an installed version. `link` at lines 1340–1357
creates the opt link without linking the shared command in that case.
Ravel's formula has no dependencies that would request an unversioned
upgrade. `HOMEBREW_NO_INSTALL_CLEANUP=1` suppresses the install cleanup
paths (`cleanup.rb:349,371,427,440`). This is source evidence, not an actual
installation rehearsal. F1 concerns the distinct fresh-prefix case.

`brew --prefix ravel@2` prints the opt path once the formula is known; it
does not need an existing opt link (`formula_path.sh:80–87` and
`cmd/--prefix.rb:78–85`). That path cannot resolve to a keg before the link
exists. Ravel resolves it only after installation. On this machine the
unpublished formula is currently unknown, so the read-only command refuses.
`KegOnlyReason#to_hash` (`keg_only_reason.rb:67–78`) serializes the symbol
with `inspect`, producing `":versioned_formula"`, as the script expects.

### CHANNEL inherited from the operator's environment

No additional finding. Task accepts an exported `CHANNEL` as supplied
input, as the producer explicitly recorded. This weakens the description
"omission refuses" if omission means only omission from the command line.
It does not permit the dangerous first unversioned 2.0.0 publication:
`release-channel.sh:87–92` refuses that target while unversioned Ravel is
below major 2, and preparation applies this before notes or the cut
(`release-prepare.sh:84–89`). An inherited `versioned-2` selects the intended
channel. Every downstream script receives an explicit argument. The
accepted trade-off therefore preserves the coordinated transition's
publication boundary.

### Version prediction and preparation refusals

No defect found in the bounded stable-version path. `next_version`
(`release-channel.sh:188–197` in the current file) uses the requested bump.
The producer's `release-prepare.test.sh:115–125` compares the zero-version
predictions with real cargo-release previews and checks nonzero examples.
The cut configuration is `release.toml`; it supplies no alternative bump.

Preparation snapshots the owner's change, prints its change ID and the
`jj edit` remedy, then opens a new change on main
(`release-prepare.sh:40,58–60`). Admission and every explicit refusal in
this script precede the `main` bookmark update at line 108. Refusal after
`jj new main` can change the displayed working copy, but the saved owner
change remains recoverable. The existing fixture checks its file by the
saved ID (`release-prepare.test.sh:63–64`). No refusal branch follows a
mutation of the `main` bookmark in preparation.

### Tap fetch, publication and stale distribution files

There is a window: admission at `release-publish.sh:206` uses the fetch
from lines 38–60, and publication copies the formula at line 127 after
GitHub/book work. A remote-only advance does not silently become the base
of this commit: the script does not fetch again, and its jj push (line 137)
or ordinary Git push (line 141) must reconcile a remote advance. An
unpushed local tap commit is recoverable, and the next preflight refuses
it with the explicit integrate-or-push remedy (line 48). The contract's
single release owner remains necessary: the script does not pin `main`
against another process that changes this same local tap clone during the
window. No such concurrent local writer is authorized in this transition.

A distribution from a different version lacks the three expected archive
names or fails rendering/comparison (`release-publish.sh:83–97`). A changed
archive with a stale formula also fails that comparison. `--tap-only`
additionally downloads public assets and compares their bytes (lines
145–159), then verifies the book before touching the tap (lines 208–211).
Rebuilding a completed old tag and intentionally resuming it can still
republish an older channel version: this interface checks channel admission,
not release recency. That is outside recovery of the current unfinished
cut and is not counted as a defect in this review.

### Default-directory coverage in the formula test

No defect found. The empty source list belongs only to the exact-prompt
HOME (`scripts/templates/ravel.rb.tmpl:57–58`). The later copy has its own
installation record, a fixture source and a separate HOME with no settings
(lines 96–107). The test checks default resolution and the source path
(lines 108–111), launches the relocated binary and checks the selected
default directory (lines 116–123). The Homebrew method runner invokes the
actual `formula.test` for both classes (`release-formula.test.rb:20–46,
67–79`). Thus the source change does not disable the private default
directory exercise.

## Evidence limits and handoff

The Ravel graph project is
`Users-antony-Development-AgenticOrchestration-Ravel.skill-directory-delivery`,
generation `2026-11-02T05:30:19Z`. Graph discovery returned no release-script
symbols; coverage explicitly excludes `scripts/` and `docs/`, and reports
changed metadata for `Taskfile.yml`. All material release-script claims above
come from direct reads of the current source, not negative graph results.

The saved producer lifecycle logs `target/release-lifecycle.30xzxI` and
`target/release-lifecycle.ItSdSk` contain the channel controls and mutant
failures. They are ignored local artifacts, do not pin all subject hashes,
and were not rerun here. Integration owns the fix, fresh verification and
any actual Homebrew exercise. Production and test files remain untouched by
this review.
